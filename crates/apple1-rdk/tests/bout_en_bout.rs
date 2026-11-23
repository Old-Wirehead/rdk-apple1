//! Essais de bout en bout de l'Apple 1 2.0 par les pilotes de
//! RDK-Interface-Commune (sans fenêtre).

use apple1_rdk::{MachineApple1, FREQUENCE_CASSETTE};
use rdk_pilotes::pilotes::{
    biblio_memoire::BiblioMemoire, cassette_machine::CassetteMachine,
    clavier_machine::ClavierMachine, debogueur_memoire::DebogueurMemoire,
    debogueur_processeur::DebogueurProcesseur, desassembleur_memoire::DesassembleurMemoire,
    dump_memoire::DumpMemoire, editeur_machine::EditeurMachine, editeur_memoire::EditeurMemoire,
    machine_etat::MachineEtatPilote, video_couleur::VideoCouleur, video_numerique::VideoNumerique,
};
use rdk_pilotes::*;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

/// Un dossier de kit neuf par machine (les réglages des cartes y sont écrits).
fn dossier_neuf() -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let d = std::env::temp_dir().join(format!(
        "rdk-apple1-essai-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst)
    ));
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
    ToucheHote {
        nom: nom.into(),
        caractere: c,
        ..ToucheHote::default()
    }
}

fn taper(m: &mut MachineApple1, texte: &str) {
    for c in texte.chars() {
        let t = if c == '\n' {
            touche("Entrée", None)
        } else {
            touche(&c.to_string(), Some(c))
        };
        m.touche_machine(&t, true);
        m.touche_machine(&t, false);
        // une touche à la fois, comme un humain
        m.jusqua_attente(200_000);
    }
}

fn doc(langage: Langage, texte: &str) -> DocumentEditeur {
    DocumentEditeur {
        titre: "essai".into(),
        chemin: None,
        langage,
        texte: texte.into(),
    }
}

#[test]
fn identite_et_manifeste() {
    let m = machine();
    assert_eq!(m.machine_nom(), "Apple 1");
    // titre propre à la 2.0 (rdk-apple1#26)
    assert_eq!(m.machine_titre_fenetre(), "Retro Dev Kit Apple 1 2.0.0");
    assert_eq!(m.machine_version(), "2.0.0");
    assert_eq!(
        m.manifeste_moniteurs(),
        vec![Moniteur::Monochrome, Moniteur::Couleur]
    );
    assert!(m.couleur_disponible());
    let pal = m.palette();
    assert_eq!(pal.len(), 16);
    assert_eq!(
        (pal[0].nom.as_str(), pal[15].nom.as_str(), pal[15].gris),
        ("noir", "blanc", 255)
    );
    let ids: Vec<String> = m
        .manifeste_boutons_peripheriques()
        .into_iter()
        .map(|b| b.id)
        .collect();
    for id in [
        apple1_rdk::BOUTON_ENREGISTRER,
        apple1_rdk::BOUTON_GEN2,
        apple1_rdk::BOUTON_PLAB,
        apple1_rdk::BOUTON_ASCII,
        apple1_rdk::BOUTON_SAUVER,
        apple1_rdk::BOUTON_REGLAGES_ECRAN,
        apple1_rdk::BOUTON_ECRAN_VINTAGE,
        apple1_rdk::BOUTON_CHARGEMENT_CASSETTE,
        apple1_rdk::BOUTON_EXEMPLES,
    ] {
        assert!(ids.iter().any(|i| i == id), "{id} : {ids:?}");
    }
    assert!(m
        .manifeste_dossiers_bibliotheque()
        .iter()
        .any(|d| d == "GEN2 COLOR"));
    assert!(m.manifeste_clavier_au_demarrage());
    assert!(m.manifeste_a_programmateur_prom());
    let c = m
        .manifeste_clavier()
        .expect("clavier dessiné par le manifeste");
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
    assert!(
        m.texte_ecran().contains("FF00: D8 58 A0 7F 8C 12 D0 A9"),
        "{}",
        m.texte_ecran()
    );
    let mut m2 = m;
    let img = m2.image(Moniteur::Monochrome).expect("image monochrome");
    // 2.0 : image calculée à 3 fois 280 × 192 (écran vintage de la v1.0)
    assert_eq!((img.largeur, img.hauteur), (840, 576));
    assert!(
        m2.image(Moniteur::Couleur).is_none(),
        "sans carte, le moniteur couleur n'a pas d'image"
    );
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
    let r = m
        .editeur_lancer(&doc(Langage::Assembleur, src))
        .expect("lancer");
    assert!(r.contains("$0300"), "{r}");
    assert!(m.jusqua_attente(5_000_000));
    assert!(m.texte_ecran().contains("HI"), "{}", m.texte_ecran());
    let l = m.desassembler(0x0300).unwrap();
    assert!(
        l.texte.contains("LDX") && l.texte.starts_with("start"),
        "{}",
        l.texte
    );
    assert_eq!(m.desassembleur_depart(), 0x0300);
    assert!(m.desassembler(0xFF00).unwrap().texte.starts_with("RESET"));
}

