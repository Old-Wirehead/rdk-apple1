//! Programmateur d'EPROM de la v1.0 (`apple1-gui/src/eprom_ui.rs`) : lire,
//! écrire ou vérifier une puce avec un programmateur `minipro` (XGecu
//! TL866II+, T48, T56), par `apple1_core::eprom` (inchangé).
//!
//! 2.0 : les fichiers de PROM sont en **Intel HEX** (`.hex`, format commun
//! de l'interface commune 0.1.5, celui des outils de programmation) ; le
//! dossier par défaut est `PROM/` de la bibliothèque. Les images brutes
//! (`.bin`, `.rom`), les dumps (`.dump`, `.woz`) et les cassettes restent lus.
//! Le programmateur tourne dans un fil à part ; ses messages remplissent le
//! journal de la fenêtre.

use crate::i18n::tr;
use crate::trf;
use crate::MachineApple1;
use apple1_core::eprom::{self, Chip, CHIPS};
use eframe::egui::{self, Color32, RichText};
use rdk_interface::outils::selecteur::{self, Raccourci};
use rdk_pilotes::{formats, BlocMemoire};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};

/// Dossier des images de PROM dans la bibliothèque.
pub const DOSSIER: &str = "PROM";

/// Écrit `octets` (à l'adresse `base`) dans `chemin` : Intel HEX pour `.hex`
/// (et sans extension), dump RDK pour `.dump`, image brute sinon.
pub fn ecrire_fichier(chemin: &Path, base: u32, octets: &[u8]) -> Result<(), String> {
    if let Some(d) = chemin.parent() {
        std::fs::create_dir_all(d).map_err(|e| trf!("{0} : {1}", d.display(), e))?;
    }
    let ext = chemin
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("hex")
        .to_ascii_lowercase();
    let blocs = [BlocMemoire {
        adresse: base,
        octets: octets.to_vec(),
    }];
    let contenu = match ext.as_str() {
        "hex" => formats::ecrire_intel_hex(&blocs).into_bytes(),
        "dump" => formats::ecrire_dump(&blocs).into_bytes(),
        _ => octets.to_vec(),
    };
    std::fs::write(chemin, contenu).map_err(|e| trf!("{0} : {1}", chemin.display(), e))
}

/// Les octets d'un fichier tels qu'écrits sur la puce : image brute (`.bin`,
/// `.rom`), ou programme aplati de sa première à sa dernière adresse
/// (`$FF` dans les trous) : Intel HEX, `.dump`, `.woz`, cassette…
pub fn image_fichier(chemin: &Path) -> Result<Vec<u8>, String> {
    let ext = chemin
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    if ext == "bin" || ext == "rom" {
        return std::fs::read(chemin).map_err(|e| trf!("{0} : {1}", chemin.display(), e));
    }
    let blocs: Vec<(usize, Vec<u8>)> = if ext == formats::EXT_HEX || ext == formats::EXT_DUMP {
        let octets = std::fs::read(chemin).map_err(|e| trf!("{0} : {1}", chemin.display(), e))?;
        match formats::lire_format_commun(&ext, &octets) {
            Some(Ok(b)) => b
                .into_iter()
                .map(|b| (b.adresse as usize, b.octets))
                .collect(),
            Some(Err(e)) => return Err(trf!("{0} : {1}", chemin.display(), e)),
            None => Vec::new(),
        }
    } else {
        apple1_core::program::load_file(chemin, None)?
            .blocks
            .into_iter()
            .map(|(a, o)| (a as usize, o))
            .collect()
    };
    let bas = blocs.iter().map(|b| b.0).min().ok_or(tr("fichier vide"))?;
    let haut = blocs.iter().map(|b| b.0 + b.1.len()).max().unwrap_or(bas);
    let mut img = vec![0xFFu8; haut - bas];
    for (a, d) in &blocs {
        let o = a - bas;
        img[o..o + d.len()].copy_from_slice(d);
    }
    Ok(img)
}

enum Msg {
    Ligne(String),
    Fini,
}

#[derive(Clone)]
enum Travail {
    Lire { enregistrer: Option<PathBuf> },
    Ecrire,
    Verifier,
}

