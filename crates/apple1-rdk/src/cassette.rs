//! Son et oscilloscope de la cassette (v1.0 : `sound_tick` et `scope_ui` de
//! `apple1-gui/src/app.rs`), avec les modules repris de la v1.0
//! ([`crate::v1::audio`], [`crate::v1::tapesig`]) et ses textes.
//!
//! Le signal qui passe sous la tête (lecture 📼 de l'ACI ou écriture ⏺) est
//! rebâti à partir des fronts, envoyé à la carte son (ALSA, ouvert au
//! premier usage ; sans ALSA ni carte, le son reste coupé) et dessiné dans
//! la fenêtre « Oscilloscope » de la machine. Le son est coupé en vitesse
//! accélérée, comme en v1.0.

use crate::i18n::t;
use crate::v1::{audio, tapesig};
use apple1_core::machine::Board;
use eframe::egui::{self, Color32, RichText};

/// La cassette avance : (écriture ?, temps de la tête en secondes). Lecture
/// : temps sur la bande ; écriture : temps émulé des basculements.
pub fn tete(b: &Board) -> Option<(bool, f64)> {
    if b.recording.is_some() {
        return Some((true, b.now_secs()));
    }
    let p = b.player.as_ref()?;
    let maintenant = b.now_secs();
    (p.started_at.is_some() && !p.finished(maintenant)).then(|| (false, p.position(maintenant)))
}

/// Fronts du signal entre `t0` et `t1`.
pub fn fenetre_signal(b: &Board, ecriture: bool, t0: f64, t1: f64) -> tapesig::Window {
    if ecriture {
        let r = b.recording.as_deref().unwrap_or(&[]);
        let tm = b.config.timing;
        let a = r.partition_point(|&c| tm.cycles_to_secs(c) <= t0);
        let z = r.partition_point(|&c| tm.cycles_to_secs(c) <= t1);
        tapesig::Window {
            edges: r[a..z].iter().map(|&c| tm.cycles_to_secs(c)).collect(),
            level0: a % 2 == 1,
        }
    } else {
        b.player
            .as_ref()
            .map(|p| tapesig::window(&p.edges, t0, t1))
            .unwrap_or_default()
    }
}

/// Sortie son de la cassette.
#[derive(Default)]
pub struct Son {
    /// `None` : pas encore ouverte ; `Some(None)` : pas de carte son.
    carte: Option<Option<audio::Audio>>,
    /// Dernier temps envoyé à la carte (écriture ?, secondes).
    dernier: Option<(bool, f64)>,
    filtre: f32,
}

impl Son {
    /// Envoie à la carte son le signal passé sous la tête depuis l'image
    /// précédente (vitesse réelle seulement).
    pub fn avancer(&mut self, b: &Board, actif: bool, rapide: bool, volume: f32) {
        let (Some((w, t1)), true, false) = (tete(b), actif, rapide) else {
            self.dernier = None;
            return;
        };
        if self.carte.is_none() {
            self.carte = Some(audio::Audio::open());
        }
        let freq = audio::RATE as f64;
        let t0 = match self.dernier {
            Some((w0, t0)) if w0 == w && t0 <= t1 && t1 - t0 < 0.3 => t0,
            _ => t1 - 0.02,
        };
        let n = ((t1 - t0) * freq) as usize;
        let t1 = t0 + n as f64 / freq;
        self.dernier = Some((w, t1));
        let f = fenetre_signal(b, w, t0, t1);
        let ech = tapesig::render(
            &f,
            t0,
            t1,
            n,
            tapesig::alpha(5000.0, freq as f32),
            &mut self.filtre,
        );
        if let Some(Some(a)) = &self.carte {
            a.push(&ech, volume);
        }
    }

    /// Vrai si l'ouverture de la carte son a échoué.
    pub fn sans_carte(&self) -> bool {
        matches!(self.carte, Some(None))
    }
}

