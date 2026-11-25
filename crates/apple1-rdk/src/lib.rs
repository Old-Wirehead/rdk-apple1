//! # apple1-rdk — l'Apple 1 sur RDK-Interface-Commune
//!
//! Version **2.0** du Retro Dev Kit Apple 1 (Linux). Tout ce qui est commun
//! (fenêtre principale, barre d'icônes, éditeur, moniteur, Dump, débogueur,
//! désassemblage, bibliothèque, assistant, préférences, barre d'état…) vient
//! de **RDK-Interface-Commune 0.1.9** (dépendance Git épinglée sur le commit
//! de fusion de la 0.1.9, étiquette v0.1.9 ; 7 langues, zoom « Ajuster » et menu État depuis la 0.1.7). Ce paquet ne contient que **la liaison** entre la machine de
//! la v1.0 (`apple1-core` : 6502 NMOS, PIA 6820/6821, Woz Monitor, ACI,
//! terminal 40 × 24) et l'interface : [`MachineApple1`] implémente le trait
//! [`rdk_pilotes::Machine`] et ses pilotes, comme le demande
//! `manifest-apple1.md`.
//!
//! | Pilote | Ce que fait l'Apple 1 |
//! |---|---|
//! | `clavier-machine` | touches → ASCII 7 bits en majuscules ; Retour arrière = `_` ; clavier QWERTY dessiné sans pavé numérique |
//! | `video-numerique` | moniteur monochrome : terminal 40 × 24, ROM de caractères 2513, 280 × 192, curseur `@` clignotant ; moniteur couleur : carte GEN2 (280 × 192) ou P-LAB (TMS9918A, 256 × 192) |
//! | `video-couleur` | option du pilote précédent : les deux moniteurs, palette des 16 couleurs GEN2 |
//! | `video-analogique` | vide (pas de signal composite émulé) |
//! | `dump-memoire`, `debogueur-memoire` | `$0000`–`$FFFF`, ROM en lecture seule ; variables du BASIC lues dans sa table |
//! | `debogueur-processeur` | liste commune du 6502 ; Pas, Par-dessus, Sortir, Continuer, Pause, Exécuter jusqu'à |
//! | `desassembleur-memoire` | 6502 NMOS, étiquettes du Woz Monitor, de l'ACI et du programme assemblé |
//! | `editeur-machine` | ▶ Lancer : comme la v1.0, le listing BASIC est tapé dans le BASIC (`E000R`, les lignes, `RUN`) ; assembleur 6502 rangé puis `xxxxR` ; ⏹ = RESET |
//! | `editeur-memoire` | ⬇ RAM : listing BASIC tapé sans `RUN`, ou code assemblé |
//! | `biblio-memoire` | `.wav`/`.aif` (cassette ACI, cassettes connues de la v1.0 reconnues), `.woz` (dump du Woz Monitor ; anciens `.txt`, `.mon`, `.apl` en lecture), `.dump` et Intel `.hex` (formats communs), `.bin`, `.prg`, art ASCII (`.txt`, `.png`, `.jpg`) ; chargement ⚡ rapide ou 📼 par la cassette ; Play = `xxxxR` ou `E2B3R` + `RUN` |
//! | `cassette-machine` | entrée : échantillons à 44 100 Hz vers le comparateur de l'ACI ; sortie : signal de l'ACI rendu à 44 100 Hz |
//! | `syntaxe-basic`, `syntaxe-asm` | BASIC entier de l'Apple 1, assembleur 6502 de la v1.0 |
//! | `machine-etat` | marche, vitesse, PC, mémoire, BASIC, moniteur, modes GEN2 et P-LAB, cassette, enregistrement |
//! | `rom-machine` (0.1.5) | ROM réelle = votre Integer BASIC d'origine (`ROM/integer_basic.bin`, 4 Ko, reconnu par sa signature) ; Émulation = RDK G2 Basic intégré |
//! | les autres | vides (pas de souris, joystick, imprimante ni ports sur l'Apple 1) |
//!
//! **Cartes** (comme la v1.0) : la carte couleur GEN2 d'Uncle Bernie et la
//! carte graphique P-LAB (TMS9918A, CodeTank) sont des périphériques de la
//! machine, branchés par les boutons Périphériques du manifeste ; leurs
//! réglages sont gardés dans `apple1-rdk.cfg` (module [`options`]).
//!
//! **ROM** : seules celles de la v1.0 sont dans le dépôt (Woz Monitor, ACI,
//! 2513). Le BASIC par défaut est, comme en v1.0, **RDK G2 Basic** (clone
//! « salle blanche » du kit, MIT, intégré au cœur) : c'est le mode
//! « Émulation » du bouton commun. Le BASIC d'origine n'est **pas** fourni :
//! l'utilisateur qui le préfère pose sa copie `integer_basic.bin` (4 Ko) dans
//! `RDK librairies/Apple 1/ROM/` ; reconnue, elle libère « ROM réelle ».
//!
//! **Fenêtres de la machine** (v1.0) : programmateur d'EPROM (minipro,
//! Intel HEX), Photo → ASCII, réglages de l'écran vintage et du phosphore,
//! sauvegarde de la mémoire. L'interface commune n'a pas de fenêtre propre à
//! une machine : elles sont dessinées par [`fenetres`] avec le contexte egui
//! reçu au lancement ([`MachineApple1::contexte`]).
//!
//! **Bibliothèque** (0.1.5) : `Documents/RDK librairies/Apple 1/`, lue dans
//! les préférences communes à chaque usage ([`MachineApple1::bibliotheque`]).
//!
//! **Repris aussi de la v1.0** (modules lus dans ses sources, [`v1`]) : son
//! et oscilloscope de la cassette ([`cassette`]), réglages par programme
//! `programs.json` ([`programmes`]), projets GitHub et « Tester le dernier
//! build » ([`projets_github`]), état de la machine `.a1state` (menu 📸 État
//! de l'interface commune 0.1.7).
//!
//! **Langues** (0.1.7) : français, anglais, allemand, russe, espagnol,
//! italien, portugais, selon la langue de l'interface commune ([`i18n`]).

pub mod art;
pub mod ascii_outil;
pub mod cassette;
pub mod clavier;
pub mod contrat_1_1;
pub mod couleur;
pub mod crt;
pub mod ecran;
pub mod fenetres;
pub mod i18n;
pub mod miniature;
pub mod options;
pub mod programmes;
pub mod projets_github;
pub mod prom;
pub mod reconnus;
pub mod syntaxe;
pub mod v1;

use options::{Options, SourceCouleur};

use apple1_core::debug::{self, Debugger, Mission, Stop, Symbols, VarValue};
use apple1_core::machine::Config;
use apple1_core::program::{self, Program};
use apple1_core::tape::{self, TapePlayer};
use apple1_core::{asm6502, g2basic, intbasic, Machine as Apple1};
use i18n::tr;
use rdk_pilotes::configuration::ids;
use rdk_pilotes::formats;
use rdk_pilotes::pilotes::{
    biblio_memoire::BiblioMemoire,
    cassette_machine::CassetteMachine,
    clavier_machine::ClavierMachine,
    debogueur_memoire::DebogueurMemoire,
    debogueur_processeur::{DebogueurProcesseur, REGISTRES_6502},
    desassembleur_memoire::DesassembleurMemoire,
    dump_memoire::DumpMemoire,
    editeur_machine::EditeurMachine,
    editeur_memoire::EditeurMemoire,
    joystick_machine::JoystickMachine,
    machine_etat::MachineEtatPilote,
    machine_imprimante::MachineImprimante,
    port_parallele::PortParallele,
    port_serie::PortSerie,
    souris_machine::SourisMachine,
    syntaxe::Syntaxe,
    video_analogique::VideoAnalogique,
    video_couleur::VideoCouleur,
    video_numerique::VideoNumerique,
};
use rdk_pilotes::{
    Adresse, BoutonPeripherique, CommandeExecution, ConfigurationMachine, CouleurPalette,
    DessinClavier, DocumentEditeur, EcranAssistant, ErreurPilote, EtatMachine, FormatFichier,
    GenreFormat, GenrePeripherique, ImageVideo, JeuRom, Langage, LigneDesassemblee, Machine,
    Marche, ModeRom, Moniteur, PeripheriqueOffert, Processeur, Programme, Registre, Resultat,
    RomAttendue, RomMachine, RomTrouvee, ToucheHote, Variable, VerdictRom, Vitesse,
};
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

/// Version de l'application RDK Apple 1.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Fréquence d'échantillonnage du pilote `cassette-machine` (entrée et sortie).
pub const FREQUENCE_CASSETTE: u32 = 44_100;

/// Variable d'environnement qui peut désigner la copie du BASIC (déclarée
/// dans le manifeste des ROM : la partie commune l'essaie en premier).
pub const VARIABLE_BASIC: &str = "RDK_APPLE1_BASIC";

/// Fichier du BASIC d'origine de l'utilisateur, dans `ROM/` de la bibliothèque.
pub const FICHIER_BASIC: &str = "integer_basic.bin";
/// Taille du BASIC d'origine ($E000-$EFFF).
pub const TAILLE_BASIC: usize = 4096;
/// Identifiant du jeu de ROM (préférences communes).
pub const JEU_ROM: &str = "origine";

/// Démarrage à chaud du BASIC (le programme en mémoire est gardé).
pub const BASIC_CHAUD: &str = "E2B3R";

/// Identifiant du bouton « Enregistrer la sortie ACI ».
pub const BOUTON_ENREGISTRER: &str = "aci-enregistrer";
/// Boutons des cartes et réglages de la v1.0 (Préférences et barre « Cartes »).
pub const BOUTON_GEN2: &str = "gen2";
pub const BOUTON_GEN2_50HZ: &str = "gen2-50hz";
pub const BOUTON_ACCOLADES: &str = "gen2-accolades";
pub const BOUTON_PLAB: &str = "plab";
pub const BOUTON_PLAB_NMI: &str = "plab-nmi";
pub const BOUTON_CODETANK: &str = "codetank";
pub const BOUTON_SOURCE: &str = "source-couleur";
/// 2.0 : fenêtres et réglages de la machine (v1.0 : barre d'outils).
pub const BOUTON_ECRAN_VINTAGE: &str = "ecran-vintage";
pub const BOUTON_REGLAGES_ECRAN: &str = "reglages-ecran";
pub const BOUTON_ASCII: &str = "photo-ascii";
pub const BOUTON_SAUVER: &str = "sauver";
pub const BOUTON_CHARGEMENT_CASSETTE: &str = "chargement-cassette";
pub const BOUTON_EXEMPLES: &str = "exemples";
/// v1.0 : son et oscilloscope de la cassette.
pub const BOUTON_SON: &str = "son-cassette";
pub const BOUTON_OSCILLOSCOPE: &str = "oscilloscope";
/// v1.0 : ⚙ réglages par programme (`programs.json`).
pub const BOUTON_REGLAGES_PROGRAMME: &str = "reglages-programme";
/// v1.0 : projets GitHub et « Tester le dernier build ».
pub const BOUTON_GITHUB: &str = "github";
/// Extension des fichiers d'état (comme la v1.0).
pub const EXTENSION_ETAT: &str = "a1state";

/// Configurations proposées par le manifeste (0.1.5), dans l'ordre : la
/// première est celle du premier lancement.
/// Sous-dossier de l'Apple 1 dans « RDK librairies » (0.1.5).
pub const DOSSIER_BIBLIOTHEQUE: &str = "Apple 1";
pub const CONFIG_VERT: &str = "Apple 1 moniteur vert";
pub const CONFIG_GEN2: &str = "Apple 1 moniteur vert + couleur GEN2";
pub const CONFIG_PLAB: &str = "Apple 1 + P-LAB";

/// Dossier de la bibliothèque où l'utilisateur pose son image CodeTank.
pub const DOSSIER_PLAB: &str = "P-LAB";
/// Taille maximale de l'EEPROM CodeTank (v1.0 : 16 384 octets).
pub const CODETANK_MAX: usize = 16384;

/// Touches d'un listing BASIC (v1.0 : `build::basic_keys`) : `E000R`, puis
/// chaque ligne non vide en majuscules.
pub fn touches_basic(listing: &str) -> String {
    let mut k = String::from("E000R\r");
    for l in listing.lines() {
        let l = l.trim_end();
        if l.trim().is_empty() {
            continue;
        }
        k.push_str(&l.to_ascii_uppercase());
        k.push('\r');
    }
    k
}

/// Facteur de vitesse accélérée.
const ACCELERATION: u64 = 20;

/// Étiquettes du Woz Monitor et de l'ACI pour le désassembleur.
///
/// 2.0 (rdk-apple1#34) : des noms que l'assembleur sait relire (pas de
/// tiret, lu comme une soustraction) ; ceux de la GEN2 sont ceux de
/// `gen2.inc` (SS_GR, SS_LORES…).
pub const ETIQUETTES: &[(u16, &str)] = &[
    (0xFF00, "RESET"),
    (0xFF0F, "NOTCR"),
    (0xFF1A, "ESCAPE"),
    (0xFF1F, "GETLINE"),
    (0xFF29, "NEXTCHAR"),
    (0xFFDC, "PRBYTE"),
    (0xFFE5, "PRHEX"),
    (0xFFEF, "ECHO"),
    (0xC100, "ACI"),
    (0xC170, "ACI_WRITE"),
    (0xC18D, "ACI_READ"),
    (0xD010, "KBD"),
    (0xD011, "KBDCR"),
    (0xD012, "DSP"),
    (0xD013, "DSPCR"),
    (0xE000, "BASIC"),
    (0xE2B3, "BASIC_CHAUD"),
    (0xC250, "SS_GR"),
    (0xC251, "SS_TEXT"),
    (0xC252, "SS_FULL"),
    (0xC253, "SS_MIX"),
    (0xC254, "SS_PAGE1"),
    (0xC255, "SS_PAGE2"),
    (0xC256, "SS_LORES"),
    (0xC257, "SS_HIRES"),
    (0xCC00, "VDP_DATA"),
    (0xCC01, "VDP_CTRL"),
];

fn symboles_de_base() -> Symbols {
    ETIQUETTES
        .iter()
        .map(|(a, n)| (*a, n.to_string()))
        .collect()
}

