//! Apple-1 video terminal: 40 x 24 characters, upper case only, hardware
//! scrolling, blinking '@' cursor. Only carriage return is interpreted; other
//! control characters are ignored, exactly like the real Signetics shift
//! register based terminal.

pub const COLS: usize = 40;
pub const ROWS: usize = 24;

#[derive(Debug, Clone)]
pub struct Display {
    pub(crate) cells: [[u8; COLS]; ROWS],
    pub col: usize,
    pub row: usize,
    /// Incremented each time the content changes (for frontends).
    pub generation: u64,
    /// Optional transcript of every character output (tests, documentation).
    pub log: Option<Vec<u8>>,
}

impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}

/// Blinking of the `@` cursor, produced on the real board by the NE555 D13
/// wired as an astable multivibrator (R10 = Ra, R11 = Rb, C7). The output is
/// high for `ln 2 (Ra + Rb) C` and low for `ln 2 Rb C`; the cursor is shown
/// while the output is high (2/3 of the period with Ra = Rb).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CursorBlink {
    /// Blink frequency in Hz.
    pub hz: f64,
    /// Fraction of the period during which the cursor is visible (0..1).
    pub duty: f64,
}

impl CursorBlink {
    /// Classic 555 astable timing (t = ln 2 x R x C).
    pub fn from_555(ra_ohms: f64, rb_ohms: f64, c_farads: f64) -> Self {
        let high = std::f64::consts::LN_2 * (ra_ohms + rb_ohms) * c_farads;
        let low = std::f64::consts::LN_2 * rb_ohms * c_farads;
        CursorBlink { hz: 1.0 / (high + low), duty: high / (high + low) }
    }

    /// Nominal values of the Apple-1: R10 = R11 = 10 kOhm, C7 = 22 uF
    /// (about 2.18 Hz, visible 305 ms, hidden 152 ms).
    pub fn apple1() -> Self {
        Self::from_555(10e3, 10e3, 22e-6)
    }

    /// Same duty cycle as the nominal circuit, another frequency (what
    /// happens when C7 is off its nominal value, e.g. 1.7 Hz measured).
    pub fn with_hz(hz: f64) -> Self {
        CursorBlink { hz, ..Self::apple1() }
    }

    /// Visible time and hidden time, in seconds.
    pub fn on_off_secs(&self) -> (f64, f64) {
        let p = 1.0 / self.hz;
        (p * self.duty, p * (1.0 - self.duty))
    }

    /// Is the cursor shown `t` seconds after power-up?
    pub fn visible_at(&self, t: f64) -> bool {
        if self.hz.is_nan() || self.hz <= 0.0 {
            return true;
        }
        (t * self.hz).fract() < self.duty
    }
}

impl Default for CursorBlink {
    fn default() -> Self {
        Self::apple1()
    }
}

/// Map a 7-bit code to the glyph displayed by the 2513 character generator.
/// Returns `None` for control characters (ignored by the hardware).
pub fn glyph(code: u8) -> Option<u8> {
    let c = code & 0x7F;
    if c < 0x20 {
        return None; // control character (rejected by the video hardware)
    }
    // Only 6 bits are stored (b0-b4 and b6; b5 is ignored, b6 inverted
    // before the 2513, see hardware/INTERFACES.md): lower case $60-$7F
    // therefore shows as upper case $40-$5F.
    Some(if c & 0x40 != 0 { 0x40 | (c & 0x1F) } else { 0x20 | (c & 0x1F) })
}

impl Display {
    pub fn new() -> Self {
        Display { cells: [[b' '; COLS]; ROWS], col: 0, row: 0, generation: 0, log: None }
    }

    /// The CLEAR SCREEN button.
    pub fn clear(&mut self) {
        self.cells = [[b' '; COLS]; ROWS];
        self.col = 0;
        self.row = 0;
        self.generation += 1;
    }

    fn newline(&mut self) {
        self.col = 0;
        if self.row + 1 >= ROWS {
            self.cells.copy_within(1.., 0);
            self.cells[ROWS - 1] = [b' '; COLS];
        } else {
            self.row += 1;
        }
    }

    /// Output one character (bit 7 ignored).
    pub fn put(&mut self, code: u8) {
        let c = code & 0x7F;
        if let Some(l) = self.log.as_mut() {
            l.push(c);
        }
        if c == 0x0D {
            self.newline();
            self.generation += 1;
            return;
        }
        if let Some(g) = glyph(c) {
            self.cells[self.row][self.col] = g;
            self.col += 1;
            if self.col >= COLS {
                self.newline();
            }
            self.generation += 1;
        }
    }

    pub fn row_text(&self, r: usize) -> String {
        self.cells[r].iter().map(|&b| b as char).collect()
    }

    pub fn cell(&self, r: usize, c: usize) -> u8 {
        self.cells[r][c]
    }

    /// Whole screen as text (rows joined by '\n', trailing spaces trimmed).
    pub fn text(&self) -> String {
        (0..ROWS).map(|r| self.row_text(r).trim_end().to_string()).collect::<Vec<_>>().join("\n")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cursor_555_nominal() {
        let b = CursorBlink::apple1();
        assert!((b.hz - 2.186).abs() < 0.01, "{}", b.hz);
        assert!((b.duty - 2.0 / 3.0).abs() < 1e-9);
        let (on, off) = b.on_off_secs();
        assert!((on - 0.305).abs() < 0.001 && (off - 0.1525).abs() < 0.001, "{on} {off}");
        assert!(b.visible_at(0.0) && b.visible_at(0.30) && !b.visible_at(0.31) && !b.visible_at(0.45));
        assert!(b.visible_at(1.0 / b.hz + 0.01));
        let m = CursorBlink::with_hz(1.7);
        assert!((m.duty - b.duty).abs() < 1e-12 && m.hz == 1.7);
    }

    use super::*;
    #[test]
    fn cr_wrap_scroll_and_controls() {
        let mut d = Display::new();
        for b in b"HELLO\rWORLD" {
            d.put(*b | 0x80);
        }
        assert_eq!(d.row_text(0).trim_end(), "HELLO");
        assert_eq!(d.row_text(1).trim_end(), "WORLD");
        d.put(0x07); // bell: ignored
        d.put(0x0A); // LF: ignored
        assert_eq!((d.row, d.col), (1, 5));
        d.clear();
        for _ in 0..41 {
            d.put(b'A');
        }
        assert_eq!((d.row, d.col), (1, 1));
        d.clear();
        for i in 0..30u8 {
            d.put(b'0' + (i % 10));
            d.put(0x0D);
        }
        assert_eq!(d.row, ROWS - 1);
        // last printed line (29 -> '9') is just above the cursor line
        assert_eq!(d.row_text(ROWS - 2).trim_end(), "9");
        // lower case shows as the 2513 would
        assert_eq!(glyph(b'a'), Some(b'A'));
        assert_eq!(glyph(b'!'), Some(b'!'));
        assert_eq!(glyph(0x7F), Some(b'_'));
        assert_eq!(glyph(b'_'), Some(b'_'));
    }
}