#[test]
fn erreurs_de_l_editeur() {
    let mut m = machine();
    assert!(m
        .editeur_lancer(&doc(Langage::Assembleur, "  .org $0300\n  LDQ #1\n"))
        .is_err());
    assert!(m
        .injecter(&doc(Langage::Basic, "10 PRINT \"A\"\n20 LIST\n"))
        .is_err());
    assert!(m.editeur_lancer(&doc(Langage::Texte, "x")).is_err());
    assert!(!m.etat().erreurs.is_empty());
}

#[test]
fn basic_g2_integre_tape_le_listing() {
    // v1.0 : RDK G2 Basic par défaut, listing tapé (E000R, lignes), puis RUN
    let mut m = machine();
    assert!(m.options().basic_g2);
    let r = m
        .editeur_lancer(&doc(
            Langage::Basic,
            "10 FOR I = 1 TO 3\n20 PRINT \"HELLO \"; I\n30 NEXT I\n40 END\n",
        ))
        .expect("lancer");
    assert!(r.contains("RDK G2 Basic") && r.contains("RUN"), "{r}");
    assert!(m.g2_present());
    assert!(m.jusqua_attente(80_000_000));
    let t = m.texte_ecran();
    assert!(t.contains("HELLO 1") && t.contains("HELLO 3"), "{t}");
    assert!(
        m.debogueur_variables()
            .iter()
            .any(|v| v.nom == "I" && v.valeur == "4"),
        "{:?}",
        m.debogueur_variables()
    );
    // ⬇ RAM : tapé sans RUN
    let mut m = machine();
    assert!(m.injection_possible(&Langage::Basic));
    let r = m
        .injecter(&doc(Langage::Basic, "10 PRINT \"OK\"\n"))
        .expect("injecter");
    assert!(r.contains("RUN ou LIST"), "{r}");
    assert!(m.jusqua_attente(30_000_000));
    m.taper("LIST\n");
    assert!(m.jusqua_attente(30_000_000));
    assert!(
        m.texte_ecran().contains("10 PRINT \"OK\""),
        "{}",
        m.texte_ecran()
    );
}

/// ROM trouvées comme la partie commune les transmet (0.1.5).
fn roms(
    m: &mut MachineApple1,
    octets: Option<Vec<u8>>,
) -> (
    pilotes::rom_machine::JeuRom,
    Vec<pilotes::rom_machine::RomTrouvee>,
) {
    use pilotes::rom_machine::{PresenceRom, RomTrouvee};
    let jeu = m.manifeste_roms().remove(0);
    let attendue = jeu.roms[0].clone();
    assert_eq!(
        (attendue.fichier.as_str(), attendue.taille),
        ("integer_basic.bin", 4096)
    );
    let t = match octets {
        Some(o) => RomTrouvee {
            attendue,
            chemin: None,
            presence: PresenceRom::Presente,
            octets: o,
        },
        None => RomTrouvee {
            attendue,
            chemin: None,
            presence: PresenceRom::Absente,
            octets: Vec::new(),
        },
    };
    (jeu, vec![t])
}

/// Faux BASIC (jamais l'original) : juste la signature reconnue.
fn faux_basic() -> Vec<u8> {
    let mut o = vec![0xEAu8; 4096];
    o[0] = 0x4C;
    o[0x86B..0x86F].copy_from_slice(&[0x85, 0xDC, 0x84, 0xDD]);
    o
}

