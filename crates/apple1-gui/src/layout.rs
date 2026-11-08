//! Layout of the main window: every tool is a pane of an egui_dock tree
//! (side by side, stacked or in tabs, rearranged by dragging its tab), or
//! in its own OS window (crate::dock::window). The tree is saved in gui.json.

use egui_dock::{DockState, NodeIndex};
use serde::{Deserialize, Serialize};

/// Version of the saved layout: an older or missing one gives the default.
pub const VERSION: u32 = 3;
/// Under this width (points: 1024x768, 1366x768…) the default layout is
/// Editor + screen only, so that the Apple-1 screen stays readable (#36).
pub const SMALL_WINDOW: f32 = 1400.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Pane {
    Editor,
    Screen,
    Color,
    /// P-LAB graphic card screen (TMS9918A), v7.9.
    Plab,
    Debugger,
    Disasm,
    Dump,
    Library,
    /// « Photo → ASCII » (v7.7).
    Ascii,
}

impl Pane {
    pub const ALL: [Pane; 9] = [Pane::Editor, Pane::Screen, Pane::Color, Pane::Plab, Pane::Debugger, Pane::Disasm, Pane::Dump, Pane::Library, Pane::Ascii];

    /// Id of its separate window.
    pub fn id(self) -> &'static str {
        match self {
            Pane::Editor => "editeur",
            Pane::Screen => "ecran",
            Pane::Color => "ecran-couleur",
            Pane::Plab => "ecran-plab",
            Pane::Debugger => "debogueur",
            Pane::Disasm => "desassemblage",
            Pane::Dump => "dump",
            Pane::Library => "bibliotheque",
            Pane::Ascii => "ascii",
        }
    }

    pub fn title_key(self) -> &'static str {
        match self {
            Pane::Editor => "ed_title",
            Pane::Screen => "scr_title",
            Pane::Color => "cap_color",
            Pane::Plab => "tb_plabwin",
            Pane::Debugger => "db_title",
            Pane::Disasm => "db_disasm",
            Pane::Dump => "db_dump",
            Pane::Library => "lib_title",
            Pane::Ascii => "ax_title",
        }
    }

    /// Window / tab title. GEN2 uses Bernie's exact English credit, untranslated.
    pub fn title(self) -> String {
        match self {
            Pane::Color => "Uncle Bernie's A-1 color graphics card".into(),
            Pane::Plab => "P-LAB Apple-1 Graphic Card (TMS9918A)".into(),
            other => crate::i18n::t(other.title_key()).into(),
        }
    }

    /// Every pane can be closed (the « Fenêtres » group of the bar shows it again).
    pub fn closable(self) -> bool {
        true
    }
}

/// Share of the window (0..1) of a pane wanted at `frac` of the width `w`
/// (points), but never under `min` nor over `max` points: the panes follow
/// the size of the window (a 1366 px laptop or a 1920 px monitor) without a
/// pane being squeezed on a small one or swollen on a big one, the Apple-1
/// screen taking what is left.
fn share(w: f32, frac: f32, min: f32, max: f32) -> f32 {
    let w = w.max(600.0);
    (w * frac).clamp(min, max) / w
}

/// The side panes together never take more than 74 % of the window (on a very
/// narrow window the minimums above would not fit: the split fractions must
/// stay inside 0..1), each one shrunk in proportion.
fn fit<const N: usize>(mut v: [f32; N]) -> [f32; N] {
    let total: f32 = v.iter().sum();
    if total > 0.74 {
        for x in v.iter_mut() {
            *x *= 0.74 / total;
        }
    }
    v
}

/// Default: editor on the left; Apple-1 screen in the centre (GEN2 colour
/// screen below it); debugger with the disassembly below it; dump; library
/// on the far right. `w`: width of the window, in points.
pub fn default_layout(color: bool, library_right: bool, w: f32) -> DockState<Pane> {
    // proportions of the reference screenshot (a big monitor): editor 24 %,
    // Apple-1 screens 27 %, debugger 20 %, dump 17.5 %, library 10.5 %; on a
    // small window the minimums win (1366 pt: editor 328, debugger 273,
    // dump 239, library 190 — the `fit` below shrinks them to 74 % at most)
    let [ed, dbg, dump, lib] = fit([share(w, 0.24, 260.0, f32::MAX), share(w, 0.20, 230.0, f32::MAX), share(w, 0.175, 200.0, f32::MAX), share(w, 0.105, 190.0, f32::MAX)]);
    let mut st = DockState::new(vec![Pane::Screen]);
    let t = st.main_surface_mut();
    // `fraction` of split_*: share of the first (left or top) node
    let [center, _] = t.split_left(NodeIndex::root(), ed, vec![Pane::Editor, Pane::Ascii]);
    let mut rest: f32 = 1.0 - ed;
    let mut center = center;
    if library_right {
        [center, _] = t.split_right(center, 1.0 - lib / rest, vec![Pane::Library]);
        rest -= lib;
    }
    [center, _] = t.split_right(center, 1.0 - dump / rest, vec![Pane::Dump]);
    rest -= dump;
    let [center, d] = t.split_right(center, 1.0 - dbg / rest, vec![Pane::Debugger]);
    t.split_below(d, 0.5, vec![Pane::Disasm]);
    if color {
        t.split_below(center, 0.5, vec![Pane::Color]);
    }
    if !library_right {
        let t = st.main_surface_mut();
        t.split_left(NodeIndex::root(), lib, vec![Pane::Library]);
    }
    st
}

