//! Fenêtres propres à l'Apple 1 (v1.0), dessinées par la machine : la
//! partie commune n'a pas de fenêtre de machine. L'application donne le
//! contexte egui à la machine au lancement (`OptionsLancement::preparer`) ;
//! la machine dessine ses fenêtres à chaque image, dans `machine_avancer`.
//!
//! - **Programmateur d'EPROM** ([`crate::prom`]), par le bouton PROM commun ;
//! - **Photo → ASCII** ([`crate::ascii_outil`]) ;
//! - **Écran** : phosphore du moniteur (vert, blanc, ambre) et réglages de
//!   l'écran vintage (v1.0 : Préférences → Affichage) ;
//! - **Sauver** : plages de la mémoire vers `.woz`, `.wav`, `.aiff`, `.bin`,
//!   `.prg`, Intel `.hex` ou `.dump` (v1.0 : bouton 💾 Sauver).

use crate::ascii_outil::OutilAscii;
use crate::crt::Phosphore;
use crate::i18n::tr;
use crate::prom::FenetreProm;
use crate::trf;
use crate::MachineApple1;
use eframe::egui;
use rdk_interface::outils::selecteur::{self, Raccourci};
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct Fenetres {
    pub prom: FenetreProm,
    pub ascii: OutilAscii,
    /// Fenêtre « Écran » ouverte.
    pub ecran: bool,
    pub sauver: FenetreSauver,
    /// v1.0 : ⚙ réglages par programme.
    pub reglages: crate::programmes::FenetreReglages,
    /// v1.0 : projets GitHub.
    pub github: crate::projets_github::FenetreGithub,
}

impl Fenetres {
    pub fn dessiner(&mut self, ctx: &egui::Context, m: &mut MachineApple1) {
        self.prom.dessiner(ctx, m);
        self.ascii.dessiner(ctx, m);
        self.sauver.dessiner(ctx, m);
        if self.ecran {
            self.ecran = fenetre_ecran(ctx, m);
        }
        if let Some(f) = self.reglages.dessiner(ctx, &m.bibliotheque()) {
            m.reglages_changes(&f);
        }
        m.dessiner_github(ctx, &mut self.github);
        m.dessiner_oscilloscope(ctx);
    }
}

/// Fenêtre « Écran » : renvoie `false` quand elle est fermée.
fn fenetre_ecran(ctx: &egui::Context, m: &mut MachineApple1) -> bool {
    let mut ouvert = true;
    let mut change = false;
    egui::Window::new("Écran de l'Apple 1").id(egui::Id::new("apple1-fenetre-ecran")).open(&mut ouvert).collapsible(false).resizable(false).default_width(420.0).show(ctx, |ui| {
        let o = m.options_mut();
        ui.horizontal(|ui| {
            ui.label(tr("Phosphore du moniteur :"));
            for p in Phosphore::TOUS {
                change |= ui.selectable_value(&mut o.phosphore, p, p.nom()).changed();
            }
        });
        ui.separator();
        let r = &mut o.ecran;
        change |= ui.checkbox(&mut r.actif, tr("Écran vintage (screening)")).on_hover_text(tr("Imitation d'un moniteur à tube des années 1970, comme la photo d'un vrai Apple 1.")).changed();
        ui.add_enabled_ui(r.actif, |ui| {
            change |= ui.add(egui::Slider::new(&mut r.intensite, 0.0..=1.0).text(tr("Intensité"))).changed();
            change |= ui.checkbox(&mut r.lignes, tr("Séparation des lignes de balayage")).on_hover_text(tr("Une fine ligne sombre sous chaque ligne de points (l'effet principal).")).changed();
            change |= ui.checkbox(&mut r.adoucir, tr("Points adoucis (horizontalement)")).changed();
            change |= ui.checkbox(&mut r.halo, tr("Halo lumineux")).changed();
            change |= ui.add(egui::Slider::new(&mut r.remanence, 0.0..=1.0).text(tr("Rémanence du phosphore"))).on_hover_text(tr("Durée pendant laquelle les points s'éteignent (à gauche : aucune). Plus courte en blanc, moyenne en vert, plus longue en ambre.")).changed();
            change |= ui.checkbox(&mut r.bombe, tr("Tube bombé et coins assombris")).changed();
            ui.add_enabled_ui(r.bombe, |ui| {
                change |= ui.add(egui::Slider::new(&mut r.courbure, 0.0..=1.0).text(tr("Courbure"))).changed();
            });
            change |= ui.checkbox(&mut r.scintillement, tr("Scintillement")).changed();
            change |= ui.checkbox(&mut r.masque_rvb, tr("Masque de phosphores RVB (moniteur couleur)")).changed();
        });
        if ui.button(tr("↺ Réglages par défaut")).clicked() {
            o.ecran = crate::crt::ReglagesEcran::default();
            o.phosphore = Phosphore::Vert;
            change = true;
        }
        ui.weak(tr("Avec l'écran vintage, l'image des moniteurs est calculée à 3 fois la résolution de l'Apple 1 (zoom ×1 = zoom 3 de la v1.0) ; sans lui, image native de 280 × 192 points."));
    });
    if change {
        m.enregistrer_reglages();
    }
    ouvert
}

