//! Save state: a running program (RAM, CPU registers, PIA, terminal, ACI,
//! GEN2 colour card, keyboard queue, tape in the player…) written to a
//! file and restored exactly later.
//!
//! File layout (all integers little endian):
//!
//! ```text
//! "A1RS"  u8 version (1)
//! u32 length + UTF-8 title (free text, e.g. the program name)
//! config  : ram_kb u32, flags u8 (bit0 e000, bit1 aci, bit2 throttle,
//!           bit3 refresh, bit4 colour card, bit5 50 Hz, bit6 custom ROMs),
//!           clock_hz f64, [monitor ROM 256, ACI ROM 256 if bit6]
//! cpu     : a x y sp p u8, pc u16, cycles u64, flags u8 (jammed, nmi, irq)
//! board   : cycles, display_busy_until, next_key_cycle u64, tape_out u8,
//!           aci_accesses u64, keyboard queue (u32 n + bytes)
//! pia     : ora ddra cra orb ddrb crb pa_in pb7_busy
//! display : 24 x 40 cells, col, row (u8)
//! memory  : u32 number of pages, then (page u8 + 256 bytes) for each RAM page
//! gen2    : text mixed page2 hires hz50 (u8), rng u32, accesses u64,
//!           current and previous frame logs
//! tape    : u8 present, then edges, started_at, autostart, idx, level, name
//! ```
//!
//! ROMs are not stored unless they differ from the built-in ones, so a
//! state file never carries copyrighted code the user did not load
//! himself (programs in RAM are the user's own).

use crate::display::{COLS, ROWS};
use crate::gen2::{FrameLog, Modes};
use crate::machine::{Board, Config, Machine};
use crate::roms::{ACI_ROM, WOZ_MONITOR};
use crate::tape::TapePlayer;
use crate::timing::Timing;

const MAGIC: &[u8; 4] = b"A1RS";
const VERSION: u8 = 1;
/// Refuse absurd sizes in a damaged or foreign file.
const MAX_ITEMS: usize = 64 * 1024 * 1024;

struct W(Vec<u8>);

impl W {
    fn u8(&mut self, v: u8) {
        self.0.push(v);
    }
    fn bool(&mut self, v: bool) {
        self.0.push(v as u8);
    }
    fn u16(&mut self, v: u16) {
        self.0.extend_from_slice(&v.to_le_bytes());
    }
    fn u32(&mut self, v: u32) {
        self.0.extend_from_slice(&v.to_le_bytes());
    }
    fn u64(&mut self, v: u64) {
        self.0.extend_from_slice(&v.to_le_bytes());
    }
    fn f64(&mut self, v: f64) {
        self.0.extend_from_slice(&v.to_le_bytes());
    }
    fn bytes(&mut self, v: &[u8]) {
        self.u32(v.len() as u32);
        self.0.extend_from_slice(v);
    }
    fn modes(&mut self, m: Modes) {
        self.u8(m.text as u8 | (m.mixed as u8) << 1 | (m.page2 as u8) << 2 | (m.hires as u8) << 3);
    }
    fn log(&mut self, l: &FrameLog) {
        self.u64(l.frame);
        self.modes(l.start);
        self.u32(l.changes.len() as u32);
        for (line, m) in &l.changes {
            self.u32(*line);
            self.modes(*m);
        }
    }
}

struct R<'a> {
    b: &'a [u8],
    at: usize,
}

type Res<T> = Result<T, String>;

impl<'a> R<'a> {
    fn take(&mut self, n: usize) -> Res<&'a [u8]> {
        let end = self.at.checked_add(n).filter(|e| *e <= self.b.len()).ok_or("state file truncated")?;
        let s = &self.b[self.at..end];
        self.at = end;
        Ok(s)
    }
    fn u8(&mut self) -> Res<u8> {
        Ok(self.take(1)?[0])
    }
    fn bool(&mut self) -> Res<bool> {
        Ok(self.u8()? != 0)
    }
    fn u16(&mut self) -> Res<u16> {
        Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }
    fn u32(&mut self) -> Res<u32> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn u64(&mut self) -> Res<u64> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
    fn f64(&mut self) -> Res<f64> {
        Ok(f64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
    fn len(&mut self) -> Res<usize> {
        let n = self.u32()? as usize;
        if n > MAX_ITEMS {
            return Err("state file damaged".into());
        }
        Ok(n)
    }
    fn bytes(&mut self) -> Res<&'a [u8]> {
        let n = self.len()?;
        self.take(n)
    }
    fn modes(&mut self) -> Res<Modes> {
        let v = self.u8()?;
        Ok(Modes { text: v & 1 != 0, mixed: v & 2 != 0, page2: v & 4 != 0, hires: v & 8 != 0 })
    }
    fn log(&mut self) -> Res<FrameLog> {
        let frame = self.u64()?;
        let start = self.modes()?;
        let n = self.len()?;
        let mut changes = Vec::with_capacity(n.min(4096));
        for _ in 0..n {
            let line = self.u32()?;
            changes.push((line, self.modes()?));
        }
        Ok(FrameLog { frame, start, changes })
    }
}

