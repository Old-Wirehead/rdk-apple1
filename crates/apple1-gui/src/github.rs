//! GitHub projects in the library: the programs and sources of a public
//! (or, with a token, private) repository are downloaded from its default
//! branch into ~/.cache/apple1-emu/github/<owner>/<repo>/<sha>/ and shown
//! as a library folder. « Tester le dernier build » takes the latest
//! Release asset, else the latest Actions artifact (token needed), else a
//! source of the repository, built when it is started (see `build`).
//!
//! The network runs in a worker thread; the token is never written in a
//! message or a log.

use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::i18n::{t, tf};

const API: &str = "https://api.github.com";
const RAW: &str = "https://raw.githubusercontent.com";
/// Files bigger than this are not downloaded.
const MAX_FILE: u64 = 4 << 20;
const MAX_FILES: usize = 400;
pub const META: &str = "apple1-github.json";

/// A project: "owner/repo", optionally a branch and a subfolder.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Spec {
    pub owner: String,
    pub repo: String,
    pub branch: Option<String>,
    pub sub: Option<String>,
}

fn valid(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c)) && s != "." && s != ".."
}

impl Spec {
    /// `owner/repo`, `owner/repo/sub/folder`, `https://github.com/owner/repo`,
    /// `…/repo.git`, `…/repo/tree/branch/sub/folder`.
    pub fn parse(s: &str) -> Result<Spec, String> {
        let s = s.trim();
        let rest = s.strip_prefix("https://").or_else(|| s.strip_prefix("http://")).unwrap_or(s);
        let rest = rest.strip_prefix("www.").unwrap_or(rest);
        let rest = rest.strip_prefix("github.com/").or_else(|| rest.strip_prefix("github.com:")).unwrap_or(rest);
        let rest = rest.strip_prefix("git@github.com:").unwrap_or(rest);
        let parts: Vec<&str> = rest.split(['/', '?', '#']).filter(|p| !p.is_empty()).collect();
        let bad = || tf("gh_bad_spec", &[&s]);
        if parts.len() < 2 {
            return Err(bad());
        }
        let owner = parts[0].to_string();
        let repo = parts[1].trim_end_matches(".git").to_string();
        if !valid(&owner) || !valid(&repo) {
            return Err(bad());
        }
        let (branch, sub_parts) = match parts.get(2) {
            Some(&"tree") | Some(&"blob") if parts.len() >= 4 => (Some(parts[3].to_string()), &parts[4..]),
            _ => (None, &parts[2..]),
        };
        if sub_parts.iter().any(|p| *p == ".." || *p == ".") {
            return Err(bad());
        }
        let sub = (!sub_parts.is_empty()).then(|| sub_parts.join("/"));
        Ok(Spec { owner, repo, branch, sub })
    }

    /// Text saved in gui.json.
    pub fn text(&self) -> String {
        let mut s = format!("{}/{}", self.owner, self.repo);
        match (&self.branch, &self.sub) {
            (Some(b), sub) => {
                s.push_str(&format!("/tree/{b}"));
                if let Some(x) = sub {
                    s.push_str(&format!("/{x}"));
                }
            }
            (None, Some(x)) => s.push_str(&format!("/{x}")),
            (None, None) => {}
        }
        s
    }

    pub fn label(&self) -> String {
        match &self.sub {
            Some(x) => format!("{}/{} ({x})", self.owner, self.repo),
            None => format!("{}/{}", self.owner, self.repo),
        }
    }

    pub fn url(&self) -> String {
        format!("https://github.com/{}", self.text())
    }

    /// ~/.cache/apple1-emu/github/<owner>/<repo>
    pub fn cache_dir(&self, cache: &Path) -> PathBuf {
        cache.join(&self.owner).join(&self.repo)
    }
}

/// What was downloaded last time (saved next to the files).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Meta {
    pub sha: String,
    pub date: String,
    pub branch: String,
    pub message: String,
    pub files: usize,
}