/// The layouts offered by « ⊞ Disposition » (besides the user's own ones).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preset {
    /// Every tool open, as at the first launch.
    Default,
    /// Apple-1 screen + library.
    Game,
    /// BASIC editor + Apple-1 screen.
    Basic,
    /// Editor + debugger (+ the screen).
    Asm,
    /// Editor + debugger + disassembly + hex dump (+ the screen).
    Debug,
}

impl Preset {
    pub const ALL: [Preset; 5] = [Preset::Default, Preset::Game, Preset::Basic, Preset::Asm, Preset::Debug];

    /// Name kept in gui.json (`layout_choice`).
    pub fn key(self) -> &'static str {
        match self {
            Preset::Default => "default",
            Preset::Game => "game",
            Preset::Basic => "basic",
            Preset::Asm => "asm",
            Preset::Debug => "debug",
        }
    }

    pub fn title_key(self) -> &'static str {
        match self {
            Preset::Default => "ly_default",
            Preset::Game => "ly_game",
            Preset::Basic => "ly_basic",
            Preset::Asm => "ly_asm",
            Preset::Debug => "ly_debug",
        }
    }

    /// Key of the sentence saying what the layout is for.
    pub fn desc_key(self) -> &'static str {
        match self {
            Preset::Default => "wl_d_default",
            Preset::Game => "wl_d_game",
            Preset::Basic => "wl_d_basic",
            Preset::Asm => "wl_d_asm",
            Preset::Debug => "wl_d_debug",
        }
    }

    /// Screen size in pixels (width, height): the smallest at which the layout
    /// is usable, and the one it is comfortable at.
    pub fn resolution(self) -> ((u32, u32), (u32, u32)) {
        match self {
            Preset::Default => ((1366, 768), (1920, 1080)),
            Preset::Game => ((800, 600), (1280, 720)),
            Preset::Basic => ((1024, 700), (1366, 768)),
            Preset::Asm => ((1280, 720), (1600, 900)),
            Preset::Debug => ((1366, 768), (1920, 1080)),
        }
    }

    /// Thumbnail: the panes as (pane, x0, y0, x1, y1) in 0..1 of the window.
    /// Every provided layout shows the colour screen (GEN2 / P-LAB) under the
    /// monochrome terminal: that rectangle is added here, the others stay put.
    pub fn thumb(self) -> Vec<(Pane, f32, f32, f32, f32)> {
        use Pane::*;
        let mut v = match self {
            Preset::Default => vec![(Editor, 0.0, 0.0, 0.24, 1.0), (Screen, 0.24, 0.0, 0.52, 1.0), (Debugger, 0.52, 0.0, 0.72, 0.5), (Disasm, 0.52, 0.5, 0.72, 1.0), (Dump, 0.72, 0.0, 0.895, 1.0), (Library, 0.895, 0.0, 1.0, 1.0)],
            Preset::Game => vec![(Screen, 0.0, 0.0, 0.7, 1.0), (Library, 0.7, 0.0, 1.0, 1.0)],
            Preset::Basic => vec![(Editor, 0.0, 0.0, 0.5, 1.0), (Screen, 0.5, 0.0, 1.0, 1.0)],
            Preset::Asm => vec![(Editor, 0.0, 0.0, 0.4, 1.0), (Screen, 0.4, 0.0, 0.7, 1.0), (Debugger, 0.7, 0.0, 1.0, 1.0)],
            Preset::Debug => vec![(Editor, 0.0, 0.0, 0.25, 1.0), (Screen, 0.25, 0.0, 0.5, 1.0), (Debugger, 0.5, 0.0, 0.72, 0.5), (Disasm, 0.5, 0.5, 0.72, 1.0), (Dump, 0.72, 0.0, 1.0, 1.0)],
        };
        if let Some(i) = v.iter().position(|(p, ..)| *p == Screen) {
            let (_, x0, y0, x1, y1) = v[i];
            let mid = y0 + (y1 - y0) * 0.5;
            v[i] = (Screen, x0, y0, x1, mid);
            v.insert(i + 1, (Color, x0, mid, x1, y1));
        }
        v
    }

    /// Panes open (the screens are always there): editor, debugger, disassembly, dump, library, ASCII.
    pub fn open(self) -> [bool; 6] {
        match self {
            Preset::Default => [true, true, true, true, true, false],
            Preset::Game => [false, false, false, false, true, false],
            Preset::Basic => [true, false, false, false, false, false],
            Preset::Asm => [true, true, false, false, false, false],
            Preset::Debug => [true, true, true, true, false, false],
        }
    }
}