#[test]
fn basic_d_origine_sans_copie_de_l_utilisateur() {
    use pilotes::rom_machine::{ModeRom, RomMachine};
    let mut m = machine();
    assert_eq!(m.mode_rom(), ModeRom::Emulation);
    let (jeu, absente) = roms(&mut m, None);
    let v = m.rom_identifier(&jeu, &absente);
    assert!(
        !v.reconnues && v.message.contains("integer_basic.bin"),
        "{}",
        v.message
    );
    // sans copie : « ROM réelle » retombe sur l'émulation (RDK G2 Basic)
    let r = m.rom_appliquer(ModeRom::RomReelle, &absente).unwrap();
    assert!(r.contains("RDK G2 Basic"), "{r}");
    assert_eq!(m.mode_rom(), ModeRom::Emulation);
    assert!(m.options().basic_g2);
    // un fichier de la bonne taille mais sans la signature est refusé
    let (_, autre) = roms(&mut m, Some(vec![0u8; 4096]));
    assert!(!m.rom_identifier(&jeu, &autre).reconnues);
    // RDK G2 Basic n'est pas pris pour l'original
    assert!(MachineApple1::reconnaitre_basic(&[0u8; 100]).is_err());
}

#[test]
fn rom_reelle_avec_une_copie_reconnue() {
    use pilotes::rom_machine::{ModeRom, RomMachine};
    let mut m = machine();
    let (jeu, trouvee) = roms(&mut m, Some(faux_basic()));
    assert!(m.rom_identifier(&jeu, &trouvee).reconnues);
    let r = m.rom_appliquer(ModeRom::RomReelle, &trouvee).unwrap();
    assert!(r.contains("ROM réelle"), "{r}");
    assert_eq!(m.mode_rom(), ModeRom::RomReelle);
    assert!(!m.options().basic_g2);
    let r = m.rom_appliquer(ModeRom::Emulation, &[]).unwrap();
    assert!(r.contains("Émulation"), "{r}");
    assert!(m.options().basic_g2);
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
    // avec l'écran vintage (défaut de la v1.0) : image calculée à 3 fois
    assert_eq!(m.image(Moniteur::Couleur).unwrap().largeur, 840);
    // sans : l'image native, pixels purs
    m.options_mut().ecran.actif = false;
    let img = m.image(Moniteur::Couleur).expect("image GEN2");
    assert_eq!((img.largeur, img.hauteur), (280, 192));
    let magenta = m.palette()[1].rvb;
    assert_eq!(
        &img.pixels[140 * 280 + 100][..3],
        &magenta[..],
        "pixel LORES magenta"
    );
    assert!(
        m.etat()
            .complements
            .iter()
            .any(|c| c.0 == "GEN2" && c.1.contains("LORES")),
        "{:?}",
        m.etat().complements
    );
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
    assert_eq!((img.largeur, img.hauteur), (768, 576));
    assert!(m.etat().complements.iter().any(|c| c.0 == "P-LAB"));
    // NMI et CodeTank (absent : message d'aide)
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_PLAB_NMI);
    assert!(m.coeur().board.config.plab_nmi);
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_CODETANK);
    assert!(
        m.etat().erreurs.iter().any(|e| e.contains("Biblio/P-LAB")),
        "{:?}",
        m.etat().erreurs
    );
    // les deux cartes : un bouton choisit la source du moniteur couleur
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_GEN2);
    assert_eq!(m.image(Moniteur::Couleur).unwrap().largeur, 840);
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_SOURCE);
    assert_eq!(m.image(Moniteur::Couleur).unwrap().largeur, 768);
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
    let p = Programme {
        nom: "pomme.txt".into(),
        chemin: Some(f.clone()),
        octets: std::fs::read(&f).unwrap(),
    };
    let mut m = machine();
    assert!(m.double_clic_charge(&p));
    m.jouer(&p).unwrap();
    assert!(m.jusqua_attente(50_000_000));
    let premiere = std::fs::read_to_string(&f)
        .unwrap()
        .lines()
        .find(|l| !l.trim().is_empty())
        .unwrap()
        .trim()
        .to_ascii_uppercase();
    assert!(m.texte_ecran().contains(&premiere), "{}", m.texte_ecran());
    // un texte ordinaire reste pour l'éditeur
    let t = Programme {
        nom: "notes.txt".into(),
        chemin: Some("notes.txt".into()),
        octets: b"Bonjour, voici mes notes sur le programme.\n".to_vec(),
    };
    assert!(!m.double_clic_charge(&t));
}

#[test]
fn memoire_rom_en_lecture_seule() {
    let mut m = machine();
    assert_eq!(m.dump_plage(), 0..=0xFFFF);
    assert_eq!(m.dump_lire(0xFF00), Some(0xD8));
    assert_eq!(
        m.dump_ecrire(0xFF00, 0),
        Err(ErreurPilote::LectureSeule(0xFF00))
    );
    m.dump_ecrire(0x0300, 0x42).unwrap();
    assert_eq!(m.debogueur_lire(0x0300), Some(0x42));
}

