//! Miniature de chaque configuration pour l'écran « Votre configuration »
//! de l'assistant de démarrage (contrat des pilotes 1.2.0,
//! `Machine::manifeste_illustration`).
//!
//! Choix de Wirehead (24/11/2026) : un **dessin du kit**, en attendant une
//! meilleure illustration. Il est calculé ici, pixel par pixel, sans aucune
//! image ni marque : la carte Apple 1 (circuit vert, rangées de puces,
//! connecteur d'extension), le moniteur vert, et selon les périphériques de
//! la configuration la carte cassette ACI, la carte couleur GEN2 ou la carte
//! graphique P-LAB, et le moniteur couleur.

use rdk_pilotes::ImageVideo;

/// Taille de la miniature (pixels), celle que conseille le contrat.
pub const LARGEUR: usize = 240;
pub const HAUTEUR: usize = 160;

/// Ce que montre la miniature.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Contenu {
    pub moniteur_vert: bool,
    pub moniteur_couleur: bool,
    pub cassette: bool,
    pub gen2: bool,
    pub plab: bool,
}

type Rgba = [u8; 4];

const CIRCUIT: Rgba = [36, 112, 62, 255];
const CIRCUIT_BORD: Rgba = [22, 78, 42, 255];
const PISTE: Rgba = [58, 140, 84, 255];
const PUCE: Rgba = [28, 28, 30, 255];
const PATTE: Rgba = [186, 186, 176, 255];
const CONNECTEUR: Rgba = [214, 196, 150, 255];
const DORURE: Rgba = [222, 178, 64, 255];
const BOITIER: Rgba = [204, 194, 166, 255];
const BOITIER_OMBRE: Rgba = [150, 140, 116, 255];
const VERRE: Rgba = [14, 18, 14, 255];
const PHOSPHORE: Rgba = [80, 230, 110, 255];
const CONDENSATEUR: Rgba = [52, 92, 178, 255];
const CONDENSATEUR_HAUT: Rgba = [170, 176, 186, 255];

struct Toile {
    pixels: Vec<Rgba>,
}

impl Toile {
    fn new() -> Toile {
        Toile {
            pixels: vec![[0, 0, 0, 0]; LARGEUR * HAUTEUR],
        }
    }

    /// Rectangle plein, `x1` et `y1` exclus.
    fn rect(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, c: Rgba) {
        for y in y0..y1.min(HAUTEUR) {
            for x in x0..x1.min(LARGEUR) {
                self.pixels[y * LARGEUR + x] = c;
            }
        }
    }

    /// Rectangle plein bordé d'un trait d'un pixel.
    fn cadre(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, fond: Rgba, bord: Rgba) {
        self.rect(x0, y0, x1, y1, bord);
        self.rect(x0 + 1, y0 + 1, x1 - 1, y1 - 1, fond);
    }

    /// Disque plein de centre (`cx`, `cy`) et de rayon `r`.
    fn disque(&mut self, cx: i32, cy: i32, r: i32, c: Rgba) {
        for y in (cy - r).max(0)..=(cy + r).min(HAUTEUR as i32 - 1) {
            for x in (cx - r).max(0)..=(cx + r).min(LARGEUR as i32 - 1) {
                if (x - cx) * (x - cx) + (y - cy) * (y - cy) <= r * r {
                    self.pixels[y as usize * LARGEUR + x as usize] = c;
                }
            }
        }
    }

    /// Puce DIP vue de dessus, pattes en haut et en bas.
    fn puce(&mut self, x: usize, y: usize, l: usize) {
        let mut px = x + 1;
        while px + 1 < x + l {
            self.rect(px, y, px + 1, y + 1, PATTE);
            self.rect(px, y + 6, px + 1, y + 7, PATTE);
            px += 2;
        }
        self.rect(x, y + 1, x + l, y + 6, PUCE);
    }

    /// Carte d'extension enfichée dans le connecteur, vue de face.
    fn carte(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, circuit: Rgba, bord: Rgba) {
        self.cadre(x0, y0, x1, y1, circuit, bord);
        self.rect(x0 + 2, y1 - 3, x1 - 2, y1, DORURE);
    }

