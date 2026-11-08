//! Interface languages (fr, en, de, ru, es, it, pt) and detection of the
//! default one from the active keyboard layout.
//!
//! The strings live in `i18n/strings.txt`: one block per key, a line per
//! language (`fr …`, `en …`). `\n` is a line break, `{0}`, `{1}`… are the
//! arguments of [`tf`]. A missing translation falls back to English, then
//! French.

use std::collections::HashMap;
use std::fmt::Display;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    Fr,
    En,
    De,
    Ru,
    Es,
    It,
    Pt,
}

pub const ALL: [Lang; 7] = [Lang::Fr, Lang::En, Lang::De, Lang::Ru, Lang::Es, Lang::It, Lang::Pt];

impl Lang {
    pub fn code(self) -> &'static str {
        ["fr", "en", "de", "ru", "es", "it", "pt"][self as usize]
    }
    pub fn from_code(s: &str) -> Option<Lang> {
        let s = s.trim().to_ascii_lowercase();
        ALL.into_iter().find(|l| s == l.code() || s.starts_with(&format!("{}_", l.code())) || s.starts_with(&format!("{}-", l.code())))
    }
    /// Name of the language in that language.
    pub fn native(self) -> &'static str {
        ["Français", "English", "Deutsch", "Русский", "Español", "Italiano", "Português"][self as usize]
    }
}

static CURRENT: AtomicUsize = AtomicUsize::new(Lang::En as usize);

pub fn set(l: Lang) {
    CURRENT.store(l as usize, Ordering::Relaxed);
}

pub fn current() -> Lang {
    ALL[CURRENT.load(Ordering::Relaxed).min(ALL.len() - 1)]
}

const SOURCE: &str = include_str!("i18n/strings.txt");

type Table = HashMap<&'static str, [Option<String>; 7]>;

fn parse(src: &'static str) -> Result<Table, String> {
    let mut t: Table = HashMap::new();
    let mut key: Option<&'static str> = None;
    for (n, line) in src.lines().enumerate() {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(k) = line.strip_prefix('@') {
            if t.insert(k.trim(), Default::default()).is_some() {
                return Err(format!("ligne {} : clé {k} en double", n + 1));
            }
            key = Some(k.trim());
            continue;
        }
        let k = key.ok_or(format!("ligne {} : texte hors clé", n + 1))?;
        let (code, text) = line.split_at(line.find(' ').unwrap_or(line.len()));
        let l = ALL.into_iter().find(|l| l.code() == code).ok_or(format!("ligne {} : langue « {code} »", n + 1))?;
        let slot = &mut t.get_mut(k).unwrap()[l as usize];
        if slot.is_some() {
            return Err(format!("ligne {} : {code} en double pour {k}", n + 1));
        }
        *slot = Some(text.trim_start().replace("\\n", "\n"));
    }
    Ok(t)
}

fn table() -> &'static Table {
    static T: OnceLock<Table> = OnceLock::new();
    T.get_or_init(|| parse(SOURCE).unwrap_or_else(|e| panic!("i18n/strings.txt : {e}")))
}

/// Translation of `key` in `l`.
pub fn t_in(l: Lang, key: &str) -> &'static str {
    match table().get(key) {
        Some(v) => v[l as usize].as_deref().or(v[Lang::En as usize].as_deref()).or(v[Lang::Fr as usize].as_deref()).unwrap_or("?"),
        None => {
            debug_assert!(false, "clé de traduction inconnue : {key}");
            "?"
        }
    }
}

/// Translation of `key` in the current language.
pub fn t(key: &str) -> &'static str {
    t_in(current(), key)
}

/// Translation with arguments `{0}`, `{1}`…
pub fn tf(key: &str, args: &[&dyn Display]) -> String {
    fill(t(key), args)
}

pub fn fill(s: &str, args: &[&dyn Display]) -> String {
    let mut out = s.to_string();
    for (i, a) in args.iter().enumerate() {
        out = out.replace(&format!("{{{i}}}"), &a.to_string());
    }
    out
}

// ------------------------------------------------------------- detection

/// Language chosen automatically, and where it came from (e.g.
/// "setxkbmap : fr").
#[derive(Clone, Debug, PartialEq)]
pub struct Detected {
    pub lang: Lang,
    pub source: String,
}

