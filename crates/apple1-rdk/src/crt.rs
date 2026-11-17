//! « Écran vintage (screening) » de la v1.0 (`apple1-gui/src/crt.rs`),
//! calculé **côté machine** : l'interface commune affiche l'image telle
//! quelle (pilote `video-numerique`), sans effet.
//!
//! Même calcul que la v1.0, sur le processeur : rémanence du phosphore, halo,
//! points adoucis horizontalement, puis l'image « cuite » à [`FACTEUR`] fois
//! la résolution de l'Apple 1 (840 × 576 pour le terminal) avec la fine ligne
//! sombre entre les lignes de balayage, le tube bombé et ses coins
//! assombris, le scintillement et, sur le moniteur couleur, le masque de
//! phosphores rouge, vert, bleu. Le phosphore lui-même (vert, blanc, ambre)
//! est appliqué par le moniteur monochrome de l'interface commune 0.1.6
//! (`VideoCouleur::phosphore_monochrome`).
//!
//! Avec l'écran vintage, l'image des moniteurs est à 3 fois la résolution :
//! le zoom ×1 de l'interface commune correspond au zoom 3 de la v1.0. Sans
//! lui, l'image native (280 × 192) est agrandie par le zoom commun.

use rdk_pilotes::ImageVideo;
use std::time::Instant;

/// Agrandissement de l'image des moniteurs (zoom 3 de la v1.0).
pub const FACTEUR: usize = 3;

/// Réglages de l'écran vintage (défauts de la v1.0, `Screening::default`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReglagesEcran {
    pub actif: bool,
    /// 0 à 1 : profondeur des lignes, force du flou et du halo.
    pub intensite: f32,
    /// Séparation des lignes de balayage (l'effet principal).
    pub lignes: bool,
    /// Points adoucis horizontalement.
    pub adoucir: bool,
    /// Halo lumineux.
    pub halo: bool,
    /// Rémanence du phosphore, 0 (aucune) à 1 (longue).
    pub remanence: f32,
    /// Tube bombé et coins assombris.
    pub bombe: bool,
    /// Courbure, 0 à 1.
    pub courbure: f32,
    pub scintillement: bool,
    /// Masque de phosphores RVB (moniteur couleur).
    pub masque_rvb: bool,
}

impl Default for ReglagesEcran {
    fn default() -> Self {
        ReglagesEcran {
            actif: true,
            intensite: 0.6,
            lignes: true,
            adoucir: true,
            halo: true,
            remanence: 0.3,
            bombe: true,
            courbure: 0.07,
            scintillement: false,
            masque_rvb: true,
        }
    }
}

impl ReglagesEcran {
    /// Facteur de distorsion du tube (0 = plat), comme la v1.0.
    pub fn distorsion(&self) -> f32 {
        if self.bombe {
            0.01 + 0.11 * self.courbure.clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}

/// Phosphores de la v1.0 : couleur, temps de rémanence (s) à pleine force.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Phosphore {
    #[default]
    Vert,
    Blanc,
    Ambre,
}

impl Phosphore {
    pub const TOUS: [Phosphore; 3] = [Phosphore::Vert, Phosphore::Blanc, Phosphore::Ambre];
    pub fn nom(self) -> &'static str {
        match self {
            Phosphore::Vert => "Vert",
            Phosphore::Blanc => "Blanc",
            Phosphore::Ambre => "Ambre",
        }
    }
    pub fn cle(self) -> &'static str {
        match self {
            Phosphore::Vert => "vert",
            Phosphore::Blanc => "blanc",
            Phosphore::Ambre => "ambre",
        }
    }
    pub fn depuis_cle(c: &str) -> Phosphore {
        match c.trim() {
            "blanc" => Phosphore::Blanc,
            "ambre" => Phosphore::Ambre,
            _ => Phosphore::Vert,
        }
    }
    /// Couleurs de la v1.0 (`screen::Phosphor::fg` et `bg`).
    pub fn couleurs(self) -> rdk_pilotes::Phosphore {
        let (encre, fond) = match self {
            Phosphore::Vert => ([0x44, 0xFF, 0x66], [0x04, 0x0C, 0x06]),
            Phosphore::Blanc => ([0xE8, 0xEE, 0xFF], [0x08, 0x08, 0x0B]),
            Phosphore::Ambre => ([0xFF, 0xB0, 0x00], [0x0C, 0x07, 0x02]),
        };
        rdk_pilotes::Phosphore { encre, fond }
    }
    /// Rémanence (s) : P4 blanc courte, P1 vert moyenne, P3 ambre longue.
    pub fn tau(self) -> f32 {
        match self {
            Phosphore::Vert => 0.06,
            Phosphore::Blanc => 0.03,
            Phosphore::Ambre => 0.15,
        }
    }
}

