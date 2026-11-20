//! Pilotes `syntaxe-basic` (BASIC entier de l'Apple 1, compatible Integer
//! BASIC) et `syntaxe-asm` (6502, syntaxe ca65 de la v1.0). Le contrôle
//! réutilise le tokeniseur et l'assembleur de la v1.0 (`apple1-core`).

use crate::trf;
use apple1_core::{asm6502, intbasic};
use rdk_pilotes::pilotes::syntaxe::{
    Categorie, ErreurSyntaxe, Segment, Syntaxe, NOM_ASM, NOM_BASIC,
};

/// Découpe une ligne en mots, nombres, chaînes et commentaire.
fn decouper(ligne: &str, commentaire: &str, mot: &dyn Fn(&str, bool) -> Categorie) -> Vec<Segment> {
    let b = ligne.as_bytes();
    let mut v = Vec::new();
    let mut i = 0;
    let mut premier = true;
    while i < b.len() {
        let c = b[i];
        let debut = i;
        let cat = if ligne[i..].to_ascii_uppercase().starts_with(commentaire) {
            i = b.len();
            if commentaire == "REM" {
                v.push(Segment {
                    plage: debut..debut + 3,
                    categorie: Categorie::MotCle,
                });
                if debut + 3 < b.len() {
                    v.push(Segment {
                        plage: debut + 3..b.len(),
                        categorie: Categorie::Commentaire,
                    });
                }
                break;
            }
            Categorie::Commentaire
        } else if c == b'"' {
            i += 1;
            while i < b.len() && b[i] != b'"' {
                i += 1;
            }
            i = (i + 1).min(b.len());
            Categorie::Chaine
        } else if c.is_ascii_digit()
            || (c == b'$' && i + 1 < b.len() && b[i + 1].is_ascii_hexdigit())
        {
            i += 1;
            while i < b.len() && b[i].is_ascii_hexdigit() {
                i += 1;
            }
            if premier && commentaire == "REM" {
                Categorie::Etiquette
            } else {
                Categorie::Nombre
            }
        } else if c.is_ascii_alphabetic() || c == b'_' || c == b'.' {
            i += 1;
            while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_' || b[i] == b'$') {
                i += 1;
            }
            mot(&ligne[debut..i], premier)
        } else {
            i += ligne[i..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
            Categorie::Texte
        };
        if !c.is_ascii_whitespace() {
            premier = false;
        }
        v.push(Segment {
            plage: debut..i,
            categorie: cat,
        });
    }
    v
}

/// `syntaxe-basic` : BASIC entier de l'Apple 1.
#[derive(Clone, Copy, Debug, Default)]
pub struct SyntaxeBasicApple1;

impl Syntaxe for SyntaxeBasicApple1 {
    fn nom(&self) -> String {
        NOM_BASIC.to_string()
    }
    fn verifier(&self, texte: &str) -> Vec<ErreurSyntaxe> {
        intbasic::tokenize(texte)
            .errors
            .into_iter()
            .map(|e| ErreurSyntaxe {
                ligne: e.src_line,
                message: e.to_string(),
            })
            .collect()
    }
    fn colorer(&self, ligne: &str) -> Vec<Segment> {
        decouper(ligne, "REM", &|m, _| {
            if intbasic::KEYWORDS.contains(&m.to_ascii_uppercase().as_str()) {
                Categorie::MotCle
            } else {
                Categorie::Variable
            }
        })
    }
    fn commentaire(&self) -> Option<&'static str> {
        Some("REM")
    }
}

/// 2.0 (RDK-Interface-Commune#41) : message de la vérification en direct.
/// L'éditeur donne déjà la ligne : « colonne 17 : symbole non défini (ECHO) »
/// pour le texte de l'onglet, et le nom du fichier pour une erreur dans un
/// `.include` (au lieu de « :4:17 : … », sans nom de fichier).
pub fn message_asm(e: &asm6502::AsmError) -> String {
    let quoi = e.to_string();
    let sans_lieu = quoi.split_once(" : ").map(|(_, m)| m.to_string()).unwrap_or(quoi);
    if e.file == 0 {
        trf!("colonne {0} : {1}", e.col, sans_lieu)
    } else {
        trf!("{0}, ligne {1} : {2}", e.file_name, e.line, sans_lieu)
    }
}

