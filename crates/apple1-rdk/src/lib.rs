//! # apple1-rdk — l'Apple 1 sur RDK-Interface-Commune
//!
//! Version **2.0** du Retro Dev Kit Apple 1 (Linux). Tout ce qui est commun
//! (fenêtre principale, barre d'icônes, éditeur, moniteur, Dump, débogueur,
//! désassemblage, bibliothèque, assistant, préférences, barre d'état…) vient
//! de **RDK-Interface-Commune 0.1.4** (dépendance Git épinglée sur le commit
//! `add001d`). Ce paquet ne contient que **la liaison** entre la machine de
//! la v1.0 (`apple1-core` : 6502 NMOS, PIA 6820/6821, Woz Monitor, ACI,
//! terminal 40 × 24) et l'interface : [`MachineApple1`] implémente le trait
//! [`rdk_pilotes::Machine`] et ses pilotes, comme le demande
//! `manifest-apple1.md`.
//!
//! | Pilote | Ce que fait l'Apple 1 |
//! |---|---|
//! | `clavier-machine` | touches → ASCII 7 bits en majuscules ; Retour arrière = `_` ; clavier QWERTY dessiné sans pavé numérique |
//! | `video-numerique` | terminal 40 × 24, ROM de caractères 2513, 280 × 192, curseur `@` clignotant ; **moniteur monochrome seul** |
//! | `video-couleur`, `video-analogique` | vides (pas de couleur, pas de signal composite émulé) |
//! | `dump-memoire`, `debogueur-memoire` | `$0000`–`$FFFF`, ROM en lecture seule ; variables du BASIC lues dans sa table |
//! | `debogueur-processeur` | liste commune du 6502 ; Pas, Par-dessus, Sortir, Continuer, Pause, Exécuter jusqu'à |
//! | `desassembleur-memoire` | 6502 NMOS, étiquettes du Woz Monitor, de l'ACI et du programme assemblé |
//! | `editeur-machine` | ▶ Lancer : BASIC rangé en mémoire puis `RUN` (BASIC de l'utilisateur) ; assembleur 6502 rangé puis `xxxxR` ; ⏹ = RESET |
//! | `editeur-memoire` | ⬇ RAM : BASIC (forme tokenisée, avec sa page zéro) ou code assemblé |
//! | `biblio-memoire` | `.wav`/`.aif` (cassette ACI), `.txt`/`.mon`/`.hex` (vidage Woz), `.bin`, `.prg` ; Play = `xxxxR` ou `E2B3R` + `RUN` |
//! | `cassette-machine` | entrée : échantillons à 44 100 Hz vers le comparateur de l'ACI ; sortie : signal de l'ACI rendu à 44 100 Hz |
//! | `syntaxe-basic`, `syntaxe-asm` | BASIC entier de l'Apple 1, assembleur 6502 de la v1.0 |
//! | `machine-etat` | marche, vitesse, PC, mémoire, BASIC, cassette, enregistrement |
//! | les autres | vides (pas de souris, joystick, imprimante ni ports sur l'Apple 1) |
//!
//! **ROM** : seules celles de la v1.0 sont dans le dépôt (Woz Monitor, ACI,
//! 2513). Le BASIC d'origine n'est **pas** fourni : l'utilisateur pose sa
//! copie `integer_basic.bin` (4 Ko) dans le dossier du kit.

pub mod clavier;
pub mod ecran;
pub mod syntaxe;

use apple1_core::debug::{self, Debugger, Mission, Stop, Symbols, VarValue};
use apple1_core::machine::Config;
use apple1_core::program::{self, Program};
use apple1_core::tape::{self, TapePlayer};
use apple1_core::{asm6502, intbasic, Machine as Apple1};
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
    Adresse, BoutonPeripherique, CommandeExecution, DessinClavier, DocumentEditeur, EcranAssistant, ErreurPilote, EtatMachine, ImageVideo, Langage, LigneDesassemblee, Machine, Marche, Moniteur,
    Processeur, Programme, Registre, Resultat, ToucheHote, Variable, Vitesse,
};
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Version de l'application RDK Apple 1.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Fréquence d'échantillonnage du pilote `cassette-machine` (entrée et sortie).
pub const FREQUENCE_CASSETTE: u32 = 44_100;