/// Fenêtre « Programmateur d'EPROM ».
pub struct FenetreProm {
    pub ouvert: bool,
    prog: usize,
    puce: usize,
    detection: Option<Result<String, String>>,
    depuis_memoire: bool,
    de: String,
    a: String,
    pub fichier: String,
    parcourir: bool,
    rx: Option<Receiver<Msg>>,
    pub journal: Vec<String>,
}

impl Default for FenetreProm {
    fn default() -> Self {
        let puce = CHIPS
            .iter()
            .position(|c| c.name.starts_with("28C256"))
            .unwrap_or(0);
        FenetreProm {
            ouvert: false,
            prog: 0,
            puce,
            detection: None,
            depuis_memoire: true,
            de: "E000".into(),
            a: "EFFF".into(),
            fichier: String::new(),
            parcourir: false,
            rx: None,
            journal: Vec::new(),
        }
    }
}

fn libelle_puce(c: &Chip) -> String {
    trf!(
        "{0} ({1} Ko)",
        c.name
            .replace("mask ROM, read only", "ROM à masque, lecture seule"),
        c.size / 1024
    )
}

impl FenetreProm {
    pub fn ouvrir(&mut self) {
        self.ouvert = true;
        self.detection = None;
    }

    fn occupe(&self) -> bool {
        self.rx.is_some()
    }

    fn plage(&self) -> Result<(u16, u16), String> {
        let err = || trf!("Plage de mémoire invalide : {0}–{1}", self.de, self.a);
        let a = apple1_core::program::parse_addr(&self.de).map_err(|_| err())?;
        let b = apple1_core::program::parse_addr(&self.a).map_err(|_| err())?;
        if b < a {
            return Err(err());
        }
        Ok((a, b))
    }

    fn donnees(&self, m: &MachineApple1) -> Result<Vec<u8>, String> {
        if self.depuis_memoire {
            let (a, b) = self.plage()?;
            Ok((a..=b).map(|x| m.lire(x)).collect())
        } else {
            if self.fichier.trim().is_empty() {
                return Err(tr("Choisissez un fichier.").into());
            }
            image_fichier(Path::new(self.fichier.trim()))
        }
    }

    fn base(&self) -> usize {
        if self.depuis_memoire {
            self.plage().map(|p| p.0 as usize).unwrap_or(0)
        } else {
            0
        }
    }

    /// Lecture : avec la mémoire ou un fichier existant, comparer ; avec un
    /// nouveau nom de fichier, enregistrer (Intel HEX pour `.hex`).
    fn entrees_lecture(
        &self,
        m: &MachineApple1,
    ) -> Result<(Option<Vec<u8>>, Option<PathBuf>), String> {
        if self.depuis_memoire {
            return Ok((Some(self.donnees(m)?), None));
        }
        if self.fichier.trim().is_empty() {
            return Err(tr("Choisissez un fichier.").into());
        }
        let p = PathBuf::from(self.fichier.trim());
        if p.exists() {
            Ok((Some(image_fichier(&p)?), None))
        } else {
            Ok((None, Some(p)))
        }
    }