/// The tree of a preset.
pub fn preset_layout(p: Preset, color: bool, library_right: bool, w: f32) -> DockState<Pane> {
    if p == Preset::Default {
        return default_layout(color, library_right, w);
    }
    let mut st = DockState::new(vec![Pane::Screen]);
    let t = st.main_surface_mut();
    let center = NodeIndex::root();
    match p {
        Preset::Game => {
            // the screen first, the library at its usual width
            let [lib] = fit([share(w, 0.20, 220.0, 360.0)]);
            if library_right {
                t.split_right(center, 1.0 - lib, vec![Pane::Library]);
            } else {
                t.split_left(center, lib, vec![Pane::Library]);
            }
        }
        Preset::Basic => {
            let [ed] = fit([share(w, 0.38, 340.0, 760.0)]);
            t.split_left(center, ed, vec![Pane::Editor]);
        }
        Preset::Asm => {
            let [ed, dbg] = fit([share(w, 0.30, 300.0, 560.0), share(w, 0.22, 250.0, 420.0)]);
            let [center, _] = t.split_left(center, ed, vec![Pane::Editor]);
            t.split_right(center, 1.0 - dbg / (1.0 - ed), vec![Pane::Debugger]);
        }
        Preset::Debug => {
            let [ed, dump, dbg] = fit([share(w, 0.25, 280.0, 480.0), share(w, 0.15, 200.0, 300.0), share(w, 0.18, 230.0, 340.0)]);
            let [center, _] = t.split_left(center, ed, vec![Pane::Editor]);
            let [center, _] = t.split_right(center, 1.0 - dump / (1.0 - ed), vec![Pane::Dump]);
            let [_, d] = t.split_right(center, 1.0 - dbg / (1.0 - ed - dump), vec![Pane::Debugger]);
            t.split_below(d, 0.5, vec![Pane::Disasm]);
        }
        Preset::Default => {}
    }
    if color {
        // the screen is the leaf that was the root
        if let Some((n, _)) = st.find_main_surface_tab(&Pane::Screen) {
            st.main_surface_mut().split_below(n, 0.5, vec![Pane::Color]);
        }
    }
    st
}

/// A layout saved by the user (« Enregistrer la disposition actuelle… »), in gui.json.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SavedLayout {
    pub name: String,
    /// Panes open: editor, debugger, disassembly, dump, library, ASCII.
    pub open: [bool; 6],
    #[serde(with = "saved", default)]
    pub tree: Option<DockState<Pane>>,
    /// Panes in their own OS window, with position and size (egui points of
    /// the whole desktop: the position also says which monitor) (#38).
    #[serde(default)]
    pub detached: Vec<(Pane, crate::dock::DockCfg)>,
    /// Size of the main window when the layout was saved (egui points).
    #[serde(default)]
    pub window: Option<[f32; 2]>,
}

/// The user's layouts in alphabetical order (case ignored).
pub fn sorted_names(v: &[SavedLayout]) -> Vec<String> {
    let mut n: Vec<String> = v.iter().map(|l| l.name.clone()).collect();
    n.sort_by_key(|s| (s.to_lowercase(), s.clone()));
    n
}

/// Is `name` usable for a user's layout? (not empty, not the name of a preset)
pub fn name_ok(name: &str, preset_names: &[String]) -> bool {
    let n = name.trim();
    !n.is_empty() && n.chars().count() <= 40 && !preset_names.iter().any(|p| p.eq_ignore_ascii_case(n))
}

pub fn contains(st: &DockState<Pane>, p: Pane) -> bool {
    st.find_tab(&p).is_some()
}