/// L'Apple 1 vu par l'interface commune.
pub struct MachineApple1 {
    m: Apple1,
    dbg: Debugger,
    /// Contrat 1.1.0 : points d'arrêt reçus de l'interface.
    arrets: contrat_1_1::Arrets,
    /// Contrat 1.1.0 : (ligne du source, adresse) au dernier assemblage.
    lignes_asm: Vec<(usize, u16)>,
    marche: Marche,
    vitesse: Vitesse,
    /// Dossier du kit (réglages, exemples du paquet `Biblio/`).
    dossier: PathBuf,
    symboles: Symbols,
    depart: u16,
    /// Touches rémanentes du clavier dessiné.
    maj: bool,
    ctrl: bool,
    message: Option<String>,
    erreurs: Vec<String>,
    /// Dernier programme chargé par la bibliothèque (pour Play).
    dernier: Option<Program>,
    /// Réglages des cartes et du BASIC (v1.0 : Préférences).
    opts: Options,
    /// Moniteur branché (Préférences de l'interface commune).
    moniteur: Moniteur,
    /// 0.1.5 : dossier de la bibliothèque de la machine (relu à chaque usage).
    bibliotheque: Box<dyn Fn() -> PathBuf>,
    /// Integer BASIC d'origine reconnu (pilote `rom-machine`, mode ROM réelle).
    basic_reel: Option<Vec<u8>>,
    mode_rom: ModeRom,
    /// Écran vintage : un tube par moniteur (rémanence).
    tube_mono: crt::Tube,
    tube_couleur: crt::Tube,
    /// Fenêtres de la machine et contexte egui (fourni par l'application).
    fenetres: fenetres::Fenetres,
    contexte: Arc<OnceLock<eframe::egui::Context>>,
    /// Chargement 📼 par la cassette en cours.
    chargement: Option<Chargement>,
    /// Son de la cassette (v1.0).
    son: cassette::Son,
    /// Réglages (`programs.json`) du dernier programme de la bibliothèque,
    /// vérifiés : lancement et BASIC nécessaire.
    plan: Option<v1::params::Plan>,
    /// Dernier fichier chargé ou lancé depuis la bibliothèque (fenêtre ⚙).
    dernier_chemin: Option<PathBuf>,
}

/// Programme en cours de lecture par l'ACI (v1.0 : `cassette_load`).
#[derive(Clone, Debug)]
struct Chargement {
    nom: String,
    /// Touches de lancement à la fin de la lecture (▶ Play).
    lancer: Option<String>,
    debut: f64,
    fin: Option<f64>,
}

impl MachineApple1 {
    /// Apple 1 standard de la v1.0 : 8 Ko de RAM en `$0000` + `$E000`-`$FEFF`,
    /// ACI ; cartes GEN2 et P-LAB selon `apple1-rdk.cfg` (débranchées par défaut).
    pub fn new(dossier: PathBuf) -> Self {
        let mut m = Apple1::new(Config::default());
        m.board.recording = None;
        let opts = Options::lire(&dossier);
        let mut a = MachineApple1 {
            m,
            dbg: Debugger::default(),
            arrets: contrat_1_1::Arrets::default(),
            lignes_asm: Vec::new(),
            marche: Marche::Marche,
            vitesse: Vitesse::Reelle,
            dossier,
            symboles: symboles_de_base(),
            depart: 0xFF00,
            maj: false,
            ctrl: false,
            message: None,
            erreurs: Vec::new(),
            dernier: None,
            opts,
            moniteur: Moniteur::Monochrome,
            bibliotheque: Box::new(PathBuf::new),
            basic_reel: None,
            mode_rom: ModeRom::Emulation,
            tube_mono: crt::Tube::default(),
            tube_couleur: crt::Tube::default(),
            fenetres: fenetres::Fenetres::default(),
            contexte: Arc::new(OnceLock::new()),
            chargement: None,
            son: cassette::Son::default(),
            plan: None,
            dernier_chemin: None,
        };
        let biblio = a.dossier.join("Biblio");
        a.bibliotheque = Box::new(move || biblio.clone());
        a.opts.basic_g2 = true;
        a.appliquer_cartes();
        a
    }

    /// 0.1.5 : la bibliothèque de la machine est donnée par l'application
    /// (`Documents/RDK librairies/Apple 1`, relue dans les préférences
    /// communes). Par défaut (essais) : `<dossier du kit>/Biblio`.
    pub fn avec_bibliotheque(mut self, f: impl Fn() -> PathBuf + 'static) -> Self {
        self.bibliotheque = Box::new(f);
        self
    }

    /// Dossier de la bibliothèque de la machine.
    pub fn bibliotheque(&self) -> PathBuf {
        (self.bibliotheque)()
    }

    /// Dossier du kit (réglages `apple1-rdk.cfg`, ancienne bibliothèque `Biblio/`).
    pub fn dossier_kit(&self) -> &Path {
        &self.dossier
    }

    /// Contexte egui de l'application, à remplir par `OptionsLancement::preparer` :
    /// sans lui, les fenêtres de la machine ne s'ouvrent pas (essais).
    pub fn contexte(&self) -> Arc<OnceLock<eframe::egui::Context>> {
        self.contexte.clone()
    }

    /// Fenêtres de la machine (essais : savoir laquelle est ouverte).
    pub fn fenetres(&self) -> &fenetres::Fenetres {
        &self.fenetres
    }

    pub fn fenetres_mut(&mut self) -> &mut fenetres::Fenetres {
        &mut self.fenetres
    }

    /// Mode du BASIC (pilote `rom-machine`) : ROM réelle ou émulation.
    pub fn mode_rom(&self) -> ModeRom {
        self.mode_rom
    }

    /// Réglages de la machine (fenêtre « Écran », essais).
    pub fn options_mut(&mut self) -> &mut Options {
        &mut self.opts
    }

    /// Enregistre les réglages de la machine dans `apple1-rdk.cfg`.
    pub fn enregistrer_reglages(&mut self) {
        self.enregistrer_options();
    }

    /// Réglages en cours (cartes, BASIC).
    pub fn options(&self) -> &Options {
        &self.opts
    }

    /// Branche les cartes de `opts` sur la machine (v1.0 : `sync_cards`).
    fn appliquer_cartes(&mut self) {
        let mut c = self.m.board.config.clone();
        c.color_card = self.opts.gen2;
        c.color_50hz = self.opts.gen2_50hz;
        c.plab_card = self.opts.plab;
        c.plab_nmi = self.opts.plab_nmi;
        c.codetank = None;
        if let Some(p) = self.opts.codetank.clone() {
            match std::fs::read(&p) {
                Ok(b) if !b.is_empty() && b.len() <= CODETANK_MAX => c.codetank = Some(b),
                Ok(b) => {
                    let _ = self.erreur(trf!(
                        "{0} : {1} octets ({2} max)",
                        p.display(),
                        b.len(),
                        CODETANK_MAX
                    ));
                }
                Err(e) => {
                    let _ = self.erreur(trf!("{0} : {1}", p.display(), e));
                }
            }
        }
        // rdk-apple1#31 : une GEN2 qu'on branche démarre en texte, page 1
        // (manuel § 5.2), et non dans l'état HIRES de la mise sous tension
        // du noyau v1.0 (écran couleur noir) ; RESET ne touche pas aux
        // commutateurs
        let gen2_branchee = c.color_card && !self.m.board.config.color_card;
        self.m.board.apply_config(c);
        if gen2_branchee {
            texte_page_1(&mut self.m.board.gen2);
        }
        self.m.board.vdp_changed_config();
    }

    fn enregistrer_options(&mut self) {
        if let Err(e) = self.opts.ecrire(&self.dossier) {
            let _ = self.erreur(format!(
                "{} : {e}",
                self.dossier.join(options::FICHIER).display()
            ));
        }
    }

    /// Change un réglage par son bouton de périphérique (comme les cases
    /// des Préférences et la barre « Cartes » de la v1.0).
    pub fn basculer(&mut self, id: &str) {
        self.erreurs.clear();
        let o = &mut self.opts;
        let msg = match id {
            BOUTON_GEN2 => {
                o.gen2 = !o.gen2;
                if o.gen2 {
                    o.source = SourceCouleur::Gen2;
                    trf!("carte couleur GEN2 branchée (RAM $0000-$BFFF) : son image va au moniteur couleur (configuration « {0} »)", CONFIG_GEN2)
                } else {
                    tr("carte couleur GEN2 débranchée").to_string()
                }
            }
            BOUTON_GEN2_50HZ => {
                o.gen2_50hz = !o.gen2_50hz;
                format!(
                    "GEN2 : {}",
                    if o.gen2_50hz {
                        tr("50 Hz (PAL)")
                    } else {
                        tr("60 Hz (NTSC)")
                    }
                )
            }
            BOUTON_ACCOLADES => {
                o.accolades_grasses = !o.accolades_grasses;
                trf!(
                    "GEN2 : accolades {0}",
                    if o.accolades_grasses {
                        tr("grasses")
                    } else {
                        tr("fines")
                    }
                )
            }
            BOUTON_PLAB => {
                o.plab = !o.plab;
                if o.plab {
                    if !o.gen2 {
                        o.source = SourceCouleur::Plab;
                    }
                    trf!("carte P-LAB branchée (TMS9918A en $CC00-$CC01) : son image va au moniteur couleur (configuration « {0} »)", CONFIG_PLAB)
                } else {
                    tr("carte P-LAB débranchée").to_string()
                }
            }
            BOUTON_PLAB_NMI => {
                o.plab_nmi = !o.plab_nmi;
                trf!(
                    "P-LAB : cavalier NMI {0}",
                    if o.plab_nmi {
                        tr("posé")
                    } else {
                        tr("retiré")
                    }
                )
            }
            BOUTON_SOURCE => {
                o.source = if o.source == SourceCouleur::Gen2 {
                    SourceCouleur::Plab
                } else {
                    SourceCouleur::Gen2
                };
                trf!(
                    "moniteur couleur : carte {0}",
                    if o.source == SourceCouleur::Gen2 {
                        "GEN2"
                    } else {
                        "P-LAB"
                    }
                )
            }
            BOUTON_ECRAN_VINTAGE => {
                o.ecran.actif = !o.ecran.actif;
                trf!(
                    "écran vintage {0}",
                    if o.ecran.actif {
                        tr("allumé")
                    } else {
                        tr("éteint")
                    }
                )
            }
            BOUTON_CHARGEMENT_CASSETTE => {
                o.cassette = !o.cassette;
                if o.cassette {
                    tr("chargement 📼 par la cassette : RESET, C100R puis lecture de l'ACI en temps réel").to_string()
                } else {
                    tr("chargement ⚡ rapide : le programme va directement en mémoire").to_string()
                }
            }
            BOUTON_REGLAGES_ECRAN => {
                self.fenetres.ecran = !self.fenetres.ecran;
                return;
            }
            BOUTON_ASCII => {
                self.fenetres.ascii.ouvert = !self.fenetres.ascii.ouvert;
                return;
            }
            BOUTON_SAUVER => {
                self.fenetres.sauver.ouvert = !self.fenetres.sauver.ouvert;
                return;
            }
            BOUTON_SON => {
                o.son_cassette = !o.son_cassette;
                if o.son_cassette {
                    tr("son de la cassette allumé (lecture 📼 et écriture ⏺, vitesse réelle)")
                        .to_string()
                } else {
                    tr("son de la cassette coupé").to_string()
                }
            }
            BOUTON_OSCILLOSCOPE => {
                o.oscilloscope = !o.oscilloscope;
                if o.oscilloscope {
                    tr("oscilloscope de la cassette : il s'ouvre quand la cassette avance")
                        .to_string()
                } else {
                    tr("oscilloscope de la cassette masqué").to_string()
                }
            }
            BOUTON_REGLAGES_PROGRAMME => {
                if self.fenetres.reglages.ouvert {
                    self.fenetres.reglages.ouvert = false;
                } else {
                    let c = self.dernier_chemin.clone();
                    self.fenetres.reglages.ouvrir(c.as_deref());
                }
                return;
            }
            BOUTON_GITHUB => {
                self.fenetres.github.ouvert = !self.fenetres.github.ouvert;
                return;
            }
            BOUTON_EXEMPLES => {
                match fenetres::installer_exemples(&self.dossier, &self.bibliotheque()) {
                    Ok(m) => self.noter(m),
                    Err(e) => {
                        let _ = self.erreur(e);
                    }
                }
                return;
            }
            BOUTON_CODETANK => {
                if o.codetank.is_some() {
                    o.codetank = None;
                    tr("P-LAB : image CodeTank retirée").to_string()
                } else {
                    match self.chercher_codetank() {
                        Some(p) => {
                            let m = trf!(
                                "P-LAB : image CodeTank {0} chargée en $4000-$7FFF",
                                p.display()
                            );
                            self.opts.codetank = Some(p);
                            m
                        }
                        None => {
                            let _ = self.erreur(format!("CodeTank : posez votre image (.bin ou .rom, {CODETANK_MAX} octets au plus) dans {} puis recliquez", self.bibliotheque().join(DOSSIER_PLAB).display()));
                            return;
                        }
                    }
                }
            }
            _ => return,
        };
        self.appliquer_cartes();
        self.enregistrer_options();
        if self.erreurs.is_empty() {
            self.noter(msg);
        }
    }

    /// Première image CodeTank de `P-LAB/` dans la bibliothèque (par ordre
    /// alphabétique), sinon dans l'ancienne bibliothèque du kit `Biblio/P-LAB/`.
    pub fn chercher_codetank(&self) -> Option<PathBuf> {
        [
            self.bibliotheque().join(DOSSIER_PLAB),
            self.dossier.join("Biblio").join(DOSSIER_PLAB),
        ]
        .into_iter()
        .find_map(|d| Self::premiere_image(&d))
    }

    fn premiere_image(d: &Path) -> Option<PathBuf> {
        let mut v: Vec<PathBuf> = std::fs::read_dir(d)
            .ok()?
            .flatten()
            .map(|e| e.path())
            .filter(|p| {
                let ext = p
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_ascii_lowercase();
                matches!(ext.as_str(), "bin" | "rom")
                    && p.metadata()
                        .map(|m| m.len() > 0 && m.len() <= CODETANK_MAX as u64)
                        .unwrap_or(false)
            })
            .collect();
        v.sort();
        v.into_iter().next()
    }

