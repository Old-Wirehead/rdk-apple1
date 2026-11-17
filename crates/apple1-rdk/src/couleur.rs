//! Moniteur couleur de la v1.0 : la carte GEN2 (rendu de
//! `apple1-gui/src/gen2screen.rs`, partagé par `build.rs`) et la carte P-LAB
//! (TMS9918A du cœur), vues par le pilote `video_numerique` + l'option
//! `video_couleur` de l'interface commune.

use crate::trf;
use apple1_core::Machine as Apple1;
use rdk_pilotes::{CouleurPalette, ImageVideo};

#[allow(dead_code, clippy::all)]
pub mod gen2screen {
    include!(concat!(env!("OUT_DIR"), "/gen2screen.rs"));
}

/// Clignotement des caractères GEN2 : 1,5 Hz (comme la v1.0).
pub const FLASH_HZ: f64 = 1.5;

/// Noms des 16 couleurs GEN2 (traductions françaises col0..col15 de la v1.0).
pub const NOMS: [&str; 16] = [
    "noir",
    "magenta",
    "bleu foncé",
    "violet",
    "vert foncé",
    "gris 1",
    "bleu moyen",
    "bleu clair",
    "marron",
    "orange",
    "gris 2",
    "rose",
    "vert clair",
    "jaune",
    "aigue-marine",
    "blanc",
];

/// L'image du moniteur couleur branché sur la GEN2 (280 × 192).
pub fn image_gen2(m: &Apple1, accolades_grasses: bool) -> ImageVideo {
    let b = &m.board;
    let flash = (b.now_secs() * 2.0 * FLASH_HZ) as u64 & 1 == 0;
    let lignes = b
        .gen2
        .frame_lines(b.config.timing.cycles_to_ticks(b.cycles));
    let img = gen2screen::render_lines_ex(&lignes, &b.mem, flash, accolades_grasses);
    let pixels = img
        .pixels
        .iter()
        .map(|p| [p.r(), p.g(), p.b(), 255])
        .collect();
    ImageVideo {
        largeur: img.size[0],
        hauteur: img.size[1],
        pixels,
    }
}

/// L'image de la carte P-LAB (TMS9918A, 256 × 192).
pub fn image_plab(m: &Apple1) -> ImageVideo {
    let rgba = m.board.vdp.render_rgba();
    let pixels = rgba
        .as_chunks::<4>()
        .0
        .iter()
        .map(|c| [c[0], c[1], c[2], 255])
        .collect();
    ImageVideo {
        largeur: 256,
        hauteur: 192,
        pixels,
    }
}

/// Palette GEN2 pour l'interface (gris BT.601 pour le moniteur monochrome).
pub fn palette() -> Vec<CouleurPalette> {
    (0..16u8)
        .map(|i| {
            let c = gen2screen::color(i);
            let (r, g, b) = (c.r(), c.g(), c.b());
            let gris = ((299 * r as u32 + 587 * g as u32 + 114 * b as u32) / 1000) as u8;
            CouleurPalette {
                numero: i,
                nom: NOMS[i as usize].to_string(),
                rvb: [r, g, b],
                nom_monochrome: trf!("gris {0}", gris),
                gris,
            }
        })
        .collect()
}