    fn lancer(&mut self, ctx: &egui::Context, travail: Travail, donnees: Option<Vec<u8>>) {
        let base = self.base();
        let (tx, rx) = channel();
        self.rx = Some(rx);
        self.journal.clear();
        let (pi, ci) = (self.prog, self.puce);
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let dire = |s: String| {
                let _ = tx.send(Msg::Ligne(s));
                ctx.request_repaint();
            };
            let progs = eprom::all();
            let puce = &CHIPS[ci];
            let Some(p) = progs.get(pi) else { return };
            let faire = || -> Result<(), String> {
                match &travail {
                    Travail::Lire { enregistrer } => {
                        dire(trf!("{0} : lecture de la puce…", puce.name));
                        let lu = p.read(puce)?;
                        dire(trf!(
                            "{0} : lecture de {1} octets terminée.",
                            puce.name,
                            lu.len()
                        ));
                        if let Some(f) = enregistrer {
                            ecrire_fichier(f, 0, &lu)?;
                            dire(trf!("Contenu enregistré dans {0}", f.display()));
                        }
                        if let Some(d) = donnees {
                            let n = d.len().min(lu.len());
                            let v = eprom::compare(&d[..n], &lu[..n]);
                            dire(trf!("Comparaison avec les données choisies : {0} octets, {1} différence(s).", v.compared, v.different));
                            for (i, a, b) in v.first.iter().take(8) {
                                dire(trf!(
                                    "  ${0} : puce ${1}, données ${2}",
                                    format!("{:04X}", base + i),
                                    format!("{:02X}", b),
                                    format!("{:02X}", a)
                                ));
                            }
                        }
                    }
                    Travail::Ecrire => {
                        let d = donnees.unwrap_or_default();
                        dire(trf!(
                            "Écriture de la puce {0} ({1} octets)…",
                            puce.name,
                            d.len()
                        ));
                        let r = p.write(puce, &d)?;
                        dire(trf!("Écriture terminée : {0}", r));
                        dire(trf!("Vérification de {0}…", puce.name));
                        let mut tout = d;
                        tout.resize(puce.size, 0xFF);
                        let v = p.verify(puce, &tout)?;
                        dire(if v.ok() {
                            trf!("Vérification réussie : {0} octets identiques.", v.compared)
                        } else {
                            trf!(
                                "Vérification ÉCHOUÉE : {0} différence(s) sur {1} octets.",
                                v.different,
                                v.compared
                            )
                        });
                    }
                    Travail::Verifier => {
                        dire(trf!("Vérification de {0}…", puce.name));
                        let mut tout = donnees.unwrap_or_default();
                        tout.resize(puce.size, 0xFF);
                        let v = p.verify(puce, &tout)?;
                        dire(if v.ok() {
                            trf!("Vérification réussie : {0} octets identiques.", v.compared)
                        } else {
                            trf!(
                                "Vérification ÉCHOUÉE : {0} différence(s) sur {1} octets.",
                                v.different,
                                v.compared
                            )
                        });
                        for (i, a, b) in v.first.iter().take(8) {
                            dire(trf!(
                                "  ${0} : puce ${1}, données ${2}",
                                format!("{:04X}", base + i),
                                format!("{:02X}", b),
                                format!("{:02X}", a)
                            ));
                        }
                    }
                }
                Ok(())
            };
            if let Err(e) = faire() {
                dire(trf!("Erreur : {0}", e));
            }
            let _ = tx.send(Msg::Fini);
            ctx.request_repaint();
        });
    }

    /// Dessine la fenêtre (si elle est ouverte).
    pub fn dessiner(&mut self, ctx: &egui::Context, m: &mut MachineApple1) {
        if !self.ouvert {
            return;
        }
        let mut fini = false;
        if let Some(rx) = &self.rx {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    Msg::Ligne(l) => self.journal.push(l),
                    Msg::Fini => fini = true,
                }
            }
        }
        if fini {
            self.rx = None;
        }
        let progs = eprom::all();
        if self.detection.is_none() {
            self.detection = progs.get(self.prog).map(|p| p.detect());
        }
        let pret = matches!(self.detection, Some(Ok(_)));
        let occupe = self.occupe();
        let biblio = m.bibliotheque();
        let mut ouvert = self.ouvert;
        egui::Window::new("Programmateur d'EPROM").id(egui::Id::new("apple1-fenetre-eprom")).open(&mut ouvert).collapsible(false).resizable(true).default_width(640.0).show(ctx, |ui| {
            egui::Grid::new("apple1-eprom-grille").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                ui.label(tr("Programmateur :"));
                ui.vertical(|ui| {
                    let nom = progs.get(self.prog).map(|p| p.name()).unwrap_or("?");
                    egui::ComboBox::from_id_salt("apple1-eprom-prog").selected_text(nom).width(330.0).show_ui(ui, |ui| {
                        for (i, p) in progs.iter().enumerate() {
                            if ui.selectable_value(&mut self.prog, i, p.name()).clicked() {
                                self.detection = None;
                            }
                        }
                    });
                    match &self.detection {
                        Some(Ok(d)) => {
                            ui.colored_label(Color32::from_rgb(90, 210, 110), trf!("✔ {0}", d));
                        }
                        _ => {
                            ui.colored_label(Color32::from_rgb(235, 90, 90), tr("× « minipro » est introuvable"));
                            ui.label(RichText::new(tr("minipro pilote les programmateurs XGecu TL866II+, T48 et T56. Sous Debian / Ubuntu / Mint : « sudo apt install minipro », puis branchez le programmateur et appuyez sur « Détecter ».")).small().weak());
                            if ui.button(tr("Détecter")).clicked() {
                                self.detection = None;
                            }
                        }
                    }
                });
                ui.end_row();

                ui.label(tr("Puce :"));
                egui::ComboBox::from_id_salt("apple1-eprom-puce").selected_text(libelle_puce(&CHIPS[self.puce])).show_ui(ui, |ui| {
                    for (i, c) in CHIPS.iter().enumerate() {
                        ui.selectable_value(&mut self.puce, i, libelle_puce(c));
                    }
                });
                ui.end_row();

                ui.label(tr("Données :"));
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut self.depuis_memoire, true, tr("Mémoire de l'Apple 1"));
                        ui.add_enabled_ui(self.depuis_memoire, |ui| {
                            ui.label("$");
                            ui.add(egui::TextEdit::singleline(&mut self.de).desired_width(48.0).font(egui::TextStyle::Monospace));
                            ui.label("–  $");
                            ui.add(egui::TextEdit::singleline(&mut self.a).desired_width(48.0).font(egui::TextStyle::Monospace));
                        });
                    });
                    ui.radio_value(&mut self.depuis_memoire, false, tr("Fichier (Intel .hex, .bin, .dump, .woz, cassette…)"));
                    ui.horizontal(|ui| {
                        ui.add_enabled_ui(!self.depuis_memoire, |ui| {
                            if self.fichier.is_empty() {
                                self.fichier = format!("{}{}", biblio.join(DOSSIER).display(), std::path::MAIN_SEPARATOR);
                            }
                            ui.add(egui::TextEdit::singleline(&mut self.fichier).desired_width(380.0));
                            if ui.button("…").on_hover_text(tr("Choisir le fichier (en lecture : fichier où enregistrer le contenu de la puce, Intel HEX si .hex)")).clicked() {
                                self.parcourir = !self.parcourir;
                            }
                        });
                    });
                    if self.parcourir && !self.depuis_memoire {
                        let r = raccourcis(&biblio);
                        if let Some(f) = selecteur::panneau(ui, &mut self.fichier, &r, true) {
                            self.fichier = f.display().to_string();
                            self.parcourir = false;
                        }
                    }
                });
                ui.end_row();
            });
            ui.add_space(6.0);
            let lecture_seule = CHIPS[self.puce].name.contains("read only");
            ui.horizontal(|ui| {
                ui.add_enabled_ui(pret && !occupe, |ui| {
                    if ui.button(tr("Lire la puce")).on_hover_text(tr("Lire le contenu de la puce. Avec la mémoire ou un fichier existant : le comparer. Avec un nouveau nom de fichier : l'enregistrer (Intel HEX pour .hex)")).clicked() {
                        match self.entrees_lecture(m) {
                            Ok((d, f)) => self.lancer(ctx, Travail::Lire { enregistrer: f }, d),
                            Err(e) => self.journal.push(trf!("Erreur : {0}", e)),
                        }
                    }
                    // rdk-apple1#33 : bleu seulement quand le bouton agit vraiment
                    // (programmateur prêt, puce inscriptible) ; sinon grisé
                    // comme « Lire » et « Vérifier », avec la raison au survol
                    let ecrire_actif = pret && !occupe && !lecture_seule;
                    let mut bouton = egui::Button::new(RichText::new(tr("Écrire la puce")).strong());
                    if ecrire_actif {
                        bouton = bouton.fill(Color32::from_rgb(40, 100, 190));
                    }
                    let w = ui
                        .add_enabled(!lecture_seule, bouton)
                        .on_hover_text(tr("Programmer la puce avec les données choisies (la puce doit être vide ou effaçable)"))
                        .on_disabled_hover_text(if lecture_seule { tr("Cette puce est une mémoire morte à masque : lecture seule.") } else if !pret { tr("Aucun programmateur prêt : installez « minipro », branchez le programmateur et appuyez sur « Détecter ».") } else { tr("Un travail est en cours sur la puce.") });
                    if w.clicked() {
                        match self.donnees(m) {
                            Ok(d) => self.lancer(ctx, Travail::Ecrire, Some(d)),
                            Err(e) => self.journal.push(trf!("Erreur : {0}", e)),
                        }
                    }
                    if ui.button(tr("Vérifier")).on_hover_text(tr("Comparer la puce avec les données choisies")).clicked() {
                        match self.donnees(m) {
                            Ok(d) => self.lancer(ctx, Travail::Verifier, Some(d)),
                            Err(e) => self.journal.push(trf!("Erreur : {0}", e)),
                        }
                    }
                });
                if ui.add_enabled(self.depuis_memoire, egui::Button::new(tr("💾 Image Intel HEX"))).on_hover_text(tr("Écrire la plage de la mémoire en Intel HEX dans le dossier PROM de la bibliothèque (sans programmateur)")).clicked() {
                    match self.plage().and_then(|(a, b)| m.exporter_prom(a, b)) {
                        Ok(t) => self.journal.push(t),
                        Err(e) => self.journal.push(trf!("Erreur : {0}", e)),
                    }
                }
                if ui.add_enabled(!occupe, egui::Button::new(tr("Fermer"))).clicked() {
                    self.ouvert = false;
                }
            });
            ui.add_space(4.0);
            let frac = if occupe { ((ctx.input(|i| i.time) * 0.8) % 1.0) as f32 } else { 0.0 };
            ui.add(egui::ProgressBar::new(frac).desired_width(ui.available_width()).text(if occupe { tr("En cours…") } else { tr("Prêt") }));
            if occupe {
                ctx.request_repaint();
            }
            egui::Frame::new().fill(ui.visuals().extreme_bg_color).inner_margin(6.0).corner_radius(3.0).show(ui, |ui| {
                egui::ScrollArea::vertical().max_height(150.0).min_scrolled_height(110.0).stick_to_bottom(true).auto_shrink([false, false]).show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    for l in &self.journal {
                        ui.label(RichText::new(l).monospace().size(12.0));
                    }
                    if lecture_seule {
                        ui.label(RichText::new(tr("Cette puce est une mémoire morte à masque : lecture seule.")).small().weak());
                    }
                });
            });
        });
        if !ouvert {
            self.ouvert = false;
        }
    }
}

