//! NMOS 6502 CPU core.
//!
//! Every call to [`Bus::read`] / [`Bus::write`] corresponds to exactly one
//! CPU clock cycle, including the "dummy" reads and writes performed by the
//! real NMOS chip (e.g. the double write of read-modify-write instructions,
//! or the read of the wrong page in indexed addressing). This matters on the
//! Apple-1 because merely *touching* some I/O addresses has side effects
//! (reading `KBD` clears the key flag, any access to `$C0xx` toggles the
//! cassette output flip-flop).
//!
//! All 151 documented opcodes are implemented, including the NMOS decimal
//! mode behaviour (N/V/Z flags of ADC in decimal mode are computed the way
//! the NMOS 6502 does) and the `JMP ($xxFF)` page-wrap bug. Undocumented
//! opcodes are executed as NOPs of the proper length (best effort), except the
//! `KIL/JAM` opcodes which halt the CPU.

/// Memory/IO bus seen by the CPU. One call == one clock cycle.
pub trait Bus {
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}

pub const FLAG_C: u8 = 0x01;
pub const FLAG_Z: u8 = 0x02;
pub const FLAG_I: u8 = 0x04;
pub const FLAG_D: u8 = 0x08;
pub const FLAG_B: u8 = 0x10;
pub const FLAG_U: u8 = 0x20;
pub const FLAG_V: u8 = 0x40;
pub const FLAG_N: u8 = 0x80;

