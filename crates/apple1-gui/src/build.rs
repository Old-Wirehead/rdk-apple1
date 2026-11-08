//! Start a source program: an Integer BASIC listing is typed into BASIC,
//! a 6502 assembly source is assembled with ca65 / ld65 (cc65 suite) when
//! they are installed, else (and for the xa sources: `#include`,
//! `#define`) with the built-in assembler. An optional `apple1.toml` (or `apple1.cfg`) in the
//! folder of the source or above tells what to build and where:
//!
//! ```text
//! file = "src/jeu.s"     # source or binary to start ("Tester le dernier build")
//! load = $0300           # load address (else the .org of the source)
//! run  = $0300           # run address (else the load address)
//! ld65 = "apple1.cfg"    # ld65 configuration (else -t none --start-addr load)
//! defines = "APPLE1 DEBUG=1"  # ca65 -D symbols (default: APPLE1)
//! ```

use std::path::{Path, PathBuf};
use std::process::Command;

use apple1_core::program::{self, Program};

use crate::i18n::tf;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RepoConfig {
    /// Folder of the configuration file.
    pub dir: Option<PathBuf>,
    pub file: Option<String>,
    pub load: Option<u16>,
    pub run: Option<u16>,
    pub ld65: Option<String>,
    /// ca65 `-D` symbols; None = `APPLE1` (the usual platform switch).
    pub defines: Option<Vec<String>>,
}

pub const CONFIG_NAMES: [&str; 3] = ["apple1.toml", "apple1.cfg", ".apple1.toml"];

fn parse_num(v: &str) -> Option<u16> {
    let v = v.trim().trim_matches('"').trim();
    if let Some(h) = v.strip_prefix('$').or_else(|| v.strip_prefix("0x")).or_else(|| v.strip_prefix("0X")) {
        return u16::from_str_radix(h, 16).ok();
    }
    // bare numbers are hexadecimal, as everywhere on the Apple-1
    u16::from_str_radix(v, 16).ok()
}

impl RepoConfig {
    pub fn parse(text: &str) -> RepoConfig {
        let mut c = RepoConfig::default();
        for line in text.lines() {
            let line = line.split('#').next().unwrap_or("").trim();
            let Some((k, v)) = line.split_once('=') else { continue };
            let (k, v) = (k.trim().to_ascii_lowercase(), v.trim());
            let s = v.trim_matches('"').trim_matches('\'').to_string();
            match k.as_str() {
                "file" | "main" | "source" | "fichier" => c.file = Some(s),
                "load" | "org" | "chargement" => c.load = parse_num(v),
                "run" | "start" | "entry" | "lancement" => c.run = parse_num(v),
                "ld65" | "config" | "cfg" => c.ld65 = Some(s),
                "defines" | "define" | "symboles" => c.defines = Some(s.split([',', ' ']).filter(|d| !d.is_empty()).map(str::to_string).collect()),
                _ => {}
            }
        }
        c
    }

    /// The configuration of the folder of `path` or of up to 4 folders above
    /// (a ld65 `apple1.cfg` with MEMORY / SEGMENTS is not one).
    pub fn find(path: &Path) -> RepoConfig {
        let mut d = if path.is_dir() { Some(path) } else { path.parent() };
        for _ in 0..5 {
            let Some(dir) = d else { break };
            for n in CONFIG_NAMES {
                if let Ok(t) = std::fs::read_to_string(dir.join(n)) {
                    if t.contains("MEMORY") && t.contains("SEGMENTS") {
                        continue;
                    }
                    let mut c = RepoConfig::parse(&t);
                    c.dir = Some(dir.to_path_buf());
                    return c;
                }
            }
            d = dir.parent();
        }
        RepoConfig::default()
    }
}

