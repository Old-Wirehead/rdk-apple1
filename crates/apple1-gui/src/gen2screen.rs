//! Picture of the colour monitor plugged into Uncle Bernie's GEN2 card.
//!
//! Apple II video modes and memory layout (same data, same picture, same
//! colours as an Apple II); the usual Apple II order of the 16 LORES
//! colours; character codes $00-$3F inverse, $40-$7F flashing, $80-$FF
//! normal, $E0-$FE lower case, $FF = checkerboard.
//!
//! Character generator: the GEN2 glyphs supplied by Uncle Bernie (MIT
//! licence, see NOTICE), in two variants — fat curly braces (better on a
//! colour CRT TV) and thin braces (better on monochrome). Only the first
//! 8 rows of each 9-row pattern are used (empty top line, except $FF which
//! skips the leading blank so the checkerboard fills the cell). Flash rate
//! 1.5 Hz.
//!
//! Approximations: exact RGB values (they depend on NTSC and on the card's
//! trim capacitor: IIgs-like values used here); the HIRES artifact colours
//! are a simplified 280-dot model (the half-dot shift of bit 7 is ignored).

use std::sync::LazyLock;

#[cfg(test)]
use apple1_core::gen2::Gen2;
use apple1_core::gen2::{hires_line_addr, text_row_addr, Modes};
use eframe::egui::{Color32, ColorImage};

use crate::screen::{CELL_W, HEIGHT, WIDTH};

/// Fat curly braces (V1): better on colour CRT TVs. Default.
pub const GLYPHS_FAT: &str = include_str!("../glyphs/A1_G2_glyphs_V1.txt");
/// Thin curly braces (V2): better on monochrome monitors.
pub const GLYPHS_THIN: &str = include_str!("../glyphs/A1_G2_glyphs_V2.txt");

/// 256 characters × 8 rows; bit 7 = leftmost pixel (`@` in the dump).
type GlyphTable = [[u8; 8]; 256];

fn parse_glyphs(text: &str) -> GlyphTable {
    let mut table = [[0u8; 8]; 256];
    let mut present = [false; 256];
    let mut code: Option<u8> = None;
    let mut rows: Vec<&str> = Vec::new();
    let flush = |code: Option<u8>, rows: &mut Vec<&str>, table: &mut GlyphTable, present: &mut [bool; 256]| {
        let Some(c) = code else { return };
        // use the first 8 of the 9 rows; for $FF, skip the leading blank so
        // the checkerboard has no empty top line (every other character has one)
        let start = if c == 0xFF && rows.len() >= 9 { 1 } else { 0 };
        #[allow(clippy::needless_range_loop)]
        for r in 0..8 {
            let line = rows.get(start + r).copied().unwrap_or("--------");
            let mut bits = 0u8;
            for (i, ch) in line.chars().take(8).enumerate() {
                if ch == '@' {
                    bits |= 0x80 >> i;
                }
            }
            table[c as usize][r] = bits;
        }
        present[c as usize] = true;
        rows.clear();
    };
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix('$') {
            flush(code, &mut rows, &mut table, &mut present);
            code = u8::from_str_radix(rest.trim(), 16).ok();
        } else if !line.is_empty() {
            rows.push(line);
        }
    }
    flush(code, &mut rows, &mut table, &mut present);
    // File covers $00-$7F and $E0-$FF. Alias the missing normal range
    // $80-$DF to the non-inverted shapes in $40-$7F.
    for c in 0x80..=0xDF {
        if !present[c] {
            table[c] = table[0x40 | (c & 0x3F)];
        }
    }
    table
}

static TABLE_FAT: LazyLock<GlyphTable> = LazyLock::new(|| parse_glyphs(GLYPHS_FAT));
static TABLE_THIN: LazyLock<GlyphTable> = LazyLock::new(|| parse_glyphs(GLYPHS_THIN));

fn table(fat_braces: bool) -> &'static GlyphTable {
    if fat_braces {
        &TABLE_FAT
    } else {
        &TABLE_THIN
    }
}