/// `syntaxe-asm` : assembleur 6502 de la v1.0 (ca65, `.org`, étiquettes).
#[derive(Clone, Copy, Debug, Default)]
pub struct SyntaxeAsm6502;

impl Syntaxe for SyntaxeAsm6502 {
    fn nom(&self) -> String {
        NOM_ASM.to_string()
    }
    fn verifier(&self, texte: &str) -> Vec<ErreurSyntaxe> {
        // 2.0 (rdk-apple1#37) : APPLE1 défini, comme pour ▶ Lancer et ca65
        match asm6502::assemble_text_with(texte, None, &[], &[("APPLE1".to_string(), 1)]) {
            Ok(_) => Vec::new(),
            Err(erreurs) => erreurs
                .into_iter()
                .map(|e| ErreurSyntaxe {
                    ligne: e.line,
                    message: message_asm(&e),
                })
                .collect(),
        }
    }
    fn colorer(&self, ligne: &str) -> Vec<Segment> {
        decouper(ligne, ";", &|m, premier| {
            if asm6502::is_mnemonic(&m.to_ascii_uppercase()) {
                Categorie::Instruction
            } else if m.starts_with('.') {
                Categorie::MotCle
            } else if premier {
                Categorie::Etiquette
            } else {
                Categorie::Variable
            }
        })
    }
    fn commentaire(&self) -> Option<&'static str> {
        Some(";")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_mots_cles_et_erreurs() {
        let s = SyntaxeBasicApple1;
        let seg = s.colorer("10 PRINT \"HI\": REM X");
        assert_eq!(seg[0].categorie, Categorie::Etiquette);
        assert!(seg.iter().any(|g| g.categorie == Categorie::MotCle
            && &"10 PRINT \"HI\": REM X"[g.plage.clone()] == "PRINT"));
        assert!(seg.iter().any(|g| g.categorie == Categorie::Chaine));
        assert!(seg.iter().any(|g| g.categorie == Categorie::Commentaire));
        assert!(s.verifier("10 PRINT \"HI\"\n20 END\n").is_empty());
        assert_eq!(s.verifier("10 PRINT \"HI\"\n20 LIST\n")[0].ligne, 2);
    }

    #[test]
    fn asm_instructions_et_erreurs() {
        let s = SyntaxeAsm6502;
        let seg = s.colorer("boucle: LDA #$01 ; commentaire");
        assert_eq!(seg[0].categorie, Categorie::Etiquette);
        assert!(seg.iter().any(|g| g.categorie == Categorie::Instruction));
        assert!(seg.iter().any(|g| g.categorie == Categorie::Commentaire));
        assert!(s.verifier("  .org $0300\n  LDA #1\n  RTS\n").is_empty());
        assert!(!s.verifier("  .org $0300\n  LDQ #1\n").is_empty());
        // 2.0 : plus de « :2:7 : » sans nom de fichier
        let e = s.verifier("  .org $0300\n  JSR NULLEPART\n");
        assert_eq!((e[0].ligne, e[0].message.as_str()), (2, "colonne 7 : symbole non défini (NULLEPART)"));
    }

    /// 2.0 : les exemples GEN2 (rdk-apple1#38) et le choix de plateforme
    /// des projets GitHub (rdk-apple1#37) passent la vérification en direct.
    #[test]
    fn asm_gen2_inc_et_apple1() {
        let s = SyntaxeAsm6502;
        let lores = include_str!("../../../docs/exemples/asm/gen2-lores.s");
        assert_eq!(s.verifier(lores), Vec::new());
        let jmon = ".if .defined(APPLE1)\n  LDA #1\n.else\n  .error \"Platform not defined\"\n.endif\n";
        assert_eq!(s.verifier(jmon), Vec::new());
    }
}
