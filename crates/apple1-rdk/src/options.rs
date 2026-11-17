//! Réglages de la machine que la v1.0 rangeait dans ses Préférences (cartes
//! GEN2 et P-LAB, BASIC). L'interface commune n'a pas de page de préférences
//! propre à la machine : ils sont changés par les boutons de périphériques du
//! manifeste et la fenêtre « Écran » de la machine, et gardés dans
//! `apple1-rdk.cfg`, dans le dossier du kit.

use crate::crt::{Phosphore, ReglagesEcran};
use std::path::{Path, PathBuf};

/// Nom du fichier des réglages, dans le dossier du kit.
pub const FICHIER: &str = "apple1-rdk.cfg";

/// Source montrée sur le moniteur couleur quand les deux cartes sont branchées.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceCouleur {
    Gen2,
    Plab,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Options {
    /// Carte couleur GEN2 d'Uncle Bernie (v1.0 : « Carte couleur GEN2 »).
    pub gen2: bool,
    /// GEN2 en 50 Hz (PAL) au lieu de 60 Hz.
    pub gen2_50hz: bool,
    /// Accolades « grasses » du générateur de caractères GEN2 (défaut v1.0).
    pub accolades_grasses: bool,
    /// Carte graphique P-LAB (TMS9918A).
    pub plab: bool,
    /// Cavalier NMI de la P-LAB.
    pub plab_nmi: bool,
    /// Image CodeTank (EEPROM de la P-LAB, 16 Ko max) choisie par l'utilisateur.
    pub codetank: Option<PathBuf>,
    pub source: SourceCouleur,
    /// RDK G2 Basic intégré (défaut v1.0) ou le BASIC d'origine de
    /// l'utilisateur. 2.0 : suit le bouton commun « ROM réelle / Émulation »
    /// (pilote `rom-machine`) ; n'est plus enregistré.
    pub basic_g2: bool,
    /// Phosphore du moniteur monochrome (v1.0 : vert par défaut).
    pub phosphore: Phosphore,
    /// Écran vintage (v1.0 : « screening »).
    pub ecran: ReglagesEcran,
    /// Chargement de la bibliothèque : 📼 par la cassette (C100R puis
    /// lecture de l'ACI, en temps réel) au lieu de ⚡ rapide (v1.0 : mode
    /// du double-clic).
    pub cassette: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            gen2: false,
            gen2_50hz: false,
            accolades_grasses: true,
            plab: false,
            plab_nmi: false,
            codetank: None,
            source: SourceCouleur::Gen2,
            basic_g2: true,
            phosphore: Phosphore::Vert,
            ecran: ReglagesEcran::default(),
            cassette: false,
        }
    }
}

fn oui(v: &str) -> bool {
    matches!(v.trim(), "oui" | "1" | "true" | "vrai")
}

fn nombre(v: &str, defaut: f32) -> f32 {
    v.trim()
        .parse::<f32>()
        .map(|x| x.clamp(0.0, 1.0))
        .unwrap_or(defaut)
}

fn texte(b: bool) -> &'static str {
    if b {
        "oui"
    } else {
        "non"
    }
}

impl Options {
    /// Lit les réglages ; un fichier absent ou illisible donne les défauts de la v1.0.
    pub fn lire(dossier: &Path) -> Options {
        let mut o = Options::default();
        let Ok(t) = std::fs::read_to_string(dossier.join(FICHIER)) else {
            return o;
        };
        for l in t.lines() {
            let Some((k, v)) = l.split_once('=') else {
                continue;
            };
            match k.trim() {
                "gen2" => o.gen2 = oui(v),
                "gen2_50hz" => o.gen2_50hz = oui(v),
                "accolades_grasses" => o.accolades_grasses = oui(v),
                "plab" => o.plab = oui(v),
                "plab_nmi" => o.plab_nmi = oui(v),
                "codetank" => {
                    o.codetank = Some(PathBuf::from(v.trim())).filter(|p| !p.as_os_str().is_empty())
                }
                "source_couleur" => {
                    o.source = if v.trim() == "plab" {
                        SourceCouleur::Plab
                    } else {
                        SourceCouleur::Gen2
                    }
                }
                "phosphore" => o.phosphore = Phosphore::depuis_cle(v),
                "ecran_vintage" => o.ecran.actif = oui(v),
                "ecran_intensite" => o.ecran.intensite = nombre(v, o.ecran.intensite),
                "ecran_lignes" => o.ecran.lignes = oui(v),
                "ecran_adoucir" => o.ecran.adoucir = oui(v),
                "ecran_halo" => o.ecran.halo = oui(v),
                "ecran_remanence" => o.ecran.remanence = nombre(v, o.ecran.remanence),
                "ecran_bombe" => o.ecran.bombe = oui(v),
                "ecran_courbure" => o.ecran.courbure = nombre(v, o.ecran.courbure),
                "ecran_scintillement" => o.ecran.scintillement = oui(v),
                "ecran_masque_rvb" => o.ecran.masque_rvb = oui(v),
                "chargement" => o.cassette = v.trim() == "cassette",
                _ => {}
            }
        }
        o
    }

    pub fn texte(&self) -> String {
        format!(
            "# Retro Dev Kit Apple 1 2.0 : réglages de la machine (boutons Périphériques)\n\
             gen2={}\ngen2_50hz={}\naccolades_grasses={}\nplab={}\nplab_nmi={}\ncodetank={}\nsource_couleur={}\nphosphore={}\n\
             ecran_vintage={}\necran_intensite={:.2}\necran_lignes={}\necran_adoucir={}\necran_halo={}\necran_remanence={:.2}\n\
             ecran_bombe={}\necran_courbure={:.2}\necran_scintillement={}\necran_masque_rvb={}\nchargement={}\n",
            texte(self.gen2),
            texte(self.gen2_50hz),
            texte(self.accolades_grasses),
            texte(self.plab),
            texte(self.plab_nmi),
            self.codetank.as_ref().map(|p| p.display().to_string()).unwrap_or_default(),
            if self.source == SourceCouleur::Plab { "plab" } else { "gen2" },
            self.phosphore.cle(),
            texte(self.ecran.actif),
            self.ecran.intensite,
            texte(self.ecran.lignes),
            texte(self.ecran.adoucir),
            texte(self.ecran.halo),
            self.ecran.remanence,
            texte(self.ecran.bombe),
            self.ecran.courbure,
            texte(self.ecran.scintillement),
            texte(self.ecran.masque_rvb),
            if self.cassette { "cassette" } else { "rapide" },
        )
    }

    pub fn ecrire(&self, dossier: &Path) -> std::io::Result<()> {
        std::fs::write(dossier.join(FICHIER), self.texte())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aller_retour() {
        let d = std::env::temp_dir().join(format!("a1-options-{}", std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        assert_eq!(Options::lire(&d), Options::default());
        let o = Options {
            gen2: true,
            gen2_50hz: true,
            accolades_grasses: false,
            plab: true,
            plab_nmi: true,
            codetank: Some("/x/ct.bin".into()),
            source: SourceCouleur::Plab,
            basic_g2: true,
            phosphore: Phosphore::Ambre,
            ecran: ReglagesEcran {
                actif: false,
                intensite: 0.25,
                scintillement: true,
                ..ReglagesEcran::default()
            },
            cassette: true,
        };
        o.ecrire(&d).unwrap();
        assert_eq!(Options::lire(&d), o);
        let _ = std::fs::remove_dir_all(&d);
    }
}
