//! Debugger: 6502 stepping and breakpoints, a disassembler, and helpers for
//! Integer BASIC (current line, variable table).
//!
//! Integer BASIC (the real one, loaded at `$E000`) was observed in the test
//! harness: right after the instruction at `$E86D` (`STY $DD`, following
//! `STA $DC` at `$E86B`) the word at `$DC` points to the program line about
//! to run (length byte, then the line number as a 16 bit word). The variable
//! table runs from LOMEM (`$4A`) to PV (`$CC`); each entry is two name bytes
//! (the letter as `(n * 2) | $80` with A = 1, then `$00`, a digit `$B0..$B9`,
//! or `$40` for `$`), the address of the next entry, then the data: 16 bit
//! integers (a scalar is one, `DIM X(n)` makes n of them, X(1) first) or the
//! characters of a string (high bit set) ended by `$1E`.

use std::collections::{BTreeMap, BTreeSet};

use crate::asm6502::{decode, mode_len, Mode};
use crate::machine::Machine;

/// One disassembled instruction.
#[derive(Debug, Clone, PartialEq)]
pub struct DisLine {
    pub addr: u16,
    pub bytes: Vec<u8>,
    /// `LDA $0200,X` or `.byte $FF` for an unknown opcode.
    pub text: String,
}

/// Symbols for the disassembler: value to name.
pub type Symbols = BTreeMap<u16, String>;

/// Build the address to name map from an assembly's symbol table (the
/// first name wins when several share a value).
pub fn symbols_from(list: &[(String, i64)]) -> Symbols {
    let mut s = Symbols::new();
    for (n, v) in list {
        if (0..=0xFFFF).contains(v) {
            s.entry(*v as u16).or_insert_with(|| n.clone());
        }
    }
    s
}

fn name(v: u16, zp: bool, syms: &Symbols) -> String {
    match syms.get(&v) {
        Some(n) => n.clone(),
        None if zp => format!("${v:02X}"),
        None => format!("${v:04X}"),
    }
}

/// Disassemble the instruction at `pc`.
pub fn disasm(peek: &dyn Fn(u16) -> u8, pc: u16, syms: &Symbols) -> DisLine {
    let op = peek(pc);
    let Some((mn, m)) = decode(op) else {
        return DisLine { addr: pc, bytes: vec![op], text: format!(".byte ${op:02X}") };
    };
    let n = mode_len(m);
    let bytes: Vec<u8> = (0..n as u16).map(|i| peek(pc.wrapping_add(i))).collect();
    let b1 = *bytes.get(1).unwrap_or(&0);
    let w = b1 as u16 | (*bytes.get(2).unwrap_or(&0) as u16) << 8;
    use Mode::*;
    let arg = match m {
        Imp => String::new(),
        Acc => "A".into(),
        Imm => format!("#${b1:02X}"),
        Zp => name(b1 as u16, true, syms),
        ZpX => format!("{},X", name(b1 as u16, true, syms)),
        ZpY => format!("{},Y", name(b1 as u16, true, syms)),
        Abs => name(w, false, syms),
        AbsX => format!("{},X", name(w, false, syms)),
        AbsY => format!("{},Y", name(w, false, syms)),
        Ind => format!("({})", name(w, false, syms)),
        IndX => format!("({},X)", name(b1 as u16, true, syms)),
        IndY => format!("({}),Y", name(b1 as u16, true, syms)),
        Rel => {
            let t = pc.wrapping_add(2).wrapping_add(b1 as i8 as u16);
            name(t, false, syms)
        }
        ZpInd => format!("({})", name(b1 as u16, true, syms)),
        AbsIndX => format!("({},X)", name(w, false, syms)),
        ZpRel => {
            let t = pc.wrapping_add(3).wrapping_add(*bytes.get(2).unwrap_or(&0) as i8 as u16);
            format!("{},{}", name(b1 as u16, true, syms), name(t, false, syms))
        }
    };
    let text = if arg.is_empty() { mn.to_string() } else { format!("{mn} {arg}") };
    DisLine { addr: pc, bytes, text }
}