/// Réglages modifiés par la fenêtre.
pub struct ReglagesOscillo<'a> {
    pub son: &'a mut bool,
    pub volume: &'a mut f32,
    pub rapide: bool,
    pub sans_carte: bool,
}

/// Fenêtre « Oscilloscope » : le signal autour de la tête, trace verte sur
/// grille sombre (10 ms ; en écriture, les 10 dernières ms). Renvoie
/// `(ouverte, réglage changé)` ; rien n'est dessiné quand la cassette ne
/// bouge pas.
pub fn fenetre(ctx: &egui::Context, b: &Board, r: ReglagesOscillo<'_>) -> (bool, bool) {
    let Some((w, tete)) = tete(b) else {
        return (true, false);
    };
    let mut ouverte = true;
    let mut change = false;
    let (t0, t1) = if w {
        (tete - 0.010, tete)
    } else {
        (tete - 0.005, tete + 0.005)
    };
    let f = fenetre_signal(b, w, t0, t1);
    egui::Window::new(t("sc_title"))
        .id(egui::Id::new("apple1-oscilloscope"))
        .open(&mut ouverte)
        .default_size([380.0, 170.0])
        .default_pos([24.0, (ctx.content_rect().bottom() - 240.0).max(80.0)])
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(if w { t("sc_write") } else { t("sc_read") }).small());
                change |= ui
                    .checkbox(r.son, t("sc_sound"))
                    .on_hover_text(t("sc_sound_tip"))
                    .changed();
                change |= ui
                    .add(egui::Slider::new(r.volume, 0.0..=1.0).fixed_decimals(2))
                    .on_hover_text(t("sc_volume_tip"))
                    .changed();
                if r.rapide && *r.son {
                    ui.label(RichText::new(t("sc_muted")).small().weak());
                } else if r.sans_carte && *r.son {
                    ui.label(RichText::new(t("sc_no_audio")).small().weak());
                }
            });
            let taille = egui::vec2(
                ui.available_width().max(200.0),
                ui.available_height().max(90.0),
            );
            let (rect, rep) = ui.allocate_exact_size(taille, egui::Sense::hover());
            let p = ui.painter_at(rect);
            p.rect_filled(rect, 3.0, Color32::from_rgb(6, 14, 8));
            let grille = egui::Stroke::new(1.0, Color32::from_rgb(22, 52, 28));
            for i in 1..10 {
                let x = rect.left() + rect.width() * i as f32 / 10.0;
                p.line_segment(
                    [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                    grille,
                );
            }
            for i in 1..4 {
                let y = rect.top() + rect.height() * i as f32 / 4.0;
                p.line_segment(
                    [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                    grille,
                );
            }
            let n = (rect.width() as usize * 2).max(2);
            let mut lp = if f.level0 { 1.0 } else { -1.0 };
            let a = tapesig::alpha(4000.0, n as f32 / 0.010);
            let ys = tapesig::render(&f, t0, t1, n, a, &mut lp);
            let points: Vec<egui::Pos2> = ys
                .iter()
                .enumerate()
                .map(|(i, y)| {
                    egui::pos2(
                        rect.left() + rect.width() * i as f32 / (n - 1) as f32,
                        rect.center().y - y * rect.height() * 0.36,
                    )
                })
                .collect();
            p.add(egui::Shape::line(
                points,
                egui::Stroke::new(1.6, Color32::from_rgb(70, 255, 110)),
            ));
            // la tête
            let hx = if w {
                rect.right() - 1.0
            } else {
                rect.center().x
            };
            p.line_segment(
                [egui::pos2(hx, rect.top()), egui::pos2(hx, rect.bottom())],
                egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 200, 80, 90)),
            );
            p.text(
                rect.left_bottom() + egui::vec2(4.0, -3.0),
                egui::Align2::LEFT_BOTTOM,
                t("sc_div"),
                egui::FontId::monospace(11.0),
                Color32::from_rgb(70, 140, 80),
            );
            rep.on_hover_text(t("sc_tip"));
        });
    ctx.request_repaint();
    (ouverte, change)
}
