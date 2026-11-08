//! Native two-pass 6502 assembler, for the subset of the ca65 syntax used
//! by the Apple-1 ports (and by most small hand written programs).
//!
//! * every official NMOS opcode and addressing mode (`asl` and `asl a`,
//!   `(zp,x)`, `(zp),y`, `jmp (abs)`); zero page is chosen, like ca65, when
//!   the operand is already known and below `$100` where it is used;
//! * labels `NAME:`, cheap locals `@name:` (scope: up to the next regular
//!   label), unnamed labels `:` used as `:+ :++ :- :--`, constants
//!   `NAME = expr` (or `:=`), `*` for the current address;
//! * numbers `123 $7F %1010 'A'`, operators with the ca65 precedence:
//!   unary `- + ~ ! < > ^`, then `* / & ^ << >>`, then `+ - |`, then
//!   `= <> < > <= >=`, then `&&`, then `||`;
//! * `.org .byte/.byt .word/.addr .res .asciiz .include .segment` (one flat
//!   image: like `ld65` with a single memory area, all segments follow each
//!   other), `.macro/.endmacro` with parameters and `.local`,
//!   `.ifblank/.ifnblank/.else/.endif`;
//! * conditionals `.if/.elseif/.else/.endif`, `.ifdef/.ifndef` and
//!   `.defined(NAME)` (on constants and labels already seen, like ca65 which
//!   needs a value known at that point), `.repeat n[, i]/.endrep`,
//!   `.define NAME tokens` / `.undefine`;
//! * address size prefixes `a:` (absolute) and `z:` (zero page), `.feature
//!   string_escapes` (`\n \t \r \\ \" \' \xHH` in strings, as ca65);
//! * `.setcpu "65C02"` (or `.pc02`, back with `.setcpu "6502"` / `.p02`):
//!   the CMOS instructions and modes as `ca65 --cpu 65C02` (`bra phx phy plx
//!   ply stz trb tsb stp wai`, `(zp)`, `jmp (abs,x)`, `bit #`, `inc a`, and
//!   the Rockwell `rmb smb bbr bbs`);
//! * warnings (the assembly still succeeds): an operand that fits in the
//!   zero page coded on 2 bytes because its symbol is defined further down.
//!
//! Without `.org` the program starts at `$0300` (the `apple1.cfg` of the
//! ports). The result is the same bytes as `ca65` + `ld65` on the ports.

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

/// Load address used when the source has no `.org`.
pub const DEFAULT_ORG: u16 = 0x0300;

/// Instructions added by the 65C02 (with `.setcpu "65C02"`).
pub const MNEMONICS_65C02: [&str; 42] = [
    "BRA", "PHX", "PHY", "PLX", "PLY", "STZ", "TRB", "TSB", "STP", "WAI", "BBR0", "BBR1", "BBR2", "BBR3", "BBR4", "BBR5", "BBR6", "BBR7", "BBS0", "BBS1", "BBS2", "BBS3", "BBS4",
    "BBS5", "BBS6", "BBS7", "RMB0", "RMB1", "RMB2", "RMB3", "RMB4", "RMB5", "RMB6", "RMB7", "SMB0", "SMB1", "SMB2", "SMB3", "SMB4", "SMB5", "SMB6", "SMB7",
];

pub const MNEMONICS: [&str; 56] = [
    "ADC", "AND", "ASL", "BCC", "BCS", "BEQ", "BIT", "BMI", "BNE", "BPL", "BRK", "BVC", "BVS", "CLC", "CLD", "CLI", "CLV", "CMP", "CPX", "CPY", "DEC", "DEX", "DEY", "EOR",
    "INC", "INX", "INY", "JMP", "JSR", "LDA", "LDX", "LDY", "LSR", "NOP", "ORA", "PHA", "PHP", "PLA", "PLP", "ROL", "ROR", "RTI", "RTS", "SBC", "SEC", "SED", "SEI", "STA",
    "STX", "STY", "TAX", "TAY", "TSX", "TXA", "TXS", "TYA",
];

/// Directives understood (lower case, with their dot).
pub const DIRECTIVES: [&str; 37] = [
    ".org", ".byte", ".byt", ".word", ".addr", ".res", ".asciiz", ".include", ".segment", ".code", ".data", ".rodata", ".bss", ".macro", ".mac", ".endmacro", ".endmac", ".local",
    ".ifblank", ".ifnblank", ".else", ".endif", ".if", ".elseif", ".ifdef", ".ifndef", ".defined", ".repeat", ".endrep", ".endrepeat", ".define", ".undefine", ".undef",
    ".setcpu", ".pc02", ".p02", ".feature",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrKind {
    /// Anything that cannot be parsed.
    Syntax,
    /// Not a mnemonic, a directive or a macro.
    Unknown,
    /// Addressing mode that the instruction does not have.
    BadMode,
    /// Symbol never defined.
    Undefined,
    /// Symbol defined twice.
    Duplicate,
    /// Value too large for a byte (or a word, or the address space).
    Range,
    /// Branch target further than -128..+127 bytes.
    Branch,
    /// `.include` file not found.
    Include,
    /// `.macro` without `.endmacro`, `.endif` without `.if`…
    Block,
    /// Division by zero.
    DivZero,
    /// String or character without its closing quote.
    String,
    /// Value needed now (`.org`, `.res`, `.if`, `.repeat`) but not known yet.
    NotConst,
    /// Warning: zero page operand coded as absolute (symbol defined further down).
    ZpAbs,
}

impl ErrKind {
    /// Short identifier (used as a translation key by the GUI).
    pub fn key(self) -> &'static str {
        match self {
            ErrKind::Syntax => "syntax",
            ErrKind::Unknown => "unknown",
            ErrKind::BadMode => "bad_mode",
            ErrKind::Undefined => "undefined",
            ErrKind::Duplicate => "duplicate",
            ErrKind::Range => "range",
            ErrKind::Branch => "branch",
            ErrKind::Include => "include",
            ErrKind::Block => "block",
            ErrKind::DivZero => "div_zero",
            ErrKind::String => "string",
            ErrKind::NotConst => "not_const",
            ErrKind::ZpAbs => "zp_abs",
        }
    }
    pub const ALL: [ErrKind; 13] = [
        ErrKind::Syntax,
        ErrKind::Unknown,
        ErrKind::BadMode,
        ErrKind::Undefined,
        ErrKind::Duplicate,
        ErrKind::Range,
        ErrKind::Branch,
        ErrKind::Include,
        ErrKind::Block,
        ErrKind::DivZero,
        ErrKind::String,
        ErrKind::NotConst,
        ErrKind::ZpAbs,
    ];
    fn fr(self) -> &'static str {
        match self {
            ErrKind::Syntax => "erreur de syntaxe",
            ErrKind::Unknown => "instruction, directive ou macro inconnue",
            ErrKind::BadMode => "mode d'adressage impossible pour cette instruction",
            ErrKind::Undefined => "symbole non défini",
            ErrKind::Duplicate => "symbole déjà défini",
            ErrKind::Range => "valeur hors limites",
            ErrKind::Branch => "branchement trop loin (-128 à +127 octets)",
            ErrKind::Include => "fichier .include introuvable",
            ErrKind::Block => "bloc mal fermé (.macro/.endmacro, .if/.endif)",
            ErrKind::DivZero => "division par zéro",
            ErrKind::String => "chaîne ou caractère sans guillemet fermant",
            ErrKind::NotConst => "valeur inconnue à cet endroit (.org, .res, .if, .repeat)",
            ErrKind::ZpAbs => "avertissement : adresse en page zéro codée sur 2 octets (symbole défini plus bas ; définissez-le avant, ou écrivez z: ou a:)",
        }
    }
}

/// An error, with its place: file (0 = the main text), line and column (1-based).
#[derive(Debug, Clone, PartialEq)]
pub struct AsmError {
    pub file: usize,
    pub file_name: String,
    /// Path of that file (empty for an unsaved main text).
    pub path: PathBuf,
    pub line: usize,
    pub col: usize,
    pub kind: ErrKind,
    /// The symbol, value or text concerned.
    pub detail: String,
}

impl fmt::Display for AsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{} : {}", self.file_name, self.line, self.col, self.kind.fr())?;
        if !self.detail.is_empty() {
            write!(f, " ({})", self.detail)?;
        }
        Ok(())
    }
}

