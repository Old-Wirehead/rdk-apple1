//! Native tokenizer of the Apple-1 Integer BASIC (Wozniak, 1976).
//!
//! Turns a listing (one `NUMBER STATEMENT[:STATEMENT…]` per line) into the
//! program exactly as the interpreter stores it when the lines are typed:
//! lines sorted by number, stored from `PP` up to `HIMEM - 1`, each one
//! `length, number (lo, hi), tokens…, $01`. Keywords and punctuation are
//! tokens `$00…$7F` whose value depends on the context (the interpreter is
//! syntax directed: `=` is `$71` in an assignment, `$16` in a comparison,
//! `$39` between strings, `$56` in `FOR`…); names, string and REM text are
//! ASCII with bit 7 set; a constant is `$B0 + first digit` then its value
//! (lo, hi). The token values and contexts were measured on the real
//! interpreter (typed listings dumped from memory), not copied from it.
//!
//! Also builds the zero page block `$004A-$00FF` (LOMEM, HIMEM, PP, PV),
//! so that the pair can be put in memory or on an ACI tape and started
//! with `E2B3R` (warm start) then `RUN`.

use std::collections::BTreeMap;
use std::fmt;

/// First and last byte of the zero page block saved with a program.
pub const ZP_START: u16 = 0x004A;
pub const ZP_END: u16 = 0x00FF;
/// Default LOMEM of the Apple-1 BASIC after `E000R`.
pub const DEFAULT_LOMEM: u16 = 0x0800;

/// Keywords, to colour a listing (statements, functions, operators).
pub const KEYWORDS: [&str; 27] = [
    "REM", "LET", "DIM", "TAB", "END", "INPUT", "FOR", "TO", "STEP", "NEXT", "RETURN", "GOSUB", "GOTO", "IF", "THEN", "PRINT", "POKE", "CALL", "PEEK", "RND", "SGN", "ABS",
    "LEN", "NOT", "AND", "OR", "MOD",
];

/// Commands of the immediate mode, refused in a program line.
const IMMEDIATE: [&str; 11] = ["LIST", "RUN", "CLR", "NEW", "AUTO", "MAN", "DEL", "HIMEM", "LOMEM", "SCR", "CON"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrKind {
    /// Missing line number, or above 32767.
    LineNumber,
    /// Constant above 32767 (the interpreter says `>32767 ERR`).
    NumberTooBig,
    /// Anything the interpreter refuses with `SYNTAX ERR`.
    Syntax,
    /// Character the Apple-1 cannot store (lower case is converted).
    Charset,
    /// String without its closing quote.
    String,
    /// Immediate mode command (LIST, RUN…) in a program line.
    Command,
    /// Name other than a letter and an optional digit (`A`, `A1`, `A$`).
    VarName,
    /// More than 255 bytes once tokenized.
    TooLong,
    /// Program larger than the space between LOMEM and HIMEM.
    MemFull,
}

impl ErrKind {
    /// Short identifier (used as a translation key by the GUI).
    pub fn key(self) -> &'static str {
        match self {
            ErrKind::LineNumber => "line_number",
            ErrKind::NumberTooBig => "number_too_big",
            ErrKind::Syntax => "syntax",
            ErrKind::Charset => "charset",
            ErrKind::String => "string",
            ErrKind::Command => "command",
            ErrKind::VarName => "var_name",
            ErrKind::TooLong => "too_long",
            ErrKind::MemFull => "mem_full",
        }
    }
    fn fr(self) -> &'static str {
        match self {
            ErrKind::LineNumber => "numéro de ligne absent ou supérieur à 32767",
            ErrKind::NumberTooBig => "nombre supérieur à 32767",
            ErrKind::Syntax => "erreur de syntaxe",
            ErrKind::Charset => "caractère impossible sur l'Apple-1",
            ErrKind::String => "chaîne sans guillemet fermant",
            ErrKind::Command => "commande du mode direct interdite dans un programme",
            ErrKind::VarName => "nom de variable invalide (une lettre et un chiffre au plus, A$ pour une chaîne)",
            ErrKind::TooLong => "ligne trop longue (plus de 255 octets)",
            ErrKind::MemFull => "programme trop grand pour la mémoire",
        }
    }
}

/// An error, located in the source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokError {
    /// Line of the source text (1 = first).
    pub src_line: usize,
    /// BASIC line number, when it could be read.
    pub number: Option<u16>,
    /// Column (characters, 0 = first) where the problem was found.
    pub col: usize,
    pub kind: ErrKind,
    /// The text from the column (a few characters), to show.
    pub near: String,
}

