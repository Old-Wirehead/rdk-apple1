//! Uncle Bernie's GEN2 colour graphics card for the Apple-1 (optional).
//!
//! What the emulator does:
//! - Apple II video subsystem: TEXT 40x24, LORES 40x48 (16 colours), HIRES
//!   280x192 (6 colours), MIXED (last 4 text rows), two pages
//!   (TEXT/LORES $0400/$0800, HIRES $2000/$4000).
//! - Soft switches at $C250-$C257 (the Apple II $C050-$C057, moved to $C2xx
//!   so that $C0xx/$C1xx stay free for the ACI). Read only: a read flips the
//!   switch, a write is ignored.
//! - Address decoder `SEL = $Cxxx & !A11 & A9 & A4`: mirrors every 8 bytes
//!   in $C2xx/$C3xx/$C6xx/$C7xx where A4 = 1.
//! - D7 of every read = HST0 (1 = horizontal or vertical blank, 0 = live
//!   scan or the 3-cycle colour burst, see `hst0_state`); D6..D0 = floating
//!   bus, returned as random bits.
//! - 65 cycles per line, 262 lines (60 Hz) or 312 lines (50 Hz, jumper).
//! - RESET does not touch the switches. At power-up the emulator starts in
//!   HIRES, primary page, no mix, TEXT off, with random contents in
//!   $2000-$3FFF.
//! - The card is also the RAM expansion: $0000-$BFFF (54 KB with $E000).
//!
//! Beam racing: mode changes are logged with the scan line where they occur
//! (`frame_lines`), so vertical splits timed with HST0 (TEST4) show up. The
//! switches are sampled once per line here; MIXED is applied on lines
//! 160-191. Not emulated: mode changes in the middle of a line, memory
//! writes racing the beam, the free-running phase between the motherboard
//! and the card scanners (assumed 0). The 4 refresh cycles out of 65 are
//! modelled through `Timing`.

/// Lines 0-191 are visible.
pub const VISIBLE_LINES: u32 = 192;
pub const LINE_CYCLES: u64 = 65;

/// The four soft switches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Modes {
    pub text: bool,
    pub mixed: bool,
    pub page2: bool,
    pub hires: bool,
}

/// Mode changes logged during one frame: (first visible line affected, modes).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FrameLog {
    pub frame: u64,
    pub start: Modes,
    pub changes: Vec<(u32, Modes)>,
}

const MAX_CHANGES: usize = 4096;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gen2 {
    /// SS_TEXT: true = TEXT mode (all other modes disabled).
    pub text: bool,
    /// SS_MIX: last four text rows in graphics mode.
    pub mixed: bool,
    /// SS_PAGE: secondary page.
    pub page2: bool,
    /// SS_HIRES: HIRES instead of LORES (when TEXT is off).
    pub hires: bool,
    /// 50 Hz jumper (312 lines instead of 262).
    pub hz50: bool,
    pub(crate) rng: u32,
    /// Number of soft switch reads (diagnostics).
    pub accesses: u64,
    pub(crate) cur: FrameLog,
    pub(crate) prev: Option<FrameLog>,
}

impl Default for Gen2 {
    fn default() -> Self {
        Gen2::new(false)
    }
}

/// Decoded soft switch (0..8) for an address, `None` if the card ignores it.
pub fn decode(addr: u16) -> Option<u8> {
    let sel = addr & 0xF000 == 0xC000 && addr & 0x0800 == 0 && addr & 0x0200 != 0 && addr & 0x0010 != 0;
    sel.then_some((addr & 7) as u8)
}

/// HST0 seen by the 6502 at scan line `line`, cycle `hcnt` of the line
/// (true during the horizontal and vertical blanks, false during the
/// visible part and the 3-cycle colour burst).
pub fn hst0_state(line: u32, hcnt: u32) -> bool {
    if hcnt > 12 && hcnt < 16 {
        return false; // colour burst
    }
    if line > 191 {
        return true; // VBLANK
    }
    hcnt <= 24 // HBLANK (25..64 = live scan)
}

