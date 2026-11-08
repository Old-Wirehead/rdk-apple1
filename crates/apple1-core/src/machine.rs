//! The Apple-1 board: memory map, PIA, display, ACI and the CPU.
//!
//! Memory map (default configuration):
//!
//! | Range         | Device                                             |
//! |---------------|----------------------------------------------------|
//! | `$0000-$1FFF` | RAM (8 KB, configurable 4..48 KB from $0000)       |
//! | `$C000-$C0FF` | ACI I/O: any access toggles the tape output,       |
//! |               | odd reads in `$C081-$C0FF` sense the tape input    |
//! | `$C100-$C1FF` | ACI ROM                                            |
//! | `$C2xx-$C7xx` | optional GEN2 colour card soft switches ($C250-$C257, |
//! |               | see `gen2`); the card then also provides $0000-$BFFF |
//! | `$D010-$D013` | PIA 6821: KBD, KBDCR, DSP, DSPCR (mirrored in Dxxx |
//! |               | wherever A4=1)                                     |
//! | `$E000-$EFFF` | RAM (4 KB, for Integer BASIC; can be disabled)     |
//! | `$FF00-$FFFF` | Woz Monitor ROM                                    |

use std::collections::VecDeque;

use crate::cpu::{Bus, Cpu};
use crate::display::Display;
use crate::gen2::{self, Gen2};
use crate::pia::{Pia, PiaEvent};
use crate::roms::{ACI_ROM, WOZ_MONITOR};
use crate::tape::TapePlayer;
use crate::tms9918::Vdp;
use crate::timing::Timing;

#[derive(Debug, Clone)]
pub struct Config {
    /// Contiguous RAM from $0000, in KB (4..=48).
    pub ram_kb: u32,
    /// 4 KB of RAM at $E000-$EFFF (needed for Integer BASIC).
    pub ram_e000: bool,
    /// ACI card installed.
    pub aci: bool,
    /// Limit the display to ~60 characters per second like the real terminal.
    pub display_throttle: bool,
    pub timing: Timing,
    /// Woz Monitor PROM image ($FF00-$FFFF, PROMs A1/A2).
    pub monitor_rom: [u8; 256],
    /// ACI PROM image ($C100-$C1FF and $C000-$C0FF, PROMs A3/A4).
    pub aci_rom: [u8; 256],
    /// Uncle Bernie's GEN2 colour graphics card (implies RAM $0000-$BFFF).
    pub color_card: bool,
    /// 50 Hz jumper of the colour card (312 lines instead of 262).
    pub color_50hz: bool,
    /// P-LAB graphic card (TMS9918A video processor at $CC00-$CFFF).
    pub plab_card: bool,
    /// P-LAB card IRQ/NMI jumper: true = NMI (default: IRQ).
    pub plab_nmi: bool,
    /// P-LAB "CodeTank" EEPROM image mapped read-only at $4000-$7FFF (16 KB).
    pub codetank: Option<Vec<u8>>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ram_kb: 8,
            ram_e000: true,
            aci: true,
            display_throttle: true,
            timing: Timing::default(),
            monitor_rom: *WOZ_MONITOR,
            aci_rom: *ACI_ROM,
            color_card: false,
            color_50hz: false,
            plab_card: false,
            plab_nmi: false,
            codetank: None,
        }
    }
}

/// Marker in the key queue (real keys have 7 bits): a random pause, see `Machine::type_pause`.
pub const KEY_PAUSE: u8 = 0x80;