impl fmt::Display for TokError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.number {
            Some(n) => write!(f, "ligne {} (BASIC {n}), colonne {} : {}", self.src_line, self.col + 1, self.kind.fr())?,
            None => write!(f, "ligne {}, colonne {} : {}", self.src_line, self.col + 1, self.kind.fr())?,
        }
        if !self.near.is_empty() {
            write!(f, " près de « {} »", self.near)?;
        }
        Ok(())
    }
}

/// A tokenized line: its number and its bytes in memory (with the length
/// byte, the number and the final `$01`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub number: u16,
    pub bytes: Vec<u8>,
    pub src_line: usize,
}

/// Result of [`tokenize`]: the lines kept (sorted, a later line with the
/// same number replaces the earlier one, a number alone deletes it, as when
/// typing) and the errors (lines with an error are left out).
#[derive(Debug, Clone, Default)]
pub struct Listing {
    pub lines: Vec<Line>,
    pub errors: Vec<TokError>,
}

impl Listing {
    /// Size of the program block.
    pub fn size(&self) -> usize {
        self.lines.iter().map(|l| l.bytes.len()).sum()
    }
}

/// Tokenize a whole listing. Blank lines are ignored.
pub fn tokenize(src: &str) -> Listing {
    let mut map: BTreeMap<u16, Line> = BTreeMap::new();
    let mut errors = Vec::new();
    for (i, text) in src.lines().enumerate() {
        match tokenize_line(text) {
            Ok(None) => {}
            Ok(Some((n, None))) => {
                map.remove(&n);
            }
            Ok(Some((n, Some(bytes)))) => {
                map.insert(n, Line { number: n, bytes, src_line: i + 1 });
            }
            Err(mut e) => {
                e.src_line = i + 1;
                errors.push(e);
            }
        }
    }
    Listing { lines: map.into_values().collect(), errors }
}

/// Tokenize one line. `Ok(None)` for a blank line, `Ok(Some((n, None)))`
/// for a number alone (deletes line n).
#[allow(clippy::type_complexity)]
pub fn tokenize_line(text: &str) -> Result<Option<(u16, Option<Vec<u8>>)>, TokError> {
    let chars: Vec<char> = text.chars().map(|c| if c == '\t' { ' ' } else { c.to_ascii_uppercase() }).collect();
    let mut p = P { s: chars, i: 0, out: Vec::new(), number: None };
    if p.peek().is_none() {
        return Ok(None);
    }
    if let Some(c) = p.s.iter().position(|c| !(' '..='~').contains(c) || *c == '`' || *c == '{' || *c == '|' || *c == '}' || *c == '~') {
        // bit 7 set, the Apple-1 keeps 6 bit codes $20-$5F (no lower case after conversion)
        return Err(p.error_at(c, ErrKind::Charset));
    }
    if !p.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        return Err(p.error(ErrKind::LineNumber));
    }
    let n = p.number_value().map_err(|mut e| {
        e.kind = ErrKind::LineNumber;
        e
    })?;
    p.number = Some(n);
    if p.peek().is_none() {
        return Ok(Some((n, None)));
    }
    p.statement()?;
    while p.peek() == Some(':') {
        p.i_skip();
        p.out.push(0x03);
        p.statement()?;
    }
    if p.peek().is_some() {
        return Err(p.error(ErrKind::Syntax));
    }
    p.out.push(0x01);
    let len = p.out.len() + 3;
    if len > 255 {
        return Err(p.error_at(0, ErrKind::TooLong));
    }
    let mut bytes = vec![len as u8, n as u8, (n >> 8) as u8];
    bytes.extend_from_slice(&p.out);
    Ok(Some((n, Some(bytes))))
}

type R<T> = Result<T, TokError>;

struct P {
    s: Vec<char>,
    i: usize,
    out: Vec<u8>,
    number: Option<u16>,
}