/// Rémanence du moniteur couleur (v1.0 : celle du blanc).
pub const TAU_COULEUR: f32 = 0.03;

fn vers_lineaire(c: u8) -> f32 {
    let v = c as f32 / 255.0;
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

struct Tables {
    lin: [f32; 256],
    srgb: Vec<u8>,
}

const PAS_SRGB: usize = 8192;

fn tables() -> &'static Tables {
    static T: std::sync::OnceLock<Tables> = std::sync::OnceLock::new();
    T.get_or_init(|| {
        let mut lin = [0.0; 256];
        for (i, v) in lin.iter_mut().enumerate() {
            *v = vers_lineaire(i as u8);
        }
        let srgb = (0..PAS_SRGB)
            .map(|i| {
                let v = i as f32 / (PAS_SRGB - 1) as f32;
                let s = if v <= 0.003_130_8 {
                    v * 12.92
                } else {
                    1.055 * v.powf(1.0 / 2.4) - 0.055
                };
                (s * 255.0 + 0.5).clamp(0.0, 255.0) as u8
            })
            .collect();
        Tables { lin, srgb }
    })
}

fn encoder(v: f32) -> u8 {
    tables().srgb[(v.clamp(0.0, 1.0) * (PAS_SRGB - 1) as f32 + 0.5) as usize]
}

/// Le tube d'un moniteur : garde la rémanence d'une image à l'autre.
#[derive(Default)]
pub struct Tube {
    acc: Vec<[f32; 3]>,
    taille: (usize, usize),
    dernier: Option<Instant>,
    /// La rémanence s'éteint encore : il faut recalculer à l'image suivante.
    pub actif: bool,
    /// Dernière image source et réglages, pour ne pas recalculer pour rien.
    source: Vec<[u8; 4]>,
    reglages: Option<ReglagesEcran>,
    cache: Option<ImageVideo>,
    depart: Option<Instant>,
}

impl Tube {
    /// Image du moniteur vue à travers le tube. `fond` : couleur d'un point
    /// éteint de `src`, `tau` : rémanence du phosphore, `couleur` : masque RVB possible.
    pub fn image(
        &mut self,
        src: ImageVideo,
        r: &ReglagesEcran,
        tau: f32,
        couleur: bool,
    ) -> ImageVideo {
        let maintenant = Instant::now();
        let dt = self
            .dernier
            .map(|d| (maintenant - d).as_secs_f32())
            .unwrap_or(0.0);
        self.dernier = Some(maintenant);
        let depart = *self.depart.get_or_insert(maintenant);
        if !r.actif {
            // écran vintage éteint : l'image native (280 × 192), agrandie
            // par le zoom du moniteur de la partie commune
            self.acc.clear();
            self.cache = None;
            return src;
        }
        let meme = self.source == src.pixels
            && self.reglages == Some(*r)
            && !self.actif
            && !r.scintillement;
        if meme {
            if let Some(c) = &self.cache {
                return c.clone();
            }
        }
        let traite = self.traiter(&src, r, tau, dt);
        let phase = (maintenant - depart).as_secs_f64();
        let img = cuire(
            &traite,
            src.largeur,
            src.hauteur,
            r,
            couleur,
            teinte(r, phase),
        );
        self.source = src.pixels;
        self.reglages = Some(*r);
        self.cache = Some(img.clone());
        img
    }