/// One line of the listing: bytes produced by a source line.
#[derive(Debug, Clone, PartialEq)]
pub struct ListLine {
    pub file: usize,
    pub line: usize,
    pub addr: u16,
    pub bytes: Vec<u8>,
    /// Text of the macro line that produced these bytes (macro expansion), if any.
    pub exp: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Assembly {
    /// Load (and start) address: the first `.org`, or `$0300`.
    pub origin: u16,
    pub bytes: Vec<u8>,
    pub listing: Vec<ListLine>,
    /// Files read: index 0 is the main text, then the `.include`s.
    pub files: Vec<PathBuf>,
    /// For each file, the file and line of its `.include` (`(0, 0)` for the main text).
    pub parents: Vec<(usize, usize)>,
    /// Labels and constants, sorted by name.
    pub symbols: Vec<(String, i64)>,
    /// Warnings (the assembly succeeded anyway).
    pub warnings: Vec<AsmError>,
}

impl Assembly {
    pub fn end(&self) -> u16 {
        (self.origin as usize + self.bytes.len().max(1) - 1) as u16
    }
    /// Range to type after `C100R`, 4 digits each side (`0300.170FR`).
    pub fn aci_range(&self) -> String {
        format!("{:04X}.{:04X}R", self.origin, self.end())
    }
    /// Woz Monitor command that starts the program (`300R`).
    pub fn run_command(&self) -> String {
        format!("{:X}R", self.origin)
    }
}

/// One row of the listing.
#[derive(Debug, Clone, PartialEq)]
pub struct ListRow {
    /// `ADDR  BYTES…  source`.
    pub text: String,
    pub file: usize,
    /// Source line (1-based), 0 for a file header.
    pub line: usize,
    /// Address of the first byte of the row, when it has bytes.
    pub addr: Option<u16>,
}

/// The listing in source order: every line of the main text (with its
/// address and bytes, 4 bytes per row), and the lines of the included files
/// that produce bytes, after their `.include` line. `main_text` is the main
/// text when it is not the file on disk (editor).
pub fn listing_rows(a: &Assembly, main_text: Option<&str>) -> Vec<ListRow> {
    let lines_of = |t: &str| t.lines().map(|l| l.replace('\t', "        ")).collect::<Vec<String>>();
    let mut texts: Vec<Vec<String>> = Vec::new();
    for (k, f) in a.files.iter().enumerate() {
        let t = match (k, main_text) {
            (0, Some(m)) => lines_of(m),
            _ => std::fs::read(f).map(|b| lines_of(&String::from_utf8_lossy(&b))).unwrap_or_default(),
        };
        texts.push(t);
    }
    let src = |f: usize, l: usize| texts.get(f).and_then(|t| t.get(l.wrapping_sub(1))).cloned().unwrap_or_default();
    let mut rows = Vec::new();
    let mut next_main = 1;
    let mut last_file = 0;
    let flush = |rows: &mut Vec<ListRow>, next_main: &mut usize, upto: usize| {
        while *next_main <= upto {
            rows.push(ListRow { text: format!("{:18}{}", "", src(0, *next_main)), file: 0, line: *next_main, addr: None });
            *next_main += 1;
        }
    };
    let push = |rows: &mut Vec<ListRow>, l: &ListLine| {
        for (k, ch) in l.bytes.chunks(4).enumerate() {
            let hex: Vec<String> = ch.iter().map(|b| format!("{:02X}", b)).collect();
            let addr = l.addr as usize + k * 4;
            let text = match (k, &l.exp) {
                (0, Some(x)) => format!("{:04X}  {:<12}    + {}", addr, hex.join(" "), x),
                (0, None) => format!("{:04X}  {:<12}{}", addr, hex.join(" "), src(l.file, l.line)),
                _ => format!("{:04X}  {}", addr, hex.join(" ")),
            };
            rows.push(ListRow { text, file: l.file, line: l.line, addr: Some(addr as u16) });
        }
    };
    for l in &a.listing {
        // the line of the main text this comes from
        let mut f = l.file;
        let mut ln = l.line;
        while f != 0 {
            let (pf, pl) = a.parents.get(f).copied().unwrap_or((0, 0));
            f = pf;
            ln = pl;
        }
        if l.file == 0 {
            // a macro call: its line first, then the expanded lines
            flush(&mut rows, &mut next_main, if l.exp.is_some() { ln } else { ln.saturating_sub(1) });
            if ln >= next_main {
                push(&mut rows, l);
                next_main = ln + 1;
            } else {
                push(&mut rows, l);
            }
        } else {
            flush(&mut rows, &mut next_main, ln);
            if l.file != last_file {
                let name = a.files[l.file].file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                rows.push(ListRow { text: format!("{:18}; ---- {}", "", name), file: l.file, line: 0, addr: None });
            }
            push(&mut rows, l);
        }
        last_file = l.file;
    }
    let n = texts.first().map(|t| t.len()).unwrap_or(0);
    flush(&mut rows, &mut next_main, n);
    if !a.symbols.is_empty() {
        rows.push(ListRow { text: String::new(), file: 0, line: 0, addr: None });
        for t in symbol_table_text(a).lines() {
            rows.push(ListRow { text: t.to_string(), file: 0, line: 0, addr: None });
        }
    }
    rows
}

/// Symbol table: a `; Symboles` header, then `NAME  $VALUE` sorted by name.
pub fn symbol_table_text(a: &Assembly) -> String {
    let mut s = format!("; ---- Symboles ({})\n", a.symbols.len());
    let w = a.symbols.iter().map(|x| x.0.chars().count()).max().unwrap_or(0).max(8);
    for (n, v) in &a.symbols {
        let val = if (0..0x100).contains(v) { format!("${:02X}", v) } else if (0..0x10000).contains(v) { format!("${:04X}", v) } else { v.to_string() };
        s.push_str(&format!("; {:w$}  {}\n", n, val, w = w));
    }
    s
}

/// Listing as text (see [`listing_rows`]).
pub fn listing_text(a: &Assembly) -> String {
    let mut s = String::new();
    for r in listing_rows(a, None) {
        s.push_str(r.text.trim_end());
        s.push('\n');
    }
    s
}

// ---------------------------------------------------------------- lexer

#[derive(Debug, Clone, PartialEq)]
enum T {
    Id(String),
    Num(i64),
    Str(Vec<u8>),
    Op(&'static str),
    /// `:+` (1), `:++` (2), `:-` (-1)…
    Unnamed(i32),
    Colon,
    Comma,
    Hash,
    LP,
    RP,
}

#[derive(Debug, Clone, PartialEq)]
struct Tok {
    t: T,
    col: usize,
}

type LexErr = (ErrKind, usize, String);

/// `esc`: `.feature string_escapes` is on.
fn lex(line: &str, esc: bool) -> Result<Vec<Tok>, LexErr> {
    let c: Vec<char> = line.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    while i < c.len() {
        let ch = c[i];
        let col = i + 1;
        if ch == ';' {
            break;
        }
        if ch.is_whitespace() {
            i += 1;
            continue;
        }
        let t = if ch.is_ascii_alphabetic() || ch == '_' || ch == '.' || ch == '@' {
            let s = i;
            i += 1;
            while i < c.len() && (c[i].is_ascii_alphanumeric() || c[i] == '_') {
                i += 1;
            }
            let id: String = c[s..i].iter().collect();
            if id == "." || id == "@" {
                return Err((ErrKind::Syntax, col, id));
            }
            out.push(Tok { t: T::Id(id), col });
            continue;
        } else if ch.is_ascii_digit() || ((ch == '$' || ch == '%') && i + 1 < c.len() && c[i + 1].is_ascii_alphanumeric()) {
            let (radix, s) = match ch {
                '$' => (16, i + 1),
                '%' => (2, i + 1),
                _ => (10, i),
            };
            i = s;
            while i < c.len() && c[i].is_ascii_alphanumeric() {
                i += 1;
            }
            let txt: String = c[s..i].iter().collect();
            match i64::from_str_radix(&txt, radix) {
                Ok(v) if v <= u32::MAX as i64 => T::Num(v),
                _ => return Err((ErrKind::Syntax, col, c[col - 1..i].iter().collect())),
            }
        } else if ch == '"' {
            let mut j = i + 1;
            let mut s: Vec<u8> = Vec::new();
            while j < c.len() && c[j] != '"' {
                if esc && c[j] == '\\' && j + 1 < c.len() {
                    let e = c[j + 1];
                    let v = match e {
                        'n' => Some(10),
                        't' => Some(9),
                        'r' => Some(13),
                        '\\' | '"' | '\'' => Some(e as u8),
                        'x' | 'X' => {
                            let h: String = c[j + 2..(j + 4).min(c.len())].iter().collect();
                            match (h.len(), u8::from_str_radix(&h, 16)) {
                                (2, Ok(v)) => {
                                    j += 2;
                                    Some(v)
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    };
                    match v {
                        Some(v) => s.push(v),
                        None => return Err((ErrKind::String, j + 1, c[j..(j + 2).min(c.len())].iter().collect())),
                    }
                    j += 2;
                    continue;
                }
                let mut b = [0u8; 4];
                s.extend_from_slice(c[j].encode_utf8(&mut b).as_bytes());
                j += 1;
            }
            if j >= c.len() {
                return Err((ErrKind::String, col, c[i..].iter().collect()));
            }
            i = j + 1;
            T::Str(s)
        } else if ch == '\'' {
            if i + 2 < c.len() && c[i + 2] == '\'' {
                let v = c[i + 1] as i64;
                i += 3;
                T::Num(v)
            } else {
                return Err((ErrKind::String, col, c[i..(i + 3).min(c.len())].iter().collect()));
            }
        } else if ch == ':' {
            i += 1;
            if i < c.len() && (c[i] == '+' || c[i] == '-') {
                let sign = c[i];
                let mut n = 0;
                while i < c.len() && c[i] == sign {
                    n += 1;
                    i += 1;
                }
                T::Unnamed(if sign == '+' { n } else { -n })
            } else if i < c.len() && c[i] == '=' {
                i += 1;
                T::Op(":=")
            } else {
                T::Colon
            }
        } else {
            i += 1;
            let two = if i < c.len() { Some(c[i]) } else { None };
            let op2 = match (ch, two) {
                ('<', Some('<')) => Some("<<"),
                ('>', Some('>')) => Some(">>"),
                ('<', Some('=')) => Some("<="),
                ('>', Some('=')) => Some(">="),
                ('<', Some('>')) => Some("<>"),
                ('&', Some('&')) => Some("&&"),
                ('|', Some('|')) => Some("||"),
                _ => None,
            };
            if let Some(o) = op2 {
                i += 1;
                T::Op(o)
            } else {
                match ch {
                    ',' => T::Comma,
                    '#' => T::Hash,
                    '(' => T::LP,
                    ')' => T::RP,
                    '+' => T::Op("+"),
                    '-' => T::Op("-"),
                    '*' => T::Op("*"),
                    '/' => T::Op("/"),
                    '&' => T::Op("&"),
                    '|' => T::Op("|"),
                    '^' => T::Op("^"),
                    '~' => T::Op("~"),
                    '!' => T::Op("!"),
                    '<' => T::Op("<"),
                    '>' => T::Op(">"),
                    '=' => T::Op("="),
                    _ => return Err((ErrKind::Syntax, col, ch.to_string())),
                }
            }
        };
        out.push(Tok { t, col });
    }
    Ok(out)
}

// ------------------------------------------------------------ expressions

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Num(i64),
    Sym(String, usize),
    Pc,
    Un(&'static str, Box<Expr>),
    Bin(&'static str, Box<Expr>, Box<Expr>),
    /// `.defined(NAME)`.
    Defined(String),
}

struct P<'a> {
    t: &'a [Tok],
    i: usize,
    /// Rewrites `@x` and `:+` into global names.
    scope: &'a str,
    unnamed: usize,
}

type PErr = (ErrKind, usize, String);

impl<'a> P<'a> {
    fn peek(&self) -> Option<&T> {
        self.t.get(self.i).map(|t| &t.t)
    }
    fn col(&self) -> usize {
        self.t.get(self.i).or(self.t.last()).map(|t| t.col).unwrap_or(1)
    }
    fn expr(&mut self) -> Result<Expr, PErr> {
        self.bin(0)
    }
    fn bin(&mut self, level: usize) -> Result<Expr, PErr> {
        const LEVELS: [&[&str]; 5] = [&["||"], &["&&"], &["=", "<>", "<", ">", "<=", ">="], &["+", "-", "|"], &["*", "/", "&", "^", "<<", ">>"]];
        if level == LEVELS.len() {
            return self.unary();
        }
        let mut l = self.bin(level + 1)?;
        loop {
            let op = match self.peek() {
                Some(T::Op(o)) if LEVELS[level].contains(o) => *o,
                _ => break,
            };
            self.i += 1;
            let r = self.bin(level + 1)?;
            l = Expr::Bin(op, Box::new(l), Box::new(r));
        }
        Ok(l)
    }
    fn unary(&mut self) -> Result<Expr, PErr> {
        let col = self.col();
        match self.peek().cloned() {
            Some(T::Op(o)) if ["-", "+", "~", "!", "<", ">", "^"].contains(&o) => {
                self.i += 1;
                let e = self.unary()?;
                Ok(Expr::Un(o, Box::new(e)))
            }
            Some(T::Op("*")) => {
                self.i += 1;
                Ok(Expr::Pc)
            }
            Some(T::Num(v)) => {
                self.i += 1;
                Ok(Expr::Num(v))
            }
            Some(T::Id(s)) if s.eq_ignore_ascii_case(".defined") || s.eq_ignore_ascii_case(".def") => {
                let ok = matches!(self.t.get(self.i + 1).map(|t| &t.t), Some(T::LP)) && matches!(self.t.get(self.i + 3).map(|t| &t.t), Some(T::RP));
                match (ok, self.t.get(self.i + 2).map(|t| &t.t)) {
                    (true, Some(T::Id(n))) if !n.starts_with('.') => {
                        let name = if n.starts_with('@') { format!("{}{}", self.scope, n) } else { n.clone() };
                        self.i += 4;
                        Ok(Expr::Defined(name))
                    }
                    _ => Err((ErrKind::Syntax, col, s)),
                }
            }
            Some(T::Id(s)) if !s.starts_with('.') => {
                self.i += 1;
                let name = if s.starts_with('@') { format!("{}{}", self.scope, s) } else { s };
                Ok(Expr::Sym(name, col))
            }
            Some(T::Unnamed(n)) => {
                self.i += 1;
                let k = if n > 0 { self.unnamed as i64 + n as i64 - 1 } else { self.unnamed as i64 + n as i64 };
                Ok(Expr::Sym(format!(":{}", k), col))
            }
            Some(T::LP) => {
                self.i += 1;
                let e = self.expr()?;
                if self.peek() != Some(&T::RP) {
                    return Err((ErrKind::Syntax, self.col(), ")".into()));
                }
                self.i += 1;
                Ok(e)
            }
            Some(_) => Err((ErrKind::Syntax, col, tok_text(&self.t[self.i].t))),
            None => Err((ErrKind::Syntax, col, String::new())),
        }
    }
}

fn tok_text(t: &T) -> String {
    match t {
        T::Id(s) => s.clone(),
        T::Num(v) => v.to_string(),
        T::Str(s) => format!("\"{}\"", String::from_utf8_lossy(s)),
        T::Op(o) => o.to_string(),
        T::Unnamed(n) => {
            if *n > 0 {
                format!(":{}", "+".repeat(*n as usize))
            } else {
                format!(":{}", "-".repeat((-*n) as usize))
            }
        }
        T::Colon => ":".into(),
        T::Comma => ",".into(),
        T::Hash => "#".into(),
        T::LP => "(".into(),
        T::RP => ")".into(),
    }
}

// -------------------------------------------------------------- opcodes

/// Addressing mode of an instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Imp,
    Acc,
    Imm,
    Zp,
    ZpX,
    ZpY,
    Abs,
    AbsX,
    AbsY,
    Ind,
    IndX,
    IndY,
    Rel,
    /// 65C02 `(zp)`.
    ZpInd,
    /// 65C02 `jmp (abs,x)`.
    AbsIndX,
    /// 65C02 (Rockwell) `bbr0 zp, label`.
    ZpRel,
}


/// Opcode of an official NMOS instruction in a mode.
fn opcode(mn: &str, m: Mode) -> Option<u8> {
    use Mode::*;
    // ALU group: ora and eor adc sta lda cmp sbc
    let alu = |base: u8| -> Option<u8> {
        Some(
            base + match m {
                IndX => 0x01,
                Zp => 0x05,
                Imm => 0x09,
                Abs => 0x0D,
                IndY => 0x11,
                ZpX => 0x15,
                AbsY => 0x19,
                AbsX => 0x1D,
                _ => return None,
            },
        )
    };
    // shift group: asl rol lsr ror (dec inc without Acc)
    let sh = |base: u8, acc: bool| -> Option<u8> {
        Some(
            base + match m {
                Zp => 0x06,
                Acc if acc => 0x0A,
                Abs => 0x0E,
                ZpX => 0x16,
                AbsX => 0x1E,
                _ => return None,
            },
        )
    };
    let pick = |l: &[(Mode, u8)]| l.iter().find(|(x, _)| *x == m).map(|(_, o)| *o);
    match mn {
        "ORA" => alu(0x00),
        "AND" => alu(0x20),
        "EOR" => alu(0x40),
        "ADC" => alu(0x60),
        "STA" => {
            if m == Imm {
                None
            } else {
                alu(0x80)
            }
        }
        "LDA" => alu(0xA0),
        "CMP" => alu(0xC0),
        "SBC" => alu(0xE0),
        "ASL" => sh(0x00, true),
        "ROL" => sh(0x20, true),
        "LSR" => sh(0x40, true),
        "ROR" => sh(0x60, true),
        "DEC" => sh(0xC0, false),
        "INC" => sh(0xE0, false),
        "LDX" => pick(&[(Imm, 0xA2), (Zp, 0xA6), (Abs, 0xAE), (ZpY, 0xB6), (AbsY, 0xBE)]),
        "LDY" => pick(&[(Imm, 0xA0), (Zp, 0xA4), (Abs, 0xAC), (ZpX, 0xB4), (AbsX, 0xBC)]),
        "STX" => pick(&[(Zp, 0x86), (Abs, 0x8E), (ZpY, 0x96)]),
        "STY" => pick(&[(Zp, 0x84), (Abs, 0x8C), (ZpX, 0x94)]),
        "CPX" => pick(&[(Imm, 0xE0), (Zp, 0xE4), (Abs, 0xEC)]),
        "CPY" => pick(&[(Imm, 0xC0), (Zp, 0xC4), (Abs, 0xCC)]),
        "BIT" => pick(&[(Zp, 0x24), (Abs, 0x2C)]),
        "JMP" => pick(&[(Abs, 0x4C), (Ind, 0x6C)]),
        "JSR" => pick(&[(Abs, 0x20)]),
        _ => {
            let rel = [("BPL", 0x10), ("BMI", 0x30), ("BVC", 0x50), ("BVS", 0x70), ("BCC", 0x90), ("BCS", 0xB0), ("BNE", 0xD0), ("BEQ", 0xF0)];
            if let Some((_, o)) = rel.iter().find(|(n, _)| *n == mn) {
                return if m == Rel { Some(*o) } else { None };
            }
            let imp = [
                ("BRK", 0x00),
                ("PHP", 0x08),
                ("CLC", 0x18),
                ("PLP", 0x28),
                ("SEC", 0x38),
                ("RTI", 0x40),
                ("PHA", 0x48),
                ("CLI", 0x58),
                ("RTS", 0x60),
                ("PLA", 0x68),
                ("SEI", 0x78),
                ("DEY", 0x88),
                ("TXA", 0x8A),
                ("TYA", 0x98),
                ("TXS", 0x9A),
                ("TAY", 0xA8),
                ("TAX", 0xAA),
                ("CLV", 0xB8),
                ("TSX", 0xBA),
                ("INY", 0xC8),
                ("DEX", 0xCA),
                ("CLD", 0xD8),
                ("INX", 0xE8),
                ("NOP", 0xEA),
                ("SED", 0xF8),
            ];
            imp.iter().find(|(n, _)| *n == mn).and_then(|(_, o)| if m == Imp { Some(*o) } else { None })
        }
    }
}

/// Opcode of an instruction in a mode, with the 65C02 additions when `c02`.
fn opc(mn: &str, m: Mode, c02: bool) -> Option<u8> {
    use Mode::*;
    if let Some(o) = opcode(mn, m) {
        return Some(o);
    }
    if !c02 {
        return None;
    }
    let bit = |p: &str| mn.strip_prefix(p).and_then(|d| d.parse::<u8>().ok()).filter(|d| *d < 8);
    if let Some(d) = bit("RMB") {
        return (m == Zp).then_some(0x07 + d * 0x10);
    }
    if let Some(d) = bit("SMB") {
        return (m == Zp).then_some(0x87 + d * 0x10);
    }
    if let Some(d) = bit("BBR") {
        return (m == ZpRel).then_some(0x0F + d * 0x10);
    }
    if let Some(d) = bit("BBS") {
        return (m == ZpRel).then_some(0x8F + d * 0x10);
    }
    let l: &[(Mode, u8)] = match mn {
        "ORA" => &[(ZpInd, 0x12)],
        "AND" => &[(ZpInd, 0x32)],
        "EOR" => &[(ZpInd, 0x52)],
        "ADC" => &[(ZpInd, 0x72)],
        "STA" => &[(ZpInd, 0x92)],
        "LDA" => &[(ZpInd, 0xB2)],
        "CMP" => &[(ZpInd, 0xD2)],
        "SBC" => &[(ZpInd, 0xF2)],
        "BIT" => &[(Imm, 0x89), (ZpX, 0x34), (AbsX, 0x3C)],
        "INC" => &[(Acc, 0x1A)],
        "DEC" => &[(Acc, 0x3A)],
        "JMP" => &[(AbsIndX, 0x7C)],
        "BRA" => &[(Rel, 0x80)],
        "PHX" => &[(Imp, 0xDA)],
        "PHY" => &[(Imp, 0x5A)],
        "PLX" => &[(Imp, 0xFA)],
        "PLY" => &[(Imp, 0x7A)],
        "STP" => &[(Imp, 0xDB)],
        "WAI" => &[(Imp, 0xCB)],
        "STZ" => &[(Zp, 0x64), (ZpX, 0x74), (Abs, 0x9C), (AbsX, 0x9E)],
        "TRB" => &[(Zp, 0x14), (Abs, 0x1C)],
        "TSB" => &[(Zp, 0x04), (Abs, 0x0C)],
        _ => &[],
    };
    l.iter().find(|(x, _)| *x == m).map(|(_, o)| *o)
}

/// Mnemonic and addressing mode of an opcode (official NMOS set), for the
/// disassembler. Built once from [`opcode`], so both always agree.
pub fn decode(op: u8) -> Option<(&'static str, Mode)> {
    use std::sync::OnceLock;
    static TABLE: OnceLock<Vec<Option<(&'static str, Mode)>>> = OnceLock::new();
    let t = TABLE.get_or_init(|| {
        use Mode::*;
        let mut t = vec![None; 256];
        for mn in MNEMONICS {
            for m in [Imp, Acc, Imm, Zp, ZpX, ZpY, Abs, AbsX, AbsY, Ind, IndX, IndY, Rel] {
                if let Some(o) = opcode(mn, m) {
                    t[o as usize] = Some((mn, m));
                }
            }
        }
        t
    });
    t[op as usize]
}

/// Bytes taken by an instruction in this mode, opcode included.
pub fn mode_len(m: Mode) -> u8 {
    use Mode::*;
    match m {
        Imp | Acc => 1,
        Imm | Zp | ZpX | ZpY | IndX | IndY | Rel | ZpInd => 2,
        Abs | AbsX | AbsY | Ind | AbsIndX | ZpRel => 3,
    }
}

fn is_branch(mn: &str) -> bool {
    matches!(mn, "BPL" | "BMI" | "BVC" | "BVS" | "BCC" | "BCS" | "BNE" | "BEQ" | "BRA")
}

/// Is `w` (any case) a 6502 (or 65C02) mnemonic?
pub fn is_mnemonic(w: &str) -> bool {
    let u = w.to_ascii_uppercase();
    MNEMONICS.contains(&u.as_str()) || MNEMONICS_65C02.contains(&u.as_str())
}

// ------------------------------------------------------------ statements

#[derive(Debug, Clone, Copy, PartialEq)]
struct Loc {
    file: usize,
    line: usize,
}

#[derive(Debug, Clone)]
struct Line {
    loc: Loc,
    toks: Vec<Tok>,
    /// Text of the line as expanded from a macro (for the listing).
    exp: Option<String>,
}

#[derive(Debug, Clone)]
enum Operand {
    None,
    Acc,
    Imm(Expr),
    /// expr, index register (b'X', b'Y' or 0).
    Addr(Expr, u8),
    Ind(Expr),
    IndX(Expr),
    IndY(Expr),
    /// `bbr0 zp, label`.
    Pair(Expr, Expr),
}

#[derive(Debug, Clone)]
enum ByteArg {
    E(Expr, usize),
    S(Vec<u8>),
}

#[derive(Debug, Clone)]
enum Item {
    Label(String, usize),
    Const(String, Expr, usize),
    /// `c02`: 65C02 enabled there; `force`: `a:` or `z:` prefix (b'a', b'z', or 0).
    Instr { mn: String, op: Operand, col: usize, mode: Option<Mode>, c02: bool, force: u8 },
    Bytes(Vec<ByteArg>, bool),
    Words(Vec<(Expr, usize)>),
    Res(Expr, Option<Expr>, usize),
    Org(Expr, usize),
}

struct Macro {
    params: Vec<String>,
    body: Vec<Line>,
}

/// Finds an `.include` file: returns its path and text.
pub type Loader<'a> = dyn Fn(&str, &[PathBuf]) -> Option<(PathBuf, String)> + 'a;

/// Places searched for `.include` when assembling `main`: its folder, then
/// for it and its two parent folders, the folder itself and its `lib`
/// subfolder (the layout of `ports/asm`: `NOM/NOM.s` includes `lib/zp.inc`).
pub fn default_include_dirs(main: Option<&Path>) -> Vec<PathBuf> {
    let mut v = Vec::new();
    if let Some(d) = main.and_then(|p| p.parent()) {
        let mut cur = Some(d.to_path_buf());
        for _ in 0..3 {
            if let Some(c) = cur {
                for p in [c.clone(), c.join("lib")] {
                    if !v.contains(&p) {
                        v.push(p);
                    }
                }
                cur = c.parent().map(|x| x.to_path_buf());
            }
        }
    }
    v
}

/// Reads include files from the disk.
pub fn file_loader(name: &str, dirs: &[PathBuf]) -> Option<(PathBuf, String)> {
    let p = Path::new(name);
    if p.is_absolute() {
        return std::fs::read(p).ok().map(|b| (p.to_path_buf(), String::from_utf8_lossy(&b).into_owned()));
    }
    for d in dirs {
        let f = d.join(name);
        if let Ok(b) = std::fs::read(&f) {
            return Some((f, String::from_utf8_lossy(&b).into_owned()));
        }
    }
    // the standard definitions are built in: an unsaved tab, or a folder
    // without the file, still assembles (#31)
    if p.file_name().is_some_and(|n| n == "apple1.inc") {
        return Some((PathBuf::from("apple1.inc"), APPLE1_INC.to_string()));
    }
    None
}

/// `apple1.inc`: the Apple-1 addresses (KBD, DSP, ECHO, PRBYTE, …) used by the
/// examples of the manual.
pub const APPLE1_INC: &str = include_str!("../../../docs/exemples/en/asm/apple1.inc");

/// Assembles a file from the disk (includes found with [`default_include_dirs`] and `extra`).
pub fn assemble_file(path: &Path, extra: &[PathBuf]) -> Result<Assembly, Vec<AsmError>> {
    let text = match std::fs::read(path) {
        Ok(b) => String::from_utf8_lossy(&b).into_owned(),
        Err(e) => {
            return Err(vec![AsmError {
                file: 0,
                file_name: path.display().to_string(),
                path: path.to_path_buf(),
                line: 0,
                col: 0,
                kind: ErrKind::Include,
                detail: e.to_string(),
            }])
        }
    };
    assemble_text(&text, Some(path), extra)
}

/// Assembles `text` (the content of `main`, which may not be saved yet).
/// An xa source (`#include`, `#define`: crate::xa) is translated first,
/// with its included files.
pub fn assemble_text(text: &str, main: Option<&Path>, extra: &[PathBuf]) -> Result<Assembly, Vec<AsmError>> {
    let mut dirs: Vec<PathBuf> = extra.to_vec();
    dirs.extend(default_include_dirs(main));
    let main = main.map(|p| p.to_path_buf()).unwrap_or_default();
    if crate::xa::is_xa(text) {
        let load = |n: &str, d: &[PathBuf]| file_loader(n, d).map(|(p, t)| (p, crate::xa::to_ca65(&t)));
        return assemble(&crate::xa::to_ca65(text), main, &dirs, &load);
    }
    assemble(text, main, &dirs, &file_loader)
}

/// Assembles `text`; `.include` files are found by `load` in `dirs`
/// (the folder of the including file is tried first).
pub fn assemble(text: &str, main: PathBuf, dirs: &[PathBuf], load: &Loader) -> Result<Assembly, Vec<AsmError>> {
    let mut a = Asm::new(main, dirs, load);
    a.expand_text(0, text, 0);
    if let Some((name, loc)) = a.defining.take() {
        a.err(loc, 1, ErrKind::Block, format!(".macro {}", name));
    }
    if let Some(loc) = a.conds.last().map(|c| c.loc) {
        a.err(loc, 1, ErrKind::Block, ".endif".into());
    }
    if let Some(r) = a.rep.take() {
        a.err(r.loc, 1, ErrKind::Block, ".endrep".into());
    }
    let lines = std::mem::take(&mut a.lines);
    let mut items = a.parse(&lines);
    a.run(&mut items, false);
    a.run(&mut items, true);
    if !a.errors.is_empty() {
        let mut e = a.errors;
        e.sort_by_key(|x| (x.file, x.line, x.col));
        e.dedup_by(|x, y| x.file == y.file && x.line == y.line && x.kind == y.kind);
        e.truncate(100);
        return Err(e);
    }
    let mut symbols: Vec<(String, i64)> = Vec::new();
    for (k, d) in a.syms.iter() {
        if k.starts_with(':') || k.contains('\u{b7}') {
            continue;
        }
        let v = match d {
            Def::Label(v) => Some(*v),
            Def::Const(e) => a.eval(e, true, 0).ok().flatten(),
        };
        if let Some(v) = v {
            symbols.push((k.clone(), v));
        }
    }
    symbols.sort();
    let mut warnings = a.warnings;
    warnings.sort_by_key(|x| (x.file, x.line, x.col));
    warnings.dedup();
    Ok(Assembly { origin: a.origin.unwrap_or(DEFAULT_ORG), bytes: a.out, listing: a.listing, files: a.files, parents: a.parents, symbols, warnings })
}

#[derive(Debug, Clone)]
enum Def {
    Label(i64),
    Const(Expr),
}

struct Asm<'a> {
    files: Vec<PathBuf>,
    parents: Vec<(usize, usize)>,
    dirs: Vec<PathBuf>,
    load: &'a Loader<'a>,
    lines: Vec<Line>,
    errors: Vec<AsmError>,
    macros: HashMap<String, Macro>,
    /// Macro being defined: name, start.
    defining: Option<(String, Loc)>,
    def_params: Vec<String>,
    def_body: Vec<Line>,
    /// .if stack.
    conds: Vec<Cond>,
    /// `.repeat` being read.
    rep: Option<Rep>,
    /// Symbols seen while reading the source, with their value when it is a
    /// constant known there (for `.if`, `.ifdef`, `.repeat`).
    pre: HashMap<String, Option<i64>>,
    /// `.define NAME tokens`.
    defines: HashMap<String, Vec<T>>,
    /// `.feature string_escapes`.
    esc: bool,
    /// `.setcpu "65C02"` (while parsing).
    c02: bool,
    warnings: Vec<AsmError>,
    /// Warnings of the current item (kind, column, detail).
    pw: Vec<PErr>,
    item_exp: Vec<Option<String>>,
    local_n: usize,
    syms: HashMap<String, Def>,
    pc: i64,
    origin: Option<u16>,
    out: Vec<u8>,
    listing: Vec<ListLine>,
    item_locs: Vec<Loc>,
}


#[derive(Debug, Clone)]
struct Cond {
    /// Lines are assembled now.
    active: bool,
    /// A branch of this block was already taken.
    taken: bool,
    else_seen: bool,
    loc: Loc,
}

#[derive(Debug, Clone)]
struct Rep {
    count: i64,
    var: Option<String>,
    loc: Loc,
    outer: Option<Loc>,
    /// Nested `.repeat` inside the body.
    nest: usize,
    body: Vec<Line>,
}

/// Source text rebuilt from tokens (macro expansion in the listing).
fn detok(t: &[Tok]) -> String {
    let mut s = String::new();
    for (k, x) in t.iter().enumerate() {
        let txt = match &x.t {
            T::Num(v) if (0..10).contains(v) => v.to_string(),
            T::Num(v) if (0..0x100).contains(v) => format!("${:02X}", v),
            T::Num(v) if (0..0x10000).contains(v) => format!("${:04X}", v),
            other => tok_text(other),
        };
        let prev = if k > 0 { Some(&t[k - 1].t) } else { None };
        let space = match (prev, &x.t) {
            (None, _) => false,
            (Some(_), T::Comma) | (Some(_), T::RP) | (Some(_), T::Colon) => false,
            (Some(T::LP), _) | (Some(T::Hash), _) => false,
            (Some(T::Op(o)), _) if k >= 2 && matches!(t[k - 2].t, T::Hash | T::LP | T::Comma | T::Op(_)) && ["<", ">", "-", "~", "^"].contains(o) => false,
            (Some(T::Colon), _) => k > 1,
            _ => k == 1 || matches!(prev, Some(T::Comma)) || matches!(x.t, T::Op(_)) || matches!(prev, Some(T::Op(_))),
        };
        if space {
            s.push(' ');
        }
        s.push_str(&txt);
    }
    s
}

fn id_lower(t: Option<&Tok>) -> Option<String> {
    match t {
        Some(Tok { t: T::Id(s), .. }) => Some(s.to_ascii_lowercase()),
        _ => None,
    }
}

fn split_commas(t: &[Tok]) -> Vec<Vec<Tok>> {
    let mut v = vec![Vec::new()];
    let mut depth = 0i32;
    for x in t {
        match x.t {
            T::LP => depth += 1,
            T::RP => depth -= 1,
            T::Comma if depth == 0 => {
                v.push(Vec::new());
                continue;
            }
            _ => {}
        }
        v.last_mut().unwrap().push(x.clone());
    }
    v
}

impl<'a> Asm<'a> {
    fn new(main: PathBuf, dirs: &[PathBuf], load: &'a Loader<'a>) -> Asm<'a> {
        Asm {
            files: vec![main],
            parents: vec![(0, 0)],
            dirs: dirs.to_vec(),
            load,
            lines: Vec::new(),
            errors: Vec::new(),
            macros: HashMap::new(),
            defining: None,
            def_params: Vec::new(),
            def_body: Vec::new(),
            conds: Vec::new(),
            rep: None,
            pre: HashMap::new(),
            defines: HashMap::new(),
            esc: false,
            c02: false,
            warnings: Vec::new(),
            pw: Vec::new(),
            item_exp: Vec::new(),
            local_n: 0,
            syms: HashMap::new(),
            pc: DEFAULT_ORG as i64,
            origin: None,
            out: Vec::new(),
            listing: Vec::new(),
            item_locs: Vec::new(),
        }
    }

    fn err(&mut self, loc: Loc, col: usize, kind: ErrKind, detail: String) {
        let file_name = self.files.get(loc.file).and_then(|p| p.file_name()).map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        let path = self.files.get(loc.file).cloned().unwrap_or_default();
        self.errors.push(AsmError { file: loc.file, file_name, path, line: loc.line, col, kind, detail });
    }

    fn expand_text(&mut self, file: usize, text: &str, depth: usize) {
        for (n, l) in text.lines().enumerate() {
            let loc = Loc { file, line: n + 1 };
            match lex(l, self.esc) {
                Ok(t) => self.process(Line { loc, toks: t, exp: None }, depth, None),
                Err((k, c, d)) => {
                    if self.defining.is_none() {
                        self.err(loc, c, k, d)
                    }
                }
            }
        }
    }

    /// Macro definitions, conditionals, repeats, defines, includes and macro
    /// calls; other lines go to `self.lines`.
    fn process(&mut self, line: Line, depth: usize, outer: Option<Loc>) {
        let loc = outer.unwrap_or(line.loc);
        let first = id_lower(line.toks.first());
        if self.defining.is_some() {
            match first.as_deref() {
                Some(".endmacro") | Some(".endmac") => {
                    let (name, _) = self.defining.take().unwrap();
                    let m = Macro { params: std::mem::take(&mut self.def_params), body: std::mem::take(&mut self.def_body) };
                    self.macros.insert(name, m);
                }
                Some(".macro") | Some(".mac") => self.err(loc, line.toks[0].col, ErrKind::Block, ".macro".into()),
                _ => self.def_body.push(line),
            }
            return;
        }
        if let Some(r) = self.rep.as_mut() {
            match first.as_deref() {
                Some(".repeat") => {
                    r.nest += 1;
                    r.body.push(line);
                }
                Some(".endrep") | Some(".endrepeat") if r.nest > 0 => {
                    r.nest -= 1;
                    r.body.push(line);
                }
                Some(".endrep") | Some(".endrepeat") => {
                    let r = self.rep.take().unwrap();
                    self.expand_repeat(r, depth);
                }
                _ => r.body.push(line),
            }
            return;
        }
        let line = if matches!(first.as_deref(), Some(".define") | Some(".undefine") | Some(".undef") | Some(".ifdef") | Some(".ifndef")) || self.defines.is_empty() {
            line
        } else {
            self.subst_defines(line)
        };
        let col0 = line.toks.first().map(|t| t.col).unwrap_or(1);
        let parent_active = |c: &[Cond]| c.iter().all(|x| x.active);
        match first.as_deref() {
            Some(".ifblank") | Some(".ifnblank") | Some(".if") | Some(".ifdef") | Some(".ifndef") => {
                let active = parent_active(&self.conds);
                let v = if active { self.cond_value(first.as_deref().unwrap(), &line, loc) } else { false };
                self.conds.push(Cond { active: active && v, taken: active && v, else_seen: false, loc });
                return;
            }
            Some(".elseif") => {
                let n = self.conds.len();
                let outer_active = n < 2 || parent_active(&self.conds[..n - 1]);
                match self.conds.last() {
                    Some(c) if !c.else_seen => {
                        let taken = c.taken;
                        let v = if outer_active && !taken { self.cond_value(".if", &line, loc) } else { false };
                        let c = self.conds.last_mut().unwrap();
                        c.active = v;
                        c.taken = taken || v;
                    }
                    _ => self.err(loc, col0, ErrKind::Block, ".elseif".into()),
                }
                return;
            }
            Some(".else") => {
                let n = self.conds.len();
                let outer_active = n < 2 || parent_active(&self.conds[..n - 1]);
                match self.conds.last_mut() {
                    Some(c) if !c.else_seen => {
                        c.active = !c.taken && outer_active;
                        c.taken = true;
                        c.else_seen = true;
                    }
                    _ => self.err(loc, col0, ErrKind::Block, ".else".into()),
                }
                return;
            }
            Some(".endif") => {
                if self.conds.pop().is_none() {
                    self.err(loc, col0, ErrKind::Block, ".endif".into());
                }
                return;
            }
            _ => {}
        }
        if !parent_active(&self.conds) {
            return;
        }
        match first.as_deref() {
            Some(".macro") | Some(".mac") => {
                let name = match line.toks.get(1) {
                    Some(Tok { t: T::Id(n), .. }) => n.clone(),
                    _ => {
                        self.err(loc, col0, ErrKind::Syntax, ".macro".into());
                        return;
                    }
                };
                let mut params = Vec::new();
                for p in split_commas(&line.toks[2..]) {
                    match p.as_slice() {
                        [Tok { t: T::Id(s), .. }] => params.push(s.clone()),
                        [] if params.is_empty() => {}
                        _ => self.err(loc, p.first().map(|t| t.col).unwrap_or(1), ErrKind::Syntax, ".macro".into()),
                    }
                }
                self.defining = Some((name, loc));
                self.def_params = params;
                self.def_body.clear();
                return;
            }
            Some(".endmacro") | Some(".endmac") => {
                self.err(loc, col0, ErrKind::Block, ".endmacro".into());
                return;
            }
            Some(".endrep") | Some(".endrepeat") => {
                self.err(loc, col0, ErrKind::Block, ".endrep".into());
                return;
            }
            Some(".repeat") => {
                let parts = split_commas(&line.toks[1..]);
                let var = match parts.get(1).map(|v| v.as_slice()) {
                    None => None,
                    Some([Tok { t: T::Id(v), .. }]) if parts.len() == 2 => Some(v.clone()),
                    Some(v) => {
                        self.err(loc, v.first().map(|t| t.col).unwrap_or(col0), ErrKind::Syntax, ".repeat".into());
                        return;
                    }
                };
                let count = match self.const_expr(&parts[0], col0, ".repeat") {
                    Ok(v) if (0..=0x10000).contains(&v) => v,
                    Ok(v) => {
                        self.err(loc, col0, ErrKind::Range, v.to_string());
                        0
                    }
                    Err((k, c, d)) => {
                        self.err(loc, c, k, d);
                        0
                    }
                };
                self.rep = Some(Rep { count, var, loc, outer, nest: 0, body: Vec::new() });
                return;
            }
            Some(".define") => {
                match line.toks.get(1) {
                    Some(Tok { t: T::Id(n), .. }) if !n.starts_with('.') => {
                        let v = line.toks[2..].iter().map(|t| t.t.clone()).collect();
                        self.defines.insert(n.clone(), v);
                    }
                    _ => self.err(loc, col0, ErrKind::Syntax, ".define".into()),
                }
                return;
            }
            Some(".undefine") | Some(".undef") => {
                match line.toks.get(1) {
                    Some(Tok { t: T::Id(n), .. }) if line.toks.len() == 2 && self.defines.remove(n).is_some() => {}
                    _ => self.err(loc, col0, ErrKind::Undefined, ".undefine".into()),
                }
                return;
            }
            Some(".feature") => {
                for p in split_commas(&line.toks[1..]) {
                    match p.as_slice() {
                        [Tok { t: T::Id(f), .. }] if f.eq_ignore_ascii_case("string_escapes") => self.esc = true,
                        [Tok { t: T::Id(f), .. }, Tok { t: T::Op(o), .. }] if f.eq_ignore_ascii_case("string_escapes") && (*o == "+" || *o == "-") => self.esc = *o == "+",
                        _ => self.err(loc, p.first().map(|t| t.col).unwrap_or(col0), ErrKind::Unknown, ".feature".into()),
                    }
                }
                return;
            }
            Some(".include") => {
                let name = match line.toks.get(1) {
                    Some(Tok { t: T::Str(s), .. }) if line.toks.len() == 2 => String::from_utf8_lossy(s).into_owned(),
                    _ => {
                        self.err(loc, col0, ErrKind::Syntax, ".include".into());
                        return;
                    }
                };
                if depth > 16 {
                    self.err(loc, col0, ErrKind::Include, name);
                    return;
                }
                let mut dirs = Vec::new();
                if let Some(d) = self.files[line.loc.file].parent() {
                    dirs.push(d.to_path_buf());
                }
                dirs.extend(self.dirs.iter().cloned());
                match (self.load)(&name, &dirs) {
                    Some((p, text)) => {
                        self.files.push(p);
                        self.parents.push((line.loc.file, line.loc.line));
                        let f = self.files.len() - 1;
                        self.expand_text(f, &text, depth + 1);
                    }
                    None => self.err(loc, line.toks[1].col, ErrKind::Include, name),
                }
                return;
            }
            _ => {}
        }
        // macro call, possibly after a label
        let start = match (line.toks.first(), line.toks.get(1)) {
            (Some(Tok { t: T::Id(_), .. }), Some(Tok { t: T::Colon, .. })) => 2,
            (Some(Tok { t: T::Colon, .. }), _) => 1,
            _ => 0,
        };
        if start == 2 {
            if let Some(Tok { t: T::Id(n), .. }) = line.toks.first() {
                if !n.starts_with('@') {
                    self.pre.insert(n.clone(), None);
                }
            }
        }
        if let Some(Tok { t: T::Id(name), col }) = line.toks.get(start) {
            if self.macros.contains_key(name) {
                if depth > 32 {
                    self.err(loc, *col, ErrKind::Block, name.clone());
                    return;
                }
                if start > 0 {
                    self.lines.push(Line { loc: line.loc, toks: line.toks[..start].to_vec(), exp: line.exp.clone() });
                }
                let m = &self.macros[name];
                let args = if line.toks.len() > start + 1 { split_commas(&line.toks[start + 1..]) } else { Vec::new() };
                let params = m.params.clone();
                let body = m.body.clone();
                // .local names
                self.local_n += 1;
                let n = self.local_n;
                let mut locals: HashMap<String, String> = HashMap::new();
                for b in &body {
                    if id_lower(b.toks.first()).as_deref() == Some(".local") {
                        for t in &b.toks[1..] {
                            if let T::Id(s) = &t.t {
                                locals.insert(s.clone(), format!("{}\u{b7}{}", s, n));
                            }
                        }
                    }
                }
                let col0 = *col;
                for b in body {
                    if id_lower(b.toks.first()).as_deref() == Some(".local") {
                        continue;
                    }
                    let mut toks = Vec::new();
                    let mut shown = Vec::new();
                    for t in b.toks {
                        if let T::Id(s) = &t.t {
                            if let Some(k) = params.iter().position(|p| p == s) {
                                if let Some(a) = args.get(k) {
                                    toks.extend(a.iter().map(|x| Tok { t: x.t.clone(), col: col0 }));
                                    shown.extend(a.iter().cloned());
                                }
                                continue;
                            }
                            if let Some(r) = locals.get(s) {
                                toks.push(Tok { t: T::Id(r.clone()), col: col0 });
                                shown.push(t.clone());
                                continue;
                            }
                        }
                        shown.push(t.clone());
                        toks.push(Tok { t: t.t, col: col0 });
                    }
                    let exp = Some(detok(&shown));
                    self.process(Line { loc: b.loc, toks, exp }, depth + 1, Some(loc));
                }
                return;
            }
        }
        // constants known now (for .if / .repeat)
        if let (Some(Tok { t: T::Id(n), .. }), Some(Tok { t: T::Op(o), .. })) = (line.toks.first(), line.toks.get(1)) {
            if (*o == "=" || *o == ":=") && !n.starts_with('.') && !n.starts_with('@') {
                let v = self.const_expr(&line.toks[2..], 1, "").ok();
                self.pre.insert(n.clone(), v);
            }
        }
        self.lines.push(Line { loc, toks: line.toks, exp: line.exp });
    }

    /// `.repeat` body, `count` times.
    fn expand_repeat(&mut self, r: Rep, depth: usize) {
        if depth > 32 {
            self.err(r.loc, 1, ErrKind::Block, ".repeat".into());
            return;
        }
        for i in 0..r.count {
            for b in &r.body {
                let toks: Vec<Tok> = b
                    .toks
                    .iter()
                    .map(|t| match (&t.t, &r.var) {
                        (T::Id(s), Some(v)) if s == v => Tok { t: T::Num(i), col: t.col },
                        _ => t.clone(),
                    })
                    .collect();
                let exp = b.exp.as_ref().map(|_| detok(&toks));
                self.process(Line { loc: b.loc, toks, exp }, depth + 1, r.outer);
            }
        }
    }

    /// Replaces the `.define` names (recursively, at most 16 levels).
    fn subst_defines(&self, mut line: Line) -> Line {
        for _ in 0..16 {
            let mut changed = false;
            let mut out = Vec::with_capacity(line.toks.len());
            for t in line.toks {
                match &t.t {
                    T::Id(s) if self.defines.contains_key(s) => {
                        out.extend(self.defines[s].iter().map(|x| Tok { t: x.clone(), col: t.col }));
                        changed = true;
                    }
                    _ => out.push(t),
                }
            }
            line.toks = out;
            if !changed {
                break;
            }
        }
        line
    }

    /// Value of an expression needed while reading (`.if`, `.repeat`).
    fn const_expr(&self, t: &[Tok], col: usize, what: &str) -> Result<i64, PErr> {
        if t.is_empty() {
            return Err((ErrKind::Syntax, col, what.into()));
        }
        let mut p = P { t, i: 0, scope: "", unnamed: 0 };
        let e = p.expr()?;
        end(&p)?;
        match self.pre_eval(&e)? {
            Some(v) => Ok(v),
            None => Err((ErrKind::NotConst, t[0].col, what.into())),
        }
    }

    /// Condition of `.if` (or `.ifdef`…) on `line`.
    fn cond_value(&mut self, kind: &str, line: &Line, loc: Loc) -> bool {
        let rest = &line.toks[1..];
        let col0 = line.toks[0].col;
        match kind {
            ".ifblank" => rest.is_empty(),
            ".ifnblank" => !rest.is_empty(),
            ".ifdef" | ".ifndef" => match rest {
                [Tok { t: T::Id(n), .. }] => self.pre.contains_key(n) == (kind == ".ifdef"),
                _ => {
                    self.err(loc, col0, ErrKind::Syntax, kind.into());
                    false
                }
            },
            _ => match self.const_expr(rest, col0, ".if") {
                Ok(v) => v != 0,
                Err((k, c, d)) => {
                    self.err(loc, c, k, d);
                    false
                }
            },
        }
    }

    // ---------------------------------------------------------- parsing

    fn parse(&mut self, lines: &[Line]) -> Vec<Item> {
        let mut items = Vec::new();
        let mut scope = String::new();
        let mut unnamed = 0usize;
        for l in lines {
            let mut t: &[Tok] = &l.toks;
            // labels
            loop {
                match t {
                    [Tok { t: T::Id(n), col }, Tok { t: T::Colon, .. }, ..] if !n.starts_with('.') => {
                        let name = if n.starts_with('@') {
                            format!("{}{}", scope, n)
                        } else {
                            scope = n.clone();
                            n.clone()
                        };
                        items.push(Item::Label(name, *col));
                        self.item_locs.push(l.loc);
                        self.item_exp.push(l.exp.clone());
                        t = &t[2..];
                    }
                    [Tok { t: T::Colon, col }, ..] => {
                        items.push(Item::Label(format!(":{}", unnamed), *col));
                        self.item_locs.push(l.loc);
                        self.item_exp.push(l.exp.clone());
                        unnamed += 1;
                        t = &t[1..];
                    }
                    _ => break,
                }
            }
            if t.is_empty() {
                continue;
            }
            let mut p = P { t, i: 0, scope: &scope, unnamed };
            match self.parse_stmt(&mut p) {
                Ok(Some(it)) => {
                    items.push(it);
                    self.item_locs.push(l.loc);
                    self.item_exp.push(l.exp.clone());
                }
                Ok(None) => {}
                Err((k, c, d)) => self.err(l.loc, c, k, d),
            }
        }
        items
    }

    fn parse_stmt(&mut self, p: &mut P) -> Result<Option<Item>, PErr> {
        let t0 = &p.t[0];
        let (word, col) = match &t0.t {
            T::Id(w) => (w.clone(), t0.col),
            other => return Err((ErrKind::Syntax, t0.col, tok_text(other))),
        };
        // constant
        if let Some(Tok { t: T::Op(o), .. }) = p.t.get(1) {
            if (*o == "=" || *o == ":=") && !word.starts_with('.') {
                p.i = 2;
                let e = p.expr()?;
                end(p)?;
                let name = if word.starts_with('@') { format!("{}{}", p.scope, word) } else { word };
                return Ok(Some(Item::Const(name, e, col)));
            }
        }
        p.i = 1;
        let lw = word.to_ascii_lowercase();
        if lw.starts_with('.') {
            let it = match lw.as_str() {
                ".org" => {
                    let e = p.expr()?;
                    Item::Org(e, col)
                }
                ".byte" | ".byt" | ".asciiz" => {
                    let mut v = Vec::new();
                    loop {
                        match p.peek() {
                            Some(T::Str(s)) => {
                                v.push(ByteArg::S(s.clone()));
                                p.i += 1;
                            }
                            _ => {
                                let c = p.col();
                                v.push(ByteArg::E(p.expr()?, c));
                            }
                        }
                        if p.peek() == Some(&T::Comma) {
                            p.i += 1;
                        } else {
                            break;
                        }
                    }
                    Item::Bytes(v, lw == ".asciiz")
                }
                ".word" | ".addr" => {
                    let mut v = Vec::new();
                    loop {
                        let c = p.col();
                        v.push((p.expr()?, c));
                        if p.peek() == Some(&T::Comma) {
                            p.i += 1;
                        } else {
                            break;
                        }
                    }
                    Item::Words(v)
                }
                ".res" => {
                    let n = p.expr()?;
                    let f = if p.peek() == Some(&T::Comma) {
                        p.i += 1;
                        Some(p.expr()?)
                    } else {
                        None
                    };
                    Item::Res(n, f, col)
                }
                ".segment" => {
                    p.i = p.t.len();
                    return Ok(None);
                }
                ".code" | ".data" | ".rodata" | ".bss" => {
                    end(p)?;
                    return Ok(None);
                }
                ".local" => return Err((ErrKind::Block, col, word)),
                ".setcpu" => {
                    match p.peek() {
                        Some(T::Str(c)) if c.eq_ignore_ascii_case(b"6502") => self.c02 = false,
                        Some(T::Str(c)) if c.eq_ignore_ascii_case(b"65C02") => self.c02 = true,
                        _ => return Err((ErrKind::Unknown, p.col(), ".setcpu (\"6502\", \"65C02\")".into())),
                    }
                    p.i += 1;
                    end(p)?;
                    return Ok(None);
                }
                ".pc02" | ".p02" => {
                    end(p)?;
                    self.c02 = lw == ".pc02";
                    return Ok(None);
                }
                _ => return Err((ErrKind::Unknown, col, word)),
            };
            end(p)?;
            return Ok(Some(it));
        }
        let mn = word.to_ascii_uppercase();
        if !MNEMONICS.contains(&mn.as_str()) {
            if MNEMONICS_65C02.contains(&mn.as_str()) && !self.c02 {
                return Err((ErrKind::Unknown, col, format!("{} (65C02 : .setcpu \"65C02\")", word)));
            } else if !MNEMONICS_65C02.contains(&mn.as_str()) {
                return Err((ErrKind::Unknown, col, word));
            }
        }
        // address size prefix: a: (absolute), z: (zero page)
        let mut force = 0u8;
        let mut off = 1;
        if let [Tok { t: T::Id(z), .. }, Tok { t: T::Colon, .. }, _, ..] = &p.t[1..] {
            if z.eq_ignore_ascii_case("a") || z.eq_ignore_ascii_case("z") || z.eq_ignore_ascii_case("abs") || z.eq_ignore_ascii_case("zp") {
                force = z.as_bytes()[0].to_ascii_lowercase();
                off = 3;
            }
        }
        let rest = &p.t[off..];
        let op = if (mn.starts_with("BBR") || mn.starts_with("BBS")) && !rest.is_empty() {
            let parts = split_commas(rest);
            if parts.len() != 2 || parts.iter().any(|x| x.is_empty()) {
                return Err((ErrKind::BadMode, col, mn));
            }
            let ex = |t: &[Tok], p: &P| -> Result<Expr, PErr> {
                let mut q = P { t, i: 0, scope: p.scope, unnamed: p.unnamed };
                let e = q.expr()?;
                end(&q)?;
                Ok(e)
            };
            Operand::Pair(ex(&parts[0], p)?, ex(&parts[1], p)?)
        } else if rest.is_empty() {
            Operand::None
        } else if rest.len() == 1 && matches!(&rest[0].t, T::Id(a) if a.eq_ignore_ascii_case("a")) {
            Operand::Acc
        } else if rest[0].t == T::Hash {
            p.i = off + 1;
            let e = p.expr()?;
            end(p)?;
            Operand::Imm(e)
        } else {
            let n = rest.len();
            let idx = |t: &Tok| match &t.t {
                T::Id(r) if r.eq_ignore_ascii_case("x") => Some(b'X'),
                T::Id(r) if r.eq_ignore_ascii_case("y") => Some(b'Y'),
                _ => None,
            };
            // matching parenthesis of a leading one
            let close = if rest[0].t == T::LP {
                let mut d = 0;
                let mut m = None;
                for (k, x) in rest.iter().enumerate() {
                    match x.t {
                        T::LP => d += 1,
                        T::RP => {
                            d -= 1;
                            if d == 0 {
                                m = Some(k);
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                m
            } else {
                None
            };
            let sub = |a: usize, b: usize, p: &P| -> Result<Expr, PErr> {
                let mut q = P { t: &rest[a..b], i: 0, scope: p.scope, unnamed: p.unnamed };
                let e = q.expr()?;
                if q.i != b - a {
                    return Err((ErrKind::Syntax, rest[a + q.i].col, tok_text(&rest[a + q.i].t)));
                }
                Ok(e)
            };
            match close {
                Some(m) if m == n - 1 && n >= 5 && rest[m - 2].t == T::Comma && idx(&rest[m - 1]) == Some(b'X') => Operand::IndX(sub(1, m - 2, p)?),
                Some(m) if m == n - 3 && rest[m + 1].t == T::Comma && idx(&rest[m + 2]) == Some(b'Y') => Operand::IndY(sub(1, m, p)?),
                Some(m) if m == n - 1 => Operand::Ind(sub(1, m, p)?),
                _ => {
                    if n >= 3 && rest[n - 2].t == T::Comma && idx(&rest[n - 1]).is_some() {
                        Operand::Addr(sub(0, n - 2, p)?, idx(&rest[n - 1]).unwrap())
                    } else {
                        Operand::Addr(sub(0, n, p)?, 0)
                    }
                }
            }
        };
        Ok(Some(Item::Instr { mn, op, col, mode: None, c02: self.c02, force }))
    }

    // ------------------------------------------------------- evaluation

    /// Value of `e`: `Ok(None)` if a symbol is not known (yet).
    fn eval(&self, e: &Expr, fin: bool, depth: usize) -> Result<Option<i64>, PErr> {
        Ok(Some(match e {
            Expr::Num(v) => *v,
            Expr::Pc => self.pc,
            Expr::Sym(s, col) => match self.syms.get(s) {
                Some(Def::Label(v)) => *v,
                Some(Def::Const(x)) => {
                    if depth > 64 {
                        return Err((ErrKind::Undefined, *col, s.clone()));
                    }
                    match self.eval(x, fin, depth + 1)? {
                        Some(v) => v,
                        None => return Ok(None),
                    }
                }
                None if fin => return Err((ErrKind::Undefined, *col, display_name(s))),
                None => return Ok(None),
            },
            Expr::Defined(n) => self.syms.contains_key(n) as i64,
            Expr::Un(o, a) => {
                let Some(v) = self.eval(a, fin, depth)? else { return Ok(None) };
                un_op(o, v)
            }
            Expr::Bin(o, a, b) => {
                let x = self.eval(a, fin, depth)?;
                let y = self.eval(b, fin, depth)?;
                let (Some(x), Some(y)) = (x, y) else { return Ok(None) };
                bin_op(o, x, y, b)?
            }
        }))
    }

    /// Value of `e` while reading the source (`.if`, `.repeat`): only the
    /// constants already defined, like ca65 which needs it known there.
    fn pre_eval(&self, e: &Expr) -> Result<Option<i64>, PErr> {
        Ok(Some(match e {
            Expr::Num(v) => *v,
            Expr::Pc => return Ok(None),
            Expr::Sym(s, _) => match self.pre.get(s) {
                Some(Some(v)) => *v,
                _ => return Ok(None),
            },
            Expr::Defined(n) => self.pre.contains_key(n) as i64,
            Expr::Un(o, a) => {
                let Some(v) = self.pre_eval(a)? else { return Ok(None) };
                un_op(o, v)
            }
            Expr::Bin(o, a, b) => {
                let x = self.pre_eval(a)?;
                let y = self.pre_eval(b)?;
                let (Some(x), Some(y)) = (x, y) else { return Ok(None) };
                bin_op(o, x, y, b)?
            }
        }))
    }
    // (end of the evaluators)
}

fn un_op(o: &str, v: i64) -> i64 {
    match o {
        "-" => -v,
        "+" => v,
        "~" => !v,
        "!" => (v == 0) as i64,
        "<" => v & 0xFF,
        ">" => (v >> 8) & 0xFF,
        _ => (v >> 16) & 0xFF,
    }
}

fn bin_op(o: &str, x: i64, y: i64, b: &Expr) -> Result<i64, PErr> {
    Ok(match o {
                    "+" => x.wrapping_add(y),
                    "-" => x.wrapping_sub(y),
                    "*" => x.wrapping_mul(y),
                    "/" => {
                        if y == 0 {
                            return Err((ErrKind::DivZero, expr_col(b), String::new()));
                        }
                        x / y
                    }
                    "&" => x & y,
                    "|" => x | y,
                    "^" => x ^ y,
                    "<<" => x.wrapping_shl(y as u32),
                    ">>" => x.wrapping_shr(y as u32),
                    "=" => (x == y) as i64,
                    "<>" => (x != y) as i64,
                    "<" => (x < y) as i64,
                    ">" => (x > y) as i64,
                    "<=" => (x <= y) as i64,
                    ">=" => (x >= y) as i64,
                    "&&" => (x != 0 && y != 0) as i64,
                    _ => (x != 0 || y != 0) as i64,
    })
}

impl<'a> Asm<'a> {

    fn emit(&mut self, b: &[u8]) {
        self.out.extend_from_slice(b);
        self.pc += b.len() as i64;
    }

    /// Pass 1 (`fin` false: sizes and labels) or pass 2 (bytes).
    fn run(&mut self, items: &mut [Item], fin: bool) {
        self.pc = DEFAULT_ORG as i64;
        self.out.clear();
        self.listing.clear();
        self.warnings.clear();
        if !fin {
            self.syms.clear();
            self.origin = None;
        }
        #[allow(clippy::needless_range_loop)]
        for k in 0..items.len() {
            let loc = self.item_locs[k];
            let exp = self.item_exp[k].clone();
            let start_pc = self.pc;
            let start_len = self.out.len();
            match self.step(&items[k], fin) {
                Ok(Some(m)) => {
                    if let Item::Instr { mode, .. } = &mut items[k] {
                        *mode = Some(m);
                    }
                }
                Ok(None) => {}
                Err((kd, c, d)) => {
                    let c = if c == 0 { item_col(&items[k]) } else { c };
                    self.err(loc, c, kd, d)
                }
            }
            for (kd, c, d) in std::mem::take(&mut self.pw) {
                let file_name = self.files.get(loc.file).and_then(|p| p.file_name()).map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                let path = self.files.get(loc.file).cloned().unwrap_or_default();
                self.warnings.push(AsmError { file: loc.file, file_name, path, line: loc.line, col: c, kind: kd, detail: d });
            }
            if self.pc > 0x10000 {
                self.err(loc, 1, ErrKind::Range, format!("${:X}", self.pc));
                return;
            }
            if fin && self.out.len() > start_len {
                let bytes = self.out[start_len..].to_vec();
                match self.listing.last_mut() {
                    Some(l) if l.file == loc.file && l.line == loc.line && l.exp == exp && l.addr as usize + l.bytes.len() == start_pc as usize => l.bytes.extend(bytes),
                    _ => self.listing.push(ListLine { file: loc.file, line: loc.line, addr: start_pc as u16, bytes, exp }),
                }
            }
        }
    }

    fn known(&self, e: &Expr) -> Option<i64> {
        self.eval(e, false, 0).ok().flatten()
    }

    fn val(&self, e: &Expr, fin: bool) -> Result<i64, PErr> {
        Ok(self.eval(e, fin, 0)?.unwrap_or(0))
    }

    /// One item; for an instruction, returns the mode chosen in pass 1.
    fn step(&mut self, it: &Item, fin: bool) -> Result<Option<Mode>, PErr> {
        match it {
            Item::Label(n, col) => {
                if !fin {
                    if self.syms.contains_key(n) {
                        return Err((ErrKind::Duplicate, *col, display_name(n)));
                    }
                    self.syms.insert(n.clone(), Def::Label(self.pc));
                } else if let Some(Def::Label(v)) = self.syms.get(n) {
                    // a source with errors (duplicate label, bad line) may
                    // lay out differently: only check a clean pass 1
                    debug_assert!(*v == self.pc || !self.errors.is_empty(), "label {n}: ${v:04X} != ${:04X}", self.pc);
                }
            }
            Item::Const(n, e, col) => {
                if !fin {
                    if self.syms.contains_key(n) {
                        return Err((ErrKind::Duplicate, *col, display_name(n)));
                    }
                    // `*` is frozen where the constant is defined
                    let e = subst_pc(e, self.pc);
                    self.syms.insert(n.clone(), Def::Const(e));
                } else if let Some(Def::Const(x)) = self.syms.get(n).cloned() {
                    self.eval(&x, true, 0)?;
                }
            }
            Item::Org(e, col) => {
                let v = match self.eval(e, fin, 0)? {
                    Some(v) => v,
                    None => return Err((ErrKind::NotConst, *col, ".org".into())),
                };
                if !(0..=0xFFFF).contains(&v) {
                    return Err((ErrKind::Range, *col, format!("${:X}", v)));
                }
                if self.origin.is_none() && !fin {
                    self.origin = Some(v as u16);
                }
                self.pc = v;
            }
            Item::Res(n, f, col) => {
                let c = match self.eval(n, fin, 0)? {
                    Some(c) if (0..=0x10000).contains(&c) => c,
                    Some(c) => return Err((ErrKind::Range, *col, c.to_string())),
                    None => return Err((ErrKind::NotConst, *col, ".res".into())),
                };
                let fill = match f {
                    Some(f) => byte(self.val(f, fin)?, expr_col(f), fin)?,
                    None => 0,
                };
                self.note_origin();
                self.emit(&vec![fill; c as usize]);
            }
            Item::Bytes(v, z) => {
                self.note_origin();
                // like ca65, `*` is the address of each element
                let pc0 = self.pc;
                let mut b = Vec::new();
                for a in v {
                    self.pc = pc0 + b.len() as i64;
                    match a {
                        ByteArg::S(s) => b.extend_from_slice(s),
                        ByteArg::E(e, c) => b.push(byte(self.val(e, fin)?, *c, fin)?),
                    }
                }
                if *z {
                    b.push(0);
                }
                self.pc = pc0;
                self.emit(&b);
            }
            Item::Words(v) => {
                self.note_origin();
                let pc0 = self.pc;
                let mut b = Vec::new();
                for (e, c) in v {
                    self.pc = pc0 + b.len() as i64;
                    let w = self.val(e, fin)?;
                    if fin && !(0..=0xFFFF).contains(&w) {
                        return Err((ErrKind::Range, *c, w.to_string()));
                    }
                    b.push(w as u8);
                    b.push((w >> 8) as u8);
                }
                self.pc = pc0;
                self.emit(&b);
            }
            Item::Instr { mn, op, col, mode, c02, force } => {
                self.note_origin();
                let m = match mode {
                    Some(m) => *m,
                    None => self.choose(mn, op, *col, *c02, *force)?,
                };
                let code = opc(mn, m, *c02).ok_or((ErrKind::BadMode, *col, mn.clone()))?;
                let mut b = vec![code];
                if let Operand::Pair(z, t) = op {
                    let zv = self.val(z, fin)?;
                    b.push(byte(zv, expr_col(z).max(*col), fin)?);
                    let v = self.val(t, fin)?;
                    let d = v - (self.pc + 3);
                    if fin && !(-128..=127).contains(&d) {
                        return Err((ErrKind::Branch, expr_col(t).max(*col), d.to_string()));
                    }
                    b.push(d as u8);
                    self.emit(&b);
                    return Ok(Some(m));
                }
                let e = match op {
                    Operand::Imm(e) | Operand::Addr(e, _) | Operand::Ind(e) | Operand::IndX(e) | Operand::IndY(e) => Some(e),
                    _ => None,
                };
                if let Some(e) = e {
                    let v = self.val(e, fin)?;
                    let c = expr_col(e).max(*col);
                    match m {
                        Mode::Rel => {
                            let d = v - (self.pc + 2);
                            if fin && !(-128..=127).contains(&d) {
                                return Err((ErrKind::Branch, c, d.to_string()));
                            }
                            b.push(d as u8);
                        }
                        Mode::Abs | Mode::AbsX | Mode::AbsY | Mode::Ind | Mode::AbsIndX => {
                            if fin && !(0..=0xFFFF).contains(&v) {
                                return Err((ErrKind::Range, c, v.to_string()));
                            }
                            // zero page value on 2 bytes: symbol defined further down
                            let z = match m {
                                Mode::Abs => Some(Mode::Zp),
                                Mode::AbsX => Some(Mode::ZpX),
                                Mode::AbsY => Some(Mode::ZpY),
                                _ => None,
                            };
                            if fin && *force == 0 && (0..0x100).contains(&v) && z.is_some_and(|z| opc(mn, z, *c02).is_some()) {
                                let name = sym_name(e).map(|n| display_name(&n)).unwrap_or_default();
                                self.pw.push((ErrKind::ZpAbs, c, format!("{} = ${:02X}", name, v)));
                            }
                            b.push(v as u8);
                            b.push((v >> 8) as u8);
                        }
                        _ => b.push(byte(v, c, fin)?),
                    }
                }
                self.emit(&b);
                return Ok(Some(m));
            }
        }
        Ok(None)
    }

    fn note_origin(&mut self) {
        if self.origin.is_none() {
            self.origin = Some(self.pc as u16);
        }
    }

    /// Addressing mode, decided in pass 1 like ca65: zero page if the value is known and below $100.
    fn choose(&self, mn: &str, op: &Operand, col: usize, c02: bool, force: u8) -> Result<Mode, PErr> {
        let bad = || (ErrKind::BadMode, col, mn.to_string());
        let has = |m: Mode| opc(mn, m, c02).is_some();
        let m = match op {
            Operand::None => {
                if has(Mode::Imp) {
                    Mode::Imp
                } else if has(Mode::Acc) {
                    Mode::Acc
                } else {
                    return Err(bad());
                }
            }
            Operand::Acc => Mode::Acc,
            Operand::Imm(_) => Mode::Imm,
            Operand::Ind(_) if mn == "JMP" => Mode::Ind,
            Operand::Ind(_) => Mode::ZpInd,
            Operand::IndX(_) if mn == "JMP" => Mode::AbsIndX,
            Operand::IndX(_) => Mode::IndX,
            Operand::Pair(..) => Mode::ZpRel,
            Operand::IndY(_) => Mode::IndY,
            Operand::Addr(e, r) => {
                if is_branch(mn) && *r == 0 {
                    Mode::Rel
                } else {
                    let zp = match force {
                        b'a' => false,
                        b'z' => true,
                        _ => matches!(self.known(e), Some(v) if (0..0x100).contains(&v)),
                    };
                    let (z, a) = match r {
                        b'X' => (Mode::ZpX, Mode::AbsX),
                        b'Y' => (Mode::ZpY, Mode::AbsY),
                        _ => (Mode::Zp, Mode::Abs),
                    };
                    if zp && has(z) {
                        z
                    } else {
                        a
                    }
                }
            }
        };
        if has(m) {
            Ok(m)
        } else {
            Err(bad())
        }
    }
}

/// First symbol of an expression (for messages).
fn sym_name(e: &Expr) -> Option<String> {
    match e {
        Expr::Sym(s, _) => Some(s.clone()),
        Expr::Un(_, a) => sym_name(a),
        Expr::Bin(_, a, b) => sym_name(a).or_else(|| sym_name(b)),
        _ => None,
    }
}

fn byte(v: i64, col: usize, fin: bool) -> Result<u8, PErr> {
    if fin && !(0..=0xFF).contains(&v) {
        return Err((ErrKind::Range, col, v.to_string()));
    }
    Ok(v as u8)
}

fn subst_pc(e: &Expr, pc: i64) -> Expr {
    match e {
        Expr::Pc => Expr::Num(pc),
        Expr::Un(o, a) => Expr::Un(o, Box::new(subst_pc(a, pc))),
        Expr::Bin(o, a, b) => Expr::Bin(o, Box::new(subst_pc(a, pc)), Box::new(subst_pc(b, pc))),
        x => x.clone(),
    }
}

fn display_name(s: &str) -> String {
    if let Some(n) = s.strip_prefix(':') {
        let _ = n;
        ":+/:-".into()
    } else if let Some(k) = s.find('@') {
        s[k..].to_string()
    } else if let Some(k) = s.find('\u{b7}') {
        s[..k].to_string()
    } else {
        s.to_string()
    }
}

/// Column of the first symbol of `e` (0 if none: the caller uses its own).
fn expr_col(e: &Expr) -> usize {
    match e {
        Expr::Sym(_, c) => *c,
        Expr::Un(_, a) => expr_col(a),
        Expr::Bin(_, a, b) => {
            let c = expr_col(a);
            if c > 0 {
                c
            } else {
                expr_col(b)
            }
        }
        _ => 0,
    }
}

fn item_col(it: &Item) -> usize {
    match it {
        Item::Label(_, c) | Item::Const(_, _, c) | Item::Instr { col: c, .. } | Item::Res(_, _, c) | Item::Org(_, c) => *c,
        Item::Bytes(v, _) => match v.first() {
            Some(ByteArg::E(_, c)) => *c,
            _ => 1,
        },
        Item::Words(v) => v.first().map(|x| x.1).unwrap_or(1),
    }
}

fn end(p: &P) -> Result<(), PErr> {
    if p.i < p.t.len() {
        Err((ErrKind::Syntax, p.t[p.i].col, tok_text(&p.t[p.i].t)))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn asm(src: &str) -> Assembly {
        assemble(src, PathBuf::from("t.s"), &[], &|_: &str, _: &[PathBuf]| None).unwrap_or_else(|e| panic!("{:?}", e))
    }
    fn errs(src: &str) -> Vec<AsmError> {
        assemble(src, PathBuf::from("t.s"), &[], &|_: &str, _: &[PathBuf]| None).expect_err("error expected")
    }

    #[test]
    fn every_official_opcode() {
        use Mode::*;
        let modes = [Imp, Acc, Imm, Zp, ZpX, ZpY, Abs, AbsX, AbsY, Ind, IndX, IndY, Rel];
        let mut seen = std::collections::HashSet::new();
        for m in MNEMONICS {
            for md in modes {
                if let Some(o) = opcode(m, md) {
                    assert!(seen.insert(o), "{m} {md:?} {o:02X} twice");
                }
            }
        }
        assert_eq!(seen.len(), 151);
    }

    #[test]
    fn modes_and_zero_page() {
        let a = asm("ZP = $12\n lda ZP\n lda FW\n lda ZP,x\n ldx ZP,y\n lda ZP,y\n lda (ZP),y\n lda (ZP,x)\n jmp ($1234)\n asl\n asl a\n lda #<FW\n rts\nFW = $34\n");
        assert_eq!(a.origin, 0x300);
        assert_eq!(
            a.bytes,
            vec![0xA5, 0x12, 0xAD, 0x34, 0x00, 0xB5, 0x12, 0xB6, 0x12, 0xB9, 0x12, 0x00, 0xB1, 0x12, 0xA1, 0x12, 0x6C, 0x34, 0x12, 0x0A, 0x0A, 0xA9, 0x34, 0x60]
        );
    }

    #[test]
    fn labels_expressions_data() {
        let a = asm(".org $1000\nS: nop\n@l: dex\n bne @l\n: beq :-\n bcc :+\n nop\n: .word S, *, S+1\n .byte \"AB\", 'C', <S, >S, 2*3+4/2-1, (1<<3)|2, ~0&$FF\n .asciiz \"Z\"\n .res 2, $EA\n lda #>S+1\n");
        assert_eq!(a.origin, 0x1000);
        assert_eq!(&a.bytes[..10], &[0xEA, 0xCA, 0xD0, 0xFD, 0xF0, 0xFE, 0x90, 0x01, 0xEA, 0x00]);
        assert_eq!(&a.bytes[9..15], &[0x00, 0x10, 0x0B, 0x10, 0x01, 0x10]);
        assert_eq!(&a.bytes[15..24], &[0x41, 0x42, 0x43, 0x00, 0x10, 7, 10, 0xFF, 0x5A]);
        assert_eq!(&a.bytes[24..], &[0x00, 0xEA, 0xEA, 0xA9, 0x11]);
        assert_eq!(a.aci_range(), "1000.101CR");
        assert_eq!(a.run_command(), "1000R");
    }

    #[test]
    fn macros_like_ca65() {
        let src = ".macro PRT p1,p2\n jsr $1234\n .byte p1\n .ifnblank p2\n .byte p2\n .endif\n .byte 0\n.endmacro\n.macro TWO v\n .local x\nx: lda #<(v)\n PRT \"A\"\n.endmacro\nL: PRT \"HI\",13\n PRT \"B\"\n TWO $3456\n TWO 1\n";
        let a = asm(src);
        assert_eq!(
            a.bytes,
            vec![0x20, 0x34, 0x12, b'H', b'I', 13, 0, 0x20, 0x34, 0x12, b'B', 0, 0xA9, 0x56, 0x20, 0x34, 0x12, b'A', 0, 0xA9, 1, 0x20, 0x34, 0x12, b'A', 0]
        );
        // a macro local label ends the scope of cheap locals, like ca65
        let e = errs(".macro M\n .local s\ns: nop\n.endmacro\n@l: nop\n M\n bne @l\n");
        assert_eq!((e[0].kind, e[0].line), (ErrKind::Undefined, 7));
    }

    #[test]
    fn include_files() {
        let load = |n: &str, d: &[PathBuf]| -> Option<(PathBuf, String)> {
            assert_eq!(d[0], PathBuf::from("/src"));
            (n == "zp.inc").then(|| (PathBuf::from("/lib/zp.inc"), "PTR = $30\n".to_string()))
        };
        let a = assemble(" .include \"zp.inc\"\n lda PTR\n", PathBuf::from("/src/m.s"), &[], &load).unwrap();
        assert_eq!(a.bytes, vec![0xA5, 0x30]);
        assert_eq!(a.files.len(), 2);
        let e = assemble(" .include \"no.inc\"\n", PathBuf::from("/src/m.s"), &[], &load).unwrap_err();
        assert_eq!((e[0].kind, e[0].line, e[0].col), (ErrKind::Include, 1, 11));
    }

    #[test]
    fn errors_with_line_and_column() {
        let e = errs(" lda #300\n jmp nowhere\n stx $1234,x\nS: nop\nS: nop\n foo\n lda #\"x\n .byte 1/0\n bne S+200\n");
        let k: Vec<(ErrKind, usize, usize)> = e.iter().map(|e| (e.kind, e.line, e.col)).collect();
        assert_eq!(
            k,
            vec![
                (ErrKind::Range, 1, 2),
                (ErrKind::Undefined, 2, 6),
                (ErrKind::BadMode, 3, 2),
                (ErrKind::Duplicate, 5, 1),
                (ErrKind::Unknown, 6, 2),
                (ErrKind::String, 7, 7),
                (ErrKind::DivZero, 8, 8),
                (ErrKind::Branch, 9, 6),
            ]
        );
        assert_eq!(e[1].detail, "nowhere");
        assert!(e[0].to_string().starts_with("t.s:1:2 : valeur hors limites"));
    }

    // The expected bytes below come from ca65 V2.18 + ld65 (apple1.cfg) on
    // the same sources (`--cpu 65C02` for the last one).
    #[test]
    fn conditionals_like_ca65() {
        let a = asm(r#"VER = 2
DEBUG = 1
.if VER = 1
 .byte 1
.elseif VER = 2
 .byte 2
 .if DEBUG
  .byte $D0
 .else
  .byte $D1
 .endif
.elseif VER = 3
 .byte 3
.else
 .byte 4
.endif
.ifdef DEBUG
 .byte $DE
.endif
.ifndef NOPE
 .byte $AB
.endif
.ifdef NOPE
 .byte $FF
.else
 .byte $EE
.endif
.if (.defined(VER)) && (!.defined(NOPE))
 .byte $77
.endif
.if 0
 .if 1
  .byte $99
 .else
  .byte $98
 .endif
.elseif VER > 1 && VER < 3
 .byte $55
.endif
start:
.ifdef start
 .byte <start
.endif
"#);
        assert_eq!(a.bytes, vec![0x02, 0xD0, 0xDE, 0xAB, 0xEE, 0x77, 0x55, 0x07]);
        let e = errs(".if LATER\n.endif\nLATER = 1\n.else\n.elseif 1\n");
        let k: Vec<(ErrKind, usize)> = e.iter().map(|e| (e.kind, e.line)).collect();
        assert_eq!(k, vec![(ErrKind::NotConst, 1), (ErrKind::Block, 4), (ErrKind::Block, 5)]);
        assert_eq!(errs(".if 1\n nop\n")[0].kind, ErrKind::Block);
    }

    #[test]
    fn repeat_like_ca65() {
        let a = asm(r#".repeat 4, I
 .byte I*2+1
.endrep
.repeat 3
 nop
.endrepeat
N = 2
.repeat N, J
 .repeat 2, K
  .byte J*16+K
 .endrep
.endrep
.repeat 0
 .byte $FF
.endrep
.macro FILL n, v
 .repeat n
  .byte v
 .endrep
.endmacro
 FILL 3, $AA
"#);
        assert_eq!(a.bytes, vec![0x01, 0x03, 0x05, 0x07, 0xEA, 0xEA, 0xEA, 0x00, 0x01, 0x10, 0x11, 0xAA, 0xAA, 0xAA]);
        assert_eq!(errs(".repeat 2\n nop\n")[0].kind, ErrKind::Block);
        assert_eq!(errs(".repeat N\n.endrep\nN = 1\n")[0].kind, ErrKind::NotConst);
    }

    #[test]
    fn define_like_ca65() {
        let a = asm(r#".define PTR $30
.define CNT 4
.define LOADPTR lda PTR
 LOADPTR
 sta PTR+1
 ldx #CNT
.define VAL CNT*2
 .byte VAL
.undefine CNT
CNT = 9
 .byte CNT
"#);
        assert_eq!(a.bytes, vec![0xA5, 0x30, 0x85, 0x31, 0xA2, 0x04, 0x08, 0x09]);
    }

    #[test]
    fn size_prefixes_and_escapes_like_ca65() {
        let a = asm(r#"ZP = $10
 lda a:ZP
 lda a:$12,x
 sta a:$20,y
 lda z:FWD
 ldx z:FWD,y
 lda FWD
 lda ZP
 jmp a:$1234
FWD = $40
 .feature string_escapes
 .byte "a\nb\t\\\"\x41\r\'"
"#);
        assert_eq!(a.bytes, vec![0xAD, 0x10, 0x00, 0xBD, 0x12, 0x00, 0x99, 0x20, 0x00, 0xA5, 0x40, 0xB6, 0x40, 0xAD, 0x40, 0x00, 0xA5, 0x10, 0x4C, 0x34, 0x12, 0x61, 0x0A, 0x62, 0x09, 0x5C, 0x22, 0x41, 0x0D, 0x27]);
        // ca65 too: "Didn't use zeropage addressing for 'FWD'"
        let w: Vec<(ErrKind, usize, &str)> = a.warnings.iter().map(|w| (w.kind, w.line, w.detail.as_str())).collect();
        assert_eq!(w, vec![(ErrKind::ZpAbs, 7, "FWD = $40")]);
        // without .feature string_escapes a backslash is a character
        assert_eq!(asm(" .byte \"a\\n\"\n").bytes, b"a\\n");
        assert_eq!(errs(" .feature string_escapes\n .byte \"\\q\"\n")[0].kind, ErrKind::String);
        assert_eq!(errs(" lda z:$1234\n")[0].kind, ErrKind::Range);
    }

    #[test]
    fn cpu_65c02_like_ca65() {
        let a = asm(r#".setcpu "65C02"
ZP = $12
L: bra L
 phx
 phy
 plx
 ply
 stz ZP
 stz ZP,x
 stz $1234
 stz $1234,x
 trb ZP
 trb $1234
 tsb ZP
 tsb $1234
 lda (ZP)
 sta (ZP)
 ora (ZP)
 and (ZP)
 eor (ZP)
 adc (ZP)
 cmp (ZP)
 sbc (ZP)
 bit #$80
 bit ZP,x
 bit $1234,x
 inc a
 dec a
 inc
 dec
 jmp ($1234,x)
 jmp ($1234)
 rmb0 ZP
 rmb7 ZP
 smb0 ZP
 smb5 ZP
 bbr0 ZP, L
 bbs7 ZP, M
 bbr3 $FF, M
M: stp
 wai
 bra M
.setcpu "6502"
 lda ZP
"#);
        assert_eq!(a.bytes, vec![0x80, 0xFE, 0xDA, 0x5A, 0xFA, 0x7A, 0x64, 0x12, 0x74, 0x12, 0x9C, 0x34, 0x12, 0x9E, 0x34, 0x12, 0x14, 0x12, 0x1C, 0x34, 0x12, 0x04, 0x12, 0x0C, 0x34, 0x12, 0xB2, 0x12, 0x92, 0x12, 0x12, 0x12, 0x32, 0x12, 0x52, 0x12, 0x72, 0x12, 0xD2, 0x12, 0xF2, 0x12, 0x89, 0x80, 0x34, 0x12, 0x3C, 0x34, 0x12, 0x1A, 0x3A, 0x1A, 0x3A, 0x7C, 0x34, 0x12, 0x6C, 0x34, 0x12, 0x07, 0x12, 0x77, 0x12, 0x87, 0x12, 0xD7, 0x12, 0x0F, 0x12, 0xBA, 0xFF, 0x12, 0x03, 0x3F, 0xFF, 0x00, 0xDB, 0xCB, 0x80, 0xFC, 0xA5, 0x12]);
        let e = errs(" stz $12\n");
        assert_eq!((e[0].kind, e[0].detail.as_str()), (ErrKind::Unknown, "stz (65C02 : .setcpu \"65C02\")"));
        assert_eq!(errs(".pc02\n lda ($1234)\n")[0].kind, ErrKind::Range);
        // the disassembler of the debugger stays NMOS (the Apple-1 has a 6502)
        assert_eq!(decode(0x80), None);
    }

    #[test]
    fn listing_shows_macros_and_symbols() {
        let src = ".macro PUT c\n lda #c\n jsr $FFEF\n.endmacro\nSTART: PUT 'A'\n rts\n";
        let a = asm(src);
        let t: String = listing_rows(&a, Some(src)).iter().map(|r| format!("{}\n", r.text.trim_end())).collect();
        let want = "                  .macro PUT c\n                   lda #c\n                   jsr $FFEF\n                  .endmacro\n                  START: PUT 'A'\n0300  A9 41           + lda #$41\n0302  20 EF FF        + jsr $FFEF\n0305  60           rts\n\n; ---- Symboles (1)\n; START     $0300\n";
        assert_eq!(t, want);
        let rows = listing_rows(&a, None);
        assert_eq!((rows[5].line, rows[5].addr), (5, Some(0x300)));
    }

    /// The ports of `ports/` (outside this repository), when present: same bytes as ca65 + ld65.
    #[test]
    fn ports_same_as_ca65() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../ports");
        let mut list = vec![(root.join("gunner.s"), root.join("gunner.bin"))];
        for n in ["guner1", "splat", "rockt1", "orbit", "target"] {
            list.push((root.join(format!("asm/{n}/{n}.s")), root.join(format!("asm/{n}/{n}.bin"))));
        }
        for (s, b) in list {
            let Ok(want) = std::fs::read(&b) else { continue };
            let a = assemble_file(&s, &[]).unwrap_or_else(|e| panic!("{}: {:?}", s.display(), e));
            assert_eq!(a.origin, 0x300);
            assert!(a.bytes == want, "{} differs", s.display());
        }
    }
}