    /// Carte dont l'image va au moniteur couleur.
    pub fn source_couleur(&self) -> Option<SourceCouleur> {
        match (self.opts.gen2, self.opts.plab) {
            (true, true) => Some(self.opts.source),
            (true, false) => Some(SourceCouleur::Gen2),
            (false, true) => Some(SourceCouleur::Plab),
            (false, false) => None,
        }
    }

    /// Accès au cœur de la v1.0 (tests, captures).
    pub fn coeur(&self) -> &Apple1 {
        &self.m
    }

    pub fn coeur_mut(&mut self) -> &mut Apple1 {
        &mut self.m
    }

    /// Texte de l'écran (lignes séparées par `\n`).
    pub fn texte_ecran(&self) -> String {
        self.m.screen_text()
    }

    /// Fait tourner la machine jusqu'à ce qu'elle attende une touche (tests).
    pub fn jusqua_attente(&mut self, cycles: u64) -> bool {
        self.m.run_until_idle(cycles)
    }

    /// Tape du texte au clavier (file de touches de la v1.0).
    pub fn taper(&mut self, texte: &str) {
        self.m.type_text(texte);
    }

    fn noter(&mut self, texte: impl Into<String>) {
        self.message = Some(texte.into());
    }

    fn erreur(&mut self, texte: impl Into<String>) -> ErreurPilote {
        let t = texte.into();
        self.erreurs.clear();
        self.erreurs.push(t.clone());
        ErreurPilote::Message(t)
    }

    /// Est-ce l'Integer BASIC d'origine ? 4096 octets, `JMP` en tête, la
    /// signature de son interpréteur en `$E86B` (comme `debug::basic_present`
    /// de la v1.0), et pas RDK G2 Basic.
    pub fn reconnaitre_basic(octets: &[u8]) -> Result<(), String> {
        if octets.len() != TAILLE_BASIC {
            return Err(trf!(
                "{0} octets au lieu de {1}",
                octets.len(),
                TAILLE_BASIC
            ));
        }
        let lire = |a: u16| octets[(a - 0xE000) as usize];
        if g2basic::present(&lire) {
            return Err(tr("c'est RDK G2 Basic, déjà intégré (mode Émulation)").into());
        }
        if octets[0] != 0x4C || !debug::basic_present(&lire) {
            return Err(tr("ce n'est pas l'Integer BASIC de l'Apple 1 (signature absente)").into());
        }
        Ok(())
    }

    /// Un BASIC (RDK G2 Basic ou celui d'origine) est-il en mémoire en `$E000` ?
    pub fn basic_present(&self) -> bool {
        debug::basic_present(&|a| self.m.board.peek(a))
    }

    /// RDK G2 Basic est-il en mémoire ?
    pub fn g2_present(&self) -> bool {
        g2basic::present(&|a| self.m.board.peek(a))
    }

    /// Range en `$E000` le BASIC choisi (v1.0 : `ensure_basic`) : RDK G2 Basic
    /// intégré par défaut, sinon la copie du BASIC d'origine de l'utilisateur.
    fn assurer_basic(&mut self) -> Resultat<()> {
        if self.opts.basic_g2 {
            if self.basic_present() {
                return Ok(());
            }
            let p = g2basic::program();
            return self.ranger(&p.blocks).map(|_| ());
        }
        if self.basic_present() && !self.g2_present() {
            return Ok(());
        }
        let Some(octets) = self.basic_reel.clone() else {
            return Err(self.erreur(format!("BASIC d'origine absent : il n'est pas fourni (propriétaire). Copiez votre {FICHIER_BASIC} (4 Ko) dans {}, ou donnez son chemin dans {VARIABLE_BASIC} ; ou repassez en « Émulation » (RDK G2 Basic).", self.bibliotheque().join(rdk_pilotes::pilotes::rom_machine::DOSSIER_ROM).display())));
        };
        self.ranger_basic(&octets).map_err(|e| self.erreur(e))
    }

    /// Range une image de 4 Ko du BASIC en `$E000` (premier octet `JMP`).
    pub fn ranger_basic(&mut self, octets: &[u8]) -> Result<(), String> {
        if octets.len() != 4096 || octets[0] != 0x4C {
            return Err(trf!(
                "image du BASIC invalide ({0} octets, il en faut 4096 commençant par JMP)",
                octets.len()
            ));
        }
        if self.m.load(0xE000, octets) > 0 {
            return Err(tr("pas de RAM en $E000").into());
        }
        Ok(())
    }

    /// v1.0 (`ensure_ram`) : un programme au-delà des 8 Ko d'un Apple 1 de
    /// base (Football, Star Trek 2003, Life…) reçoit la RAM qu'il lui faut,
    /// comme une carte mémoire, jusqu'à la fin de sa plage la plus haute
    /// arrondie à 4 Ko (sous $C000 ; la GEN2 a déjà 48 Ko).
    fn assurer_ram(&mut self, blocs: &[(u16, Vec<u8>)]) -> Option<u32> {
        let c = self.m.board.config.clone();
        if c.color_card {
            return None;
        }
        let haut = blocs
            .iter()
            .map(|(a, o)| *a as u32 + o.len() as u32)
            .filter(|&e| e <= 0xC000)
            .max()?;
        let ko = haut.div_ceil(4096) * 4;
        if ko <= c.ram_kb {
            return None;
        }
        let mut c2 = c;
        c2.ram_kb = ko;
        self.m.board.apply_config(c2);
        Some(ko)
    }

    fn ranger(&mut self, blocs: &[(u16, Vec<u8>)]) -> Resultat<usize> {
        if let Some(ko) = self.assurer_ram(blocs) {
            self.noter(trf!("RAM portée à {0} Ko pour ce programme", ko));
        }
        let mut total = 0;
        for (a, o) in blocs {
            if self.m.load(*a, o) > 0 {
                return Err(self.erreur(trf!(
                    "pas de RAM pour ${0}-${1}",
                    format!("{:04X}", a),
                    format!("{:04X}", (*a as usize + o.len().max(1) - 1) as u16)
                )));
            }
            total += o.len();
        }
        Ok(total)
    }

    /// RESET puis touches tapées au prompt du Woz Monitor.
    fn reset_et_taper(&mut self, touches: &str) {
        self.machine_reset();
        self.m.run_cycles(20_000);
        self.m.type_text(touches);
    }

    fn assembler(&mut self, doc: &DocumentEditeur) -> Resultat<asm6502::Assembly> {
        match asm6502::assemble_text(&doc.texte, doc.chemin.as_deref(), &[]) {
            // v1.0 (`as_empty`) : rien à assembler, rien à lancer
            // (rdk-apple1#17, point 2)
            Ok(a) if a.bytes.is_empty() => Err(self.erreur(tr("aucune instruction à assembler"))),
            Ok(a) => {
                self.symboles = symboles_de_base();
                self.symboles.extend(debug::symbols_from(&a.symbols));
                self.depart = a.origin;
                // contrat 1.1.0 : les lignes du source ont une adresse
                self.lignes_asm = contrat_1_1::adresses_des_lignes(&a);
                self.appliquer_arrets();
                Ok(a)
            }
            Err(e) => {
                let n = e.len();
                let premiere = e.first().map(|x| x.to_string()).unwrap_or_default();
                Err(self.erreur(trf!("{0} erreur(s) d'assemblage ; {1}", n, premiere)))
            }
        }
    }

    /// Contrat 1.1.0 : points d'arrêt posés dans le débogueur de la v1.0.
    fn appliquer_arrets(&mut self) {
        self.dbg.breaks = self.arrets.adresses_completes(&self.lignes_asm);
        self.dbg.basic_breaks = self.arrets.basic.clone();
    }

    /// Contrôle du listing (même analyse que le pilote `syntaxe-basic`).
    fn verifier_basic(&mut self, texte: &str) -> Resultat<usize> {
        let l = intbasic::tokenize(texte);
        if let Some(e) = l.errors.first() {
            return Err(self.erreur(trf!("{0} erreur(s) BASIC ; {1}", l.errors.len(), e)));
        }
        if l.lines.is_empty() {
            return Err(self.erreur(tr("aucune ligne BASIC numérotée")));
        }
        Ok(l.lines.len())
    }

    /// Comme la v1.0 (`run_source`) : le BASIC choisi en `$E000`, RESET, puis
    /// le listing tapé au clavier (`E000R`, les lignes en majuscules) et,
    /// après une pause au hasard (graine de RND), `RUN`.
    /// ▶ Lancer et ⬇ RAM d'un BASIC, **comme la v1.0** (`editor_run`) : le
    /// listing est tokenisé, LOMEM et HIMEM sont choisis d'après sa taille
    /// (`intbasic::layout` : 8 Ko avec LOMEM $0800 ou $0300, puis 12, 16 ou
    /// 32 Ko), la RAM est agrandie si besoin, le programme et sa page zéro
    /// sont posés en mémoire, puis `E2B3R` (démarrage à chaud du BASIC) et,
    /// pour Lancer, `RUN`.
    ///
    /// Correction de rdk-apple1#17, point 1 : la 2.0 tapait le listing au
    /// clavier, dans la mémoire par défaut de l'Integer BASIC (LOMEM $0800,
    /// HIMEM $1000) : `*** MEM FULL ERR` dès 2 Ko, et une minute et demie de
    /// frappe pour 94 lignes.
    fn taper_basic(&mut self, texte: &str, lancer: bool) -> Resultat<String> {
        let n = self.verifier_basic(texte)?;
        let l = intbasic::tokenize(texte);
        let (lomem, himem) = intbasic::layout(l.size(), None);
        let image = intbasic::Image::build(&l.lines, lomem, himem).map_err(|_| {
            self.erreur(trf!(
                "programme trop grand : {0} octets pour {1} de mémoire",
                l.size(),
                himem - lomem
            ))
        })?;
        self.machine_reset();
        self.m.run_cycles(20_000);
        self.assurer_basic()?;
        self.ranger(&image.blocks())?;
        self.m
            .type_text(if lancer { "E2B3R\rRUN\r" } else { "E2B3R\r" });
        self.erreurs.clear();
        let basic = if self.g2_present() {
            tr("RDK G2 Basic")
        } else {
            tr("Integer BASIC")
        };
        Ok(trf!(
            "{0} : {1} ligne(s), {2} octets, LOMEM ${3}, HIMEM ${4} ({5} Ko) ; E2B3R{6}",
            basic,
            n,
            l.size(),
            format!("{:04X}", image.lomem),
            format!("{:04X}", image.himem),
            image.ram_kb(),
            if lancer {
                tr(" puis RUN")
            } else {
                tr(" ; tapez RUN ou LIST")
            }
        ))
    }

    /// v1.0 (`cassette_load`, mode 📼 du double-clic) : le programme va sur
    /// la cassette virtuelle (converti en signal ACI s'il n'en est pas un),
    /// RESET, `C100R` puis la commande de lecture de l'ACI ; la cassette défile
    /// en temps réel. Avec ▶, le programme est lancé à la fin de la lecture.
    fn charger_par_cassette(
        &mut self,
        programme: &Programme,
        p: Program,
        lancer: Option<String>,
    ) -> Resultat<()> {
        if p.blocks.is_empty() {
            return Err(self.erreur(trf!("{0} : rien à charger", programme.nom)));
        }
        let wav = if program::kind_of(Path::new(&programme.nom), None) == program::Kind::Wav
            && !programme.octets.is_empty()
            && apple1_core::aciwav::import_wav_bytes(&programme.octets).is_ok()
        {
            programme.octets.clone()
        } else {
            let segs: Vec<(u16, &[u8])> =
                p.blocks.iter().map(|(a, d)| (*a, d.as_slice())).collect();
            let o = apple1_core::aciwav::ExportOptions {
                timing: self.m.board.config.timing,
                ..Default::default()
            };
            apple1_core::aciwav::export_wav_bytes(&segs, &o)
                .map_err(|e| self.erreur(trf!("{0} : {1}", programme.nom, e)))?
        };
        let audio = apple1_core::wav::parse_audio(&wav)
            .map_err(|e| self.erreur(trf!("{0} : {1}", programme.nom, e)))?;
        self.assurer_ram(&p.blocks);
        self.m.board.player = Some(TapePlayer::from_samples(
            audio.best_channel(),
            audio.sample_rate,
            programme.nom.clone(),
        ));
        let cmd = p.aci_read_command();
        self.machine_reset();
        self.m.run_cycles(20_000);
        self.m.type_text(&format!("C100R\r{cmd}\r"));
        if !Self::est_basic(&p) {
            if let Some((a, _)) = p.blocks.first() {
                self.depart = *a;
            }
        }
        self.chargement = Some(Chargement {
            nom: programme.nom.clone(),
            lancer,
            debut: self.m.board.now_secs(),
            fin: None,
        });
        self.dernier = Some(p);
        self.erreurs.clear();
        self.noter(trf!(
            "{0} : lecture de la cassette (C100R, {1})",
            programme.nom,
            cmd
        ));
        Ok(())
    }

    /// Suit la lecture 📼 : quand la cassette est finie, le programme est en
    /// mémoire (et lancé pour ▶). v1.0 : `loading_tick`.
    fn suivre_chargement(&mut self) {
        let Some(c) = self.chargement.clone() else {
            return;
        };
        let maintenant = self.m.board.now_secs();
        let etat = self
            .m
            .board
            .player
            .as_ref()
            .map(|p| (p.started_at.is_some(), p.finished(maintenant)));
        match etat {
            None => {
                self.chargement = None;
                return;
            }
            Some((false, _)) if maintenant - c.debut > 20.0 => {
                self.chargement = None;
                let _ = self.erreur(trf!(
                    "{0} : l'ACI ne lit pas la cassette (tapez C100R puis la plage suivie de R)",
                    c.nom
                ));
                return;
            }
            Some((true, true)) if c.fin.is_none() => {
                if let Some(ch) = &mut self.chargement {
                    ch.fin = Some(maintenant);
                }
            }
            _ => {}
        }
        let fin = self.chargement.as_ref().and_then(|c| c.fin);
        if fin.map(|f| maintenant - f > 0.3).unwrap_or(false) {
            self.chargement = None;
            match c.lancer {
                Some(touches) => {
                    self.m.reset();
                    self.m.run_cycles(20_000);
                    self.m.type_text(&touches);
                    self.noter(trf!(
                        "{0} : chargé par la cassette, lancé ({1})",
                        c.nom,
                        touches.replace('\r', " ").trim()
                    ));
                }
                None => self.noter(trf!("{0} : chargé par la cassette", c.nom)),
            }
        }
    }