/// Formats proposés par 💾 Sauver.
pub const FORMATS_SAUVER: [(&str, &str); 7] = [
    ("woz", "Dump du Woz Monitor (.woz)"),
    ("wav", "Cassette ACI (.wav)"),
    ("aiff", "Cassette ACI pour smartphone (.aiff)"),
    ("hex", "Intel HEX (.hex, programmateurs de PROM)"),
    ("dump", "Dump RDK (.dump)"),
    ("bin", "Binaire brut (.bin, une plage)"),
    ("prg", "Binaire avec adresse (.prg, une plage)"),
];

/// Dossier de la bibliothèque proposé pour un format.
pub fn dossier_format(biblio: &Path, ext: &str) -> PathBuf {
    match ext {
        "wav" | "aiff" => biblio.join("Cassettes"),
        "hex" => biblio.join(crate::prom::DOSSIER),
        _ => biblio.join("Jeux"),
    }
}

pub struct FenetreSauver {
    pub ouvert: bool,
    plages: String,
    format: usize,
    nom: String,
    chemin: String,
    parcourir: bool,
    message: Option<(String, bool)>,
}

impl Default for FenetreSauver {
    fn default() -> Self {
        FenetreSauver {
            ouvert: false,
            plages: "0300.03FF".into(),
            format: 0,
            nom: "programme".into(),
            chemin: String::new(),
            parcourir: false,
            message: None,
        }
    }
}

impl FenetreSauver {
    pub fn dessiner(&mut self, ctx: &egui::Context, m: &mut MachineApple1) {
        if !self.ouvert {
            return;
        }
        let biblio = m.bibliotheque();
        let mut ouvert = true;
        egui::Window::new("💾 Sauver la mémoire").id(egui::Id::new("apple1-fenetre-sauver")).open(&mut ouvert).collapsible(false).resizable(true).default_width(560.0).show(ctx, |ui| {
            egui::Grid::new("apple1-sauver").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                ui.label(tr("Plages :"));
                ui.add(egui::TextEdit::singleline(&mut self.plages).desired_width(320.0).font(egui::TextStyle::Monospace)).on_hover_text(tr("Comme pour l'ACI : 0300.03FF, ou 004A.00FF 0800.0FFF pour un programme BASIC (page zéro puis programme)"));
                ui.end_row();
                ui.label(tr("Format :"));
                let avant = self.format;
                egui::ComboBox::from_id_salt("apple1-sauver-format").selected_text(tr(FORMATS_SAUVER[self.format].1)).width(320.0).show_ui(ui, |ui| {
                    for (i, f) in FORMATS_SAUVER.iter().enumerate() {
                        ui.selectable_value(&mut self.format, i, tr(f.1));
                    }
                });
                if avant != self.format || self.chemin.is_empty() {
                    let ext = FORMATS_SAUVER[self.format].0;
                    self.chemin = dossier_format(&biblio, ext).join(format!("{}.{ext}", self.nom)).display().to_string();
                }
                ui.end_row();
                ui.label(tr("Fichier :"));
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.chemin).desired_width(380.0));
                    if ui.button("…").clicked() {
                        self.parcourir = !self.parcourir;
                    }
                });
                ui.end_row();
            });
            if self.parcourir {
                let r = vec![Raccourci { libelle: "🖥 Apple 1".into(), chemin: biblio.clone() }, Raccourci { libelle: "Cassettes".into(), chemin: biblio.join("Cassettes") }, Raccourci { libelle: "PROM".into(), chemin: biblio.join(crate::prom::DOSSIER) }, Raccourci { libelle: "Jeux".into(), chemin: biblio.join("Jeux") }];
                if let Some(f) = selecteur::panneau(ui, &mut self.chemin, &r, true) {
                    self.chemin = f.display().to_string();
                    self.parcourir = false;
                }
            }
            ui.horizontal(|ui| {
                if ui.button(tr("💾 Enregistrer")).clicked() {
                    let c = PathBuf::from(self.chemin.trim());
                    let existe = c.exists();
                    let r = if existe { Err(trf!("{0} existe déjà : changez de nom", c.display())) } else { m.sauver(&c, &self.plages) };
                    match r {
                        Ok(t) => {
                            m.informer(t.clone(), false);
                            self.message = Some((t, false));
                        }
                        Err(e) => self.message = Some((e, true)),
                    }
                }
                if ui.button(tr("Fermer")).clicked() {
                    self.ouvert = false;
                }
            });
            if let Some((t, err)) = &self.message {
                let c = if *err { ui.visuals().warn_fg_color } else { ui.visuals().weak_text_color() };
                ui.colored_label(c, t);
            }
        });
        if !ouvert {
            self.ouvert = false;
        }
    }
}