impl P {
    fn skip(&mut self) {
        while self.s.get(self.i) == Some(&' ') {
            self.i += 1;
        }
    }
    fn peek(&mut self) -> Option<char> {
        self.skip();
        self.s.get(self.i).copied()
    }
    /// Consume the character seen by peek.
    fn i_skip(&mut self) {
        self.skip();
        self.i += 1;
    }
    fn at_end(&mut self) -> bool {
        matches!(self.peek(), None | Some(':'))
    }
    /// Match a keyword (spaces between its letters are ignored, as by the
    /// interpreter); nothing is consumed when it does not match.
    fn kw(&mut self, w: &str) -> bool {
        let save = self.i;
        for c in w.chars() {
            self.skip();
            if self.s.get(self.i) != Some(&c) {
                self.i = save;
                return false;
            }
            self.i += 1;
        }
        true
    }
    fn eat(&mut self, c: char, tok: u8) -> R<()> {
        if self.peek() == Some(c) {
            self.i += 1;
            self.out.push(tok);
            Ok(())
        } else {
            Err(self.error(ErrKind::Syntax))
        }
    }
    fn error_at(&self, col: usize, kind: ErrKind) -> TokError {
        let near: String = self.s.iter().skip(col).take(12).collect();
        TokError { src_line: 0, number: self.number, col, kind, near: near.trim_end().to_string() }
    }
    fn error(&mut self, kind: ErrKind) -> TokError {
        self.skip();
        self.error_at(self.i, kind)
    }

    /// Digits of a constant (spaces between them are ignored).
    fn number_value(&mut self) -> R<u16> {
        self.skip();
        let start = self.i;
        let mut v: u32 = 0;
        while let Some(c) = self.peek().filter(|c| c.is_ascii_digit()) {
            self.i += 1;
            v = v * 10 + c.to_digit(10).unwrap();
            if v > 32767 {
                return Err(self.error_at(start, ErrKind::NumberTooBig));
            }
        }
        Ok(v as u16)
    }
    fn number(&mut self) -> R<()> {
        let first = self.peek().unwrap_or('0');
        let v = self.number_value()?;
        self.out.extend_from_slice(&[0xB0 + (first as u8 - b'0'), v as u8, (v >> 8) as u8]);
        Ok(())
    }
    fn hi(c: char) -> u8 {
        c as u8 | 0x80
    }

    /// A name: a letter, then a digit (number) or `$` (string). Returns
    /// whether it is a string; the `$` token is emitted.
    fn name(&mut self) -> R<bool> {
        let start = {
            self.skip();
            self.i
        };
        match self.peek() {
            Some(c) if c.is_ascii_uppercase() => {
                self.i += 1;
                self.out.push(Self::hi(c));
            }
            _ => return Err(self.error(ErrKind::Syntax)),
        }
        if let Some(d) = self.peek().filter(|c| c.is_ascii_digit()) {
            self.i += 1;
            self.out.push(Self::hi(d));
            if matches!(self.peek(), Some(c) if c.is_ascii_digit() || c == '$') {
                return Err(self.error_at(start, ErrKind::VarName));
            }
            return self.name_end(start).map(|_| false);
        }
        if self.peek() == Some('$') {
            self.i_skip();
            self.out.push(0x40);
            return Ok(true);
        }
        self.name_end(start).map(|_| false)
    }

    /// After a name, a letter can only start a keyword (THEN, TO…): the
    /// interpreter refuses longer names.
    fn name_end(&mut self, start: usize) -> R<()> {
        if self.peek().map(|c| c.is_ascii_uppercase()).unwrap_or(false) {
            let rest: String = self.s[self.i..].iter().filter(|c| **c != ' ').collect();
            if !["THEN", "TO", "STEP", "AND", "OR", "MOD"].iter().any(|k| rest.starts_with(k)) {
                return Err(self.error_at(start, ErrKind::VarName));
            }
        }
        Ok(())
    }