impl Meta {
    pub fn load(spec: &Spec, cache: &Path) -> Option<Meta> {
        let key = meta_name(spec);
        serde_json::from_slice(&std::fs::read(spec.cache_dir(cache).join(key)).ok()?).ok()
    }

    pub fn short(&self) -> &str {
        &self.sha[..self.sha.len().min(7)]
    }

    /// Library folder of this download.
    pub fn dir(&self, spec: &Spec, cache: &Path) -> PathBuf {
        let mut d = spec.cache_dir(cache).join(self.short());
        if let Some(s) = &spec.sub {
            d = d.join(s);
        }
        d
    }
}

fn meta_name(spec: &Spec) -> String {
    match &spec.sub {
        Some(s) => format!("{}-{}", s.replace('/', "_"), META),
        None => META.to_string(),
    }
}

pub fn cache_root() -> PathBuf {
    crate::config::cache_base().join("github")
}

/// HTTP GET (a trait so that the tests run without network).
pub trait Http: Send + Sync {
    fn get(&self, url: &str, token: Option<&str>) -> Result<Vec<u8>, HttpError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum HttpError {
    Status(u16),
    Other(String),
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HttpError::Status(403) | HttpError::Status(429) => write!(f, "{}", t("gh_rate_limit")),
            HttpError::Status(404) => write!(f, "{}", t("gh_not_found")),
            HttpError::Status(c) => write!(f, "HTTP {c}"),
            HttpError::Other(e) => write!(f, "{e}"),
        }
    }
}

/// The real client: ureq + rustls (pure Rust, no OpenSSL).
pub struct Ureq(ureq::Agent);

impl Ureq {
    pub fn new() -> Ureq {
        Ureq(ureq::AgentBuilder::new().timeout_connect(std::time::Duration::from_secs(15)).timeout(std::time::Duration::from_secs(120)).user_agent(concat!("apple1-emu/", env!("CARGO_PKG_VERSION"))).build())
    }
}

impl Http for Ureq {
    fn get(&self, url: &str, token: Option<&str>) -> Result<Vec<u8>, HttpError> {
        let mut r = self.0.get(url);
        if url.starts_with(API) {
            r = r.set("Accept", "application/vnd.github+json").set("X-GitHub-Api-Version", "2022-11-28");
            if let Some(tk) = token {
                r = r.set("Authorization", &format!("Bearer {tk}"));
            }
        }
        match r.call() {
            Ok(resp) => {
                let mut v = Vec::new();
                std::io::Read::read_to_end(&mut resp.into_reader().take(256 << 20), &mut v).map_err(|e| HttpError::Other(e.to_string()))?;
                Ok(v)
            }
            Err(ureq::Error::Status(c, _)) => Err(HttpError::Status(c)),
            // never show the URL with its query or the headers
            Err(ureq::Error::Transport(e)) => Err(HttpError::Other(e.kind().to_string())),
        }
    }
}

use std::io::Read as _;

fn json(http: &dyn Http, url: &str, token: Option<&str>) -> Result<Value, String> {
    let b = http.get(url, token).map_err(|e| e.to_string())?;
    serde_json::from_slice(&b).map_err(|e| format!("GitHub : {e}"))
}

/// Progress callback: (text, fraction 0..1).
pub type Progress<'a> = &'a dyn Fn(String, f32);

