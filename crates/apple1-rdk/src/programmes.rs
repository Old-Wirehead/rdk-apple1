//! Réglages par programme (v1.0 : bouton ⚙ de la bibliothèque et fichier
//! `programs.json` du dossier du programme, même format, donc les réglages
//! de la v1.0 sont relus tels quels).
//!
//! L'interface commune ne dit pas à la machine quel fichier est choisi dans
//! sa bibliothèque : la fenêtre « ⚙ Réglages du programme » (bouton du
//! groupe Périphériques) s'ouvre sur le **dernier programme chargé ou
//! lancé** depuis la bibliothèque, et un sélecteur permet d'en choisir un
//! autre. Les réglages enregistrés sont appliqués par Charger, ▶ Play et le
//! double-clic : plages, adresse de chargement d'un `.bin`, adresse et
//! commande de lancement, BASIC nécessaire.

use crate::i18n::{t, tf, tr};
use crate::trf;
use crate::v1::params::{self, Plan, ProgKind, ProgParams, Store};
use apple1_core::aciwav;
use apple1_core::program::{self, Kind, Program};
use eframe::egui::{self, Color32, RichText};
use rdk_interface::outils::selecteur::{self, Raccourci};
use std::path::{Path, PathBuf};

/// Réglages enregistrés pour ce fichier (`programs.json` de son dossier).
pub fn enregistres(chemin: &Path) -> Option<ProgParams> {
    let dossier = chemin.parent()?;
    let nom = chemin.file_name()?.to_string_lossy().into_owned();
    Store::load(dossier).0.map.get(&nom).cloned()
}

/// Réglages d'un fichier : enregistrés, sinon lus dans le fichier (v1.0 :
/// `params_of`).
pub fn reglages_de(chemin: &Path) -> ProgParams {
    enregistres(chemin).unwrap_or_else(|| params::from_file(chemin))
}

/// Enregistre les réglages dans `programs.json` du dossier du fichier.
pub fn enregistrer(chemin: &Path, p: ProgParams) -> Result<(), String> {
    let dossier = chemin
        .parent()
        .ok_or_else(|| tr("chemin sans dossier").to_string())?;
    let nom = chemin
        .file_name()
        .ok_or_else(|| tr("chemin sans nom de fichier").to_string())?
        .to_string_lossy()
        .into_owned();
    let (mut st, _) = Store::load(dossier);
    st.map.insert(nom, p);
    st.save(dossier)
}

/// Vérifie les réglages d'un fichier de `taille` octets.
pub fn plan(chemin: &Path, p: &ProgParams, taille: usize) -> Result<Plan, String> {
    let bin = program::kind_of(chemin, None) == Kind::Bin;
    p.validate(bin, taille)
        .map_err(|e| tf("e_params", &[&nom(chemin), &e]))
}

