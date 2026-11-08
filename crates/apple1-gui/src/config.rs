//! Settings saved in ~/.config/apple1-emu/gui.json.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::crt::Screening;
use crate::screen::Phosphor;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Zoom {
    /// Largest integer zoom that fits the window.
    Ajuster,
    /// Screen points per Apple-1 dot (1..=8).
    Fixe(u32),
}

pub const ZOOM_MAX: u32 = 8;

/// A machine state (.a1state) saved or loaded lately, for the « État » menu.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RecentState {
    pub title: String,
    pub path: PathBuf,
    /// Date shown in the menu (« 26/10 21:24 »).
    pub when: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct GuiConfig {
    pub zoom: Zoom,
    pub phosphor: Phosphor,
    pub scanlines: bool,
    /// « Écran vintage (screening) »: imitation of a CRT monitor.
    pub screening: Screening,
    /// Inner window size in points (None = first run: 80% of the monitor).
    pub window: Option<[f32; 2]>,
    pub maximized: bool,
    pub library: PathBuf,
    pub show_library: bool,
    pub fast: bool,
    pub throttle: bool,
    pub basic: Option<PathBuf>,
    /// Built-in RDK G2 Basic (true) or the user's original Integer BASIC file (false).
    pub basic_g2: bool,
    /// Folder last used by the file dialogs.
    pub last_dir: Option<PathBuf>,
    /// Uncle Bernie's GEN2 colour graphics card (second, colour monitor).
    pub carte_couleur: bool,
    /// The Apple-1 screen / the GEN2 screen shown (the « Fenêtres » group of the bar).
    pub screen_open: bool,
    /// « Ne plus afficher » of the workspace window shown at start.
    pub welcome_hide: bool,
    pub color_open: bool,
    /// P-LAB graphic card (TMS9918A) plugged in.
    pub carte_plab: bool,
    /// Its screen shown.
    pub plab_open: bool,
    /// IRQ/NMI jumper of the card (true = NMI).
    pub plab_nmi: bool,
    /// CodeTank EEPROM image (16 KB) installed on the card.
    pub plab_rom: Option<PathBuf>,
    /// 50 Hz jumper of the GEN2 card (312 lines).
    pub couleur_50hz: bool,
    /// Interface language: "auto" (keyboard layout, then locale) or fr, en, de, ru, es, it, pt.
    pub langue: String,
    /// Library loads through the simulated cassette (ACI) instead of
    /// instantly: command line only (--mode-cassette), never read from or
    /// written to gui.json (fast load is the implicit default since v8).
    #[serde(skip)]
    pub cassette_mode: bool,
    /// More library folders (the first one is `library`).
    pub libraries: Vec<PathBuf>,
    /// GitHub projects shown in the library ("owner/repo" or "owner/repo/sub/folder").
    pub github: Vec<String>,
    /// Latest machine states (newest first) for the « État » menu.
    pub states: Vec<RecentState>,
    /// GitHub token (optional, for private repos and Actions artifacts);
    /// gui.json is then readable by you only (0600). GITHUB_TOKEN wins.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub github_token: String,
    /// Folders of the library tree: open (true) or closed.
    pub folders_open: BTreeMap<String, bool>,
    /// Library panel on the right of the screen (else on the left).
    pub library_right: bool,
    /// Width of the library panel (points).
    pub library_width: f32,
    /// Help mode: hovering a control explains it.
    pub help_mode: bool,
    /// Hear the cassette signal while it is read or written.
    pub tape_sound: bool,
    /// Volume of the cassette sound (0..1, low by default).
    pub tape_volume: f32,
    /// Oscilloscope window showing the cassette signal.
    pub tape_scope: bool,
    /// « Éditeur BASIC » window: open?, text size, memory, tabs.
    pub editor: crate::editor::EditorCfg,
    /// Editor docked on the left, or in its own window.
    pub editor_dock: crate::dock::DockCfg,
    /// « Débogueur » panel (right) or window, its watches.
    pub debugger: crate::debugger::DbgCfg,
    /// Apple-1 screen in the main window or in its own window.
    pub screen_dock: crate::dock::DockCfg,
    /// « Ajuster » zoom by whole steps only (sharp dots), else fills.
    pub integer_scale: bool,
    /// GEN2 colour screen and library: separate window or not.
    pub color_dock: crate::dock::DockCfg,
    pub plab_dock: crate::dock::DockCfg,
    pub library_dock: crate::dock::DockCfg,
    /// Panes of the main window (crate::layout), and its version.
    #[serde(with = "crate::layout::saved", skip_serializing_if = "Option::is_none", default)]
    pub layout: Option<egui_dock::DockState<crate::layout::Pane>>,
    pub layout_version: u32,
    /// Layouts saved by the user (« ⊞ Disposition »), and the last one chosen (a preset key like "game", or "u:NAME").
    pub saved_layouts: Vec<crate::layout::SavedLayout>,
    pub layout_choice: String,
    /// « Photo → ASCII » tool.
    pub ascii: crate::ascii::AsciiCfg,
    /// Additions made once to an existing setup: 1 (v7.7) = the « Art
    /// ASCII » folder in the library and the dschmenk/Apple1-Slideshow
    /// GitHub project.
    pub extras: u32,
    /// GEN2 colour screen: its own zoom and vintage switch (the fine
    /// vintage settings are shared with the Apple-1 screen).
    pub zoom2: Zoom,
    pub screening2_on: bool,
    /// Character generator curly braces: true = fat (V1, colour CRT), false = thin (V2, mono).
    pub gen2_fat_braces: bool,
    /// Interface: theme, icon size (scale of the whole interface),
    /// tooltips, icons-only toolbar when the window is narrow.
    pub theme: Theme,
    pub icon_size: IconSize,
    pub tooltips: bool,
    pub compact_bar: bool,
    /// Editor: syntax colours of BASIC and of 6502 assembly.
    pub syntax: crate::syntax::SyntaxCfg,
    /// Dump, debugger and button colours per theme.
    pub ui_colors: crate::uicolors::UiColors,
    /// Kit release that last saved this file ("" in a gui.json from before 1.0, then numbered 7.9.3).
    pub kit_version: String,
    /// Text size of the library panel.
    #[serde(default = "crate::fontsz::default_size")]
    pub library_font: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    Sombre,
    Clair,
    Systeme,
}

