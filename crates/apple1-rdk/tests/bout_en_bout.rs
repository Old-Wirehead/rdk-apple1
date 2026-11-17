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
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

/// Un dossier de kit neuf par machine (les réglages des cartes y sont écrits).
fn dossier_neuf() -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let d = std::env::temp_dir().join(format!("rdk-apple1-essai-{}-{}", std::process::id(), N.fetch_add(1, Ordering::SeqCst)));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn machine() -> MachineApple1 {
    machine_dans(dossier_neuf())
}

fn machine_dans(dossier: PathBuf) -> MachineApple1 {
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
    assert_eq!(m.manifeste_moniteurs(), vec![Moniteur::Monochrome, Moniteur::Couleur]);
    assert!(m.couleur_disponible());
    let pal = m.palette();
    assert_eq!(pal.len(), 16);
    assert_eq!((pal[0].nom.as_str(), pal[15].nom.as_str(), pal[15].gris), ("noir", "blanc", 255));
    let ids: Vec<String> = m.manifeste_boutons_peripheriques().into_iter().map(|b| b.id).collect();
    for id in [apple1_rdk::BOUTON_ENREGISTRER, apple1_rdk::BOUTON_GEN2, apple1_rdk::BOUTON_PLAB, apple1_rdk::BOUTON_BASIC_G2] {
        assert!(ids.iter().any(|i| i == id), "{id} : {ids:?}");
    }
    assert!(m.manifeste_dossiers_bibliotheque().iter().any(|d| d == "GEN2 COLOR"));
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
    assert!(m2.image(Moniteur::Couleur).is_none(), "sans carte, le moniteur couleur n'a pas d'image");
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
fn basic_g2_integre_tape_le_listing() {
    // v1.0 : RDK G2 Basic par défaut, listing tapé (E000R, lignes), puis RUN
    let mut m = machine();
    assert!(m.options().basic_g2);
    let r = m.editeur_lancer(&doc(Langage::Basic, "10 FOR I = 1 TO 3\n20 PRINT \"HELLO \"; I\n30 NEXT I\n40 END\n")).expect("lancer");
    assert!(r.contains("RDK G2 Basic") && r.contains("RUN"), "{r}");
    assert!(m.g2_present());
    assert!(m.jusqua_attente(80_000_000));
    let t = m.texte_ecran();
    assert!(t.contains("HELLO 1") && t.contains("HELLO 3"), "{t}");
    assert!(m.debogueur_variables().iter().any(|v| v.nom == "I" && v.valeur == "4"), "{:?}", m.debogueur_variables());
    // ⬇ RAM : tapé sans RUN
    let mut m = machine();
    assert!(m.injection_possible(&Langage::Basic));
    let r = m.injecter(&doc(Langage::Basic, "10 PRINT \"OK\"\n")).expect("injecter");
    assert!(r.contains("RUN ou LIST"), "{r}");
    assert!(m.jusqua_attente(30_000_000));
    m.taper("LIST\n");
    assert!(m.jusqua_attente(30_000_000));
    assert!(m.texte_ecran().contains("10 PRINT \"OK\""), "{}", m.texte_ecran());
}

#[test]
fn basic_d_origine_sans_copie_de_l_utilisateur() {
    std::env::remove_var(apple1_rdk::VARIABLE_BASIC);
    let mut m = machine();
    if m.chercher_basic().is_some() {
        return; // une copie personnelle existe sur ce poste
    }
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_BASIC_G2);
    assert!(!m.options().basic_g2);
    let e = m.editeur_lancer(&doc(Langage::Basic, "10 PRINT 1\n")).unwrap_err();
    assert!(e.to_string().contains("integer_basic.bin"), "{e}");
}

#[test]
fn carte_gen2_sur_le_moniteur_couleur() {
    let dossier = dossier_neuf();
    let mut m = machine_dans(dossier.clone());
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_GEN2);
    assert!(m.coeur().board.config.color_card);
    assert!(m.etat().peripheriques_actifs.iter().any(|p| p == "GEN2"));
    // LORES plein écran, page 1 : couleur 1 (magenta) partout
    let prog = "  .org $0300\n  LDA $C250\n  LDA $C252\n  LDA $C254\n  LDA $C256\n  LDA #$11\n  LDX #0\nl: STA $0400,X\n  STA $0500,X\n  STA $0600,X\n  STA $0700,X\n  INX\n  BNE l\n  JMP $FF1F\n";
    m.editeur_lancer(&doc(Langage::Assembleur, prog)).unwrap();
    assert!(m.jusqua_attente(10_000_000));
    m.machine_avancer(Duration::from_millis(40));
    let img = m.image(Moniteur::Couleur).expect("image GEN2");
    assert_eq!((img.largeur, img.hauteur), (280, 192));
    let magenta = m.palette()[1].rvb;
    assert_eq!(&img.pixels[140 * 280 + 100][..3], &magenta[..], "pixel LORES magenta");
    assert!(m.etat().complements.iter().any(|c| c.0 == "GEN2" && c.1.contains("LORES")), "{:?}", m.etat().complements);
    // le terminal reste sur le moniteur monochrome
    assert!(m.image(Moniteur::Monochrome).is_some());
    // réglage gardé dans le dossier du kit
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_GEN2_50HZ);
    let m2 = machine_dans(dossier);
    assert!(m2.options().gen2 && m2.options().gen2_50hz);
    assert!(m2.coeur().board.config.color_card && m2.coeur().board.config.color_50hz);
}