impl Gen2 {
    pub fn new(hz50: bool) -> Self {
        // power-up state chosen by the emulator: HIRES, primary page, TEXT off
        let mut g = Gen2 { text: false, mixed: false, page2: false, hires: true, hz50, rng: 0x1234_5678, accesses: 0, cur: FrameLog::default(), prev: None };
        g.cur.start = g.modes();
        g
    }

    pub fn modes(&self) -> Modes {
        Modes { text: self.text, mixed: self.mixed, page2: self.page2, hires: self.hires }
    }

    /// Modes of each of the 192 visible lines of the last complete frame
    /// at master tick `now` (all equal when nothing raced the beam).
    pub fn frame_lines(&self, now: u64) -> [Modes; VISIBLE_LINES as usize] {
        let frame = now / self.frame_ticks();
        let log = if self.cur.frame + 1 == frame {
            Some(&self.cur)
        } else if self.cur.frame == frame {
            self.prev.as_ref().filter(|p| p.frame + 1 == frame)
        } else {
            None
        };
        let mut out = [self.modes(); VISIBLE_LINES as usize];
        if let Some(log) = log {
            let mut m = log.start;
            let mut it = log.changes.iter().peekable();
            for (y, slot) in out.iter_mut().enumerate() {
                while let Some(&&(line, nm)) = it.peek() {
                    if line as usize > y {
                        break;
                    }
                    m = nm;
                    it.next();
                }
                *slot = m;
            }
        }
        out
    }

    fn log_change(&mut self, ticks: u64) {
        let frame = ticks / self.frame_ticks();
        if frame != self.cur.frame {
            let start = self.modes();
            let old = std::mem::replace(&mut self.cur, FrameLog { frame, start, changes: Vec::new() });
            self.prev = Some(old);
        }
    }

    pub fn lines(&self) -> u64 {
        if self.hz50 {
            312
        } else {
            262
        }
    }

    pub fn frame_ticks(&self) -> u64 {
        LINE_CYCLES * self.lines()
    }

    /// (line, hcnt) of the card's scanner at master clock tick `ticks`.
    pub fn beam(&self, ticks: u64) -> (u32, u32) {
        let f = ticks % self.frame_ticks();
        ((f / LINE_CYCLES) as u32, (f % LINE_CYCLES) as u32)
    }

    /// Apply soft switch `n` (0..8), as a read does.
    pub fn flip(&mut self, n: u8) {
        let on = n & 1 != 0;
        match n >> 1 {
            0 => self.text = on,
            1 => self.mixed = on,
            2 => self.page2 = on,
            _ => self.hires = on,
        }
    }

    /// CPU read of a decoded address at master tick `ticks`.
    pub fn read(&mut self, n: u8, ticks: u64) -> u8 {
        self.accesses += 1;
        self.log_change(ticks);
        let before = self.modes();
        self.flip(n);
        if self.modes() != before && self.cur.changes.len() < MAX_CHANGES {
            // a change during the live part of a line counts from the next one
            let (line, hcnt) = self.beam(ticks);
            let first = if hcnt <= 24 { line } else { line + 1 };
            self.cur.changes.push((first, self.modes()));
        }
        // xorshift32: the floating bus is "random garbage"
        let mut x = self.rng;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.rng = x;
        let (line, hcnt) = self.beam(ticks);
        ((hst0_state(line, hcnt) as u8) << 7) | (x as u8 & 0x7F)
    }

    /// Mode name, in French, for the status bar.
    pub fn mode_name(&self) -> String {
        let p = if self.page2 { 2 } else { 1 };
        if self.text {
            format!("TEXTE page {p}")
        } else {
            let g = if self.hires { "HIRES" } else { "LORES" };
            if self.mixed {
                format!("{g} + texte page {p}")
            } else {
                format!("{g} page {p}")
            }
        }
    }
}

/// Address of text (and LORES) row `row` (0..24) of a page.
pub fn text_row_addr(page2: bool, row: usize) -> usize {
    let base = if page2 { 0x800 } else { 0x400 };
    base + (row & 7) * 0x80 + (row >> 3) * 0x28
}

