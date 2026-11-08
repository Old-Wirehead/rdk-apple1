//! Colours of the interface that are not the editor's syntax colours: dump
//! addresses and ASCII, the debugger's current line, the active buttons…
//! One value per theme (dark / light), readable defaults, saved in gui.json.

use eframe::egui::Color32;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::RwLock;

type Rgb = [u8; 3];

/// What can be coloured.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// Selected / active buttons (Normal, Fast, the tool buttons that are on…).
    Active,
    /// Current line (PC) in the disassembly and the dump, current BASIC line.
    CurLine,
    /// Bytes of the source line selected in the editor.
    HlLine,
    /// Cell being edited in the dump.
    SelCell,
    /// Breakpoint line.
    BrkLine,
    /// Addresses at the start of a dump row.
    DumpAddr,
    /// ASCII column of the dump (and values of the watches).
    DumpAscii,
    /// Value that has just changed.
    Changed,
    /// Labels (symbols) in the disassembly.
    Label,
}

impl Role {
    pub const ALL: [Role; 9] = [Role::Active, Role::CurLine, Role::HlLine, Role::SelCell, Role::BrkLine, Role::DumpAddr, Role::DumpAscii, Role::Changed, Role::Label];

    /// Name in gui.json.
    pub fn id(self) -> &'static str {
        match self {
            Role::Active => "active",
            Role::CurLine => "cur_line",
            Role::HlLine => "hl_line",
            Role::SelCell => "sel_cell",
            Role::BrkLine => "brk_line",
            Role::DumpAddr => "dump_addr",
            Role::DumpAscii => "dump_ascii",
            Role::Changed => "changed",
            Role::Label => "label",
        }
    }

    /// i18n key of its name.
    pub fn key(self) -> &'static str {
        match self {
            Role::Active => "uc_active",
            Role::CurLine => "uc_cur_line",
            Role::HlLine => "uc_hl_line",
            Role::SelCell => "uc_sel_cell",
            Role::BrkLine => "uc_brk_line",
            Role::DumpAddr => "uc_dump_addr",
            Role::DumpAscii => "uc_dump_ascii",
            Role::Changed => "uc_changed",
            Role::Label => "uc_label",
        }
    }

    /// True for a background (the text on it keeps its own colour).
    #[cfg(test)]
    pub fn is_background(self) -> bool {
        matches!(self, Role::Active | Role::CurLine | Role::HlLine | Role::SelCell | Role::BrkLine)
    }

    /// The default colour in a theme.
    pub fn default_rgb(self, dark: bool) -> Rgb {
        match (self, dark) {
            (Role::Active, true) => [0, 92, 128],
            (Role::Active, false) => [0, 92, 170],
            (Role::CurLine, true) => [28, 78, 36],
            (Role::CurLine, false) => [200, 240, 200],
            (Role::HlLine, true) => [96, 72, 14],
            (Role::HlLine, false) => [255, 226, 150],
            (Role::SelCell, true) => [40, 90, 150],
            (Role::SelCell, false) => [170, 205, 245],
            (Role::BrkLine, true) => [92, 40, 92],
            (Role::BrkLine, false) => [240, 205, 240],
            (Role::DumpAddr, true) => [140, 200, 255],
            (Role::DumpAddr, false) => [0, 60, 150],
            (Role::DumpAscii, true) => [211, 211, 211],
            (Role::DumpAscii, false) => [40, 40, 40],
            (Role::Changed, true) => [255, 210, 80],
            (Role::Changed, false) => [170, 90, 0],
            (Role::Label, true) => [170, 215, 255],
            (Role::Label, false) => [0, 70, 150],
        }
    }
}

/// The user's choices: only what differs from the default is kept.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct UiColors {
    pub dark: BTreeMap<String, Rgb>,
    pub light: BTreeMap<String, Rgb>,
}

impl UiColors {
    fn map(&self, dark: bool) -> &BTreeMap<String, Rgb> {
        if dark { &self.dark } else { &self.light }
    }

    fn map_mut(&mut self, dark: bool) -> &mut BTreeMap<String, Rgb> {
        if dark { &mut self.dark } else { &mut self.light }
    }

