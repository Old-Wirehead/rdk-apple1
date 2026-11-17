//! Pilote `video-numerique` de l'Apple 1 : le terminal 40 × 24 dessiné avec
//! la vraie ROM de caractères Signetics 2513 (7 × 8 points par case, soit
//! 280 × 192 points), comme la v1.0. Moniteur **monochrome** seulement.

use apple1_core::display::{CursorBlink, COLS, ROWS};
use apple1_core::Machine;
use rdk_pilotes::ImageVideo;

/// ROM de caractères 2513 (64 caractères × 8 lignes, 5 bits) : **le même
/// fichier** que la v1.0 (`apple1-gui/assets/2513.bin`), sans copie.
pub const CHARGEN: &[u8; 512] = include_bytes!("../../apple1-gui/assets/2513.bin");

pub const CASE_L: usize = 7;
pub const CASE_H: usize = 8;
pub const LARGEUR: usize = COLS * CASE_L; // 280
pub const HAUTEUR: usize = ROWS * CASE_H; // 192

/// Points allumés et fond, en blanc et noir purs : le moniteur monochrome de
/// l'interface commune (0.1.6) les teinte avec le phosphore déclaré par la
/// machine (vert, blanc ou ambre de la v1.0).
pub const ALLUME: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
pub const ETEINT: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];

/// Adresse dans la 2513 d'un code ASCII ($20-$5F) : 6 bits, b6 inversé.
pub fn code_rom(ascii: u8) -> usize {
    let a = ascii & 0x7F;
    ((a & 0x1F) | (((a ^ 0x40) & 0x40) >> 1)) as usize
}

/// Les 5 points de la ligne `ligne` (0..8) du caractère ; bit 4 = à gauche.
pub fn ligne_glyphe(ascii: u8, ligne: usize) -> u8 {
    CHARGEN[(code_rom(ascii) << 3) | ligne] & 0x1F
}

/// Image de l'écran ; `curseur` = position du « @ » clignotant s'il est visible.
pub fn image_texte(
    cases: &dyn Fn(usize, usize) -> u8,
    curseur: Option<(usize, usize)>,
) -> ImageVideo {
    let mut px = vec![ETEINT; LARGEUR * HAUTEUR];
    for r in 0..ROWS {
        for c in 0..COLS {
            let ch = if curseur == Some((r, c)) {
                b'@'
            } else {
                cases(r, c)
            };
            if ch == b' ' {
                continue;
            }
            for gy in 0..CASE_H {
                let bits = ligne_glyphe(ch, gy);
                for gx in 0..5 {
                    if bits & (0x10 >> gx) != 0 {
                        px[(r * CASE_H + gy) * LARGEUR + c * CASE_L + gx] = ALLUME;
                    }
                }
            }
        }
    }
    ImageVideo {
        largeur: LARGEUR,
        hauteur: HAUTEUR,
        pixels: px,
    }
}

/// Image de la machine, curseur clignotant (minuterie 555 de la carte).
pub fn image(m: &Machine) -> ImageVideo {
    let d = &m.board.display;
    let visible = CursorBlink::apple1().visible_at(m.board.now_secs());
    let curseur = visible.then_some((d.row, d.col.min(COLS - 1)));
    image_texte(&|r, c| d.cell(r, c), curseur)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimensions_et_glyphe() {
        let img = image_texte(&|_, _| b' ', None);
        assert_eq!(
            (img.largeur, img.hauteur, img.pixels.len()),
            (280, 192, 280 * 192)
        );
        assert!(img.pixels.iter().all(|p| *p == ETEINT));
        // « @ » a des points allumés
        assert!((0..8).any(|l| ligne_glyphe(b'@', l) != 0));
        let img = image_texte(&|_, _| b' ', Some((0, 0)));
        assert!(img.pixels.contains(&ALLUME));
    }
}
