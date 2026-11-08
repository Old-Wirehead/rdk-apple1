//! Program library: folders (and their subfolders) of .wav/.aiff/.txt/.bin
//! files and BASIC / assembly sources, shown as a tree on the side. Each
//! folder keeps the ⚙ parameters of its programs in its own programs.json.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Instant, SystemTime};

use apple1_core::aciwav::ExportOptions;
use apple1_core::program::{self, Kind};
use apple1_core::wav;

use crate::params::{self, ProgParams, Store};

pub const HELLO_TXT: &str = include_str!("../../../samples/hello.txt");
pub const MULTI_TXT: &str = include_str!("../../../samples/multi.txt");

pub const EXTENSIONS: [&str; 13] = ["wav", "aiff", "aif", "txt", "bin", "apl", "hex", "prg", "bas", "s", "asm", "a65", "inc"];

/// Folders never shown in the tree.
const SKIP_DIRS: [&str; 4] = ["target", "node_modules", "__pycache__", "build-cache"];
/// Limits of the scan of one library folder.
const MAX_DEPTH: usize = 8;
const MAX_FILES: usize = 4000;

pub fn is_program(p: &Path) -> bool {
    let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    EXTENSIONS.contains(&ext.as_str())
}

/// Source programs, built when they are started.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    /// Integer BASIC listing, typed into BASIC.
    Basic,
    /// 6502 assembly (ca65 / ld65).
    Asm,
    /// ASCII art (.txt of at most 24 rows of 40 characters): shown in the
    /// « Photo → ASCII » tool, ▶ runs its 6502 program.
    Art,
}

/// Is this file a source program? `.bas`, `.s`/`.asm`/`.a65`, or a `.txt`
/// whose lines are BASIC lines ("10 PRINT ...") rather than a Woz dump.
pub fn source_kind(p: &Path, bytes: Option<&[u8]>) -> Option<Source> {
    let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    match ext.as_str() {
        "bas" => Some(Source::Basic),
        "s" | "asm" | "a65" => Some(Source::Asm),
        "txt" => {
            let owned;
            let b = match bytes {
                Some(b) => b,
                None => {
                    owned = std::fs::read(p).ok()?;
                    &owned
                }
            };
            let t = std::str::from_utf8(b).ok()?;
            if looks_basic_listing(t) {
                Some(Source::Basic)
            } else if apple1_core::hexdump::parse(t).map(|h| h.total_len() > 0).unwrap_or(false) {
                None
            } else {
                crate::ascii::looks_art(t, p).then_some(Source::Art)
            }
        }
        _ => None,
    }
}

/// Most non-empty lines start with a line number followed by a letter
/// (and the text is not a Woz monitor dump "0300: A9 00 ...").
pub fn looks_basic_listing(t: &str) -> bool {
    let lines: Vec<&str> = t.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
    if lines.is_empty() || apple1_core::hexdump::parse(t).map(|h| h.total_len() > 0).unwrap_or(false) {
        return false;
    }
    let ok = lines
        .iter()
        .filter(|l| {
            let digits = l.chars().take_while(|c| c.is_ascii_digit()).count();
            digits > 0 && digits <= 5 && l[digits..].trim_start().chars().next().map(|c| c.is_ascii_alphabetic()).unwrap_or(false)
        })
        .count();
    ok * 10 >= lines.len() * 8
}

#[derive(Clone, Debug, Default)]
pub struct Info {
    /// Translation key of the kind ("k_wav", "k_hex", "k_bin"...).
    pub kind: &'static str,
    pub ranges: String,
    pub run: Option<String>,
    pub duration: Option<f64>,
    pub size: u64,
    pub error: Option<String>,
    pub is_tape: bool,
    pub is_bin: bool,
    /// Tape recognised by `known` (parameters filled in automatically).
    pub known: Option<crate::known::Match>,
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub path: PathBuf,
    pub name: String,
    /// Folder of the file (its programs.json).
    pub dir: PathBuf,
    /// Index of its library folder in `Library::roots`.
    pub root: usize,
    pub info: Option<Info>,
}

/// A library folder (top of a tree).
#[derive(Clone, Debug, PartialEq)]
pub struct Root {
    pub path: PathBuf,
    /// Shown name ("owner/repo" for a GitHub project).
    pub label: String,
    /// GitHub project ("owner/repo[/sub]") when the folder is its cache.
    pub github: Option<String>,
    /// Why the folder cannot be read.
    pub error: Option<String>,
}

impl Root {
    pub fn local(path: PathBuf) -> Root {
        let label = path.display().to_string();
        Root { path, label, github: None, error: None }
    }
}

/// A folder of the tree: subfolders, then programs (indexes in `entries`).
#[derive(Clone, Debug, Default)]
pub struct Folder {
    pub path: PathBuf,
    pub name: String,
    pub folders: Vec<Folder>,
    pub files: Vec<usize>,
}