    fn statement(&mut self) -> R<()> {
        if self.at_end() {
            return Ok(()); // empty statement (after ':')
        }
        if self.kw("REM") {
            self.out.push(0x5D);
            let rest: Vec<u8> = self.s[self.i..].iter().map(|c| Self::hi(*c)).collect();
            self.out.extend(rest);
            self.i = self.s.len();
            return Ok(());
        }
        if self.kw("LET") {
            self.out.push(0x5E);
            return self.assignment();
        }
        if self.kw("DIM") {
            let at = self.out.len();
            self.out.push(0);
            let s = self.dim_item()?;
            self.out[at] = if s { 0x4E } else { 0x4F };
            while self.peek() == Some(',') {
                self.i_skip();
                let at = self.out.len();
                self.out.push(0);
                let s = self.dim_item()?;
                self.out[at] = if s { 0x43 } else { 0x44 };
            }
            return Ok(());
        }
        if self.kw("TAB") {
            self.out.push(0x50);
            return self.expr();
        }
        if self.kw("END") {
            self.out.push(0x51);
            return Ok(());
        }
        if self.kw("INPUT") {
            return self.input();
        }
        if self.kw("FOR") {
            self.out.push(0x55);
            if self.name()? || self.peek() == Some('(') {
                return Err(self.error(ErrKind::Syntax));
            }
            self.eat('=', 0x56)?;
            self.expr()?;
            if !self.kw("TO") {
                return Err(self.error(ErrKind::Syntax));
            }
            self.out.push(0x57);
            self.expr()?;
            if self.kw("STEP") {
                self.out.push(0x58);
                self.expr()?;
            }
            return Ok(());
        }
        if self.kw("NEXT") {
            self.out.push(0x59);
            self.numeric_name()?;
            while self.peek() == Some(',') {
                self.i_skip();
                self.out.push(0x5A);
                self.numeric_name()?;
            }
            return Ok(());
        }
        if self.kw("RETURN") {
            self.out.push(0x5B);
            return Ok(());
        }
        if self.kw("GOSUB") {
            self.out.push(0x5C);
            return self.expr();
        }
        if self.kw("GOTO") {
            self.out.push(0x5F);
            return self.expr();
        }
        if self.kw("IF") {
            self.out.push(0x60);
            self.expr()?;
            if !self.kw("THEN") {
                return Err(self.error(ErrKind::Syntax));
            }
            if self.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                self.out.push(0x24);
                return self.number();
            }
            self.out.push(0x25);
            return self.statement();
        }
        if self.kw("PRINT") {
            return self.print();
        }
        if self.kw("POKE") {
            self.out.push(0x64);
            self.expr()?;
            self.eat(',', 0x65)?;
            return self.expr();
        }
        if self.kw("CALL") {
            self.out.push(0x4D);
            return self.expr();
        }
        for c in IMMEDIATE {
            let save = self.i;
            if self.kw(c) {
                self.i = save;
                return Err(self.error(ErrKind::Command));
            }
        }
        self.assignment()
    }

    fn numeric_name(&mut self) -> R<()> {
        if self.name()? {
            return Err(self.error(ErrKind::Syntax));
        }
        Ok(())
    }

    fn assignment(&mut self) -> R<()> {
        if self.name()? {
            if self.peek() == Some('(') {
                self.i_skip();
                self.out.push(0x42);
                self.expr()?;
                self.eat(')', 0x72)?;
            }
            self.eat('=', 0x70)?;
            self.string_expr()
        } else {
            self.subscript()?;
            self.eat('=', 0x71)?;
            self.expr()
        }
    }

    /// Optional `(expr)` after a numeric name.
    fn subscript(&mut self) -> R<()> {
        if self.peek() == Some('(') {
            self.i_skip();
            self.out.push(0x2D);
            self.expr()?;
            self.eat(')', 0x72)?;
        }
        Ok(())
    }

    fn dim_item(&mut self) -> R<bool> {
        let s = self.name()?;
        self.eat('(', if s { 0x22 } else { 0x34 })?;
        self.expr()?;
        self.eat(')', 0x72)?;
        Ok(s)
    }

    fn input_var(&mut self) -> R<bool> {
        let s = self.name()?;
        if self.peek() == Some('(') {
            self.i_skip();
            self.out.push(if s { 0x42 } else { 0x2D });
            self.expr()?;
            self.eat(')', 0x72)?;
        }
        Ok(s)
    }

    fn input(&mut self) -> R<()> {
        let at = self.out.len();
        self.out.push(0);
        if self.peek() == Some('"') {
            self.out[at] = 0x53;
            self.literal()?;
            if self.peek() != Some(',') {
                return Err(self.error(ErrKind::Syntax));
            }
        } else {
            let s = self.input_var()?;
            self.out[at] = if s { 0x52 } else { 0x54 };
        }
        while self.peek() == Some(',') {
            self.i_skip();
            let at = self.out.len();
            self.out.push(0);
            let s = self.input_var()?;
            self.out[at] = if s { 0x26 } else { 0x27 };
        }
        Ok(())
    }

    fn print(&mut self) -> R<()> {
        let at = self.out.len();
        self.out.push(0x63);
        if self.at_end() {
            return Ok(());
        }
        let s = self.any_expr()?;
        self.out[at] = if s { 0x61 } else { 0x62 };
        while let Some(sep @ (';' | ',')) = self.peek() {
            self.i_skip();
            let at = self.out.len();
            self.out.push(0);
            if self.at_end() {
                if sep == ',' {
                    return Err(self.error(ErrKind::Syntax));
                }
                self.out[at] = 0x47;
                break;
            }
            let s = self.any_expr()?;
            self.out[at] = match (sep, s) {
                (';', true) => 0x45,
                (';', false) => 0x46,
                (_, true) => 0x48,
                _ => 0x49,
            };
        }
        Ok(())
    }

    fn literal(&mut self) -> R<()> {
        self.skip();
        if self.s.get(self.i) != Some(&'"') {
            return Err(self.error(ErrKind::Syntax));
        }
        let start = self.i;
        self.i += 1;
        self.out.push(0x28);
        loop {
            match self.s.get(self.i) {
                None => return Err(self.error_at(start, ErrKind::String)),
                Some('"') => break,
                Some(c) => {
                    self.out.push(Self::hi(*c));
                    self.i += 1;
                }
            }
        }
        self.i += 1;
        self.out.push(0x29);
        Ok(())
    }

    /// The rest of a string operand once its name (and `$`) was read.
    fn string_tail(&mut self) -> R<()> {
        if self.peek() == Some('(') {
            self.i_skip();
            self.out.push(0x2A);
            self.expr()?;
            if self.peek() == Some(',') {
                self.i_skip();
                self.out.push(0x23);
                self.expr()?;
            }
            self.eat(')', 0x72)?;
        }
        Ok(())
    }

    fn string_expr(&mut self) -> R<()> {
        if self.peek() == Some('"') {
            return self.literal();
        }
        if !self.name()? {
            return Err(self.error(ErrKind::Syntax));
        }
        self.string_tail()
    }

    /// Numeric expression.
    fn expr(&mut self) -> R<()> {
        if self.any_expr()? {
            return Err(self.error(ErrKind::Syntax));
        }
        Ok(())
    }

    /// Expression of either kind; returns true for a lone string.
    fn any_expr(&mut self) -> R<bool> {
        let s = self.operand(true)?;
        if s {
            return Ok(true);
        }
        while let Some(tok) = self.binop() {
            self.out.push(tok);
            if self.operand(false)? {
                return Err(self.error(ErrKind::Syntax));
            }
        }
        Ok(false)
    }

    fn binop(&mut self) -> Option<u8> {
        const OPS: [(&str, u8); 15] = [
            ("<=", 0x1A),
            (">=", 0x18),
            ("<>", 0x1B),
            ("<", 0x1C),
            (">", 0x19),
            ("=", 0x16),
            ("#", 0x17),
            ("+", 0x12),
            ("-", 0x13),
            ("*", 0x14),
            ("/", 0x15),
            ("^", 0x20),
            ("AND", 0x1D),
            ("OR", 0x1E),
            ("MOD", 0x1F),
        ];
        OPS.iter().find(|(w, _)| self.kw(w)).map(|o| o.1)
    }

    /// An operand with its optional sign or NOT. With `string_ok`, a lone
    /// string is accepted (returns true).
    fn operand(&mut self, string_ok: bool) -> R<bool> {
        let unary = match self.peek() {
            Some('-') => Some(0x36),
            Some('+') => Some(0x35),
            _ if self.kw("NOT") => {
                self.out.push(0x37);
                return self.primary(false).map(|_| false);
            }
            _ => None,
        };
        if let Some(u) = unary {
            self.i_skip();
            self.out.push(u);
            self.primary(false)?;
            return Ok(false);
        }
        self.primary(string_ok)
    }

    fn primary(&mut self, string_ok: bool) -> R<bool> {
        let c = match self.peek() {
            Some(c) => c,
            None => return Err(self.error(ErrKind::Syntax)),
        };
        if c.is_ascii_digit() {
            self.number()?;
            return Ok(false);
        }
        if c == '(' {
            self.i_skip();
            self.out.push(0x38);
            self.expr()?;
            self.eat(')', 0x72)?;
            return Ok(false);
        }
        if c == '"' {
            self.literal()?;
            return self.after_string(string_ok);
        }
        for (w, tok) in [("PEEK", 0x2E), ("RND", 0x2F), ("SGN", 0x30), ("ABS", 0x31)] {
            if self.kw(w) {
                self.out.push(tok);
                self.eat('(', 0x3F)?;
                self.expr()?;
                self.eat(')', 0x72)?;
                return Ok(false);
            }
        }
        if self.kw("LEN") {
            if self.peek() != Some('(') {
                return Err(self.error(ErrKind::Syntax));
            }
            self.i_skip();
            self.out.push(0x3B);
            self.string_expr()?;
            self.eat(')', 0x72)?;
            return Ok(false);
        }
        if ["-", "+", "NOT"].iter().any(|w| self.peek_kw(w)) {
            return Err(self.error(ErrKind::Syntax));
        }
        if self.name()? {
            self.string_tail()?;
            return self.after_string(string_ok);
        }
        self.subscript()?;
        Ok(false)
    }

    fn peek_kw(&mut self, w: &str) -> bool {
        let save = self.i;
        let r = self.kw(w);
        self.i = save;
        r
    }

    /// After a string operand: a comparison (`=` or `#` another string)
    /// makes a number; otherwise it must stand alone.
    fn after_string(&mut self, string_ok: bool) -> R<bool> {
        let tok = match self.peek() {
            Some('=') => 0x39,
            Some('#') => 0x3A,
            _ if string_ok => return Ok(true),
            _ => return Err(self.error(ErrKind::Syntax)),
        };
        if self.peek_kw("<>") {
            return Err(self.error(ErrKind::Syntax));
        }
        self.i_skip();
        self.out.push(tok);
        self.string_expr()?;
        Ok(false)
    }
}