/// What a state file says about itself (without restoring it).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateInfo {
    pub title: String,
    pub pc: u16,
    pub cycles: u64,
    pub ram_kb: u32,
    pub color_card: bool,
    pub tape: bool,
}

impl Machine {
    /// Serialise the machine. `title` is free text shown when loading.
    pub fn save_state(&self, title: &str) -> Vec<u8> {
        let b = &self.board;
        let c = &self.cpu;
        let mut w = W(Vec::with_capacity(20_000));
        w.0.extend_from_slice(MAGIC);
        w.u8(VERSION);
        w.bytes(title.as_bytes());
        // config
        let cfg = &b.config;
        let custom = cfg.monitor_rom != *WOZ_MONITOR || cfg.aci_rom != *ACI_ROM;
        w.u32(cfg.ram_kb);
        w.u8(cfg.ram_e000 as u8 | (cfg.aci as u8) << 1 | (cfg.display_throttle as u8) << 2 | (cfg.timing.refresh_stall as u8) << 3 | (cfg.color_card as u8) << 4 | (cfg.color_50hz as u8) << 5 | (custom as u8) << 6);
        w.f64(cfg.timing.clock_hz);
        if custom {
            w.0.extend_from_slice(&cfg.monitor_rom);
            w.0.extend_from_slice(&cfg.aci_rom);
        }
        // cpu
        for v in [c.a, c.x, c.y, c.sp, c.p] {
            w.u8(v);
        }
        w.u16(c.pc);
        w.u64(c.cycles);
        w.u8(c.jammed as u8 | (c.nmi_pending as u8) << 1 | (c.irq_line as u8) << 2);
        // board
        w.u64(b.cycles);
        w.u64(b.display_busy_until);
        w.u64(b.next_key_cycle);
        w.bool(b.tape_out);
        w.u64(b.aci_accesses);
        let keys: Vec<u8> = b.key_queue.iter().copied().collect();
        w.bytes(&keys);
        // pia
        let p = &b.pia;
        for v in [p.ora, p.ddra, p.cra, p.orb, p.ddrb, p.crb, p.pa_in] {
            w.u8(v);
        }
        w.bool(p.pb7_busy);
        // display
        for r in 0..ROWS {
            w.0.extend_from_slice(&b.display.cells[r]);
        }
        w.u8(b.display.col as u8);
        w.u8(b.display.row as u8);
        // RAM pages
        let pages: Vec<usize> = (0..256).filter(|p| b.ram_page[*p]).collect();
        w.u32(pages.len() as u32);
        for pg in pages {
            w.u8(pg as u8);
            w.0.extend_from_slice(&b.mem[pg * 256..pg * 256 + 256]);
        }
        // gen2
        let g = &b.gen2;
        for v in [g.text, g.mixed, g.page2, g.hires, g.hz50] {
            w.bool(v);
        }
        w.u32(g.rng);
        w.u64(g.accesses);
        w.log(&g.cur);
        match &g.prev {
            Some(l) => {
                w.u8(1);
                w.log(l);
            }
            None => w.u8(0),
        }
        // tape in the player
        match &b.player {
            Some(t) => {
                w.u8(1);
                w.u32(t.edges.len() as u32);
                for e in &t.edges {
                    w.f64(*e);
                }
                w.u8(t.started_at.is_some() as u8);
                w.f64(t.started_at.unwrap_or(0.0));
                w.bool(t.autostart);
                w.u32(t.idx as u32);
                w.bool(t.level);
                w.bytes(t.name.as_bytes());
            }
            None => w.u8(0),
        }
        w.0
    }