/// Size of the icons and text, chosen by the user (never derived from the
/// monitor: a scale computed from the screen made the window flip between two
/// sizes on some systems). `Auto` is only read from an old gui.json: 100 %.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum IconSize {
    Auto,
    Petite,
    #[default]
    Normale,
    Grande,
    TresGrande,
}

impl IconSize {
    pub const ALL: [IconSize; 4] = [IconSize::Petite, IconSize::Normale, IconSize::Grande, IconSize::TresGrande];

    pub fn key(self) -> &'static str {
        match self {
            IconSize::Auto => "pr_icons_auto",
            IconSize::Petite => "pr_icons_small",
            IconSize::Normale => "pr_icons_normal",
            IconSize::Grande => "pr_icons_large",
            IconSize::TresGrande => "pr_icons_xlarge",
        }
    }

    /// Zoom of the interface, on top of the system's pixels per point: a
    /// constant of the setting, applied when the user changes it.
    pub fn zoom(self) -> f32 {
        match self {
            IconSize::Petite => 0.85,
            IconSize::Normale | IconSize::Auto => 1.0,
            IconSize::Grande => 1.35,
            IconSize::TresGrande => 1.8,
        }
    }
}

/// Current value of `GuiConfig::extras`.
pub const EXTRAS: u32 = 2;
/// GitHub projects added by default.
pub const DEFAULT_GITHUB: [&str; 1] = ["dschmenk/Apple1-Slideshow"];
/// Kit release written into gui.json (a different saved value offers a fresh start).
/// 1.0 is the first public release (it was 7.9.3 while the repository was private).
pub const KIT_VERSION: &str = "1.0";

pub fn home() -> PathBuf {
    std::env::var_os("HOME").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."))
}