/// A program ready to go in memory or on tape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub lomem: u16,
    pub himem: u16,
    /// Start of the program (PP); the program ends at HIMEM - 1.
    pub pp: u16,
    /// End of the variables (PV = LOMEM: no variables yet).
    pub pv: u16,
    /// Bytes $004A-$00FF.
    pub zp: Vec<u8>,
    /// Bytes PP..HIMEM-1.
    pub prog: Vec<u8>,
}

impl Image {
    /// Lay out the lines below `himem`; the zero page holds LOMEM, HIMEM,
    /// PP and PV (the other bytes are scratch space of the interpreter,
    /// left at zero).
    pub fn build(lines: &[Line], lomem: u16, himem: u16) -> Result<Image, ErrKind> {
        let prog: Vec<u8> = lines.iter().flat_map(|l| l.bytes.iter().copied()).collect();
        let pp = himem as usize as isize - prog.len() as isize;
        if pp < lomem as isize || himem <= lomem {
            return Err(ErrKind::MemFull);
        }
        let pp = pp as u16;
        let mut zp = vec![0u8; (ZP_END - ZP_START + 1) as usize];
        let mut put = |a: u16, v: u16| {
            zp[(a - ZP_START) as usize] = v as u8;
            zp[(a - ZP_START) as usize + 1] = (v >> 8) as u8;
        };
        put(0x4A, lomem);
        put(0x4C, himem);
        put(0xCA, pp);
        put(0xCC, lomem);
        Ok(Image { lomem, himem, pp, pv: lomem, zp, prog })
    }
    /// The two blocks (zero page, program).
    pub fn blocks(&self) -> Vec<(u16, Vec<u8>)> {
        vec![(ZP_START, self.zp.clone()), (self.pp, self.prog.clone())]
    }
    /// Bytes left for the variables (between LOMEM and PP).
    pub fn free(&self) -> usize {
        (self.pp - self.lomem) as usize
    }
    /// What to type after `C100R` to read the tape (4 hex digits on both
    /// sides of each range: the ACI ROM does not clear its accumulator).
    pub fn aci_command(&self) -> String {
        format!("{}R {}R", crate::tape::aci_range_text(ZP_START, ZP_END), crate::tape::aci_range_text(self.pp, self.himem - 1))
    }
    /// Size of the Apple-1 it is meant for (8 = the standard 4K + 4K at
    /// $E000, HIMEM $1000; 12, 16, 32 = contiguous RAM from $0000).
    pub fn ram_kb(&self) -> u32 {
        MEMORY.iter().find(|m| m.1 == self.himem).map(|m| m.0).unwrap_or_else(|| (self.himem as u32).div_ceil(4096) * 4)
    }
}