/// Everything on the bus except the CPU.
pub struct Board {
    pub mem: Vec<u8>,
    pub(crate) ram_page: [bool; 256],
    pub pia: Pia,
    pub display: Display,
    pub config: Config,
    /// Bus cycles elapsed (== CPU cycles).
    pub cycles: u64,
    pub(crate) display_busy_until: u64,
    /// ACI output flip-flop level.
    pub tape_out: bool,
    /// Recording of the ACI output: cycle stamps of every toggle.
    pub recording: Option<Vec<u64>>,
    /// Tape currently in the "player".
    pub player: Option<TapePlayer>,
    /// Number of accesses to the ACI I/O space (diagnostics).
    pub aci_accesses: u64,
    pub key_queue: VecDeque<u8>,
    pub(crate) next_key_cycle: u64,
    /// State of the GEN2 colour card (used only when `config.color_card`).
    pub gen2: Gen2,
    /// State of the P-LAB graphic card (used only when `config.plab_card`).
    pub vdp: Vdp,
    pub(crate) vdp_irq: bool,
    pub(crate) vdp_nmi: bool,
    pub(crate) vdp_int_prev: bool,
}

impl Board {
    pub fn new(config: Config) -> Self {
        let mut mem = vec![0u8; 65536];
        mem[0xFF00..].copy_from_slice(&config.monitor_rom);
        mem[0xC100..0xC200].copy_from_slice(&config.aci_rom);
        let mut b = Board {
            mem,
            ram_page: [false; 256],
            pia: Pia::new(),
            display: Display::new(),
            display_busy_until: 0,
            config: config.clone(),
            cycles: 0,
            tape_out: false,
            recording: None,
            player: None,
            aci_accesses: 0,
            key_queue: VecDeque::new(),
            next_key_cycle: 0,
            gen2: Gen2::new(config.color_50hz),
            vdp: Vdp::new(),
            vdp_irq: false,
            vdp_nmi: false,
            vdp_int_prev: false,
        };
        b.apply_config(config);
        if b.config.color_card {
            // random HIRES primary page at power-up
            let mut x = 0xC0FF_EE42u32;
            for b in &mut b.mem[0x2000..=0x3FFF] {
                x ^= x << 13;
                x ^= x >> 17;
                x ^= x << 5;
                *b = x as u8;
            }
        }
        b
    }

    pub fn apply_config(&mut self, config: Config) {
        // The GEN2 card is also a RAM expansion: $0000-$BFFF.
        let kb = if config.color_card { 48 } else { config.ram_kb.clamp(4, 48) as usize };
        self.gen2.hz50 = config.color_50hz;
        self.vdp.nmi_jumper = config.plab_nmi;
        let mut pages = [false; 256];
        for p in pages.iter_mut().take(kb * 4) {
            *p = true;
        }
        if config.ram_e000 {
            // $E000-$FEFF: Integer BASIC (4 KB) or RDK G2 BASIC (up to 7.75 KB)
            for p in pages.iter_mut().take(0xFF).skip(0xE0) {
                *p = true;
            }
        }
        self.ram_page = pages;
        self.config = config;
    }

    /// Make every address that is not ROM or I/O behave like RAM (used by
    /// the headless tape encoder and tests).
    pub fn all_ram(&mut self) {
        for p in 0..256usize {
            self.ram_page[p] = !matches!(p, 0xC0 | 0xC1 | 0xD0 | 0xFF);
        }
    }

    pub fn is_ram(&self, addr: u16) -> bool {
        self.ram_page[(addr >> 8) as usize]
    }

    pub fn now_secs(&self) -> f64 {
        self.config.timing.cycles_to_secs(self.cycles)
    }

    /// Side effect free read (for debuggers, dumps, savers).
    pub fn peek(&self, addr: u16) -> u8 {
        let a = addr as usize;
        if a & 0xC000 == 0x4000 && self.config.codetank.is_some() {
            return self.codetank_byte(a - 0x4000);
        }
        if self.ram_page[a >> 8] || (0xFF00..=0xFFFF).contains(&a) || (self.config.aci && (0xC100..0xC200).contains(&a)) {
            self.mem[a]
        } else if (0xD000..0xE000).contains(&a) && a & 0x10 != 0 {
            self.pia.peek(a as u8)
        } else {
            0xFF
        }
    }