/// Variable d'environnement qui peut désigner la copie du BASIC.
pub const VARIABLE_BASIC: &str = "RDK_APPLE1_BASIC";

/// Noms cherchés pour le BASIC de l'utilisateur (comme la v1.0).
pub const NOMS_BASIC: [&str; 6] = ["integer_basic.bin", "Integer_Basic.bin", "INTEGER_BASIC.bin", "basic.bin", "BASIC.bin", "intbasic.bin"];

/// Démarrage à chaud du BASIC (le programme en mémoire est gardé).
pub const BASIC_CHAUD: &str = "E2B3R";

/// Identifiant du bouton « Enregistrer la sortie ACI ».
pub const BOUTON_ENREGISTRER: &str = "aci-enregistrer";

/// Facteur de vitesse accélérée.
const ACCELERATION: u64 = 20;

/// Étiquettes du Woz Monitor et de l'ACI pour le désassembleur.
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
    (0xC170, "ACI-WRITE"),
    (0xC18D, "ACI-READ"),
    (0xD010, "KBD"),
    (0xD011, "KBDCR"),
    (0xD012, "DSP"),
    (0xD013, "DSPCR"),
    (0xE000, "BASIC"),
    (0xE2B3, "BASIC-CHAUD"),
];

fn symboles_de_base() -> Symbols {
    ETIQUETTES.iter().map(|(a, n)| (*a, n.to_string())).collect()
}

/// L'Apple 1 vu par l'interface commune.
pub struct MachineApple1 {
    m: Apple1,
    dbg: Debugger,
    marche: Marche,
    vitesse: Vitesse,
    /// Dossier du kit (bibliothèque `Biblio/`, BASIC de l'utilisateur).
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
}