/// About `before` instructions before `pc` (found by trying start points that
/// decode into `pc` exactly) and `after` from `pc` on.
pub fn disasm_window(peek: &dyn Fn(u16) -> u8, pc: u16, before: usize, after: usize, syms: &Symbols) -> Vec<DisLine> {
    let mut lines = Vec::new();
    // walk back: the earliest start whose decoding lands on pc
    let mut best: Vec<DisLine> = Vec::new();
    for back in (1..=(before as u16 * 3)).rev() {
        let mut a = pc.wrapping_sub(back);
        let mut v = Vec::new();
        while a != pc && v.len() <= before * 3 {
            let l = disasm(peek, a, syms);
            let next = a.wrapping_add(l.bytes.len() as u16);
            // crossed pc without landing on it
            if (pc.wrapping_sub(a) as usize) < l.bytes.len() {
                v.clear();
                break;
            }
            v.push(l);
            a = next;
        }
        if a == pc && !v.is_empty() && v.iter().all(|l| decode(l.bytes[0]).is_some()) {
            best = v;
            break;
        }
    }
    let skip = best.len().saturating_sub(before);
    lines.extend(best.into_iter().skip(skip));
    let mut a = pc;
    for _ in 0..after {
        let l = disasm(peek, a, syms);
        a = a.wrapping_add(l.bytes.len() as u16);
        lines.push(l);
    }
    lines
}

/// Why the machine stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stop {
    /// Pause asked by the user.
    Pause,
    /// One step done.
    Step,
    /// Breakpoint on this address.
    Break(u16),
    /// Integer BASIC is about to run this line (BASIC step).
    BasicLine(u16),
    /// BASIC breakpoint on this line.
    BasicBreak(u16),
    /// The CPU ran a JAM opcode.
    Jam,
}

/// What `run` is doing until it stops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mission {
    /// Run until a breakpoint.
    Run,
    /// One instruction.
    Into,
    /// Until `ret` with the stack back at `sp` (after a JSR).
    Over { ret: u16, sp: u8 },
    /// Until an RTS/RTI leaves the current routine.
    Out { sp: u8 },
    /// Until this address (run to cursor).
    To(u16),
    /// Until the next Integer BASIC line.
    BasicLine,
    /// Until this BASIC line (run to cursor).
    BasicTo(u16),
}

/// Address of the instruction after which `$DC` holds the new BASIC line.
pub const BASIC_LINE_HOOK: u16 = 0xE86D;

/// G2 Basic: the two places where a new BASIC line starts, after which `CUR` ($06) points
/// to it: the `STA CUR+1` ending SETLINE (RUN, GOTO, GOSUB) and the `STA IP+1` of the step
/// to the next line (ENDST). Checked against the image by a test.
pub const G2_LINE_HOOKS: [u16; 2] = [0xEE7C, 0xF1B6];

/// Where the line pointer is set: the hook, and the pointer (G2 Basic: `CUR` at `$06`;
/// Integer BASIC: `$DC`).
fn basic_hook_and_ptr(peek: &dyn Fn(u16) -> u8) -> ([u16; 2], u16) {
    if crate::g2basic::present(peek) {
        (G2_LINE_HOOKS, 0x06)
    } else {
        ([BASIC_LINE_HOOK; 2], 0xDC)
    }
}

/// Is the real Integer BASIC at `$E000` (checked on the hook bytes)?
pub fn basic_present(peek: &dyn Fn(u16) -> u8) -> bool {
    crate::g2basic::present(peek) || [0x85, 0xDC, 0x84, 0xDD].iter().enumerate().all(|(i, b)| peek(0xE86B + i as u16) == *b)
}

fn word(peek: &dyn Fn(u16) -> u8, a: u16) -> u16 {
    peek(a) as u16 | (peek(a.wrapping_add(1)) as u16) << 8
}