    /// Write to RAM only. Returns false if the address is not RAM.
    pub fn poke(&mut self, addr: u16, v: u8) -> bool {
        if self.ram_page[(addr >> 8) as usize] && !(addr & 0xC000 == 0x4000 && self.config.codetank.is_some()) {
            self.mem[addr as usize] = v;
            true
        } else {
            false
        }
    }

    fn codetank_byte(&self, off: usize) -> u8 {
        self.config.codetank.as_ref().and_then(|c| c.get(off % 16384).copied()).unwrap_or(0xFF)
    }

    /// Recompute the IRQ/NMI request of the P-LAB card from the VDP state.
    /// IRQ is level triggered; for NMI an edge is generated when the
    /// interrupt condition becomes true.
    /// After `apply_config`: release the interrupt lines if the card was removed.
    pub fn vdp_changed_config(&mut self) {
        self.vdp_update_irq();
        if !self.config.plab_card {
            self.vdp_irq = false;
        }
    }

    pub(crate) fn vdp_update_irq(&mut self) {
        let lvl = self.config.plab_card && self.vdp.int_level();
        if self.config.plab_nmi {
            if lvl && !self.vdp_int_prev {
                self.vdp_nmi = true;
            }
        } else {
            self.vdp_irq = lvl;
        }
        self.vdp_int_prev = lvl;
    }

    pub(crate) fn vdp_advance(&mut self) {
        if self.config.plab_card && self.vdp.advance(self.cycles) {
            self.vdp_update_irq();
        }
    }

    fn aci_access(&mut self) {
        self.aci_accesses += 1;
        self.tape_out = !self.tape_out;
        if let Some(r) = self.recording.as_mut() {
            r.push(self.cycles);
        }
    }

    fn tape_in_level(&mut self) -> bool {
        let t = self.now_secs();
        match self.player.as_mut() {
            Some(p) => p.level_at(t),
            None => false,
        }
    }

    /// Feed queued keys into the PIA when the previous one has been read.
    pub fn service_keyboard(&mut self) {
        if !self.pia.key_available() && self.cycles >= self.next_key_cycle {
            if let Some(k) = self.key_queue.pop_front() {
                if k == KEY_PAUSE {
                    // a human takes a variable time to type: 0.1 to 1.5 s of
                    // machine time, so BASIC's RND seed (stirred while it waits
                    // for a key) differs from one launch to the next
                    let n = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.subsec_nanos()).unwrap_or(12345) as u64;
                    self.next_key_cycle = self.cycles + 100_000 + (n.wrapping_mul(2654435761) >> 7) % 1_400_000;
                    return;
                }
                self.pia.key_strobe(k);
                // small gap between typed keys (like a fast typist)
                self.next_key_cycle = self.cycles + 2_000;
            }
        }
    }

    /// CPU cycle at which the video hardware will have consumed a character
    /// written now (beam passes the cursor position, + 1 CHAR period of /RDA).
    fn display_ready_cycle(&self) -> u64 {
        let t = &self.config.timing;
        let now = t.cycles_to_ticks(self.cycles);
        let pos = video::cursor_tick_in_frame(self.display.row, self.display.col);
        let phase = now % video::FRAME_TICKS;
        let wait = if pos > phase { pos - phase } else { video::FRAME_TICKS - phase + pos };
        t.ticks_to_cycles(now + wait + 1).max(self.cycles + 1)
    }

    fn update_display_busy(&mut self) {
        self.pia.pb7_busy = self.config.display_throttle && self.cycles < self.display_busy_until;
    }
}