/// Apple II LORES colours 0-15 (order documented, RGB approximate).
pub const PALETTE: [u32; 16] = [
    0x000000, // 0 noir
    0xDD0033, // 1 rouge / magenta
    0x000099, // 2 bleu foncé
    0xDD22DD, // 3 violet
    0x007722, // 4 vert foncé
    0x555555, // 5 gris 1
    0x2222FF, // 6 bleu moyen
    0x66AAFF, // 7 bleu clair
    0x885500, // 8 marron
    0xFF6600, // 9 orange
    0xAAAAAA, // 10 gris 2
    0xFF9988, // 11 rose
    0x11DD00, // 12 vert clair
    0xFFFF00, // 13 jaune
    0x44FF99, // 14 aigue-marine
    0xFFFFFF, // 15 blanc
];

/// French colour names (the interface uses the col0..col15 translations).
#[cfg(test)]
pub const NAMES: [&str; 16] = [
    "noir", "magenta", "bleu foncé", "violet", "vert foncé", "gris 1", "bleu moyen", "bleu clair",
    "marron", "orange", "gris 2", "rose", "vert clair", "jaune", "aigue-marine", "blanc",
];

pub fn color(i: u8) -> Color32 {
    let v = PALETTE[(i & 15) as usize];
    Color32::from_rgb((v >> 16) as u8, (v >> 8) as u8, v as u8)
}

const WHITE: Color32 = Color32::WHITE;
const BLACK: Color32 = Color32::BLACK;

/// HIRES artifact colours: [bit 7 = 0: (even x, odd x)], [bit 7 = 1: ...].
fn hires_color(x: usize, hi: bool) -> Color32 {
    match (hi, x & 1) {
        (false, 0) => color(3),  // violet
        (false, _) => color(12), // vert
        (true, 0) => color(6),   // bleu
        (true, _) => color(9),   // orange
    }
}

/// The 7 dots (bit 6 = leftmost) of row `row` (0..8) of screen code `c`,
/// before the flash inversion of $40-$7F. Patterns come from the
/// glyph dump as displayed (`@` = lit); $00-$3F are already inverse.
pub fn char_row(c: u8, row: usize, fat_braces: bool) -> u8 {
    let bits = table(fat_braces)[c as usize][row.min(7)];
    // Dump is 8 dots wide; Apple II cells are 7: drop the rightmost column.
    (bits >> 1) & 0x7F
}

/// One scan line of TEXT (`row` 0..24, glyph line `gy` 0..8).
fn text_line(out: &mut [Color32], mem: &[u8], page2: bool, row: usize, gy: usize, flash_on: bool, fat_braces: bool) {
    let base = text_row_addr(page2, row);
    for col in 0..40 {
        let c = mem[base + col];
        // $00-$3F already stored inverse; $40-$7F stored normal and blink.
        let mut bits = char_row(c, gy, fat_braces);
        if (0x40..0x80).contains(&c) && flash_on {
            bits ^= 0x7F;
        }
        for gx in 0..CELL_W {
            out[col * CELL_W + gx] = if bits & (0x40 >> gx) != 0 { WHITE } else { BLACK };
        }
    }
}

/// One scan line of LORES: 4 lines per block, low nibble = upper block.
fn lores_line(out: &mut [Color32], mem: &[u8], page2: bool, y: usize) {
    let base = text_row_addr(page2, y / 8);
    for col in 0..40 {
        let b = mem[base + col];
        let c = color(if y % 8 < 4 { b & 15 } else { b >> 4 });
        for dx in 0..CELL_W {
            out[col * CELL_W + dx] = c;
        }
    }
}

fn hires_line(out: &mut [Color32], mem: &[u8], page2: bool, y: usize) {
    let base = hires_line_addr(page2, y);
    let bytes = &mem[base..base + 40];
    let dot = |x: isize| -> bool { x >= 0 && x < WIDTH as isize && bytes[x as usize / 7] & (1 << (x as usize % 7)) != 0 };
    for (x, px) in out.iter_mut().enumerate().take(WIDTH) {
        let xi = x as isize;
        let hi = bytes[x / 7] & 0x80 != 0;
        let (l, c, r) = (dot(xi - 1), dot(xi), dot(xi + 1));
        *px = if c {
            if l || r {
                WHITE
            } else {
                hires_color(x, hi)
            }
        } else if l && r {
            // 101: the gap takes the colour of its neighbours
            hires_color(x - 1, bytes[(x - 1) / 7] & 0x80 != 0)
        } else {
            BLACK
        };
    }
}