impl Default for GuiConfig {
    fn default() -> Self {
        GuiConfig {
            zoom: Zoom::Ajuster,
            phosphor: Phosphor::Vert,
            scanlines: false,
            screening: Screening::default(),
            window: None,
            maximized: false,
            library: default_library(),
            show_library: true,
            fast: false,
            throttle: true,
            basic: None,
            basic_g2: true,
            last_dir: None,
            carte_couleur: false,
            screen_open: true,
            welcome_hide: false,
            color_open: true,
            carte_plab: false,
            plab_open: true,
            plab_nmi: false,
            plab_rom: None,
            couleur_50hz: false,
            langue: "auto".into(),
            cassette_mode: false,
            libraries: Vec::new(),
            github: Vec::new(),
            states: Vec::new(),
            github_token: String::new(),
            folders_open: BTreeMap::new(),
            library_right: true,
            library_width: 320.0,
            help_mode: false,
            tape_sound: true,
            tape_volume: 0.06,
            tape_scope: true,
            editor: Default::default(),
            editor_dock: Default::default(),
            debugger: Default::default(),
            screen_dock: crate::dock::DockCfg { detached: false, width: 900.0, pos: None, size: Some([900.0, 640.0]) },
            integer_scale: true,
            color_dock: crate::dock::DockCfg { detached: false, width: 600.0, pos: None, size: Some([700.0, 560.0]) },
            plab_dock: crate::dock::DockCfg { detached: false, width: 600.0, pos: None, size: Some([700.0, 560.0]) },
            library_dock: crate::dock::DockCfg { detached: false, width: 320.0, pos: None, size: Some([360.0, 700.0]) },
            layout: None,
            layout_version: 0,
            saved_layouts: Vec::new(),
            layout_choice: "default".into(),
            ascii: Default::default(),
            extras: 0,
            zoom2: Zoom::Ajuster,
            screening2_on: true,
            gen2_fat_braces: true,
            theme: Theme::Sombre,
            icon_size: IconSize::Normale,
            tooltips: true,
            compact_bar: true,
            syntax: Default::default(),
            ui_colors: Default::default(),
            kit_version: String::new(),
            library_font: crate::fontsz::DEFAULT,
        }
    }
}

/// Folder of a portable kit: the directory of `exe` when `APPLE1_PORTABLE` is
/// set or that directory contains a file named `portable`. Preferences
/// (`config/gui.json`), the default library (`programmes`) and the cache
/// then live there, so replacing the kit folder starts clean.
pub fn portable_dir(exe: Option<&std::path::Path>, forced: bool) -> Option<PathBuf> {
    let dir = exe.and_then(|p| p.parent())?;
    // the kit puts the programs in `<kit>/bin/x86_64` or `<kit>/bin/i686`: its root is two levels up,
    // next to run.sh, and that is where a user puts the `portable` file (#54)
    let root = dir.parent().filter(|b| b.file_name().is_some_and(|n| n == "bin")).and_then(|b| b.parent());
    if dir.join("portable").is_file() {
        Some(dir.to_path_buf())
    } else if let Some(r) = root.filter(|r| forced || r.join("portable").is_file()) {
        Some(r.to_path_buf())
    } else if forced {
        Some(dir.to_path_buf())
    } else {
        None
    }
}

fn portable_forced() -> bool {
    std::env::var_os("APPLE1_PORTABLE").is_some_and(|v| {
        let s = v.to_string_lossy();
        !s.is_empty() && s != "0" && !s.eq_ignore_ascii_case("false")
    })
}

pub fn portable_root() -> Option<PathBuf> {
    portable_dir(std::env::current_exe().ok().as_deref(), portable_forced())
}

/// `~/apple1/programmes`, or `<kit>/programmes` in portable mode.
pub fn default_library() -> PathBuf {
    match portable_root() {
        Some(d) => d.join("programmes"),
        None => home().join("apple1").join("programmes"),
    }
}

/// `~/.cache/apple1-emu`, or `<kit>/cache` in portable mode.
pub fn cache_base() -> PathBuf {
    match portable_root() {
        Some(d) => d.join("cache"),
        None => match std::env::var_os("XDG_CACHE_HOME") {
            Some(d) if !d.is_empty() => PathBuf::from(d),
            _ => home().join(".cache"),
        }
        .join("apple1-emu"),
    }
}

pub fn config_dir() -> PathBuf {
    if let Some(d) = portable_root() {
        return d.join("config");
    }
    match std::env::var_os("XDG_CONFIG_HOME") {
        Some(d) if !d.is_empty() => PathBuf::from(d),
        _ => home().join(".config"),
    }
    .join("apple1-emu")
}

/// Write a file readable by its owner only (it may hold a token).
pub fn write_private(p: &std::path::Path, data: &[u8]) -> std::io::Result<()> {
    use std::io::Write;
    #[cfg(unix)]
    {
        use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
        let mut f = std::fs::OpenOptions::new().write(true).create(true).truncate(true).mode(0o600).open(p)?;
        f.set_permissions(std::fs::Permissions::from_mode(0o600))?;
        f.write_all(data)
    }
    #[cfg(not(unix))]
    {
        std::fs::File::create(p)?.write_all(data)
    }
}

