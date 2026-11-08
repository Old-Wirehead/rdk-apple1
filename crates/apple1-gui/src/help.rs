//! « Aide sur … »: the manual entry of a BASIC keyword (taken from the
//! reference chapter of the manual, which is compiled in) or the short
//! description of a 6502 instruction.

use crate::i18n::{self, Lang};

const BASIC_FR: &str = include_str!("../../../docs/src/manuel/11-basic-reference.md");
const BASIC_EN: &str = include_str!("../../../docs/src/manual-en/11-basic-reference.md");
const BASIC_RU: &str = include_str!("../../../docs/src/manual-ru/11-basic-reference.md");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// Keyword (upper case).
    pub word: String,
    /// Syntax line (empty for functions and instructions).
    pub syntax: String,
    pub body: String,
}

/// Remove the markdown of the manual: `{{X}}` → X, backticks, bold.
fn plain(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find("{{") {
        out.push_str(&rest[..i]);
        match rest[i..].find("}}") {
            Some(j) => {
                out.push_str(&rest[i + 2..i + j]);
                rest = &rest[i + j + 2..];
            }
            None => {
                rest = &rest[i + 2..];
            }
        }
    }
    out.push_str(rest);
    out.replace(['`', '*'], "").replace("{: .syntax }", "").trim().to_string()
}

fn braces(s: &str) -> Vec<String> {
    let mut v = Vec::new();
    let mut rest = s;
    while let Some(i) = rest.find("{{") {
        match rest[i + 2..].find("}}") {
            Some(j) => {
                v.push(rest[i + 2..i + 2 + j].to_string());
                rest = &rest[i + 2 + j + 2..];
            }
            None => break,
        }
    }
    v
}

/// Upper case words written in backticks (`GOTO`, `FOR`…).
fn code_words(s: &str) -> Vec<String> {
    s.split('`').skip(1).step_by(2).filter(|w| !w.is_empty() && w.chars().all(|c| c.is_ascii_uppercase())).map(String::from).collect()
}

fn source(l: Lang) -> &'static str {
    match l {
        Lang::Fr => BASIC_FR,
        Lang::Ru => BASIC_RU,
        _ => BASIC_EN,
    }
}

/// Entry of a BASIC keyword in the manual of this language.
pub fn basic_entry(word: &str, lang: Lang) -> Option<Entry> {
    let w = word.trim_end_matches(['(', '$']).to_ascii_uppercase();
    if w.is_empty() {
        return None;
    }
    let src = source(lang);
    let blocks: Vec<&str> = src.split("\n\n").collect();
    for (i, b) in blocks.iter().enumerate() {
        // statements: a syntax paragraph, then the description
        if b.trim_end().ends_with("{: .syntax }") && i + 1 < blocks.len() {
            let first = b.lines().next().unwrap_or("");
            let desc = blocks[i + 1];
            if code_words(first).contains(&w) || braces(desc).contains(&w) {
                return Some(Entry { word: w, syntax: plain(first), body: plain(desc) });
            }
        }
        // functions: a table row starting with {{WORD}}
        for row in b.lines().filter(|l| l.starts_with("| {{")) {
            let cells: Vec<&str> = row.trim_matches('|').split('|').collect();
            if cells.len() >= 2 && braces(cells[0]).contains(&w) {
                let ex = cells.get(2).map(|c| plain(c)).filter(|c| c != "—" && !c.is_empty());
                let mut body = plain(cells[1]);
                if let Some(e) = ex {
                    body.push_str("\n\n");
                    body.push_str(&e);
                }
                return Some(Entry { word: w, syntax: plain(cells[0]), body });
            }
        }
    }
    // operators and anything else: the paragraph that names it (with its list)
    for (i, b) in blocks.iter().enumerate() {
        if !b.starts_with('|') && !b.starts_with('#') && braces(b).contains(&w) {
            let mut body = plain(b);
            if b.trim_end().ends_with(':') {
                if let Some(n) = blocks.get(i + 1) {
                    body.push_str("\n\n");
                    body.push_str(&n.lines().map(|l| l.replacen("- ", "• ", 1)).collect::<Vec<_>>().join("\n").replace(['`', '*'], ""));
                }
            }
            return Some(Entry { word: w, syntax: String::new(), body });
        }
    }
    None
}