    /// Chargement 📼 en cours ?
    pub fn chargement_en_cours(&self) -> bool {
        self.chargement.is_some()
    }

    fn est_basic(p: &Program) -> bool {
        p.blocks.iter().any(|b| b.0 == intbasic::ZP_START)
    }

    /// Art ASCII de la v1.0 : dessin `.txt` ou photo, en programme 6502 (`300R`).
    fn programme_art(&mut self, programme: &Programme, chemin: &Path) -> Resultat<Option<Program>> {
        let titre = chemin
            .file_stem()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let dessin = if art::est_image(&programme.nom) {
            art::depuis_image(&programme.octets)
                .map_err(|e| self.erreur(trf!("{0} : {1}", programme.nom, e)))?
        } else if Self::est_art(programme, chemin) {
            apple1_core::asciiart::Art::from_text(&String::from_utf8_lossy(&programme.octets))
        } else {
            return Ok(None);
        };
        let a = asm6502::assemble_text(&art::source(&dessin, &titre), None, &[]).map_err(|e| {
            self.erreur(trf!(
                "art ASCII : {0}",
                e.first().map(|x| x.to_string()).unwrap_or_default()
            ))
        })?;
        Ok(Some(Program {
            blocks: vec![(a.origin, a.bytes)],
            run: Some(a.origin),
            notes: vec![tr("art ASCII").into()],
        }))
    }

    /// Le texte se lit comme un vidage du Woz Monitor (`0300: A9 01…`).
    fn est_vidage_woz(programme: &Programme) -> bool {
        program::load_bytes(&programme.octets, program::Kind::Hex, None)
            .map(|p| !p.blocks.is_empty())
            .unwrap_or(false)
    }

    /// Un `.txt` de dessin (et pas un vidage du Woz Monitor).
    fn est_art(programme: &Programme, chemin: &Path) -> bool {
        let txt = chemin
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("txt"))
            .unwrap_or(false);
        txt && !Self::est_vidage_woz(programme)
            && std::str::from_utf8(&programme.octets)
                .map(|t| art::ressemble_art(t, chemin))
                .unwrap_or(false)
    }

    fn programme_de(&mut self, programme: &Programme) -> Resultat<Program> {
        let chemin = programme
            .chemin
            .clone()
            .unwrap_or_else(|| PathBuf::from(&programme.nom));
        if let Some(p) = self.programme_art(programme, &chemin)? {
            self.plan = None;
            return Ok(p);
        }
        // v1.0 : réglages enregistrés (⚙, programs.json du dossier)
        self.plan = None;
        if let Some(reglages) = programmes::enregistres(&chemin) {
            let plan = programmes::plan(&chemin, &reglages, programme.octets.len())
                .map_err(|e| self.erreur(e))?;
            let mut p = programmes::construire(&chemin, &programme.octets, &plan)
                .map_err(|e| self.erreur(trf!("{0} : {1}", programme.nom, e)))?;
            p.notes.push(tr("réglages de programs.json").into());
            self.plan = Some(plan);
            return Ok(p);
        }
        // binaire brut sans réglages : en $0300, comme la v1.0 (`params::from_file`)
        if program::kind_of(&chemin, Some(&programme.octets)) == program::Kind::Bin {
            let fin = (0x300 + programme.octets.len().max(1) - 1).min(0xFFFF);
            let reglages = v1::params::ProgParams {
                load_addr: "300".into(),
                ranges: format!("300.{fin:X}"),
                run_addr: "300".into(),
                run_cmd: "300R".into(),
                ..Default::default()
            };
            let plan = programmes::plan(&chemin, &reglages, programme.octets.len())
                .map_err(|e| self.erreur(e))?;
            let p = programmes::construire(&chemin, &programme.octets, &plan)
                .map_err(|e| self.erreur(trf!("{0} : {1}", programme.nom, e)))?;
            self.plan = Some(plan);
            return Ok(p);
        }
        let nom = chemin
            .file_name()
            .map(|n| n.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        // le BASIC de l'utilisateur, posé dans la bibliothèque
        if programme.octets.len() == 4096 && programme.octets[0] == 0x4C && nom.contains("basic") {
            return Ok(Program {
                blocks: vec![(0xE000, programme.octets.clone())],
                run: Some(0xE000),
                notes: vec![],
            });
        }
        if nom.ends_with(".prg") {
            if programme.octets.len() < 3 {
                return Err(self.erreur(tr("fichier .prg trop court")));
            }
            let a = u16::from_le_bytes([programme.octets[0], programme.octets[1]]);
            return Ok(Program {
                blocks: vec![(a, programme.octets[2..].to_vec())],
                run: Some(a),
                notes: vec![],
            });
        }
        // 0.1.5 : .dump et Intel .hex, formats communs ; un ancien .hex de la
        // v1.0 au format du Woz Monitor est encore lu (avec un conseil)
        let ext = chemin
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if ext == formats::EXT_DUMP || ext == formats::EXT_HEX {
            match formats::lire_format_commun(&ext, &programme.octets) {
                Some(Ok(blocs)) if !blocs.is_empty() => {
                    let blocks: Vec<(u16, Vec<u8>)> = blocs
                        .into_iter()
                        .map(|b| (b.adresse as u16, b.octets))
                        .collect();
                    let run = blocks.first().map(|b| b.0);
                    return Ok(Program {
                        blocks,
                        run,
                        notes: vec![if ext == formats::EXT_HEX {
                            tr("Intel HEX").into()
                        } else {
                            tr("dump RDK").into()
                        }],
                    });
                }
                Some(Err(e)) if ext == formats::EXT_HEX && Self::est_vidage_woz(programme) => {
                    let _ = e;
                    let mut p = program::load_bytes(&programme.octets, program::Kind::Hex, None)
                        .map_err(|e| self.erreur(trf!("{0} : {1}", programme.nom, e)))?;
                    p.notes.push(tr("ancien .hex au format du Woz Monitor : renommez-le en .woz (.hex = Intel HEX)").into());
                    return Ok(p);
                }
                Some(Err(e)) => return Err(self.erreur(trf!("{0} : {1}", programme.nom, e))),
                _ => return Err(self.erreur(trf!("{0} : fichier vide", programme.nom))),
            }
        }
        let genre = program::kind_of(&chemin, Some(&programme.octets));
        // cassettes connues de la v1.0 : plages et lancement de la table
        if genre == program::Kind::Wav {
            if let Ok(imp) = apple1_core::aciwav::import_wav_bytes(&programme.octets) {
                if let Some(k) = reconnus::identifier(&imp, Some(&chemin)) {
                    let plages =
                        program::parse_ranges(k.known.ranges).map_err(|e| self.erreur(e))?;
                    let mut p = program::wav_with_ranges(&imp, &plages)
                        .map_err(|e| self.erreur(trf!("{0} : {1}", programme.nom, e)))?;
                    if !k.known.basic {
                        p.run = program::parse_addr(k.known.run_cmd.trim_end_matches('R')).ok();
                    }
                    p.notes.push(trf!("cassette reconnue : {0}", k.known.title));
                    return Ok(p);
                }
            }
        }
        program::load_bytes(&programme.octets, genre, None)
            .map_err(|e| self.erreur(trf!("{0} : {1}", programme.nom, e)))
    }

    /// Touches qui lancent le programme : la commande des réglages
    /// (`programs.json`), sinon `E2B3R` + `RUN` pour un programme BASIC (le
    /// BASIC est d'abord mis en mémoire), sinon `xxxxR`.
    fn touches_lancement(&mut self, p: &Program) -> Resultat<String> {
        if let Some(plan) = self.plan.clone().filter(|pl| !pl.run_keys.is_empty()) {
            if plan.needs_basic {
                self.assurer_basic()?;
            }
            return Ok(format!("{}\r", plan.run_keys));
        }
        if Self::est_basic(p) {
            self.assurer_basic()?;
            return Ok(format!("{BASIC_CHAUD}\rRUN\r"));
        }
        let a = p
            .run
            .or_else(|| p.blocks.first().map(|b| b.0))
            .unwrap_or(0x0300);
        Ok(format!("{a:X}R\r"))
    }

    /// v1.0 : lance un fichier trouvé par « ▶ Tester le dernier build » :
    /// source assembleur construit (ca65 / ld65 ou assembleur intégré,
    /// `apple1.toml`), listing BASIC tapé, ou programme joué comme ▶ Play.
    pub fn lancer_fichier(&mut self, chemin: &Path) -> Result<String, String> {
        use v1::library::{source_kind, Source};
        match source_kind(chemin, None) {
            Some(Source::Asm) => {
                let (p, depart) = v1::build::assemble(chemin)?;
                self.reset_et_taper("");
                self.ranger(&p.blocks).map_err(|e| e.to_string())?;
                self.depart = depart;
                self.plan = None;
                self.dernier = Some(p);
                self.m.type_text(&format!("{depart:X}R\r"));
                Ok(trf!(
                    "{0} assemblé et lancé ({1}R)",
                    chemin.display(),
                    format!("{depart:X}")
                ))
            }
            Some(Source::Basic) => {
                let texte = std::fs::read_to_string(chemin)
                    .map_err(|e| trf!("{0} : {1}", chemin.display(), e))?;
                self.lancer_source(Langage::Basic, &texte)
                    .map_err(|e| e.to_string())
            }
            _ => {
                let octets =
                    std::fs::read(chemin).map_err(|e| trf!("{0} : {1}", chemin.display(), e))?;
                let nom = chemin
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default();
                let prog = Programme {
                    nom: nom.clone(),
                    chemin: Some(chemin.to_path_buf()),
                    octets,
                };
                self.jouer(&prog).map_err(|e| e.to_string())?;
                Ok(trf!("{0} lancé", nom))
            }
        }
    }

    /// Les réglages de `chemin` viennent d'être enregistrés : ils comptent
    /// dès le prochain Charger ou ▶ Play.
    pub fn reglages_changes(&mut self, chemin: &Path) {
        if self.dernier_chemin.as_deref() == Some(chemin) {
            self.plan = None;
            self.dernier = None;
        }
        self.noter(trf!(
            "réglages de {0} enregistrés : ils comptent au prochain Charger ou ▶",
            chemin
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default()
        ));
    }

    /// Dernier fichier chargé ou lancé depuis la bibliothèque.
    pub fn dernier_chemin(&self) -> Option<&Path> {
        self.dernier_chemin.as_deref()
    }

    /// Son de la cassette : à appeler après chaque tranche d'émulation.
    fn son_cassette(&mut self) {
        let rapide = self.vitesse == Vitesse::Acceleree;
        self.son.avancer(
            &self.m.board,
            self.opts.son_cassette,
            rapide,
            self.opts.volume_cassette,
        );
    }

    /// Fenêtre « Oscilloscope » (quand la cassette avance).
    pub fn dessiner_oscilloscope(&mut self, ctx: &eframe::egui::Context) {
        if !self.opts.oscilloscope {
            return;
        }
        let r = cassette::ReglagesOscillo {
            son: &mut self.opts.son_cassette,
            volume: &mut self.opts.volume_cassette,
            rapide: self.vitesse == Vitesse::Acceleree,
            sans_carte: self.son.sans_carte(),
        };
        let (ouverte, change) = cassette::fenetre(ctx, &self.m.board, r);
        if !ouverte {
            self.opts.oscilloscope = false;
        }
        if change || !ouverte {
            self.enregistrer_options();
        }
    }

    /// Fenêtre « Projets GitHub » et suites du réseau.
    pub fn dessiner_github(
        &mut self,
        ctx: &eframe::egui::Context,
        f: &mut projets_github::FenetreGithub,
    ) {
        let biblio = self.bibliotheque();
        let mut projets = std::mem::take(&mut self.opts.github);
        let (change, mut suites) = f.dessiner(ctx, &biblio, &mut projets);
        self.opts.github = projets;
        if change {
            self.enregistrer_options();
        }
        suites.extend(f.evenements());
        for s in suites {
            match s {
                projets_github::Suite::Message(m, false) => self.noter(m),
                projets_github::Suite::Message(m, true) => {
                    let _ = self.erreur(m);
                }
                projets_github::Suite::Lancer(chemin, origine) => {
                    match self.lancer_fichier(&chemin) {
                        Ok(m) => self.noter(trf!("{0} — {1}", origine, m)),
                        Err(e) => {
                            let _ = self.erreur(trf!("{0} — {1}", origine, e));
                        }
                    }
                }
            }
        }
    }

    fn source(programme: &Programme) -> Option<Langage> {
        let ext = Path::new(&programme.nom)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        match Langage::depuis_extension(&ext) {
            l @ (Langage::Basic | Langage::Assembleur) => Some(l),
            _ => None,
        }
    }

    fn document(programme: &Programme, langage: Langage) -> DocumentEditeur {
        DocumentEditeur {
            titre: programme.nom.clone(),
            chemin: programme.chemin.clone(),
            langage,
            texte: String::from_utf8_lossy(&programme.octets).into_owned(),
        }
    }

    fn arret_debogueur(&mut self, s: Stop) {
        self.marche = Marche::Pause;
        let pc = self.m.cpu.pc;
        self.noter(match s {
            Stop::Break(a) => trf!("point d'arrêt en ${0}", format!("{:04X}", a)),
            Stop::BasicLine(l) | Stop::BasicBreak(l) => trf!("BASIC : ligne {0}", l),
            Stop::Jam => trf!("6502 bloqué (opcode JAM) en ${0}", format!("{:04X}", pc)),
            Stop::Pause | Stop::Step => trf!("arrêt en ${0}", format!("{:04X}", pc)),
        });
    }

    fn mener(&mut self, cycles: u64) {
        match self.dbg.run(&mut self.m, cycles) {
            Some(s) => self.arret_debogueur(s),
            None => {
                self.marche = if self.dbg.paused {
                    Marche::Pause
                } else {
                    Marche::Marche
                }
            }
        }
    }

    /// Démarre ou arrête l'enregistrement de la sortie ACI ; à l'arrêt, le
    /// signal est écrit dans `Cassettes/` de la bibliothèque (fichier WAV).
    fn basculer_enregistrement(&mut self) {
        if self.m.board.recording.is_none() {
            self.m.board.recording = Some(Vec::new());
            self.noter(tr(
                "enregistrement de la sortie ACI : tapez C100R puis début.finW",
            ));
            return;
        }
        let ech = self.cassette_sortie();
        self.m.board.recording = None;
        let dossier = self.bibliotheque().join("Cassettes");
        // rdk-apple1#35 : date lisible à l'heure locale, comme les états
        let fichier = dossier.join(format!(
            "sortie-aci-{}.wav",
            rdk_interface::app::horodatage_local()
        ));
        let octets = apple1_core::wav::encode_wav(
            &ech,
            FREQUENCE_CASSETTE,
            16,
            Some(tr("Retro Dev Kit Apple 1 2.0")),
        );
        match std::fs::create_dir_all(&dossier).and_then(|_| std::fs::write(&fichier, octets)) {
            Ok(()) => self.noter(trf!("cassette écrite : {0}", fichier.display())),
            Err(e) => {
                let _ = self.erreur(trf!("{0} : {1}", fichier.display(), e));
            }
        }
    }
}