#[test]
fn carte_plab_sur_le_moniteur_couleur() {
    let mut m = machine();
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_PLAB);
    assert!(m.coeur().board.config.plab_card);
    let img = m.image(Moniteur::Couleur).expect("image P-LAB");
    assert_eq!((img.largeur, img.hauteur), (256, 192));
    assert!(m.etat().complements.iter().any(|c| c.0 == "P-LAB"));
    // NMI et CodeTank (absent : message d'aide)
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_PLAB_NMI);
    assert!(m.coeur().board.config.plab_nmi);
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_CODETANK);
    assert!(m.etat().erreurs.iter().any(|e| e.contains("Biblio/P-LAB")), "{:?}", m.etat().erreurs);
    // les deux cartes : un bouton choisit la source du moniteur couleur
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_GEN2);
    assert_eq!(m.image(Moniteur::Couleur).unwrap().largeur, 280);
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_SOURCE);
    assert_eq!(m.image(Moniteur::Couleur).unwrap().largeur, 256);
}

#[test]
fn codetank_lu_dans_la_bibliotheque() {
    let dossier = dossier_neuf();
    let d = dossier.join("Biblio").join(apple1_rdk::DOSSIER_PLAB);
    std::fs::create_dir_all(&d).unwrap();
    let mut img = vec![0xEA; 16384];
    img[0] = 0x42;
    std::fs::write(d.join("essai.bin"), &img).unwrap();
    let mut m = machine_dans(dossier);
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_PLAB);
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_CODETANK);
    assert_eq!(m.dump_lire(0x4000), Some(0x42), "{:?}", m.etat().erreurs);
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_CODETANK);
    assert!(m.coeur().board.config.codetank.is_none());
}

#[test]
fn art_ascii_de_la_bibliotheque() {
    let f = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../apple1-gui/ascii/pomme.txt");
    let p = Programme { nom: "pomme.txt".into(), chemin: Some(f.clone()), octets: std::fs::read(&f).unwrap() };
    let mut m = machine();
    assert!(m.double_clic_charge(&p));
    m.jouer(&p).unwrap();
    assert!(m.jusqua_attente(50_000_000));
    let premiere = std::fs::read_to_string(&f).unwrap().lines().find(|l| !l.trim().is_empty()).unwrap().trim().to_ascii_uppercase();
    assert!(m.texte_ecran().contains(&premiere), "{}", m.texte_ecran());
    // un texte ordinaire reste pour l'éditeur
    let t = Programme { nom: "notes.txt".into(), chemin: Some("notes.txt".into()), octets: b"Bonjour, voici mes notes sur le programme.\n".to_vec() };
    assert!(!m.double_clic_charge(&t));
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
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_BASIC_G2);
    let r = m.editeur_lancer(&doc(Langage::Basic, "10 FOR I = 1 TO 3\n20 PRINT \"HELLO \"; I\n30 NEXT I\n40 END\n")).expect("lancer");
    assert!(r.contains("Integer BASIC") && r.contains("RUN"), "{r}");
    assert!(!m.g2_present());
    assert!(m.jusqua_attente(80_000_000));
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

#[test]
fn exemples_de_la_v1_copies_dans_le_paquet() {
    // construire-paquet.sh copie ces programmes de la v1.0 dans Biblio/
    let racine = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/exemples");
    for (dossier, ext, langage) in [("basic", "bas", Langage::Basic), ("asm", "s", Langage::Assembleur)] {
        for f in std::fs::read_dir(racine.join(dossier)).unwrap().flatten() {
            if f.path().extension().and_then(|e| e.to_str()) != Some(ext) {
                continue;
            }
            let texte = std::fs::read_to_string(f.path()).unwrap();
            let mut m = machine();
            m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_GEN2);
            let r = m.injecter(&DocumentEditeur { titre: f.file_name().to_string_lossy().into(), chemin: Some(f.path()), langage: langage.clone(), texte });
            // erreur.bas : l'exemple d'erreur du manuel
            assert_eq!(r.is_ok(), f.file_name() != "erreur.bas", "{} : {r:?}", f.path().display());
        }
    }
    let barres = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/gen2-barres.txt");
    let p = Programme { nom: "gen2-barres.txt".into(), chemin: Some(barres.clone()), octets: std::fs::read(&barres).unwrap() };
    let mut m = machine();
    assert!(m.double_clic_charge(&p));
    m.charger(&p).unwrap();
    assert_eq!(m.dump_lire(0x0C00), Some(0xAD));
}