    /// v1.0 `Crt::process` : rémanence, halo, points adoucis. Renvoie une
    /// image de `largeur × FACTEUR` sur `hauteur`.
    fn traiter(&mut self, src: &ImageVideo, r: &ReglagesEcran, tau: f32, dt: f32) -> ImageVideo {
        let t = tables();
        let (w, h) = (src.largeur, src.hauteur);
        let n = w * h;
        if self.acc.len() != n || self.taille != (w, h) {
            self.acc = vec![[0.0; 3]; n];
            self.taille = (w, h);
        }
        // fond noir (points éteints) : la teinte du phosphore vient ensuite
        // (interface commune 0.1.6), comme le fond du moniteur de la v1.0
        let fond = [0u8, 0, 0, 255];
        let fl = [
            t.lin[fond[0] as usize],
            t.lin[fond[1] as usize],
            t.lin[fond[2] as usize],
        ];
        let k = r.intensite.clamp(0.0, 1.0);
        let ag = r.remanence.clamp(0.0, 1.0);
        let decroit = if ag > 0.0 {
            (-dt.max(0.0) / (tau * ag)).exp()
        } else {
            0.0
        };
        let mut actif = false;
        for (a, p) in self.acc.iter_mut().zip(&src.pixels) {
            let sig = [
                (t.lin[p[0] as usize] - fl[0]).max(0.0),
                (t.lin[p[1] as usize] - fl[1]).max(0.0),
                (t.lin[p[2] as usize] - fl[2]).max(0.0),
            ];
            for c in 0..3 {
                let reste = a[c] * decroit;
                if reste > sig[c] + 0.002 {
                    a[c] = reste;
                    actif = true;
                } else {
                    a[c] = sig[c];
                }
            }
        }
        self.actif = actif;
        let mut v = self.acc.clone();
        if r.halo && k > 0.0 {
            let g = flou(&self.acc, w, h, 1.3);
            let q = 0.05 * k;
            for (o, g) in v.iter_mut().zip(&g) {
                for c in 0..3 {
                    o[c] += q * g[c];
                }
            }
        }
        let sx = FACTEUR;
        let ww = w * sx;
        let fond_px = [fond[0], fond[1], fond[2], 255];
        let mut out = vec![fond_px; ww * h];
        let px = |c: [f32; 3]| -> [u8; 4] {
            let p = [fl[0] + c[0], fl[1] + c[1], fl[2] + c[2]];
            // les points très lumineux tirent vers le blanc (phosphore saturé)
            let sur = (p[0].max(p[1]).max(p[2]) - 1.0).max(0.0) * 0.5;
            [
                encoder(p[0] + sur),
                encoder(p[1] + sur),
                encoder(p[2] + sur),
                255,
            ]
        };
        let poids = 0.18 * (0.5 + k);
        let melange = |a: [f32; 3], b: [f32; 3]| {
            [
                (a[0] + poids * b[0]) / (1.0 + poids),
                (a[1] + poids * b[1]) / (1.0 + poids),
                (a[2] + poids * b[2]) / (1.0 + poids),
            ]
        };
        let zero = [0.0f32; 3];
        let allume = |p: [f32; 3]| p[0] + p[1] + p[2] > 0.0;
        for y in 0..h {
            let ligne = &v[y * w..(y + 1) * w];
            let o = &mut out[y * ww..(y + 1) * ww];
            for x in 0..w {
                let c = ligne[x];
                let g = if x > 0 { ligne[x - 1] } else { zero };
                let d = if x + 1 < w { ligne[x + 1] } else { zero };
                let cases = &mut o[x * sx..(x + 1) * sx];
                if !r.adoucir {
                    if allume(c) {
                        cases.fill(px(c));
                    }
                    continue;
                }
                if !(allume(c) || allume(g) || allume(d)) {
                    continue;
                }
                let milieu = if allume(c) { px(c) } else { fond_px };
                for m in cases[1..sx - 1].iter_mut() {
                    *m = milieu;
                }
                cases[0] = px(melange(c, g));
                cases[sx - 1] = px(melange(c, d));
            }
        }
        ImageVideo {
            largeur: ww,
            hauteur: h,
            pixels: out,
        }
    }
}

/// Flou gaussien séparable (sigma en points), points éteints sautés.
fn flou(src: &[[f32; 3]], w: usize, h: usize, sigma: f32) -> Vec<[f32; 3]> {
    let r = (sigma * 2.5).ceil() as isize;
    let noyau: Vec<f32> = (-r..=r)
        .map(|i| (-(i * i) as f32 / (2.0 * sigma * sigma)).exp())
        .collect();
    let somme: f32 = noyau.iter().sum();
    let passe = |entree: &[[f32; 3]], horizontal: bool| {
        let mut sortie = vec![[0.0f32; 3]; w * h];
        for y in 0..h {
            for x in 0..w {
                let p = entree[y * w + x];
                if p[0] + p[1] + p[2] <= 0.0 {
                    continue;
                }
                for (j, kw) in noyau.iter().enumerate() {
                    let d = j as isize - r;
                    let (xx, yy) = if horizontal {
                        (x as isize + d, y as isize)
                    } else {
                        (x as isize, y as isize + d)
                    };
                    if xx < 0 || yy < 0 || xx >= w as isize || yy >= h as isize {
                        continue;
                    }
                    let o = &mut sortie[yy as usize * w + xx as usize];
                    let f = kw / somme;
                    for i in 0..3 {
                        o[i] += f * p[i];
                    }
                }
            }
        }
        sortie
    };
    passe(&passe(src, true), false)
}

