//! Projets GitHub (v1.0 : dossier GitHub de la bibliothèque, « Ajouter un
//! projet GitHub », « ⟳ Mettre à jour », « ▶ Tester le dernier build »),
//! avec le module réseau de la v1.0 ([`crate::v1::github`]).
//!
//! La 2.0 range les fichiers téléchargés **dans sa bibliothèque**
//! (`RDK librairies/Apple 1/GitHub/<propriétaire>/<dépôt>/<commit>/`) : ils
//! apparaissent dans la bibliothèque commune, où Charger, ▶ Play, le
//! double-clic et l'éditeur s'en servent comme des autres programmes. La
//! liste des projets est gardée dans `apple1-rdk.cfg` ; le jeton (facultatif,
//! dépôts privés et artefacts Actions) vient de la variable `GITHUB_TOKEN`
//! ou est saisi pour la session seulement : il n'est jamais écrit.

use crate::i18n::{t, tf, tr};
use crate::trf;
use crate::v1::github::{Event, Job, Meta, Spec, Ureq, Worker};
use eframe::egui::{self, Color32, RichText};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Sous-dossier de la bibliothèque pour les projets GitHub.
pub const DOSSIER: &str = "GitHub";

/// Ce que la machine doit faire après un événement du réseau.
#[derive(Clone, Debug, PartialEq)]
pub enum Suite {
    /// Message pour la barre d'état (erreur ?).
    Message(String, bool),
    /// « Tester le dernier build » : fichier à lancer, et son origine.
    Lancer(PathBuf, String),
}

#[derive(Default)]
pub struct FenetreGithub {
    pub ouvert: bool,
    saisie: String,
    erreur: Option<String>,
    /// Jeton de la session (jamais enregistré).
    jeton: String,
    travailleur: Option<(Worker, PathBuf)>,
    /// Avancement par projet (texte, fraction).
    avancement: HashMap<String, (String, f32)>,
}

/// Jeton : `GITHUB_TOKEN`, sinon celui de la session.
fn jeton(session: &str) -> Option<String> {
    std::env::var("GITHUB_TOKEN")
        .ok()
        .filter(|t| !t.trim().is_empty())
        .or_else(|| Some(session.trim().to_string()).filter(|t| !t.is_empty()))
}

impl FenetreGithub {
    fn travailleur(&mut self, cache: &Path) -> &Worker {
        if self
            .travailleur
            .as_ref()
            .map(|(_, c)| c != cache)
            .unwrap_or(true)
        {
            self.travailleur = Some((
                Worker::new(Arc::new(Ureq::new()), cache.to_path_buf()),
                cache.to_path_buf(),
            ));
        }
        &self.travailleur.as_ref().expect("travailleur créé").0
    }

    /// Lance un travail réseau sur `spec`.
    pub fn demander(&mut self, cache: &Path, job: Job) {
        let spec = match &job {
            Job::Sync(s) | Job::Latest(s) => s.clone(),
        };
        self.avancement
            .insert(spec.text(), (t("gh_waiting").into(), 0.0));
        let jt = jeton(&self.jeton);
        self.travailleur(cache).send(job, jt);
    }

    /// Événements du réseau arrivés depuis l'image précédente.
    pub fn evenements(&mut self) -> Vec<Suite> {
        let mut v = Vec::new();
        let Some((w, _)) = &self.travailleur else {
            return v;
        };
        while let Ok(e) = w.rx.try_recv() {
            match e {
                Event::Progress(s, texte, f) => {
                    self.avancement.insert(s.text(), (texte, f));
                }
                Event::Synced(s, r) => {
                    self.avancement.remove(&s.text());
                    v.push(match r {
                        Ok(m) => Suite::Message(
                            tf(
                                "gh_synced",
                                &[
                                    &s.label(),
                                    &m.short(),
                                    &m.date.get(..10).unwrap_or(&m.date),
                                    &m.files,
                                ],
                            ),
                            false,
                        ),
                        Err(e) => Suite::Message(trf!("{0} : {1}", s.label(), e), true),
                    });
                }
                Event::Built(s, r) => {
                    self.avancement.remove(&s.text());
                    v.push(match r {
                        Ok(b) => Suite::Lancer(b.path, b.origin),
                        Err(e) => Suite::Message(trf!("{0} : {1}", s.label(), e), true),
                    });
                }
            }
        }
        v
    }

    /// Un travail est en cours (l'image doit être redessinée).
    pub fn occupe(&self) -> bool {
        !self.avancement.is_empty()
    }