    /// Colour of a role in a theme: the user's, or the default.
    pub fn rgb(&self, r: Role, dark: bool) -> Rgb {
        self.map(dark).get(r.id()).copied().unwrap_or_else(|| r.default_rgb(dark))
    }

    /// Set a colour (the default one is not stored).
    pub fn set(&mut self, r: Role, dark: bool, c: Rgb) {
        if c == r.default_rgb(dark) {
            self.map_mut(dark).remove(r.id());
        } else {
            self.map_mut(dark).insert(r.id().into(), c);
        }
    }

    /// Back to the defaults of one theme.
    pub fn reset(&mut self, dark: bool) {
        self.map_mut(dark).clear();
    }

    pub fn is_default(&self, dark: bool) -> bool {
        self.map(dark).is_empty()
    }
}

static ACTIVE: RwLock<Option<(bool, [Rgb; 9])>> = RwLock::new(None);

/// Set each frame by the application: the colours of the current theme.
pub fn set_active(cfg: &UiColors, dark: bool) {
    let v = Role::ALL.map(|r| cfg.rgb(r, dark));
    if let Ok(mut a) = ACTIVE.write() {
        *a = Some((dark, v));
    }
}

/// The colour of a role in the current theme.
pub fn get(r: Role) -> Color32 {
    let dark = crate::syntax::dark();
    let c = match ACTIVE.read().ok().and_then(|a| *a) {
        Some((d, v)) if d == dark => v[Role::ALL.iter().position(|x| *x == r).unwrap_or(0)],
        _ => r.default_rgb(dark),
    };
    Color32::from_rgb(c[0], c[1], c[2])
}

/// WCAG contrast ratio of two colours (1 to 21).
#[cfg(test)]
pub fn contrast(a: Rgb, b: Rgb) -> f32 {
    let lum = |c: Rgb| {
        let f = |v: u8| {
            let v = v as f32 / 255.0;
            if v <= 0.03928 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
        };
        0.2126 * f(c[0]) + 0.7152 * f(c[1]) + 0.0722 * f(c[2])
    };
    let (la, lb) = (lum(a), lum(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Background of the panels (egui's dark and light themes).
    const PANEL: [Rgb; 2] = [[27, 27, 27], [248, 248, 248]];

    #[test]
    fn defaults_are_readable_on_both_themes() {
        for (i, dark) in [true, false].into_iter().enumerate() {
            for r in Role::ALL {
                let c = r.default_rgb(dark);
                if r.is_background() {
                    // the text drawn on it: the ASCII colour (normal text of the theme)
                    let text = Role::DumpAscii.default_rgb(dark);
                    assert!(contrast(c, text) >= 4.5 || r == Role::Active, "{r:?} dark={dark}: text on it {:.1}", contrast(c, text));
                } else {
                    assert!(contrast(c, PANEL[i]) >= 4.5, "{r:?} dark={dark}: {:.1}", contrast(c, PANEL[i]));
                }
            }
            // active buttons carry white text
            assert!(contrast(Role::Active.default_rgb(dark), [255, 255, 255]) >= 4.5, "active dark={dark}");
        }
    }

    #[test]
    fn choices_per_theme_and_reset() {
        let mut u = UiColors::default();
        assert_eq!(u.rgb(Role::DumpAddr, true), Role::DumpAddr.default_rgb(true));
        u.set(Role::DumpAddr, true, [1, 2, 3]);
        assert_eq!(u.rgb(Role::DumpAddr, true), [1, 2, 3]);
        // the other theme is untouched
        assert_eq!(u.rgb(Role::DumpAddr, false), Role::DumpAddr.default_rgb(false));
        assert!(!u.is_default(true) && u.is_default(false));
        // the default value is not stored
        u.set(Role::DumpAddr, true, Role::DumpAddr.default_rgb(true));
        assert!(u.is_default(true));
        u.set(Role::Changed, false, [9, 9, 9]);
        u.reset(false);
        assert!(u.is_default(false));
        // old gui.json without the section
        let s: UiColors = serde_json::from_str("{}").unwrap();
        assert_eq!(s, UiColors::default());
        let back: UiColors = serde_json::from_str(r#"{"dark":{"label":[5,6,7],"future_role":[1,1,1]}}"#).unwrap();
        assert_eq!(back.rgb(Role::Label, true), [5, 6, 7]);
    }
}
