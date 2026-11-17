//! Partage du code de la v1.0 sans copie : le rendu de l'écran couleur GEN2
//! (`apple1-gui/src/gen2screen.rs`) et la table des cassettes reconnues
//! (`apple1-gui/src/known.rs`) sont lus dans les sources de la v1.0 et
//! recopiés dans `OUT_DIR` à chaque construction (sans leurs tests, qui
//! dépendent de l'interface 1.0).

use std::path::PathBuf;

fn main() {
    let ici = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let gui = ici.join("../apple1-gui/src");
    let sortie = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // écran couleur GEN2
    let f = gui.join("gen2screen.rs");
    println!("cargo:rerun-if-changed={}", f.display());
    let s = std::fs::read_to_string(&f).expect("gen2screen.rs de la v1.0");
    let fin = s.find("#[cfg(test)]\nmod tests").unwrap_or(s.len());
    let glyphes = gui.join("../glyphs").canonicalize().expect("glyphes GEN2 de la v1.0");
    let s = s[..fin]
        .replace("use crate::screen::{CELL_W, HEIGHT, WIDTH};", "const CELL_W: usize = 7;\nconst HEIGHT: usize = 192;\nconst WIDTH: usize = 280;")
        .replace("include_str!(\"../glyphs/", &format!("include_str!(\"{}/", glyphes.display()))
        .lines()
        .filter(|l| !l.starts_with("//!"))
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(sortie.join("gen2screen.rs"), s).unwrap();

    // cassettes reconnues : structures et table seulement
    let f = gui.join("known.rs");
    println!("cargo:rerun-if-changed={}", f.display());
    let s = std::fs::read_to_string(&f).expect("known.rs de la v1.0");
    let debut = s.find("#[derive(Debug, PartialEq, Eq)]\npub struct Known").expect("struct Known");
    let fin = s[debut..].find("\n];\n").map(|i| debut + i + 4).expect("fin de TABLE");
    std::fs::write(sortie.join("known_table.rs"), &s[debut..fin]).unwrap();
}