fn nom(chemin: &Path) -> String {
    chemin
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Programme d'un fichier avec ses réglages (v1.0 : `launch::build_program`,
/// ici sur les octets déjà lus par la bibliothèque).
pub fn construire(chemin: &Path, octets: &[u8], plan: &Plan) -> Result<Program, String> {
    let premier = plan.load_addr.or(plan.ranges.first().map(|r| r.0));
    let genre = program::kind_of(chemin, Some(octets));
    let mut p = match genre {
        Kind::Bin => program::load_bytes(octets, Kind::Bin, premier)?,
        Kind::Wav => {
            let imp =
                aciwav::import_wav_bytes(octets).map_err(|e| trf!("{0} : {1}", nom(chemin), e))?;
            let n = imp.segments.len();
            let r = &plan.ranges;
            if n > 0 && r.len() == n {
                program::wav_with_ranges(&imp, r)?
            } else if !imp.embedded.is_empty() || n == 0 {
                program::wav_to_program(&imp, None)?
            } else if r.len() == 1 {
                program::wav_to_program(&imp, premier)?
            } else if r.is_empty() {
                match program::guess_layout(&imp) {
                    Some((g, _)) => program::wav_with_ranges(&imp, &g)?,
                    None => program::wav_to_program(&imp, None)?,
                }
            } else {
                return Err(tf("e_ranges_records", &[&r.len(), &n]));
            }
        }
        autre => program::load_bytes(octets, autre, None)?,
    };
    if plan.run_addr.is_some() {
        p.run = plan.run_addr;
    }
    if plan.kind.is_basic() && !p.notes.iter().any(|n| n.contains("BASIC")) {
        p.notes.push("BASIC".into());
    }
    Ok(p)
}

/// Fenêtre « ⚙ Réglages du programme ».
#[derive(Default)]
pub struct FenetreReglages {
    pub ouvert: bool,
    /// Chemin saisi (ou choisi dans le sélecteur).
    chemin: String,
    /// Fichier dont les réglages sont affichés, et ces réglages.
    fichier: Option<PathBuf>,
    p: ProgParams,
    taille: usize,
    erreur: Option<String>,
    message: Option<String>,
}

impl FenetreReglages {
    /// Ouvre la fenêtre sur `chemin` (dernier programme de la bibliothèque).
    pub fn ouvrir(&mut self, chemin: Option<&Path>) {
        self.ouvert = true;
        if let Some(c) = chemin {
            self.chemin = c.display().to_string();
            self.choisir(c.to_path_buf());
        }
    }

    fn choisir(&mut self, c: PathBuf) {
        self.taille = std::fs::metadata(&c).map(|m| m.len() as usize).unwrap_or(0);
        self.p = reglages_de(&c);
        self.fichier = Some(c);
        self.erreur = None;
        self.message = None;
    }

    /// `biblio` : dossier de la bibliothèque de la machine (raccourcis).
    /// Renvoie le fichier dont les réglages viennent d'être enregistrés.
    pub fn dessiner(&mut self, ctx: &egui::Context, biblio: &Path) -> Option<PathBuf> {
        if !self.ouvert {
            return None;
        }
        let mut ouvert = true;
        let mut enregistre = None;
        egui::Window::new(tr("⚙ Réglages du programme")).id(egui::Id::new("apple1-reglages-programme")).open(&mut ouvert).default_width(520.0).show(ctx, |ui| {
            ui.label(RichText::new(tr("Choisissez un programme de la bibliothèque (le dernier chargé est proposé). Les réglages sont rangés dans programs.json, dans son dossier, comme en v1.0.")).small().weak());
            let raccourcis = vec![
                Raccourci { libelle: format!("🖥 {}", biblio.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default()), chemin: biblio.to_path_buf() },
                Raccourci { libelle: tr("🎮 Jeux").into(), chemin: biblio.join("Jeux") },
                Raccourci { libelle: tr("📼 Cassettes").into(), chemin: biblio.join("Cassettes") },
                Raccourci { libelle: "🐙 GitHub".into(), chemin: biblio.join("GitHub") },
            ];
            egui::CollapsingHeader::new(tr("📂 Choisir un fichier")).default_open(self.fichier.is_none()).show(ui, |ui| {
                if let Some(f) = selecteur::panneau(ui, &mut self.chemin, &raccourcis, false) {
                    self.choisir(f);
                }
            });
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.chemin).desired_width(400.0));
                let c = PathBuf::from(self.chemin.trim());
                if ui.add_enabled(c.is_file(), egui::Button::new(tr("Afficher"))).clicked() {
                    self.choisir(c);
                }
            });
            let Some(fichier) = self.fichier.clone() else {
                ui.weak(tr("Aucun programme choisi."));
                return;
            };
            ui.separator();
            let nom = nom(&fichier);
            ui.heading(tf("p_title", &[&nom]));
            ui.label(RichText::new(tf("p_saved_in", &[&params::FILE])).small().weak());
            let bin = program::kind_of(&fichier, None) == Kind::Bin;
            let p = &mut self.p;
            egui::Grid::new("apple1-parametres").num_columns(2).spacing([10.0, 6.0]).show(ui, |ui| {
                ui.label(t("p_ranges"));
                ui.add(egui::TextEdit::singleline(&mut p.ranges).desired_width(360.0).hint_text("300.31E   4A.00FF 800.FFF")).on_hover_text(t("p_ranges_tip"));
                ui.end_row();
                ui.label(t("p_load"));
                ui.add_enabled(bin, egui::TextEdit::singleline(&mut p.load_addr).desired_width(80.0).hint_text("300")).on_hover_text(t("p_load_tip"));
                ui.end_row();
                ui.label(t("p_kind"));
                ui.horizontal(|ui| {
                    let avant = p.kind;
                    ui.radio_value(&mut p.kind, ProgKind::Machine, t("p_machine"));
                    ui.radio_value(&mut p.kind, ProgKind::Basic, t("p_basic"));
                    ui.radio_value(&mut p.kind, ProgKind::BasicInteger, t("p_basic_int"));
                    if avant != p.kind {
                        if p.kind.is_basic() {
                            p.needs_basic = true;
                            p.run_cmd = "E2B3R\\nRUN".into();
                        } else {
                            p.run_cmd = if p.run_addr.trim().is_empty() { String::new() } else { format!("{}R", p.run_addr.trim()) };
                        }
                    }
                });
                ui.end_row();
                ui.label(t("p_run"));
                let avant = p.run_addr.clone();
                ui.add_enabled(p.kind == ProgKind::Machine, egui::TextEdit::singleline(&mut p.run_addr).desired_width(80.0).hint_text("300"));
                if avant != p.run_addr && (p.run_cmd.trim().is_empty() || p.run_cmd == format!("{}R", avant.trim())) {
                    p.run_cmd = if p.run_addr.trim().is_empty() { String::new() } else { format!("{}R", p.run_addr.trim()) };
                }
                ui.end_row();
                ui.label(t("p_cmd"));
                ui.add(egui::TextEdit::singleline(&mut p.run_cmd).desired_width(360.0).hint_text("300R   E2B3R\\nRUN")).on_hover_text(t("p_cmd_tip"));
                ui.end_row();
                ui.label("");
                ui.add_enabled(p.kind == ProgKind::Machine, egui::Checkbox::new(&mut p.needs_basic, t("p_needs_basic")));
                ui.end_row();
                ui.label(t("p_notes"));
                ui.add(egui::TextEdit::multiline(&mut p.notes).desired_rows(3).desired_width(360.0));
                ui.end_row();
            });
            let apercu = plan(&fichier, &self.p, self.taille);
            match (&apercu, &self.erreur) {
                (_, Some(e)) => {
                    ui.colored_label(Color32::LIGHT_RED, e.as_str());
                }
                (Ok(pl), None) => {
                    ui.label(RichText::new(tf("p_preview", &[&pl.run_keys.replace('\r', " ⏎ ")])).small().weak());
                }
                (Err(e), None) => {
                    ui.colored_label(Color32::from_rgb(230, 170, 60), e.as_str());
                }
            }
            if !self.p.auto.is_empty() {
                ui.label(RichText::new(tf("p_auto_info", &[&self.p.auto])).small().color(Color32::from_rgb(90, 200, 120)));
            }
            if let Some(m) = &self.message {
                ui.label(RichText::new(m.as_str()).small());
            }
            ui.horizontal(|ui| {
                if ui.button(t("b_save")).clicked() {
                    match plan(&fichier, &self.p, self.taille) {
                        Ok(_) => {
                            // modifié à la main : la table des cassettes connues ne le remplace plus
                            if !self.p.auto.is_empty() && self.p != reglages_de(&fichier) {
                                self.p.auto.clear();
                            }
                            match enregistrer(&fichier, self.p.clone()) {
                                Ok(()) => {
                                    self.message = Some(tf("p_saved", &[&nom]));
                                    enregistre = Some(fichier.clone());
                                }
                                Err(e) => self.erreur = Some(e),
                            }
                        }
                        Err(e) => self.erreur = Some(e),
                    }
                } else if self.erreur.is_some() && apercu.is_ok() {
                    self.erreur = None;
                }
                if ui.button(t("p_from_file")).on_hover_text(t("p_from_file_tip")).clicked() {
                    let notes = std::mem::take(&mut self.p.notes);
                    self.p = params::from_file(&fichier);
                    self.p.notes = notes;
                    self.erreur = None;
                }
                if enregistres(&fichier).is_some() && ui.button(tr("🗑 Oublier les réglages")).on_hover_text(tr("Retire ce programme de programs.json : les réglages sont de nouveau lus dans le fichier.")).clicked() {
                    match oublier(&fichier) {
                        Ok(()) => {
                            self.p = params::from_file(&fichier);
                            self.message = Some(trf!("Réglages de « {0} » oubliés", nom));
                            enregistre = Some(fichier.clone());
                        }
                        Err(e) => self.erreur = Some(e),
                    }
                }
            });
        });
        self.ouvert = ouvert;
        enregistre
    }
}