/// Line number at the BASIC line pointer `$DC`, if it points into the
/// program (PP `$CA` .. HIMEM `$4C`).
pub fn basic_current_line(peek: &dyn Fn(u16) -> u8) -> Option<u16> {
    let p = word(peek, basic_hook_and_ptr(peek).1);
    let pp = word(peek, 0xCA);
    let himem = word(peek, 0x4C);
    if p >= pp && p < himem {
        Some(word(peek, p.wrapping_add(1)))
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarValue {
    /// Integers: one for a scalar, n for `DIM X(n)` (X(1) first).
    Int(Vec<i16>),
    /// Text and room (the DIM size).
    Str(String, usize),
}

/// One entry of Integer BASIC's variable table.
#[derive(Debug, Clone, PartialEq)]
pub struct BasicVar {
    pub name: String,
    /// Address of the entry (its data starts 4 bytes further).
    pub addr: u16,
    pub value: VarValue,
}

/// G2 Basic's variable table: name (letter token, digit token or 0), kind (0 number,
/// 1 array, 2 string); numbers are 5 bytes with the value at +3, the others carry their
/// size at +3 and their data (2 bytes per element, or a length byte and the characters) at +5.
fn g2_vars(peek: &dyn Fn(u16) -> u8) -> Vec<BasicVar> {
    let (lomem, pv) = (word(peek, 0x4A), word(peek, 0xCC));
    let mut out = Vec::new();
    let mut p = lomem;
    while p < pv && out.len() < 300 {
        let (n1, n2, kind) = (peek(p), peek(p.wrapping_add(1)), peek(p.wrapping_add(2)));
        if !(0xC1..=0xDA).contains(&n1) || kind > 2 {
            break;
        }
        let size = if kind == 0 { 5 } else { word(peek, p.wrapping_add(3)) };
        if size < 5 || p as u32 + size as u32 > pv as u32 {
            break;
        }
        let mut name = ((n1 & 0x7F) as char).to_string();
        if (0xB0..=0xB9).contains(&n2) {
            name.push((n2 & 0x7F) as char);
        }
        let d = p.wrapping_add(5);
        let value = match kind {
            0 => VarValue::Int(vec![word(peek, p.wrapping_add(3)) as i16]),
            1 => VarValue::Int((0..(size - 5) / 2).map(|i| word(peek, d + 2 * i) as i16).collect()),
            _ => {
                name.push('$');
                let len = (peek(d) as usize).min(size as usize - 6);
                let s: String = (0..len).map(|i| (peek(d + 1 + i as u16) & 0x7F) as char).collect();
                VarValue::Str(s, size as usize - 6)
            }
        };
        out.push(BasicVar { name, addr: p, value });
        p += size;
    }
    out
}

/// Decode the variable table (LOMEM `$4A` .. PV `$CC`).
pub fn basic_vars(peek: &dyn Fn(u16) -> u8) -> Vec<BasicVar> {
    if crate::g2basic::present(peek) {
        return g2_vars(peek);
    }
    let lomem = word(peek, 0x4A);
    let pv = word(peek, 0xCC);
    let mut out = Vec::new();
    let mut p = lomem;
    while p < pv && out.len() < 300 {
        let n1 = peek(p);
        let n2 = peek(p.wrapping_add(1));
        let next = word(peek, p.wrapping_add(2));
        if n1 & 0x80 == 0 || next <= p || next > pv {
            break;
        }
        let l = ((n1 & 0x7F) >> 1) as u32;
        if !(1..=26).contains(&l) {
            break;
        }
        let mut name = char::from_u32('A' as u32 + l - 1).unwrap().to_string();
        let string = n2 == 0x40;
        match n2 {
            0x40 => name.push('$'),
            0xB0..=0xB9 => name.push((n2 & 0x7F) as char),
            _ => {}
        }
        let d = p.wrapping_add(4);
        let value = if string {
            let mut s = String::new();
            let mut a = d;
            while a < next && peek(a) != 0x1E {
                s.push((peek(a) & 0x7F) as char);
                a += 1;
            }
            VarValue::Str(s, (next - d).saturating_sub(1) as usize)
        } else {
            VarValue::Int((0..(next - d) / 2).map(|i| word(peek, d + 2 * i) as i16).collect())
        };
        out.push(BasicVar { name, addr: p, value });
        p = next;
    }
    out
}

/// Value of a watch such as `A`, `B1`, `C$`, `D(2)`, or None if unknown.
pub fn basic_watch(vars: &[BasicVar], expr: &str) -> Option<String> {
    let e: String = expr.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_ascii_uppercase();
    let (n, idx) = match e.find('(') {
        Some(i) => {
            let k: usize = e[i + 1..].trim_end_matches(')').parse().ok()?;
            (e[..i].to_string(), Some(k))
        }
        None => (e.clone(), None),
    };
    let v = vars.iter().find(|v| v.name == n)?;
    Some(match (&v.value, idx) {
        (VarValue::Str(s, _), None) => format!("\"{s}\""),
        (VarValue::Int(l), None) if l.len() == 1 => l[0].to_string(),
        (VarValue::Int(l), None) => {
            let s: Vec<String> = l.iter().take(16).map(|x| x.to_string()).collect();
            format!("({}{})", s.join(", "), if l.len() > 16 { ", …" } else { "" })
        }
        (VarValue::Int(l), Some(k)) if k >= 1 && k <= l.len() => l[k - 1].to_string(),
        _ => return None,
    })
}

/// Parse `$0300`, `0300`, `E000` or a symbol name (`label`, `label+2`).
pub fn parse_addr(text: &str, syms: &[(String, i64)]) -> Option<u16> {
    let t = text.trim();
    if t.is_empty() {
        return None;
    }
    let (base, off) = match t.rfind('+') {
        Some(i) if i > 0 => (&t[..i], t[i + 1..].trim().parse::<u16>().ok()?),
        _ => (t, 0),
    };
    let base = base.trim();
    let h = base.strip_prefix('$').unwrap_or(base);
    let v = if let Some((_, v)) = syms.iter().find(|(n, _)| n.eq_ignore_ascii_case(base)) {
        *v as u16
    } else if !h.is_empty() && h.len() <= 4 && h.chars().all(|c| c.is_ascii_hexdigit()) {
        u16::from_str_radix(h, 16).ok()?
    } else {
        return None;
    };
    Some(v.wrapping_add(off))
}

/// Debugger state. Call [`Debugger::run`] instead of `Machine::run_cycles`.
#[derive(Debug, Clone)]
pub struct Debugger {
    pub paused: bool,
    pub breaks: BTreeSet<u16>,
    /// BASIC line numbers.
    pub basic_breaks: BTreeSet<u16>,
    pub mission: Mission,
    pub last_stop: Option<Stop>,
    /// Last BASIC line seen at the hook (None until one is seen).
    pub basic_line: Option<u16>,
    /// Instructions run since creation (for the display).
    pub steps: u64,
}

impl Default for Debugger {
    fn default() -> Self {
        Self { paused: false, breaks: BTreeSet::new(), basic_breaks: BTreeSet::new(), mission: Mission::Run, last_stop: None, basic_line: None, steps: 0 }
    }
}

const JSR: u8 = 0x20;

impl Debugger {
    pub fn pause(&mut self) {
        self.paused = true;
        self.mission = Mission::Run;
        self.last_stop = Some(Stop::Pause);
    }

    /// Continue until a breakpoint.
    pub fn cont(&mut self) {
        self.go(Mission::Run);
    }

    pub fn go(&mut self, m: Mission) {
        self.mission = m;
        self.paused = false;
        self.last_stop = None;
    }

    /// Step over: like a step, but a JSR runs to its return.
    pub fn over(&mut self, m: &Machine) {
        let pc = m.cpu.pc;
        if m.board.peek(pc) == JSR {
            self.go(Mission::Over { ret: pc.wrapping_add(3), sp: m.cpu.sp });
        } else {
            self.go(Mission::Into);
        }
    }

    pub fn out(&mut self, m: &Machine) {
        self.go(Mission::Out { sp: m.cpu.sp });
    }

    fn stop(&mut self, s: Stop) -> Option<Stop> {
        self.paused = true;
        self.mission = Mission::Run;
        self.last_stop = Some(s);
        Some(s)
    }

    /// Run for about `cycles` cycles, or until a stop. Does nothing when
    /// paused. The breakpoint on the first instruction is skipped, so
    /// continuing from a breakpoint moves on.
    pub fn run(&mut self, m: &mut Machine, cycles: u64) -> Option<Stop> {
        if self.paused {
            return None;
        }
        let end = m.board.cycles + cycles;
        let basic = basic_present(&|a| m.board.peek(a));
        let hooks = basic_hook_and_ptr(&|a| m.board.peek(a)).0;
        let mut first = true;
        while m.board.cycles < end {
            let pc = m.cpu.pc;
            if !first {
                if self.breaks.contains(&pc) {
                    return self.stop(Stop::Break(pc));
                }
                match self.mission {
                    Mission::To(a) if a == pc => return self.stop(Stop::Step),
                    Mission::Over { ret, sp } if ret == pc && m.cpu.sp >= sp => return self.stop(Stop::Step),
                    _ => {}
                }
            }
            let op = m.board.peek(pc);
            let sp = m.cpu.sp;
            m.step();
            self.steps += 1;
            first = false;
            if m.cpu.jammed {
                return self.stop(Stop::Jam);
            }
            match self.mission {
                Mission::Into => return self.stop(Stop::Step),
                Mission::Out { sp: s0 } if (op == 0x60 || op == 0x40) && sp >= s0 && m.cpu.sp > s0 => return self.stop(Stop::Step),
                _ => {}
            }
            if basic && hooks.contains(&pc) {
                if let Some(l) = basic_current_line(&|a| m.board.peek(a)) {
                    self.basic_line = Some(l);
                    if self.mission == Mission::BasicLine || self.mission == Mission::BasicTo(l) {
                        return self.stop(Stop::BasicLine(l));
                    }
                    if self.basic_breaks.contains(&l) {
                        return self.stop(Stop::BasicBreak(l));
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asm6502::assemble_text;
    use crate::machine::Config;

    fn machine(src: &str) -> (Machine, crate::asm6502::Assembly) {
        let a = assemble_text(src, None, &[]).expect("asm");
        let mut m = Machine::new(Config::default());
        for (i, b) in a.bytes.iter().enumerate() {
            m.board.poke(a.origin + i as u16, *b);
        }
        m.cpu.pc = a.origin;
        (m, a)
    }

    const PROG: &str = "        .org $0300
start:  ldx #0
loop:   jsr sub
        inx
        cpx #3
        bne loop
done:   jmp done
sub:    lda #$41
        sta $10
        rts
";

    #[test]
    fn disassembles_with_labels() {
        let (m, a) = machine(PROG);
        let s = symbols_from(&a.symbols);
        let peek = |x| m.board.peek(x);
        let l = disasm(&peek, 0x0302, &s);
        assert_eq!(l.text, "JSR sub");
        assert_eq!(l.bytes.len(), 3);
        let w = disasm_window(&peek, 0x0305, 2, 3, &s);
        let t: Vec<&str> = w.iter().map(|l| l.text.as_str()).collect();
        assert_eq!(t, ["LDX #$00", "JSR sub", "INX", "CPX #$03", "BNE loop"]);
        assert_eq!(disasm(&peek, 0x030A, &s).text, "JMP done");
        // every official opcode round trips through the assembler's table
        let mut n = 0;
        for op in 0..=255u8 {
            if decode(op).is_some() {
                n += 1;
            }
        }
        assert_eq!(n, 151);
    }

    #[test]
    fn step_over_out_breakpoints_run_to() {
        let (mut m, a) = machine(PROG);
        let mut d = Debugger::default();
        let sym = |n: &str| parse_addr(n, &a.symbols).unwrap();
        d.go(Mission::Into);
        assert_eq!(d.run(&mut m, 1000), Some(Stop::Step));
        assert_eq!(m.cpu.pc, sym("loop"));
        // over the JSR: lands on inx with A set by sub
        d.over(&m);
        assert_eq!(d.run(&mut m, 1000), Some(Stop::Step));
        assert_eq!(m.cpu.pc, sym("loop") + 3);
        assert_eq!(m.cpu.a, 0x41);
        // into the next JSR, then out
        d.go(Mission::To(sym("loop")));
        d.run(&mut m, 1000);
        assert_eq!(m.cpu.pc, sym("loop"));
        d.over(&m); // not used: check into instead
        d.go(Mission::Into);
        d.run(&mut m, 1000);
        assert_eq!(m.cpu.pc, sym("sub"));
        d.out(&m);
        assert_eq!(d.run(&mut m, 1000), Some(Stop::Step));
        assert_eq!(m.cpu.pc, sym("loop") + 3);
        // breakpoint, continue moves past it
        d.breaks.insert(sym("sub") + 2);
        d.cont();
        assert_eq!(d.run(&mut m, 1000), Some(Stop::Break(sym("sub") + 2)));
        assert!(d.paused);
        assert_eq!(d.run(&mut m, 1000), None, "paused: nothing runs");
        d.cont();
        assert_eq!(d.run(&mut m, 1000), None);
        assert_eq!(m.cpu.pc, sym("done"));
        assert_eq!(m.cpu.x, 3);
        assert_eq!(parse_addr("$03", &[]), Some(3));
        assert_eq!(parse_addr("done+1", &a.symbols), Some(sym("done") + 1));
        assert_eq!(parse_addr("nope", &a.symbols), None);
    }

    #[test]
    fn decodes_basic_variables() {
        // table as written by the real BASIC for
        // 10 A=5:B1=-3:DIM C$(10):C$="HI":DIM D(3):D(2)=7
        let t = [
            0x82, 0x00, 0x06, 0x08, 0x05, 0x00, 0x84, 0xB1, 0x0C, 0x08, 0xFD, 0xFF, 0x86, 0x40, 0x1B, 0x08, 0xC8, 0xC9, 0x1E, 0, 0, 0, 0, 0, 0, 0, 0, 0x88, 0x00, 0x25, 0x08, 0x00,
            0x00, 0x07, 0x00, 0x00, 0x00,
        ];
        let mut mem = vec![0u8; 0x10000];
        mem[0x0800..0x0800 + t.len()].copy_from_slice(&t);
        mem[0x4A] = 0x00;
        mem[0x4B] = 0x08;
        mem[0xCC] = 0x25;
        mem[0xCD] = 0x08;
        let peek = |a: u16| mem[a as usize];
        let v = basic_vars(&peek);
        let names: Vec<&str> = v.iter().map(|v| v.name.as_str()).collect();
        assert_eq!(names, ["A", "B1", "C$", "D"]);
        assert_eq!(basic_watch(&v, "a"), Some("5".into()));
        assert_eq!(basic_watch(&v, "B1"), Some("-3".into()));
        assert_eq!(basic_watch(&v, "C$"), Some("\"HI\"".into()));
        assert_eq!(v[2].value, VarValue::Str("HI".into(), 10));
        assert_eq!(basic_watch(&v, "D(2)"), Some("7".into()));
        assert_eq!(basic_watch(&v, "D"), Some("(0, 7, 0)".into()));
        assert_eq!(basic_watch(&v, "D(4)"), None);
        assert_eq!(basic_watch(&v, "Q"), None);
    }
}