    /// Rebuild a machine from [`Machine::save_state`] bytes.
    pub fn load_state(bytes: &[u8]) -> Result<(Machine, StateInfo), String> {
        let mut r = R { b: bytes, at: 0 };
        if r.take(4)? != MAGIC {
            return Err("not an Apple-1 state file".into());
        }
        let v = r.u8()?;
        if v != VERSION {
            return Err(format!("state file version {v} not supported (this program reads version {VERSION})"));
        }
        let title = String::from_utf8_lossy(r.bytes()?).into_owned();
        let ram_kb = r.u32()?;
        let f = r.u8()?;
        let clock_hz = r.f64()?;
        let mut cfg = Config {
            ram_kb: ram_kb.clamp(4, 48),
            ram_e000: f & 1 != 0,
            aci: f & 2 != 0,
            display_throttle: f & 4 != 0,
            timing: Timing { clock_hz, refresh_stall: f & 8 != 0 },
            color_card: f & 16 != 0,
            color_50hz: f & 32 != 0,
            ..Config::default()
        };
        if f & 64 != 0 {
            cfg.monitor_rom.copy_from_slice(r.take(256)?);
            cfg.aci_rom.copy_from_slice(r.take(256)?);
        }
        let mut board = Board::new(cfg);
        // the card's random power-up screen must not leak into the restore
        for b in board.mem.iter_mut().take(0xC000) {
            *b = 0;
        }
        let mut m = Machine { cpu: crate::cpu::Cpu::new(), board };
        {
            let c = &mut m.cpu;
            c.a = r.u8()?;
            c.x = r.u8()?;
            c.y = r.u8()?;
            c.sp = r.u8()?;
            c.p = r.u8()?;
            c.pc = r.u16()?;
            c.cycles = r.u64()?;
            let fl = r.u8()?;
            c.jammed = fl & 1 != 0;
            c.nmi_pending = fl & 2 != 0;
            c.irq_line = fl & 4 != 0;
        }
        let b = &mut m.board;
        b.cycles = r.u64()?;
        b.display_busy_until = r.u64()?;
        b.next_key_cycle = r.u64()?;
        b.tape_out = r.bool()?;
        b.aci_accesses = r.u64()?;
        b.key_queue = r.bytes()?.iter().copied().collect();
        {
            let p = &mut b.pia;
            p.ora = r.u8()?;
            p.ddra = r.u8()?;
            p.cra = r.u8()?;
            p.orb = r.u8()?;
            p.ddrb = r.u8()?;
            p.crb = r.u8()?;
            p.pa_in = r.u8()?;
            p.pb7_busy = r.bool()?;
        }
        for row in 0..ROWS {
            b.display.cells[row].copy_from_slice(r.take(COLS)?);
        }
        b.display.col = (r.u8()? as usize).min(COLS - 1);
        b.display.row = (r.u8()? as usize).min(ROWS - 1);
        b.display.generation += 1;
        let n = r.len()?;
        if n > 256 {
            return Err("state file damaged".into());
        }
        for _ in 0..n {
            let pg = r.u8()? as usize;
            let data = r.take(256)?;
            if b.ram_page[pg] {
                b.mem[pg * 256..pg * 256 + 256].copy_from_slice(data);
            }
        }
        {
            let g = &mut b.gen2;
            g.text = r.bool()?;
            g.mixed = r.bool()?;
            g.page2 = r.bool()?;
            g.hires = r.bool()?;
            g.hz50 = r.bool()?;
            g.rng = r.u32()?;
            g.accesses = r.u64()?;
            g.cur = r.log()?;
            g.prev = if r.u8()? != 0 { Some(r.log()?) } else { None };
        }
        let tape = r.u8()? != 0;
        if tape {
            let n = r.len()?;
            let mut edges = Vec::with_capacity(n.min(1 << 20));
            for _ in 0..n {
                edges.push(r.f64()?);
            }
            let has_start = r.u8()? != 0;
            let st = r.f64()?;
            let autostart = r.bool()?;
            let idx = r.u32()? as usize;
            let level = r.bool()?;
            let name = String::from_utf8_lossy(r.bytes()?).into_owned();
            let mut t = TapePlayer::new(edges, name);
            t.started_at = has_start.then_some(st);
            t.autostart = autostart;
            t.idx = idx.min(t.edges.len());
            t.level = level;
            b.player = Some(t);
        }
        let info = StateInfo { title, pc: m.cpu.pc, cycles: m.board.cycles, ram_kb, color_card: m.board.config.color_card, tape };
        Ok((m, info))
    }
}

