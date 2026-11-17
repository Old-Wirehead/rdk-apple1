//! Langues de la 2.0 (interface commune 0.1.7) : la langue est celle de
//! l'interface commune ([`rdk_pilotes::langue::courante`]), choisie dans
//! ⚙ Préférences → Interface ou dans l'assistant.
//!
//! Deux tables :
//!
//! - [`t`] / [`tf`] : la table de la **v1.0** (`apple1-gui/src/i18n/strings.txt`,
//!   lue sans copie, 7 langues), par clé (`gh_sync`…), pour le code repris
//!   de la v1.0 ([`crate::v1`]) ;
//! - [`tr`] / [`trf!`](crate::trf) : la table de la **2.0**
//!   (`traductions/apple1.txt`), dont la clé est le texte français, comme
//!   celle de la partie commune.

use rdk_pilotes::langue::{self, Langue, Traductions};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::OnceLock;

pub use rdk_pilotes::langue::remplir as fill;

const SOURCE_V1: &str = include_str!("../../apple1-gui/src/i18n/strings.txt");

type Table = HashMap<&'static str, [Option<String>; 7]>;

fn table_v1() -> &'static Table {
    static T: OnceLock<Table> = OnceLock::new();
    T.get_or_init(|| {
        let mut t: Table = HashMap::new();
        let mut cle: Option<&'static str> = None;
        for ligne in SOURCE_V1.lines() {
            let ligne = ligne.trim_end();
            if ligne.is_empty() || ligne.starts_with('#') {
                continue;
            }
            if let Some(k) = ligne.strip_prefix('@') {
                let k = k.trim();
                t.insert(k, Default::default());
                cle = Some(k);
                continue;
            }
            let (Some(k), Some((code, texte))) = (cle, ligne.split_once(' ').or(Some((ligne, ""))))
            else {
                continue;
            };
            if let (Some(l), Some(v)) = (Langue::depuis_code(code), t.get_mut(k)) {
                v[l as usize] = Some(texte.trim_start().replace("\\n", "\n"));
            }
        }
        t
    })
}

/// Texte de la v1.0 (clé de `strings.txt`) dans la langue courante.
pub fn t(cle: &str) -> &'static str {
    let l = langue::courante();
    match table_v1().get(cle) {
        Some(v) => v[l as usize]
            .as_deref()
            .or(v[Langue::En as usize].as_deref())
            .or(v[Langue::Fr as usize].as_deref())
            .unwrap_or("?"),
        None => "?",
    }
}

/// [`t`] avec arguments `{0}`, `{1}`…
pub fn tf(cle: &str, args: &[&dyn Display]) -> String {
    fill(t(cle), args)
}

/// Table des textes de la 2.0 (`traductions/apple1.txt`).
pub fn table() -> &'static Traductions {
    static T: OnceLock<Traductions> = OnceLock::new();
    T.get_or_init(|| {
        Traductions::depuis_texte(include_str!("../traductions/apple1.txt"))
            .unwrap_or_else(|e| panic!("traductions/apple1.txt : {e}"))
    })
}

/// Texte de la 2.0 (clé = texte français) dans la langue courante.
pub fn tr(fr: &'static str) -> &'static str {
    table().tr(fr)
}

/// [`tr`] avec arguments `{0}`, `{1}`… (voir [`trf!`](crate::trf)).
pub fn trf_args(fr: &'static str, args: &[&dyn Display]) -> String {
    table().trf(fr, args)
}

/// `trf!("{0} octets", n)` : texte de l'Apple 1 traduit, avec arguments.
#[macro_export]
macro_rules! trf {
    ($cle:expr $(, $a:expr)* $(,)?) => {
        $crate::i18n::trf_args($cle, &[$(&$a as &dyn ::std::fmt::Display),*])
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Chaque texte `tr("…")` / `trf!("…")` de la machine a ses 6
    /// traductions (sauf noms propres et sigles), avec ses arguments.
    #[test]
    fn textes_de_la_machine_tous_traduits() {
        let identiques = [
            "Apple1-Slideshow",
            "Integer BASIC",
            "RDK G2 Basic",
            "Intel HEX",
            "Retro Dev Kit Apple 1 2.0",
            "50 Hz",
            "CodeTank",
        ];
        let t = table();
        let mut manquants = Vec::new();
        let d = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        for f in std::fs::read_dir(&d)
            .unwrap()
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|e| e == "rs"))
        {
            let source = std::fs::read_to_string(&f).unwrap();
            let source = source.split("#[cfg(test)]").next().unwrap_or_default();
            for (debut, _) in source
                .match_indices("tr(\"")
                .chain(source.match_indices("trf!(\""))
            {
                // `push_str("…")` n'est pas `tr("…")`
                if source[..debut]
                    .chars()
                    .next_back()
                    .is_some_and(|c| c.is_alphanumeric() || c == '_')
                {
                    continue;
                }
                let reste = &source[debut..];
                let reste = &reste[reste.find('"').unwrap() + 1..];
                let mut cle = String::new();
                let mut it = reste.chars();
                while let Some(c) = it.next() {
                    match c {
                        '\\' => match it.next() {
                            Some('n') => cle.push('\n'),
                            Some(x) => cle.push(x),
                            None => break,
                        },
                        '"' => break,
                        c => cle.push(c),
                    }
                }
                if cle
                    .replace(|c: char| !c.is_alphabetic(), "")
                    .chars()
                    .count()
                    < 2
                    || identiques.contains(&cle.as_str())
                {
                    continue;
                }
                for l in langue::TOUTES.into_iter().skip(1) {
                    let tr = t.dans(l, &cle);
                    if !t.traduit(l, &cle)
                        || (0..4).any(|i| {
                            cle.contains(&format!("{{{i}}}")) != tr.contains(&format!("{{{i}}}"))
                        })
                    {
                        manquants.push(format!(
                            "{} ({}) : {cle}",
                            f.file_name().unwrap().to_string_lossy(),
                            l.code()
                        ));
                    }
                }
            }
        }
        assert!(
            manquants.is_empty(),
            "textes sans traduction :\n{}",
            manquants.join("\n")
        );
    }

    #[test]
    fn table_v1_lue() {
        assert!(table_v1().len() > 1000);
        assert!(table_v1()["gh_done"][Langue::Fr as usize].is_some());
        assert_ne!(t("gh_done"), "?");
    }
}