/// Tube bombé (v1.0 `warp`) : un point en coordonnées -1..1 est tiré vers le centre.
pub fn bomber(u: f32, v: f32, k: f32) -> (f32, f32) {
    let f = (1.0 - k * (u * u + v * v)) / (1.0 - k);
    (u * f, v * f)
}

/// Inverse de [`bomber`] (v1.0 `unwarp`).
pub fn debomber(a: f32, b: f32, k: f32) -> (f32, f32) {
    let (mut u, mut v) = (a, b);
    for _ in 0..8 {
        let f = (1.0 - k * (u * u + v * v)) / (1.0 - k);
        u = a / f;
        v = b / f;
    }
    (u, v)
}

/// Épaisseur (pixels) et noirceur (0..255) de la ligne sombre entre deux
/// lignes de balayage hautes de `ligne_px` pixels (v1.0 `gap`).
pub fn interligne(ligne_px: f32, intensite: f32) -> Option<(f32, u8)> {
    if ligne_px < 2.0 {
        return None;
    }
    let k = intensite.clamp(0.0, 1.0);
    let t = (ligne_px / 5.0).round().max(1.0);
    let frac = t / ligne_px;
    let noir = (0.45 + 0.5 * k) * if frac > 0.25 { 0.25 / frac } else { 1.0 };
    Some((t, (noir * 255.0).round().clamp(0.0, 255.0) as u8))
}

/// Luminosité (0..1) multipliée à l'image : le scintillement (v1.0 `tint_gray`).
pub fn teinte(r: &ReglagesEcran, phase: f64) -> f32 {
    if r.scintillement {
        let k = r.intensite.clamp(0.0, 1.0);
        let n = ((phase * 7.3).sin() * 0.5 + (phase * 59.0).sin() * 0.5) as f32;
        (255.0 - (6.0 + 10.0 * k) * (0.5 + 0.5 * n)).round() / 255.0
    } else {
        1.0
    }
}

/// Poids des trois raies du masque de phosphores (v1.0 `mask_weights`).
pub fn poids_masque(intensite: f32) -> [[f32; 3]; 3] {
    let bas = 1.0 - 0.7 * intensite.clamp(0.0, 1.0);
    let gain = 3.0 / (1.0 + 2.0 * bas);
    let mut w = [[bas * gain; 3]; 3];
    for (i, l) in w.iter_mut().enumerate() {
        l[i] = gain;
    }
    w
}