impl Folder {
    pub fn count(&self) -> usize {
        self.files.len() + self.folders.iter().map(Folder::count).sum::<usize>()
    }
}

type Key = (PathBuf, Option<SystemTime>, u64);

pub struct Library {
    /// Main folder (the first root).
    pub dir: PathBuf,
    pub roots: Vec<Root>,
    pub entries: Vec<Entry>,
    /// programs.json of each folder holding programs.
    stores: HashMap<PathBuf, Store>,
    /// Entries migrated from descriptions.json at the last scan.
    pub migrated: usize,
    cache: HashMap<PathBuf, (Key, Info)>,
    pending: HashMap<PathBuf, Key>,
    to_worker: Sender<Key>,
    from_worker: Receiver<(Key, Info)>,
    last_scan: Option<Instant>,
    pub error: Option<String>,
}

/// Decode a file to describe it (slow for tapes: done in a worker thread).
pub fn describe(path: &Path, size: u64) -> Info {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => return Info { error: Some(e.to_string()), size, ..Default::default() },
    };
    let mut info = Info { size, ..Default::default() };
    if let Some(src) = source_kind(path, Some(&bytes)) {
        let text = String::from_utf8_lossy(&bytes);
        match src {
            Source::Basic => {
                info.kind = "k_basic_src";
                let n = text.lines().filter(|l| !l.trim().is_empty()).count();
                info.ranges = crate::i18n::tf("lib_lines", &[&n]);
                info.run = Some("RUN".into());
            }
            Source::Asm => {
                info.kind = "k_asm_src";
                let cfg = crate::build::RepoConfig::find(path);
                let org = cfg.load.or_else(|| crate::build::org_of(&text));
                info.ranges = match org {
                    Some(a) => format!("{a:04X}…"),
                    None => crate::i18n::t("lib_unknown_addr").into(),
                };
                info.run = cfg.run.or(org).map(|a| format!("{a:X}R"));
            }
            Source::Art => {
                info.kind = "k_ascii_art";
                let a = apple1_core::asciiart::Art::from_text(&text);
                info.ranges = format!("{}x{}", a.lines.iter().map(|l| l.len()).max().unwrap_or(0), a.rows());
                info.run = Some("300R".into());
            }
        }
        return info;
    }
    let k = program::kind_of(path, Some(&bytes));
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_uppercase();
    match k {
        Kind::Wav => {
            info.is_tape = true;
            info.kind = if ext.starts_with("AIF") { "k_aiff" } else { "k_wav" };
            if let Ok(a) = wav::read_wav(path) {
                info.duration = Some(a.best_channel().len() as f64 / a.sample_rate.max(1) as f64);
            }
        }
        Kind::Hex => info.kind = "k_hex",
        Kind::Prg => info.kind = "k_prg",
        Kind::Bin => {
            info.is_bin = true;
            info.kind = "k_bin";
            return info;
        }
    }
    let imp = (k == Kind::Wav).then(|| apple1_core::aciwav::import_wav_bytes(&bytes));
    info.known = imp.as_ref().and_then(|r| r.as_ref().ok()).and_then(|i| crate::known::identify(i, Some(path)));
    if let Some(m) = info.known {
        info.ranges = m.known.ranges.to_string();
        info.run = Some(if m.known.basic { "E2B3R ⏎ RUN".to_string() } else { m.known.run_cmd.to_string() });
        return info;
    }
    let loaded = match &imp {
        Some(Ok(i)) => program::wav_to_program(i, None),
        Some(Err(e)) => Err(e.clone()),
        None => program::load_bytes(&bytes, k, None),
    };
    match loaded {
        Ok(p) => {
            info.ranges = crate::session::ranges_text(&p);
            info.run = Some(crate::session::run_hint(&p));
        }
        Err(e) => {
            info.ranges.clear();
            match imp.as_ref().and_then(|r| r.as_ref().ok()).and_then(program::guess_layout) {
                Some((r, g)) => {
                    info.ranges = format!("{} {}", r.iter().map(|(a, b)| format!("{a:04X}.{b:04X}")).collect::<Vec<_>>().join(" "), crate::i18n::t("l_guessed"));
                    info.run = (g != program::Guess::IntegerBasic).then(|| "E2B3R ⏎ RUN".to_string());
                }
                None => info.error = Some(e),
            }
        }
    }
    info
}

impl Library {
    pub fn new(dir: PathBuf) -> Self {
        let (to_worker, rx) = channel::<Key>();
        let (tx, from_worker) = channel();
        std::thread::spawn(move || {
            while let Ok(key) = rx.recv() {
                let info = describe(&key.0, key.2);
                if tx.send((key, info)).is_err() {
                    break;
                }
            }
        });
        let mut l = Library {
            roots: vec![Root::local(dir.clone())],
            dir,
            entries: Vec::new(),
            stores: HashMap::new(),
            migrated: 0,
            cache: HashMap::new(),
            pending: HashMap::new(),
            to_worker,
            from_worker,
            last_scan: None,
            error: None,
        };
        l.scan();
        l
    }