/// Language of a keyboard layout name: `fr`, `fr(azerty)`, `fr+oss`,
/// `ch(fr)`, `us`, `ru`… None when the layout says nothing (unknown).
pub fn lang_from_layout(layout: &str) -> Option<Lang> {
    let l = layout.trim().trim_matches(|c| c == '\'' || c == '"').to_ascii_lowercase();
    let (base, variant) = match l.find(['(', '+', '\t', ' ']) {
        Some(i) => (&l[..i], l[i + 1..].trim_end_matches(')')),
        None => (l.as_str(), ""),
    };
    Some(match base {
        "fr" | "be" => Lang::Fr,
        "ca" if variant.is_empty() || variant == "fr" || variant.starts_with("fr-") || variant == "multix" => Lang::Fr,
        "ch" if variant.starts_with("fr") => Lang::Fr,
        "ch" if variant.starts_with("it") => Lang::It,
        "de" | "at" | "ch" | "li" => Lang::De,
        "ru" => Lang::Ru,
        "es" | "latam" => Lang::Es,
        "it" | "sm" => Lang::It,
        "pt" | "br" => Lang::Pt,
        "us" | "gb" | "uk" | "ie" | "au" | "nz" | "za" | "ca" => Lang::En,
        _ => return None,
    })
}

/// Language of a locale value: `fr_FR.UTF-8`, `ru_RU`, `de`… (C/POSIX: None).
pub fn lang_from_locale(v: &str) -> Option<Lang> {
    let first = v.split(':').next().unwrap_or("").trim();
    if first.is_empty() || first.starts_with('C') || first == "POSIX" {
        return None;
    }
    Lang::from_code(first)
}

/// First xkb layout of a gsettings list: `[('xkb', 'fr+oss'), ('xkb', 'us')]`
/// (GNOME input-sources) or `['fr\tazerty', 'us']` (libgnomekbd, Cinnamon).
fn first_gsettings_layout(s: &str) -> Option<String> {
    let s = s.trim();
    if s.contains("('") {
        // tuples: take the first ('xkb', 'xx') one
        s.split("('").skip(1).find_map(|t| {
            let mut parts = t.split('\'');
            let kind = parts.next()?;
            let _ = parts.next();
            let val = parts.next()?;
            (kind == "xkb").then(|| val.to_string())
        })
    } else {
        s.split('\'').nth(1).map(|v| v.replace("\\t", "\t"))
    }
}

fn layout_line(out: &str, label: &str) -> Option<String> {
    out.lines().find_map(|l| {
        let l = l.trim();
        let rest = l.strip_prefix(label)?;
        let v = rest.trim_start_matches(':').trim().trim_matches('"');
        let first = v.split(',').next()?.trim();
        (!first.is_empty()).then(|| first.to_string())
    })
}

/// Detection with injectable sources (tests). `run(cmd, args)` returns the
/// standard output of a command, `env(name)` a variable, `file(path)` a file.
pub fn detect_with(run: &dyn Fn(&str, &[&str]) -> Option<String>, env: &dyn Fn(&str) -> Option<String>, file: &dyn Fn(&str) -> Option<String>) -> Detected {
    let mut tried: Vec<(String, String)> = Vec::new();
    let mut found = |src: &str, layout: Option<String>| -> Option<Detected> {
        let layout = layout?;
        let lang = lang_from_layout(&layout);
        tried.push((src.to_string(), layout.clone()));
        lang.map(|lang| Detected { lang, source: format!("{src} : {}", layout.replace('\t', " ")) })
    };
    let gs = |schema: &str, k: &str| run("gsettings", &["get", schema, k]).and_then(|o| first_gsettings_layout(&o));
    // 1. GNOME: most recently used source = the active one
    if let Some(d) = found("gsettings mru-sources", gs("org.gnome.desktop.input-sources", "mru-sources")) {
        return d;
    }
    // 2. X server (Cinnamon, MATE, Xfce on X11; XWayland under Wayland)
    if let Some(d) = found("setxkbmap -query", run("setxkbmap", &["-query"]).and_then(|o| layout_line(&o, "layout"))) {
        return d;
    }
    // 3. Cinnamon keyboard applet (libgnomekbd), then GNOME sources
    if let Some(d) = found("gsettings libgnomekbd", gs("org.gnome.libgnomekbd.keyboard", "layouts")) {
        return d;
    }
    if let Some(d) = found("gsettings input-sources", gs("org.gnome.desktop.input-sources", "sources")) {
        return d;
    }
    // 4. system keyboard
    if let Some(d) = found("localectl", run("localectl", &["status"]).and_then(|o| layout_line(&o, "X11 Layout"))) {
        return d;
    }
    if let Some(d) = found("/etc/default/keyboard", file("/etc/default/keyboard").and_then(|o| layout_line(&o, "XKBLAYOUT="))) {
        return d;
    }
    // 5. locale
    for v in ["LC_ALL", "LC_MESSAGES", "LANG", "LANGUAGE"] {
        if let Some(val) = env(v) {
            if let Some(lang) = lang_from_locale(&val) {
                return Detected { lang, source: format!("{v}={val}") };
            }
        }
    }
    let note = tried.iter().map(|(s, l)| format!("{s} : {l}")).collect::<Vec<_>>().join(", ");
    Detected { lang: Lang::En, source: if note.is_empty() { "défaut".into() } else { format!("défaut ({note})") } }
}