/// First `.org $XXXX`, `*= $XXXX` or `org $XXXX` of an assembly source.
pub fn org_of(text: &str) -> Option<u16> {
    for line in text.lines() {
        let l = line.split(';').next().unwrap_or("").trim();
        let low = l.to_ascii_lowercase();
        let rest = if let Some(r) = low.strip_prefix(".org") {
            r
        } else if let Some(r) = low.strip_prefix("org ") {
            r
        } else if let Some(r) = low.strip_prefix('*') {
            match r.trim_start().strip_prefix('=') {
                Some(r) => r,
                None => continue,
            }
        } else {
            continue;
        };
        if let Some(a) = parse_num(rest.trim()) {
            return Some(a);
        }
    }
    None
}

/// Keys typed for a BASIC listing: cold start of BASIC (E000R, which
/// clears the program), the lines, then RUN when `run`.
pub fn basic_keys(listing: &str, run: bool) -> String {
    let mut k = String::from("E000R\r");
    for l in listing.lines() {
        let l = l.trim_end();
        if l.trim().is_empty() {
            continue;
        }
        k.push_str(&l.to_ascii_uppercase());
        k.push('\r');
    }
    if run {
        k.push_str("RUN\r");
    }
    k
}

/// ca65 and ld65: in PATH, in ~/cc65/bin, /opt/cc65/bin, or a `cc65/bin`
/// folder next to the emulator.
pub fn find_cc65() -> Option<(PathBuf, PathBuf)> {
    let mut dirs: Vec<PathBuf> = std::env::var_os("PATH").map(|p| std::env::split_paths(&p).collect()).unwrap_or_default();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(d) = exe.parent() {
            dirs.push(d.join("cc65").join("bin"));
        }
    }
    dirs.push(crate::config::home().join("cc65").join("bin"));
    dirs.push(PathBuf::from("/opt/cc65/bin"));
    dirs.into_iter().find(|d| d.join("ca65").is_file() && d.join("ld65").is_file()).map(|d| (d.join("ca65"), d.join("ld65")))
}

/// Assembled program and its run address.
pub fn assemble(src: &Path) -> Result<(Program, u16), String> {
    let text = std::fs::read_to_string(src).map_err(|e| format!("{} : {e}", src.display()))?;
    let cfg = RepoConfig::find(src);
    let load = cfg.load.or_else(|| org_of(&text)).unwrap_or(0x0300);
    let run = cfg.run.unwrap_or(load);
    // xa sources (#include, #define: Apple1-Slideshow...), or no cc65: the
    // built-in assembler
    let cc65 = if cfg.ld65.is_some() || !apple1_core::xa::is_xa(&text) { find_cc65() } else { None };
    let Some((ca65, ld65)) = cc65 else {
        let a = apple1_core::asm6502::assemble_file(src, &[]).map_err(|e| {
            let first: Vec<String> = e.iter().take(5).map(|e| e.to_string()).collect();
            tf("e_asm_failed", &[&"asm6502", &first.join("\n")])
        })?;
        let run = cfg.run.unwrap_or(a.origin);
        let p = Program { blocks: vec![(a.origin, a.bytes)], run: Some(run), notes: vec![] };
        return Ok((p, run));
    };
    // private, unpredictable work folder (mode 0700), removed on every exit
    let tmp = TempDir::new("apple1-emu-asm")?;
    let dir = tmp.0.clone();
    let stem = src.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "prog".into());
    let (obj, bin) = (dir.join(format!("{stem}.o")), dir.join(format!("{stem}.bin")));
    let src_dir = src.parent().unwrap_or(Path::new("."));
    let mut ca = Command::new(&ca65);
    for d in cfg.defines.clone().unwrap_or_else(|| vec!["APPLE1".into()]) {
        ca.arg("-D").arg(d);
    }
    let out = ca.arg("-I").arg(src_dir).arg("-o").arg(&obj).arg(src).output().map_err(|e| format!("ca65 : {e}"))?;
    if !out.status.success() {
        return Err(tf("e_asm_failed", &[&"ca65", &String::from_utf8_lossy(&out.stderr).trim()]));
    }
    let mut ld = Command::new(&ld65);
    match cfg.ld65.as_ref().map(|c| cfg.dir.clone().unwrap_or_default().join(c)) {
        Some(c) => ld.arg("-C").arg(c),
        None => ld.arg("-t").arg("none").arg("--start-addr").arg(format!("${load:04X}")),
    };
    let out = ld.arg("-o").arg(&bin).arg(&obj).output().map_err(|e| format!("ld65 : {e}"))?;
    if !out.status.success() {
        return Err(tf("e_asm_failed", &[&"ld65", &String::from_utf8_lossy(&out.stderr).trim()]));
    }
    let mut p = program::load_file(&bin, Some(load))?;
    p.run = Some(run);
    Ok((p, run))
}