/// Retire un fichier de `programs.json`.
pub fn oublier(chemin: &Path) -> Result<(), String> {
    let dossier = chemin
        .parent()
        .ok_or_else(|| tr("chemin sans dossier").to_string())?;
    let nom = nom(chemin);
    let (mut st, _) = Store::load(dossier);
    st.map.remove(&nom);
    st.save(dossier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reglages_enregistres_et_appliques() {
        rdk_pilotes::langue::definir(rdk_pilotes::Langue::Fr);
        let d = std::env::temp_dir().join(format!("a1-programmes-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        let f = d.join("jeu.bin");
        std::fs::write(&f, [0xA9, 0x41, 0x20, 0xEF, 0xFF, 0x4C, 0x00, 0x08]).unwrap();
        assert!(enregistres(&f).is_none());
        // lu dans le fichier : .bin en $0300
        assert_eq!(reglages_de(&f).load_addr, "300");
        let p = ProgParams {
            load_addr: "800".into(),
            run_addr: "800".into(),
            run_cmd: "800R".into(),
            ..reglages_de(&f)
        };
        let p = ProgParams {
            ranges: "800.807".into(),
            ..p
        };
        enregistrer(&f, p.clone()).unwrap();
        assert!(d.join(params::FILE).is_file());
        assert_eq!(enregistres(&f), Some(p.clone()));
        let pl = plan(&f, &p, 8).unwrap();
        assert_eq!(pl.run_keys, "800R");
        let prog = construire(&f, &std::fs::read(&f).unwrap(), &pl).unwrap();
        assert_eq!(prog.blocks[0].0, 0x0800);
        assert_eq!(prog.run, Some(0x0800));
        oublier(&f).unwrap();
        assert!(enregistres(&f).is_none());
        let _ = std::fs::remove_dir_all(&d);
    }
}