/// 0.1.5 : pilote `rom-machine`. Woz Monitor, ACI et 2513 sont toujours
/// là ; le bouton commun choisit le BASIC rangé en `$E000` : « ROM réelle »
/// = votre Integer BASIC d'origine (reconnu par sa signature), « Émulation »
/// = RDK G2 Basic intégré (défaut de la v1.0).
impl RomMachine for MachineApple1 {
    fn rom_identifier(&mut self, _jeu: &JeuRom, roms: &[RomTrouvee]) -> VerdictRom {
        let Some(r) = roms.iter().find(|r| r.attendue.fichier == FICHIER_BASIC) else {
            return VerdictRom::non_reconnues(tr("aucun BASIC d'origine déclaré"));
        };
        if !r.presente() {
            let ou = self
                .bibliotheque()
                .join(rdk_pilotes::pilotes::rom_machine::DOSSIER_ROM);
            // une copie sous un autre nom (« apple1basic.bin »…) est
            // reconnue à sa signature : on dit comment la renommer
            // (rdk-apple1#17, point 3 ; rien n'est renommé sans l'utilisateur)
            if let Some(autre) = basic_sous_un_autre_nom(&ou) {
                return VerdictRom::non_reconnues(trf!(
                    "{0} est votre Integer BASIC d'origine, mais sous un autre nom : renommez-le en {1} (dans {2}), puis redémarrez le kit",
                    autre,
                    FICHIER_BASIC,
                    ou.display()
                ));
            }
            return VerdictRom::non_reconnues(trf!("Integer BASIC d'origine absent (non fourni) : copiez votre {0} dans {1} ; en attendant, RDK G2 Basic", FICHIER_BASIC, ou.display()));
        }
        match Self::reconnaitre_basic(&r.octets) {
            Ok(()) => VerdictRom::reconnues(tr("Integer BASIC d'origine reconnu")),
            Err(e) => VerdictRom::non_reconnues(trf!("{0} : {1}", FICHIER_BASIC, e)),
        }
    }
    fn rom_appliquer(&mut self, mode: ModeRom, roms: &[RomTrouvee]) -> Resultat<String> {
        let reel = roms
            .iter()
            .find(|r| r.attendue.fichier == FICHIER_BASIC && r.presente())
            .map(|r| r.octets.clone())
            .filter(|o| Self::reconnaitre_basic(o).is_ok());
        let (mode, msg) = match (mode, reel) {
            (ModeRom::RomReelle, Some(o)) => {
                self.basic_reel = Some(o);
                self.opts.basic_g2 = false;
                (
                    ModeRom::RomReelle,
                    tr("ROM réelle : Integer BASIC d'origine en $E000"),
                )
            }
            _ => {
                self.opts.basic_g2 = true;
                (ModeRom::Emulation, tr("Émulation : RDK G2 Basic en $E000"))
            }
        };
        self.mode_rom = mode;
        // redémarrage propre : le BASIC choisi remplace celui qui était en mémoire
        if self.basic_present() {
            let _ = self.assurer_basic_neuf();
        }
        self.machine_reset();
        self.erreurs.clear();
        self.noter(msg);
        Ok(msg.to_string())
    }
}

trait Facultative {
    fn facultative(self) -> Self;
}

impl Facultative for RomAttendue {
    /// Le BASIC d'origine est facultatif : sans lui, RDK G2 Basic (Émulation).
    fn facultative(mut self) -> Self {
        self.obligatoire = false;
        self
    }
}

impl MachineApple1 {
    /// Range le BASIC du mode choisi en `$E000`, même si un autre y est.
    fn assurer_basic_neuf(&mut self) -> Resultat<()> {
        if self.opts.basic_g2 {
            if self.g2_present() {
                return Ok(());
            }
            let p = g2basic::program();
            return self.ranger(&p.blocks).map(|_| ());
        }
        match self.basic_reel.clone() {
            Some(o) => self.ranger_basic(&o).map_err(|e| self.erreur(e)),
            None => self.assurer_basic(),
        }
    }

    /// Image Intel HEX d'une plage de la mémoire dans `PROM/` de la
    /// bibliothèque (programmateur sans fenêtre).
    pub fn exporter_prom(&mut self, debut: u16, fin: u16) -> Result<String, String> {
        let dossier = self.bibliotheque().join(prom::DOSSIER);
        let fichier = dossier.join(format!("prom-{debut:04X}-{fin:04X}.hex"));
        let octets = self.m.dump(debut, fin);
        prom::ecrire_fichier(&fichier, debut as u32, &octets)?;
        Ok(trf!(
            "image PROM ${0}-${1} écrite en Intel HEX : {2} (programmateur d'EPROM : minipro)",
            format!("{:04X}", debut),
            format!("{:04X}", fin),
            fichier.display()
        ))
    }

    /// Lecture sans effet de bord de la mémoire (fenêtres).
    pub fn lire(&self, a: u16) -> u8 {
        self.m.board.peek(a)
    }

    /// Sauve des plages de la mémoire (v1.0 : bouton 💾 Sauver). Le format
    /// suit l'extension : `.woz` (dump du Woz Monitor), `.wav` / `.aiff`
    /// (cassette ACI), `.bin`, `.prg`, `.hex` (Intel HEX) ou `.dump`.
    pub fn sauver(&mut self, chemin: &Path, plages: &str) -> Result<String, String> {
        let liste = program::parse_ranges(plages)?;
        if liste.is_empty() {
            return Err(tr("indiquez au moins une plage (ex. 0300.03FF)").into());
        }
        let mut p = Program::default();
        for (a, b) in liste {
            p.blocks.push((a, self.m.dump(a, b)));
        }
        if let Some(d) = chemin.parent() {
            std::fs::create_dir_all(d).map_err(|e| trf!("{0} : {1}", d.display(), e))?;
        }
        let ext = chemin
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        let blocs: Vec<rdk_pilotes::BlocMemoire> = p
            .blocks
            .iter()
            .map(|(a, o)| rdk_pilotes::BlocMemoire {
                adresse: *a as u32,
                octets: o.clone(),
            })
            .collect();
        let ecrire = |t: String| {
            std::fs::write(chemin, t).map_err(|e| trf!("{0} : {1}", chemin.display(), e))
        };
        match ext.as_str() {
            "hex" => ecrire(formats::ecrire_intel_hex(&blocs))?,
            "dump" => ecrire(formats::ecrire_dump(&blocs))?,
            "txt" | "mon" | "apl" => {
                return Err(trf!(
                ".{0} est lu par compatibilité, plus écrit : choisissez .woz (dump du Woz Monitor)",
                ext
            ))
            }
            "woz" | "wav" | "aif" | "aiff" | "bin" | "prg" => {
                let o = apple1_core::aciwav::ExportOptions {
                    timing: self.m.board.config.timing,
                    ..Default::default()
                };
                program::save_file(chemin, &p, &o)?;
            }
            _ => {
                return Err(format!(
                    "extension inconnue « .{ext} » : .woz, .wav, .aiff, .bin, .prg, .hex ou .dump"
                ))
            }
        }
        let k = program::kind_of(chemin, None);
        Ok(if k == program::Kind::Wav {
            trf!(
                "cassette écrite : {0} (pour la relire : C100R puis {1})",
                chemin.display(),
                p.aci_read_command()
            )
        } else {
            trf!("{0} octets écrits : {1}", p.total_len(), chemin.display())
        })
    }

    /// Lance un listing (BASIC) ou un source 6502 sans passer par l'éditeur
    /// (outil Photo → ASCII).
    pub fn lancer_source(&mut self, langage: Langage, texte: &str) -> Resultat<String> {
        self.editeur_lancer(&DocumentEditeur {
            titre: tr("Photo → ASCII").into(),
            chemin: None,
            langage,
            texte: texte.into(),
        })
    }

    /// Message de la barre d'état (fenêtres).
    pub fn informer(&mut self, texte: impl Into<String>, erreur: bool) {
        if erreur {
            let _ = self.erreur(texte);
        } else {
            self.erreurs.clear();
            self.noter(texte);
        }
    }

    /// Phosphore choisi (fenêtres : aperçu de l'art ASCII).
    pub fn phosphore(&self) -> crt::Phosphore {
        self.opts.phosphore
    }
}

// ----- pilotes vides : l'Apple 1 n'a ni souris, ni joystick, ni imprimante,
// ni ports, ni signal analogique émulé.
impl SourisMachine for MachineApple1 {}
impl JoystickMachine for MachineApple1 {}
impl MachineImprimante for MachineApple1 {}
impl PortParallele for MachineApple1 {}
impl PortSerie for MachineApple1 {}
impl VideoAnalogique for MachineApple1 {}

/// v1.0 : deux écrans, le terminal de l'Apple 1 (moniteur monochrome) et
/// l'écran de la carte GEN2 ou P-LAB (moniteur couleur). L'interface commune
/// montre les deux moniteurs ; sans carte branchée, le moniteur couleur n'a
/// pas d'image (« non branché »).
impl VideoCouleur for MachineApple1 {
    fn couleur_disponible(&self) -> bool {
        true
    }
    fn palette(&self) -> Vec<CouleurPalette> {
        couleur::palette()
    }
    fn brancher_moniteur(&mut self, moniteur: Moniteur) {
        self.moniteur = moniteur;
    }
    fn moniteur_branche(&self) -> Moniteur {
        self.moniteur
    }
    /// 0.1.6 : phosphore vert (défaut de la v1.0), blanc ou ambre.
    fn phosphore_monochrome(&self) -> Option<rdk_pilotes::Phosphore> {
        Some(self.opts.phosphore.couleurs())
    }
}

impl VideoNumerique for MachineApple1 {
    fn image(&mut self, moniteur: Moniteur) -> Option<ImageVideo> {
        match moniteur {
            Moniteur::Monochrome => {
                let i = ecran::image(&self.m);
                Some(
                    self.tube_mono
                        .image(i, &self.opts.ecran, self.opts.phosphore.tau(), false),
                )
            }
            Moniteur::Couleur => {
                let i = match self.source_couleur()? {
                    SourceCouleur::Gen2 => {
                        couleur::image_gen2(&self.m, self.opts.accolades_grasses)
                    }
                    SourceCouleur::Plab => couleur::image_plab(&self.m),
                };
                Some(
                    self.tube_couleur
                        .image(i, &self.opts.ecran, crt::TAU_COULEUR, true),
                )
            }
        }
    }
    /// Contrat 1.1.0 : cases « vintage » dans les fenêtres d'écran.
    fn reglages_ecran(&self, moniteur: Moniteur) -> Vec<rdk_pilotes::ReglageEcran> {
        contrat_1_1::reglages(moniteur, &self.opts.ecran, self.opts.phosphore)
    }
    fn regler_ecran(&mut self, _moniteur: Moniteur, id: &str, valeur: &rdk_pilotes::ValeurReglage) {
        if contrat_1_1::regler(&mut self.opts.ecran, &mut self.opts.phosphore, id, valeur) {
            self.enregistrer_reglages();
        }
    }
}

impl ClavierMachine for MachineApple1 {
    fn touche_machine(&mut self, touche: &ToucheHote, appuyee: bool) {
        if !appuyee {
            return;
        }
        if touche.caractere.is_none() {
            match touche.nom.as_str() {
                clavier::MAJ => {
                    self.maj = !self.maj;
                    return;
                }
                clavier::CTRL => {
                    self.ctrl = !self.ctrl;
                    return;
                }
                _ => {}
            }
        }
        // v1.0 : Ctrl+V tape le presse-papiers au clavier de l'Apple 1
        if touche.ctrl
            && touche
                .caractere
                .map(|c| c.eq_ignore_ascii_case(&'v'))
                .unwrap_or(touche.nom.eq_ignore_ascii_case("v"))
        {
            if let Some(texte) = self.contexte.get().and_then(|ctx| {
                ctx.input(|i| {
                    i.events.iter().find_map(|e| {
                        if let eframe::egui::Event::Paste(t) = e {
                            Some(t.clone())
                        } else {
                            None
                        }
                    })
                })
            }) {
                let n = texte.chars().count();
                self.m.type_text(
                    &texte
                        .replace("\r\n", "\r")
                        .replace('\n', "\r")
                        .to_ascii_uppercase(),
                );
                self.noter(trf!("presse-papiers tapé au clavier ({0} caractères)", n));
                return;
            }
        }
        let code = clavier::code_apple1(touche, self.maj, self.ctrl);
        self.maj = false;
        self.ctrl = false;
        if let Some(c) = code {
            self.m.press_key(c);
        }
    }
}