impl Bus for Board {
    fn read(&mut self, addr: u16) -> u8 {
        self.cycles += 1;
        let a = addr as usize;
        if a & 0xC000 == 0x4000 && self.config.codetank.is_some() {
            // the CodeTank EEPROM replaces any RAM at $4000-$7FFF
            return self.codetank_byte(a - 0x4000);
        }
        if self.ram_page[a >> 8] {
            return self.mem[a];
        }
        match a {
            0xC000..=0xC0FF if self.config.aci => {
                self.aci_access();
                let mut off = a & 0xFF;
                // Tape input gates PROM address line A0 for $C080-$C0FF.
                if off & 0x80 != 0 && self.tape_in_level() {
                    off &= !1;
                }
                self.config.aci_rom[off]
            }
            0xC100..=0xC1FF if self.config.aci => self.mem[a],
            0xCC00..=0xCFFF if self.config.plab_card => {
                if a & 1 == 0 {
                    self.vdp.read_data()
                } else {
                    let s = self.vdp.read_status();
                    self.vdp_update_irq();
                    s
                }
            }
            0xC200..=0xC7FF if self.config.color_card && gen2::decode(addr).is_some() => {
                let ticks = self.config.timing.cycles_to_ticks(self.cycles);
                self.gen2.read(gen2::decode(addr).unwrap_or(0), ticks)
            }
            0xD000..=0xDFFF if a & 0x10 != 0 => {
                if a & 3 == 2 {
                    self.update_display_busy();
                }
                self.pia.read(a as u8)
            }
            0xFF00..=0xFFFF => self.mem[a],
            _ => 0xFF,
        }
    }

    fn write(&mut self, addr: u16, v: u8) {
        self.cycles += 1;
        let a = addr as usize;
        if a & 0xC000 == 0x4000 && self.config.codetank.is_some() {
            return; // EEPROM: read-only from the bus
        }
        if self.ram_page[a >> 8] {
            self.mem[a] = v;
            return;
        }
        match a {
            0xC000..=0xC0FF if self.config.aci => self.aci_access(),
            0xCC00..=0xCFFF if self.config.plab_card => {
                if a & 1 == 0 {
                    self.vdp.write_data(v);
                } else {
                    self.vdp.write_control(v);
                    self.vdp_update_irq();
                }
            }
            0xD000..=0xDFFF if a & 0x10 != 0 => {
                if let PiaEvent::DisplayWrite(c) = self.pia.write(a as u8, v) {
                    // The terminal accepts the character when the beam
                    // reaches the cursor position: at most one per frame.
                    self.display_busy_until = self.display_ready_cycle();
                    self.display.put(c);
                    self.update_display_busy();
                }
            }
            _ => {}
        }
    }
}

/// Video timing: one scan
/// line = 65 CHAR clocks (40 visible, H counter $C0-$F9), one frame = 262
/// lines (60.05 Hz), 24 text rows of 8 lines.
pub mod video {
    pub const LINE_TICKS: u64 = 65;
    pub const LINES: u64 = 262;
    pub const FRAME_TICKS: u64 = LINE_TICKS * LINES;
    /// Offset of the first visible character inside a line.
    pub const FIRST_VISIBLE: u64 = 25;
    /// Position (in CHAR clocks from the start of the frame) at which the
    /// beam scans the given text cell.
    pub fn cursor_tick_in_frame(row: usize, col: usize) -> u64 {
        (row as u64 * 8) * LINE_TICKS + FIRST_VISIBLE + col as u64
    }
}

/// A complete Apple-1.
pub struct Machine {
    pub cpu: Cpu,
    pub board: Board,
}

impl Machine {
    pub fn new(config: Config) -> Self {
        let mut m = Machine { cpu: Cpu::new(), board: Board::new(config) };
        m.reset();
        m
    }

    /// The RESET button: resets the 6502 and the PIA (display and GEN2 soft switches untouched).
    pub fn reset(&mut self) {
        self.board.pia.reset();
        self.board.key_queue.clear();
        self.cpu.reset(&mut self.board);
    }

    /// The CLEAR SCREEN button.
    pub fn clear_screen(&mut self) {
        self.board.display.clear();
    }