/// Render the colour monitor (280x192) with the modes of each scan line
/// (`lines`, see `Gen2::frame_lines`). `flash_on` = phase of the flashing
/// characters.
#[cfg(test)]
pub fn render_lines(lines: &[Modes; HEIGHT], mem: &[u8], flash_on: bool) -> ColorImage {
    render_lines_ex(lines, mem, flash_on, true)
}

/// Like `render_lines`, with a choice of curly-brace glyph variant.
pub fn render_lines_ex(lines: &[Modes; HEIGHT], mem: &[u8], flash_on: bool, fat_braces: bool) -> ColorImage {
    let mut px = vec![BLACK; WIDTH * HEIGHT];
    for (y, m) in lines.iter().enumerate() {
        let out = &mut px[y * WIDTH..(y + 1) * WIDTH];
        if m.text || (m.mixed && y >= 160) {
            text_line(out, mem, m.page2, y / 8, y % 8, flash_on, fat_braces);
        } else if m.hires {
            hires_line(out, mem, m.page2, y);
        } else {
            lores_line(out, mem, m.page2, y);
        }
    }
    ColorImage::new([WIDTH, HEIGHT], px)
}

/// Render with the current modes on every line.
#[cfg(test)]
pub fn render(g: &Gen2, mem: &[u8], flash_on: bool) -> ColorImage {
    render_lines(&[g.modes(); HEIGHT], mem, flash_on)
}

/// Cache key: everything the picture depends on.
#[cfg(test)]
pub fn signature(lines: &[Modes; HEIGHT], mem: &[u8], flash_on: bool) -> u64 {
    signature_ex(lines, mem, flash_on, true)
}

