//! The emulated Apple-1 and its run loop (full speed, breakpoints, idle
//! detection).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use apple1_core::debug::Symbols;
use apple1_core::program::{self, Kind, Program};
use apple1_core::{Config, Machine};

/// Apple-1 clock (Hz), used to convert milliseconds to cycles.
pub const CLOCK: f64 = 1_022_727.0;

/// Why a run stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stop {
    /// Keyboard queue empty and the CPU spins in a tiny loop (key wait).
    Idle,
    /// The cycle budget ran out.
    Budget,
    Break(u16),
    Jam,
}

impl Stop {
    pub fn text(self) -> String {
        match self {
            Stop::Idle => "idle (waiting for a key)".into(),
            Stop::Budget => "cycle budget reached (machine still running)".into(),
            Stop::Break(a) => format!("breakpoint at ${a:04X}"),
            Stop::Jam => "CPU jammed (KIL opcode)".into(),
        }
    }
}

pub struct Session {
    pub m: Machine,
    pub breaks: BTreeSet<u16>,
    pub syms: Symbols,
    /// Default Integer BASIC image (command line / APPLE1_BASIC).
    pub basic_path: Option<PathBuf>,
    pub basic_loaded: bool,
}

pub fn config(ram_kb: u32, color_card: bool) -> Config {
    Config { ram_kb, color_card, display_throttle: false, ..Config::default() }
}

impl Session {
    pub fn new(basic_path: Option<PathBuf>) -> Session {
        let mut s = Session { m: Machine::new(config(8, false)), breaks: BTreeSet::new(), syms: Symbols::new(), basic_path, basic_loaded: false };
        if let Some(p) = s.basic_path.clone() {
            if let Err(e) = s.load_basic(&p) {
                eprintln!("apple1-mcp: BASIC not loaded: {e}");
            }
        }
        s.run(2_000_000, true);
        s
    }

    /// Power cycle with a new configuration.
    pub fn power_on(&mut self, cfg: Config) {
        self.m = Machine::new(cfg);
        self.basic_loaded = false;
    }

    /// Load Integer BASIC at $E000.
    pub fn load_basic(&mut self, path: &Path) -> Result<usize, String> {
        if !self.m.board.is_ram(0xE000) {
            return Err("no RAM at $E000".into());
        }
        let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let kind = program::kind_of(path, Some(&bytes));
        let p = program::load_bytes(&bytes, kind, if kind == Kind::Hex { None } else { Some(0xE000) })?;
        self.load_program(&p);
        if !apple1_core::debug::basic_present(&|a| self.m.board.peek(a)) {
            return Err(format!("{}: this is not Integer BASIC (hook bytes at $E86B not found)", path.display()));
        }
        self.basic_loaded = true;
        Ok(p.total_len())
    }

    pub fn basic_present(&self) -> bool {
        apple1_core::debug::basic_present(&|a| self.m.board.peek(a))
    }

    /// Put the blocks in memory; returns the bytes that hit ROM/unmapped space.
    pub fn load_program(&mut self, p: &Program) -> usize {
        p.blocks.iter().map(|(a, d)| self.m.load(*a, d)).sum()
    }

    /// Run at full speed for up to `max` cycles. With `until_idle`, stop
    /// as soon as the keyboard queue is empty and the CPU has stayed in a
    /// loop of at most 16 bytes for 30 000 cycles (a key wait loop).
    /// Breakpoints are checked before each instruction except the first.
    pub fn run(&mut self, max: u64, until_idle: bool) -> Stop {
        let m = &mut self.m;
        let end = m.board.cycles + max;
        let mut first = true;
        let (mut lo, mut hi, mut since) = (m.cpu.pc, m.cpu.pc, m.board.cycles);
        while m.board.cycles < end {
            let pc = m.cpu.pc;
            if !first && self.breaks.contains(&pc) {
                return Stop::Break(pc);
            }
            first = false;
            m.step();
            if m.cpu.jammed {
                return Stop::Jam;
            }
            if until_idle {
                let pc = m.cpu.pc;
                let waiting = m.board.key_queue.is_empty() && !m.board.pia.key_available();
                lo = lo.min(pc);
                hi = hi.max(pc);
                if !waiting || hi - lo > 16 {
                    lo = pc;
                    hi = pc;
                    since = m.board.cycles;
                } else if m.board.cycles - since > 30_000 {
                    return Stop::Idle;
                }
            }
        }
        Stop::Budget
    }

    pub fn registers(&self) -> String {
        let c = &self.m.cpu;
        let f: String = "NV-BDIZC".chars().enumerate().map(|(i, ch)| if c.p & (0x80 >> i) != 0 { ch } else { '.' }).collect();
        format!("PC=${:04X} A=${:02X} X=${:02X} Y=${:02X} SP=${:02X} P=${:02X} [{f}] cycles={}", c.pc, c.a, c.x, c.y, c.sp, c.p, self.m.board.cycles)
    }

    pub fn screen(&self) -> String {
        self.m.screen_text()
    }
}