/// A work folder created with a random name and mode 0700 (`create_dir` fails
/// if the name already exists, so a pre-made folder or link is never reused);
/// removed when dropped, also after an error.
struct TempDir(PathBuf);

impl TempDir {
    fn new(prefix: &str) -> Result<TempDir, String> {
        use std::hash::{BuildHasher, Hasher};
        for _ in 0..16 {
            // RandomState is seeded by the OS: unpredictable names
            let r = std::collections::hash_map::RandomState::new().build_hasher().finish();
            let path = std::env::temp_dir().join(format!("{prefix}-{}-{r:016x}", std::process::id()));
            let mut b = std::fs::DirBuilder::new();
            #[cfg(unix)]
            std::os::unix::fs::DirBuilderExt::mode(&mut b, 0o700);
            match b.create(&path) {
                Ok(()) => return Ok(TempDir(path)),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(e.to_string()),
            }
        }
        Err("temporary folder".into())
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_and_org() {
        let c = RepoConfig::parse("# test\nfile = \"src/jeu.s\"\nload = $0280\nrun=0x300 # départ\nld65 = 'a1.cfg'\n");
        assert_eq!((c.file.as_deref(), c.load, c.run, c.ld65.as_deref()), (Some("src/jeu.s"), Some(0x280), Some(0x300), Some("a1.cfg")));
        assert_eq!(RepoConfig::parse("defines = \"APPLE1, DEBUG=1\"").defines, Some(vec!["APPLE1".to_string(), "DEBUG=1".to_string()]));
        assert_eq!(org_of("; x\n  .org $0280\n"), Some(0x280));
        assert_eq!(org_of("*= $1000"), Some(0x1000));
        assert_eq!(org_of("  * = $E000 ; rom"), Some(0xE000));
        assert_eq!(org_of("  lda #1"), None);
        assert_eq!(basic_keys("10 print \"hi\"\n\n20 end\n", true), "E000R\r10 PRINT \"HI\"\r20 END\rRUN\r");
    }

    #[test]
    fn assemble_with_ca65() {
        if find_cc65().is_none() {
            eprintln!("test ignoré : ca65/ld65 absents");
            return;
        }
        let dir = std::env::temp_dir().join(format!("a1asm-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let src = dir.join("t.s");
        std::fs::write(&src, "  .org $0300\nstart: lda #$C1\n  jsr $FFEF\n  jmp $FF1F\n").unwrap();
        let (p, run) = assemble(&src).unwrap();
        // xa syntax: the built-in assembler
        let xa = dir.join("x.a65");
        std::fs::write(&xa, "#define COUT $FFEF\n\t*=$0280\nSTART\tLDA\t#$C1\n\tJSR\tCOUT\n\t.BYT\t1\n").unwrap();
        let (px, rx) = assemble(&xa).unwrap();
        assert_eq!((px.blocks[0].0, rx, px.blocks[0].1.clone()), (0x280, 0x280, vec![0xA9, 0xC1, 0x20, 0xEF, 0xFF, 1]));
        assert_eq!(run, 0x300);
        assert_eq!(p.ranges(), vec![(0x300, 0x307)]);
        std::fs::write(&src, "  lda #\n").unwrap();
        assert!(assemble(&src).unwrap_err().contains("ca65"));
        std::fs::remove_dir_all(&dir).ok();
    }
}