/// Read only the header of a state file (to show what it contains).
pub fn peek_info(bytes: &[u8]) -> Result<StateInfo, String> {
    Machine::load_state(bytes).map(|(_, i)| i)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_hello() -> Machine {
        let mut m = Machine::new(Config { display_throttle: false, ..Config::default() });
        m.run_until_idle(2_000_000);
        m.type_text("300: A2 00 BD 10 03 F0 06 20 EF FF E8 D0 F5 4C 1F FF\n");
        m.type_text("310: C8 C9 8D 00\n300R\n");
        m.run_until_idle(5_000_000);
        m
    }

    /// Two machines must behave identically from the same point.
    fn same(a: &Machine, b: &Machine) {
        assert_eq!((a.cpu.a, a.cpu.x, a.cpu.y, a.cpu.sp, a.cpu.pc, a.cpu.p), (b.cpu.a, b.cpu.x, b.cpu.y, b.cpu.sp, b.cpu.pc, b.cpu.p));
        assert_eq!(a.board.cycles, b.board.cycles);
        assert_eq!(a.board.mem, b.board.mem);
        assert_eq!(a.screen_text(), b.screen_text());
        assert_eq!((a.board.display.row, a.board.display.col), (b.board.display.row, b.board.display.col));
        assert_eq!(a.board.gen2, b.board.gen2);
    }

    #[test]
    fn round_trip_is_exact_and_continues_identically() {
        let mut m = run_hello();
        let bytes = m.save_state("hello");
        let (mut r, info) = Machine::load_state(&bytes).unwrap();
        assert_eq!(info.title, "hello");
        assert_eq!(info.pc, m.cpu.pc);
        same(&m, &r);
        // keep running both with the same input: they must stay identical
        for mm in [&mut m, &mut r] {
            mm.type_text("FF00.FF0F\n");
            mm.run_until_idle(5_000_000);
        }
        same(&m, &r);
        assert!(r.screen_text().contains("FF00: D8 58"));
    }

    #[test]
    fn mid_instruction_stream_with_keys_pending_and_gen2() {
        let cfg = Config { color_card: true, display_throttle: true, ..Config::default() };
        let mut m = Machine::new(cfg);
        m.run_cycles(300_000);
        m.type_text("C250\nC257\nHELLO\n");
        m.run_cycles(20_001); // stop with keys still queued
        assert!(!m.board.key_queue.is_empty() || m.board.pia.key_available());
        let (mut r, info) = Machine::load_state(&m.save_state("")).unwrap();
        assert!(info.color_card);
        same(&m, &r);
        m.run_cycles(500_000);
        r.run_cycles(500_000);
        same(&m, &r);
        // the frame logs (HST0 splits) survive too
        let t = m.board.config.timing.cycles_to_ticks(m.board.cycles);
        assert_eq!(m.board.gen2.frame_lines(t), r.board.gen2.frame_lines(t));
    }

    #[test]
    fn tape_in_player_is_kept() {
        let mut m = run_hello();
        m.board.player = Some(TapePlayer::new(vec![0.1, 0.2, 0.35, 0.5], "tape"));
        let (r, info) = Machine::load_state(&m.save_state("t")).unwrap();
        assert!(info.tape);
        let p = r.board.player.unwrap();
        assert_eq!((p.edges.len(), p.name.as_str()), (4, "tape"));
    }

    #[test]
    fn default_roms_are_not_stored_and_garbage_is_refused() {
        let m = run_hello();
        let b = m.save_state("x");
        assert!(b.len() < 20_000, "{}", b.len());
        assert!(!b.windows(8).any(|w| w == &WOZ_MONITOR[..8]));
        assert!(Machine::load_state(b"nope").is_err());
        assert!(Machine::load_state(&b[..b.len() / 2]).is_err());
        let mut bad = b.clone();
        bad[4] = 99;
        assert!(Machine::load_state(&bad).err().unwrap().contains("version"));
    }
}