/// (mnemonic, English, French, flags)
const OPS: &[(&str, &str, &str, &str)] = &[
    ("ADC", "Add memory to A with carry.", "Additionne la mémoire à A, avec la retenue.", "N V Z C"),
    ("AND", "A = A AND memory (bit by bit).", "A = A ET mémoire (bit à bit).", "N Z"),
    ("ASL", "Shift A or memory left; bit 7 goes to C, bit 0 becomes 0.", "Décale A ou la mémoire à gauche ; le bit 7 va dans C, le bit 0 devient 0.", "N Z C"),
    ("BCC", "Branch if carry clear (C = 0).", "Saute si la retenue est à 0 (C = 0).", ""),
    ("BCS", "Branch if carry set (C = 1).", "Saute si la retenue est à 1 (C = 1).", ""),
    ("BEQ", "Branch if equal (Z = 1).", "Saute si égal (Z = 1).", ""),
    ("BIT", "Test bits: Z = (A AND memory) = 0; N and V are copied from bits 7 and 6 of the memory.", "Teste des bits : Z = (A ET mémoire) = 0 ; N et V reprennent les bits 7 et 6 de la mémoire.", "N V Z"),
    ("BMI", "Branch if minus (N = 1).", "Saute si négatif (N = 1).", ""),
    ("BNE", "Branch if not equal (Z = 0).", "Saute si différent (Z = 0).", ""),
    ("BPL", "Branch if plus (N = 0).", "Saute si positif (N = 0).", ""),
    ("BRK", "Software interrupt: pushes PC+2 and P, then jumps through the vector at $FFFE.", "Interruption logicielle : empile PC+2 et P, puis saute par le vecteur en $FFFE.", "B I"),
    ("BVC", "Branch if overflow clear (V = 0).", "Saute si pas de débordement (V = 0).", ""),
    ("BVS", "Branch if overflow set (V = 1).", "Saute si débordement (V = 1).", ""),
    ("CLC", "Clear the carry flag.", "Met la retenue C à 0.", "C"),
    ("CLD", "Clear decimal mode.", "Quitte le mode décimal.", "D"),
    ("CLI", "Enable interrupts (I = 0).", "Autorise les interruptions (I = 0).", "I"),
    ("CLV", "Clear the overflow flag.", "Met l'indicateur de débordement V à 0.", "V"),
    ("CMP", "Compare A with memory (A - memory, result not kept).", "Compare A à la mémoire (A - mémoire, résultat non gardé).", "N Z C"),
    ("CPX", "Compare X with memory.", "Compare X à la mémoire.", "N Z C"),
    ("CPY", "Compare Y with memory.", "Compare Y à la mémoire.", "N Z C"),
    ("DEC", "Subtract 1 from memory.", "Retire 1 à la mémoire.", "N Z"),
    ("DEX", "Subtract 1 from X.", "Retire 1 à X.", "N Z"),
    ("DEY", "Subtract 1 from Y.", "Retire 1 à Y.", "N Z"),
    ("EOR", "A = A exclusive-OR memory.", "A = A OU exclusif mémoire.", "N Z"),
    ("INC", "Add 1 to memory.", "Ajoute 1 à la mémoire.", "N Z"),
    ("INX", "Add 1 to X.", "Ajoute 1 à X.", "N Z"),
    ("INY", "Add 1 to Y.", "Ajoute 1 à Y.", "N Z"),
    ("JMP", "Jump to the address (or through the pointer in JMP (addr)).", "Saute à l'adresse (ou par le pointeur de JMP (adr)).", ""),
    ("JSR", "Call a subroutine: pushes the return address - 1 and jumps.", "Appelle un sous-programme : empile l'adresse de retour - 1 et saute.", ""),
    ("LDA", "Load A from memory.", "Charge A depuis la mémoire.", "N Z"),
    ("LDX", "Load X from memory.", "Charge X depuis la mémoire.", "N Z"),
    ("LDY", "Load Y from memory.", "Charge Y depuis la mémoire.", "N Z"),
    ("LSR", "Shift A or memory right; bit 0 goes to C, bit 7 becomes 0.", "Décale A ou la mémoire à droite ; le bit 0 va dans C, le bit 7 devient 0.", "N=0 Z C"),
    ("NOP", "No operation (2 cycles).", "Ne fait rien (2 cycles).", ""),
    ("ORA", "A = A OR memory (bit by bit).", "A = A OU mémoire (bit à bit).", "N Z"),
    ("PHA", "Push A on the stack.", "Empile A.", ""),
    ("PHP", "Push the status register on the stack.", "Empile le registre d'état.", ""),
    ("PLA", "Pull A from the stack.", "Dépile dans A.", "N Z"),
    ("PLP", "Pull the status register from the stack.", "Dépile dans le registre d'état.", "tous"),
    ("ROL", "Rotate A or memory left through the carry.", "Rotation de A ou de la mémoire à gauche, par la retenue.", "N Z C"),
    ("ROR", "Rotate A or memory right through the carry.", "Rotation de A ou de la mémoire à droite, par la retenue.", "N Z C"),
    ("RTI", "Return from interrupt: pulls P, then PC.", "Retour d'interruption : dépile P, puis PC.", "tous"),
    ("RTS", "Return from subroutine: pulls the address, adds 1.", "Retour de sous-programme : dépile l'adresse, ajoute 1.", ""),
    ("SBC", "Subtract memory from A, with borrow (carry clear = borrow).", "Soustrait la mémoire de A, avec emprunt (retenue à 0 = emprunt).", "N V Z C"),
    ("SEC", "Set the carry flag.", "Met la retenue C à 1.", "C"),
    ("SED", "Set decimal mode.", "Passe en mode décimal.", "D"),
    ("SEI", "Disable interrupts (I = 1).", "Interdit les interruptions (I = 1).", "I"),
    ("STA", "Store A in memory.", "Range A en mémoire.", ""),
    ("STX", "Store X in memory.", "Range X en mémoire.", ""),
    ("STY", "Store Y in memory.", "Range Y en mémoire.", ""),
    ("TAX", "Copy A to X.", "Copie A dans X.", "N Z"),
    ("TAY", "Copy A to Y.", "Copie A dans Y.", "N Z"),
    ("TSX", "Copy the stack pointer to X.", "Copie le pointeur de pile dans X.", "N Z"),
    ("TXA", "Copy X to A.", "Copie X dans A.", "N Z"),
    ("TXS", "Copy X to the stack pointer.", "Copie X dans le pointeur de pile.", ""),
    ("TYA", "Copy Y to A.", "Copie Y dans A.", "N Z"),
];