    /// Moniteur à tube : boîtier, verre, socle ; `ecran` peint l'image.
    fn moniteur(
        &mut self,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        ecran: &dyn Fn(&mut Toile, usize, usize, usize, usize),
    ) {
        self.cadre(x0, y0, x1, y1, BOITIER, BOITIER_OMBRE);
        self.rect(x0 + 5, y0 + 5, x1 - 5, y1 - 9, VERRE);
        ecran(self, x0 + 8, y0 + 8, x1 - 8, y1 - 12);
        self.rect(x1 - 14, y1 - 6, x1 - 8, y1 - 4, BOITIER_OMBRE);
        let milieu = (x0 + x1) / 2;
        self.rect(milieu - 8, y1, milieu + 8, y1 + 4, BOITIER_OMBRE);
    }

    fn image(self) -> ImageVideo {
        ImageVideo {
            largeur: LARGEUR,
            hauteur: HAUTEUR,
            pixels: self.pixels,
        }
    }
}

/// Lignes de texte vertes du moniteur 40 × 24.
fn texte_vert(t: &mut Toile, x0: usize, y0: usize, x1: usize, y1: usize) {
    let longueurs = [0.9, 0.55, 0.75, 0.35, 0.8, 0.6, 0.25];
    let mut y = y0 + 1;
    for l in longueurs.iter().cycle() {
        if y + 2 > y1 {
            break;
        }
        let fin = x0 + ((x1 - x0) as f32 * l) as usize;
        t.rect(x0, y, fin.max(x0 + 3), y + 2, PHOSPHORE);
        y += 5;
    }
    // curseur
    t.rect(
        x0,
        y.saturating_sub(5).max(y0),
        x0 + 3,
        y.saturating_sub(3).max(y0 + 2),
        PHOSPHORE,
    );
}

/// Image couleur : bandes verticales (GEN2) ou dessin en blocs (P-LAB).
fn image_couleur(plab: bool) -> impl Fn(&mut Toile, usize, usize, usize, usize) {
    move |t: &mut Toile, x0: usize, y0: usize, x1: usize, y1: usize| {
        let couleurs: [Rgba; 6] = [
            [226, 52, 48, 255],
            [242, 164, 40, 255],
            [238, 220, 64, 255],
            [64, 186, 84, 255],
            [52, 130, 226, 255],
            [150, 82, 206, 255],
        ];
        if plab {
            // ciel, soleil et collines en gros blocs
            t.rect(x0, y0, x1, y1, [40, 70, 160, 255]);
            let h = y1 - y0;
            t.rect(x0, y0 + h * 2 / 3, x1, y1, [60, 160, 70, 255]);
            t.rect(
                x0 + (x1 - x0) / 2,
                y0 + h / 2,
                x1,
                y0 + h * 2 / 3 + 1,
                [44, 128, 56, 255],
            );
            t.rect(x0 + 3, y0 + 3, x0 + 10, y0 + 10, [244, 214, 60, 255]);
        } else {
            let n = couleurs.len();
            for (i, c) in couleurs.iter().enumerate() {
                let a = x0 + (x1 - x0) * i / n;
                let b = x0 + (x1 - x0) * (i + 1) / n;
                t.rect(a, y0, b, y1, *c);
            }
        }
    }
}