/// Run a command for at most 1.5 s, return its output when it succeeds.
fn run_cmd(cmd: &str, args: &[&str]) -> Option<String> {
    let mut child = std::process::Command::new(cmd)
        .args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .ok()?;
    let t0 = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(st)) => {
                if !st.success() {
                    return None;
                }
                let mut s = String::new();
                use std::io::Read;
                child.stdout.take()?.read_to_string(&mut s).ok()?;
                return Some(s);
            }
            Ok(None) if t0.elapsed() < Duration::from_millis(1500) => std::thread::sleep(Duration::from_millis(10)),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
        }
    }
}

pub fn detect() -> Detected {
    detect_with(&run_cmd, &|k| std::env::var(k).ok().filter(|v| !v.is_empty()), &|p| std::fs::read_to_string(p).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_complete() {
        let t = parse(SOURCE).unwrap();
        for (k, v) in &t {
            // fr, en, de, ru are required; es, it, pt too (all provided)
            for l in ALL {
                assert!(v[l as usize].is_some(), "{k} : {} manquant", l.code());
            }
            let count = |s: &str| (0..6).filter(|i| s.contains(&format!("{{{i}}}"))).count();
            let n = count(v[0].as_ref().unwrap());
            for l in ALL {
                assert_eq!(count(v[l as usize].as_ref().unwrap()), n, "{k} : arguments différents en {}", l.code());
            }
        }
    }

    /// Help mode: every `.hint(…, "h_…")` of the window has its
    /// explanation in the 7 languages.
    #[test]
    fn every_help_key_in_all_languages() {
        let t = parse(SOURCE).unwrap();
        let src = concat!(include_str!("app.rs"), include_str!("ascii.rs"));
        let mut n = 0;
        for (i, _) in src.match_indices(".hint(") {
            // calls are on one line: .hint(help, short, "h_key")
            let call = &src[i..i + src[i..].find('\n').unwrap_or(src.len() - i)];
            if call.contains(", hid)") {
                continue; // the key comes from a table: checked below by its "h_…" literals
            }
            let k = call.find("\"h_").map(|s| &call[s + 1..]).and_then(|r| r.split('"').next());
            let key = k.unwrap_or_else(|| panic!("aide manquante : {call}"));
            let v = t.get(key).unwrap_or_else(|| panic!("clé d'aide absente : {key}"));
            for l in ALL {
                assert!(v[l as usize].as_deref().is_some_and(|x| x.len() > 8), "{key} : {} manquant", l.code());
            }
            n += 1;
        }
        assert!(n >= 60, "{n}");
    }

    #[test]
    fn every_key_used_in_the_code_exists() {
        let t = parse(SOURCE).unwrap();
        let src = [include_str!("app.rs"), include_str!("session.rs"), include_str!("library.rs"), include_str!("screen.rs"), include_str!("gen2screen.rs"), include_str!("params.rs"), include_str!("github.rs"), include_str!("build.rs"), include_str!("launch.rs"), include_str!("editor.rs"), include_str!("dock.rs"), include_str!("debugger.rs")].concat();
        let mut n = 0;
        for pat in ["t(\"", "tf(\"", "t_in(l, \""] {
            for (i, _) in src.match_indices(pat) {
                // skip e.g. on_hover_text("F11")
                if src[..i].chars().last().is_some_and(|c| c.is_alphanumeric() || c == '_') {
                    continue;
                }
                let rest = &src[i + pat.len()..];
                let key = &rest[..rest.find('"').unwrap()];
                assert!(t.contains_key(key), "clé absente de strings.txt : {key}");
                n += 1;
            }
        }
        assert!(n > 100, "{n}");
    }

    /// Messages of the assembler errors and help of the editor.
    #[test]
    fn assembler_keys() {
        let t = parse(SOURCE).unwrap();
        for k in apple1_core::asm6502::ErrKind::ALL {
            let key = format!("as_e_{}", k.key());
            assert!(t.contains_key(key.as_str()), "{key}");
        }
        let ed = [include_str!("editor.rs"), include_str!("dock.rs"), include_str!("debugger.rs")].concat();
        for (i, _) in ed.match_indices("\"h_") {
            let key = ed[i + 1..].split('"').next().unwrap();
            let v = t.get(key).unwrap_or_else(|| panic!("clé d'aide absente : {key}"));
            for l in ALL {
                assert!(v[l as usize].as_deref().is_some_and(|x| x.len() > 8), "{key} : {} manquant", l.code());
            }
        }
    }

    #[test]
    fn layouts_and_locales() {
        assert_eq!(lang_from_layout("fr"), Some(Lang::Fr));
        assert_eq!(lang_from_layout("fr(azerty)"), Some(Lang::Fr));
        assert_eq!(lang_from_layout("fr+oss"), Some(Lang::Fr));
        assert_eq!(lang_from_layout("fr\tazerty"), Some(Lang::Fr));
        assert_eq!(lang_from_layout("ch(fr)"), Some(Lang::Fr));
        assert_eq!(lang_from_layout("ch"), Some(Lang::De));
        assert_eq!(lang_from_layout("de(nodeadkeys)"), Some(Lang::De));
        assert_eq!(lang_from_layout("ru"), Some(Lang::Ru));
        assert_eq!(lang_from_layout("br(abnt2)"), Some(Lang::Pt));
        assert_eq!(lang_from_layout("latam"), Some(Lang::Es));
        assert_eq!(lang_from_layout("us"), Some(Lang::En));
        assert_eq!(lang_from_layout("ca(eng)"), Some(Lang::En));
        assert_eq!(lang_from_layout("jp"), None);
        assert_eq!(lang_from_locale("fr_FR.UTF-8"), Some(Lang::Fr));
        assert_eq!(lang_from_locale("ru_RU.UTF-8"), Some(Lang::Ru));
        assert_eq!(lang_from_locale("C.UTF-8"), None);
        assert_eq!(lang_from_locale("pt_BR:en"), Some(Lang::Pt));
    }

    fn fake<'a>(cmds: &'a [(&'a str, &'a str)]) -> impl Fn(&str, &[&str]) -> Option<String> + 'a {
        move |c: &str, a: &[&str]| {
            let full = format!("{c} {}", a.join(" "));
            cmds.iter().find(|(k, _)| full.starts_with(k)).map(|(_, v)| v.to_string())
        }
    }

    #[test]
    fn detection_order() {
        let none = |_: &str| None;
        // Linux Mint Cinnamon (X11): setxkbmap gives the layout
        let d = detect_with(&fake(&[("setxkbmap -query", "rules:      evdev\nmodel:      pc105\nlayout:     fr\nvariant:    azerty\n")]), &none, &none);
        assert_eq!(d, Detected { lang: Lang::Fr, source: "setxkbmap -query : fr".into() });
        // several layouts: the first one
        let d = detect_with(&fake(&[("setxkbmap -query", "layout:     ru,us\n")]), &none, &none);
        assert_eq!(d.lang, Lang::Ru);
        // GNOME: the most recently used source wins over setxkbmap
        let d = detect_with(
            &fake(&[("gsettings get org.gnome.desktop.input-sources mru-sources", "[('xkb', 'de'), ('xkb', 'us')]\n"), ("setxkbmap -query", "layout: us\n")]),
            &none,
            &none,
        );
        assert_eq!(d.lang, Lang::De);
        // Cinnamon applet list (no X server answer)
        let d = detect_with(&fake(&[("gsettings get org.gnome.desktop.input-sources mru-sources", "@a(ss) []\n"), ("gsettings get org.gnome.libgnomekbd.keyboard layouts", "['es', 'us']\n")]), &none, &none);
        assert_eq!(d.lang, Lang::Es);
        // localectl, then /etc/default/keyboard
        let d = detect_with(&fake(&[("localectl status", "   System Locale: LANG=en_US.UTF-8\n       X11 Layout: it\n")]), &none, &none);
        assert_eq!(d.lang, Lang::It);
        let f = |p: &str| (p == "/etc/default/keyboard").then(|| "XKBMODEL=\"pc105\"\nXKBLAYOUT=\"pt\"\n".to_string());
        assert_eq!(detect_with(&fake(&[]), &none, &f).lang, Lang::Pt);
        // unknown layout: LANG, then English
        let env = |k: &str| (k == "LANG").then(|| "ru_RU.UTF-8".to_string());
        let d = detect_with(&fake(&[("setxkbmap -query", "layout: jp\n")]), &env, &none);
        assert_eq!(d, Detected { lang: Lang::Ru, source: "LANG=ru_RU.UTF-8".into() });
        assert_eq!(detect_with(&fake(&[]), &none, &none).lang, Lang::En);
    }

    #[test]
    fn english_layout_messages_have_no_french_quotes() {
        for key in ["ly_applied", "ly_saved", "ly_renamed", "ly_deleted", "wl_intro", "wl_title", "wl_again"] {
            let s = t_in(Lang::En, key);
            assert!(!s.contains('«') && !s.contains('»'), "{key}: {s}");
        }
    }

    #[test]
    fn fill_arguments() {
        assert_eq!(fill("{0} + {1} = {0}", &[&1, &"x"]), "1 + x = 1");
    }
}