    pub fn set_dir(&mut self, dir: PathBuf) {
        self.dir = dir.clone();
        self.roots[0] = Root::local(dir);
        self.scan();
    }

    /// Library folders: the main one first, then the others and the
    /// GitHub projects.
    pub fn set_roots(&mut self, roots: Vec<Root>) {
        if roots.is_empty() || roots.iter().map(|r| (&r.path, &r.label, &r.github)).eq(self.roots.iter().map(|r| (&r.path, &r.label, &r.github))) {
            return;
        }
        self.dir = roots[0].path.clone();
        self.roots = roots;
        self.scan();
    }

    /// Folders holding programs (to look for Integer BASIC), main one first.
    pub fn folders(&self) -> Vec<PathBuf> {
        let mut v = vec![self.dir.clone()];
        for e in &self.entries {
            if !v.contains(&e.dir) {
                v.push(e.dir.clone());
            }
        }
        v
    }

    /// Tree of a library folder.
    pub fn tree(&self, root: usize) -> Folder {
        let base = &self.roots[root].path;
        let mut top = Folder { path: base.clone(), name: self.roots[root].label.clone(), ..Default::default() };
        for (i, e) in self.entries.iter().enumerate().filter(|(_, e)| e.root == root) {
            let rel = e.dir.strip_prefix(base).unwrap_or(Path::new(""));
            let mut f = &mut top;
            let mut path = base.clone();
            for c in rel.components() {
                let name = c.as_os_str().to_string_lossy().to_string();
                path.push(&name);
                let pos = match f.folders.iter().position(|x| x.name == name) {
                    Some(p) => p,
                    None => {
                        f.folders.push(Folder { path: path.clone(), name, ..Default::default() });
                        f.folders.len() - 1
                    }
                };
                f = &mut f.folders[pos];
            }
            f.files.push(i);
        }
        fn sort(f: &mut Folder) {
            f.folders.sort_by_key(|x| x.name.to_lowercase());
            f.folders.iter_mut().for_each(sort);
        }
        sort(&mut top);
        top
    }

    /// Subfolders listed at the top of the main library. The root folder
    /// itself is not a row, and files left loose in that root are not listed.
    /// The five default folders come first (even when empty, if the directory
    /// exists), then any other subfolder that holds programs.
    pub fn top_folders(&self, root: usize) -> Vec<Folder> {
        let mut top = self.tree(root);
        let base = &self.roots[root].path;
        for name in DEFAULT_DIRS {
            let path = base.join(name);
            if path.is_dir() && !top.folders.iter().any(|f| f.name.eq_ignore_ascii_case(name)) {
                top.folders.push(Folder { path, name: name.to_string(), ..Default::default() });
            }
        }
        top.folders.sort_by_key(|f| folder_rank(&f.name));
        top.folders
    }

    fn store(&self, path: &Path) -> Option<&Store> {
        self.stores.get(path.parent()?)
    }

    fn param(&self, path: &Path) -> Option<&ProgParams> {
        self.store(path)?.map.get(&file_name(path))
    }

    fn save_store(&mut self, dir: &Path) -> Result<(), String> {
        match self.stores.get(dir) {
            Some(st) => st.save(dir),
            None => Ok(()),
        }
    }

    /// Re-read the folder every 2 s (cheap: only changed files are decoded).
    /// Returns true when something changed.
    pub fn tick(&mut self) -> bool {
        let mut changed = false;
        while let Ok((key, info)) = self.from_worker.try_recv() {
            self.pending.remove(&key.0);
            if let Some(m) = info.known {
                self.preset(&key.0, m);
            }
            self.cache.insert(key.0.clone(), (key, info));
            changed = true;
        }
        if self.last_scan.map(|t| t.elapsed().as_secs_f32() > 2.0).unwrap_or(true) {
            self.scan();
            changed = true;
        } else if changed {
            self.apply_cache();
        }
        changed
    }

    /// A recognised tape gets its parameters, unless the user set them
    /// (entries without `auto`, including those of older versions).
    fn preset(&mut self, path: &Path, m: crate::known::Match) {
        let Some(dir) = path.parent().map(Path::to_path_buf) else { return };
        let want = m.known.params();
        let update = match self.param(path) {
            None => true,
            Some(p) => !p.auto.is_empty() && *p != want,
        };
        if update {
            self.stores.entry(dir.clone()).or_default().map.insert(file_name(path), want);
            if let Err(e) = self.save_store(&dir) {
                self.error = Some(e);
            }
        }
    }

    /// Parameters of `path` were filled in from the table of known tapes.
    pub fn is_auto(&self, path: &Path) -> bool {
        self.param(path).map(|p| !p.auto.is_empty()).unwrap_or(false)
    }

    fn apply_cache(&mut self) {
        for e in &mut self.entries {
            e.info = self.cache.get(&e.path).map(|c| c.1.clone());
        }
    }