    pub fn step(&mut self) -> u32 {
        self.board.service_keyboard();
        if self.cpu.jammed {
            // a jammed 6502 (KIL opcode) no longer accesses the bus: keep
            // the board clock running so `run_cycles` cannot hang
            self.board.cycles += 1;
        }
        if self.board.vdp_nmi {
            self.board.vdp_nmi = false;
            self.cpu.nmi();
        }
        self.cpu.set_irq(self.board.vdp_irq);
        let c = self.cpu.step(&mut self.board);
        self.board.vdp_advance();
        c
    }

    /// Run for at least `cycles` CPU cycles.
    pub fn run_cycles(&mut self, cycles: u64) {
        let end = self.board.cycles + cycles;
        while self.board.cycles < end {
            self.step();
        }
    }

    /// Queue keystrokes. Lower case is converted to upper case, `\n` to CR.
    pub fn type_text(&mut self, text: &str) {
        for ch in text.chars() {
            if let Some(k) = map_key(ch) {
                self.board.key_queue.push_back(k);
            }
        }
    }

    /// Queue a pause of random length between two keystrokes (see `KEY_PAUSE`).
    pub fn type_pause(&mut self) {
        self.board.key_queue.push_back(KEY_PAUSE);
    }

    pub fn press_key(&mut self, code: u8) {
        self.board.key_queue.push_back(code & 0x7F);
    }

    /// Run until the keyboard queue is empty and the CPU waits for a key
    /// (or `max_cycles` elapse). Handy for scripted sessions and tests.
    pub fn run_until_idle(&mut self, max_cycles: u64) -> bool {
        let end = self.board.cycles + max_cycles;
        let mut idle_since: Option<u64> = None;
        while self.board.cycles < end {
            self.step();
            let waiting = self.board.key_queue.is_empty() && !self.board.pia.key_available();
            if waiting {
                // consider idle when the CPU spins in a KBDCR polling loop
                let s = *idle_since.get_or_insert(self.board.cycles);
                if self.board.cycles - s > 50_000 && self.in_key_wait_loop() {
                    return true;
                }
            } else {
                idle_since = None;
            }
        }
        false
    }

    fn in_key_wait_loop(&self) -> bool {
        // Woz monitor ($FF29), ACI ($C10D) key loops, or Integer BASIC's
        let pc = self.cpu.pc;
        (0xFF2B..=0xFF2F).contains(&pc) || (0xC10D..=0xC111).contains(&pc) || (0xE000..=0xFEFF).contains(&pc)
    }

    /// Load bytes into RAM. Returns the number of bytes that could not be
    /// stored because the target is not RAM.
    pub fn load(&mut self, addr: u16, data: &[u8]) -> usize {
        let mut skipped = 0;
        for (i, &b) in data.iter().enumerate() {
            if !self.board.poke(addr.wrapping_add(i as u16), b) {
                skipped += 1;
            }
        }
        skipped
    }

    /// Read a memory range (inclusive) without side effects.
    pub fn dump(&self, start: u16, end: u16) -> Vec<u8> {
        (start as u32..=end as u32).map(|a| self.board.peek(a as u16)).collect()
    }

    pub fn screen_text(&self) -> String {
        self.board.display.text()
    }
}