/// Description of a 6502 instruction.
pub fn asm_entry(word: &str, lang: Lang) -> Option<Entry> {
    let w = word.to_ascii_uppercase();
    let (m, en, fr, fl) = OPS.iter().find(|o| o.0 == w)?;
    let mut body = if lang == Lang::Fr { fr } else { en }.to_string();
    if !fl.is_empty() {
        body.push_str(&format!("\n\n{} {}", if lang == Lang::Fr { "Indicateurs modifiés :" } else { "Flags affected:" }, fl));
    }
    Some(Entry { word: m.to_string(), syntax: String::new(), body })
}

/// The entry for a word of a source of this kind.
pub fn entry(word: &str, asm: bool) -> Option<Entry> {
    let l = i18n::current();
    if asm {
        asm_entry(word, l)
    } else {
        basic_entry(word, l)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_statements_functions_and_operators_have_entries() {
        let p = basic_entry("print", Lang::En).unwrap();
        assert!(p.syntax.starts_with("PRINT list"), "{p:?}");
        assert!(p.body.contains("displays numbers and strings"), "{p:?}");
        // GOSUB shares the entry of GOTO
        assert!(basic_entry("GOSUB", Lang::En).unwrap().body.contains("computed GOTO"));
        let peek = basic_entry("PEEK", Lang::En).unwrap();
        assert!(peek.body.contains("the byte at the address"), "{peek:?}");
        let m = basic_entry("MOD", Lang::En).unwrap();
        assert!(m.body.contains("truncates toward zero"), "{m:?}");
        assert!(basic_entry("PEEK(", Lang::Fr).is_some());
        assert!(basic_entry("LDA", Lang::En).is_none());
        assert!(basic_entry("PRINT", Lang::Ru).unwrap().body.contains("выводит"));
    }

    #[test]
    fn every_6502_mnemonic_is_described() {
        assert_eq!(OPS.len(), 56);
        assert!(asm_entry("lda", Lang::En).unwrap().body.contains("Load A"));
        assert!(asm_entry("LDA", Lang::Fr).unwrap().body.contains("Charge A"));
        assert!(asm_entry("FOO", Lang::En).is_none());
    }
}