    pub fn scan(&mut self) {
        self.last_scan = Some(Instant::now());
        self.error = None;
        let mut found = Vec::new();
        for (i, root) in self.roots.iter_mut().enumerate() {
            root.error = None;
            match std::fs::read_dir(&root.path) {
                Ok(_) => walk(&root.path, i, 0, &mut found),
                Err(e) => {
                    let msg = format!("{} : {e}", root.path.display());
                    if i == 0 {
                        self.error = Some(msg.clone());
                    }
                    root.error = Some(msg);
                }
            }
        }
        let mut entries = Vec::new();
        for (path, root, key) in found {
            let fresh = self.cache.get(&path).map(|c| c.0 == key).unwrap_or(false);
            if !fresh && self.pending.get(&path) != Some(&key) {
                self.pending.insert(path.clone(), key.clone());
                let _ = self.to_worker.send(key);
            }
            let name = file_name(&path);
            let dir = path.parent().map(Path::to_path_buf).unwrap_or_default();
            entries.push(Entry { path, name, dir, root, info: None });
        }
        entries.sort_by_key(|e| (e.root, e.dir.clone(), e.name.to_lowercase()));
        self.entries = entries;
        self.apply_cache();
        let mut dirs: Vec<PathBuf> = self.entries.iter().map(|e| e.dir.clone()).collect();
        dirs.push(self.dir.clone());
        dirs.sort();
        dirs.dedup();
        self.stores.clear();
        for d in dirs {
            let (st, n) = Store::load(&d);
            self.migrated += n;
            self.stores.insert(d, st);
        }
    }

    pub fn description(&self, path: &Path) -> Option<&String> {
        self.param(path).map(|p| &p.notes).filter(|n| !n.trim().is_empty())
    }

    /// Saved parameters of a program, or those read from the file.
    pub fn params_of(&self, path: &Path) -> ProgParams {
        self.param(path).cloned().unwrap_or_else(|| params::from_file(path))
    }

    pub fn has_params(&self, path: &Path) -> bool {
        self.param(path).is_some()
    }

    pub fn set_params(&mut self, path: &Path, p: ProgParams) -> Result<(), String> {
        let dir = path.parent().map(Path::to_path_buf).unwrap_or_default();
        self.stores.entry(dir.clone()).or_default().map.insert(file_name(path), p);
        self.save_store(&dir)
    }

    pub fn rename(&mut self, from: &Path, new_name: &str) -> Result<PathBuf, String> {
        let new_name = new_name.trim();
        if new_name.is_empty() || new_name.contains('/') {
            return Err("nom invalide".into());
        }
        let to = from.with_file_name(new_name);
        if to.exists() {
            return Err(format!("« {new_name} » existe déjà"));
        }
        std::fs::rename(from, &to).map_err(|e| e.to_string())?;
        let dir = from.parent().map(Path::to_path_buf).unwrap_or_default();
        if let Some(st) = self.stores.get_mut(&dir) {
            if let Some(d) = st.map.remove(&file_name(from)) {
                st.map.insert(new_name.to_string(), d);
                self.save_store(&dir)?;
            }
        }
        self.scan();
        Ok(to)
    }

    pub fn delete(&mut self, path: &Path) -> Result<(), String> {
        std::fs::remove_file(path).map_err(|e| e.to_string())?;
        let dir = path.parent().map(Path::to_path_buf).unwrap_or_default();
        if self.stores.get_mut(&dir).map(|st| st.map.remove(&file_name(path)).is_some()).unwrap_or(false) {
            self.save_store(&dir)?;
        }
        self.scan();
        Ok(())
    }
}

fn file_name(p: &Path) -> String {
    p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default()
}

/// Programs of a folder and its subfolders (hidden folders and build
/// folders skipped).
fn walk(dir: &Path, root: usize, depth: usize, out: &mut Vec<(PathBuf, usize, Key)>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut subdirs = Vec::new();
    for de in rd.flatten() {
        if out.len() >= MAX_FILES {
            return;
        }
        let path = de.path();
        let name = de.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }
        let Ok(ft) = de.file_type() else { continue };
        let is_dir = ft.is_dir() || (ft.is_symlink() && path.is_dir());
        if is_dir {
            if depth < MAX_DEPTH && !SKIP_DIRS.contains(&name.as_str()) {
                subdirs.push(path);
            }
            continue;
        }
        if !path.is_file() || !is_program(&path) {
            continue;
        }
        let md = de.metadata().ok();
        let key: Key = (path.clone(), md.as_ref().and_then(|m| m.modified().ok()), md.map(|m| m.len()).unwrap_or(0));
        out.push((path, root, key));
    }
    for d in subdirs {
        walk(&d, root, depth + 1, out);
    }
}