impl GuiConfig {
    /// Default settings, keeping the library folders, the GitHub projects and
    /// the token. Files on disk are not touched.
    pub fn fresh_keeping_library(old: &Self) -> Self {
        Self {
            library: old.library.clone(),
            libraries: old.libraries.clone(),
            github: old.github.clone(),
            github_token: old.github_token.clone(),
            extras: old.extras.max(EXTRAS),
            kit_version: KIT_VERSION.into(),
            ..Self::default()
        }
    }

    /// All library folders, the main one first, without duplicates.
    pub fn library_dirs(&self) -> Vec<PathBuf> {
        let mut v = vec![self.library.clone()];
        for d in &self.libraries {
            if !v.contains(d) {
                v.push(d.clone());
            }
        }
        v
    }

    /// Token for GitHub: GITHUB_TOKEN, else the one saved in the preferences.
    pub fn token(&self) -> Option<String> {
        std::env::var("GITHUB_TOKEN").ok().filter(|t| !t.trim().is_empty()).or_else(|| Some(self.github_token.clone()).filter(|t| !t.trim().is_empty())).map(|t| t.trim().to_string())
    }

    pub fn path() -> PathBuf {
        config_dir().join("gui.json")
    }

    /// (config, true if it was read from disk)
    pub fn load() -> (Self, bool) {
        match std::fs::read_to_string(Self::path()) {
            Ok(s) => {
                let mut c: GuiConfig = serde_json::from_str(&s).unwrap_or_default();
                c.screening.migrate();
                (c, true)
            }
            Err(_) => (Self::default(), false),
        }
    }