/// Map a host character to an Apple-1 keyboard code (7-bit).
pub fn map_key(ch: char) -> Option<u8> {
    match ch {
        '\n' | '\r' => Some(0x0D),
        '\x1b' => Some(0x1B),
        '\x08' | '\x7f' => Some(b'_'), // rubout used by the Woz monitor
        c if c.is_ascii() => {
            let b = (c as u8).to_ascii_uppercase();
            if b < 0x80 {
                Some(b)
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn type_pause_delays_the_next_key() {
        let mut m = Machine::new(Config::default());
        m.type_pause();
        m.type_text("A");
        m.run_cycles(60_000);
        assert!(!m.board.key_queue.is_empty() || !m.board.pia.key_available(), "key arrived before the pause");
        m.run_cycles(1_600_000);
        assert!(m.board.key_queue.is_empty());
    }

    use super::*;

    #[test]
    fn gen2_card_soft_switches_and_hst0() {
        let cfg = Config { color_card: true, ..Config::default() };
        let mut m = Machine::new(cfg);
        assert!(m.board.is_ram(0xBFFF) && m.board.is_ram(0x4000));
        // LDA $C250 ; LDA $C257 ; STA $C251 (write: ignored) ;
        // wait for VBLANK like Listing 1: LDA $C254 / ORA $C254 / BPL loop
        // then count HST0=1 samples, BRK-free loop end at $0320
        let prog = [
            0xAD, 0x50, 0xC2, 0xAD, 0x57, 0xC2, 0x8D, 0x51, 0xC2, // $300
            0xAD, 0x54, 0xC2, 0x0D, 0x54, 0xC2, 0x10, 0xF8, // $309 wait blank
            0x4C, 0x11, 0x03, // $311 JMP *
        ];
        m.load(0x300, &prog);
        m.cpu.pc = 0x300;
        m.run_cycles(40_000);
        assert_eq!(m.cpu.pc, 0x311);
        let g = m.board.gen2.clone();
        assert!(!g.text && g.hires && !g.page2, "{g:?}");
        // a read in live scan has D7 = 0, in VBLANK D7 = 1
        let t = m.board.config.timing;
        let b = &mut m.board;
        b.cycles = t.ticks_to_cycles(100 * 65 + 40);
        assert_eq!(b.read(0xC254) & 0x80, 0);
        b.cycles = t.ticks_to_cycles(230 * 65 + 40);
        assert_eq!(b.read(0xC254) & 0x80, 0x80);
        // without the card $C250 is nothing
        let mut m2 = Machine::new(Config::default());
        assert_eq!(m2.board.read(0xC250), 0xFF);
    }

    #[test]
    fn woz_monitor_boots_and_echoes() {
        let mut m = Machine::new(Config { display_throttle: false, ..Config::default() });
        assert!(m.run_until_idle(2_000_000));
        assert!(m.screen_text().starts_with('\\'), "screen: {:?}", m.screen_text());
        m.type_text("FF00.FF07\n");
        assert!(m.run_until_idle(5_000_000));
        let t = m.screen_text();
        assert!(t.contains("FF00: D8 58 A0 7F 8C 12 D0 A9"), "screen: {t}");
    }

    #[test]
    fn deposit_and_run_hello() {
        let mut m = Machine::new(Config { display_throttle: false, ..Config::default() });
        m.run_until_idle(2_000_000);
        // 0300: LDX #0 ; LDA $0310,X ; BEQ done ; JSR ECHO ; INX ; BNE loop ; done: JMP $FF1F
        m.type_text("300: A2 00 BD 10 03 F0 06 20 EF FF E8 D0 F5 4C 1F FF\n");
        m.type_text("310: C8 C9 8D 00\n300R\n");
        assert!(m.run_until_idle(5_000_000));
        assert!(m.screen_text().contains("HI"), "{}", m.screen_text());
    }

    #[test]
    fn lowercase_is_uppercased() {
        assert_eq!(map_key('a'), Some(b'A'));
        assert_eq!(map_key('\n'), Some(0x0D));
    }

    #[test]
    fn display_throttle_limits_speed() {
        let mut m = Machine::new(Config::default());
        m.run_until_idle(2_000_000);
        m.type_text("FF00.FF3F\n");
        let c0 = m.board.cycles;
        m.run_until_idle(50_000_000);
        let secs = m.board.config.timing.cycles_to_secs(m.board.cycles - c0);
        // 8 lines of ~31 chars ≈ 250 chars at 60 cps ≈ 4 s
        assert!(secs > 3.0 && secs < 6.0, "{secs}");
    }
}