#[derive(Debug, Clone)]
pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    /// Status register (bit 5 always reads as 1, B is only meaningful when pushed).
    pub p: u8,
    /// Total number of cycles executed since creation.
    pub cycles: u64,
    /// Set when a JAM/KIL opcode was executed.
    pub jammed: bool,
    pub(crate) nmi_pending: bool,
    pub(crate) irq_line: bool,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Imm,
    Zp,
    Zpx,
    Zpy,
    Abs,
    Abx,
    Aby,
    Izx,
    Izy,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD,
            pc: 0,
            p: FLAG_U | FLAG_I,
            cycles: 0,
            jammed: false,
            nmi_pending: false,
            irq_line: false,
        }
    }

    // ---- bus helpers (count cycles) ----
    #[inline]
    fn rd<B: Bus>(&mut self, bus: &mut B, addr: u16) -> u8 {
        self.cycles += 1;
        bus.read(addr)
    }
    #[inline]
    fn wr<B: Bus>(&mut self, bus: &mut B, addr: u16, v: u8) {
        self.cycles += 1;
        bus.write(addr, v)
    }
    #[inline]
    fn fetch<B: Bus>(&mut self, bus: &mut B) -> u8 {
        let v = self.rd(bus, self.pc);
        self.pc = self.pc.wrapping_add(1);
        v
    }
    #[inline]
    fn push<B: Bus>(&mut self, bus: &mut B, v: u8) {
        self.wr(bus, 0x0100 | self.sp as u16, v);
        self.sp = self.sp.wrapping_sub(1);
    }
    #[inline]
    fn pull<B: Bus>(&mut self, bus: &mut B) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.rd(bus, 0x0100 | self.sp as u16)
    }

    #[inline]
    fn set_zn(&mut self, v: u8) {
        self.p &= !(FLAG_Z | FLAG_N);
        if v == 0 {
            self.p |= FLAG_Z;
        }
        self.p |= v & FLAG_N;
    }
    #[inline]
    fn set_flag(&mut self, f: u8, on: bool) {
        if on {
            self.p |= f;
        } else {
            self.p &= !f;
        }
    }
    #[inline]
    fn flag(&self, f: u8) -> bool {
        self.p & f != 0
    }

    /// Hardware reset: 7 cycles, loads PC from $FFFC.
    pub fn reset<B: Bus>(&mut self, bus: &mut B) {
        // The real chip performs 3 fake stack pushes (reads) during reset.
        let pc = self.pc;
        self.rd(bus, pc);
        self.rd(bus, pc);
        self.rd(bus, 0x0100 | self.sp as u16);
        self.sp = self.sp.wrapping_sub(1);
        self.rd(bus, 0x0100 | self.sp as u16);
        self.sp = self.sp.wrapping_sub(1);
        self.rd(bus, 0x0100 | self.sp as u16);
        self.sp = self.sp.wrapping_sub(1);
        self.p |= FLAG_I;
        let lo = self.rd(bus, 0xFFFC) as u16;
        let hi = self.rd(bus, 0xFFFD) as u16;
        self.pc = lo | (hi << 8);
        self.jammed = false;
        self.nmi_pending = false;
    }

    /// Request an NMI (edge triggered).
    pub fn nmi(&mut self) {
        self.nmi_pending = true;
    }
    /// Set the level of the IRQ line (not connected on a stock Apple-1).
    pub fn set_irq(&mut self, level: bool) {
        self.irq_line = level;
    }

    fn interrupt<B: Bus>(&mut self, bus: &mut B, vector: u16, brk: bool) {
        let pc = self.pc;
        self.rd(bus, pc);
        if !brk {
            self.rd(bus, pc);
        }
        // For BRK the opcode + padding byte were already fetched.
        self.push(bus, (self.pc >> 8) as u8);
        self.push(bus, self.pc as u8);
        let mut p = self.p | FLAG_U;
        if brk {
            p |= FLAG_B;
        } else {
            p &= !FLAG_B;
        }
        self.push(bus, p);
        self.p |= FLAG_I;
        let lo = self.rd(bus, vector) as u16;
        let hi = self.rd(bus, vector.wrapping_add(1)) as u16;
        self.pc = lo | (hi << 8);
    }

    /// Compute effective address for a mode. `write` forces the extra cycle of
    /// indexed modes (stores / RMW always take it).
    fn ea<B: Bus>(&mut self, bus: &mut B, mode: Mode, write: bool) -> u16 {
        match mode {
            Mode::Imm => {
                let a = self.pc;
                self.pc = self.pc.wrapping_add(1);
                a
            }
            Mode::Zp => self.fetch(bus) as u16,
            Mode::Zpx => {
                let b = self.fetch(bus);
                self.rd(bus, b as u16);
                b.wrapping_add(self.x) as u16
            }
            Mode::Zpy => {
                let b = self.fetch(bus);
                self.rd(bus, b as u16);
                b.wrapping_add(self.y) as u16
            }
            Mode::Abs => {
                let lo = self.fetch(bus) as u16;
                let hi = self.fetch(bus) as u16;
                lo | (hi << 8)
            }
            Mode::Abx | Mode::Aby => {
                let idx = if mode == Mode::Abx { self.x } else { self.y };
                let lo = self.fetch(bus) as u16;
                let hi = self.fetch(bus) as u16;
                let base = lo | (hi << 8);
                let addr = base.wrapping_add(idx as u16);
                if write || (base & 0xFF00) != (addr & 0xFF00) {
                    // dummy read at the un-carried address
                    self.rd(bus, (base & 0xFF00) | (addr & 0x00FF));
                }
                addr
            }
            Mode::Izx => {
                let b = self.fetch(bus);
                self.rd(bus, b as u16);
                let p = b.wrapping_add(self.x);
                let lo = self.rd(bus, p as u16) as u16;
                let hi = self.rd(bus, p.wrapping_add(1) as u16) as u16;
                lo | (hi << 8)
            }
            Mode::Izy => {
                let b = self.fetch(bus);
                let lo = self.rd(bus, b as u16) as u16;
                let hi = self.rd(bus, b.wrapping_add(1) as u16) as u16;
                let base = lo | (hi << 8);
                let addr = base.wrapping_add(self.y as u16);
                if write || (base & 0xFF00) != (addr & 0xFF00) {
                    self.rd(bus, (base & 0xFF00) | (addr & 0x00FF));
                }
                addr
            }
        }
    }

    #[inline]
    fn load<B: Bus>(&mut self, bus: &mut B, mode: Mode) -> u8 {
        let a = self.ea(bus, mode, false);
        self.rd(bus, a)
    }

    #[inline]
    fn store<B: Bus>(&mut self, bus: &mut B, mode: Mode, v: u8) {
        let a = self.ea(bus, mode, true);
        self.wr(bus, a, v);
    }

    /// Read-modify-write on memory: read, dummy write of the old value, write.
    fn rmw<B: Bus>(&mut self, bus: &mut B, mode: Mode, f: fn(&mut Cpu, u8) -> u8) {
        let a = self.ea(bus, mode, true);
        let v = self.rd(bus, a);
        self.wr(bus, a, v);
        let r = f(self, v);
        self.wr(bus, a, r);
    }

    fn adc(&mut self, v: u8) {
        let c = (self.p & FLAG_C) as u16;
        let a = self.a as u16;
        let b = v as u16;
        if self.flag(FLAG_D) {
            // NMOS decimal mode
            let bin = a + b + c;
            let mut al = (a & 0x0F) + (b & 0x0F) + c;
            if al >= 0x0A {
                al = ((al + 0x06) & 0x0F) + 0x10;
            }
            let mut r = (a & 0xF0) + (b & 0xF0) + al;
            // N and V come from the intermediate result
            self.set_flag(FLAG_N, r & 0x80 != 0);
            self.set_flag(FLAG_V, (!(a ^ b) & (a ^ r) & 0x80) != 0);
            if r >= 0xA0 {
                r += 0x60;
            }
            self.set_flag(FLAG_C, r >= 0x100);
            self.set_flag(FLAG_Z, bin & 0xFF == 0);
            self.a = r as u8;
        } else {
            let r = a + b + c;
            self.set_flag(FLAG_C, r > 0xFF);
            self.set_flag(FLAG_V, (!(a ^ b) & (a ^ r) & 0x80) != 0);
            self.a = r as u8;
            self.set_zn(self.a);
        }
    }

    fn sbc(&mut self, v: u8) {
        let c = (self.p & FLAG_C) as i16;
        let a = self.a as i16;
        let b = v as i16;
        let bin = a - b - (1 - c);
        // flags are always computed from the binary result on NMOS
        self.set_flag(FLAG_C, bin >= 0);
        self.set_flag(FLAG_V, ((a ^ b) & (a ^ bin) & 0x80) != 0);
        let binu = bin as u8;
        self.set_zn(binu);
        if self.flag(FLAG_D) {
            let mut al = (a & 0x0F) - (b & 0x0F) + c - 1;
            if al < 0 {
                al = ((al - 0x06) & 0x0F) - 0x10;
            }
            let mut r = (a & 0xF0) - (b & 0xF0) + al;
            if r < 0 {
                r -= 0x60;
            }
            self.a = r as u8;
        } else {
            self.a = binu;
        }
    }

    fn cmp(&mut self, r: u8, v: u8) {
        let t = r.wrapping_sub(v);
        self.set_flag(FLAG_C, r >= v);
        self.set_zn(t);
    }

    fn bit(&mut self, v: u8) {
        self.set_flag(FLAG_Z, self.a & v == 0);
        self.set_flag(FLAG_N, v & 0x80 != 0);
        self.set_flag(FLAG_V, v & 0x40 != 0);
    }

    fn op_asl(&mut self, v: u8) -> u8 {
        self.set_flag(FLAG_C, v & 0x80 != 0);
        let r = v << 1;
        self.set_zn(r);
        r
    }
    fn op_lsr(&mut self, v: u8) -> u8 {
        self.set_flag(FLAG_C, v & 1 != 0);
        let r = v >> 1;
        self.set_zn(r);
        r
    }
    fn op_rol(&mut self, v: u8) -> u8 {
        let c = self.p & FLAG_C;
        self.set_flag(FLAG_C, v & 0x80 != 0);
        let r = (v << 1) | c;
        self.set_zn(r);
        r
    }
    fn op_ror(&mut self, v: u8) -> u8 {
        let c = self.p & FLAG_C;
        self.set_flag(FLAG_C, v & 1 != 0);
        let r = (v >> 1) | (c << 7);
        self.set_zn(r);
        r
    }
    fn op_inc(&mut self, v: u8) -> u8 {
        let r = v.wrapping_add(1);
        self.set_zn(r);
        r
    }
    fn op_dec(&mut self, v: u8) -> u8 {
        let r = v.wrapping_sub(1);
        self.set_zn(r);
        r
    }

    fn branch<B: Bus>(&mut self, bus: &mut B, cond: bool) {
        let off = self.fetch(bus) as i8;
        if cond {
            let pc = self.pc;
            self.rd(bus, pc);
            let target = pc.wrapping_add(off as i16 as u16);
            if (target & 0xFF00) != (pc & 0xFF00) {
                self.rd(bus, (pc & 0xFF00) | (target & 0x00FF));
            }
            self.pc = target;
        }
    }

    #[inline]
    fn implied<B: Bus>(&mut self, bus: &mut B) {
        let pc = self.pc;
        self.rd(bus, pc);
    }

    /// Execute one instruction (servicing a pending interrupt first).
    /// Returns the number of cycles consumed.
    pub fn step<B: Bus>(&mut self, bus: &mut B) -> u32 {
        let start = self.cycles;
        if self.jammed {
            self.cycles += 1;
            return 1;
        }
        if self.nmi_pending {
            self.nmi_pending = false;
            self.interrupt(bus, 0xFFFA, false);
            return (self.cycles - start) as u32;
        }
        if self.irq_line && !self.flag(FLAG_I) {
            self.interrupt(bus, 0xFFFE, false);
            return (self.cycles - start) as u32;
        }
        let op = self.fetch(bus);
        use Mode::*;
        match op {
            // ---- loads ----
            0xA9 => { let v = self.load(bus, Imm); self.a = v; self.set_zn(v); }
            0xA5 => { let v = self.load(bus, Zp); self.a = v; self.set_zn(v); }
            0xB5 => { let v = self.load(bus, Zpx); self.a = v; self.set_zn(v); }
            0xAD => { let v = self.load(bus, Abs); self.a = v; self.set_zn(v); }
            0xBD => { let v = self.load(bus, Abx); self.a = v; self.set_zn(v); }
            0xB9 => { let v = self.load(bus, Aby); self.a = v; self.set_zn(v); }
            0xA1 => { let v = self.load(bus, Izx); self.a = v; self.set_zn(v); }
            0xB1 => { let v = self.load(bus, Izy); self.a = v; self.set_zn(v); }
            0xA2 => { let v = self.load(bus, Imm); self.x = v; self.set_zn(v); }
            0xA6 => { let v = self.load(bus, Zp); self.x = v; self.set_zn(v); }
            0xB6 => { let v = self.load(bus, Zpy); self.x = v; self.set_zn(v); }
            0xAE => { let v = self.load(bus, Abs); self.x = v; self.set_zn(v); }
            0xBE => { let v = self.load(bus, Aby); self.x = v; self.set_zn(v); }
            0xA0 => { let v = self.load(bus, Imm); self.y = v; self.set_zn(v); }
            0xA4 => { let v = self.load(bus, Zp); self.y = v; self.set_zn(v); }
            0xB4 => { let v = self.load(bus, Zpx); self.y = v; self.set_zn(v); }
            0xAC => { let v = self.load(bus, Abs); self.y = v; self.set_zn(v); }
            0xBC => { let v = self.load(bus, Abx); self.y = v; self.set_zn(v); }
            // ---- stores ----
            0x85 => { let v = self.a; self.store(bus, Zp, v); }
            0x95 => { let v = self.a; self.store(bus, Zpx, v); }
            0x8D => { let v = self.a; self.store(bus, Abs, v); }
            0x9D => { let v = self.a; self.store(bus, Abx, v); }
            0x99 => { let v = self.a; self.store(bus, Aby, v); }
            0x81 => { let v = self.a; self.store(bus, Izx, v); }
            0x91 => { let v = self.a; self.store(bus, Izy, v); }
            0x86 => { let v = self.x; self.store(bus, Zp, v); }
            0x96 => { let v = self.x; self.store(bus, Zpy, v); }
            0x8E => { let v = self.x; self.store(bus, Abs, v); }
            0x84 => { let v = self.y; self.store(bus, Zp, v); }
            0x94 => { let v = self.y; self.store(bus, Zpx, v); }
            0x8C => { let v = self.y; self.store(bus, Abs, v); }
            // ---- transfers ----
            0xAA => { self.implied(bus); self.x = self.a; self.set_zn(self.x); }
            0xA8 => { self.implied(bus); self.y = self.a; self.set_zn(self.y); }
            0x8A => { self.implied(bus); self.a = self.x; self.set_zn(self.a); }
            0x98 => { self.implied(bus); self.a = self.y; self.set_zn(self.a); }
            0xBA => { self.implied(bus); self.x = self.sp; self.set_zn(self.x); }
            0x9A => { self.implied(bus); self.sp = self.x; }
            // ---- stack ----
            0x48 => { self.implied(bus); let v = self.a; self.push(bus, v); }
            0x08 => { self.implied(bus); let v = self.p | FLAG_B | FLAG_U; self.push(bus, v); }
            0x68 => {
                self.implied(bus);
                let s = 0x0100 | self.sp as u16;
                self.rd(bus, s);
                let v = self.pull(bus);
                self.a = v;
                self.set_zn(v);
            }
            0x28 => {
                self.implied(bus);
                let s = 0x0100 | self.sp as u16;
                self.rd(bus, s);
                let v = self.pull(bus);
                self.p = (v & !FLAG_B) | FLAG_U;
            }
            // ---- logic ----
            0x29 => { let v = self.load(bus, Imm); self.a &= v; self.set_zn(self.a); }
            0x25 => { let v = self.load(bus, Zp); self.a &= v; self.set_zn(self.a); }
            0x35 => { let v = self.load(bus, Zpx); self.a &= v; self.set_zn(self.a); }
            0x2D => { let v = self.load(bus, Abs); self.a &= v; self.set_zn(self.a); }
            0x3D => { let v = self.load(bus, Abx); self.a &= v; self.set_zn(self.a); }
            0x39 => { let v = self.load(bus, Aby); self.a &= v; self.set_zn(self.a); }
            0x21 => { let v = self.load(bus, Izx); self.a &= v; self.set_zn(self.a); }
            0x31 => { let v = self.load(bus, Izy); self.a &= v; self.set_zn(self.a); }
            0x09 => { let v = self.load(bus, Imm); self.a |= v; self.set_zn(self.a); }
            0x05 => { let v = self.load(bus, Zp); self.a |= v; self.set_zn(self.a); }
            0x15 => { let v = self.load(bus, Zpx); self.a |= v; self.set_zn(self.a); }
            0x0D => { let v = self.load(bus, Abs); self.a |= v; self.set_zn(self.a); }
            0x1D => { let v = self.load(bus, Abx); self.a |= v; self.set_zn(self.a); }
            0x19 => { let v = self.load(bus, Aby); self.a |= v; self.set_zn(self.a); }
            0x01 => { let v = self.load(bus, Izx); self.a |= v; self.set_zn(self.a); }
            0x11 => { let v = self.load(bus, Izy); self.a |= v; self.set_zn(self.a); }
            0x49 => { let v = self.load(bus, Imm); self.a ^= v; self.set_zn(self.a); }
            0x45 => { let v = self.load(bus, Zp); self.a ^= v; self.set_zn(self.a); }
            0x55 => { let v = self.load(bus, Zpx); self.a ^= v; self.set_zn(self.a); }
            0x4D => { let v = self.load(bus, Abs); self.a ^= v; self.set_zn(self.a); }
            0x5D => { let v = self.load(bus, Abx); self.a ^= v; self.set_zn(self.a); }
            0x59 => { let v = self.load(bus, Aby); self.a ^= v; self.set_zn(self.a); }
            0x41 => { let v = self.load(bus, Izx); self.a ^= v; self.set_zn(self.a); }
            0x51 => { let v = self.load(bus, Izy); self.a ^= v; self.set_zn(self.a); }
            0x24 => { let v = self.load(bus, Zp); self.bit(v); }
            0x2C => { let v = self.load(bus, Abs); self.bit(v); }
            // ---- arithmetic ----
            0x69 => { let v = self.load(bus, Imm); self.adc(v); }
            0x65 => { let v = self.load(bus, Zp); self.adc(v); }
            0x75 => { let v = self.load(bus, Zpx); self.adc(v); }
            0x6D => { let v = self.load(bus, Abs); self.adc(v); }
            0x7D => { let v = self.load(bus, Abx); self.adc(v); }
            0x79 => { let v = self.load(bus, Aby); self.adc(v); }
            0x61 => { let v = self.load(bus, Izx); self.adc(v); }
            0x71 => { let v = self.load(bus, Izy); self.adc(v); }
            0xE9 => { let v = self.load(bus, Imm); self.sbc(v); }
            0xE5 => { let v = self.load(bus, Zp); self.sbc(v); }
            0xF5 => { let v = self.load(bus, Zpx); self.sbc(v); }
            0xED => { let v = self.load(bus, Abs); self.sbc(v); }
            0xFD => { let v = self.load(bus, Abx); self.sbc(v); }
            0xF9 => { let v = self.load(bus, Aby); self.sbc(v); }
            0xE1 => { let v = self.load(bus, Izx); self.sbc(v); }
            0xF1 => { let v = self.load(bus, Izy); self.sbc(v); }
            0xC9 => { let v = self.load(bus, Imm); self.cmp(self.a, v); }
            0xC5 => { let v = self.load(bus, Zp); self.cmp(self.a, v); }
            0xD5 => { let v = self.load(bus, Zpx); self.cmp(self.a, v); }
            0xCD => { let v = self.load(bus, Abs); self.cmp(self.a, v); }
            0xDD => { let v = self.load(bus, Abx); self.cmp(self.a, v); }
            0xD9 => { let v = self.load(bus, Aby); self.cmp(self.a, v); }
            0xC1 => { let v = self.load(bus, Izx); self.cmp(self.a, v); }
            0xD1 => { let v = self.load(bus, Izy); self.cmp(self.a, v); }
            0xE0 => { let v = self.load(bus, Imm); self.cmp(self.x, v); }
            0xE4 => { let v = self.load(bus, Zp); self.cmp(self.x, v); }
            0xEC => { let v = self.load(bus, Abs); self.cmp(self.x, v); }
            0xC0 => { let v = self.load(bus, Imm); self.cmp(self.y, v); }
            0xC4 => { let v = self.load(bus, Zp); self.cmp(self.y, v); }
            0xCC => { let v = self.load(bus, Abs); self.cmp(self.y, v); }
            // ---- inc / dec ----
            0xE6 => self.rmw(bus, Zp, Cpu::op_inc),
            0xF6 => self.rmw(bus, Zpx, Cpu::op_inc),
            0xEE => self.rmw(bus, Abs, Cpu::op_inc),
            0xFE => self.rmw(bus, Abx, Cpu::op_inc),
            0xC6 => self.rmw(bus, Zp, Cpu::op_dec),
            0xD6 => self.rmw(bus, Zpx, Cpu::op_dec),
            0xCE => self.rmw(bus, Abs, Cpu::op_dec),
            0xDE => self.rmw(bus, Abx, Cpu::op_dec),
            0xE8 => { self.implied(bus); self.x = self.x.wrapping_add(1); self.set_zn(self.x); }
            0xC8 => { self.implied(bus); self.y = self.y.wrapping_add(1); self.set_zn(self.y); }
            0xCA => { self.implied(bus); self.x = self.x.wrapping_sub(1); self.set_zn(self.x); }
            0x88 => { self.implied(bus); self.y = self.y.wrapping_sub(1); self.set_zn(self.y); }
            // ---- shifts ----
            0x0A => { self.implied(bus); let a = self.a; self.a = self.op_asl(a); }
            0x4A => { self.implied(bus); let a = self.a; self.a = self.op_lsr(a); }
            0x2A => { self.implied(bus); let a = self.a; self.a = self.op_rol(a); }
            0x6A => { self.implied(bus); let a = self.a; self.a = self.op_ror(a); }
            0x06 => self.rmw(bus, Zp, Cpu::op_asl),
            0x16 => self.rmw(bus, Zpx, Cpu::op_asl),
            0x0E => self.rmw(bus, Abs, Cpu::op_asl),
            0x1E => self.rmw(bus, Abx, Cpu::op_asl),
            0x46 => self.rmw(bus, Zp, Cpu::op_lsr),
            0x56 => self.rmw(bus, Zpx, Cpu::op_lsr),
            0x4E => self.rmw(bus, Abs, Cpu::op_lsr),
            0x5E => self.rmw(bus, Abx, Cpu::op_lsr),
            0x26 => self.rmw(bus, Zp, Cpu::op_rol),
            0x36 => self.rmw(bus, Zpx, Cpu::op_rol),
            0x2E => self.rmw(bus, Abs, Cpu::op_rol),
            0x3E => self.rmw(bus, Abx, Cpu::op_rol),
            0x66 => self.rmw(bus, Zp, Cpu::op_ror),
            0x76 => self.rmw(bus, Zpx, Cpu::op_ror),
            0x6E => self.rmw(bus, Abs, Cpu::op_ror),
            0x7E => self.rmw(bus, Abx, Cpu::op_ror),
            // ---- jumps / calls ----
            0x4C => {
                let lo = self.fetch(bus) as u16;
                let hi = self.fetch(bus) as u16;
                self.pc = lo | (hi << 8);
            }
            0x6C => {
                let lo = self.fetch(bus) as u16;
                let hi = self.fetch(bus) as u16;
                let ptr = lo | (hi << 8);
                let l = self.rd(bus, ptr) as u16;
                // NMOS bug: high byte fetched from the same page
                let h = self.rd(bus, (ptr & 0xFF00) | (ptr.wrapping_add(1) & 0x00FF)) as u16;
                self.pc = l | (h << 8);
            }
            0x20 => {
                let lo = self.fetch(bus) as u16;
                let s = 0x0100 | self.sp as u16;
                self.rd(bus, s);
                let pc = self.pc;
                self.push(bus, (pc >> 8) as u8);
                self.push(bus, pc as u8);
                let hi = self.rd(bus, self.pc) as u16;
                self.pc = lo | (hi << 8);
            }
            0x60 => {
                self.implied(bus);
                let s = 0x0100 | self.sp as u16;
                self.rd(bus, s);
                let lo = self.pull(bus) as u16;
                let hi = self.pull(bus) as u16;
                self.pc = lo | (hi << 8);
                let pc = self.pc;
                self.rd(bus, pc);
                self.pc = self.pc.wrapping_add(1);
            }
            0x40 => {
                self.implied(bus);
                let s = 0x0100 | self.sp as u16;
                self.rd(bus, s);
                let p = self.pull(bus);
                self.p = (p & !FLAG_B) | FLAG_U;
                let lo = self.pull(bus) as u16;
                let hi = self.pull(bus) as u16;
                self.pc = lo | (hi << 8);
            }
            0x00 => {
                // BRK: opcode + padding byte
                self.pc = self.pc.wrapping_add(1);
                self.cycles += 1; // padding byte read
                bus.read(self.pc.wrapping_sub(1));
                self.push(bus, (self.pc >> 8) as u8);
                self.push(bus, self.pc as u8);
                let p = self.p | FLAG_B | FLAG_U;
                self.push(bus, p);
                self.p |= FLAG_I;
                let lo = self.rd(bus, 0xFFFE) as u16;
                let hi = self.rd(bus, 0xFFFF) as u16;
                self.pc = lo | (hi << 8);
            }
            // ---- branches ----
            0x10 => { let c = !self.flag(FLAG_N); self.branch(bus, c); }
            0x30 => { let c = self.flag(FLAG_N); self.branch(bus, c); }
            0x50 => { let c = !self.flag(FLAG_V); self.branch(bus, c); }
            0x70 => { let c = self.flag(FLAG_V); self.branch(bus, c); }
            0x90 => { let c = !self.flag(FLAG_C); self.branch(bus, c); }
            0xB0 => { let c = self.flag(FLAG_C); self.branch(bus, c); }
            0xD0 => { let c = !self.flag(FLAG_Z); self.branch(bus, c); }
            0xF0 => { let c = self.flag(FLAG_Z); self.branch(bus, c); }
            // ---- flags ----
            0x18 => { self.implied(bus); self.p &= !FLAG_C; }
            0x38 => { self.implied(bus); self.p |= FLAG_C; }
            0x58 => { self.implied(bus); self.p &= !FLAG_I; }
            0x78 => { self.implied(bus); self.p |= FLAG_I; }
            0xB8 => { self.implied(bus); self.p &= !FLAG_V; }
            0xD8 => { self.implied(bus); self.p &= !FLAG_D; }
            0xF8 => { self.implied(bus); self.p |= FLAG_D; }
            0xEA => { self.implied(bus); }
            // ---- undocumented: JAM ----
            0x02 | 0x12 | 0x22 | 0x32 | 0x42 | 0x52 | 0x62 | 0x72 | 0x92 | 0xB2 | 0xD2 | 0xF2 => {
                self.pc = self.pc.wrapping_sub(1);
                self.jammed = true;
            }
            // ---- undocumented: treat as NOPs of the right size ----
            _ => self.undocumented_nop(bus, op),
        }
        (self.cycles - start) as u32
    }

    fn undocumented_nop<B: Bus>(&mut self, bus: &mut B, op: u8) {
        // Size by addressing mode column (approximation, reads are performed).
        match op & 0x1F {
            0x00 | 0x02 | 0x09 | 0x0B => {
                self.load(bus, Mode::Imm);
            }
            0x04..=0x07 => {
                self.load(bus, Mode::Zp);
            }
            0x14..=0x17 => {
                self.load(bus, Mode::Zpx);
            }
            0x0C..=0x0F => {
                self.load(bus, Mode::Abs);
            }
            0x1C | 0x1D | 0x1E | 0x1F | 0x1B | 0x19 => {
                self.load(bus, Mode::Abx);
            }
            0x01 | 0x03 => {
                self.load(bus, Mode::Izx);
            }
            0x11 | 0x13 => {
                self.load(bus, Mode::Izy);
            }
            _ => self.implied(bus),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Flat(Vec<u8>);
    impl Bus for Flat {
        fn read(&mut self, a: u16) -> u8 {
            self.0[a as usize]
        }
        fn write(&mut self, a: u16, v: u8) {
            self.0[a as usize] = v;
        }
    }

    fn run(code: &[u8], steps: usize) -> (Cpu, Flat) {
        let mut m = Flat(vec![0; 65536]);
        m.0[0x0200..0x0200 + code.len()].copy_from_slice(code);
        m.0[0xFFFC] = 0x00;
        m.0[0xFFFD] = 0x02;
        let mut c = Cpu::new();
        c.reset(&mut m);
        for _ in 0..steps {
            c.step(&mut m);
        }
        (c, m)
    }

    #[test]
    fn lda_sta_and_cycles() {
        let (c, m) = run(&[0xA9, 0x42, 0x8D, 0x00, 0x03], 2);
        assert_eq!(c.a, 0x42);
        assert_eq!(m.0[0x300], 0x42);
        assert_eq!(c.cycles, 7 + 2 + 4);
    }

    #[test]
    fn page_cross_penalty() {
        // LDX #$01 ; LDA $02FF,X  -> 4+1 cycles
        let (c, _) = run(&[0xA2, 0x01, 0xBD, 0xFF, 0x02], 2);
        assert_eq!(c.cycles, 7 + 2 + 5);
    }

    #[test]
    fn decimal_adc_sbc() {
        // SED; CLC; LDA #$58; ADC #$46 -> $04, C=1
        let (c, _) = run(&[0xF8, 0x18, 0xA9, 0x58, 0x69, 0x46], 4);
        assert_eq!(c.a, 0x04);
        assert!(c.p & FLAG_C != 0);
        // SED; SEC; LDA #$46; SBC #$12 -> $34
        let (c, _) = run(&[0xF8, 0x38, 0xA9, 0x46, 0xE9, 0x12], 4);
        assert_eq!(c.a, 0x34);
        assert!(c.p & FLAG_C != 0);
        // SED; SEC; LDA #$12; SBC #$21 -> $91, C=0
        let (c, _) = run(&[0xF8, 0x38, 0xA9, 0x12, 0xE9, 0x21], 4);
        assert_eq!(c.a, 0x91);
        assert!(c.p & FLAG_C == 0);
        // NMOS quirk: SED CLC LDA #$99 ADC #$01 -> A=0, Z=0 (binary result $9A), N=1
        let (c, _) = run(&[0xF8, 0x18, 0xA9, 0x99, 0x69, 0x01], 4);
        assert_eq!(c.a, 0x00);
        assert!(c.p & FLAG_C != 0);
        assert!(c.p & FLAG_Z == 0);
    }

    #[test]
    fn jmp_indirect_bug() {
        let mut code = vec![0x6C, 0xFF, 0x03];
        code.resize(3, 0);
        let mut m = Flat(vec![0; 65536]);
        m.0[0x0200..0x0203].copy_from_slice(&code);
        m.0[0x03FF] = 0x34;
        m.0[0x0300] = 0x12; // bug: high byte comes from $0300, not $0400
        m.0[0x0400] = 0x99;
        m.0[0xFFFC] = 0x00;
        m.0[0xFFFD] = 0x02;
        let mut c = Cpu::new();
        c.reset(&mut m);
        c.step(&mut m);
        assert_eq!(c.pc, 0x1234);
    }

    #[test]
    fn jsr_rts() {
        // JSR $0210 ; at 0210: RTS
        let mut m = Flat(vec![0; 65536]);
        m.0[0x0200..0x0203].copy_from_slice(&[0x20, 0x10, 0x02]);
        m.0[0x0210] = 0x60;
        m.0[0xFFFD] = 0x02;
        let mut c = Cpu::new();
        c.reset(&mut m);
        let s0 = c.sp;
        assert_eq!(c.step(&mut m), 6);
        assert_eq!(c.pc, 0x0210);
        assert_eq!(c.step(&mut m), 6);
        assert_eq!(c.pc, 0x0203);
        assert_eq!(c.sp, s0);
    }
}