/// Art ASCII de la v1.0 (dessins originaux ; portraits CC BY-SA, voir LICENCES.md).
pub const EXEMPLES_ART: [(&str, &str); 12] = [
    (
        "pomme.txt",
        include_str!("../../apple1-gui/ascii/pomme.txt"),
    ),
    (
        "carte-apple1.txt",
        include_str!("../../apple1-gui/ascii/carte-apple1.txt"),
    ),
    (
        "puce-6502.txt",
        include_str!("../../apple1-gui/ascii/puce-6502.txt"),
    ),
    (
        "cassette.txt",
        include_str!("../../apple1-gui/ascii/cassette.txt"),
    ),
    (
        "fusee.txt",
        include_str!("../../apple1-gui/ascii/fusee.txt"),
    ),
    ("chat.txt", include_str!("../../apple1-gui/ascii/chat.txt")),
    (
        "crane.txt",
        include_str!("../../apple1-gui/ascii/crane.txt"),
    ),
    (
        "ordinateur.txt",
        include_str!("../../apple1-gui/ascii/ordinateur.txt"),
    ),
    (
        "hello-woz.txt",
        include_str!("../../apple1-gui/ascii/hello-woz.txt"),
    ),
    (
        "portrait-wozniak.txt",
        include_str!("../../apple1-gui/ascii/portrait-wozniak.txt"),
    ),
    (
        "portrait-jobs.txt",
        include_str!("../../apple1-gui/ascii/portrait-jobs.txt"),
    ),
    (
        "LICENCES.md",
        include_str!("../../apple1-gui/ascii/LICENCES.md"),
    ),
];

/// 📥 Exemples : copie les exemples du paquet (`<dossier du kit>/Biblio/`)
/// et l'art ASCII de la v1.0 dans la bibliothèque, sans rien remplacer.
/// C'est un clic de l'utilisateur : la bibliothèque est créée s'il le faut.
pub fn installer_exemples(kit: &Path, biblio: &Path) -> Result<String, String> {
    if biblio.as_os_str().is_empty() {
        return Err(tr("dossier de la bibliothèque inconnu").into());
    }
    let mut n = 0usize;
    let source = kit.join("Biblio");
    if source.is_dir() && source != biblio {
        n += copier_sans_remplacer(&source, biblio)?;
    }
    let art = biblio.join(crate::art::DOSSIER);
    std::fs::create_dir_all(&art).map_err(|e| trf!("{0} : {1}", art.display(), e))?;
    for (nom, texte) in EXEMPLES_ART {
        let p = art.join(nom);
        if !p.exists() {
            std::fs::write(&p, texte).map_err(|e| trf!("{0} : {1}", p.display(), e))?;
            n += 1;
        }
    }
    Ok(trf!(
        "{0} exemple(s) copié(s) dans {1}",
        n,
        biblio.display()
    ))
}

fn copier_sans_remplacer(de: &Path, vers: &Path) -> Result<usize, String> {
    std::fs::create_dir_all(vers).map_err(|e| trf!("{0} : {1}", vers.display(), e))?;
    let mut n = 0;
    for e in std::fs::read_dir(de)
        .map_err(|e| trf!("{0} : {1}", de.display(), e))?
        .flatten()
    {
        let p = e.path();
        let cible = vers.join(e.file_name());
        if p.is_dir() {
            n += copier_sans_remplacer(&p, &cible)?;
        } else if !cible.exists() {
            std::fs::copy(&p, &cible).map_err(|e| trf!("{0} : {1}", cible.display(), e))?;
            n += 1;
        }
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemples_copies_une_fois() {
        let d = std::env::temp_dir().join(format!("a1-exemples-{}", std::process::id()));
        let kit = d.join("kit");
        std::fs::create_dir_all(kit.join("Biblio").join("BASIC")).unwrap();
        std::fs::write(kit.join("Biblio").join("BASIC").join("a.bas"), "10 END\n").unwrap();
        let biblio = d.join("RDK librairies").join("Apple 1");
        let m = installer_exemples(&kit, &biblio).unwrap();
        assert!(
            m.starts_with(&format!("{} exemple", EXEMPLES_ART.len() + 1)),
            "{m}"
        );
        assert!(biblio.join("BASIC").join("a.bas").is_file());
        assert!(installer_exemples(&kit, &biblio)
            .unwrap()
            .starts_with("0 exemple"));
        let _ = std::fs::remove_dir_all(&d);
    }
}