/// Examples: (file, run address when the file does not tell, notes key).
const EXAMPLES: [(&str, &str, &str); 12] = [
    ("hello.txt", "", "ex_hello_txt"),
    ("hello.wav", "", "ex_hello_wav"),
    ("hello.aiff", "", "ex_hello_aiff"),
    ("multi.txt", "", "ex_multi_txt"),
    ("multi.wav", "", "ex_multi_wav"),
    ("multi.aiff", "", "ex_multi_aiff"),
    ("puissance4-couleur.bas", "", "ex_puissance4"),
    ("gen2-barres.txt", "C00", "ex_gen2_barres_txt"),
    ("gen2-test1.apl", "24B", "ex_gen2_test1_apl"),
    ("gen2-test2.apl", "280", "ex_gen2_test2_apl"),
    ("gen2-test3.apl", "C00", "ex_gen2_test3_apl"),
    ("gen2-test4.apl", "280", "ex_gen2_test4_apl"),
];

/// The 5 folders of a library: BASIC, 6502, GEN2 COLOR, WAV (tapes), Art ASCII.
pub const DIR_BASIC: &str = "BASIC";
pub const DIR_ASM: &str = "6502";
pub const DIR_BERNIE: &str = "GEN2 COLOR";
pub const DIR_WAV: &str = "WAV";
pub const DIR_ART: &str = crate::ascii::ART_DIR;
pub const DEFAULT_DIRS: [&str; 5] = [DIR_BASIC, DIR_ASM, DIR_BERNIE, DIR_WAV, DIR_ART];

/// The five default folders first, in that order, then any other name.
fn folder_rank(name: &str) -> (usize, String) {
    let i = DEFAULT_DIRS.iter().position(|d| d.eq_ignore_ascii_case(name)).unwrap_or(DEFAULT_DIRS.len());
    (i, name.to_lowercase())
}
/// Names of the folders before 7.9.2 and their new names (renamed once, content kept).
pub const LEGACY_DIRS: [(&str, &str); 2] = [("Assembleur", DIR_ASM), ("Uncle Bernie", DIR_BERNIE)];

/// The colour Connect 4 of Claude IA (G2 Basic + GEN2 card), always available in « GEN2 COLOR ».
pub const PUISSANCE4: (&str, &str) = ("puissance4-couleur.bas", include_str!("../../../docs/exemples/basic/puissance4-couleur.bas"));

/// Which of the 4 default folders a bundled file belongs to (None: not a program).
/// `basic_stem`: a `.hex.txt` next to a `.bas` of the same name is a BASIC program.
fn category(p: &Path, basic_stem: bool) -> Option<&'static str> {
    let name = p.file_name()?.to_string_lossy().to_ascii_lowercase();
    if name.starts_with("gen2-") || name.starts_with("gen2_") || (name.ends_with(".bas") && (name.contains("-couleur") || name.contains("-color"))) {
        return Some(DIR_BERNIE);
    }
    if !is_program(p) || name.starts_with("readme") || name.starts_with("licen") {
        return None;
    }
    let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    Some(match ext.as_str() {
        "wav" | "aiff" | "aif" => DIR_WAV,
        "bas" => DIR_BASIC,
        "txt" if name.ends_with(".hex.txt") && basic_stem => DIR_BASIC,
        _ => DIR_ASM,
    })
}

/// Copy the programs of `src` (and its subfolders, but not tools, tests, harness or libraries)
/// into the 4 folders of `dir`. Files already there are kept. Returns the files copied.
fn sort_into(dir: &Path, src: &Path, depth: usize, n: &mut usize) {
    let Ok(rd) = std::fs::read_dir(src) else { return };
    let mut items: Vec<_> = rd.flatten().map(|d| d.path()).collect();
    items.sort();
    for p in items {
        let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        if p.is_dir() {
            let skip = SKIP_DIRS.contains(&name.as_str()) || ["tests", "test", "harness", "tools", "lib", "captures"].contains(&name.to_ascii_lowercase().as_str());
            if !skip && !name.starts_with('.') && depth < 4 {
                sort_into(dir, &p, depth + 1, n);
            }
            continue;
        }
        let stem_bas = name.to_ascii_lowercase().strip_suffix(".hex.txt").map(|st| src.join(format!("{st}.bas")).exists() || src.join(format!("{}.BAS", st.to_ascii_uppercase())).exists()).unwrap_or(false);
        let is_howto = name == "GEN2_TESTS_HOWTO";
        let Some(cat) = (if is_howto { Some(DIR_BERNIE) } else { category(&p, stem_bas) }) else { continue };
        let d = dir.join(cat);
        if std::fs::create_dir_all(&d).is_err() {
            continue;
        }
        let to = d.join(&name);
        if !to.exists() && std::fs::copy(&p, &to).is_ok() {
            *n += 1;
        }
    }
}

