//! « 🎨 Photo → ASCII » de la v1.0 (`apple1-gui/src/ascii.rs`) : une photo
//! (jpg, png) devient un art ASCII de 40 × 24 caractères de l'Apple 1
//! (`apple1_core::asciiart`), montré avec la police du 2513 dans la couleur
//! du phosphore, puis un programme Integer BASIC ou 6502 lancé sur la
//! machine, enregistré dans la bibliothèque (`BASIC/`, `Assembleur/`),
//! une cassette (`Cassettes/`) ou un `.txt` du dossier `Art ASCII/`.

use crate::i18n::tr;
use crate::trf;
use crate::MachineApple1;
use apple1_core::asciiart::{self, Art, Grey, Params};
use apple1_core::display::{COLS, ROWS};
use apple1_core::{asm6502, intbasic, program};
use eframe::egui::{self, Color32, ColorImage, RichText, TextureHandle, TextureOptions};
use rdk_interface::outils::selecteur::{self, Raccourci};
use rdk_pilotes::Langage;
use std::path::{Path, PathBuf};

struct Photo {
    gris: Grey,
    vignette: ColorImage,
    texture: Option<TextureHandle>,
}

/// Réglages de l'image (v1.0 : `AsciiCfg`).
#[derive(Clone, PartialEq)]
struct Reglages {
    colonnes: usize,
    lignes: usize,
    zoom: f32,
    cx: f32,
    cy: f32,
    luminosite: f32,
    contraste: f32,
    gamma: f32,
    inverser: bool,
    niveaux_auto: bool,
    tramage: bool,
    /// 0 complète, 1 simple, 2 Apple1-Slideshow, 3 personnalisée.
    rampe: usize,
    rampe_perso: String,
    /// Programme 6502 (sinon Integer BASIC).
    asm: bool,
}

impl Default for Reglages {
    fn default() -> Self {
        let p = Params::default();
        Reglages {
            colonnes: p.cols,
            lignes: p.rows,
            zoom: p.zoom,
            cx: p.cx,
            cy: p.cy,
            luminosite: p.brightness,
            contraste: p.contrast,
            gamma: p.gamma,
            inverser: p.invert,
            niveaux_auto: p.auto_level,
            tramage: p.dither,
            rampe: 0,
            rampe_perso: asciiart::RAMP_SIMPLE.into(),
            asm: false,
        }
    }
}

impl Reglages {
    fn rampe_texte(&self) -> String {
        match self.rampe {
            1 => asciiart::RAMP_SIMPLE.into(),
            2 => asciiart::RAMP_SLIDESHOW.into(),
            3 => self.rampe_perso.clone(),
            _ => asciiart::RAMP_FULL.into(),
        }
    }
    fn params(&self) -> Params {
        Params {
            cols: self.colonnes,
            rows: self.lignes,
            zoom: self.zoom,
            cx: self.cx,
            cy: self.cy,
            brightness: self.luminosite,
            contrast: self.contraste,
            gamma: self.gamma,
            invert: self.inverser,
            auto_level: self.niveaux_auto,
            dither: self.tramage,
            ramp: self.rampe_texte(),
            ..Params::default()
        }
    }
}

#[derive(Default)]
pub struct OutilAscii {
    pub ouvert: bool,
    pub nom: String,
    reglages: Reglages,
    photo: Option<Photo>,
    /// Art ouvert depuis un `.txt` (sans photo).
    art_texte: Option<Art>,
    art: Art,
    fait_pour: Option<Params>,
    apercu: Option<(TextureHandle, Art, crate::crt::Phosphore)>,
    voir_code: bool,
    chemin: String,
    parcourir: bool,
    message: Option<(String, bool)>,
}

/// Photo décodée en niveaux de gris et vignette (360 px au plus).
pub fn decoder(chemin: &Path) -> Result<(Grey, ColorImage), String> {
    let img = image::open(chemin).map_err(|e| trf!("{0} : {1}", chemin.display(), e))?;
    let rgba = img.to_rgba8();
    let gris = Grey::from_rgba(
        rgba.width() as usize,
        rgba.height() as usize,
        rgba.as_raw(),
        4,
    );
    let th = img.thumbnail(360, 360).to_rgba8();
    Ok((
        gris,
        ColorImage::from_rgba_unmultiplied(
            [th.width() as usize, th.height() as usize],
            th.as_raw(),
        ),
    ))
}

