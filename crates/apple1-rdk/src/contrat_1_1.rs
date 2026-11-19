//! Contrat des pilotes 1.1.0 (RDK-Interface-Commune 0.1.10) côté Apple 1 :
//!
//! - **points d'arrêt** de l'Éditeur et du Débogueur
//!   (`DebogueurProcesseur::points_arret`) : adresses, lignes BASIC (numéro
//!   en tête de la ligne) et lignes d'un source assembleur (adresse de
//!   l'instruction au dernier assemblage), posés dans le débogueur de la
//!   v1.0 (`Debugger::breaks`, `Debugger::basic_breaks`) ;
//! - **réglages d'écran** (cases « vintage », tirettes, phosphore) montrés
//!   dans les fenêtres d'écran communes (`VideoNumerique::reglages_ecran`),
//!   les mêmes que la fenêtre « Écran de l'Apple 1 » ; l'écran vintage est
//!   **allumé par défaut**, comme la v1.0 ;
//! - **répertoire des routines de la ROM** (issue 24 de l'interface) :
//!   Woz Monitor, ACI et Integer BASIC.

use crate::crt::{Phosphore, ReglagesEcran};
use crate::i18n::tr;
use apple1_core::asm6502::Assembly;
use rdk_pilotes::{Langage, Moniteur, PointArret, ReglageEcran, RoutineRom, ValeurReglage};
use std::collections::BTreeSet;

/// Numéro de ligne BASIC en tête de `texte` (`10 PRINT` → 10).
pub fn ligne_basic(texte: &str) -> Option<u16> {
    let t = texte.trim_start();
    let fin = t.find(|c: char| !c.is_ascii_digit()).unwrap_or(t.len());
    t[..fin].parse().ok()
}

/// Lignes du texte principal (à partir de 1) qui produisent des octets, avec
/// l'adresse de leur premier octet.
pub fn adresses_des_lignes(a: &Assembly) -> Vec<(usize, u16)> {
    let mut v: Vec<(usize, u16)> = Vec::new();
    for l in a.listing.iter().filter(|l| l.file == 0 && !l.bytes.is_empty()) {
        if !v.iter().any(|(n, _)| *n == l.line) {
            v.push((l.line, l.addr));
        }
    }
    v
}

/// Points d'arrêt reçus de l'interface, rangés pour le débogueur.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Arrets {
    pub adresses: BTreeSet<u16>,
    pub basic: BTreeSet<u16>,
    /// Lignes d'un source assembleur, traduites à chaque assemblage.
    pub asm: Vec<usize>,
}

impl Arrets {
    pub fn depuis(points: &[PointArret]) -> Arrets {
        let mut a = Arrets::default();
        for p in points {
            match p {
                PointArret::Adresse(x) if *x <= 0xFFFF => {
                    a.adresses.insert(*x as u16);
                }
                PointArret::Ligne { langage: Langage::Basic, texte, .. } => {
                    if let Some(n) = ligne_basic(texte) {
                        a.basic.insert(n);
                    }
                }
                PointArret::Ligne { langage: Langage::Assembleur, ligne, .. } => a.asm.push(*ligne),
                _ => {}
            }
        }
        a
    }

    /// Adresses d'arrêt : celles du Débogueur, plus les lignes assembleur
    /// connues du dernier assemblage (`lignes`).
    pub fn adresses_completes(&self, lignes: &[(usize, u16)]) -> BTreeSet<u16> {
        let mut s = self.adresses.clone();
        s.extend(self.asm.iter().filter_map(|n| lignes.iter().find(|(l, _)| l == n).map(|(_, a)| *a)));
        s
    }
}

fn case(id: &str, libelle: &str, bulle: &str, v: bool) -> ReglageEcran {
    ReglageEcran { id: id.into(), libelle: libelle.into(), info_bulle: bulle.into(), valeur: ValeurReglage::Case(v) }
}

fn tirette(id: &str, libelle: &str, bulle: &str, v: f32) -> ReglageEcran {
    ReglageEcran { id: id.into(), libelle: libelle.into(), info_bulle: bulle.into(), valeur: ValeurReglage::Curseur { valeur: v, min: 0.0, max: 1.0 } }
}