pub fn remove(st: &mut DockState<Pane>, p: Pane) {
    if let Some(path) = st.find_tab(&p) {
        st.remove_tab(path);
    }
}

/// Put a pane back where the default layout has it, next to a neighbour
/// still in the tree.
pub fn insert(st: &mut DockState<Pane>, p: Pane, library_right: bool) {
    if st.main_surface().is_empty() {
        *st = DockState::new(vec![p]);
        return;
    }
    let leaf = |st: &DockState<Pane>, q: Pane| st.find_main_surface_tab(&q).map(|(n, _)| n);
    // the ASCII tool: a tab next to the editor
    if p == Pane::Ascii {
        if let Some(n) = leaf(st, Pane::Editor) {
            let t = st.main_surface_mut();
            t[n].append_tab(p);
            if let Some((n, i)) = t.find_tab(&p) {
                let _ = t.set_active_tab(n, i);
            }
            return;
        }
    }
    let t_root = NodeIndex::root();
    use Pane::*;
    // (neighbour, side of the neighbour where p goes, share kept by it)
    let near: &[(Pane, egui_dock::Split, f32)] = match p {
        Editor => &[(Screen, egui_dock::Split::Left, 0.65), (Color, egui_dock::Split::Left, 0.65)],
        Screen => &[(Color, egui_dock::Split::Above, 0.5), (Editor, egui_dock::Split::Right, 0.4)],
        Color => &[(Screen, egui_dock::Split::Below, 0.5), (Editor, egui_dock::Split::Right, 0.35)],
        Plab => &[(Color, egui_dock::Split::Below, 0.5), (Screen, egui_dock::Split::Below, 0.5), (Editor, egui_dock::Split::Right, 0.35)],
        Debugger => &[(Disasm, egui_dock::Split::Above, 0.5), (Dump, egui_dock::Split::Left, 0.5), (Screen, egui_dock::Split::Right, 0.6)],
        Disasm => &[(Debugger, egui_dock::Split::Below, 0.5), (Dump, egui_dock::Split::Left, 0.5), (Screen, egui_dock::Split::Right, 0.6)],
        Dump => &[(Library, egui_dock::Split::Left, 0.4), (Debugger, egui_dock::Split::Right, 0.5), (Disasm, egui_dock::Split::Right, 0.5), (Screen, egui_dock::Split::Right, 0.6)],
        Library => &[],
        Ascii => &[(Screen, egui_dock::Split::Left, 0.6), (Library, egui_dock::Split::Left, 0.75)],
    };
    for (q, side, f) in near {
        if let Some(n) = leaf(st, *q) {
            // fraction: share of the first (left / top) node
            let f = if matches!(side, egui_dock::Split::Left | egui_dock::Split::Above) { 1.0 - f } else { *f };
            st.main_surface_mut().split(n, *side, f, egui_dock::Node::leaf_with(vec![p]));
            return;
        }
    }
    let t = st.main_surface_mut();
    match p {
        Library if !library_right => {
            t.split_left(t_root, 0.13, vec![p]);
        }
        Library => {
            t.split_right(t_root, 0.87, vec![p]);
        }
        Editor => {
            t.split_left(t_root, 0.22, vec![p]);
        }
        _ => {
            t.split_right(t_root, 0.7, vec![p]);
        }
    }
}

/// gui.json form of the tree: the rectangles of a node not drawn yet are NaN
/// (JSON null, which would not read back): saved as 0, recomputed when
/// drawn. A tree that does not read (other egui_dock) gives None: default.
pub mod saved {
    use super::*;
    use serde::{Deserializer, Serializer};