fn souche(p: &Path) -> String {
    p.file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Le programme d'un art : source 6502 (`300R`) ou listing Integer BASIC.
pub fn code(art: &Art, nom: &str, asm: bool) -> String {
    let nom = if nom.is_empty() { "ASCII ART" } else { nom };
    if asm {
        asciiart::to_asm(art, nom)
    } else {
        asciiart::to_basic(art, nom)
    }
}

/// Programme en mémoire (blocs) pour une cassette.
pub fn programme(art: &Art, nom: &str, asm: bool) -> Result<program::Program, String> {
    let source = code(art, nom, asm);
    if asm {
        let a = asm6502::assemble_text(&source, None, &[])
            .map_err(|e| e.first().map(|x| x.to_string()).unwrap_or_default())?;
        Ok(program::Program {
            blocks: vec![(a.origin, a.bytes)],
            run: Some(a.origin),
            notes: vec![],
        })
    } else {
        let l = intbasic::tokenize(&source);
        if let Some(e) = l.errors.first() {
            return Err(format!("{e:?}"));
        }
        let (lomem, himem) = intbasic::layout(l.size(), None);
        let img = intbasic::Image::build(&l.lines, lomem, himem).map_err(|e| format!("{e:?}"))?;
        Ok(program::Program {
            blocks: img.blocks(),
            run: None,
            notes: vec![],
        })
    }
}

/// Image 280 × 192 de l'art avec la police du 2513, teintée par le phosphore.
pub fn image_art(art: &Art, ph: crate::crt::Phosphore) -> ColorImage {
    let mut cases = [[b' '; COLS]; ROWS];
    for (r, l) in art.lines.iter().enumerate().take(ROWS) {
        for (c, b) in l.bytes().enumerate().take(COLS) {
            cases[r][c] = b;
        }
    }
    let img = crate::ecran::image_texte(&|r, c| cases[r][c], None);
    let p = ph.couleurs();
    let px: Vec<Color32> = img
        .pixels
        .iter()
        .map(|q| {
            let t = p.teinter(q[1], 255);
            Color32::from_rgb(t[0], t[1], t[2])
        })
        .collect();
    ColorImage {
        size: [img.largeur, img.hauteur],
        pixels: px,
        source_size: egui::vec2(img.largeur as f32, img.hauteur as f32),
    }
}

impl OutilAscii {
    /// Ouvre une photo ou un `.txt` d'art ASCII.
    pub fn ouvrir_fichier(&mut self, chemin: &Path) -> Result<String, String> {
        if crate::art::est_image(&chemin.to_string_lossy()) {
            let (gris, vignette) = decoder(chemin)?;
            let (w, h) = (gris.w, gris.h);
            self.photo = Some(Photo {
                gris,
                vignette,
                texture: None,
            });
            self.art_texte = None;
            self.fait_pour = None;
            self.nom = souche(chemin);
            Ok(trf!("Photo « {0} » chargée ({1} × {2}).", self.nom, w, h))
        } else {
            let t = std::fs::read_to_string(chemin)
                .map_err(|e| trf!("{0} : {1}", chemin.display(), e))?;
            let a = Art::from_text(&t);
            self.photo = None;
            self.art = a.clone();
            self.art_texte = Some(a);
            self.fait_pour = None;
            self.nom = souche(chemin);
            Ok(trf!("Art ASCII « {0} » chargé.", self.nom))
        }
    }

    /// L'art pour les réglages actuels.
    pub fn art(&mut self) -> &Art {
        if let Some(a) = &self.art_texte {
            self.art = a.clone();
        } else if let Some(ph) = &self.photo {
            let p = self.reglages.params();
            if self.fait_pour.as_ref() != Some(&p) {
                self.art = asciiart::convert(&ph.gris, &p);
                self.fait_pour = Some(p);
            }
        }
        &self.art
    }

    fn ecrire(chemin: &Path, contenu: &str) -> Result<String, String> {
        if let Some(d) = chemin.parent() {
            std::fs::create_dir_all(d).map_err(|e| trf!("{0} : {1}", d.display(), e))?;
        }
        std::fs::write(chemin, contenu).map_err(|e| trf!("{0} : {1}", chemin.display(), e))?;
        Ok(trf!("Enregistré : {0}", chemin.display()))
    }

    fn nom_fichier(&self) -> String {
        let n: String = self
            .nom
            .trim()
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '-'
                }
            })
            .collect();
        if n.is_empty() {
            "art-ascii".into()
        } else {
            n
        }
    }

    pub fn dessiner(&mut self, ctx: &egui::Context, m: &mut MachineApple1) {
        if !self.ouvert {
            return;
        }
        let biblio = m.bibliotheque();
        let ph = m.phosphore();
        let mut ouvert = true;
        egui::Window::new("🎨 Photo → ASCII")
            .id(egui::Id::new("apple1-fenetre-ascii"))
            .open(&mut ouvert)
            .resizable(true)
            .default_size([760.0, 720.0])
            .show(ctx, |ui| {
                self.contenu(ui, m, &biblio, ph);
            });
        if !ouvert {
            self.ouvert = false;
        }
    }

    fn contenu(
        &mut self,
        ui: &mut egui::Ui,
        m: &mut MachineApple1,
        biblio: &Path,
        ph: crate::crt::Phosphore,
    ) {
        ui.horizontal_wrapped(|ui| {
            if ui
                .button(tr("📂 Ouvrir une image…"))
                .on_hover_text(tr(
                    "Une photo (jpg, png) ou un dessin .txt du dossier Art ASCII",
                ))
                .clicked()
            {
                self.parcourir = !self.parcourir;
                if self.chemin.is_empty() {
                    self.chemin = format!(
                        "{}{}",
                        biblio.join(crate::art::DOSSIER).display(),
                        std::path::MAIN_SEPARATOR
                    );
                }
            }
            ui.label(tr("Nom :"));
            ui.add(egui::TextEdit::singleline(&mut self.nom).desired_width(160.0))
                .on_hover_text(tr("Titre du programme et nom des fichiers"));
        });
        if self.parcourir {
            let mut r = vec![Raccourci {
                libelle: "Art ASCII".into(),
                chemin: biblio.join(crate::art::DOSSIER),
            }];
            if let Some(h) = std::env::var_os("HOME").map(PathBuf::from) {
                for d in ["Images", "Pictures"] {
                    if h.join(d).is_dir() {
                        r.push(Raccourci {
                            libelle: d.into(),
                            chemin: h.join(d),
                        });
                    }
                }
                r.push(Raccourci {
                    libelle: "🏠".into(),
                    chemin: h,
                });
            }
            if let Some(f) = selecteur::panneau(ui, &mut self.chemin, &r, false) {
                self.parcourir = false;
                self.message = Some(match self.ouvrir_fichier(&f) {
                    Ok(t) => (t, false),
                    Err(e) => (e, true),
                });
            }
        }
        if self.photo.is_none() && self.art_texte.is_none() {
            ui.label(RichText::new(tr("Ouvrez une photo (jpg, png) ou un dessin .txt : il devient 40 × 24 caractères de l'Apple 1.")).weak());
        }
        let art = self.art().clone();
        let avant = self.reglages.clone();
        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                let rg = &mut self.reglages;
                if let Some(photo) = &mut self.photo {
                    let tex = photo.texture.get_or_insert_with(|| ui.ctx().load_texture("apple1-ascii-photo", photo.vignette.clone(), TextureOptions::LINEAR)).clone();
                    let sz = tex.size_vec2();
                    let echelle = (240.0 / sz.y).min(300.0 / sz.x);
                    let (rect, resp) = ui.allocate_exact_size(sz * echelle, egui::Sense::click_and_drag());
                    ui.painter().image(tex.id(), rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), Color32::WHITE);
                    let (x, y, w, h) = asciiart::crop(&photo.gris, &rg.params());
                    let (gw, gh) = (photo.gris.w as f32, photo.gris.h as f32);
                    let r = egui::Rect::from_min_size(rect.min + egui::vec2(x / gw * rect.width(), y / gh * rect.height()), egui::vec2(w / gw * rect.width(), h / gh * rect.height()));
                    ui.painter().rect_stroke(r, 0.0, egui::Stroke::new(2.0, Color32::from_rgb(90, 220, 110)), egui::StrokeKind::Outside);
                    if resp.dragged() {
                        let d = resp.drag_delta();
                        rg.cx = (rg.cx + d.x / rect.width()).clamp(0.0, 1.0);
                        rg.cy = (rg.cy + d.y / rect.height()).clamp(0.0, 1.0);
                    }
                    if resp.hovered() {
                        let s = ui.ctx().input(|i| i.smooth_scroll_delta.y);
                        if s != 0.0 {
                            rg.zoom = (rg.zoom * (1.0 + s / 400.0)).clamp(1.0, 8.0);
                        }
                    }
                    resp.on_hover_text(tr("Glisser : déplacer la partie gardée ; molette : zoom"));
                }
                // aperçu avec la police du 2513
                let perime = self.apercu.as_ref().map(|(_, a, p)| a != &art || *p != ph).unwrap_or(true);
                if perime {
                    let img = image_art(&art, ph);
                    match &mut self.apercu {
                        Some((t, a, p)) => {
                            t.set(img, TextureOptions::NEAREST);
                            *a = art.clone();
                            *p = ph;
                        }
                        None => self.apercu = Some((ui.ctx().load_texture("apple1-ascii-apercu", img, TextureOptions::NEAREST), art.clone(), ph)),
                    }
                }
                if let Some((t, _, _)) = &self.apercu {
                    let w = (ui.available_width() - 8.0).clamp(280.0, 560.0);
                    ui.add(egui::Image::new((t.id(), egui::vec2(w, w * 0.75))));
                }
            });
            ui.label(RichText::new(trf!("{0} × {1} caractères", art.lines.iter().map(|l| l.len()).max().unwrap_or(0), art.rows())).small().weak());
            ui.separator();
            let photo = self.photo.is_some();
            let rg = &mut self.reglages;
            ui.add_enabled_ui(photo, |ui| {
                egui::Grid::new("apple1-ascii-reglages").num_columns(4).spacing([10.0, 4.0]).show(ui, |ui| {
                    ui.label(tr("Colonnes"));
                    ui.add(egui::Slider::new(&mut rg.colonnes, 1..=40));
                    ui.label(tr("Lignes"));
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(&mut rg.lignes, 1..=24));
                        if ui.selectable_label(rg.lignes == 23, "23").on_hover_text(tr("23 lignes : l'invite du programme ne fait pas défiler le dessin")).clicked() {
                            rg.lignes = if rg.lignes == 23 { 24 } else { 23 };
                        }
                    });
                    ui.end_row();
                    ui.label(tr("Zoom"));
                    ui.add(egui::Slider::new(&mut rg.zoom, 1.0..=8.0).logarithmic(true));
                    ui.label(tr("Centre"));
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut rg.cx).range(0.0..=1.0).speed(0.005).prefix(tr("x ")));
                        ui.add(egui::DragValue::new(&mut rg.cy).range(0.0..=1.0).speed(0.005).prefix(tr("y ")));
                    });
                    ui.end_row();
                    ui.label(tr("Luminosité"));
                    ui.add(egui::Slider::new(&mut rg.luminosite, -1.0..=1.0));
                    ui.label(tr("Contraste"));
                    ui.add(egui::Slider::new(&mut rg.contraste, 0.2..=4.0).logarithmic(true));
                    ui.end_row();
                    ui.label(tr("Gamma"));
                    ui.add(egui::Slider::new(&mut rg.gamma, 0.2..=5.0).logarithmic(true));
                    ui.label("");
                    ui.horizontal_wrapped(|ui| {
                        ui.checkbox(&mut rg.inverser, tr("Inverser"));
                        ui.checkbox(&mut rg.niveaux_auto, tr("Niveaux automatiques"));
                        ui.checkbox(&mut rg.tramage, tr("Tramage"));
                    });
                    ui.end_row();
                    ui.label(tr("Caractères"));
                    let noms = [tr("Complète"), tr("Simple"), tr("Apple1-Slideshow"), tr("Personnalisée")];
                    egui::ComboBox::from_id_salt("apple1-ascii-rampe").selected_text(noms[rg.rampe.min(3)]).show_ui(ui, |ui| {
                        for (i, n) in noms.iter().enumerate() {
                            ui.selectable_value(&mut rg.rampe, i, *n);
                        }
                    });
                    ui.label("");
                    if rg.rampe == 3 {
                        ui.add(egui::TextEdit::singleline(&mut rg.rampe_perso).desired_width(200.0).font(egui::TextStyle::Monospace));
                    } else {
                        ui.label(RichText::new(String::from_utf8_lossy(&asciiart::clean_ramp(&rg.rampe_texte())).into_owned()).monospace().weak());
                    }
                    ui.end_row();
                });
                if ui.button(tr("↺ Réglages par défaut")).clicked() {
                    *rg = Reglages { asm: rg.asm, rampe_perso: rg.rampe_perso.clone(), ..Reglages::default() };
                }
            });
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                ui.label(tr("Programme :"));
                ui.selectable_value(&mut self.reglages.asm, false, tr("Integer BASIC"));
                ui.selectable_value(&mut self.reglages.asm, true, tr("Assembleur 6502 (300R)"));
            });
            let a = !art.lines.is_empty();
            let asm = self.reglages.asm;
            let nom = self.nom.clone();
            let fichier = self.nom_fichier();
            ui.add_enabled_ui(a, |ui| {
                ui.horizontal_wrapped(|ui| {
                    if ui.button(RichText::new(tr("▶ Lancer")).strong()).on_hover_text(tr("Taper le programme dans l'Apple 1 et le lancer")).clicked() {
                        let l = if asm { Langage::Assembleur } else { Langage::Basic };
                        self.message = Some(match m.lancer_source(l, &code(&art, &nom, asm)) {
                            Ok(t) => (t, false),
                            Err(e) => (e.to_string(), true),
                        });
                    }
                    if ui.button(tr("💾 Programme")).on_hover_text(tr("Enregistrer le programme dans BASIC/ ou Assembleur/ de la bibliothèque")).clicked() {
                        let (d, ext) = if asm { ("Assembleur", "s") } else { ("BASIC", "bas") };
                        let c = biblio.join(d).join(format!("{fichier}.{ext}"));
                        self.message = Some(Self::ecrire(&c, &code(&art, &nom, asm)).map(|t| (t, false)).unwrap_or_else(|e| (e, true)));
                    }
                    if ui.button(tr("📼 Cassette")).on_hover_text(tr("Écrire le programme en cassette ACI (.wav) dans Cassettes/")).clicked() {
                        let c = biblio.join("Cassettes").join(format!("{fichier}.wav"));
                        let r = programme(&art, &nom, asm).and_then(|p| {
                            if let Some(d) = c.parent() {
                                std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
                            }
                            program::save_file(&c, &p, &Default::default())?;
                            Ok(trf!("cassette écrite : {0} (pour la relire : C100R puis {1})", c.display(), p.aci_read_command()))
                        });
                        self.message = Some(r.map(|t| (t, false)).unwrap_or_else(|e| (e, true)));
                    }
                    if ui.button(tr("💾 Art .txt")).on_hover_text(tr("Enregistrer le dessin dans le dossier Art ASCII de la bibliothèque")).clicked() {
                        let c = biblio.join(crate::art::DOSSIER).join(format!("{fichier}.txt"));
                        self.message = Some(Self::ecrire(&c, &art.text()).map(|t| (t, false)).unwrap_or_else(|e| (e, true)));
                    }
                    if ui.button(tr("📋 Copier")).clicked() {
                        ui.ctx().copy_text(art.text());
                    }
                    ui.toggle_value(&mut self.voir_code, tr("Voir le code"));
                });
            });
            if let Some((t, err)) = &self.message {
                let c = if *err { ui.visuals().warn_fg_color } else { ui.visuals().weak_text_color() };
                ui.colored_label(c, t);
            }
            if self.voir_code && a {
                let mut c = code(&art, &nom, asm);
                ui.add(egui::TextEdit::multiline(&mut c).font(egui::TextStyle::Monospace).desired_width(f32::INFINITY).desired_rows(12));
            }
        });
        if self.reglages != avant {
            self.fait_pour = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemples_en_programmes_et_cassette() {
        for (nom, texte) in crate::fenetres::EXEMPLES_ART
            .iter()
            .filter(|e| e.0.ends_with(".txt"))
        {
            let a = Art::from_text(texte);
            programme(&a, nom, true).unwrap();
            let p = programme(&a, nom, false).unwrap();
            assert_eq!(p.blocks.len(), 2, "{nom}");
        }
        let img = image_art(&Art::from_text("@"), crate::crt::Phosphore::Ambre);
        assert_eq!(img.size, [280, 192]);
    }

    #[test]
    fn photo_png() {
        let d = std::env::temp_dir().join(format!("a1-ascii-{}", std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        let p = d.join("g.png");
        image::GrayImage::from_fn(80, 48, |x, _| image::Luma([(x * 3) as u8]))
            .save(&p)
            .unwrap();
        let mut o = OutilAscii::default();
        o.ouvrir_fichier(&p).unwrap();
        assert_eq!(o.art().rows(), 24);
        std::fs::remove_dir_all(&d).ok();
    }
}