#[test]
fn debogueur_pas_a_pas() {
    let mut m = machine();
    m.injecter(&doc(
        Langage::Assembleur,
        "  .org $0300\n  LDA #$12\n  LDX #$34\n  JMP $0300\n",
    ))
    .unwrap();
    m.commande(CommandeExecution::Pause).unwrap();
    m.ecrire_registre("PC", 0x0300).unwrap();
    m.commande(CommandeExecution::Pas).unwrap();
    assert_eq!(m.registres()[0].valeur, 0x12);
    assert_eq!(m.registres()[4].valeur, 0x0302);
    m.commande(CommandeExecution::ExecuterJusqua(0x0304))
        .unwrap();
    assert_eq!(m.registres()[1].valeur, 0x34);
    assert_eq!(m.etat().marche, Marche::Pause);
    m.commande(CommandeExecution::Continuer).unwrap();
    assert_eq!(m.etat().marche, Marche::Marche);
    m.machine_avancer(Duration::from_millis(20));
}

#[test]
fn bibliotheque_vidage_woz_et_play() {
    let mut m = machine();
    let p = Programme {
        nom: "hello.txt".into(),
        chemin: Some(PathBuf::from("hello.txt")),
        octets: b"0300: A9 C1 20 EF FF 4C 1F FF\n".to_vec(),
    };
    // .txt = texte pour l'interface commune : on passe par l'extension .mon
    let p = Programme {
        nom: "hello.mon".into(),
        chemin: Some(PathBuf::from("hello.mon")),
        ..p
    };
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
    m.injecter(&doc(
        Langage::Assembleur,
        "  .org $0300\n  .byte 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16\n",
    ))
    .unwrap();
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_ENREGISTRER);
    m.taper("C100R\n0300.030FW\n");
    assert!(m.jusqua_attente(200_000_000));
    let ech = m.cassette_sortie();
    assert!(
        ech.len() > FREQUENCE_CASSETTE as usize,
        "signal trop court : {}",
        ech.len()
    );
    let mut b = machine();
    b.cassette_entree(&ech);
    b.taper("C100R\n0300.030FR\n");
    assert!(b.jusqua_attente(200_000_000));
    let lu: Vec<u8> = (0x300..0x310).map(|a| b.dump_lire(a).unwrap()).collect();
    assert_eq!(lu, (1..=16).collect::<Vec<u8>>(), "{}", b.texte_ecran());
}

#[test]
fn programmateur_prom_ecrit_l_image() {
    // sans fenêtre (pas de contexte egui) : image Intel HEX de $E000-$EFFF
    let dossier = dossier_neuf();
    let mut m = machine_dans(dossier.clone());
    m.manifeste_ouvrir_programmateur_prom();
    let f = dossier
        .join("Biblio")
        .join("PROM")
        .join("prom-E000-EFFF.hex");
    let t = std::fs::read_to_string(&f).expect("image Intel HEX");
    assert!(t.lines().all(|l| l.starts_with(':')), "{t}");
    assert!(t.trim_end().ends_with(":00000001FF"));
    let octets = apple1_rdk::prom::image_fichier(&f).unwrap();
    assert_eq!(octets.len(), 4096);
    assert_eq!(octets[0], m.dump_lire(0xE000).unwrap());
    // la fenêtre du programmateur (minipro) est ouverte, dessinée dès que
    // l'application a donné son contexte egui
    assert!(m.fenetres().prom.ouvert);
}

#[test]
fn basic_lancer_avec_la_copie_de_l_utilisateur() {
    // ne tourne que si une copie personnelle du BASIC est sur ce poste
    // (jamais dans le dépôt)
    use pilotes::rom_machine::{ModeRom, RomMachine};
    let Some(octets) =
        std::env::var_os(apple1_rdk::VARIABLE_BASIC).and_then(|f| std::fs::read(f).ok())
    else {
        return;
    };
    let mut m = machine();
    let (_, trouvee) = roms(&mut m, Some(octets));
    if m.rom_appliquer(ModeRom::RomReelle, &trouvee).is_err() || m.mode_rom() != ModeRom::RomReelle
    {
        return;
    }
    let r = m
        .editeur_lancer(&doc(
            Langage::Basic,
            "10 FOR I = 1 TO 3\n20 PRINT \"HELLO \"; I\n30 NEXT I\n40 END\n",
        ))
        .expect("lancer");
    assert!(r.contains("Integer BASIC") && r.contains("RUN"), "{r}");
    assert!(!m.g2_present());
    assert!(m.jusqua_attente(80_000_000));
    let t = m.texte_ecran();
    assert!(t.contains("HELLO 1") && t.contains("HELLO 3"), "{t}");
    assert!(
        m.debogueur_variables()
            .iter()
            .any(|v| v.nom == "I" && v.valeur == "4"),
        "{:?}",
        m.debogueur_variables()
    );
}