/// Folders where the archive puts its programs, looking from the program: `exemples/` next to
/// it, or at the root of the archive / of the installation (`bin/<arch>/` is two levels down).
fn bundled_dirs() -> Option<(PathBuf, Option<PathBuf>)> {
    let exe_dir = std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf))?;
    let ex = [
        exe_dir.join("exemples"),
        exe_dir.join("../exemples"),
        exe_dir.join("../../exemples"),
        exe_dir.join("samples"),
        exe_dir.join("../../samples"),
        exe_dir.join("../../../samples"),
    ]
    .into_iter()
    .find(|d| d.is_dir())?;
    let root = ex.parent().map(|p| p.join("ports")).filter(|p| p.is_dir());
    Some((ex, root))
}

/// Rename the folders of the previous versions (Assembleur → 6502, Uncle Bernie → GEN2 COLOR): the
/// folder is renamed when the new name is free, else its files are moved (those that do not
/// clash) and the old folder is removed when it is empty. Returns the number of folders handled.
pub fn migrate_legacy_dirs(dir: &Path) -> usize {
    let mut n = 0;
    for (old, new) in LEGACY_DIRS {
        let (o, nw) = (dir.join(old), dir.join(new));
        if !o.is_dir() {
            continue;
        }
        if !nw.exists() {
            if std::fs::rename(&o, &nw).is_ok() {
                n += 1;
            }
            continue;
        }
        if let Ok(rd) = std::fs::read_dir(&o) {
            for e in rd.flatten() {
                let to = nw.join(e.file_name());
                if e.file_name() != params::FILE && !to.exists() {
                    let _ = std::fs::rename(e.path(), to);
                }
            }
        }
        if std::fs::remove_dir(&o).is_ok() {
            n += 1;
        }
    }
    n
}

/// Make sure a library has its 5 folders (BASIC, 6502, GEN2 COLOR, WAV, Art ASCII) with their
/// examples. Never overwrites a file. Used at the first start of an existing library after an
/// update (see `GuiConfig::extras`) and by `populate`. Returns the number of files added.
pub fn ensure_layout(dir: &Path) -> usize {
    let mut n = 0;
    migrate_legacy_dirs(dir);
    for d in DEFAULT_DIRS {
        let _ = std::fs::create_dir_all(dir.join(d));
    }
    if let Some((src, ports)) = bundled_dirs() {
        sort_into(dir, &src, 0, &mut n);
        if let Some(p) = ports {
            sort_into(dir, &p, 0, &mut n);
        }
    }
    let p4 = dir.join(DIR_BERNIE).join(PUISSANCE4.0);
    if !p4.exists() && std::fs::write(&p4, PUISSANCE4.1).is_ok() {
        n += 1;
    }
    if let Ok(k) = crate::ascii::install_examples(dir) {
        n += k;
    }
    n
}