impl MachineApple1 {
    /// Apple 1 standard de la v1.0 : 8 Ko de RAM en `$0000` + 4 Ko en `$E000`, ACI.
    pub fn new(dossier: PathBuf) -> Self {
        let mut m = Apple1::new(Config::default());
        m.board.recording = None;
        MachineApple1 {
            m,
            dbg: Debugger::default(),
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

    /// Copie du BASIC de l'utilisateur : variable `RDK_APPLE1_BASIC`, puis
    /// dossier du kit, `Biblio/`, `Biblio/BASIC/`, enfin `~/apple1/`.
    pub fn chercher_basic(&self) -> Option<PathBuf> {
        if let Some(p) = std::env::var_os(VARIABLE_BASIC).map(PathBuf::from).filter(|p| p.is_file()) {
            return Some(p);
        }
        let mut dossiers = vec![self.dossier.clone(), self.dossier.join("Biblio"), self.dossier.join("Biblio").join("BASIC")];
        if let Some(h) = std::env::var_os("HOME") {
            dossiers.push(PathBuf::from(h).join("apple1"));
        }
        dossiers.iter().flat_map(|d| NOMS_BASIC.iter().map(move |n| d.join(n))).find(|p| p.is_file())
    }

    /// Le BASIC est-il en mémoire en `$E000` ?
    pub fn basic_present(&self) -> bool {
        debug::basic_present(&|a| self.m.board.peek(a))
    }

    /// Range le BASIC de l'utilisateur en `$E000` s'il n'y est pas déjà.
    fn assurer_basic(&mut self) -> Resultat<()> {
        if self.basic_present() {
            return Ok(());
        }
        let Some(chemin) = self.chercher_basic() else {
            return Err(self.erreur(
                "BASIC absent : il n'est pas fourni (propriétaire). Copiez votre integer_basic.bin (4 Ko) dans le dossier du kit, dans Biblio/ ou ~/apple1/, ou donnez son chemin dans RDK_APPLE1_BASIC.",
            ));
        };
        let octets = std::fs::read(&chemin).map_err(|e| self.erreur(format!("{} : {e}", chemin.display())))?;
        self.ranger_basic(&octets).map_err(|e| self.erreur(format!("{} : {e}", chemin.display())))
    }

    /// Range une image de 4 Ko du BASIC en `$E000` (premier octet `JMP`).
    pub fn ranger_basic(&mut self, octets: &[u8]) -> Result<(), String> {
        if octets.len() != 4096 || octets[0] != 0x4C {
            return Err(format!("image du BASIC invalide ({} octets, il en faut 4096 commençant par JMP)", octets.len()));
        }
        if self.m.load(0xE000, octets) > 0 {
            return Err("pas de RAM en $E000".into());
        }
        Ok(())
    }

    fn ranger(&mut self, blocs: &[(u16, Vec<u8>)]) -> Resultat<usize> {
        let mut total = 0;
        for (a, o) in blocs {
            if self.m.load(*a, o) > 0 {
                return Err(self.erreur(format!("pas de RAM pour ${a:04X}-${:04X}", (*a as usize + o.len().max(1) - 1) as u16)));
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
            Ok(a) => {
                self.symboles = symboles_de_base();
                self.symboles.extend(debug::symbols_from(&a.symbols));
                self.depart = a.origin;
                Ok(a)
            }
            Err(e) => {
                let n = e.len();
                let premiere = e.first().map(|x| x.to_string()).unwrap_or_default();
                Err(self.erreur(format!("{n} erreur(s) d'assemblage ; {premiere}")))
            }
        }
    }

    fn image_basic(&mut self, texte: &str) -> Resultat<intbasic::Image> {
        let l = intbasic::tokenize(texte);
        if let Some(e) = l.errors.first() {
            return Err(self.erreur(format!("{} erreur(s) BASIC ; {e}", l.errors.len())));
        }
        if l.lines.is_empty() {
            return Err(self.erreur("aucune ligne BASIC numérotée"));
        }
        let (lomem, himem) = intbasic::layout(l.size(), Some(8));
        intbasic::Image::build(&l.lines, lomem, himem).map_err(|e| self.erreur(format!("BASIC : mémoire pleine ({e:?}), programme trop grand pour 8 Ko")))
    }

    fn est_basic(p: &Program) -> bool {
        p.blocks.iter().any(|b| b.0 == intbasic::ZP_START)
    }

    fn programme_de(&mut self, programme: &Programme) -> Resultat<Program> {
        let chemin = programme.chemin.clone().unwrap_or_else(|| PathBuf::from(&programme.nom));
        let nom = chemin.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();
        // le BASIC de l'utilisateur, posé dans la bibliothèque
        if programme.octets.len() == 4096 && programme.octets[0] == 0x4C && nom.contains("basic") {
            return Ok(Program { blocks: vec![(0xE000, programme.octets.clone())], run: Some(0xE000), notes: vec![] });
        }
        if nom.ends_with(".prg") {
            if programme.octets.len() < 3 {
                return Err(self.erreur("fichier .prg trop court"));
            }
            let a = u16::from_le_bytes([programme.octets[0], programme.octets[1]]);
            return Ok(Program { blocks: vec![(a, programme.octets[2..].to_vec())], run: Some(a), notes: vec![] });
        }
        let genre = program::kind_of(&chemin, Some(&programme.octets));
        program::load_bytes(&programme.octets, genre, None).map_err(|e| self.erreur(format!("{} : {e}", programme.nom)))
    }

    fn source(programme: &Programme) -> Option<Langage> {
        let ext = Path::new(&programme.nom).extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
        match Langage::depuis_extension(&ext) {
            l @ (Langage::Basic | Langage::Assembleur) => Some(l),
            _ => None,
        }
    }

    fn document(programme: &Programme, langage: Langage) -> DocumentEditeur {
        DocumentEditeur { titre: programme.nom.clone(), chemin: programme.chemin.clone(), langage, texte: String::from_utf8_lossy(&programme.octets).into_owned() }
    }

    fn arret_debogueur(&mut self, s: Stop) {
        self.marche = Marche::Pause;
        let pc = self.m.cpu.pc;
        self.noter(match s {
            Stop::Break(a) => format!("point d'arrêt en ${a:04X}"),
            Stop::BasicLine(l) | Stop::BasicBreak(l) => format!("BASIC : ligne {l}"),
            Stop::Jam => format!("6502 bloqué (opcode JAM) en ${pc:04X}"),
            Stop::Pause | Stop::Step => format!("arrêt en ${pc:04X}"),
        });
    }

    fn mener(&mut self, cycles: u64) {
        match self.dbg.run(&mut self.m, cycles) {
            Some(s) => self.arret_debogueur(s),
            None => self.marche = if self.dbg.paused { Marche::Pause } else { Marche::Marche },
        }
    }

    /// Démarre ou arrête l'enregistrement de la sortie ACI ; à l'arrêt, le
    /// signal est écrit dans `Biblio/Cassettes/` (fichier WAV).
    fn basculer_enregistrement(&mut self) {
        if self.m.board.recording.is_none() {
            self.m.board.recording = Some(Vec::new());
            self.noter("enregistrement de la sortie ACI : tapez C100R puis début.finW");
            return;
        }
        let ech = self.cassette_sortie();
        self.m.board.recording = None;
        let dossier = self.dossier.join("Biblio").join("Cassettes");
        let fichier = dossier.join(format!("sortie-aci-{}.wav", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)));
        let octets = apple1_core::wav::encode_wav(&ech, FREQUENCE_CASSETTE, 16, Some("Retro Dev Kit Apple 1 2.0"));
        match std::fs::create_dir_all(&dossier).and_then(|_| std::fs::write(&fichier, octets)) {
            Ok(()) => self.noter(format!("cassette écrite : {}", fichier.display())),
            Err(e) => {
                let _ = self.erreur(format!("{} : {e}", fichier.display()));
            }
        }
    }
}

// ----- pilotes vides : l'Apple 1 n'a ni souris, ni joystick, ni imprimante,
// ni ports, ni signal analogique émulé, ni couleur.
impl SourisMachine for MachineApple1 {}
impl JoystickMachine for MachineApple1 {}
impl MachineImprimante for MachineApple1 {}
impl PortParallele for MachineApple1 {}
impl PortSerie for MachineApple1 {}
impl VideoAnalogique for MachineApple1 {}

impl VideoCouleur for MachineApple1 {
    fn couleur_disponible(&self) -> bool {
        false
    }
    fn moniteur_branche(&self) -> Moniteur {
        Moniteur::Monochrome
    }
}

impl VideoNumerique for MachineApple1 {
    fn image(&mut self, moniteur: Moniteur) -> Option<ImageVideo> {
        (moniteur == Moniteur::Monochrome).then(|| ecran::image(&self.m))
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
                    VarValue::Int(l) => format!("({})", l.iter().take(16).map(|x| x.to_string()).collect::<Vec<_>>().join(", ")),
                };
                Variable { nom: v.name, valeur }
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
        let valeurs = [c.a as u32, c.x as u32, c.y as u32, c.sp as u32, c.pc as u32, c.p as u32];
        REGISTRES_6502.iter().zip(valeurs).map(|(n, v)| Registre { nom: n.to_string(), valeur: v, bits: if *n == "PC" { 16 } else { 8 } }).collect()
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
            _ => return Err(ErreurPilote::Message(format!("registre inconnu du 6502 : {nom}"))),
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
}

impl DesassembleurMemoire for MachineApple1 {
    fn desassembleur_lire(&self, adresse: Adresse) -> Option<u8> {
        self.dump_lire(adresse)
    }
    fn desassembler(&self, adresse: Adresse) -> Option<LigneDesassemblee> {
        if adresse > 0xFFFF {
            return None;
        }
        let l = debug::disasm(&|a| self.m.board.peek(a), adresse as u16, &self.symboles);
        let texte = match self.symboles.get(&l.addr) {
            Some(n) => format!("{n}: {}", l.text),
            None => l.text,
        };
        Some(LigneDesassemblee { adresse: l.addr as Adresse, octets: l.bytes, texte })
    }
    fn desassembleur_depart(&self) -> Adresse {
        self.depart as Adresse
    }
}

impl EditeurMachine for MachineApple1 {
    fn editeur_lancer(&mut self, document: &DocumentEditeur) -> Resultat<String> {
        match document.langage {
            Langage::Basic => {
                let img = self.image_basic(&document.texte)?;
                self.assurer_basic()?;
                let n = img.prog.len();
                self.reset_et_taper("");
                self.ranger(&img.blocks())?;
                self.m.type_text(&format!("{BASIC_CHAUD}\rRUN\r"));
                self.erreurs.clear();
                Ok(format!("BASIC : {n} octets rangés en ${:04X}, RUN", img.pp))
            }
            Langage::Assembleur => {
                let a = self.assembler(document)?;
                self.reset_et_taper("");
                self.ranger(&[(a.origin, a.bytes.clone())])?;
                self.m.type_text(&format!("{}\r", a.run_command()));
                self.erreurs.clear();
                Ok(format!("{} octets assemblés en ${:04X}, {}", a.bytes.len(), a.origin, a.run_command()))
            }
            _ => Err(self.erreur("l'Apple 1 lance du BASIC (.bas) ou de l'assembleur 6502 (.asm)")),
        }
    }
    fn editeur_pause(&mut self) -> Resultat<()> {
        self.machine_marche(if self.marche == Marche::Pause { Marche::Marche } else { Marche::Pause });
        Ok(())
    }
    fn editeur_arreter(&mut self) -> Resultat<()> {
        // l'Apple 1 n'a pas de BREAK : RESET ramène au Woz Monitor
        self.machine_reset();
        Ok(())
    }
}

impl EditeurMemoire for MachineApple1 {
    fn injection_possible(&self, langage: &Langage) -> bool {
        matches!(langage, Langage::Basic | Langage::Assembleur)
    }
    fn injecter(&mut self, document: &DocumentEditeur) -> Resultat<String> {
        match document.langage {
            Langage::Basic => {
                let img = self.image_basic(&document.texte)?;
                let n = self.ranger(&img.blocks())?;
                self.erreurs.clear();
                Ok(format!("BASIC : {n} octets rangés ($004A-$00FF et ${:04X}-${:04X}) ; tapez {BASIC_CHAUD} puis LIST ou RUN", img.pp, img.himem - 1))
            }
            Langage::Assembleur => {
                let a = self.assembler(document)?;
                self.ranger(&[(a.origin, a.bytes.clone())])?;
                self.erreurs.clear();
                Ok(format!("{} octets rangés en ${:04X} ; tapez {}", a.bytes.len(), a.origin, a.run_command()))
            }
            _ => Err(self.erreur("seuls le BASIC et l'assembleur 6502 vont en mémoire")),
        }
    }
}

impl BiblioMemoire for MachineApple1 {
    fn charger(&mut self, programme: &Programme) -> Resultat<()> {
        if let Some(l) = Self::source(programme) {
            return self.injecter(&Self::document(programme, l)).map(|m| self.noter(m));
        }
        let p = self.programme_de(programme)?;
        let n = self.ranger(&p.blocks)?;
        if let Some((a, _)) = p.blocks.first() {
            if !Self::est_basic(&p) {
                self.depart = *a;
            }
        }
        self.noter(format!("{} : {n} octets chargés ({})", programme.nom, p.ranges().iter().map(|(a, b)| format!("${a:04X}-${b:04X}")).collect::<Vec<_>>().join(", ")));
        self.erreurs.clear();
        self.dernier = Some(p);
        Ok(())
    }
    fn jouer(&mut self, programme: &Programme) -> Resultat<()> {
        if let Some(l) = Self::source(programme) {
            return self.editeur_lancer(&Self::document(programme, l)).map(|m| self.noter(m));
        }
        let p = self.programme_de(programme)?;
        if Self::est_basic(&p) {
            self.assurer_basic()?;
        }
        self.reset_et_taper("");
        self.ranger(&p.blocks)?;
        self.dernier = Some(p);
        self.executer(programme)
    }
    fn executer(&mut self, programme: &Programme) -> Resultat<()> {
        let Some(p) = self.dernier.clone() else {
            return Err(self.erreur("aucun programme chargé"));
        };
        let touches = if Self::est_basic(&p) {
            self.assurer_basic()?;
            format!("{BASIC_CHAUD}\rRUN\r")
        } else {
            let a = p.run.or_else(|| p.blocks.first().map(|b| b.0)).unwrap_or(0x0300);
            format!("{a:X}R\r")
        };
        self.m.type_text(&touches);
        self.noter(format!("{} : {}", programme.nom, touches.replace('\r', " ").trim()));
        Ok(())
    }
    fn double_clic_charge(&self, programme: &Programme) -> bool {
        let ext = Path::new(&programme.nom).extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
        matches!(ext.as_str(), "wav" | "wave" | "aif" | "aiff" | "bin" | "prg" | "mon" | "hex" | "apl")
    }
}

impl CassetteMachine for MachineApple1 {
    fn cassette_entree(&mut self, echantillons: &[f32]) {
        self.m.board.player = Some(TapePlayer::from_samples(echantillons, FREQUENCE_CASSETTE, "cassette"));
        self.noter("cassette insérée : tapez C100R puis début.finR");
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
        tape::render_toggles(&bascules, &tape::RenderOptions { sample_rate: FREQUENCE_CASSETTE, ..tape::RenderOptions::default() })
    }
}

impl MachineEtatPilote for MachineApple1 {
    fn etat(&self) -> EtatMachine {
        let mut actifs = vec!["ACI".to_string()];
        if self.m.board.recording.is_some() {
            actifs.push("⏺ enregistrement ACI".to_string());
        }
        let mut complements = vec![
            ("PC".to_string(), format!("${:04X}", self.m.cpu.pc)),
            ("RAM".to_string(), format!("{} Ko + 4 Ko en $E000", self.m.board.config.ram_kb)),
            ("BASIC".to_string(), if self.basic_present() { "en mémoire".to_string() } else { "absent".to_string() }),
            ("Moniteur".to_string(), "monochrome".to_string()),
        ];
        if let Some(p) = &self.m.board.player {
            let now = self.m.board.now_secs();
            complements.push(("Cassette".to_string(), format!("{} {:.1}/{:.1} s", p.name, p.position(now), p.duration())));
        }
        if let Some(m) = &self.message {
            complements.push(("Message".to_string(), m.clone()));
        }
        EtatMachine { marche: self.marche, vitesse: self.vitesse, erreurs: self.erreurs.clone(), peripheriques_actifs: actifs, complements }
    }
}

impl Machine for MachineApple1 {
    fn machine_nom(&self) -> String {
        "Apple 1".to_string()
    }
    fn machine_version(&self) -> String {
        VERSION.to_string()
    }
    fn machine_a_propos(&self) -> Vec<String> {
        vec![
            format!("Retro Dev Kit Apple 1 {VERSION} (Linux), sur RDK-Interface-Commune {}.", rdk_interface::VERSION),
            "Licence GPL-3.0. Machine reprise de la v1.0 « Original Sin » (crate apple1-core).".to_string(),
            "ROM fournies : Woz Monitor, ACI (Steve Wozniak, 1976) et générateur de caractères 2513, comme en v1.0.".to_string(),
            "Le BASIC d'origine n'est pas fourni : posez votre copie integer_basic.bin dans le dossier du kit.".to_string(),
        ]
    }
    fn machine_reset(&mut self) {
        self.m.reset();
        self.dbg.cont();
        self.marche = Marche::Marche;
    }
    fn machine_avancer(&mut self, duree: Duration) {
        if self.marche != Marche::Marche {
            return;
        }
        let mut cycles = self.m.board.config.timing.secs_to_cycles(duree.as_secs_f64().min(0.1));
        if self.vitesse == Vitesse::Acceleree {
            cycles *= ACCELERATION;
        }
        self.mener(cycles);
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
        vec![BoutonPeripherique {
            id: BOUTON_ENREGISTRER.to_string(),
            icone: "⏺".to_string(),
            libelle: "Sortie ACI".to_string(),
            info_bulle: "Enregistre la sortie cassette de l'ACI (C100R puis début.finW) ; au second clic, le fichier WAV est écrit dans Biblio/Cassettes.".to_string(),
            actif: self.m.board.recording.is_some(),
        }]
    }
    fn manifeste_basculer_peripherique(&mut self, id: &str) {
        if id == BOUTON_ENREGISTRER {
            self.basculer_enregistrement();
        }
    }
    fn manifeste_dossiers_bibliotheque(&self) -> Vec<String> {
        vec!["Cassettes".to_string(), "PROM".to_string()]
    }
    fn manifeste_ecrans_assistant(&self) -> [EcranAssistant; 2] {
        [
            EcranAssistant {
                titre: "Bienvenue dans le Retro Dev Kit Apple 1".to_string(),
                sous_titre: "Version 2.0, sur l'interface commune des Retro Dev Kits".to_string(),
                paragraphes: vec![
                    "L'Apple 1 (1976) : un 6502 à 1 MHz, 8 Ko de RAM (plus 4 Ko en $E000), un terminal de 40 × 24 caractères en majuscules, et le Woz Monitor en ROM.".to_string(),
                    "Au démarrage, le Woz Monitor affiche « \\ ». Tapez une adresse (FF00), une plage (FF00.FF0F), une écriture (300: A9 01) ou un lancement (300R).".to_string(),
                    "Écrivez en BASIC ou en assembleur 6502 dans l'éditeur, puis ▶ Lancer ; suivez le programme dans le débogueur, le Dump et le désassemblage.".to_string(),
                ],
            },
            EcranAssistant {
                titre: "Avant de commencer".to_string(),
                sous_titre: "ROM, BASIC et cassette".to_string(),
                paragraphes: vec![
                    "Le Woz Monitor, la ROM de l'ACI et le générateur de caractères 2513 sont fournis.".to_string(),
                    "Le BASIC d'origine n'est pas fourni : copiez votre integer_basic.bin (4 Ko) dans le dossier du kit ou dans ~/apple1/. Sans lui, l'assembleur et le Woz Monitor marchent, pas le BASIC.".to_string(),
                    "Cassette : la bibliothèque lit les fichiers WAV de l'ACI ; le bouton ⏺ Sortie ACI enregistre ce que la machine écrit (C100R puis début.finW).".to_string(),
                    "Cliquez sur l'écran pour envoyer le clavier de l'ordinateur à la machine, ou utilisez la fenêtre Clavier (QWERTY).".to_string(),
                ],
            },
        ]
    }
    fn manifeste_a_programmateur_prom(&self) -> bool {
        true
    }
    fn manifeste_ouvrir_programmateur_prom(&mut self) {
        // v1.0 : programmateur d'EPROM (minipro) ; 2.0 : l'image de la zone
        // $E000-$EFFF (2732, 4 Ko) est écrite dans Biblio/PROM, à graver avec minipro.
        let dossier = self.dossier.join("Biblio").join("PROM");
        let fichier = dossier.join("prom-E000-EFFF.bin");
        let image = self.m.dump(0xE000, 0xEFFF);
        match std::fs::create_dir_all(&dossier).and_then(|_| std::fs::write(&fichier, &image)) {
            Ok(()) => {
                let puce = apple1_core::eprom::chip("2732").map(|c| c.minipro).unwrap_or("2732@DIP24");
                self.noter(format!("image PROM $E000-$EFFF écrite : {} ; gravure : minipro -p {puce} -w {}", fichier.display(), fichier.display()));
            }
            Err(e) => {
                let _ = self.erreur(format!("{} : {e}", fichier.display()));
            }
        }
    }
    fn manifeste_clavier(&self) -> Option<DessinClavier> {
        Some(clavier::dessin())
    }
    fn manifeste_moniteurs(&self) -> Vec<Moniteur> {
        vec![Moniteur::Monochrome]
    }
    fn manifeste_clavier_au_demarrage(&self) -> bool {
        true
    }
    fn pilotes_syntaxe(&self) -> Vec<Box<dyn Syntaxe>> {
        vec![Box::new(syntaxe::SyntaxeBasicApple1), Box::new(syntaxe::SyntaxeAsm6502)]
    }
}
