//! Apple-1 picture: 40x24 characters drawn with the real 2513 character
//! generator ROM, 7x8 dots per cell => 280x192 dots.

use apple1_core::display::{COLS, ROWS};
use eframe::egui::{Color32, ColorImage};
use serde::{Deserialize, Serialize};

/// Signetics 2513 character generator dump (64 chars x 8 rows, 5 bits).
pub const CHARGEN: &[u8; 512] = include_bytes!("../assets/2513.bin");

pub const CELL_W: usize = 7;
pub const CELL_H: usize = 8;
pub const WIDTH: usize = COLS * CELL_W; // 280
pub const HEIGHT: usize = ROWS * CELL_H; // 192

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Phosphor {
    #[default]
    Vert,
    Blanc,
    Ambre,
}

impl Phosphor {
    pub const ALL: [Phosphor; 3] = [Phosphor::Vert, Phosphor::Blanc, Phosphor::Ambre];
    pub fn label(self) -> &'static str {
        match self {
            Phosphor::Vert => crate::i18n::t("ph_green"),
            Phosphor::Blanc => crate::i18n::t("ph_white"),
            Phosphor::Ambre => crate::i18n::t("ph_amber"),
        }
    }
    pub fn fg(self) -> Color32 {
        match self {
            Phosphor::Vert => Color32::from_rgb(0x44, 0xFF, 0x66),
            Phosphor::Blanc => Color32::from_rgb(0xE8, 0xEE, 0xFF),
            Phosphor::Ambre => Color32::from_rgb(0xFF, 0xB0, 0x00),
        }
    }
    pub fn bg(self) -> Color32 {
        match self {
            Phosphor::Vert => Color32::from_rgb(0x04, 0x0C, 0x06),
            Phosphor::Blanc => Color32::from_rgb(0x08, 0x08, 0x0B),
            Phosphor::Ambre => Color32::from_rgb(0x0C, 0x07, 0x02),
        }
    }
}

/// 2513 address of an ASCII code ($20-$5F): 6-bit code, b6 inverted.
pub fn rom_code(ascii: u8) -> usize {
    let a = ascii & 0x7F;
    ((a & 0x1F) | (((a ^ 0x40) & 0x40) >> 1)) as usize
}

/// The 5 dots of row `row` (0..8) of character `ascii`; bit 4 = leftmost dot.
pub fn glyph_row(ascii: u8, row: usize) -> u8 {
    CHARGEN[(rom_code(ascii) << 3) | row] & 0x1F
}

/// Render the 40x24 text (cells as displayed: $20-$5F) to a 280x192 image.
/// `cursor` = position of the blinking '@' when currently visible.
pub fn render(cells: &[[u8; COLS]; ROWS], cursor: Option<(usize, usize)>, ph: Phosphor) -> ColorImage {
    let (fg, bg) = (ph.fg(), ph.bg());
    let mut px = vec![bg; WIDTH * HEIGHT];
    for (r, line) in cells.iter().enumerate() {
        for (c, &ch) in line.iter().enumerate() {
            let ch = if cursor == Some((r, c)) { b'@' } else { ch };
            if ch == b' ' {
                continue;
            }
            for gy in 0..CELL_H {
                let bits = glyph_row(ch, gy);
                if bits == 0 {
                    continue;
                }
                let y = r * CELL_H + gy;
                for gx in 0..5 {
                    if bits & (0x10 >> gx) != 0 {
                        px[y * WIDTH + c * CELL_W + gx] = fg;
                    }
                }
            }
        }
    }
    ColorImage::new([WIDTH, HEIGHT], px)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rom_layout() {
        // '@' is code 0, 'A' code 1, ' ' code $20, '0' code $30
        assert_eq!(rom_code(b'@'), 0);
        assert_eq!(rom_code(b'A'), 1);
        assert_eq!(rom_code(b' '), 0x20);
        assert_eq!(rom_code(b'0'), 0x30);
        // row 0 is blank for every character
        for c in 0x20..0x60u8 {
            assert_eq!(glyph_row(c, 0), 0, "{c:02X}");
        }
        // 'A': top row of the glyph = the middle dot only (.OOO. or ..O..)
        assert_ne!(glyph_row(b'A', 1), 0);
        assert_eq!(glyph_row(b' ', 4), 0);
        // 'I' is symmetric and has a vertical bar in the middle (bit 2)
        assert_eq!(glyph_row(b'I', 4) & 0x04, 0x04);
    }

    #[test]
    fn render_cursor() {
        let mut cells = [[b' '; COLS]; ROWS];
        cells[0][0] = b'\\';
        let img = render(&cells, Some((0, 1)), Phosphor::Vert);
        let lit = |x0: usize| (0..CELL_H).flat_map(|y| (x0..x0 + CELL_W).map(move |x| (x, y))).filter(|&(x, y)| img.pixels[y * WIDTH + x] == Phosphor::Vert.fg()).count();
        assert!(lit(0) > 0 && lit(CELL_W) > 0 && lit(2 * CELL_W) == 0);
        // the two blank columns of a cell are never lit
        assert!((0..HEIGHT).all(|y| img.pixels[y * WIDTH + 5] != Phosphor::Vert.fg()));
    }
}