/// Réglages d'écran montrés dans la fenêtre de `moniteur`. Écran vintage
/// éteint : seulement sa case (et le phosphore du moniteur monochrome).
pub fn reglages(moniteur: Moniteur, r: &ReglagesEcran, phosphore: Phosphore) -> Vec<ReglageEcran> {
    let mut v = Vec::new();
    if moniteur == Moniteur::Monochrome {
        v.push(ReglageEcran {
            id: "phosphore".into(),
            libelle: tr("Phosphore du moniteur :").into(),
            info_bulle: String::new(),
            valeur: ValeurReglage::Choix {
                options: Phosphore::TOUS.iter().map(|p| p.nom().to_string()).collect(),
                choisi: Phosphore::TOUS.iter().position(|p| *p == phosphore).unwrap_or(0),
            },
        });
    }
    v.push(case("actif", tr("Écran vintage (screening)"), tr("Imitation d'un moniteur à tube des années 1970, comme la photo d'un vrai Apple 1."), r.actif));
    if !r.actif {
        return v;
    }
    v.push(case("lignes", tr("Séparation des lignes de balayage"), tr("Une fine ligne sombre sous chaque ligne de points (l'effet principal)."), r.lignes));
    v.push(case("adoucir", tr("Points adoucis (horizontalement)"), "", r.adoucir));
    v.push(case("halo", tr("Halo lumineux"), "", r.halo));
    v.push(case("bombe", tr("Tube bombé et coins assombris"), "", r.bombe));
    v.push(case("scintillement", tr("Scintillement"), "", r.scintillement));
    if moniteur == Moniteur::Couleur {
        v.push(case("masque_rvb", tr("Masque de phosphores RVB (moniteur couleur)"), "", r.masque_rvb));
    }
    v.push(tirette("intensite", tr("Intensité"), "", r.intensite));
    v.push(tirette(
        "remanence",
        tr("Rémanence du phosphore"),
        tr("Durée pendant laquelle les points s'éteignent (à gauche : aucune). Plus courte en blanc, moyenne en vert, plus longue en ambre."),
        r.remanence,
    ));
    if r.bombe {
        v.push(tirette("courbure", tr("Courbure"), "", r.courbure));
    }
    v
}

/// Applique le réglage `id` ; renvoie `true` s'il a changé quelque chose.
pub fn regler(r: &mut ReglagesEcran, phosphore: &mut Phosphore, id: &str, valeur: &ValeurReglage) -> bool {
    let avant = (*r, *phosphore);
    match (id, valeur) {
        ("phosphore", ValeurReglage::Choix { choisi, .. }) => {
            if let Some(p) = Phosphore::TOUS.get(*choisi) {
                *phosphore = *p;
            }
        }
        ("actif", ValeurReglage::Case(b)) => r.actif = *b,
        ("lignes", ValeurReglage::Case(b)) => r.lignes = *b,
        ("adoucir", ValeurReglage::Case(b)) => r.adoucir = *b,
        ("halo", ValeurReglage::Case(b)) => r.halo = *b,
        ("bombe", ValeurReglage::Case(b)) => r.bombe = *b,
        ("scintillement", ValeurReglage::Case(b)) => r.scintillement = *b,
        ("masque_rvb", ValeurReglage::Case(b)) => r.masque_rvb = *b,
        ("intensite", ValeurReglage::Curseur { valeur, .. }) => r.intensite = valeur.clamp(0.0, 1.0),
        ("remanence", ValeurReglage::Curseur { valeur, .. }) => r.remanence = valeur.clamp(0.0, 1.0),
        ("courbure", ValeurReglage::Curseur { valeur, .. }) => r.courbure = valeur.clamp(0.0, 1.0),
        _ => {}
    }
    avant != (*r, *phosphore)
}