#[test]
fn exemples_du_paquet() {
    let racine = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../packaging/linux-2.0/Biblio");
    for (dossier, langage) in [
        ("BASIC", Langage::Basic),
        ("Assembleur", Langage::Assembleur),
    ] {
        for f in std::fs::read_dir(racine.join(dossier)).unwrap().flatten() {
            let texte = std::fs::read_to_string(f.path()).unwrap();
            let mut m = machine();
            let r = m.injecter(&DocumentEditeur {
                titre: f.file_name().to_string_lossy().into(),
                chemin: Some(f.path()),
                langage: langage.clone(),
                texte,
            });
            assert!(r.is_ok(), "{} : {r:?}", f.path().display());
        }
    }
    // la cassette d'exemple de la v1.0 se charge par la bibliothèque
    let wav = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/hello.wav");
    let mut m = machine();
    let p = Programme {
        nom: "hello.wav".into(),
        chemin: Some(wav.clone()),
        octets: std::fs::read(&wav).unwrap(),
    };
    m.jouer(&p).unwrap();
    assert!(m.jusqua_attente(20_000_000));
    assert!(
        m.texte_ecran().contains("HELLO WORLD"),
        "{}",
        m.texte_ecran()
    );
}

#[test]
fn exemples_de_la_v1_copies_dans_le_paquet() {
    // construire-paquet.sh copie ces programmes de la v1.0 dans Biblio/
    let racine = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/exemples");
    for (dossier, ext, langage) in [
        ("basic", "bas", Langage::Basic),
        ("asm", "s", Langage::Assembleur),
    ] {
        for f in std::fs::read_dir(racine.join(dossier)).unwrap().flatten() {
            if f.path().extension().and_then(|e| e.to_str()) != Some(ext) {
                continue;
            }
            let texte = std::fs::read_to_string(f.path()).unwrap();
            let mut m = machine();
            m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_GEN2);
            let r = m.injecter(&DocumentEditeur {
                titre: f.file_name().to_string_lossy().into(),
                chemin: Some(f.path()),
                langage: langage.clone(),
                texte,
            });
            // erreur.bas : l'exemple d'erreur du manuel
            assert_eq!(
                r.is_ok(),
                f.file_name() != "erreur.bas",
                "{} : {r:?}",
                f.path().display()
            );
        }
    }
    let barres = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/gen2-barres.txt");
    let p = Programme {
        nom: "gen2-barres.txt".into(),
        chemin: Some(barres.clone()),
        octets: std::fs::read(&barres).unwrap(),
    };
    let mut m = machine();
    assert!(m.double_clic_charge(&p));
    m.charger(&p).unwrap();
    assert_eq!(m.dump_lire(0x0C00), Some(0xAD));
    // livré en .woz dans le paquet 2.0 (rdk-apple1#17) : même chargement
    let woz = Programme {
        nom: "gen2-barres.woz".into(),
        chemin: Some(barres.with_extension("woz")),
        octets: p.octets.clone(),
    };
    let mut m = machine();
    assert!(m.double_clic_charge(&woz));
    m.charger(&woz).unwrap();
    assert_eq!(m.dump_lire(0x0C00), Some(0xAD));
}

#[test]
fn manifeste_0_1_5_formats_roms_configurations() {
    let m = machine();
    assert_eq!(m.manifeste_dossier_bibliotheque(), "Apple 1");
    let f = m.manifeste_formats();
    let woz = f.iter().find(|x| x.extension == "woz").expect(".woz");
    assert!(woz.lecture && woz.ecriture, ".woz en lecture et écriture");
    let txt = f.iter().find(|x| x.extension == "txt").expect(".txt");
    assert!(txt.lecture && !txt.ecriture, "ancien .txt en lecture seule");
    let roms = m.manifeste_roms();
    assert_eq!(roms.len(), 1);
    assert_eq!(roms[0].roms[0].fichier, "integer_basic.bin");
    assert!(!roms[0].roms[0].obligatoire);
    let c = m.manifeste_configurations();
    let noms: Vec<&str> = c.iter().map(|c| c.nom.as_str()).collect();
    assert_eq!(
        noms,
        // la première est celle du premier lancement (rdk-apple1#17, point 4)
        [
            "Apple 1 moniteur vert + couleur GEN2",
            "Apple 1 moniteur vert",
            "Apple 1 + P-LAB"
        ]
    );
    assert!(c[0].a(apple1_rdk::BOUTON_GEN2) && c[2].a(apple1_rdk::BOUTON_PLAB));
    assert!(!c[1].a(apple1_rdk::BOUTON_GEN2) && !c[1].a(apple1_rdk::BOUTON_PLAB));
    assert_eq!(
        m.phosphore_monochrome().map(|p| p.encre),
        Some([0x44, 0xFF, 0x66])
    );
}