impl DumpMemoire for MachineApple1 {
    fn dump_plage(&self) -> RangeInclusive<Adresse> {
        0..=0xFFFF
    }
    /// v1.0 : raccourcis ZP, $0200 (tampon du clavier), $0300 (programmes),
    /// $E000 (BASIC) ; le bouton PC est ajouté par l'interface commune.
    fn dump_signets(&self) -> Vec<(String, Adresse)> {
        vec![
            ("ZP".to_string(), 0x0000),
            ("$0200".to_string(), 0x0200),
            ("$0300".to_string(), 0x0300),
            ("$E000".to_string(), 0xE000),
        ]
    }
    fn dump_lire(&self, adresse: Adresse) -> Option<u8> {
        (adresse <= 0xFFFF).then(|| self.m.board.peek(adresse as u16))
    }
    fn dump_ecrire(&mut self, adresse: Adresse, valeur: u8) -> Resultat<()> {
        if adresse > 0xFFFF {
            return Err(ErreurPilote::AdresseInvalide(adresse));
        }
        if self.m.board.poke(adresse as u16, valeur) {
            Ok(())
        } else {
            Err(ErreurPilote::LectureSeule(adresse))
        }
    }
}

impl DebogueurMemoire for MachineApple1 {
    fn debogueur_lire(&self, adresse: Adresse) -> Option<u8> {
        self.dump_lire(adresse)
    }
    fn debogueur_ecrire(&mut self, adresse: Adresse, valeur: u8) -> Resultat<()> {
        self.dump_ecrire(adresse, valeur)
    }
    fn debogueur_variables(&self) -> Vec<Variable> {
        if !self.basic_present() {
            return Vec::new();
        }
        debug::basic_vars(&|a| self.m.board.peek(a))
            .into_iter()
            .map(|v| {
                let valeur = match &v.value {
                    VarValue::Str(s, _) => format!("\"{s}\""),
                    VarValue::Int(l) if l.len() == 1 => l[0].to_string(),
                    VarValue::Int(l) => format!(
                        "({})",
                        l.iter()
                            .take(16)
                            .map(|x| x.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                };
                Variable {
                    nom: v.name,
                    valeur,
                }
            })
            .collect()
    }
}

impl DebogueurProcesseur for MachineApple1 {
    fn processeur(&self) -> Processeur {
        Processeur::M6502
    }
    fn registres(&self) -> Vec<Registre> {
        let c = &self.m.cpu;
        let valeurs = [
            c.a as u32,
            c.x as u32,
            c.y as u32,
            c.sp as u32,
            c.pc as u32,
            c.p as u32,
        ];
        REGISTRES_6502
            .iter()
            .zip(valeurs)
            .map(|(n, v)| Registre {
                nom: n.to_string(),
                valeur: v,
                bits: if *n == "PC" { 16 } else { 8 },
            })
            .collect()
    }
    fn ecrire_registre(&mut self, nom: &str, valeur: u32) -> Resultat<()> {
        let c = &mut self.m.cpu;
        match nom {
            "A" => c.a = valeur as u8,
            "X" => c.x = valeur as u8,
            "Y" => c.y = valeur as u8,
            "S" => c.sp = valeur as u8,
            "PC" => c.pc = valeur as u16,
            "P" => c.p = valeur as u8 | apple1_core::cpu::FLAG_U,
            _ => {
                return Err(ErreurPilote::Message(trf!(
                    "registre inconnu du 6502 : {0}",
                    nom
                )))
            }
        }
        Ok(())
    }
    fn commande(&mut self, commande: CommandeExecution) -> Resultat<()> {
        let budget = self.m.board.config.timing.secs_to_cycles(2.0);
        match commande {
            CommandeExecution::Pas => {
                self.dbg.go(Mission::Into);
                self.mener(1_000);
            }
            CommandeExecution::ParDessus => {
                self.dbg.over(&self.m);
                self.mener(budget);
            }
            CommandeExecution::Sortir => {
                self.dbg.out(&self.m);
                self.mener(budget);
            }
            CommandeExecution::Continuer => {
                self.dbg.cont();
                self.marche = Marche::Marche;
            }
            CommandeExecution::Pause => {
                self.dbg.pause();
                self.marche = Marche::Pause;
            }
            CommandeExecution::ExecuterJusqua(a) => {
                self.dbg.go(Mission::To(a as u16));
                self.mener(budget);
            }
        }
        Ok(())
    }
    /// Contrat 1.1.0 : marge de l'Éditeur et rubrique du Débogueur.
    fn points_arret(&mut self, points: &[rdk_pilotes::PointArret]) -> Resultat<()> {
        self.arrets = contrat_1_1::Arrets::depuis(points);
        self.appliquer_arrets();
        Ok(())
    }
}

impl DesassembleurMemoire for MachineApple1 {
    fn desassembleur_lire(&self, adresse: Adresse) -> Option<u8> {
        self.dump_lire(adresse)
    }
    /// v1.0 : « Aller » accepte une étiquette (Woz Monitor, ACI, programme
    /// assemblé), sans tenir compte de la casse.
    fn desassembleur_symbole(&self, nom: &str) -> Option<Adresse> {
        let nom = nom.trim().trim_end_matches(':');
        self.symboles
            .iter()
            .find(|(_, n)| n.eq_ignore_ascii_case(nom))
            .map(|(a, _)| *a as Adresse)
    }
    fn desassembler(&self, adresse: Adresse) -> Option<LigneDesassemblee> {
        if adresse > 0xFFFF {
            return None;
        }
        let l = debug::disasm(&|a| self.m.board.peek(a), adresse as u16, &self.symboles);
        let texte = match self.symboles.get(&l.addr) {
            Some(n) => trf!("{0}: {1}", n, l.text),
            None => l.text,
        };
        Some(LigneDesassemblee {
            adresse: l.addr as Adresse,
            octets: l.bytes,
            texte,
        })
    }
    fn desassembleur_depart(&self) -> Adresse {
        self.depart as Adresse
    }
}

impl EditeurMachine for MachineApple1 {
    fn editeur_lancer(&mut self, document: &DocumentEditeur) -> Resultat<String> {
        match document.langage {
            Langage::Basic => self.taper_basic(&document.texte, true),
            Langage::Assembleur => {
                let a = self.assembler(document)?;
                self.reset_et_taper("");
                self.ranger(&[(a.origin, a.bytes.clone())])?;
                self.m.type_text(&format!("{}\r", a.run_command()));
                self.erreurs.clear();
                Ok(trf!(
                    "{0} octets assemblés en ${1}, {2}",
                    a.bytes.len(),
                    format!("{:04X}", a.origin),
                    a.run_command()
                ))
            }
            _ => Err(self.erreur(tr(
                "l'Apple 1 lance du BASIC (.bas) ou de l'assembleur 6502 (.asm)",
            ))),
        }
    }
    fn editeur_pause(&mut self) -> Resultat<()> {
        self.machine_marche(if self.marche == Marche::Pause {
            Marche::Marche
        } else {
            Marche::Pause
        });
        Ok(())
    }
    fn editeur_arreter(&mut self) -> Resultat<()> {
        // l'Apple 1 n'a pas de BREAK : RESET ramène au Woz Monitor
        self.machine_reset();
        Ok(())
    }
    /// Contrat 1.1.0 (issue 24 de l'interface) : routines de la ROM.
    fn editeur_routines_rom(&self) -> Vec<rdk_pilotes::RoutineRom> {
        contrat_1_1::routines_rom()
    }
}

impl EditeurMemoire for MachineApple1 {
    fn injection_possible(&self, langage: &Langage) -> bool {
        matches!(langage, Langage::Basic | Langage::Assembleur)
    }
    fn injecter(&mut self, document: &DocumentEditeur) -> Resultat<String> {
        match document.langage {
            Langage::Basic => self.taper_basic(&document.texte, false),
            Langage::Assembleur => {
                let a = self.assembler(document)?;
                self.ranger(&[(a.origin, a.bytes.clone())])?;
                self.erreurs.clear();
                Ok(trf!(
                    "{0} octets rangés en ${1} ; tapez {2}",
                    a.bytes.len(),
                    format!("{:04X}", a.origin),
                    a.run_command()
                ))
            }
            _ => Err(self.erreur(tr("seuls le BASIC et l'assembleur 6502 vont en mémoire"))),
        }
    }
}

/// Un fichier de 4 Ko du dossier ROM, autre que `integer_basic.bin`, qui
/// porte la signature de l'Integer BASIC d'origine : son nom.
pub fn basic_sous_un_autre_nom(dossier: &Path) -> Option<String> {
    std::fs::read_dir(dossier)
        .ok()?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.file_name().is_some_and(|n| n != FICHIER_BASIC))
        .filter(|p| std::fs::metadata(p).is_ok_and(|m| m.is_file() && m.len() == 4096))
        .find(|p| std::fs::read(p).is_ok_and(|o| MachineApple1::reconnaitre_basic(&o).is_ok()))
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
}

/// Ligne de description d'un programme de la bibliothèque (v1.0,
/// `library::describe`) : « Source BASIC · 47 lignes · lancer : RUN »,
/// « Woz Monitor (texte hexa) · 0300.031E · lancer : 300R »…
pub fn description_programme(programme: &Programme) -> Option<String> {
    let chemin = programme
        .chemin
        .clone()
        .unwrap_or_else(|| PathBuf::from(&programme.nom));
    let ext = chemin
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let texte = || String::from_utf8_lossy(&programme.octets).into_owned();
    match Langage::depuis_extension(&ext) {
        Langage::Basic => {
            let n = texte().lines().filter(|l| !l.trim().is_empty()).count();
            return Some(trf!("Source BASIC · {0} lignes · lancer : RUN", n));
        }
        Langage::Assembleur => {
            return Some(match asm6502::assemble_text(&texte(), Some(&chemin), &[]) {
                Ok(a) if !a.bytes.is_empty() => trf!(
                    "Source assembleur 6502 · {0}.{1} · lancer : {2}",
                    format!("{:04X}", a.origin),
                    format!("{:04X}", a.end()),
                    a.run_command()
                ),
                Ok(_) => tr("Source assembleur 6502 · vide").to_string(),
                Err(e) => trf!("Source assembleur 6502 · {0} erreur(s)", e.len()),
            });
        }
        _ => {}
    }
    let kind = program::kind_of(&chemin, Some(&programme.octets));
    if kind == program::Kind::Wav {
        let duree = apple1_core::wav::parse_audio(&programme.octets)
            .ok()
            .map(|a| a.best_channel().len() as f64 / a.sample_rate.max(1) as f64);
        return Some(match duree {
            Some(d) => trf!(
                "Cassette ACI · {0} s · ▶ la lit et la lance",
                format!("{d:.1}")
            ),
            None => tr("Cassette ACI").to_string(),
        });
    }
    let nature = match kind {
        program::Kind::Hex => tr("Woz Monitor (texte hexa)"),
        program::Kind::Prg => tr("Programme .prg"),
        program::Kind::Bin => return Some(trf!("Binaire · {0} octets", programme.octets.len())),
        _ => tr("Programme"),
    };
    match program::load_bytes(&programme.octets, kind, None) {
        Ok(p) => {
            let plages: Vec<String> = p
                .ranges()
                .iter()
                .map(|(a, b)| format!("{a:04X}.{b:04X}"))
                .collect();
            let lancer = p
                .run
                .or_else(|| p.blocks.first().map(|b| b.0))
                .map(|a| trf!(" · lancer : {0}R", format!("{a:X}")))
                .unwrap_or_default();
            Some(format!("{nature} · {}{lancer}", plages.join(" ")))
        }
        Err(e) => Some(format!("{nature} · {e}")),
    }
}

impl BiblioMemoire for MachineApple1 {
    /// v1.0 (`library::describe`) : nature du fichier, lignes ou plages
    /// d'adresses, commande de lancement. Les cassettes ne sont pas décodées
    /// ici (trop long pour l'affichage) : leur durée seulement.
    fn biblio_description(&self, programme: &Programme) -> Option<String> {
        description_programme(programme)
    }
    fn charger(&mut self, programme: &Programme) -> Resultat<()> {
        self.dernier_chemin = programme.chemin.clone().or(self.dernier_chemin.take());
        if let Some(l) = Self::source(programme) {
            return self
                .injecter(&Self::document(programme, l))
                .map(|m| self.noter(m));
        }
        let p = self.programme_de(programme)?;
        if self.opts.cassette {
            return self.charger_par_cassette(programme, p, None);
        }
        let n = self.ranger(&p.blocks)?;
        if let Some((a, _)) = p.blocks.first() {
            if !Self::est_basic(&p) {
                self.depart = *a;
            }
        }
        let notes: String = p.notes.iter().map(|n| trf!(" ; {0}", n)).collect();
        self.noter(format!(
            "{} : {n} octets chargés ({}){notes}",
            programme.nom,
            p.ranges()
                .iter()
                .map(|(a, b)| format!("${a:04X}-${b:04X}"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        self.erreurs.clear();
        self.dernier = Some(p);
        Ok(())
    }
    fn jouer(&mut self, programme: &Programme) -> Resultat<()> {
        self.dernier_chemin = programme.chemin.clone().or(self.dernier_chemin.take());
        if let Some(l) = Self::source(programme) {
            return self
                .editeur_lancer(&Self::document(programme, l))
                .map(|m| self.noter(m));
        }
        let p = self.programme_de(programme)?;
        // BASIC d'abord (avant le RESET), puis les touches de lancement
        let lancer = self.touches_lancement(&p)?;
        if self.opts.cassette {
            return self.charger_par_cassette(programme, p, Some(lancer));
        }
        self.reset_et_taper("");
        self.ranger(&p.blocks)?;
        self.dernier = Some(p);
        self.executer(programme)
    }
    fn executer(&mut self, programme: &Programme) -> Resultat<()> {
        let Some(p) = self.dernier.clone() else {
            return Err(self.erreur(tr("aucun programme chargé")));
        };
        let touches = self.touches_lancement(&p)?;
        self.m.type_text(&touches);
        self.noter(format!(
            "{} : {}",
            programme.nom,
            touches.replace('\r', " ").trim()
        ));
        Ok(())
    }
    fn double_clic_charge(&self, programme: &Programme) -> bool {
        let ext = Path::new(&programme.nom)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        let chemin = programme
            .chemin
            .clone()
            .unwrap_or_else(|| PathBuf::from(&programme.nom));
        // .txt : vidage du Woz Monitor ou dessin ASCII (sinon ouvert dans l'éditeur)
        let txt =
            ext == "txt" && (Self::est_vidage_woz(programme) || Self::est_art(programme, &chemin));
        txt || matches!(
            ext.as_str(),
            "wav"
                | "wave"
                | "aif"
                | "aiff"
                | "bin"
                | "prg"
                | "mon"
                | "woz"
                | "hex"
                | "dump"
                | "apl"
                | "png"
                | "jpg"
                | "jpeg"
        )
    }
}

impl CassetteMachine for MachineApple1 {
    fn cassette_entree(&mut self, echantillons: &[f32]) {
        self.m.board.player = Some(TapePlayer::from_samples(
            echantillons,
            FREQUENCE_CASSETTE,
            "cassette",
        ));
        self.noter(tr("cassette insérée : tapez C100R puis début.finR"));
    }
    fn cassette_sortie(&mut self) -> Vec<f32> {
        let Some(r) = self.m.board.recording.as_mut() else {
            return Vec::new();
        };
        let cycles = std::mem::take(r);
        if cycles.is_empty() {
            return Vec::new();
        }
        let t = self.m.board.config.timing;
        let bascules: Vec<f64> = cycles.iter().map(|c| t.cycles_to_secs(*c)).collect();
        tape::render_toggles(
            &bascules,
            &tape::RenderOptions {
                sample_rate: FREQUENCE_CASSETTE,
                ..tape::RenderOptions::default()
            },
        )
    }
}

impl MachineEtatPilote for MachineApple1 {
    fn etat(&self) -> EtatMachine {
        let b = &self.m.board;
        let mut actifs = vec!["ACI".to_string()];
        if b.config.color_card {
            actifs.push("GEN2".to_string());
        }
        if b.config.plab_card {
            actifs.push("P-LAB".to_string());
        }
        if b.recording.is_some() {
            actifs.push(tr("⏺ enregistrement ACI").to_string());
        }
        let ko = (0..0xE0u16).filter(|p| b.is_ram(p << 8)).count() / 4;
        let basic = if self.g2_present() {
            tr("RDK G2 Basic").to_string()
        } else if self.basic_present() {
            tr("Integer BASIC").to_string()
        } else if self.opts.basic_g2 {
            tr("RDK G2 Basic (émulation, chargé au lancement)").to_string()
        } else {
            tr("Integer BASIC d'origine (ROM réelle, chargé au lancement)").to_string()
        };
        let mut complements = vec![
            ("PC".to_string(), format!("${:04X}", self.m.cpu.pc)),
            ("RAM".to_string(), trf!("{0} Ko + $E000-$FEFF", ko)),
            ("BASIC".to_string(), basic),
            (
                tr("Moniteur").to_string(),
                if self.moniteur == Moniteur::Couleur {
                    tr("couleur")
                } else {
                    tr("monochrome")
                }
                .to_string(),
            ),
        ];
        if b.config.color_card {
            complements.push((
                "GEN2".to_string(),
                trf!(
                    "{0} ({1} Hz)",
                    b.gen2.mode_name(),
                    if b.config.color_50hz { 50 } else { 60 }
                ),
            ));
        }
        if b.config.plab_card {
            let v = &b.vdp;
            let mut t = if v.display_on() {
                v.mode().name().to_string()
            } else {
                tr("écran éteint").to_string()
            };
            if b.config.plab_nmi {
                t.push_str(", NMI");
            }
            if let Some(ct) = &b.config.codetank {
                t.push_str(&trf!(", CodeTank {0} Ko", ct.len() / 1024));
            }
            complements.push(("P-LAB".to_string(), t));
        }
        if let Some(p) = &self.m.board.player {
            let now = self.m.board.now_secs();
            complements.push((
                tr("Cassette").to_string(),
                trf!(
                    "{0} {1}/{2} s",
                    p.name,
                    format!("{:.1}", p.position(now)),
                    format!("{:.1}", p.duration())
                ),
            ));
        }
        if let Some(c) = &self.chargement {
            complements.push(("📼".to_string(), trf!("lecture de {0}", c.nom)));
        }
        if self.opts.ecran.actif {
            complements.push((
                tr("Écran").to_string(),
                trf!(
                    "vintage, phosphore {0}",
                    self.opts.phosphore.nom().to_lowercase()
                ),
            ));
        }
        if let Some(m) = &self.message {
            complements.push((tr("Message").to_string(), m.clone()));
        }
        EtatMachine {
            marche: self.marche,
            vitesse: self.vitesse,
            erreurs: self.erreurs.clone(),
            peripheriques_actifs: actifs,
            complements,
        }
    }
}

impl Machine for MachineApple1 {
    fn machine_nom(&self) -> String {
        DOSSIER_BIBLIOTHEQUE.to_string()
    }
    /// Identifiant **stable**, déclaré (feuille de route, contrat § 5 règle
    /// 3) : il est enregistré dans les configurations des utilisateurs.
    fn machine_id(&self) -> String {
        "apple-1".to_string()
    }
    /// Titre de la 2.0 (rdk-apple1#26) : « Original Sin » est le nom de la
    /// v1.0, que l'utilisateur ne doit pas croire avoir lancée.
    fn machine_titre_fenetre(&self) -> String {
        format!("Retro Dev Kit Apple 1 {VERSION}")
    }
    fn machine_version(&self) -> String {
        VERSION.to_string()
    }
    fn machine_a_propos(&self) -> Vec<String> {
        vec![
            trf!("Retro Dev Kit Apple 1 {0} (Linux), sur RDK-Interface-Commune {1}.", VERSION, rdk_interface::VERSION),
            tr("Licence GPL-3.0. Machine reprise de la v1.0 « Original Sin » (crate apple1-core).").to_string(),
            tr("ROM fournies : Woz Monitor, ACI (Steve Wozniak, 1976) et générateur de caractères 2513, comme en v1.0.").to_string(),
            tr("BASIC : RDK G2 Basic intégré (clone « salle blanche » du kit, licence MIT), mode « Émulation », comme en v1.0. Le BASIC d'origine n'est pas fourni : posez votre copie integer_basic.bin dans RDK librairies/Apple 1/ROM/ pour le mode « ROM réelle ».").to_string(),
            tr("Cartes de la v1.0 : couleur GEN2 (Uncle Bernie) et graphique P-LAB (TMS9918A, CodeTank).").to_string(),
        ]
    }
    fn machine_reset(&mut self) {
        self.m.reset();
        self.dbg.cont();
        self.marche = Marche::Marche;
    }
    fn machine_avancer(&mut self, duree: Duration) {
        // fenêtres de la machine (programmateur d'EPROM, Photo → ASCII…)
        if let Some(ctx) = self.contexte.get().cloned() {
            let mut f = std::mem::take(&mut self.fenetres);
            f.dessiner(&ctx, self);
            self.fenetres = f;
            if self.tube_mono.actif || self.tube_couleur.actif || self.opts.ecran.scintillement {
                ctx.request_repaint();
            }
        }
        if self.chargement.is_some() {
            self.suivre_chargement();
        }
        if self.marche != Marche::Marche {
            return;
        }
        let mut cycles = self
            .m
            .board
            .config
            .timing
            .secs_to_cycles(duree.as_secs_f64().min(0.1));
        if self.vitesse == Vitesse::Acceleree {
            cycles *= ACCELERATION;
        }
        self.mener(cycles);
        self.son_cassette();
    }
    fn machine_marche(&mut self, marche: Marche) {
        match marche {
            Marche::Marche => self.dbg.cont(),
            Marche::Pause | Marche::Arret => self.dbg.pause(),
        }
        self.marche = marche;
    }
    fn machine_vitesse(&mut self, vitesse: Vitesse) {
        self.vitesse = vitesse;
        // vitesse réelle : le terminal affiche ~60 caractères par seconde
        self.m.board.config.display_throttle = vitesse == Vitesse::Reelle;
    }
    fn machine_effacer_ecran(&mut self) {
        self.m.clear_screen();
    }
    fn manifeste_boutons_peripheriques(&self) -> Vec<BoutonPeripherique> {
        let bouton = |id: &str, icone: &str, libelle: &str, info_bulle: &str, actif: bool| {
            BoutonPeripherique {
                id: id.to_string(),
                icone: icone.to_string(),
                libelle: libelle.to_string(),
                info_bulle: info_bulle.to_string(),
                actif,
            }
        };
        let o = &self.opts;
        let mut v = vec![bouton(BOUTON_ENREGISTRER, "⏺", tr("Sortie ACI"), tr("Enregistre la sortie cassette de l'ACI (C100R puis début.finW) ; au second clic, le fichier WAV est écrit dans le dossier Cassettes de la bibliothèque."), self.m.board.recording.is_some())];
        v.push(bouton(BOUTON_CHARGEMENT_CASSETTE, "📼", tr("Temps réel"), tr("Allumé : Charger et ▶ de la bibliothèque passent par la cassette, comme sur la vraie machine (RESET, C100R, lecture de l'ACI en temps réel). Éteint : chargement ⚡ rapide, directement en mémoire."), o.cassette));
        v.push(bouton(BOUTON_SAUVER, "💾", tr("Sauver"), tr("Sauver une ou plusieurs plages de la mémoire : .woz (dump du Woz Monitor), cassette .wav / .aiff, .bin, .prg, Intel .hex ou .dump."), self.fenetres.sauver.ouvert));
        v.push(bouton(BOUTON_GEN2, "🌈", "GEN2", tr("Carte couleur GEN2 d'Uncle Bernie : LORES, HIRES, TEXTE et MIXED (commutateurs $C250-$C257), RAM $0000-$BFFF ; son image va au moniteur couleur."), o.gen2));
        if o.gen2 {
            v.push(bouton(
                BOUTON_GEN2_50HZ,
                "〰",
                tr("50 Hz"),
                tr("GEN2 : balayage 50 Hz (PAL) au lieu de 60 Hz (NTSC)."),
                o.gen2_50hz,
            ));
            v.push(bouton(BOUTON_ACCOLADES, "{}", tr("Accolades grasses"), tr("GEN2 : dessin des accolades du générateur de caractères (grasses par défaut, ou fines)."), o.accolades_grasses));
        }
        v.push(bouton(BOUTON_PLAB, "🎮", "P-LAB", tr("Carte graphique P-LAB : TMS9918A en $CC00-$CC01 (Graphics I et II, Multicolor, Text, sprites) ; son image va au moniteur couleur."), o.plab));
        if o.plab {
            v.push(bouton(
                BOUTON_PLAB_NMI,
                "⚡",
                "NMI",
                tr("P-LAB : cavalier NMI (interruption du VDP à chaque image)."),
                o.plab_nmi,
            ));
            v.push(bouton(BOUTON_CODETANK, "💾", tr("CodeTank"), tr("P-LAB : EEPROM CodeTank en $4000-$7FFF, lue dans le dossier P-LAB de la bibliothèque (votre image .bin ou .rom, 16 Ko au plus) ; second clic : la retirer."), self.m.board.config.codetank.is_some()));
        }
        if o.gen2 && o.plab {
            v.push(bouton(BOUTON_SOURCE, "🖵", tr("Écran P-LAB"), tr("Les deux cartes sont branchées : le moniteur couleur montre la P-LAB (allumé) ou la GEN2 (éteint)."), o.source == SourceCouleur::Plab));
        }
        v.push(bouton(BOUTON_ECRAN_VINTAGE, "📺", tr("Vintage"), tr("Écran vintage : imitation d'un moniteur à tube des années 1970 : lignes de balayage, rémanence, halo, tube bombé."), o.ecran.actif));
        v.push(bouton(
            BOUTON_REGLAGES_ECRAN,
            "🔆",
            tr("Écran"),
            tr("Phosphore du moniteur (vert, blanc, ambre) et réglages de l'écran vintage."),
            self.fenetres.ecran,
        ));
        v.push(bouton(BOUTON_ASCII, "🎨", "ASCII", tr("Photo → ASCII : une photo (jpg, png) devient un art ASCII de 40 × 24 caractères, puis un programme BASIC ou 6502, une cassette ou un .txt du dossier Art ASCII."), self.fenetres.ascii.ouvert));
        v.push(bouton(BOUTON_SON, "🔊", tr("Son"), tr("Son de la cassette pendant la lecture 📼 ou l'écriture ⏺ (vitesse réelle), par la carte son (ALSA), comme en v1.0."), o.son_cassette));
        v.push(bouton(BOUTON_OSCILLOSCOPE, "📈", tr("Oscilloscope"), tr("Oscilloscope de la cassette : le signal sous la tête, pendant la lecture ou l'écriture."), o.oscilloscope));
        v.push(bouton(BOUTON_REGLAGES_PROGRAMME, "⚙", tr("Réglages"), tr("Réglages par programme (v1.0) : plages, adresses, commande de lancement et BASIC du dernier programme de la bibliothèque, ou d'un autre ; rangés dans programs.json."), self.fenetres.reglages.ouvert));
        v.push(bouton(BOUTON_GITHUB, "🐙", "GitHub", tr("Projets GitHub : téléchargés dans le dossier GitHub de la bibliothèque, mis à jour, et « ▶ Tester le dernier build »."), self.fenetres.github.ouvert));
        v.push(bouton(BOUTON_EXEMPLES, "📥", tr("Exemples"), tr("Copie les exemples du kit (BASIC, assembleur, GEN2, P-LAB, art ASCII, cassette) dans votre bibliothèque, sans rien remplacer."), false));
        v
    }
    fn manifeste_basculer_peripherique(&mut self, id: &str) {
        if id == BOUTON_ENREGISTRER {
            self.basculer_enregistrement();
        } else {
            self.basculer(id);
        }
    }
    fn manifeste_dossiers_bibliotheque(&self) -> Vec<String> {
        vec![
            "Cassettes".to_string(),
            prom::DOSSIER.to_string(),
            "GEN2 COLOR".to_string(),
            DOSSIER_PLAB.to_string(),
            art::DOSSIER.to_string(),
        ]
    }
    // ----- 0.1.5 : bibliothèque, ROM, formats, configurations
    fn manifeste_dossier_bibliotheque(&self) -> String {
        DOSSIER_BIBLIOTHEQUE.to_string()
    }
    fn manifeste_roms(&self) -> Vec<JeuRom> {
        // Woz Monitor, ACI et 2513 sont intégrés : seul le BASIC d'origine
        // (non fourni) est cherché dans ROM/
        vec![JeuRom {
            id: JEU_ROM.into(),
            nom: tr("Integer BASIC d'origine (votre copie)").into(),
            roms: vec![RomAttendue::new(
                tr("Integer BASIC de l'Apple 1 ($E000-$EFFF)"),
                FICHIER_BASIC,
                TAILLE_BASIC,
            )
            .avec_variable(VARIABLE_BASIC)
            .facultative()],
        }]
    }
    fn pilote_rom_machine(&mut self) -> Option<&mut dyn RomMachine> {
        Some(self)
    }
    fn manifeste_formats(&self) -> Vec<FormatFichier> {
        vec![
            FormatFichier::new(
                "woz",
                tr("Dump du Woz Monitor (Apple 1, « 0300: A9 01 … »)"),
                GenreFormat::Memoire,
            ),
            FormatFichier::new(
                "txt",
                tr("Ancien dump Woz ou art ASCII (v1.0)"),
                GenreFormat::Memoire,
            )
            .lecture_seule(),
            FormatFichier::new("mon", tr("Ancien dump Woz (v1.0)"), GenreFormat::Memoire)
                .lecture_seule(),
            FormatFichier::new(
                "apl",
                tr("Programme Apple 1 au format du Woz Monitor"),
                GenreFormat::Memoire,
            )
            .lecture_seule(),
            FormatFichier::new("wav", "Cassette ACI (WAV)", GenreFormat::Memoire).dans("Cassettes"),
            FormatFichier::new("aif", "Cassette ACI (AIFF)", GenreFormat::Memoire)
                .dans("Cassettes"),
            FormatFichier::new("aiff", "Cassette ACI (AIFF)", GenreFormat::Memoire)
                .dans("Cassettes"),
            FormatFichier::new(
                "bin",
                tr("Image binaire (BASIC d'origine en ROM/, CodeTank en P-LAB/)"),
                GenreFormat::Memoire,
            ),
            FormatFichier::new(
                "prg",
                tr("Binaire avec son adresse de chargement"),
                GenreFormat::Memoire,
            ),
            FormatFichier::new(
                "s",
                "Source assembleur 6502 (v1.0, ca65)",
                GenreFormat::Source(Langage::Assembleur),
            )
            .dans("Assembleur"),
            FormatFichier::new("png", "Photo convertie en art ASCII", GenreFormat::Memoire)
                .lecture_seule()
                .dans(art::DOSSIER),
            FormatFichier::new("jpg", "Photo convertie en art ASCII", GenreFormat::Memoire)
                .lecture_seule()
                .dans(art::DOSSIER),
            FormatFichier::new("jpeg", "Photo convertie en art ASCII", GenreFormat::Memoire)
                .lecture_seule()
                .dans(art::DOSSIER),
        ]
    }
    fn manifeste_peripheriques(&self) -> Vec<PeripheriqueOffert> {
        vec![
            PeripheriqueOffert::new(
                ids::MONITEUR_MONOCHROME,
                tr("Moniteur vert (terminal 40 × 24)"),
                GenrePeripherique::Moniteur(Moniteur::Monochrome),
                true,
            ),
            PeripheriqueOffert::new(
                ids::MONITEUR_COULEUR,
                tr("Moniteur couleur (GEN2 ou P-LAB)"),
                GenrePeripherique::Moniteur(Moniteur::Couleur),
                false,
            ),
            PeripheriqueOffert::new(
                BOUTON_GEN2,
                tr("Carte couleur GEN2"),
                GenrePeripherique::Bouton,
                false,
            ),
            PeripheriqueOffert::new(
                BOUTON_PLAB,
                tr("Carte graphique P-LAB"),
                GenrePeripherique::Bouton,
                false,
            ),
            PeripheriqueOffert::new(
                ids::CASSETTE,
                tr("Cassette (ACI)"),
                GenrePeripherique::Cassette,
                true,
            ),
        ]
    }
    fn manifeste_langages(&self) -> Vec<Langage> {
        vec![Langage::Basic, Langage::Assembleur]
    }
    fn manifeste_configurations(&self) -> Vec<ConfigurationMachine> {
        let c = |nom: &str, p: &[&str]| ConfigurationMachine {
            nom: nom.into(),
            machine: self.machine_id(),
            peripheriques: p.iter().map(|x| x.to_string()).collect(),
            langages: self.manifeste_langages(),
        };
        // la première est celle du premier lancement : écran vert, écran
        // couleur (carte GEN2) et clavier, comme le veut Wirehead
        // (rdk-apple1#17, point 4)
        vec![
            c(
                CONFIG_GEN2,
                &[
                    ids::MONITEUR_MONOCHROME,
                    ids::MONITEUR_COULEUR,
                    BOUTON_GEN2,
                    ids::CASSETTE,
                ],
            ),
            c(CONFIG_VERT, &[ids::MONITEUR_MONOCHROME, ids::CASSETTE]),
            c(
                CONFIG_PLAB,
                &[
                    ids::MONITEUR_MONOCHROME,
                    ids::MONITEUR_COULEUR,
                    BOUTON_PLAB,
                    ids::CASSETTE,
                ],
            ),
        ]
    }
    /// Contrat 1.2.0 : miniature de la configuration pour l'assistant de
    /// démarrage (dessin du kit, `miniature.rs`), d'après ses périphériques.
    fn manifeste_illustration(
        &self,
        configuration: &ConfigurationMachine,
    ) -> Option<rdk_pilotes::ImageVideo> {
        Some(miniature::dessiner(miniature::Contenu {
            moniteur_vert: configuration.a(ids::MONITEUR_MONOCHROME),
            moniteur_couleur: configuration.a(ids::MONITEUR_COULEUR),
            cassette: configuration.a(ids::CASSETTE),
            gen2: configuration.a(BOUTON_GEN2),
            plab: configuration.a(BOUTON_PLAB),
        }))
    }
    fn configuration_appliquer(&mut self, configuration: &ConfigurationMachine) {
        // les cartes suivent déjà la configuration (boutons du manifeste) ;
        // le moniteur couleur montre la carte de la configuration
        if configuration.a(BOUTON_GEN2) && !configuration.a(BOUTON_PLAB) {
            self.opts.source = SourceCouleur::Gen2;
        } else if configuration.a(BOUTON_PLAB) && !configuration.a(BOUTON_GEN2) {
            self.opts.source = SourceCouleur::Plab;
        }
        self.enregistrer_options();
        self.noter(trf!("configuration « {0} »", configuration.nom));
    }
    fn manifeste_ecrans_assistant(&self) -> [EcranAssistant; 2] {
        [
            EcranAssistant {
                titre: tr("Bienvenue dans le Retro Dev Kit Apple 1").to_string(),
                sous_titre: tr("Version 2.0, sur l'interface commune des Retro Dev Kits").to_string(),
                paragraphes: vec![
                    tr("L'Apple 1 (1976) : un 6502 à 1 MHz, 8 Ko de RAM (plus $E000-$FEFF pour le BASIC), un terminal de 40 × 24 caractères en majuscules, et le Woz Monitor en ROM.").to_string(),
                    tr("Au démarrage, le Woz Monitor affiche « \\ ». Tapez une adresse (FF00), une plage (FF00.FF0F), une écriture (300: A9 01) ou un lancement (300R).").to_string(),
                    tr("Écrivez en BASIC ou en assembleur 6502 dans l'éditeur, puis ▶ Lancer ; suivez le programme dans le débogueur, le Dump et le désassemblage.").to_string(),
                ],
            },
            EcranAssistant {
                titre: tr("Avant de commencer").to_string(),
                sous_titre: tr("ROM, BASIC, cartes et cassette").to_string(),
                paragraphes: vec![
                    tr("Le Woz Monitor, la ROM de l'ACI et le générateur de caractères 2513 sont fournis.").to_string(),
                    tr("BASIC : RDK G2 Basic, le clone de l'Integer BASIC du kit, est intégré et chargé en $E000 au premier lancement d'un programme BASIC (mode « Émulation »). Pour le BASIC d'origine (non fourni), copiez votre integer_basic.bin dans RDK librairies/Apple 1/ROM/ : reconnu, il libère le bouton « ROM réelle ».").to_string(),
                    tr("Configurations : « Apple 1 moniteur vert » (défaut), « Apple 1 moniteur vert + couleur GEN2 » (carte couleur d'Uncle Bernie) et « Apple 1 + P-LAB » (TMS9918A, NMI, CodeTank), dans la liste des configurations. Leur image va au moniteur couleur ; le terminal reste sur le moniteur vert.").to_string(),
                    tr("Cassette : la bibliothèque lit les fichiers WAV de l'ACI (⚡ rapide ou 📼 en temps réel) ; le bouton ⏺ Sortie ACI enregistre ce que la machine écrit (C100R puis début.finW).").to_string(),
                    tr("Comme en v1.0 : 🔆 Écran (phosphore vert, blanc ou ambre ; 📺 écran vintage), 🎨 Photo → ASCII, 💾 Sauver (.woz, .wav, Intel .hex…), programmateur d'EPROM (PROM), 📥 Exemples pour remplir votre bibliothèque.").to_string(),
                    tr("Cliquez sur l'écran pour envoyer le clavier de l'ordinateur à la machine, ou utilisez la fenêtre Clavier (QWERTY).").to_string(),
                ],
            },
        ]
    }
    // ----- 0.1.7 : état de la machine (menu 📸 État), fichiers .a1state de la v1.0
    fn etat_extension(&self) -> Option<String> {
        Some(EXTENSION_ETAT.into())
    }
    fn etat_enregistrer(&mut self, titre: &str) -> Resultat<Vec<u8>> {
        Ok(self.m.save_state(titre))
    }
    fn etat_charger(&mut self, octets: &[u8]) -> Resultat<String> {
        let (m, info) =
            Apple1::load_state(octets).map_err(|e| self.erreur(trf!("état illisible : {0}", e)))?;
        // rdk-apple1#36 : cartes et RAM de la machine avant le chargement
        // (celles de la configuration choisie), pour signaler un écart
        let avant = cartes_et_ram(&self.m.board.config);
        self.m = m;
        self.dbg.cont();
        self.chargement = None;
        self.plan = None;
        self.dernier = None;
        self.marche = Marche::Marche;
        // les cartes et la RAM viennent de l'état : les boutons suivent
        let c = &self.m.board.config;
        self.opts.gen2 = c.color_card;
        self.opts.gen2_50hz = c.color_50hz;
        self.opts.plab = c.plab_card;
        self.opts.plab_nmi = c.plab_nmi;
        if self.opts.gen2 && !self.opts.plab {
            self.opts.source = SourceCouleur::Gen2;
        } else if self.opts.plab && !self.opts.gen2 {
            self.opts.source = SourceCouleur::Plab;
        }
        self.m.board.config.display_throttle = self.vitesse == Vitesse::Reelle;
        self.enregistrer_options();
        let mut m = trf!(
            "état « {0} » chargé (PC ${1})",
            info.title,
            format!("{:04X}", info.pc)
        );
        if cartes_et_ram(&self.m.board.config) != avant {
            m.push_str(" — ");
            m.push_str(tr("cartes ou RAM de l'état différentes de la configuration choisie : la machine ne correspond plus au nom affiché ; choisissez une configuration dans 🖥 pour y revenir"));
        }
        self.noter(m.clone());
        Ok(m)
    }
    fn machine_langue(&mut self, _langue: rdk_pilotes::Langue) {
        // les textes de la machine sont traduits à chaque image (crate::i18n)
        if let Some(ctx) = self.contexte.get() {
            ctx.request_repaint();
        }
    }

    fn manifeste_a_programmateur_prom(&self) -> bool {
        true
    }
    fn manifeste_ouvrir_programmateur_prom(&mut self) {
        // v1.0 : fenêtre « Programmateur d'EPROM » (minipro : lire, écrire,
        // vérifier) ; 2.0 : fichiers en Intel HEX (.hex), dossier PROM/
        self.fenetres.prom.ouvrir();
        if self.contexte.get().is_none() {
            // sans fenêtre (essais, ligne de commande) : image Intel HEX de $E000-$EFFF
            match self.exporter_prom(0xE000, 0xEFFF) {
                Ok(m) => self.noter(m),
                Err(e) => {
                    let _ = self.erreur(e);
                }
            }
        }
    }
    fn manifeste_clavier(&self) -> Option<DessinClavier> {
        Some(clavier::dessin())
    }
    fn manifeste_moniteurs(&self) -> Vec<Moniteur> {
        vec![Moniteur::Monochrome, Moniteur::Couleur]
    }
    fn manifeste_clavier_au_demarrage(&self) -> bool {
        true
    }
    fn pilotes_syntaxe(&self) -> Vec<Box<dyn Syntaxe>> {
        vec![
            Box::new(syntaxe::SyntaxeBasicApple1),
            Box::new(syntaxe::SyntaxeAsm6502),
        ]
    }
}

/// rdk-apple1#31 : commutateurs de la GEN2 en texte, page 1, sans mélange.
fn texte_page_1(g: &mut apple1_core::gen2::Gen2) {
    g.flip(0b001); // SS_TEXT
    g.flip(0b100); // SS_PAGE1
    g.flip(0b010); // SS_FULL (pas de mélange)
}

/// rdk-apple1#36 : ce qui fait la configuration matérielle d'un état.
fn cartes_et_ram(c: &Config) -> (bool, bool, bool, u32, bool) {
    (c.color_card, c.plab_card, c.plab_nmi, c.ram_kb, c.ram_e000)
}

#[cfg(test)]
mod tests_lot4 {
    use super::*;

    /// rdk-apple1#31 : après la mise sous tension (HIRES), texte page 1.
    #[test]
    fn gen2_branchee_en_texte_page_1() {
        let mut g = apple1_core::gen2::Gen2::new(false);
        assert!(!g.modes().text);
        texte_page_1(&mut g);
        let m = g.modes();
        assert!(m.text && !m.page2 && !m.mixed, "{m:?}");
    }
}