/// Répertoire des routines de la ROM (Woz Monitor, ACI, Integer BASIC).
pub fn routines_rom() -> Vec<RoutineRom> {
    [
        (0xFFEF, "ECHO", tr("affiche le caractère ASCII de A (A conservé)")),
        (0xFFDC, "PRBYTE", tr("affiche A en deux chiffres hexadécimaux (A modifié)")),
        (0xFFE5, "PRHEX", tr("affiche le chiffre hexadécimal des 4 bits bas de A")),
        (0xFF1F, "GETLINE", tr("retour au Woz Monitor : nouvelle ligne et attente d'une commande")),
        (0xFF00, "RESET", tr("démarrage du Woz Monitor")),
        (0xC100, "ACI", tr("moniteur de la carte cassette ACI")),
        (0xE000, "BASIC", tr("démarrage à froid de l'Integer BASIC (programme effacé)")),
        (0xE2B3, "BASIC-CHAUD", tr("démarrage à chaud de l'Integer BASIC (programme gardé)")),
    ]
    .into_iter()
    .map(|(a, n, f)| RoutineRom { nom: n.into(), adresse: a, fonction: f.into() })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lignes_basic() {
        assert_eq!(ligne_basic("10 PRINT \"A\""), Some(10));
        assert_eq!(ligne_basic("  250GOTO 10"), Some(250));
        assert_eq!(ligne_basic("REM"), None);
        assert_eq!(ligne_basic(""), None);
    }

    #[test]
    fn arrets_ranges() {
        let points = vec![
            PointArret::Adresse(0x0300),
            PointArret::Adresse(0x1_0000),
            PointArret::Ligne { langage: Langage::Basic, ligne: 1, texte: "20 GOTO 10".into() },
            PointArret::Ligne { langage: Langage::Basic, ligne: 2, texte: "sans numéro".into() },
            PointArret::Ligne { langage: Langage::Assembleur, ligne: 3, texte: "  LDA #$C1".into() },
        ];
        let a = Arrets::depuis(&points);
        assert_eq!(a.adresses.iter().copied().collect::<Vec<_>>(), vec![0x0300]);
        assert_eq!(a.basic.iter().copied().collect::<Vec<_>>(), vec![20]);
        assert_eq!(a.asm, vec![3]);
        assert_eq!(a.adresses_completes(&[(2, 0x0280), (3, 0x0282)]).into_iter().collect::<Vec<_>>(), vec![0x0282, 0x0300]);
    }

    #[test]
    fn lignes_d_un_source_assemble() {
        let a = apple1_core::asm6502::assemble_text("        .org $0300\nDEBUT:  LDA #$C1\n        JSR $FFEF\n        RTS\n", None, &[]).unwrap();
        assert_eq!(adresses_des_lignes(&a), vec![(2, 0x0300), (3, 0x0302), (4, 0x0305)]);
    }

    #[test]
    fn reglages_vintage_par_defaut() {
        let r = ReglagesEcran::default();
        let v = reglages(Moniteur::Monochrome, &r, Phosphore::Vert);
        assert_eq!(v[0].id, "phosphore");
        assert!(v.iter().any(|x| x.id == "actif" && x.valeur == ValeurReglage::Case(true)));
        assert!(!v.iter().any(|x| x.id == "masque_rvb"));
        let eteint = reglages(Moniteur::Couleur, &ReglagesEcran { actif: false, ..r }, Phosphore::Vert);
        assert_eq!(eteint.len(), 1);
        let (mut r2, mut p) = (r, Phosphore::Vert);
        assert!(regler(&mut r2, &mut p, "actif", &ValeurReglage::Case(false)));
        assert!(!r2.actif);
        assert!(regler(&mut r2, &mut p, "phosphore", &ValeurReglage::Choix { options: vec![], choisi: 2 }));
        assert_eq!(p, Phosphore::Ambre);
        assert!(!regler(&mut r2, &mut p, "inconnu", &ValeurReglage::Case(true)));
    }

    #[test]
    fn routines_du_woz_monitor() {
        let r = routines_rom();
        assert!(r.iter().any(|x| x.nom == "ECHO" && x.adresse == 0xFFEF));
        assert!(r.iter().all(|x| !x.fonction.is_empty()));
    }
}
