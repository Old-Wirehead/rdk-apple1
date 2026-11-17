//! Clavier de l'Apple 1 2.0 (manifeste, rubrique 3).
//!
//! - **Fenêtre Clavier** : un clavier **QWERTY générique, sans pavé
//!   numérique** (demande de Wirehead du 17/11/2026), dessiné ici par le
//!   manifeste ([`dessin`]). Chaque clic passe par `clavier-machine`.
//!   « Maj » et « Ctrl » du clavier dessiné sont **rémanents** : ils
//!   s'appliquent à la touche suivante.
//! - **Clavier réel** : l'interface commune envoie les touches avec leur nom
//!   français ; [`code_apple1`] les traduit en codes ASCII 7 bits de
//!   l'Apple 1 (majuscules seulement, Retour arrière = `_` du Woz Monitor).

use rdk_pilotes::{DessinClavier, ToucheHote, ToucheVirtuelle};

/// Titre affiché au-dessus du clavier dessiné.
pub const TITRE: &str = "Clavier QWERTY générique (sans pavé numérique) — Apple 1";

/// Nom envoyé par la touche « Maj » du clavier dessiné.
pub const MAJ: &str = "Maj";
/// Nom envoyé par la touche « Ctrl » du clavier dessiné.
pub const CTRL: &str = "Ctrl";

fn car(s: &str) -> Vec<ToucheVirtuelle> {
    s.chars().map(ToucheVirtuelle::caractere).collect()
}

/// Le dessin du clavier : 6 rangées, aucune touche de pavé numérique.
pub fn dessin() -> DessinClavier {
    let mut r1 = car("`1234567890-=");
    r1.push(ToucheVirtuelle::commande("⌫ Retour", "Retour arrière", 2.0));
    let mut r2 = vec![ToucheVirtuelle::commande("Tab", "Tab", 1.5)];
    r2.extend(car("QWERTYUIOP[]"));
    r2.push(ToucheVirtuelle { largeur: 1.5, ..ToucheVirtuelle::caractere('\\') });
    let mut r3 = vec![ToucheVirtuelle::commande("Verr. Maj", "Verr. Maj", 1.75)];
    r3.extend(car("ASDFGHJKL;'"));
    r3.push(ToucheVirtuelle::commande("Entrée ⏎", "Entrée", 2.25));
    let mut r4 = vec![ToucheVirtuelle::commande("⇧ Maj", MAJ, 2.25)];
    r4.extend(car("ZXCVBNM,./"));
    r4.push(ToucheVirtuelle::commande("⇧ Maj", MAJ, 2.75));
    let mut espace = ToucheVirtuelle::caractere(' ');
    espace.libelle = "Espace".to_string();
    espace.largeur = 7.0;
    espace.touche.nom = "Espace".to_string();
    let r5 = vec![
        ToucheVirtuelle::commande("Ctrl", CTRL, 1.5),
        ToucheVirtuelle::commande("Alt", "Alt", 1.5),
        espace,
        ToucheVirtuelle::commande("Alt Gr", "Alt Gr", 1.5),
        ToucheVirtuelle::commande("Ctrl", CTRL, 1.5),
    ];
    DessinClavier { titre: TITRE.to_string(), rangees: vec![vec![ToucheVirtuelle::commande("Échap", "Échap", 1.25)], r1, r2, r3, r4, r5] }
}

/// Caractère obtenu avec Maj sur un clavier QWERTY (États-Unis).
pub fn avec_maj(c: char) -> char {
    const BAS: &str = "`1234567890-=[]\\;',./";
    const HAUT: &str = "~!@#$%^&*()_+{}|:\"<>?";
    match BAS.chars().position(|b| b == c) {
        Some(i) => HAUT.chars().nth(i).unwrap_or(c),
        None => c.to_ascii_uppercase(),
    }
}

/// Code ASCII 7 bits de l'Apple 1 pour une touche, ou `None` si la touche
/// n'existe pas sur l'Apple 1 (flèches, F1…, Tab…). `maj` et `ctrl`
/// s'ajoutent aux modificateurs de la touche (touches rémanentes).
pub fn code_apple1(t: &ToucheHote, maj: bool, ctrl: bool) -> Option<u8> {
    match t.nom.as_str() {
        "Entrée" => return Some(0x0D),
        "Échap" => return Some(0x1B),
        // le Woz Monitor efface avec « _ » (rubout)
        "Retour arrière" | "Suppr" => return Some(b'_'),
        _ => {}
    }
    let c = t.caractere?;
    if !c.is_ascii() {
        return None;
    }
    let c = if maj || t.maj { avec_maj(c) } else { c };
    if (ctrl || t.ctrl) && c.is_ascii_alphabetic() {
        return Some(c.to_ascii_uppercase() as u8 & 0x1F);
    }
    apple1_core::machine::map_key(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qwerty_sans_pave_numerique() {
        let d = dessin();
        let libelles: Vec<String> = d.rangees.iter().flatten().map(|t| t.libelle.clone()).collect();
        let lettres: String = d.rangees[2].iter().skip(1).take(10).map(|t| t.libelle.clone()).collect();
        assert_eq!(lettres, "QWERTYUIOP");
        assert!(d.rangees[4].iter().any(|t| t.libelle == "Z"));
        // une seule fois chaque chiffre : pas de pavé numérique
        for chiffre in "0123456789".chars() {
            assert_eq!(libelles.iter().filter(|l| **l == chiffre.to_string()).count(), 1, "{chiffre}");
        }
        assert!(!libelles.iter().any(|l| l.contains("Pavé") || l.contains("Verr. Num")));
        assert!(d.rangees.len() <= 6);
    }

    #[test]
    fn traduction_des_touches() {
        let t = |nom: &str, c: Option<char>| ToucheHote { nom: nom.into(), caractere: c, ..ToucheHote::default() };
        assert_eq!(code_apple1(&t("Entrée", None), false, false), Some(0x0D));
        assert_eq!(code_apple1(&t("Retour arrière", None), false, false), Some(b'_'));
        assert_eq!(code_apple1(&t("A", Some('a')), false, false), Some(b'A'));
        assert_eq!(code_apple1(&t("1", Some('1')), true, false), Some(b'!'));
        assert_eq!(code_apple1(&t("'", Some('\'')), true, false), Some(b'"'));
        assert_eq!(code_apple1(&t("C", Some('c')), false, true), Some(0x03));
        assert_eq!(code_apple1(&t("Flèche gauche", None), false, false), None);
        assert_eq!(code_apple1(&t("E", Some('é')), false, false), None);
    }
}