fn programme_octets(nom: &str, octets: Vec<u8>) -> Programme {
    Programme {
        nom: nom.into(),
        chemin: Some(nom.into()),
        octets,
    }
}

#[test]
fn intel_hex_et_dump_communs() {
    let bloc = [BlocMemoire {
        adresse: 0x2000,
        octets: (1..=32).collect(),
    }];
    for (nom, contenu) in [
        ("essai.hex", formats::ecrire_intel_hex(&bloc)),
        ("essai.dump", formats::ecrire_dump(&bloc)),
        ("ancien.hex", "2000: 01 02 03 04 05 06 07 08\n".to_string()),
    ] {
        let mut m = machine();
        m.charger(&programme_octets(nom, contenu.into_bytes()))
            .unwrap_or_else(|e| panic!("{nom} : {e}"));
        assert_eq!(m.dump_lire(0x2000), Some(1), "{nom}");
        assert_eq!(m.dump_lire(0x2007), Some(8), "{nom}");
    }
}

#[test]
fn sauver_woz_hex_et_cassette() {
    let dossier = dossier_neuf();
    let mut m = machine_dans(dossier.clone());
    for (i, a) in (0x0300u32..0x0310).enumerate() {
        m.dump_ecrire(a, i as u8 + 1).unwrap();
    }
    for ext in ["woz", "hex", "dump", "wav"] {
        let f = dossier.join(format!("sauve.{ext}"));
        let r = m.sauver(&f, "0300.030F").unwrap();
        assert!(f.is_file(), "{r}");
        let mut b = machine();
        b.charger(&Programme {
            nom: format!("sauve.{ext}"),
            chemin: Some(f.clone()),
            octets: std::fs::read(&f).unwrap(),
        })
        .unwrap_or_else(|e| panic!("{ext} : {e}"));
        let lu: Vec<u8> = (0x300..0x310).map(|a| b.dump_lire(a).unwrap()).collect();
        assert_eq!(lu, (1..=16).collect::<Vec<u8>>(), "{ext}");
    }
    assert!(std::fs::read_to_string(dossier.join("sauve.woz"))
        .unwrap()
        .starts_with("0300:"));
    assert!(m.sauver(&dossier.join("x.woz"), "").is_err());
}

#[test]
fn chargement_par_cassette_en_temps_reel() {
    let mut m = machine();
    m.options_mut().cassette = true;
    m.charger(&programme_octets(
        "petit.woz",
        b"0300: 01 02 03 04 05 06 07 08\n".to_vec(),
    ))
    .unwrap();
    assert!(m.chargement_en_cours());
    for _ in 0..600 {
        if !m.chargement_en_cours() {
            break;
        }
        m.machine_avancer(Duration::from_millis(50));
    }
    assert!(!m.chargement_en_cours(), "{}", m.texte_ecran());
    assert!(m.texte_ecran().contains("C100R"), "{}", m.texte_ecran());
    let lu: Vec<u8> = (0x300..0x308).map(|a| m.dump_lire(a).unwrap()).collect();
    assert_eq!(lu, (1..=8).collect::<Vec<u8>>(), "{}", m.texte_ecran());
}

#[test]
fn ram_etendue_pour_un_programme_haut() {
    let mut m = machine();
    let bloc = [BlocMemoire {
        adresse: 0x7000,
        octets: vec![0x60; 16],
    }];
    m.charger(&programme_octets(
        "haut.hex",
        formats::ecrire_intel_hex(&bloc).into_bytes(),
    ))
    .unwrap();
    assert_eq!(m.dump_lire(0x7000), Some(0x60));
    assert!(m.coeur().board.config.ram_kb >= 32);
}

