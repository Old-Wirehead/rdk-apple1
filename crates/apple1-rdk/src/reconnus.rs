//! Cassettes reconnues de la v1.0 : la table de `apple1-gui/src/known.rs`
//! (lue par `build.rs`, sans copie) donne les plages et la commande de
//! lancement d'une cassette identifiée par le contenu de ses blocs (SHA-256)
//! ou, à défaut, par le nom du fichier.

use apple1_core::aciwav::Imported;
use sha2::{Digest, Sha256};
use std::path::Path;

include!(concat!(env!("OUT_DIR"), "/known_table.rs"));

pub fn sha256_hex(data: &[u8]) -> String {
    Sha256::digest(data)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn norm(nom: &str) -> String {
    nom.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase()
}

/// Même règle que la v1.0 : d'abord le contenu, puis le nom du fichier
/// (avec les mêmes tailles de blocs).
pub fn identifier(imp: &Imported, chemin: Option<&Path>) -> Option<Match> {
    let blocs: Vec<&[u8]> = imp.segments.iter().map(|s| s.data.as_slice()).collect();
    if blocs.is_empty() {
        return None;
    }
    let tailles: Vec<usize> = blocs.iter().map(|r| r.len()).collect();
    let memes = |k: &Known| {
        k.records.len() == tailles.len() && k.records.iter().zip(&tailles).all(|(r, s)| r.0 == *s)
    };
    let empreintes: Vec<String> = blocs.iter().map(|r| sha256_hex(r)).collect();
    if let Some(k) = TABLE
        .iter()
        .find(|k| memes(k) && k.records.iter().zip(&empreintes).all(|(r, h)| r.1 == h))
    {
        return Some(Match {
            known: k,
            how: How::Content,
        });
    }
    let nom = norm(&chemin?.file_stem()?.to_string_lossy());
    TABLE
        .iter()
        .find(|k| memes(k) && k.names.iter().any(|n| *n == nom))
        .map(|k| Match {
            known: k,
            how: How::Name,
        })
}

impl Known {
    /// Touches de lancement (`\n` de la table = RETURN).
    pub fn touches(&self) -> String {
        format!("{}\r", self.run_cmd.replace("\\n", "\r"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_valide() {
        assert!(TABLE.len() > 20);
        for k in TABLE {
            let plages = apple1_core::program::parse_ranges(k.ranges)
                .unwrap_or_else(|e| panic!("{} : {e}", k.title));
            let tailles: Vec<usize> = plages.iter().map(|&(a, b)| (b - a) as usize + 1).collect();
            let attendu: Vec<usize> = k.records.iter().map(|r| r.0).collect();
            assert_eq!(tailles, attendu, "{}", k.title);
        }
    }
}