/// La miniature de la configuration décrite par `c`.
pub fn dessiner(c: Contenu) -> ImageVideo {
    let mut t = Toile::new();

    // ----- carte Apple 1 (en bas)
    let (bx0, by0, bx1, by1) = (8, 96, 232, 156);
    t.cadre(bx0, by0, bx1, by1, CIRCUIT, CIRCUIT_BORD);
    // pistes
    for i in 0..5 {
        let y = by0 + 6 + i * 11;
        t.rect(bx0 + 3, y, bx1 - 3, y + 1, PISTE);
    }
    // rangées de puces
    for rang in 0..4 {
        let y = by0 + 7 + rang * 12;
        let mut x = 64;
        while x + 14 < bx1 - 4 {
            t.puce(x, y, 12);
            x += 17;
        }
    }
    // gros condensateurs de l'alimentation, à gauche
    for (i, cy) in [112, 136].iter().enumerate() {
        let cx = 22 + i as i32 * 18;
        t.disque(cx, *cy, 8, CONDENSATEUR);
        t.disque(cx, *cy, 4, CONDENSATEUR_HAUT);
    }
    // connecteur d'extension
    t.rect(16, by0 - 3, 100, by0 + 1, CONNECTEUR);

    // ----- cartes enfichées
    if c.cassette {
        // carte ACI : petite carte verte avec ses deux prises jack
        t.carte(18, 70, 44, by0, CIRCUIT, CIRCUIT_BORD);
        t.puce(22, 76, 8);
        t.disque(25, 88, 2, [30, 30, 30, 255]);
        t.disque(35, 88, 2, [30, 30, 30, 255]);
    }
    if c.gen2 {
        // carte couleur GEN2 : plus grande, avec ses trois couleurs de base
        t.carte(48, 56, 98, by0, [30, 96, 70, 255], CIRCUIT_BORD);
        t.puce(54, 62, 14);
        t.puce(74, 62, 14);
        t.puce(54, 74, 14);
        for (i, col) in [[226, 52, 48, 255], [64, 186, 84, 255], [52, 130, 226, 255]]
            .iter()
            .enumerate()
        {
            t.disque(78 + i as i32 * 6, 80, 2, *col);
        }
    }
    if c.plab {
        // carte graphique P-LAB : circuit bleu
        t.carte(48, 56, 98, by0, [40, 74, 140, 255], [24, 48, 96, 255]);
        t.puce(54, 62, 14);
        t.puce(74, 62, 14);
        t.puce(54, 74, 14);
        t.puce(74, 74, 14);
    }

    // ----- moniteurs (en haut à droite)
    let couleur = image_couleur(c.plab);
    match (c.moniteur_vert, c.moniteur_couleur) {
        (true, true) => {
            t.moniteur(104, 6, 168, 70, &couleur);
            t.moniteur(170, 14, 234, 78, &texte_vert);
        }
        (false, true) => t.moniteur(132, 8, 222, 82, &couleur),
        (true, false) => t.moniteur(132, 8, 222, 82, &texte_vert),
        (false, false) => {}
    }
    t.image()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn px(i: &ImageVideo, x: usize, y: usize) -> Rgba {
        i.pixels[y * i.largeur + x]
    }

    #[test]
    fn taille_et_fond_transparent() {
        let i = dessiner(Contenu::default());
        assert_eq!(
            (i.largeur, i.hauteur, i.pixels.len()),
            (LARGEUR, HAUTEUR, LARGEUR * HAUTEUR)
        );
        assert_eq!(px(&i, 0, 0)[3], 0);
        // la carte Apple 1 est toujours là
        assert_eq!(px(&i, 120, 98)[3], 255);
    }

    #[test]
    fn chaque_peripherique_se_voit() {
        let vert = Contenu {
            moniteur_vert: true,
            cassette: true,
            ..Contenu::default()
        };
        let gen2 = Contenu {
            moniteur_couleur: true,
            gen2: true,
            ..vert
        };
        let plab = Contenu {
            gen2: false,
            plab: true,
            ..gen2
        };
        let (a, b, c) = (dessiner(vert), dessiner(gen2), dessiner(plab));
        assert_ne!(a.pixels, b.pixels);
        assert_ne!(b.pixels, c.pixels);
        // sans carte d'extension, rien au-dessus du connecteur à gauche
        assert_eq!(px(&a, 70, 70)[3], 0);
        assert_eq!(px(&b, 50, 70)[3], 255);
        // carte ACI présente dans les trois
        assert_eq!(px(&a, 20, 80), CIRCUIT);
        // sans cassette, pas de carte ACI
        let sans = dessiner(Contenu {
            cassette: false,
            ..vert
        });
        assert_eq!(px(&sans, 20, 80)[3], 0);
    }
}