fn wanted(path: &str) -> bool {
    let p = Path::new(path);
    let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let ext = p.extension().map(|e| e.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
    // files included by the sources (.inc, .rle tables of Apple1-Slideshow...)
    let include = INCLUDE_EXT.contains(&ext.as_str());
    crate::build::CONFIG_NAMES.contains(&name.as_str()) || ((crate::library::is_program(p) || include) && !name.starts_with('.'))
}

/// Extensions of the files a source may include (downloaded, not listed).
pub const INCLUDE_EXT: [&str; 6] = ["inc", "i", "h", "mac", "rle", "a"];

/// Download the programs of the default branch (or the chosen one).
pub fn sync(spec: &Spec, cache: &Path, http: &dyn Http, token: Option<&str>, progress: Progress) -> Result<Meta, String> {
    let (o, r) = (&spec.owner, &spec.repo);
    progress(t("gh_contacting").into(), 0.0);
    let branch = match &spec.branch {
        Some(b) => b.clone(),
        None => json(http, &format!("{API}/repos/{o}/{r}"), token)?["default_branch"].as_str().unwrap_or("main").to_string(),
    };
    let c = json(http, &format!("{API}/repos/{o}/{r}/commits/{branch}"), token)?;
    let sha = c["sha"].as_str().ok_or_else(|| t("gh_not_found").to_string())?.to_string();
    let date = c["commit"]["committer"]["date"].as_str().unwrap_or("").to_string();
    let message = c["commit"]["message"].as_str().unwrap_or("").lines().next().unwrap_or("").to_string();
    let tree = json(http, &format!("{API}/repos/{o}/{r}/git/trees/{sha}?recursive=1"), token)?;
    let prefix = spec.sub.as_ref().map(|s| format!("{}/", s.trim_end_matches('/')));
    let mut files: Vec<(String, u64)> = tree["tree"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter(|e| e["type"] == "blob")
                .filter_map(|e| Some((e["path"].as_str()?.to_string(), e["size"].as_u64().unwrap_or(0))))
                .filter(|(p, s)| *s <= MAX_FILE && wanted(p) && prefix.as_ref().map(|x| p.starts_with(x.as_str())).unwrap_or(true))
                .filter(|(p, _)| !p.split('/').any(|c| c.starts_with('.') && !crate::build::CONFIG_NAMES.contains(&c) || c == ".." ))
                .collect()
        })
        .unwrap_or_default();
    files.truncate(MAX_FILES);
    let meta = Meta { sha: sha.clone(), date, branch, message, files: files.len() };
    let base = spec.cache_dir(cache).join(meta.short());
    let n = files.len().max(1);
    for (i, (p, size)) in files.iter().enumerate() {
        let to = base.join(p);
        if to.metadata().map(|m| m.len() == *size).unwrap_or(false) {
            continue;
        }
        progress(tf("gh_downloading", &[&p, &(i + 1), &n]), i as f32 / n as f32);
        let data = http.get(&format!("{RAW}/{o}/{r}/{sha}/{}", encode_path(p)), None).or_else(|_| {
            // private repository: the contents API with the token
            let v = json(http, &format!("{API}/repos/{o}/{r}/contents/{}?ref={sha}", encode_path(p)), token).map_err(HttpError::Other)?;
            let b64 = v["content"].as_str().unwrap_or("").replace('\n', "");
            base64_decode(&b64).ok_or(HttpError::Other("base64".into()))
        });
        let data = data.map_err(|e| format!("{p} : {e}"))?;
        if let Some(d) = to.parent() {
            std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
        }
        std::fs::write(&to, data).map_err(|e| format!("{} : {e}", to.display()))?;
    }
    std::fs::create_dir_all(meta.dir(spec, cache)).map_err(|e| e.to_string())?;
    let md = spec.cache_dir(cache);
    std::fs::write(md.join(meta_name(spec)), serde_json::to_vec_pretty(&meta).unwrap_or_default()).map_err(|e| e.to_string())?;
    // older downloads of this project are removed (unless another
    // subfolder of the same repository still uses them)
    let keep: Vec<String> = std::fs::read_dir(&md)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.file_name().to_string_lossy().ends_with(META))
        .filter_map(|e| serde_json::from_slice::<Meta>(&std::fs::read(e.path()).ok()?).ok())
        .map(|m| m.short().to_string())
        .collect();
    for e in std::fs::read_dir(&md).into_iter().flatten().flatten() {
        let name = e.file_name().to_string_lossy().to_string();
        if e.path().is_dir() && !keep.contains(&name) {
            let _ = std::fs::remove_dir_all(e.path());
        }
    }
    progress(t("gh_done").into(), 1.0);
    Ok(meta)
}