/// First run: create the folder with 5 subfolders (BASIC, 6502, GEN2 COLOR, WAV, Art ASCII) and put
/// the examples in them (copied from `exemples/` of the archive when present, else generated),
/// then the game ports (`ports/` next to `exemples/`) when the archive has them.
pub fn populate(dir: &Path) -> Result<usize, String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("{} : {e}", dir.display()))?;
    let mut n = ensure_layout(dir);
    if !dir.join(DIR_ASM).read_dir().map(|mut r| r.next().is_some()).unwrap_or(false) {
        let asm = dir.join(DIR_ASM);
        let wav = dir.join(DIR_WAV);
        std::fs::create_dir_all(&asm).map_err(|e| format!("{} : {e}", asm.display()))?;
        std::fs::create_dir_all(&wav).map_err(|e| format!("{} : {e}", wav.display()))?;
        for (name, txt) in [("hello", HELLO_TXT), ("multi", MULTI_TXT)] {
            let t = asm.join(format!("{name}.txt"));
            if !t.exists() && std::fs::write(&t, txt).is_ok() {
                n += 1;
            }
            if let Ok(p) = program::load_bytes(txt.as_bytes(), Kind::Hex, None) {
                for ext in ["wav", "aiff"] {
                    let f = wav.join(format!("{name}.{ext}"));
                    if !f.exists() && program::save_file(&f, &p, &ExportOptions::default()).is_ok() {
                        n += 1;
                    }
                }
            }
        }
    }
    // ⚙ notes and run addresses of the examples, in the programs.json of their folder
    for d in DEFAULT_DIRS {
        let folder = dir.join(d);
        if folder.join(params::FILE).exists() {
            continue;
        }
        let mut st = Store::default();
        for (f, run, notes) in EXAMPLES {
            let path = folder.join(f);
            if path.exists() {
                let mut p = params::from_file(&path);
                if !run.is_empty() {
                    p.run_addr = run.to_string();
                    p.run_cmd = format!("{run}R");
                }
                p.notes = crate::i18n::t(notes).to_string();
                st.map.insert(f.to_string(), p);
            }
        }
        if !st.map.is_empty() {
            let _ = st.save(&folder);
        }
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn populate_and_describe() {
        let dir = std::env::temp_dir().join(format!("a1lib-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        assert!(populate(&dir).unwrap() >= 2);
        let mut l = Library::new(dir.clone());
        let t0 = Instant::now();
        while l.entries.iter().any(|e| e.info.is_none()) && t0.elapsed().as_secs() < 60 {
            std::thread::sleep(std::time::Duration::from_millis(20));
            l.tick();
        }
        for d in DEFAULT_DIRS {
            assert!(dir.join(d).is_dir(), "{d}");
        }
        assert_eq!(std::fs::read_dir(&dir).unwrap().flatten().filter(|e| e.path().is_dir()).count(), 5);
        assert!(dir.join("Art ASCII").is_dir());
        let hello = l.entries.iter().find(|e| e.name == "hello.txt").unwrap();
        assert!(hello.path.ends_with("6502/hello.txt"));
        let i = hello.info.as_ref().unwrap();
        assert_eq!(i.ranges, "0300.031E");
        assert_eq!(i.run.as_deref(), Some("300R"));
        assert!(l.description(&dir.join("6502/hello.txt")).is_some());
        assert_eq!(l.params_of(&dir.join("6502/hello.txt")).run_cmd, "300R");
        if let Some(w) = l.entries.iter().find(|e| e.name == "hello.wav") {
            let i = w.info.as_ref().unwrap();
            assert!(i.is_tape && i.duration.unwrap() > 10.0, "{i:?}");
            assert_eq!(i.ranges, "0300.031E");
        }
        let p = l.rename(&dir.join("6502/hello.txt"), "salut.txt").unwrap();
        assert!(l.description(&dir.join("6502/salut.txt")).is_some());
        l.delete(&p).unwrap();
        assert!(l.entries.iter().all(|e| e.name != "salut.txt"));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn bundled_files_go_to_the_5_folders() {
        let base = std::env::temp_dir().join(format!("a1sort-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let (src, dir) = (base.join("src"), base.join("lib"));
        for d in ["basic", "asm/orbit", "asm/lib", "tests", "harness"] {
            std::fs::create_dir_all(src.join(d)).unwrap();
        }
        for f in [
            "basic/stars.bas", "basic/stars.hex.txt", "basic/STARS.wav", "asm/orbit/orbit.s", "asm/orbit/orbit.bin",
            "asm/orbit/orbit.hex.txt", "asm/orbit/ORBIT.wav", "asm/orbit/orbit.lst", "asm/lib/fp.s", "tests/t.txt",
            "gen2-test1.apl", "gen2-barres.txt", "GEN2_TESTS_HOWTO", "hello.txt", "README.txt",
        ] {
            std::fs::write(src.join(f), "x").unwrap();
        }
        let mut n = 0;
        sort_into(&dir, &src, 0, &mut n);
        let ls = |d: &str| {
            let mut v: Vec<String> = std::fs::read_dir(dir.join(d)).unwrap().flatten().map(|e| e.file_name().to_string_lossy().to_string()).collect();
            v.sort();
            v
        };
        assert_eq!(ls("BASIC"), ["stars.bas", "stars.hex.txt"]);
        assert_eq!(ls("6502"), ["hello.txt", "orbit.bin", "orbit.hex.txt", "orbit.s"]);
        assert_eq!(ls("GEN2 COLOR"), ["GEN2_TESTS_HOWTO", "gen2-barres.txt", "gen2-test1.apl"]);
        assert_eq!(ls("WAV"), ["ORBIT.wav", "STARS.wav"]);
        assert_eq!(n, 11);
        std::fs::remove_dir_all(&base).ok();
    }

    #[test]
    fn legacy_folders_are_renamed_and_layout_restored() {
        let dir = std::env::temp_dir().join(format!("a1mig-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("Assembleur")).unwrap();
        std::fs::create_dir_all(dir.join("Uncle Bernie")).unwrap();
        std::fs::create_dir_all(dir.join("GEN2 COLOR")).unwrap();
        std::fs::write(dir.join("Assembleur/mon.s"), "  rts\n").unwrap();
        std::fs::write(dir.join("Uncle Bernie/a.apl"), "x").unwrap();
        std::fs::write(dir.join("GEN2 COLOR/a.apl"), "y").unwrap();
        std::fs::write(dir.join("Uncle Bernie/b.apl"), "z").unwrap();
        std::fs::write(dir.join("vrac.txt"), "a user file at the top").unwrap();
        assert!(ensure_layout(&dir) >= 1);
        for d in DEFAULT_DIRS {
            assert!(dir.join(d).is_dir(), "{d}");
        }
        assert!(!dir.join("Assembleur").exists());
        // the clashing file stays in the old folder (nothing is ever lost or overwritten)
        assert!(dir.join("Uncle Bernie/a.apl").is_file() && !dir.join("Uncle Bernie/b.apl").exists());
        assert!(dir.join("6502/mon.s").is_file());
        // clash: the file already in the new folder wins, the other one is moved
        assert_eq!(std::fs::read_to_string(dir.join("GEN2 COLOR/a.apl")).unwrap(), "y");
        assert!(dir.join("GEN2 COLOR/b.apl").is_file());
        assert!(dir.join("GEN2 COLOR/puissance4-couleur.bas").is_file());
        assert!(std::fs::read_to_string(dir.join("GEN2 COLOR/puissance4-couleur.bas")).unwrap().contains("CLAUDE IA"));
        assert!(dir.join("Art ASCII").is_dir() && dir.join("vrac.txt").is_file());
        // second call: nothing more to add
        assert_eq!(ensure_layout(&dir), 0);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn tree_of_several_folders() {
        let base = std::env::temp_dir().join(format!("a1tree-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let (a, b) = (base.join("a"), base.join("b"));
        for d in [a.join("jeux/vieux"), a.join(".cache"), a.join("target"), b.clone()] {
            std::fs::create_dir_all(d).unwrap();
        }
        std::fs::write(a.join("hello.txt"), HELLO_TXT).unwrap();
        std::fs::write(a.join("jeux/vieux/z.txt"), HELLO_TXT).unwrap();
        std::fs::write(a.join("jeux/listing.bas"), "10 PRINT \"HI\"\n20 GOTO 10\n").unwrap();
        std::fs::write(a.join("jeux/notes.md"), "pas un programme").unwrap();
        std::fs::write(a.join(".cache/cache.txt"), HELLO_TXT).unwrap();
        std::fs::write(a.join("target/t.bin"), [0u8; 4]).unwrap();
        std::fs::write(b.join("prog.s"), "  .org $0300\n  rts\n").unwrap();
        let mut l = Library::new(a.clone());
        l.set_roots(vec![Root::local(a.clone()), Root::local(b.clone()), Root::local(base.join("absent"))]);
        let names: Vec<&str> = l.entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["hello.txt", "listing.bas", "z.txt", "prog.s"]);
        let t = l.tree(0);
        assert_eq!((t.files.len(), t.folders.len(), t.count()), (1, 1, 3));
        assert_eq!(t.folders[0].name, "jeux");
        assert_eq!(t.folders[0].folders[0].name, "vieux");
        assert_eq!(l.tree(1).count(), 1);
        assert!(l.roots[2].error.is_some() && l.error.is_none());
        // the main library lists its folders directly: the loose file stays out
        let shown = l.top_folders(0);
        assert_eq!(shown.iter().map(|f| f.name.as_str()).collect::<Vec<_>>(), ["jeux"]);
        let loose = l.entries.iter().position(|e| e.name == "hello.txt").unwrap();
        assert!(shown.iter().all(|f| !folder_holds(f, loose)));
        // ⚙ parameters go to the programs.json of the file's folder
        let z = a.join("jeux/vieux/z.txt");
        let mut p = l.params_of(&z);
        p.notes = "dans un sous-dossier".into();
        l.set_params(&z, p).unwrap();
        assert!(a.join("jeux/vieux").join(params::FILE).is_file());
        l.scan();
        assert_eq!(l.description(&z).map(String::as_str), Some("dans un sous-dossier"));
        assert!(l.folders().contains(&b));
        // sources
        assert_eq!(source_kind(&a.join("jeux/listing.bas"), None), Some(Source::Basic));
        assert_eq!(source_kind(&b.join("prog.s"), None), Some(Source::Asm));
        assert_eq!(source_kind(&a.join("hello.txt"), None), None);
        assert!(looks_basic_listing("10 PRINT 1\n20 END\n"));
        assert!(!looks_basic_listing("0300: A9 00\n"));
        std::fs::remove_dir_all(&base).ok();
    }

    fn folder_holds(f: &Folder, i: usize) -> bool {
        f.files.contains(&i) || f.folders.iter().any(|s| folder_holds(s, i))
    }

    #[test]
    fn five_default_folders_are_listed_first_and_loose_files_are_hidden() {
        let dir = std::env::temp_dir().join(format!("a1five-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        for name in DEFAULT_DIRS {
            std::fs::create_dir_all(dir.join(name)).unwrap();
        }
        std::fs::create_dir_all(dir.join("extra")).unwrap();
        std::fs::write(dir.join("BASIC/hello.bas"), "10 END\n").unwrap();
        std::fs::write(dir.join("6502/mon.s"), "  rts\n").unwrap();
        std::fs::write(dir.join("extra/z.bas"), "20 END\n").unwrap();
        std::fs::write(dir.join("loose.txt"), HELLO_TXT).unwrap();
        let l = Library::new(dir.clone());
        let shown = l.top_folders(0);
        let names: Vec<&str> = shown.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, ["BASIC", "6502", "GEN2 COLOR", "WAV", "Art ASCII", "extra"]);
        let loose = l.entries.iter().position(|e| e.name == "loose.txt").unwrap();
        assert!(l.top_folders(0).iter().all(|f| !folder_holds(f, loose)));
        // the tree still knows the loose file; the panel simply does not show it
        assert!(l.tree(0).files.contains(&loose));
        std::fs::remove_dir_all(&dir).ok();
    }
}