/// Mouse selection on an emulated screen: a stream of characters from the
/// cell where the button went down to the cell where it is now (like in a
/// text editor), as (row, column).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScreenSel {
    /// false: Apple-1 terminal, true: GEN2 colour screen.
    pub color: bool,
    pub from: (usize, usize),
    pub to: (usize, usize),
}

impl ScreenSel {
    /// (first, last) cell in reading order, both included.
    pub fn ordered(&self) -> ((usize, usize), (usize, usize)) {
        if self.from <= self.to {
            (self.from, self.to)
        } else {
            (self.to, self.from)
        }
    }

    /// Per row: (row, first column, last column) of the highlighted cells.
    pub fn spans(&self) -> Vec<(usize, usize, usize)> {
        let ((r0, c0), (r1, c1)) = self.ordered();
        (r0..=r1.min(ROWS - 1))
            .map(|r| (r, if r == r0 { c0 } else { 0 }, if r == r1 { c1.min(COLS - 1) } else { COLS - 1 }))
            .collect()
    }

    /// The selected characters; rows are cut at the selection, trailing
    /// spaces dropped, empty rows at both ends removed.
    pub fn text(&self, row_text: impl Fn(usize) -> String) -> String {
        let mut rows: Vec<String> = self
            .spans()
            .into_iter()
            .map(|(r, a, b)| row_text(r).chars().skip(a).take(b + 1 - a).collect::<String>().trim_end().to_string())
            .collect();
        while rows.last().is_some_and(|r| r.is_empty()) {
            rows.pop();
        }
        while rows.first().is_some_and(|r| r.is_empty()) {
            rows.remove(0);
        }
        rows.join("\n")
    }
}

/// Cell under a point of a picture drawn in `rect` with `dot` points per dot.
pub fn cell_at(rect: eframe::egui::Rect, dot: f32, p: eframe::egui::Pos2) -> (usize, usize) {
    let c = ((p.x - rect.min.x) / (dot * CELL_W as f32)).floor().clamp(0.0, COLS as f32 - 1.0) as usize;
    let r = ((p.y - rect.min.y) / (dot * CELL_H as f32)).floor().clamp(0.0, ROWS as f32 - 1.0) as usize;
    (r, c)
}

#[cfg(test)]
mod sel_tests {
    use super::*;

    fn rows(r: usize) -> String {
        format!("{:<40}", ["HELLO WORLD", "  second line", "", "LAST"].get(r).copied().unwrap_or(""))
    }

    #[test]
    fn selection_is_a_stream_in_reading_order() {
        let s = ScreenSel { color: false, from: (0, 6), to: (1, 7) };
        assert_eq!(s.text(rows), "WORLD\n  second");
        // dragged backwards: same text
        let b = ScreenSel { color: false, from: (1, 7), to: (0, 6) };
        assert_eq!(b.text(rows), s.text(rows));
        assert_eq!(b.spans(), vec![(0, 6, 39), (1, 0, 7)]);
        // one row: both ends included
        assert_eq!(ScreenSel { color: false, from: (0, 0), to: (0, 4) }.text(rows), "HELLO");
        // trailing empty rows dropped
        assert_eq!(ScreenSel { color: false, from: (0, 0), to: (2, 39) }.text(rows), "HELLO WORLD\n  second line");
    }

    #[test]
    fn cell_under_the_pointer() {
        use eframe::egui::{pos2, Rect};
        let r = Rect::from_min_size(pos2(100.0, 50.0), eframe::egui::vec2(560.0, 384.0)); // 2x
        assert_eq!(cell_at(r, 2.0, pos2(100.0, 50.0)), (0, 0));
        assert_eq!(cell_at(r, 2.0, pos2(100.0 + 14.0, 50.0 + 16.0)), (1, 1));
        assert_eq!(cell_at(r, 2.0, pos2(5000.0, 5000.0)), (ROWS - 1, COLS - 1));
        assert_eq!(cell_at(r, 2.0, pos2(0.0, 0.0)), (0, 0));
    }
}