    /// Dessine la fenêtre. `projets` : liste de `apple1-rdk.cfg` (modifiée
    /// ici ; `true` en retour = à enregistrer).
    pub fn dessiner(
        &mut self,
        ctx: &egui::Context,
        biblio: &Path,
        projets: &mut Vec<String>,
    ) -> (bool, Vec<Suite>) {
        let mut change = false;
        let mut suites = Vec::new();
        if !self.ouvert {
            return (change, suites);
        }
        let cache = biblio.join(DOSSIER);
        let mut ouvert = true;
        let mut travaux: Vec<Job> = Vec::new();
        egui::Window::new(tr("🐙 Projets GitHub")).id(egui::Id::new("apple1-github")).open(&mut ouvert).default_width(560.0).show(ctx, |ui| {
            ui.label(RichText::new(trf!("Les fichiers sont téléchargés dans {0} : ils apparaissent dans la bibliothèque.", cache.display())).small().weak());
            ui.separator();
            if projets.is_empty() {
                ui.weak(tr("Aucun projet."));
            }
            let mut retirer = None;
            for (i, texte) in projets.iter().enumerate() {
                let Ok(spec) = Spec::parse(texte) else {
                    ui.colored_label(Color32::LIGHT_RED, tf("gh_bad_spec", &[texte]));
                    continue;
                };
                ui.horizontal(|ui| {
                    ui.strong(spec.label());
                    match Meta::load(&spec, &cache) {
                        Some(m) => {
                            ui.label(RichText::new(format!("{} · {}", m.short(), m.date.get(..10).unwrap_or(&m.date))).small().weak()).on_hover_text(format!("{}\n{}", tf("gh_commit_tip", &[&m.short(), &m.branch]), m.message));
                        }
                        None => {
                            ui.label(RichText::new(tr("pas encore téléchargé")).small().weak());
                        }
                    }
                });
                ui.horizontal(|ui| {
                    let occupe = self.avancement.contains_key(&spec.text());
                    if ui.add_enabled(!occupe, egui::Button::new(t("gh_update"))).clicked() {
                        travaux.push(Job::Sync(spec.clone()));
                    }
                    if ui.add_enabled(!occupe, egui::Button::new(t("gh_latest"))).on_hover_text(tr("Dernière Release, sinon dernier artefact Actions (jeton), sinon un source du dépôt, construit et lancé.")).clicked() {
                        travaux.push(Job::Latest(spec.clone()));
                    }
                    if let Some(m) = Meta::load(&spec, &cache) {
                        let d = m.dir(&spec, &cache);
                        ui.label(RichText::new(format!("📁 {}", d.strip_prefix(biblio).unwrap_or(&d).display())).small().weak());
                    }
                    if ui.small_button("🗑").on_hover_text(t("gh_remove")).clicked() {
                        retirer = Some(i);
                    }
                });
                if let Some((m, f)) = self.avancement.get(&spec.text()) {
                    ui.add(egui::ProgressBar::new(*f).text(m.as_str()).desired_width(520.0));
                }
                ui.add_space(4.0);
            }
            if let Some(i) = retirer {
                let p = projets.remove(i);
                change = true;
                let label = Spec::parse(&p).map(|s| s.label()).unwrap_or(p);
                suites.push(Suite::Message(tf("gh_removed", &[&label]) + &trf!(" (les fichiers restent dans {0})", cache.display()), false));
            }
            ui.separator();
            ui.label(t("gh_add_text"));
            ui.horizontal(|ui| {
                let r = ui.add(egui::TextEdit::singleline(&mut self.saisie).desired_width(400.0).hint_text("https://github.com/owner/repo"));
                let entree = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                if ui.button(t("gh_add")).clicked() || entree {
                    match Spec::parse(&self.saisie) {
                        Ok(s) if projets.iter().any(|p| Spec::parse(p).map(|x| x == s).unwrap_or(false)) => self.erreur = Some(tf("gh_already", &[&s.label()])),
                        Ok(s) => {
                            projets.push(s.text());
                            change = true;
                            self.saisie.clear();
                            self.erreur = None;
                            travaux.push(Job::Sync(s));
                        }
                        Err(e) => self.erreur = Some(e),
                    }
                }
            });
            ui.label(RichText::new(t("gh_add_hint")).small().weak());
            if let Some(e) = &self.erreur {
                ui.colored_label(Color32::LIGHT_RED, e.as_str());
            }
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(t("pr_token"));
                ui.add(egui::TextEdit::singleline(&mut self.jeton).password(true).desired_width(260.0)).on_hover_text(tr("Seulement pour les dépôts privés et les artefacts Actions. Gardé pour cette session seulement, jamais écrit ni affiché."));
            });
            if std::env::var("GITHUB_TOKEN").map(|t| !t.trim().is_empty()).unwrap_or(false) {
                ui.label(RichText::new(t("pr_token_env")).small().weak());
            }
        });
        for j in travaux {
            self.demander(&cache, j);
        }
        if self.occupe() {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }
        self.ouvert = ouvert;
        (change, suites)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jeton_de_la_session() {
        if std::env::var("GITHUB_TOKEN").is_err() {
            assert_eq!(jeton("  "), None);
            assert_eq!(jeton(" abc "), Some("abc".into()));
        }
    }

    #[test]
    fn projets_lus_comme_en_v1() {
        let s = Spec::parse("https://github.com/jefftranter/6502/tree/master/asm").unwrap();
        assert_eq!(s.text(), "jefftranter/6502/tree/master/asm");
        assert_eq!(Spec::parse(&s.text()).unwrap(), s);
        assert!(Spec::parse("pas un dépôt").is_err());
    }
}