pub fn signature_ex(lines: &[Modes; HEIGHT], mem: &[u8], flash_on: bool, fat_braces: bool) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    (lines, flash_on, fat_braces).hash(&mut h);
    let (mut text, mut hires) = ([false; 2], [false; 2]);
    for m in lines {
        let p = m.page2 as usize;
        text[p] |= m.text || m.mixed || !m.hires;
        hires[p] |= !m.text && m.hires;
    }
    for p in 0..2 {
        if text[p] {
            let b = 0x400 + p * 0x400;
            mem[b..b + 0x400].hash(&mut h);
        }
        if hires[p] {
            let b = 0x2000 + p * 0x2000;
            mem[b..b + 0x2000].hash(&mut h);
        }
    }
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card() -> Gen2 {
        let mut g = Gen2::new(false);
        g.text = false;
        g.hires = false; // LORES (power-up is HIRES)
        g
    }

    #[test]
    fn lores_blocks_and_palette_order() {
        let mut mem = vec![0u8; 0x10000];
        mem[0x400] = 0xE1; // top: 1 (magenta), bottom: 14 (aquamarine)
        mem[0x7D0 + 39] = 0xDD; // last block: yellow
        let img = render(&card(), &mem, false);
        assert_eq!(img.pixels[0], color(1));
        assert_eq!(img.pixels[5 * WIDTH + 6], color(14));
        assert_eq!(img.pixels[191 * WIDTH + 279], color(13));
        assert_eq!(NAMES[13], "jaune");
        assert_eq!(color(15), Color32::WHITE);
    }

    #[test]
    fn hires_artifacts() {
        let mut mem = vec![0u8; 0x10000];
        let mut g = card();
        g.hires = true;
        mem[0x2000] = 0x01; // x=0 alone, bit 7 = 0: violet
        mem[0x2400] = 0x02; // line 1, x=1: green
        mem[0x2800] = 0x81; // line 2, x=0, bit 7 = 1: blue
        mem[0x2C00] = 0x82; // line 3, x=1: orange
        mem[0x3000] = 0x03; // line 4, x=0,1: white
        mem[0x3400] = 0x05; // line 5, 101: violet filled
        let img = render(&g, &mem, false);
        assert_eq!(img.pixels[0], color(3));
        assert_eq!(img.pixels[WIDTH + 1], color(12));
        assert_eq!(img.pixels[2 * WIDTH], color(6));
        assert_eq!(img.pixels[3 * WIDTH + 1], color(9));
        assert_eq!(img.pixels[4 * WIDTH], WHITE);
        assert_eq!(img.pixels[4 * WIDTH + 1], WHITE);
        assert_eq!(img.pixels[5 * WIDTH + 1], color(3));
        assert_eq!(img.pixels[6 * WIDTH], BLACK);
    }

    #[test]
    fn hires_msb_selects_blue_orange_and_right_edge_stays_orange() {
        // bit 7 of a byte switches the pair of artifact colours
        // (violet/green -> blue/orange). A vertical line in the last column
        // (dot 279 = bit 6 of byte 39) is orange with bit 7 set, green without,
        // on all 192 lines, never grey or white.
        for (msb, want) in [(0x80u8, color(9)), (0x00, color(12))] {
            let mut mem = vec![0u8; 0x10000];
            let mut g = card();
            g.hires = true;
            for y in 0..HEIGHT {
                mem[apple1_core::gen2::hires_line_addr(false, y) + 39] = msb | 0x40;
            }
            let img = render(&g, &mem, false);
            for y in 0..HEIGHT {
                assert_eq!(img.pixels[y * WIDTH + WIDTH - 1], want, "line {y}");
                assert_eq!(img.pixels[y * WIDTH + WIDTH - 2], BLACK, "line {y}");
            }
        }
        // same dot, other parity (x = 278, bit 5): violet / blue
        let mut mem = vec![0u8; 0x10000];
        let mut g = card();
        g.hires = true;
        mem[0x2000 + 39] = 0x20;
        mem[0x2400 + 39] = 0xA0;
        let img = render(&g, &mem, false);
        assert_eq!(img.pixels[WIDTH - 2], color(3));
        assert_eq!(img.pixels[WIDTH + WIDTH - 2], color(6));
    }

    #[test]
    fn text_inverse_flash_normal_lower() {
        let mut mem = vec![0xA0u8; 0x10000];
        mem[0x400] = 0x01; // inverse A
        mem[0x401] = 0x41; // flashing A
        mem[0x402] = 0xC1; // normal A
        mem[0x403] = 0xE1; // a
        let mut g = Gen2::new(false);
        g.text = true;
        g.hires = false;
        let on = render(&g, &mem, true);
        let off = render(&g, &mem, false);
        let lit = |img: &ColorImage, c: usize| (0..8).flat_map(|y| (0..7).map(move |x| (x, y))).filter(|&(x, y)| img.pixels[y * WIDTH + c * 7 + x] == WHITE).count();
        assert_eq!(lit(&on, 0), 56 - lit(&on, 2)); // inverse = complement of normal
        assert_eq!(lit(&on, 1), lit(&on, 0));
        assert_eq!(lit(&off, 1), lit(&off, 2));
        assert!(lit(&on, 3) > 0 && lit(&on, 3) != lit(&on, 2));
        assert_eq!(lit(&on, 4), 0); // space
        // MIXED: rows 20-23 are text
        let mut g = card();
        g.mixed = true;
        mem[0x650] = 0x01;
        let img = render(&g, &mem, false);
        assert!((0..7).any(|x| img.pixels[160 * WIDTH + x] == WHITE));
    }

    #[test]
    fn split_screen_per_line() {
        let mut mem = vec![0x11u8; 0x10000]; // LORES: all magenta / TEXT: inverse Q
        mem[0x400..0x800].fill(0x11);
        let gfx = Modes::default();
        let txt = Modes { text: true, ..gfx };
        let mut lines = [gfx; HEIGHT];
        lines[96..].fill(txt);
        let img = render_lines(&lines, &mem, false);
        assert_eq!(img.pixels[95 * WIDTH + 3], color(1));
        // the text half: inverse characters, black or white only
        assert!((96 * WIDTH..HEIGHT * WIDTH).all(|i| img.pixels[i] == WHITE || img.pixels[i] == BLACK));
        assert_ne!(signature(&lines, &mem, false), signature(&[gfx; HEIGHT], &mem, false));
    }

    #[test]
    fn bernie_glyphs_and_brace_variants() {
        assert_ne!(char_row(0x01, 1, true), 0);
        assert_eq!(char_row(0x01, 1, true), char_row(0x41, 1, true) ^ 0x7F);
        assert_eq!(char_row(0xC1, 1, true), char_row(0x41, 1, true));
        assert_eq!(char_row(0x41, 0, true), 0);
        assert_ne!(char_row(0xFF, 0, true), 0);
        assert_ne!(char_row(0xFB, 1, true), char_row(0xFB, 1, false));
        assert_eq!(char_row(0x41, 1, true), char_row(0x41, 1, false));
    }

    /// Fill text page 1 with a full 24-row glyph showcase.
    fn fill_showcase(mem: &mut [u8]) {
        // Apple II text screen layout is not linear: use text_row_addr.
        let put = |mem: &mut [u8], row: usize, col: usize, c: u8| {
            let a = text_row_addr(false, row) + col;
            mem[a] = c;
        };
        for r in 0..24 {
            for c in 0..40 {
                put(mem, r, c, 0xA0); // space
            }
        }
        // Row 0: title-ish normal uppercase
        for (i, ch) in b"UNCLE BERNIE CHARGEN $00-$FF".iter().enumerate() {
            put(mem, 0, i, ch | 0x80); // normal
        }
        // Row 1: full alphabet + digits
        for i in 0..26usize {
            put(mem, 1, i, 0xC1 + i as u8);
        }
        for i in 0..10usize {
            put(mem, 1, 27 + i, 0xB0 + i as u8);
        }
        // Row 2: lowercase + braces + checkerboard
        for i in 0..26usize {
            put(mem, 2, i, 0xE1 + i as u8);
        }
        put(mem, 2, 27, 0xFB); // {
        put(mem, 2, 28, 0xFC); // |
        put(mem, 2, 29, 0xFD); // }
        put(mem, 2, 30, 0xFE); // ~
        put(mem, 2, 31, 0xFF); // checkerboard
        // Row 3: inverse $00-$27
        for i in 0..40usize {
            put(mem, 3, i, (i as u8) & 0x3F);
        }
        // Row 4: flashing $40-$67 (shown in "on"/inverse phase)
        for i in 0..40usize {
            put(mem, 4, i, 0x40 + ((i as u8) & 0x3F));
        }
        // Row 5: flashing same, will render flash_off in second image — label
        for (i, ch) in b"FLASH OFF PHASE (NORMAL LOOK):".iter().enumerate() {
            put(mem, 5, i, ch | 0x80);
        }
        for i in 0..10usize {
            put(mem, 5, 30 + i, 0x40 + i as u8);
        }
        // Rows 6-7: punctuation / symbols normal
        for i in 0..32usize {
            put(mem, 6, i, 0xA0 + i as u8); // space through ?
        }
        for i in 0..32usize {
            put(mem, 7, i, 0xC0 + i as u8); // @ through _
        }
        // Rows 8-23: repeating lowercase pangram so all 24 lines are visibly filled
        // Exactly 40 columns.
        let line = b"the quick brown fox jumps over lazy dogs";
        assert_eq!(line.len(), 40);
        for r in 8..24 {
            for (i, &ch) in line.iter().enumerate() {
                let c = if ch.is_ascii_lowercase() {
                    0xE1 + (ch - b'a')
                } else if ch == b' ' {
                    0xA0
                } else {
                    ch | 0x80
                };
                put(mem, r, i, c);
            }
        }
    }

    fn save_rgba(path: &str, w: usize, h: usize, data: &[u8]) {
        let f = std::fs::File::create(path).expect(path);
        let mut enc = png::Encoder::new(std::io::BufWriter::new(f), w as u32, h as u32);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        let mut wr = enc.write_header().unwrap();
        wr.write_image_data(data).unwrap();
    }

    fn to_amber(img: &ColorImage) -> ColorImage {
        use crate::screen::Phosphor;
        let (fg, bg) = (Phosphor::Ambre.fg(), Phosphor::Ambre.bg());
        let mut out = img.clone();
        for p in &mut out.pixels {
            *p = if *p == WHITE { fg } else if *p == BLACK { bg } else { *p };
        }
        out
    }

    /// Integer-scale into a padded frame, optional CRT barrel (~bulge) and thin scan gaps.
    /// `pad` = black margin in source pixels (pre-scale) so bulge does not clip text.
    fn upscale_vintage(img: &ColorImage, scale: usize, bulge: f32, gaps: bool, pad: usize) -> (usize, usize, Vec<u8>) {
        use crate::screen::Phosphor;
        let bg = Phosphor::Ambre.bg().to_array();
        let sw = img.size[0];
        let sh = img.size[1];
        let pw = sw + pad * 2;
        let ph = sh + pad * 2;
        let w = pw * scale;
        let h = ph * scale;
        let mut data = vec![0u8; w * h * 4];
        let k = bulge.clamp(0.0, 0.3);
        for dy in 0..h {
            // One darkened destination row under each source pixel row (thin gap, not zebra).
            let gap = gaps && scale >= 4 && dy % scale == scale - 1;
            for dx in 0..w {
                let x = dx as f32 / (w - 1) as f32 * 2.0 - 1.0;
                let y = dy as f32 / (h - 1) as f32 * 2.0 - 1.0;
                let (tx, ty) = if k > 0.0 {
                    let r2 = (x * x + y * y).min(0.995);
                    let f = (1.0 - k) / (1.0 - k * r2);
                    (x * f, y * f)
                } else {
                    (x, y)
                };
                // Map padded frame coords back to source pixels.
                let sx = ((tx + 1.0) * 0.5 * (pw - 1) as f32).round() as isize - pad as isize;
                let sy = ((ty + 1.0) * 0.5 * (ph - 1) as f32).round() as isize - pad as isize;
                let mut rgba = if sx < 0 || sy < 0 || sx >= sw as isize || sy >= sh as isize {
                    bg
                } else {
                    img.pixels[sy as usize * sw + sx as usize].to_array()
                };
                if gap {
                    for v in rgba.iter_mut().take(3) {
                        *v = (*v as u16 * 78 / 100) as u8;
                    }
                }
                let i = (dy * w + dx) * 4;
                data[i..i + 4].copy_from_slice(&rgba);
            }
        }
        (w, h, data)
    }

    /// Vintage colour screen as the application draws it (curvature 0.08,
    /// raster gaps, RGB phosphor stripes): /tmp/gen2-vintage-*.png at 3x, 4x
    /// and 5x, for a look at the result (the stripes themselves are tested
    /// in crt.rs): only checks that the picture is lit.
    #[test]
    fn write_vintage_rgb_mask_screenshots() {
        use crate::crt::{self, Screening};
        let mut mem = vec![0u8; 0x10000];
        fill_showcase(&mut mem);
        let mut g = Gen2::new(false);
        g.text = true;
        g.hires = false;
        let raw = render_lines_ex(&[g.modes(); HEIGHT], &mem, true, true);
        let scr = Screening { on: true, curvature: true, curve: (0.08 - 0.01) / 0.11, ..Screening::default() };
        assert!((scr.curvature_amount() - 0.08).abs() < 1e-4);
        for scale in [3usize, 4, 5] {
            let mut c = crt::Crt::default();
            let processed = c.process(&raw, BLACK, &scr, crt::tau(None), 0.016);
            let out = crt::bake(&processed, WIDTH * scale, HEIGHT * scale, &scr, 1.0);
            let data: Vec<u8> = out.pixels.iter().flat_map(|p| [p.r(), p.g(), p.b(), 255]).collect();
            save_rgba(&format!("/tmp/gen2-vintage-{scale}x.png"), out.size[0], out.size[1], &data);
            assert!(out.pixels.iter().any(|p| p.r() > 200 || p.g() > 200 || p.b() > 200));
        }
    }

    #[test]
    fn write_bernie_glyphs_screenshot() {
        let mut mem = vec![0u8; 0x10000];
        fill_showcase(&mut mem);
        let mut g = Gen2::new(false);
        g.text = true;
        g.hires = false;

        // 1) Raw fat-brace frame (no CRT) — diagnostic of the glyph path
        let raw = to_amber(&render_lines_ex(&[g.modes(); HEIGHT], &mem, true, true));
        let (w, h, data) = upscale_vintage(&raw, 4, 0.0, false, 0);
        save_rgba("/tmp/bernie-glyphs-raw.png", w, h, &data);

        // 2) Vintage fat braces, flash ON, bulge 0.08, padded so text is not clipped
        let (w, h, data) = upscale_vintage(&raw, 4, 0.08, true, 16);
        save_rgba("/tmp/bernie-glyphs-test.png", w, h, &data);
        // Also a no-gap reference (bulge only) to separate CRT gaps from glyph issues.
        let (w, h, data) = upscale_vintage(&raw, 4, 0.08, false, 16);
        save_rgba("/tmp/bernie-glyphs-bulge.png", w, h, &data);

        // 3) Thin braces variant (same content)
        let raw_thin = to_amber(&render_lines_ex(&[g.modes(); HEIGHT], &mem, true, false));
        let (w, h, data) = upscale_vintage(&raw_thin, 4, 0.08, true, 16);
        save_rgba("/tmp/bernie-glyphs-thin.png", w, h, &data);

        // 4) Flash OFF phase (flashing $40-$7F look normal)
        let raw_off = to_amber(&render_lines_ex(&[g.modes(); HEIGHT], &mem, false, true));
        let (w, h, data) = upscale_vintage(&raw_off, 4, 0.08, true, 16);
        save_rgba("/tmp/bernie-glyphs-flashoff.png", w, h, &data);



        assert!(std::path::Path::new("/tmp/bernie-glyphs-test.png").exists());
        assert!(std::path::Path::new("/tmp/bernie-glyphs-raw.png").exists());
    }
}