/// v1.0 `bake` + coins assombris : l'image traitée (`w × FACTEUR` sur `h`)
/// devient l'image du tube, `w × FACTEUR` sur `h × FACTEUR`.
fn cuire(
    src: &ImageVideo,
    w: usize,
    h: usize,
    r: &ReglagesEcran,
    couleur: bool,
    lumiere: f32,
) -> ImageVideo {
    let (lw, lh) = (w * FACTEUR, h * FACTEUR);
    let sw = src.largeur;
    let k = r.intensite.clamp(0.0, 1.0);
    let courbe = r.distorsion();
    let ligne_px = lh as f32 / h as f32;
    let lignes = if r.lignes {
        interligne(ligne_px, k)
    } else {
        None
    };
    let poids = poids_masque(k);
    let mut out = vec![[0u8, 0, 0, 255]; lw * lh];
    for y in 0..lh {
        let b = (y as f32 + 0.5) / lh as f32 * 2.0 - 1.0;
        for x in 0..lw {
            let a = (x as f32 + 0.5) / lw as f32 * 2.0 - 1.0;
            let (u, v) = if courbe > 0.0 {
                debomber(a, b, courbe)
            } else {
                (a, b)
            };
            if u.abs() >= 1.0 || v.abs() >= 1.0 {
                continue;
            }
            let ligne = (((v * 0.5 + 0.5) * h as f32) as usize).min(h - 1);
            let col = (((u * 0.5 + 0.5) * sw as f32) as usize).min(sw - 1);
            let p = src.pixels[ligne * sw + col];
            let mut f = lumiere;
            if let Some((t, noir)) = lignes {
                let dans = if courbe == 0.0 {
                    let yb = ((ligne + 1) as f32 * lh as f32 / h as f32).round() as usize;
                    y < yb && y + t as usize >= yb
                } else {
                    (v * 0.5 + 0.5) * h as f32 - ligne as f32 > 1.0 - t / ligne_px
                };
                if dans {
                    f *= 1.0 - noir as f32 / 255.0;
                }
            }
            if r.bombe {
                // coins assombris (v1.0 `vignette`)
                let rr = (u * u * 0.6 + v * v * 0.6).min(1.2);
                let alpha = ((rr * rr) * (30.0 + 1000.0 * courbe)).clamp(0.0, 200.0) / 255.0;
                f *= 1.0 - alpha;
            }
            let wt = if couleur && r.masque_rvb {
                poids[x % 3]
            } else {
                [1.0; 3]
            };
            let c = |v: u8, w: f32| (v as f32 * w * f).round().clamp(0.0, 255.0) as u8;
            out[y * lw + x] = [c(p[0], wt[0]), c(p[1], wt[1]), c(p[2], wt[2]), 255];
        }
    }
    ImageVideo {
        largeur: lw,
        hauteur: lh,
        pixels: out,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn damier(w: usize, h: usize) -> ImageVideo {
        let px = (0..w * h)
            .map(|i| {
                if (i / w + i % w).is_multiple_of(2) {
                    [255, 255, 255, 255]
                } else {
                    [0, 0, 0, 255]
                }
            })
            .collect();
        ImageVideo {
            largeur: w,
            hauteur: h,
            pixels: px,
        }
    }

    #[test]
    fn taille_3x_avec_effet_native_sans() {
        let mut t = Tube::default();
        let r = ReglagesEcran::default();
        let a = t.image(damier(280, 192), &r, Phosphore::Vert.tau(), false);
        assert_eq!((a.largeur, a.hauteur), (840, 576));
        let b = t.image(
            damier(280, 192),
            &ReglagesEcran { actif: false, ..r },
            0.06,
            false,
        );
        assert_eq!((b.largeur, b.hauteur), (280, 192));
        assert_eq!(b.pixels[0], [255, 255, 255, 255]);
    }

    #[test]
    fn lignes_de_balayage_plus_sombres() {
        let mut t = Tube::default();
        let blanc = ImageVideo {
            largeur: 4,
            hauteur: 4,
            pixels: vec![[255, 255, 255, 255]; 16],
        };
        let r = ReglagesEcran {
            bombe: false,
            halo: false,
            adoucir: false,
            remanence: 0.0,
            ..ReglagesEcran::default()
        };
        let i = t.image(blanc, &r, 0.06, false);
        // ligne 0 : rangées 0 et 1 allumées, rangée 2 = interligne
        let l = |y: usize| i.pixels[y * i.largeur + 5][0];
        assert!(l(0) > l(2), "{} {}", l(0), l(2));
        assert_eq!(l(0), l(1));
    }

    #[test]
    fn remanence_s_eteint() {
        let mut t = Tube::default();
        let r = ReglagesEcran {
            remanence: 1.0,
            ..ReglagesEcran::default()
        };
        let blanc = ImageVideo {
            largeur: 8,
            hauteur: 8,
            pixels: vec![[255, 255, 255, 255]; 64],
        };
        let noir = ImageVideo {
            largeur: 8,
            hauteur: 8,
            pixels: vec![[0, 0, 0, 255]; 64],
        };
        t.image(blanc, &r, 0.15, false);
        t.image(noir, &r, 0.15, false);
        assert!(t.actif, "la rémanence garde les points allumés un moment");
    }

    #[test]
    fn phosphores_de_la_v1() {
        assert_eq!(Phosphore::Vert.couleurs().encre, [0x44, 0xFF, 0x66]);
        for p in Phosphore::TOUS {
            assert_eq!(Phosphore::depuis_cle(p.cle()), p);
        }
        assert!(interligne(1.0, 0.5).is_none());
        assert_eq!(interligne(3.0, 0.6).map(|g| g.0), Some(1.0));
    }
}