/// Address of HIRES line `y` (0..192) of a page.
pub fn hires_line_addr(page2: bool, y: usize) -> usize {
    let base = if page2 { 0x4000 } else { 0x2000 };
    base + (y & 7) * 0x400 + ((y >> 3) & 7) * 0x80 + (y >> 6) * 0x28
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decoder_equation() {
        for n in 0..8u16 {
            assert_eq!(decode(0xC250 + n), Some(n as u8));
        }
        // mirrors: A3 ignored, A8/A10 ignored, any odd third nibble
        assert_eq!(decode(0xC258), Some(0));
        assert_eq!(decode(0xC217), Some(7));
        assert_eq!(decode(0xC3F1), Some(1));
        assert_eq!(decode(0xC6D4), Some(4));
        assert_eq!(decode(0xC7B2), Some(2));
        // not the card: A4 = 0, A9 = 0 (ACI $C0xx/$C1xx), A11 = 1, PIA
        assert_eq!(decode(0xC240), None);
        assert_eq!(decode(0xC050), None);
        assert_eq!(decode(0xC150), None);
        assert_eq!(decode(0xCA50), None);
        assert_eq!(decode(0xD012), None);
    }

    #[test]
    fn hst0_model() {
        assert!(hst0_state(0, 0));
        assert!(hst0_state(0, 24));
        assert!(!hst0_state(0, 25));
        assert!(!hst0_state(191, 64));
        assert!(hst0_state(192, 40));
        assert!(!hst0_state(200, 13)); // burst notch, even in VBLANK
        assert!(!hst0_state(10, 15));
        assert!(hst0_state(10, 16));
    }

    #[test]
    fn power_up_is_hires_primary() {
        let g = Gen2::new(false);
        assert!(!g.text && g.hires && !g.mixed && !g.page2);
    }

    #[test]
    fn switches_and_addresses() {
        let mut g = Gen2::new(false);
        g.read(0, 0); // TEXT_OFF (already off at power-up)
        g.read(7, 0); // HIRES_ON (already on at power-up)
        g.read(3, 0); // MIX_ON
        g.read(5, 0); // PAGE_TWO
        assert!(!g.text && g.hires && g.mixed && g.page2);
        assert_eq!(g.mode_name(), "HIRES + texte page 2");
        // Apple II Reference Manual (1979), fig. 2
        assert_eq!(text_row_addr(false, 0), 0x400);
        assert_eq!(text_row_addr(false, 1), 0x480);
        assert_eq!(text_row_addr(false, 8), 0x428);
        assert_eq!(text_row_addr(false, 23), 0x7D0);
        assert_eq!(hires_line_addr(false, 1), 0x2400);
        assert_eq!(hires_line_addr(false, 191), 0x3FD0);
        assert_eq!(hires_line_addr(true, 0), 0x4000);
        assert_eq!(Gen2::new(true).frame_ticks(), 65 * 312);
    }

    #[test]
    fn beam_racing_split() {
        let mut g = Gen2::new(false);
        let ft = g.frame_ticks();
        g.read(0, 5); // frame 0: graphics (LORES) from line 0
        g.read(1, 100 * 65 + 40); // TEXT from line 101 (live part of line 100)
        g.read(0, 150 * 65 + 3); // graphics again from line 150 (HBLANK)
        let l = g.frame_lines(ft + 10);
        assert!(!l[0].text && !l[100].text && l[101].text && l[149].text && !l[150].text);
        // the same frame keeps being shown during frame 1, while frame 1 is logged
        g.read(1, ft + 20 * 65); // frame 1: TEXT from line 20
        let l = g.frame_lines(ft + 30 * 65);
        assert!(!l[0].text && l[101].text);
        let l = g.frame_lines(2 * ft + 1);
        assert!(!l[0].text && l[20].text && l[191].text);
        // much later, nothing logged: uniform current state
        let l = g.frame_lines(10 * ft);
        assert!(l.iter().all(|m| m.text));
    }
}