    pub fn save(&self) {
        let p = Self::path();
        if let Some(d) = p.parent() {
            let _ = std::fs::create_dir_all(d);
        }
        if let Ok(s) = serde_json::to_string_pretty(self) {
            let _ = write_private(&p, s.as_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crt::SCREENING_VERSION;

    fn parse(json: &str) -> GuiConfig {
        let mut c: GuiConfig = serde_json::from_str(json).unwrap();
        c.screening.migrate();
        c
    }

    #[test]
    fn screening_round_trip_and_old_files() {
        let mut c = GuiConfig::default();
        let d = c.screening;
        assert!(d.on && d.gaps && d.soften && d.glow && d.curvature && !d.flicker);
        assert!(d.afterglow > 0.0 && d.afterglow < 0.5 && d.curve > 0.0 && d.curve < 0.5);
        c.screening.on = true;
        c.screening.intensity = 0.3;
        c.screening.curvature = true;
        c.screening.curve = 0.8;
        c.screening.afterglow = 0.9;
        c.screening.glow = false;
        let json = serde_json::to_string(&c).unwrap();
        assert!(!json.contains("persistence"));
        assert_eq!(parse(&json).screening, c.screening, "choices of the new version are kept");
        // a gui.json from before the screening, or with a partial section
        assert_eq!(parse(r#"{"scanlines": true}"#).screening, Screening::default());
        let part = parse(r#"{"screening": {"on": true}}"#);
        assert!(part.screening.on && part.screening.gaps && part.screening.glow);
    }

    #[test]
    fn library_v6_fields() {
        // a gui.json of the v5: library on the right, no other folder
        let c = parse(r#"{"library": "/tmp/x", "show_library": true}"#);
        assert!(c.library_right && c.libraries.is_empty() && c.github.is_empty() && !c.help_mode);
        assert_eq!(c.library_dirs(), vec![PathBuf::from("/tmp/x")]);
        let mut c = c;
        c.libraries = vec![PathBuf::from("/tmp/y"), PathBuf::from("/tmp/x")];
        assert_eq!(c.library_dirs().len(), 2);
        // the token is not written when empty
        assert!(!serde_json::to_string(&c).unwrap().contains("github_token"));
    }

    #[test]
    fn v8_interface_fields() {
        // a gui.json from before v8: defaults, cassette mode never read
        let c = parse(r#"{"cassette_mode": true, "zoom": {"Fixe": 3}}"#);
        assert!(!c.cassette_mode && c.tooltips && c.compact_bar);
        assert_eq!((c.theme, c.icon_size, c.zoom2), (Theme::Sombre, IconSize::Normale, Zoom::Ajuster));
        assert_eq!(c.zoom, Zoom::Fixe(3));
        let mut c = c;
        c.theme = Theme::Clair;
        c.icon_size = IconSize::Grande;
        c.syntax.basic.scheme = crate::syntax::Scheme::Perso;
        c.syntax.basic.custom.keyword = [1, 2, 3];
        let back = parse(&serde_json::to_string(&c).unwrap());
        assert_eq!((back.theme, back.icon_size, back.syntax.basic.custom.keyword), (Theme::Clair, IconSize::Grande, [1, 2, 3]));
        assert!(!serde_json::to_string(&c).unwrap().contains("cassette_mode"));
        // the zoom is a constant of the setting
        assert_eq!((IconSize::Auto.zoom(), IconSize::Normale.zoom(), IconSize::Grande.zoom()), (1.0, 1.0, 1.35));
    }

    #[test]
    fn private_file() {
        let p = std::env::temp_dir().join(format!("a1cfg-{}.json", std::process::id()));
        write_private(&p, b"{}").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(std::fs::metadata(&p).unwrap().permissions().mode() & 0o777, 0o600);
        }
        std::fs::remove_file(p).ok();
    }

    #[test]
    fn screening_v53_migration() {
        // the v5.3 saved every field, without "version"
        let v53 = r#"{"screening": {"on": true, "intensity": 0.6, "gaps": true, "soften": true,
            "glow": false, "persistence": true, "curvature": true, "flicker": false}}"#;
        let s = parse(v53).screening;
        let d = Screening::default();
        assert!(s.on && s.curvature && s.glow, "halo on, user's switches kept");
        assert_eq!((s.afterglow, s.curve, s.version), (d.afterglow, d.curve, SCREENING_VERSION));
        assert_eq!(s.persistence, None);
        // afterglow switched off by the user stays off
        let off = parse(&v53.replace(r#""persistence": true"#, r#""persistence": false"#)).screening;
        assert_eq!(off.afterglow, 0.0);
    }

    #[test]
    fn fresh_start_keeps_the_library_and_stamps_the_version() {
        let old = parse(r#"{"library":"/tmp/mes-programmes","libraries":["/tmp/autre"],"github":["a/b"],"github_token":"ghp_x","theme":"Clair","kit_version":"7.9.2","zoom":{"Fixe":4}}"#);
        assert_eq!(old.kit_version, "7.9.2");
        assert_eq!(old.zoom, Zoom::Fixe(4));
        let c = GuiConfig::fresh_keeping_library(&old);
        assert_eq!(c.library, PathBuf::from("/tmp/mes-programmes"));
        assert_eq!(c.libraries, vec![PathBuf::from("/tmp/autre")]);
        assert_eq!(c.github, vec!["a/b".to_string()]);
        assert_eq!(c.github_token, "ghp_x");
        assert_eq!(c.kit_version, KIT_VERSION);
        assert_eq!(c.zoom, Zoom::Ajuster);
        assert_eq!(c.theme, Theme::Sombre);
        // a gui.json from before 1.0 (then 7.9.3) has no version
        assert!(parse(r#"{"library":"/tmp/x"}"#).kit_version.is_empty());
    }

    #[test]
    fn portable_dir_follows_the_marker_or_the_flag() {
        let base = std::env::temp_dir().join(format!("a1port-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let kit = base.join("kit");
        std::fs::create_dir_all(&kit).unwrap();
        let exe = kit.join("apple1-emu-gui");
        assert!(portable_dir(Some(&exe), false).is_none());
        assert_eq!(portable_dir(Some(&exe), true).as_deref(), Some(kit.as_path()));
        std::fs::write(kit.join("portable"), b"").unwrap();
        assert_eq!(portable_dir(Some(&exe), false).as_deref(), Some(kit.as_path()));
        assert!(portable_dir(None, true).is_none());
        // the kit layout: <kit>/bin/x86_64/apple1-emu-gui, marker at the root of the kit
        let kit2 = base.join("kit2");
        let bin = kit2.join("bin").join("x86_64");
        std::fs::create_dir_all(&bin).unwrap();
        let exe2 = bin.join("apple1-emu-gui");
        assert!(portable_dir(Some(&exe2), false).is_none());
        assert_eq!(portable_dir(Some(&exe2), true).as_deref(), Some(kit2.as_path()));
        std::fs::write(kit2.join("portable"), b"").unwrap();
        assert_eq!(portable_dir(Some(&exe2), false).as_deref(), Some(kit2.as_path()));
        // a marker next to the program still wins
        std::fs::write(bin.join("portable"), b"").unwrap();
        assert_eq!(portable_dir(Some(&exe2), false).as_deref(), Some(bin.as_path()));
        std::fs::remove_dir_all(&base).ok();
    }
}