#[test]
fn bouton_exemples_remplit_la_bibliotheque() {
    let dossier = dossier_neuf();
    let biblio = dossier.join("RDK librairies").join("Apple 1");
    let b = biblio.clone();
    let mut m = machine_dans(dossier).avec_bibliotheque(move || b.clone());
    m.manifeste_basculer_peripherique(apple1_rdk::BOUTON_EXEMPLES);
    assert!(biblio.join("Art ASCII").join("pomme.txt").is_file());
    assert_eq!(m.bibliotheque(), biblio);
}

/// v1.0 : état de la machine enregistré puis rechargé (menu 📸 État, .a1state).
#[test]
fn etat_de_la_machine_aller_retour() {
    let mut m = machine();
    assert_eq!(m.etat_extension().as_deref(), Some("a1state"));
    m.dump_ecrire(0x0300, 0x42).unwrap();
    taper(&mut m, "300\n");
    let o = m.etat_enregistrer("essai").unwrap();
    // l'état est celui de la v1.0 : lisible par apple1-core
    assert_eq!(apple1_core::state::peek_info(&o).unwrap().title, "essai");
    let mut n = machine();
    assert_eq!(n.dump_lire(0x0300), Some(0x00));
    let msg = n.etat_charger(&o).unwrap();
    assert!(msg.contains("essai"), "{msg}");
    assert_eq!(n.dump_lire(0x0300), Some(0x42));
    assert!(n.texte_ecran().contains("0300: 42"), "{}", n.texte_ecran());
    assert!(n.etat_charger(b"pas un etat").is_err());
}

/// v1.0 : réglages par programme (programs.json) appliqués par ▶ Play.
#[test]
fn reglages_par_programme_appliques() {
    use apple1_rdk::programmes;
    use apple1_rdk::v1::params::ProgParams;
    let d = dossier_neuf();
    let f = d.join("bonjour.bin");
    // LDA #'A' ; JSR ECHO ; JMP WOZMON
    let octets = vec![0xA9, 0xC1, 0x20, 0xEF, 0xFF, 0x4C, 0x1F, 0xFF];
    std::fs::write(&f, &octets).unwrap();
    // sans réglages : .bin en $0300
    let mut m = machine();
    let p = Programme {
        nom: "bonjour.bin".into(),
        chemin: Some(f.clone()),
        octets: octets.clone(),
    };
    m.charger(&p).unwrap();
    assert_eq!(m.dump_lire(0x0300), Some(0xA9));
    assert_eq!(m.dernier_chemin(), Some(f.as_path()));
    // réglages : chargé et lancé en $0800
    let r = ProgParams {
        ranges: "800.807".into(),
        load_addr: "800".into(),
        run_addr: "800".into(),
        run_cmd: "800R".into(),
        ..ProgParams::default()
    };
    programmes::enregistrer(&f, r).unwrap();
    let mut m = machine();
    m.jouer(&p).unwrap();
    assert_eq!(m.dump_lire(0x0800), Some(0xA9));
    assert!(m.jusqua_attente(5_000_000));
    assert!(m.texte_ecran().contains("800R"), "{}", m.texte_ecran());
    let _ = std::fs::remove_dir_all(&d);
}

/// v1.0 : « Tester le dernier build » lance un source assembleur du dépôt.
#[test]
fn lancer_un_source_assembleur() {
    let d = dossier_neuf();
    let f = d.join("essai.s");
    std::fs::write(
        &f,
        "        .org $0280\nDEBUT:  LDA #$C2\n        JSR $FFEF\n        JMP $FF1F\n",
    )
    .unwrap();
    let mut m = machine();
    let msg = m.lancer_fichier(&f).unwrap();
    assert!(msg.contains("280R"), "{msg}");
    assert_eq!(m.dump_lire(0x0280), Some(0xA9));
    assert!(m.jusqua_attente(5_000_000));
    assert!(m.texte_ecran().contains("280R"), "{}", m.texte_ecran());
    let _ = std::fs::remove_dir_all(&d);
}

