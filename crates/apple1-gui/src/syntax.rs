//! Syntax colours of the editor: a scheme per language (BASIC, 6502
//! assembly), or the user's own colours. Saved in gui.json.

use eframe::egui::Color32;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::RwLock;

use crate::editor::{Kind, SrcLang};

type Rgb = [u8; 3];

/// One colour per kind of text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Palette {
    pub line_no: Rgb,
    pub keyword: Rgb,
    pub number: Rgb,
    pub string: Rgb,
    pub comment: Rgb,
    pub operator: Rgb,
    pub name: Rgb,
    pub other: Rgb,
    pub label: Rgb,
    pub directive: Rgb,
}

impl Default for Palette {
    fn default() -> Self {
        Scheme::Classique.palette(true)
    }
}

impl Palette {
    pub fn get(&self, k: Kind) -> Color32 {
        let c = match k {
            Kind::LineNo => self.line_no,
            Kind::Keyword => self.keyword,
            Kind::Number => self.number,
            Kind::Str => self.string,
            Kind::Rem => self.comment,
            Kind::Op => self.operator,
            Kind::Name => self.name,
            Kind::Other => self.other,
            Kind::Label => self.label,
            Kind::Directive => self.directive,
        };
        Color32::from_rgb(c[0], c[1], c[2])
    }

    /// The colours shown in the preferences for a language: (i18n key, colour).
    pub fn fields(&mut self, lang: SrcLang) -> Vec<(&'static str, &mut Rgb)> {
        let Palette { line_no, keyword, number, string, comment, operator, name, other, label, directive } = self;
        match lang {
            SrcLang::Basic => vec![("sy_line_no", line_no), ("sy_keyword", keyword), ("sy_number", number), ("sy_string", string), ("sy_comment", comment), ("sy_operator", operator), ("sy_name", name)],
            SrcLang::Asm => vec![("sy_mnemonic", keyword), ("sy_label", label), ("sy_directive", directive), ("sy_number", number), ("sy_string", string), ("sy_comment", comment), ("sy_operator", operator), ("sy_name", name), ("sy_other", other)],
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Scheme {
    /// Classique in the dark theme, Clair in the light one.
    #[default]
    Auto,
    Classique,
    Clair,
    Ambre,
    Contraste,
    /// The user's colours.
    Perso,
}

impl Scheme {
    pub const ALL: [Scheme; 6] = [Scheme::Auto, Scheme::Classique, Scheme::Clair, Scheme::Ambre, Scheme::Contraste, Scheme::Perso];

    pub fn key(self) -> &'static str {
        match self {
            Scheme::Auto => "sy_auto",
            Scheme::Classique => "sy_classic",
            Scheme::Clair => "sy_light",
            Scheme::Ambre => "sy_amber",
            Scheme::Contraste => "sy_contrast",
            Scheme::Perso => "sy_custom",
        }
    }

    /// The colours of a fixed scheme (`dark`: theme, for Auto).
    pub fn palette(self, dark: bool) -> Palette {
        let p = |l: Rgb, k: Rgb, n: Rgb, s: Rgb, c: Rgb, o: Rgb, na: Rgb, ot: Rgb, la: Rgb, d: Rgb| Palette { line_no: l, keyword: k, number: n, string: s, comment: c, operator: o, name: na, other: ot, label: la, directive: d };
        match self {
            Scheme::Auto | Scheme::Perso if dark => Scheme::Classique.palette(true),
            Scheme::Auto | Scheme::Perso => Scheme::Clair.palette(false),
            Scheme::Classique => p([235, 190, 90], [110, 175, 255], [205, 150, 255], [150, 220, 120], [135, 135, 135], [255, 150, 150], [228, 228, 228], [200, 200, 200], [235, 190, 90], [90, 210, 200]),
            Scheme::Clair => p([150, 90, 0], [20, 70, 190], [130, 40, 170], [30, 120, 30], [110, 110, 110], [180, 30, 30], [30, 30, 30], [60, 60, 60], [150, 90, 0], [0, 120, 120]),
            Scheme::Ambre => p([255, 200, 90], [255, 176, 0], [255, 214, 140], [230, 160, 60], [150, 105, 40], [255, 190, 70], [255, 220, 170], [220, 170, 100], [255, 200, 90], [255, 160, 40]),
            Scheme::Contraste => p([255, 255, 0], [0, 200, 255], [255, 120, 255], [0, 255, 0], [170, 170, 170], [255, 80, 80], [255, 255, 255], [255, 255, 255], [255, 255, 0], [0, 255, 200]),
        }
    }
}

/// Colours of one language.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct LangColors {
    pub scheme: Scheme,
    /// Used when the scheme is Perso.
    pub custom: Palette,
}

impl LangColors {
    pub fn palette(&self, dark: bool) -> Palette {
        if self.scheme == Scheme::Perso {
            self.custom
        } else {
            self.scheme.palette(dark)
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SyntaxCfg {
    pub basic: LangColors,
    pub asm: LangColors,
}

static ACTIVE: RwLock<Option<[Palette; 2]>> = RwLock::new(None);
static DARK: AtomicBool = AtomicBool::new(true);

/// Set each frame by the application.
pub fn set_active(cfg: &SyntaxCfg, dark: bool) {
    let v = [cfg.basic.palette(dark), cfg.asm.palette(dark)];
    DARK.store(dark, Ordering::Relaxed);
    if let Ok(mut a) = ACTIVE.write() {
        *a = Some(v);
    }
}

pub fn dark() -> bool {
    DARK.load(Ordering::Relaxed)
}

/// The colour of a kind of text in a language.
pub fn color(lang: SrcLang, k: Kind) -> Color32 {
    let i = if lang == SrcLang::Asm { 1 } else { 0 };
    match ACTIVE.read().ok().and_then(|a| *a) {
        Some(p) => p[i].get(k),
        None => Palette::default().get(k),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schemes() {
        assert_eq!(Scheme::Auto.palette(true), Scheme::Classique.palette(true));
        assert_eq!(Scheme::Auto.palette(false), Scheme::Clair.palette(false));
        let mut c = LangColors { scheme: Scheme::Perso, ..Default::default() };
        c.custom.keyword = [1, 2, 3];
        assert_eq!(c.palette(true).get(Kind::Keyword), Color32::from_rgb(1, 2, 3));
        // old files: missing section = defaults
        let s: SyntaxCfg = serde_json::from_str("{}").unwrap();
        assert_eq!(s, SyntaxCfg::default());
    }
}