/// Raccourcis du sélecteur : PROM, ROM, bibliothèque, dossier personnel.
pub fn raccourcis(biblio: &Path) -> Vec<Raccourci> {
    let mut v = vec![
        Raccourci {
            libelle: "PROM".into(),
            chemin: biblio.join(DOSSIER),
        },
        Raccourci {
            libelle: "ROM".into(),
            chemin: biblio.join(rdk_pilotes::pilotes::rom_machine::DOSSIER_ROM),
        },
        Raccourci {
            libelle: "🖥 Apple 1".into(),
            chemin: biblio.to_path_buf(),
        },
    ];
    if let Some(h) = std::env::var_os("HOME") {
        v.push(Raccourci {
            libelle: "🏠".into(),
            chemin: PathBuf::from(h),
        });
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intel_hex_aller_retour() {
        let d = std::env::temp_dir().join(format!("a1-prom-{}", std::process::id()));
        let f = d.join("PROM").join("essai.hex");
        let octets: Vec<u8> = (0..300u32).map(|i| (i * 7) as u8).collect();
        ecrire_fichier(&f, 0xE000, &octets).unwrap();
        let t = std::fs::read_to_string(&f).unwrap();
        assert!(
            t.starts_with(':') && t.trim_end().ends_with(":00000001FF"),
            "{t}"
        );
        assert_eq!(image_fichier(&f).unwrap(), octets);
        let b = d.join("essai.bin");
        ecrire_fichier(&b, 0, &octets).unwrap();
        assert_eq!(std::fs::read(&b).unwrap(), octets);
        let _ = std::fs::remove_dir_all(&d);
    }
}