/// 0.1.7 : les textes de la machine ont leur traduction (la langue
/// courante est globale : l'essai lit la table sans la changer, pour ne pas
/// gêner les essais qui tournent en même temps).
#[test]
fn textes_de_la_machine_traduits() {
    let m = machine();
    let t = apple1_rdk::i18n::table();
    let libelles: Vec<String> = m
        .manifeste_boutons_peripheriques()
        .into_iter()
        .map(|b| b.libelle)
        .collect();
    assert!(libelles.contains(&"Réglages".to_string()), "{libelles:?}");
    assert_eq!(t.dans(Langue::En, "Réglages"), "Settings");
    assert_eq!(t.dans(Langue::De, "Oscilloscope"), "Oszilloskop");
    for b in m.manifeste_boutons_peripheriques() {
        for l in rdk_pilotes::langue::TOUTES {
            // sigles et noms propres (GEN2, P-LAB, NMI, ASCII, 50 Hz, CodeTank, GitHub) : identiques
            let nom_propre = !b.libelle.chars().any(|c| c.is_lowercase())
                || ["50 Hz", "CodeTank", "GitHub"].contains(&b.libelle.as_str());
            assert!(
                t.traduit(l, &b.libelle) || nom_propre,
                "{:?} : {}",
                l,
                b.libelle
            );
        }
    }
}

/// rdk-apple1#17, point 1 : ▶ Lancer pose le programme comme la v1.0
/// (tokenisé, LOMEM/HIMEM d'après sa taille) au lieu de le taper dans 2 Ko.
#[test]
fn lancer_un_gros_basic_comme_la_v1() {
    let mut m = machine();
    // 220 lignes d'environ 40 octets : plus de 8 Ko, impossible en 8 Ko
    let mut texte = String::new();
    for i in 1..=220 {
        texte.push_str(&format!(
            "{} PRINT \"LIGNE NUMERO {} DU GROS PROGRAMME\";\n",
            i * 5,
            i
        ));
    }
    texte.push_str("2000 PRINT \"FIN DU PROGRAMME\"\n2010 END\n");
    let r = m
        .editeur_lancer(&doc(Langage::Basic, &texte))
        .expect("lancer");
    assert!(
        r.contains("HIMEM $3000") && r.contains("12 Ko") && r.contains("RUN"),
        "{r}"
    );
    // le terminal affiche à la vitesse de l'Apple 1 : on regarde le début
    m.jusqua_attente(20_000_000);
    let t = m.texte_ecran();
    assert!(
        t.contains("LIGNE NUMERO 1 DU GROS") && !t.contains("MEM FULL"),
        "{t}"
    );
}

/// rdk-apple1#17, point 2 : un document assembleur vide n'est pas lancé.
#[test]
fn assembleur_vide_refuse() {
    let mut m = machine();
    let e = m
        .editeur_lancer(&doc(Langage::Assembleur, "; rien\n"))
        .unwrap_err();
    assert!(e.to_string().contains("aucune instruction"), "{e}");
}

/// rdk-apple1#17 : description de la v1.0 dans la bibliothèque, signets du
/// Dump, étiquettes pour « Aller », identifiant stable et titre.
#[test]
fn reprises_de_la_v1() {
    let m = machine();
    let prog = |nom: &str, t: &str| Programme {
        nom: nom.into(),
        chemin: Some(PathBuf::from(nom)),
        octets: t.as_bytes().to_vec(),
    };
    assert_eq!(
        m.biblio_description(&prog("a.bas", "10 PRINT 1\n20 END\n"))
            .as_deref(),
        Some("Source BASIC · 2 lignes · lancer : RUN")
    );
    let d = m
        .biblio_description(&prog("h.woz", "0300: A9 C1 20 EF FF 60\n"))
        .unwrap();
    assert!(d.contains("0300.0305") && d.contains("300R"), "{d}");
    let a = m
        .biblio_description(&prog("x.asm", "  .org $0300\n  rts\n"))
        .unwrap();
    assert!(a.contains("0300.0300") && a.contains("300R"), "{a}");
    assert!(m.dump_signets().iter().any(|(n, a)| n == "ZP" && *a == 0));
    assert_eq!(m.desassembleur_symbole("reset"), Some(0xFF00));
    assert_eq!(m.machine_id(), "apple-1");
    assert!(!m.machine_titre_fenetre().contains("Original Sin"));
    assert!(m.manifeste_configurations()[0].a(rdk_pilotes::configuration::ids::MONITEUR_COULEUR));
}

/// rdk-apple1#17, point 3 : une copie du BASIC sous un autre nom est
/// signalée (faux BASIC : seulement la signature, jamais l'original).
#[test]
fn basic_mal_nomme_signale() {
    let d = dossier_neuf().join("ROM");
    std::fs::create_dir_all(&d).unwrap();
    assert_eq!(apple1_rdk::basic_sous_un_autre_nom(&d), None);
    std::fs::write(d.join("apple1basic.bin"), faux_basic()).unwrap();
    assert_eq!(
        apple1_rdk::basic_sous_un_autre_nom(&d).as_deref(),
        Some("apple1basic.bin")
    );
}