/// The character a text-page byte shows (inverse and flashing letters give
/// the plain letter), as ASCII.
pub fn text_char(c: u8) -> char {
    match c {
        0xFF => ' ',
        0xE0..=0xFE => (c & 0x7F) as char,
        _ => {
            let l = c & 0x3F;
            (if l < 0x20 { l + 0x40 } else { l }) as char
        }
    }
}

/// Text of row `row` (0..24) of the picture: spaces where the row shows
/// graphics instead of text.
pub fn row_text(lines: &[Modes; HEIGHT], mem: &[u8], row: usize) -> String {
    let m = lines[(row * 8).min(HEIGHT - 1)];
    if !(m.text || (m.mixed && row * 8 >= 160)) {
        return " ".repeat(40);
    }
    let base = text_row_addr(m.page2, row);
    (0..40).map(|c| text_char(mem[base + c])).collect()
}

#[cfg(test)]
mod text_tests {
    use super::*;

    #[test]
    fn text_page_bytes_read_back_as_ascii() {
        assert_eq!(text_char(0xC1), 'A');
        assert_eq!(text_char(0x01), 'A'); // inverse
        assert_eq!(text_char(0x41), 'A'); // flashing
        assert_eq!(text_char(0xA0), ' ');
        assert_eq!(text_char(0xB1), '1');
        assert_eq!(text_char(0xE1), 'a');
        assert_eq!(text_char(0xFF), ' ');
        let mut mem = vec![0xA0u8; 0x10000];
        let base = text_row_addr(false, 3);
        for (i, ch) in "READY".bytes().enumerate() {
            mem[base + i] = ch | 0x80;
        }
        let text = Modes { text: true, mixed: false, page2: false, hires: false };
        let gr = Modes { text: false, mixed: false, page2: false, hires: true };
        assert!(row_text(&[text; HEIGHT], &mem, 3).starts_with("READY "));
        assert_eq!(row_text(&[gr; HEIGHT], &mem, 3).trim(), "");
    }
}