fn encode_path(p: &str) -> String {
    p.split('/')
        .map(|c| {
            c.bytes()
                .map(|b| if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) { (b as char).to_string() } else { format!("%{b:02X}") })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn base64_decode(s: &str) -> Option<Vec<u8>> {
    let val = |c: u8| -> Option<u32> {
        Some(match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => return None,
        } as u32)
    };
    let b: Vec<u8> = s.bytes().filter(|c| *c != b'=').collect();
    let mut out = Vec::with_capacity(b.len() * 3 / 4);
    for ch in b.chunks(4) {
        let mut n = 0u32;
        for (i, c) in ch.iter().enumerate() {
            n |= val(*c)? << (18 - 6 * i);
        }
        out.push((n >> 16) as u8);
        if ch.len() > 2 {
            out.push((n >> 8) as u8);
        }
        if ch.len() > 3 {
            out.push(n as u8);
        }
    }
    Some(out)
}

/// Rank of a ready-made program (lower is better): tapes, then binaries
/// and dumps.
fn asset_rank(name: &str) -> Option<u8> {
    let ext = Path::new(name).extension().map(|e| e.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
    match ext.as_str() {
        "wav" | "aiff" | "aif" => Some(0),
        "bin" => Some(1),
        "hex" | "apl" | "txt" => Some(2),
        _ => None,
    }
}

/// What « Tester le dernier build » found.
#[derive(Clone, Debug, PartialEq)]
pub struct Build {
    /// File to start (▶).
    pub path: PathBuf,
    /// Where it comes from (shown in the status bar).
    pub origin: String,
}

/// Latest build of a synced project (`meta`): Release asset, else Actions
/// artifact, else a source or program of the repository.
pub fn latest_build(spec: &Spec, cache: &Path, meta: &Meta, http: &dyn Http, token: Option<&str>, progress: Progress) -> Result<Build, String> {
    let (o, r) = (&spec.owner, &spec.repo);
    let base = spec.cache_dir(cache).join(meta.short());
    let mut notes = Vec::new();
    // 1. latest Release
    progress(t("gh_release").into(), 0.1);
    match json(http, &format!("{API}/repos/{o}/{r}/releases/latest"), token) {
        Ok(rel) => {
            let tag = rel["tag_name"].as_str().unwrap_or("release").to_string();
            let mut assets: Vec<(u8, String, String)> = rel["assets"]
                .as_array()
                .map(|a| a.iter().filter_map(|x| Some((asset_rank(x["name"].as_str()?)?, x["name"].as_str()?.to_string(), x["browser_download_url"].as_str()?.to_string()))).collect())
                .unwrap_or_default();
            assets.sort();
            if let Some((_, name, url)) = assets.first() {
                if !valid(name) || !valid(&tag.replace('/', "_")) {
                    return Err(tf("gh_bad_name", &[name]));
                }
                let dir = base.join("_release").join(tag.replace('/', "_"));
                let to = dir.join(name);
                if !to.is_file() {
                    progress(tf("gh_downloading", &[name, &1, &1]), 0.3);
                    let data = http.get(url, token).map_err(|e| format!("{name} : {e}"))?;
                    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
                    std::fs::write(&to, data).map_err(|e| e.to_string())?;
                }
                return Ok(Build { path: to, origin: tf("gh_from_release", &[&tag, name]) });
            }
            notes.push(tf("gh_release_no_asset", &[&tag]));
        }
        Err(e) => notes.push(format!("{} ({e})", t("gh_no_release"))),
    }
    // 2. latest Actions artifact (download needs a token)
    progress(t("gh_artifacts").into(), 0.4);
    if let Ok(v) = json(http, &format!("{API}/repos/{o}/{r}/actions/artifacts?per_page=20"), token) {
        let art = v["artifacts"].as_array().and_then(|a| a.iter().find(|x| x["expired"] != true).cloned());
        match (art, token) {
            (Some(a), Some(tk)) => {
                let name = a["name"].as_str().unwrap_or("artifact").replace(['/', '\\'], "_");
                let url = a["archive_download_url"].as_str().unwrap_or("").to_string();
                progress(tf("gh_downloading", &[&name, &1, &1]), 0.5);
                match http.get(&url, Some(tk)) {
                    Ok(zip) => {
                        let dir = base.join("_artifact").join(&name);
                        if let Some(p) = extract_best(&zip, &dir)? {
                            return Ok(Build { path: p, origin: tf("gh_from_artifact", &[&name]) });
                        }
                        notes.push(tf("gh_artifact_empty", &[&name]));
                    }
                    Err(e) => notes.push(format!("{name} : {e}")),
                }
            }
            (Some(_), None) => notes.push(t("gh_artifact_token").into()),
            (None, _) => {}
        }
    }
    // 3. the repository itself
    progress(t("gh_sources").into(), 0.8);
    let root = meta.dir(spec, cache);
    let p = pick_source(&root, &base, &spec.repo).ok_or_else(|| {
        let mut e = t("gh_nothing").to_string();
        for n in &notes {
            e.push_str(&format!(" — {n}"));
        }
        e
    })?;
    let origin = tf("gh_from_source", &[&p.strip_prefix(&base).unwrap_or(&p).display(), &meta.short()]);
    Ok(Build { path: p, origin })
}

/// Unzip the supported files of an artifact; returns the best one.
fn extract_best(zip: &[u8], dir: &Path) -> Result<Option<PathBuf>, String> {
    let mut a = zip::ZipArchive::new(std::io::Cursor::new(zip)).map_err(|e| format!("zip : {e}"))?;
    let mut best: Option<(u8, PathBuf)> = None;
    for i in 0..a.len() {
        let mut f = a.by_index(i).map_err(|e| format!("zip : {e}"))?;
        let Some(rel) = f.enclosed_name() else { continue };
        let name = rel.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let Some(rank) = asset_rank(&name) else { continue };
        if f.size() > MAX_FILE * 16 {
            continue;
        }
        let to = dir.join(&rel);
        if let Some(d) = to.parent() {
            std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
        }
        let mut data = Vec::new();
        f.read_to_end(&mut data).map_err(|e| e.to_string())?;
        std::fs::write(&to, data).map_err(|e| e.to_string())?;
        if best.as_ref().map(|b| rank < b.0).unwrap_or(true) {
            best = Some((rank, to));
        }
    }
    Ok(best.map(|b| b.1))
}

/// Program of the repository to start: the `file` of apple1.toml, else the
/// best guess (a name like the repository's, sources before programs).
pub fn pick_source(root: &Path, base: &Path, repo: &str) -> Option<PathBuf> {
    let cfg = crate::build::RepoConfig::find(root);
    if let (Some(f), Some(d)) = (&cfg.file, &cfg.dir) {
        let p = d.join(f);
        if p.is_file() && p.starts_with(base) {
            return Some(p);
        }
    }
    let mut all = Vec::new();
    collect(root, 0, &mut all);
    let repo = repo.to_ascii_lowercase();
    let mut scored: Vec<(u32, String, PathBuf)> = all
        .into_iter()
        .filter_map(|p| {
            let ext = p.extension().map(|e| e.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
            let stem = p.file_stem().map(|s| s.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
            let kind = match crate::library::source_kind(&p, None) {
                Some(crate::library::Source::Asm) => 0,
                Some(crate::library::Source::Basic) => 1,
                Some(crate::library::Source::Art) => return None,
                None if ext == "txt" || ext == "hex" || ext == "apl" => 2,
                None if ext == "wav" || ext == "aiff" || ext == "aif" => 3,
                None if ext == "bin" => 4,
                None => return None,
            };
            // an included file (no .org) is not a program
            if kind == 0 && std::fs::read_to_string(&p).ok().and_then(|t| crate::build::org_of(&t)).is_none() && cfg.load.is_none() {
                return None;
            }
            let name_bonus = if stem == repo || repo.contains(&stem) || stem.contains(&repo) || stem == "main" { 0 } else { 10 };
            let depth = p.components().count() as u32;
            Some((name_bonus + kind * 100 + depth, p.to_string_lossy().to_string(), p))
        })
        .collect();
    scored.sort();
    scored.into_iter().next().map(|s| s.2)
}

fn collect(dir: &Path, depth: usize, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        let name = e.file_name().to_string_lossy().to_string();
        if name.starts_with('.') || name.starts_with('_') {
            continue;
        }
        if p.is_dir() && depth < 8 {
            collect(&p, depth + 1, out);
        } else if p.is_file() && crate::library::is_program(&p) {
            out.push(p);
        }
    }
}

// ------------------------------------------------------------- worker

pub enum Job {
    Sync(Spec),
    Latest(Spec),
}

pub enum Event {
    Progress(Spec, String, f32),
    Synced(Spec, Result<Meta, String>),
    Built(Spec, Result<Build, String>),
}

/// Background thread doing the network jobs one after the other.
pub struct Worker {
    tx: Sender<(Job, Option<String>)>,
    pub rx: Receiver<Event>,
}

impl Worker {
    pub fn new(http: Arc<dyn Http>, cache: PathBuf) -> Worker {
        let (tx, jobs) = channel::<(Job, Option<String>)>();
        let (etx, rx) = channel();
        std::thread::spawn(move || {
            while let Ok((job, token)) = jobs.recv() {
                let tk = token.as_deref();
                match job {
                    Job::Sync(spec) => {
                        let e2 = etx.clone();
                        let s2 = spec.clone();
                        let r = sync(&spec, &cache, http.as_ref(), tk, &move |m, f| {
                            let _ = e2.send(Event::Progress(s2.clone(), m, f));
                        });
                        let _ = etx.send(Event::Synced(spec, r));
                    }
                    Job::Latest(spec) => {
                        let e2 = etx.clone();
                        let s2 = spec.clone();
                        let pr = move |m: String, f: f32| {
                            let _ = e2.send(Event::Progress(s2.clone(), m, f));
                        };
                        // up to date first, so that a source build uses the last commit
                        let r = sync(&spec, &cache, http.as_ref(), tk, &pr).and_then(|meta| {
                            let _ = etx.send(Event::Synced(spec.clone(), Ok(meta.clone())));
                            latest_build(&spec, &cache, &meta, http.as_ref(), tk, &pr)
                        });
                        let _ = etx.send(Event::Built(spec, r));
                    }
                }
            }
        });
        Worker { tx, rx }
    }

    pub fn send(&self, job: Job, token: Option<String>) {
        let _ = self.tx.send((job, token));
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    /// Fake GitHub: URL -> body; records the calls and the token seen.
    #[derive(Default)]
    pub struct Mock {
        pub pages: HashMap<String, Vec<u8>>,
        pub calls: Mutex<Vec<(String, bool)>>,
    }

    impl Http for Mock {
        fn get(&self, url: &str, token: Option<&str>) -> Result<Vec<u8>, HttpError> {
            self.calls.lock().unwrap().push((url.to_string(), token.is_some()));
            self.pages.get(url).cloned().ok_or(HttpError::Status(404))
        }
    }

    pub fn mock_repo() -> Mock {
        let mut m = Mock::default();
        let mut put = |u: &str, b: &[u8]| {
            m.pages.insert(u.to_string(), b.to_vec());
        };
        put(&format!("{API}/repos/woz/jeux"), br#"{"default_branch": "main"}"#);
        put(&format!("{API}/repos/woz/jeux/commits/main"), br#"{"sha": "0123456789abcdef", "commit": {"committer": {"date": "2026-10-20T10:00:00Z"}, "message": "Nouvelle version\n\ndetails"}}"#);
        put(
            &format!("{API}/repos/woz/jeux/git/trees/0123456789abcdef?recursive=1"),
            br#"{"tree": [
                {"path": "README.md", "type": "blob", "size": 10},
                {"path": "src", "type": "tree"},
                {"path": "src/jeux.s", "type": "blob", "size": 19},
                {"path": "src/macros.s", "type": "blob", "size": 10},
                {"path": "basic/demo.bas", "type": "blob", "size": 14},
                {"path": ".github/x.txt", "type": "blob", "size": 5},
                {"path": "apple1.toml", "type": "blob", "size": 13}
            ]}"#,
        );
        put(&format!("{RAW}/woz/jeux/0123456789abcdef/src/jeux.s"), b"  .org $0300\n  rts\n");
        put(&format!("{RAW}/woz/jeux/0123456789abcdef/src/macros.s"), b"; include\n");
        put(&format!("{RAW}/woz/jeux/0123456789abcdef/basic/demo.bas"), b"10 PRINT \"HI\"\n");
        put(&format!("{RAW}/woz/jeux/0123456789abcdef/apple1.toml"), b"load = $0300\n");
        m
    }

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("a1gh-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        d
    }

    #[test]
    fn specs() {
        let s = Spec::parse("https://github.com/jefftranter/6502/tree/master/asm/apple1").unwrap();
        assert_eq!((s.owner.as_str(), s.repo.as_str(), s.branch.as_deref(), s.sub.as_deref()), ("jefftranter", "6502", Some("master"), Some("asm/apple1")));
        assert_eq!(Spec::parse(&s.text()).unwrap(), s);
        let s = Spec::parse(" woz/jeux.git ").unwrap();
        assert_eq!((s.repo.as_str(), s.sub.clone(), s.text()), ("jeux", None, "woz/jeux".to_string()));
        assert_eq!(Spec::parse("woz/jeux/basic").unwrap().sub.as_deref(), Some("basic"));
        assert!(Spec::parse("woz").is_err());
        assert!(Spec::parse("woz/../etc").is_err());
        assert!(Spec::parse("woz/jeux/../../x").is_err());
        assert_eq!(base64_decode("SGVsbG8=").unwrap(), b"Hello");
        assert_eq!(encode_path("a b/c#.s"), "a%20b/c%23.s");
    }

    #[test]
    fn sync_and_latest_build_offline() {
        let cache = tmp("sync");
        let m = mock_repo();
        let spec = Spec::parse("woz/jeux").unwrap();
        let steps = Mutex::new(Vec::new());
        let meta = sync(&spec, &cache, &m, None, &|t, _| steps.lock().unwrap().push(t)).unwrap();
        assert_eq!((meta.short(), meta.files, meta.message.as_str()), ("0123456", 4, "Nouvelle version"));
        let dir = meta.dir(&spec, &cache);
        assert!(dir.join("src/jeux.s").is_file() && dir.join("basic/demo.bas").is_file());
        assert!(!dir.join("README.md").exists() && !dir.join(".github").exists());
        assert_eq!(Meta::load(&spec, &cache).unwrap(), meta);
        // no token: no Authorization anywhere
        assert!(m.calls.lock().unwrap().iter().all(|c| !c.1));
        // second sync: nothing downloaded again
        let n = m.calls.lock().unwrap().len();
        sync(&spec, &cache, &m, None, &|_, _| {}).unwrap();
        assert_eq!(m.calls.lock().unwrap().len(), n + 3);
        // no release, no artifact: the assembly source named like the repository
        let b = latest_build(&spec, &cache, &meta, &m, None, &|_, _| {}).unwrap();
        assert_eq!(b.path, dir.join("src/jeux.s"));
        // with a release holding a tape, the tape wins
        let mut m2 = mock_repo();
        m2.pages.insert(format!("{API}/repos/woz/jeux/releases/latest"), br#"{"tag_name": "v1.0", "assets": [{"name": "jeux.zip", "browser_download_url": "https://x/z"}, {"name": "jeux.wav", "browser_download_url": "https://x/w"}]}"#.to_vec());
        m2.pages.insert("https://x/w".into(), b"RIFF....".to_vec());
        let b = latest_build(&spec, &cache, &meta, &m2, None, &|_, _| {}).unwrap();
        assert!(b.path.ends_with("_release/v1.0/jeux.wav") && b.path.is_file(), "{b:?}");
        // a subfolder only
        let sub = Spec::parse("woz/jeux/basic").unwrap();
        let ms = sync(&sub, &cache, &m, None, &|_, _| {}).unwrap();
        assert_eq!(ms.files, 1);
        assert!(ms.dir(&sub, &cache).join("demo.bas").is_file());
        assert_eq!(pick_source(&ms.dir(&sub, &cache), &sub.cache_dir(&cache).join(ms.short()), "jeux").unwrap().file_name().unwrap(), "demo.bas");
        std::fs::remove_dir_all(&cache).ok();
    }

    #[test]
    fn artifact_needs_token() {
        let cache = tmp("art");
        let mut m = mock_repo();
        m.pages.insert(format!("{API}/repos/woz/jeux/actions/artifacts?per_page=20"), br#"{"artifacts": [{"name": "build", "expired": false, "archive_download_url": "https://api.github.com/a/zip"}]}"#.to_vec());
        // a zip holding build/jeux.bin
        let mut zip = Vec::new();
        {
            let mut w = zip::ZipWriter::new(std::io::Cursor::new(&mut zip));
            w.start_file("out/jeux.bin", zip::write::SimpleFileOptions::default()).unwrap();
            std::io::Write::write_all(&mut w, &[0xA9, 0x00, 0x60]).unwrap();
            w.start_file("../evil.bin", zip::write::SimpleFileOptions::default()).unwrap();
            w.finish().unwrap();
        }
        m.pages.insert("https://api.github.com/a/zip".into(), zip);
        let spec = Spec::parse("woz/jeux").unwrap();
        let meta = sync(&spec, &cache, &m, None, &|_, _| {}).unwrap();
        // without token: the source
        let b = latest_build(&spec, &cache, &meta, &m, None, &|_, _| {}).unwrap();
        assert!(b.path.ends_with("src/jeux.s"));
        let b = latest_build(&spec, &cache, &meta, &m, Some("secret"), &|_, _| {}).unwrap();
        assert!(b.path.ends_with("_artifact/build/out/jeux.bin"), "{b:?}");
        assert!(!cache.join("woz/jeux/0123456/_artifact/evil.bin").exists());
        assert!(!b.origin.contains("secret"));
        std::fs::remove_dir_all(&cache).ok();
    }

    /// Real GitHub (network): run with `cargo test -- --ignored`.
    #[test]
    #[ignore]
    fn real_repository() {
        let cache = tmp("real");
        let spec = Spec::parse("https://github.com/jefftranter/6502/tree/master/asm/jmon").unwrap();
        let http = Ureq::new();
        let meta = sync(&spec, &cache, &http, None, &|t, _| eprintln!("{t}")).unwrap();
        eprintln!("{meta:?}");
        assert!(meta.files > 0);
        let b = latest_build(&spec, &cache, &meta, &http, None, &|t, _| eprintln!("{t}")).unwrap();
        eprintln!("{b:?}");
        assert!(b.path.ends_with("jmon.s"));
        // JMON needs a recent ca65 (cpu.mac, .cpu); the result is only shown
        if crate::build::find_cc65().is_some() {
            eprintln!("{:?}", crate::build::assemble(&b.path).map(|(p, run)| (p.total_len(), run)).map_err(|e| e.lines().next().unwrap_or("").to_string()));
        }
    }
}