/// Memory sizes offered (KB of RAM from $0000) and their HIMEM.
pub const MEMORY: [(u32, u16); 4] = [(8, 0x1000), (12, 0x3000), (16, 0x4000), (32, 0x8000)];

/// Margin kept for the variables when choosing the memory automatically.
pub const AUTO_MARGIN: usize = 512;

/// Choose LOMEM and HIMEM: `ram_kb` = None for automatic (8K with LOMEM
/// $0800 when it fits, then 8K with LOMEM $0300, then 12K, 16K, 32K),
/// else that size with LOMEM $0800 (or $0300 when needed in 8K).
pub fn layout(prog_len: usize, ram_kb: Option<u32>) -> (u16, u16) {
    let fits = |lo: u16, hi: u16| prog_len + AUTO_MARGIN <= (hi - lo) as usize;
    match ram_kb {
        Some(kb) => {
            let hi = MEMORY.iter().find(|m| m.0 == kb).map(|m| m.1).unwrap_or(0x1000);
            let lo = if hi == 0x1000 && !fits(DEFAULT_LOMEM, hi) { 0x0300 } else { DEFAULT_LOMEM };
            (lo, hi)
        }
        None => {
            if fits(DEFAULT_LOMEM, 0x1000) {
                return (DEFAULT_LOMEM, 0x1000);
            }
            if fits(0x0300, 0x1000) {
                return (0x0300, 0x1000);
            }
            for (_, hi) in &MEMORY[1..] {
                if fits(DEFAULT_LOMEM, *hi) {
                    return (DEFAULT_LOMEM, *hi);
                }
            }
            (DEFAULT_LOMEM, 0x8000)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(s: &str) -> Vec<u8> {
        s.split_whitespace().map(|b| u8::from_str_radix(b, 16).unwrap()).collect()
    }
    fn tok(line: &str) -> Vec<u8> {
        tokenize_line(line).unwrap().unwrap().1.unwrap()
    }
    fn err(line: &str) -> ErrKind {
        tokenize_line(line).unwrap_err().kind
    }

    // Expected bytes: dumped from the real interpreter after typing the line.
    #[test]
    fn statements_like_the_interpreter() {
        assert_eq!(tok("10 INPUT A(1)"), hex("0b 0a 00 54 c1 2d b1 01 00 72 01"));
        assert_eq!(tok("30 PRINT A$(1)"), hex("0c 1e 00 61 c1 40 2a b1 01 00 72 01"));
        assert_eq!(tok("150 A=+B"), hex("08 96 00 c1 71 35 c2 01"));
        assert_eq!(tok("160 B=LEN(\"ABC\")"), hex("0d a0 00 c2 71 3b 28 c1 c2 c3 29 72 01"));
        assert_eq!(tok("40 IF A$=\"X\" AND B=1 THEN 10"), hex("15 28 00 60 c1 40 39 28 d8 29 1d c2 16 b1 01 00 24 b1 0a 00 01"));
        assert_eq!(tok("120 RETURN"), hex("05 78 00 5b 01"));
    }

    #[test]
    fn context_tokens() {
        let body = |l: &str| {
            let b = tok(l);
            b[3..b.len() - 1].to_vec()
        };
        // DIM: 4E/4F by the first item, 43/44 before a string/number
        assert_eq!(body("30 DIM A$(10),D$(1),A(3)"), hex("4e c1 40 22 b1 0a 00 72 43 c4 40 22 b1 01 00 72 44 c1 34 b3 03 00 72"));
        // PRINT 61/62/63, separators by what follows, 47 at the end
        assert_eq!(body("400 PRINT 5,\"A\";"), hex("62 b5 05 00 48 28 c1 29 47"));
        assert_eq!(body("130 PRINT A;B,C"), hex("62 c1 46 c2 49 c3"));
        assert_eq!(body("150 PRINT"), hex("63"));
        // INPUT 52/53/54, commas 26/27
        assert_eq!(body("1230 INPUT \"A\",A$,B"), hex("53 28 c1 29 26 c1 40 27 c2"));
        assert_eq!(body("310 INPUT A$,B"), hex("52 c1 40 27 c2"));
        // FOR / NEXT
        assert_eq!(body("70 FOR I=1 TO 10 STEP 2"), hex("55 c9 56 b1 01 00 57 b1 0a 00 58 b2 02 00"));
        assert_eq!(body("90 NEXT I,J"), hex("59 c9 5a ca"));
        assert_eq!(body("90 FOR A1=B2 TO C3"), hex("55 c1 b1 56 c2 b2 57 c3 b3"));
        // IF … THEN statement / line
        assert_eq!(body("110 IF A THEN GOTO 10"), hex("60 c1 25 5f b1 0a 00"));
        // operators, unary minus, functions
        assert_eq!(body("20 A=B+C-D*E/F^2 MOD 3"), hex("c1 71 c2 12 c3 13 c4 14 c5 15 c6 20 b2 02 00 1f b3 03 00"));
        assert_eq!(body("50 IF A<>B AND C<D OR E>F THEN A=1"), hex("60 c1 1b c2 1d c3 1c c4 1e c5 19 c6 25 c1 71 b1 01 00"));
        assert_eq!(body("220 CALL -151"), hex("4d 36 b1 97 00"));
        assert_eq!(body("240 A=ABS(B)+SGN(C)+RND(10)"), hex("c1 71 31 3f c2 72 12 30 3f c3 72 12 2f 3f b1 0a 00 72"));
        assert_eq!(body("210 POKE 768,PEEK(769)"), hex("64 b7 00 03 65 2e 3f b7 01 03 72"));
        // strings: store, substring, comparison
        assert_eq!(body("280 A$(2)=\"X\""), hex("c1 40 42 b2 02 00 72 70 28 d8 29"));
        assert_eq!(body("290 B$=A$(1,2)"), hex("c2 40 70 c1 40 2a b1 01 00 23 b2 02 00 72"));
        assert_eq!(body("310 IF A$#\"X\" THEN 10"), hex("60 c1 40 3a 28 d8 29 24 b1 0a 00"));
        // REM keeps the text as typed, spaces in keywords are ignored
        assert_eq!(body("350 REM HELLO \"A\""), hex("5d a0 c8 c5 cc cc cf a0 a2 c1 a2"));
        assert_eq!(body("270 G O T O 10"), hex("5f b1 0a 00"));
        // constants: $B0 + first digit, then the value
        assert_eq!(body("60 X=007"), hex("d8 71 b0 07 00"));
        assert_eq!(body("60 PRINT \"A\":"), hex("61 28 c1 29 03"));
    }

    #[test]
    fn refused_like_the_interpreter() {
        assert_eq!(err("10 SCORE=1"), ErrKind::VarName);
        assert_eq!(err("10 A12=1"), ErrKind::VarName);
        assert_eq!(err("10 A0$=\"X\""), ErrKind::VarName);
        assert_eq!(err("10 PRINT A,"), ErrKind::Syntax);
        assert_eq!(err("10 A=32768"), ErrKind::NumberTooBig);
        assert_eq!(err("10 LIST"), ErrKind::Command);
        assert_eq!(err("10 IF A$<>B$ THEN 10"), ErrKind::Syntax);
        assert_eq!(err("10 A=--B"), ErrKind::Syntax);
        assert_eq!(err("10 INPUT \"A\";A"), ErrKind::Syntax);
        assert_eq!(err("10 NEXT"), ErrKind::Syntax);
        assert_eq!(err("10 ENDX=1"), ErrKind::Syntax);
        assert_eq!(err("10 PRINT \"A"), ErrKind::String);
        assert_eq!(err("PRINT 1"), ErrKind::LineNumber);
        assert_eq!(err("40000 END"), ErrKind::LineNumber);
        assert_eq!(err("10 IF A THEN 10+B"), ErrKind::Syntax);
        assert_eq!(err("10 PRINT \"é\""), ErrKind::Charset);
    }

    #[test]
    fn listing_and_image() {
        let l = tokenize("20 END\n10 PRINT\n\n30 GOTO 10\n30\n15 FOO\n");
        assert_eq!(l.lines.iter().map(|l| l.number).collect::<Vec<_>>(), vec![10, 20]);
        assert_eq!(l.errors.len(), 1);
        assert_eq!(l.errors[0].src_line, 6);
        assert_eq!(l.errors[0].number, Some(15));
        let img = Image::build(&l.lines, 0x0800, 0x1000).unwrap();
        assert_eq!(img.pp, 0x1000 - 10);
        assert_eq!(img.prog, hex("05 0a 00 63 01 05 14 00 51 01"));
        assert_eq!(&img.zp[0..4], &[0x00, 0x08, 0x00, 0x10]);
        assert_eq!(&img.zp[0x80..0x84], &[0xF6, 0x0F, 0x00, 0x08]);
        assert_eq!(img.aci_command(), "004A.00FFR 0FF6.0FFFR");
        assert_eq!(img.zp.len(), 0xB6);
        assert_eq!(layout(100, None), (0x0800, 0x1000));
        assert_eq!(layout(2000, None), (0x0300, 0x1000));
        assert_eq!(layout(6000, None), (0x0800, 0x3000));
        assert!(Image::build(&l.lines, 0x0FFA, 0x1000).is_err());
    }
}