    /// Coordinates (x, y) null → 0 (other nulls are real None).
    fn no_null(v: &mut serde_json::Value) {
        match v {
            serde_json::Value::Array(a) => a.iter_mut().for_each(no_null),
            serde_json::Value::Object(o) => {
                for (k, x) in o.iter_mut() {
                    if x.is_null() && (k == "x" || k == "y") {
                        *x = serde_json::Value::from(0.0);
                    } else {
                        no_null(x);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn serialize<S: Serializer>(st: &Option<DockState<Pane>>, s: S) -> Result<S::Ok, S::Error> {
        let mut v = serde_json::to_value(st).map_err(serde::ser::Error::custom)?;
        no_null(&mut v);
        v.serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<DockState<Pane>>, D::Error> {
        let v = serde_json::Value::deserialize(d)?;
        Ok(serde_json::from_value(v).ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_has_every_pane_once() {
        for color in [false, true] {
            let st = default_layout(color, true, 1366.0);
            for p in Pane::ALL {
                let n = st.iter_all_tabs().filter(|(_, t)| **t == p).count();
                assert_eq!(n, if (p == Pane::Color && !color) || p == Pane::Plab { 0 } else { 1 }, "{p:?}");
            }
        }
        let st = default_layout(true, false, 1920.0);
        assert!(contains(&st, Pane::Library));
    }

    #[test]
    fn every_width_gives_valid_fractions() {
        // egui_dock panics on a fraction outside 0..1
        for w in [100.0, 480.0, 683.0, 819.0, 1024.0, 1366.0, 1920.0, 2880.0, 5000.0] {
            for color in [false, true] {
                for lib_right in [false, true] {
                    default_layout(color, lib_right, w);
                    for pr in Preset::ALL {
                        preset_layout(pr, color, lib_right, w);
                    }
                }
            }
        }
    }

    #[test]
    fn presets_hold_their_panes() {
        use Pane::*;
        for color in [false, true] {
            for pr in Preset::ALL {
                let st = preset_layout(pr, color, true, 1366.0);
                let o = pr.open();
                let want = [(Editor, o[0]), (Debugger, o[1]), (Disasm, o[2]), (Dump, o[3]), (Library, o[4]), (Screen, true), (Color, color)];
                for (p, w) in want {
                    if pr == Preset::Default && p == Pane::Editor {
                        continue;
                    }
                    assert_eq!(contains(&st, p), w, "{pr:?} {p:?} color={color}");
                }
            }
        }
        // library on the left
        assert!(contains(&preset_layout(Preset::Game, false, false, 1366.0), Library));
    }

    #[test]
    fn saved_layouts_sorted_and_names() {
        let mk = |n: &str| SavedLayout { name: n.into(), open: [true; 6], tree: Some(default_layout(false, true, 1366.0)), detached: Vec::new(), window: None };
        let v = vec![mk("zèbre"), mk("Alpha"), mk("beta")];
        assert_eq!(sorted_names(&v), ["Alpha", "beta", "zèbre"]);
        let presets = vec!["Jeu".to_string(), "BASIC".to_string()];
        assert!(name_ok("Mon test", &presets));
        assert!(!name_ok("  ", &presets) && !name_ok("jeu", &presets) && !name_ok(&"x".repeat(41), &presets));
        let json = serde_json::to_string(&v).unwrap();
        let back: Vec<SavedLayout> = serde_json::from_str(&json).unwrap();
        assert_eq!(back.len(), 3);
        assert!(contains(back[0].tree.as_ref().unwrap(), Pane::Library));
        // detached windows keep their position and size (#38); an old gui.json has none
        let mut l = mk("deux ecrans");
        let d = crate::dock::DockCfg { detached: true, width: 640.0, pos: Some([1930.0, 20.0]), size: Some([1280.0, 800.0]) };
        l.detached = vec![(Pane::Screen, d.clone())];
        l.window = Some([1000.0, 700.0]);
        let back: SavedLayout = serde_json::from_str(&serde_json::to_string(&l).unwrap()).unwrap();
        assert_eq!(back.detached, vec![(Pane::Screen, d)]);
        assert_eq!(back.window, Some([1000.0, 700.0]));
        let old: SavedLayout = serde_json::from_str(r#"{"name":"x","open":[true,true,true,true,true,true]}"#).unwrap();
        assert!(old.detached.is_empty() && old.window.is_none());
    }

    #[test]
    fn remove_and_insert_again() {
        let mut st = default_layout(true, true, 1366.0);
        for p in Pane::ALL {
            remove(&mut st, p);
            assert!(!contains(&st, p));
            insert(&mut st, p, true);
            assert!(contains(&st, p));
        }
        // empty tree
        for p in Pane::ALL {
            remove(&mut st, p);
        }
        insert(&mut st, Pane::Dump, true);
        insert(&mut st, Pane::Screen, true);
        assert!(contains(&st, Pane::Dump) && contains(&st, Pane::Screen));
        #[derive(Serialize, Deserialize)]
        struct C {
            #[serde(with = "saved", default)]
            layout: Option<DockState<Pane>>,
        }
        let json = serde_json::to_string(&C { layout: Some(st) }).unwrap();
        let back: C = serde_json::from_str(&json).unwrap();
        assert!(contains(back.layout.as_ref().unwrap(), Pane::Screen));
        // unreadable tree: default layout
        let back: C = serde_json::from_str(r#"{"layout":{"surfaces":42}}"#).unwrap();
        assert!(back.layout.is_none());
    }
}
