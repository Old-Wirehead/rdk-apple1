//! Essais de bout en bout de l'Apple 1 2.0 par les pilotes de
//! RDK-Interface-Commune (sans fenêtre).

use apple1_rdk::{MachineApple1, FREQUENCE_CASSETTE};
use rdk_pilotes::pilotes::{
    biblio_memoire::BiblioMemoire, cassette_machine::CassetteMachine, clavier_machine::ClavierMachine, debogueur_memoire::DebogueurMemoire, debogueur_processeur::DebogueurProcesseur,
    desassembleur_memoire::DesassembleurMemoire, dump_memoire::DumpMemoire, editeur_machine::EditeurMachine, editeur_memoire::EditeurMemoire, machine_etat::MachineEtatPilote,
    video_couleur::VideoCouleur, video_numerique::VideoNumerique,
};
use rdk_pilotes::*;
use std::path::PathBuf;
use std::time::Duration;

fn machine() -> MachineApple1 {
    let dossier = std::env::temp_dir().join(format!("rdk-apple1-essai-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dossier);
    let mut m = MachineApple1::new(dossier);
    m.coeur_mut().board.config.display_throttle = false;
    assert!(m.jusqua_attente(3_000_000));
    m
}

fn touche(nom: &str, c: Option<char>) -> ToucheHote {
    ToucheHote { nom: nom.into(), caractere: c, ..ToucheHote::default() }
}

fn taper(m: &mut MachineApple1, texte: &str) {
    for c in texte.chars() {
        let t = if c == '\n' { touche("Entrée", None) } else { touche(&c.to_string(), Some(c)) };
        m.touche_machine(&t, true);
        m.touche_machine(&t, false);
        // une touche à la fois, comme un humain
        m.jusqua_attente(200_000);
    }
}

fn doc(langage: Langage, texte: &str) -> DocumentEditeur {
    DocumentEditeur { titre: "essai".into(), chemin: None, langage, texte: texte.into() }
}

#[test]
fn identite_et_manifeste() {
    let m = machine();
    assert_eq!(m.machine_nom(), "Apple 1");
    assert_eq!(m.machine_titre_fenetre(), "Retro Dev Kit Apple 1");
    assert_eq!(m.machine_version(), "2.0.0");
    assert_eq!(m.manifeste_moniteurs(), vec![Moniteur::Monochrome]);
    assert!(!m.couleur_disponible());
    assert!(m.manifeste_clavier_au_demarrage());
    assert!(m.manifeste_a_programmateur_prom());
    let c = m.manifeste_clavier().expect("clavier dessiné par le manifeste");
    assert!(c.titre.contains("QWERTY") && c.titre.contains("sans pavé numérique"));
    let ecrans = m.manifeste_ecrans_assistant();
    assert!(ecrans[0].titre.contains("Apple 1"));
    assert_eq!(m.processeur(), Processeur::M6502);
    let noms: Vec<String> = m.registres().into_iter().map(|r| r.nom).collect();
    assert_eq!(noms, ["A", "X", "Y", "S", "PC", "P"]);
    assert_eq!(m.pilotes_syntaxe().len(), 2);
}

#[test]
fn wozmon_au_demarrage_et_clavier_reel() {
    let mut m = machine();
    assert!(m.texte_ecran().starts_with('\\'), "{}", m.texte_ecran());
    taper(&mut m, "ff00.ff07\n");
    assert!(m.texte_ecran().contains("FF00: D8 58 A0 7F 8C 12 D0 A9"), "{}", m.texte_ecran());
    let mut m2 = m;
    let img = m2.image(Moniteur::Monochrome).expect("image monochrome");
    assert_eq!((img.largeur, img.hauteur), (280, 192));
    assert!(m2.image(Moniteur::Couleur).is_none(), "pas d'écran couleur");
}

#[test]
fn touches_remanentes_du_clavier_dessine() {
    let mut m = machine();
    m.touche_machine(&touche("Maj", None), true);
    m.touche_machine(&touche("1", Some('1')), true);
    m.jusqua_attente(500_000);
    assert!(m.texte_ecran().contains('!'), "{}", m.texte_ecran());
}

#[test]
fn assembleur_lancer_et_desassembler() {
    let mut m = machine();
    let src = "  .org $0300\nstart: LDX #0\nloop: LDA msg,X\n  BEQ done\n  JSR $FFEF\n  INX\n  BNE loop\ndone: JMP $FF1F\nmsg: .byte $C8, $C9, 0\n";
    let r = m.editeur_lancer(&doc(Langage::Assembleur, src)).expect("lancer");
    assert!(r.contains("$0300"), "{r}");
    assert!(m.jusqua_attente(5_000_000));
    assert!(m.texte_ecran().contains("HI"), "{}", m.texte_ecran());
    let l = m.desassembler(0x0300).unwrap();
    assert!(l.texte.contains("LDX") && l.texte.starts_with("start"), "{}", l.texte);
    assert_eq!(m.desassembleur_depart(), 0x0300);
    assert!(m.desassembler(0xFF00).unwrap().texte.starts_with("RESET"));
}

#[test]
fn erreurs_de_l_editeur() {
    let mut m = machine();
    assert!(m.editeur_lancer(&doc(Langage::Assembleur, "  .org $0300\n  LDQ #1\n")).is_err());
    assert!(m.injecter(&doc(Langage::Basic, "10 PRINT \"A\"\n20 LIST\n")).is_err());
    assert!(m.editeur_lancer(&doc(Langage::Texte, "x")).is_err());
    assert!(!m.etat().erreurs.is_empty());
}

#[test]
fn basic_range_en_memoire() {
    let mut m = machine();
    assert!(m.injection_possible(&Langage::Basic));
    let r = m.injecter(&doc(Langage::Basic, "10 PRINT \"HELLO\"\n20 END\n")).expect("injecter");
    assert!(r.contains("E2B3R"), "{r}");
    // LOMEM / HIMEM dans la page zéro, programme sous HIMEM ($1000)
    assert_eq!(m.dump_lire(0x4C), Some(0x00));
    assert_eq!(m.dump_lire(0x4D), Some(0x10));
    let pp = m.dump_lire(0xCA).unwrap() as u32 | (m.dump_lire(0xCB).unwrap() as u32) << 8;
    assert!(pp < 0x1000 && pp > 0x0800);
}

#[test]
fn basic_sans_copie_de_l_utilisateur() {
    std::env::remove_var(apple1_rdk::VARIABLE_BASIC);
    let mut m = machine();
    if m.chercher_basic().is_some() {
        return; // une copie personnelle existe sur ce poste
    }
    let e = m.editeur_lancer(&doc(Langage::Basic, "10 PRINT 1\n")).unwrap_err();
    assert!(e.to_string().contains("integer_basic.bin"), "{e}");
}

#[test]
fn memoire_rom_en_lecture_seule() {
    let mut m = machine();
    assert_eq!(m.dump_plage(), 0..=0xFFFF);
    assert_eq!(m.dump_lire(0xFF00), Some(0xD8));
    assert_eq!(m.dump_ecrire(0xFF00, 0), Err(ErreurPilote::LectureSeule(0xFF00)));
    m.dump_ecrire(0x0300, 0x42).unwrap();
    assert_eq!(m.debogueur_lire(0x0300), Some(0x42));
}

#[test]
fn debogueur_pas_a_pas() {
    let mut m = machine();
    m.injecter(&doc(Langage::Assembleur, "  .org $0300\n  LDA #$12\n  LDX #$34\n  JMP $0300\n")).unwrap();
    m.commande(CommandeExecution::Pause).unwrap();
    m.ecrire_registre("PC", 0x0300).unwrap();
    m.commande(CommandeExecution::Pas).unwrap();
    assert_eq!(m.registres()[0].valeur, 0x12);
    assert_eq!(m.registres()[4].valeur, 0x0302);
    m.commande(CommandeExecution::ExecuterJusqua(0x0304)).unwrap();
    assert_eq!(m.registres()[1].valeur, 0x34);
    assert_eq!(m.etat().marche, Marche::Pause);
    m.commande(CommandeExecution::Continuer).unwrap();
    assert_eq!(m.etat().marche, Marche::Marche);
    m.machine_avancer(Duration::from_millis(20));
}

#[test]
fn bibliotheque_vidage_woz_et_play() {
    let mut m = machine();
    let p = Programme { nom: "hello.txt".into(), chemin: Some(PathBuf::from("hello.txt")), octets: b"0300: A9 C1 20 EF FF 4C 1F FF\n".to_vec() };
    // .txt = texte pour l'interface commune : on passe par l'extension .mon
    let p = Programme { nom: "hello.mon".into(), chemin: Some(PathBuf::from("hello.mon")), ..p };
    assert!(m.double_clic_charge(&p));
    m.charger(&p).unwrap();
    assert_eq!(m.dump_lire(0x0300), Some(0xA9));
    m.jouer(&p).unwrap();
    assert!(m.jusqua_attente(5_000_000));
    assert!(m.texte_ecran().contains("300R"), "{}", m.texte_ecran());
    assert!(m.texte_ecran().contains("0300: A9A"), "{}", m.texte_ecran());
}

#[test]
fn cassette_aci_aller_retour() {
    // écriture par la ROM de l'ACI, puis relecture sur une autre machine
    let mut m = machine();
    m.injecter(&doc(Langage::Assembleur, "  .org $0300\n  .byte 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16\n")).unwrap();
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_ENREGISTRER);
    m.taper("C100R\n0300.030FW\n");
    assert!(m.jusqua_attente(200_000_000));
    let ech = m.cassette_sortie();
    assert!(ech.len() > FREQUENCE_CASSETTE as usize, "signal trop court : {}", ech.len());
    let mut b = machine();
    b.cassette_entree(&ech);
    b.taper("C100R\n0300.030FR\n");
    assert!(b.jusqua_attente(200_000_000));
    let lu: Vec<u8> = (0x300..0x310).map(|a| b.dump_lire(a).unwrap()).collect();
    assert_eq!(lu, (1..=16).collect::<Vec<u8>>(), "{}", b.texte_ecran());
}

#[test]
fn programmateur_prom_ecrit_l_image() {
    let mut m = machine();
    m.manifeste_ouvrir_programmateur_prom();
    let etat = m.etat();
    let msg = etat.complements.iter().find(|c| c.0 == "Message").map(|c| c.1.clone()).unwrap_or_default();
    assert!(msg.contains("minipro -p 2732@DIP24"), "{msg} {:?}", etat.erreurs);
}

#[test]
fn basic_lancer_avec_la_copie_de_l_utilisateur() {
    // ne tourne que si une copie personnelle du BASIC est sur ce poste
    // (jamais dans le dépôt)
    let mut m = machine();
    if m.chercher_basic().is_none() {
        return;
    }
    let r = m.editeur_lancer(&doc(Langage::Basic, "10 FOR I = 1 TO 3\n20 PRINT \"HELLO \"; I\n30 NEXT I\n40 END\n")).expect("lancer");
    assert!(r.contains("RUN"), "{r}");
    assert!(m.jusqua_attente(50_000_000));
    let t = m.texte_ecran();
    assert!(t.contains("HELLO 1") && t.contains("HELLO 3"), "{t}");
    assert!(m.debogueur_variables().iter().any(|v| v.nom == "I" && v.valeur == "4"), "{:?}", m.debogueur_variables());
}

#[test]
fn exemples_du_paquet() {
    let racine = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../packaging/linux-2.0/Biblio");
    for (dossier, langage) in [("BASIC", Langage::Basic), ("Assembleur", Langage::Assembleur)] {
        for f in std::fs::read_dir(racine.join(dossier)).unwrap().flatten() {
            let texte = std::fs::read_to_string(f.path()).unwrap();
            let mut m = machine();
            let r = m.injecter(&DocumentEditeur { titre: f.file_name().to_string_lossy().into(), chemin: Some(f.path()), langage: langage.clone(), texte });
            assert!(r.is_ok(), "{} : {r:?}", f.path().display());
        }
    }
    // la cassette d'exemple de la v1.0 se charge par la bibliothèque
    let wav = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/hello.wav");
    let mut m = machine();
    let p = Programme { nom: "hello.wav".into(), chemin: Some(wav.clone()), octets: std::fs::read(&wav).unwrap() };
    m.jouer(&p).unwrap();
    assert!(m.jusqua_attente(20_000_000));
    assert!(m.texte_ecran().contains("HELLO WORLD"), "{}", m.texte_ecran());
}
