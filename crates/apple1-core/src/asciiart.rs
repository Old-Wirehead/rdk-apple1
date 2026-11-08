//! ASCII art for the Apple-1 screen: a photo (grey levels) becomes 40x24
//! characters of the 2513 generator (upper case and symbols only), then an
//! Integer BASIC program (PRINT lines) or a 6502 program (ECHO $FFEF and a
//! run-length coded table) that draws it.
//!
//! The Apple-1 terminal goes to the next line by itself after the 40th
//! character: a full row is not followed by a CR. With 24 rows the last one
//! keeps its 40th column blank, otherwise the screen would scroll.

pub const COLS: usize = 40;
pub const ROWS: usize = 24;

/// Lit dots of each 2513 character $20-$5F (5x8 matrix).
const DENSITY: [u8; 64] = [
    0, 6, 6, 20, 17, 13, 14, 3, 7, 7, 15, 9, 3, 5, 1, 5, 19, 10, 15, 15, 14, 17, 16, 11, 17, 16, 2, 4, 7, 10, 7, 9, 20, 16, 20, 13, 18, 18,
    14, 16, 17, 11, 10, 14, 11, 18, 17, 16, 15, 17, 18, 15, 11, 15, 13, 18, 13, 10, 15, 20, 5, 20, 5, 5,
];

/// Few levels, clean result.
pub const RAMP_SIMPLE: &str = " .:-=+*%#@";
/// Many levels (photos).
pub const RAMP_FULL: &str = " .,:;-!+=?7IT%XK*SAOHMWB#@";
/// The 16 levels of Dave Schmenk's Apple1-Slideshow (2014).
pub const RAMP_SLIDESHOW: &str = " .:,;!-^+=?&*%#@";

/// Ink of a character (lit dots, 0-20).
pub fn density(c: u8) -> u8 {
    if (0x20..0x60).contains(&c) {
        DENSITY[(c - 0x20) as usize]
    } else {
        0
    }
}

/// A character usable in the art: shown by the 2513 ($20-$5F) and safe in a
/// BASIC string and when typed (no `"`, no `_` which is the backspace).
pub fn allowed(c: u8) -> bool {
    (0x20..0x60).contains(&c) && c != b'"' && c != b'_'
}

/// Upper case, allowed characters only, a space, sorted from the lightest
/// to the darkest ink, one character per ink level.
pub fn clean_ramp(s: &str) -> Vec<u8> {
    let mut v: Vec<u8> = vec![b' '];
    for c in s.bytes().map(|c| c.to_ascii_uppercase()) {
        if allowed(c) && !v.contains(&c) {
            v.push(c);
        }
    }
    v.sort_by_key(|&c| density(c));
    // equal ink: the same level, keep the last one given
    let mut out: Vec<u8> = Vec::new();
    for c in v {
        if out.last().is_some_and(|&l| density(l) == density(c)) {
            out.pop();
        }
        out.push(c);
    }
    out
}

/// A grey image, 0.0 = black .. 1.0 = white, row by row.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Grey {
    pub w: usize,
    pub h: usize,
    pub px: Vec<f32>,
}

impl Grey {
    /// From 8-bit RGB(A) pixels (`n` = 3 or 4 bytes per pixel); a
    /// transparent pixel counts as black (the screen).
    pub fn from_rgba(w: usize, h: usize, data: &[u8], n: usize) -> Grey {
        let px = data
            .chunks_exact(n)
            .take(w * h)
            .map(|p| {
                let l = (0.299 * p[0] as f32 + 0.587 * p[1] as f32 + 0.114 * p[2] as f32) / 255.0;
                if n == 4 {
                    l * p[3] as f32 / 255.0
                } else {
                    l
                }
            })
            .collect();
        Grey { w, h, px }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Params {
    pub cols: usize,
    pub rows: usize,
    /// 1 = the largest part of the image with the screen's shape.
    pub zoom: f32,
    /// Centre of the part, 0..1 of the width / height.
    pub cx: f32,
    pub cy: f32,
    /// -1..1 (0 = unchanged).
    pub brightness: f32,
    /// 1 = unchanged.
    pub contrast: f32,
    /// 1 = unchanged, > 1 lightens the middle greys.
    pub gamma: f32,
    pub invert: bool,
    /// Stretch the grey levels of the part (2 %-98 %) to black-white first.
    pub auto_level: bool,
    /// Floyd-Steinberg error diffusion.
    pub dither: bool,
    pub ramp: String,
    /// Width / height of a character cell on the screen (7x8 dots of a
    /// 280x192 picture shown 4:3: 0.8).
    pub aspect: f32,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            cols: COLS,
            rows: ROWS,
            zoom: 1.0,
            cx: 0.5,
            cy: 0.5,
            brightness: 0.0,
            contrast: 1.0,
            gamma: 1.0,
            invert: false,
            auto_level: true,
            dither: false,
            ramp: RAMP_FULL.into(),
            aspect: 0.8,
        }
    }
}

impl Params {
    pub fn size(&self) -> (usize, usize) {
        (self.cols.clamp(1, COLS), self.rows.clamp(1, ROWS))
    }
}

/// The part of the image converted: (x, y, w, h) in pixels.
pub fn crop(img: &Grey, p: &Params) -> (f32, f32, f32, f32) {
    let (cols, rows) = p.size();
    let (w, h) = (img.w.max(1) as f32, img.h.max(1) as f32);
    let a = cols as f32 * p.aspect.max(0.1) / rows as f32;
    let (mut cw, mut ch) = if w / h > a { (h * a, h) } else { (w, w / a) };
    let z = p.zoom.clamp(1.0, 20.0);
    cw /= z;
    ch /= z;
    let x = (p.cx.clamp(0.0, 1.0) * w - cw / 2.0).clamp(0.0, w - cw);
    let y = (p.cy.clamp(0.0, 1.0) * h - ch / 2.0).clamp(0.0, h - ch);
    (x, y, cw, ch)
}

/// Mean grey of each cell of the cropped part.
fn sample(img: &Grey, p: &Params) -> Vec<f32> {
    let (cols, rows) = p.size();
    let (x0, y0, cw, ch) = crop(img, p);
    let mut out = vec![0.0; cols * rows];
    if img.w == 0 || img.h == 0 {
        return out;
    }
    for r in 0..rows {
        let ya = y0 + ch * r as f32 / rows as f32;
        let yb = y0 + ch * (r + 1) as f32 / rows as f32;
        let (ia, ib) = (ya.floor() as usize, (yb.ceil() as usize).max(ya.floor() as usize + 1).min(img.h));
        for c in 0..cols {
            let xa = x0 + cw * c as f32 / cols as f32;
            let xb = x0 + cw * (c + 1) as f32 / cols as f32;
            let (ja, jb) = (xa.floor() as usize, (xb.ceil() as usize).max(xa.floor() as usize + 1).min(img.w));
            let (mut s, mut n) = (0.0, 0usize);
            for i in ia.min(img.h - 1)..ib {
                for j in ja.min(img.w - 1)..jb {
                    s += img.px[i * img.w + j];
                    n += 1;
                }
            }
            out[r * cols + c] = if n > 0 { s / n as f32 } else { 0.0 };
        }
    }
    out
}

/// Brightness, contrast, gamma, invert.
pub fn adjust(v: f32, p: &Params) -> f32 {
    let v = ((v - 0.5) * p.contrast + 0.5 + p.brightness).clamp(0.0, 1.0);
    let v = v.powf(1.0 / p.gamma.clamp(0.05, 20.0));
    if p.invert {
        1.0 - v
    } else {
        v
    }
}

/// Text on the screen: at most 24 rows of at most 40 allowed characters.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Art {
    pub lines: Vec<String>,
}

/// Converts the image: bright = much ink (green on black screen).
pub fn convert(img: &Grey, p: &Params) -> Art {
    let (cols, rows) = p.size();
    let ramp = clean_ramp(&p.ramp);
    let max = ramp.iter().map(|&c| density(c)).max().unwrap_or(1).max(1) as f32;
    let levels: Vec<f32> = ramp.iter().map(|&c| density(c) as f32 / max).collect();
    let mut v = sample(img, p);
    if p.auto_level && !v.is_empty() {
        let mut s = v.clone();
        s.sort_by(f32::total_cmp);
        let (lo, hi) = (s[s.len() * 2 / 100], s[(s.len() * 98 / 100).min(s.len() - 1)]);
        if hi - lo > 0.02 {
            v.iter_mut().for_each(|x| *x = ((*x - lo) / (hi - lo)).clamp(0.0, 1.0));
        }
    }
    let mut v: Vec<f32> = v.into_iter().map(|x| adjust(x, p)).collect();
    let mut lines = Vec::with_capacity(rows);
    for r in 0..rows {
        let mut line = String::with_capacity(cols);
        for c in 0..cols {
            let x = v[r * cols + c];
            let (i, _) = levels
                .iter()
                .enumerate()
                .min_by(|a, b| (a.1 - x).abs().total_cmp(&(b.1 - x).abs()))
                .unwrap_or((0, &0.0));
            line.push(ramp[i] as char);
            if p.dither {
                let e = x - levels[i];
                let mut add = |rr: usize, cc: isize, f: f32| {
                    if rr < rows && cc >= 0 && (cc as usize) < cols {
                        v[rr * cols + cc as usize] += e * f;
                    }
                };
                add(r, c as isize + 1, 7.0 / 16.0);
                add(r + 1, c as isize - 1, 3.0 / 16.0);
                add(r + 1, c as isize, 5.0 / 16.0);
                add(r + 1, c as isize + 1, 1.0 / 16.0);
            }
        }
        lines.push(line);
    }
    let mut a = Art { lines };
    a.fit();
    a
}

impl Art {
    /// From a text file (a ".txt" of the "Art ASCII" library).
    pub fn from_text(t: &str) -> Art {
        let mut a = Art { lines: t.lines().map(str::to_string).collect() };
        a.fit();
        a
    }

    /// Upper case, forbidden characters become spaces, 40 columns, 24
    /// rows, the 24th row 39 columns, trailing spaces and blank last rows
    /// removed.
    pub fn fit(&mut self) {
        self.lines.truncate(ROWS);
        for l in &mut self.lines {
            let s: String = l
                .chars()
                .map(|c| c.to_ascii_uppercase())
                .map(|c| if c.is_ascii() && allowed(c as u8) { c } else { ' ' })
                .take(COLS)
                .collect();
            *l = s.trim_end().to_string();
        }
        if self.lines.len() == ROWS {
            let l = &mut self.lines[ROWS - 1];
            l.truncate(COLS - 1);
            *l = l.trim_end().to_string();
        }
        while self.lines.last().is_some_and(|l| l.is_empty()) {
            self.lines.pop();
        }
    }

    pub fn rows(&self) -> usize {
        self.lines.len()
    }

    pub fn text(&self) -> String {
        let mut s = self.lines.join("\n");
        s.push('\n');
        s
    }

    /// 24 rows: the program waits for a key before giving the prompt back
    /// (the prompt would scroll the picture).
    pub fn full(&self) -> bool {
        self.lines.len() == ROWS
    }

    /// The characters sent to the terminal (CR = $0D), without a CR after
    /// a full row (automatic return) nor after the last row.
    pub fn stream(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for (i, l) in self.lines.iter().enumerate() {
            out.extend(l.bytes());
            if i + 1 < self.lines.len() && l.len() < COLS {
                out.push(0x0D);
            }
        }
        out
    }
}

/// A name for a REM / comment: upper case, allowed characters.
fn clean_title(t: &str) -> String {
    t.chars()
        .map(|c| c.to_ascii_uppercase())
        .map(|c| if c.is_ascii() && allowed(c as u8) { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(30)
        .collect()
}

/// Integer BASIC: one PRINT per row ("…"; when the row is full: the
/// terminal returns by itself), with 24 rows a key wait at the end.
pub fn to_basic(art: &Art, title: &str) -> String {
    let mut s = String::new();
    let t = clean_title(title);
    s.push_str(&format!("10 REM {}\n", if t.is_empty() { "ASCII ART".into() } else { t }));
    let mut n = 100;
    let last = art.lines.len().saturating_sub(1);
    for (i, l) in art.lines.iter().enumerate() {
        if l.is_empty() {
            s.push_str(&format!("{n} PRINT\n"));
        } else {
            // a full row returns by itself; the last row keeps the cursor
            // on its line (the prompt then takes the next one)
            let semi = l.len() >= COLS || i == last;
            s.push_str(&format!("{n} PRINT \"{l}\"{}\n", if semi { ";" } else { "" }));
        }
        n += 10;
    }
    if art.full() {
        s.push_str(&format!("{n} IF PEEK(-12271)<128 THEN {n}\n"));
        s.push_str(&format!("{} K=PEEK(-12272)\n", n + 10));
        n += 20;
    }
    s.push_str(&format!("{n} END\n"));
    s
}

/// Bytes of the table: $80 + character, or count (2-127) then the
/// character, 0 at the end. Starts with 24 CR (clears the screen).
pub fn rle(art: &Art) -> Vec<u8> {
    // 24 CR first: the screen is cleared and the art starts on the top row
    let mut st = vec![0x0D; ROWS];
    st.extend(art.stream());
    let mut out = Vec::new();
    let mut i = 0;
    while i < st.len() {
        let c = st[i];
        let mut n = 1;
        while i + n < st.len() && st[i + n] == c && n < 127 {
            n += 1;
        }
        if n >= 3 {
            out.push(n as u8);
            out.push(c | 0x80);
            i += n;
        } else {
            out.push(c | 0x80);
            i += 1;
        }
    }
    out.push(0);
    out
}

/// 6502 source (ca65 syntax, also for the built-in assembler): reads the
/// table and prints with ECHO ($FFEF); with 24 rows waits for a key; then
/// back to the Woz monitor. Start: 300R.
pub fn to_asm(art: &Art, title: &str) -> String {
    let t = clean_title(title);
    let data = rle(art);
    let mut s = format!(
        "; {} - ASCII art {}x{} for the Apple-1 (Retro Dev Kit Apple 1)\n\
; Start: 300R. Table: $80+character, or a count then the character, 0 = end.\n\
\n\
ECHO    = $FFEF         ; Woz monitor: print A\n\
KBD     = $D010\n\
KBDCR   = $D011\n\
MONITOR = $FF1F         ; Woz monitor: GETLINE\n\
PTR     = $06           ; table pointer (zero page)\n\
\n\
        .org $0300\n\
START:  lda #<ART\n\
        sta PTR\n\
        lda #>ART\n\
        sta PTR+1\n\
        ldy #0\n\
LOOP:   jsr NEXT\n\
        beq DONE        ; 0: end\n\
        bmi ONE         ; bit 7: one character\n\
        tax             ; count\n\
        jsr NEXT        ; then the character\n\
REP:    jsr ECHO        ; (ECHO keeps X)\n\
        dex\n\
        bne REP\n\
        beq LOOP\n\
ONE:    jsr ECHO\n\
        jmp LOOP\n\
DONE:\n",
        if t.is_empty() { "ASCII ART" } else { &t },
        COLS,
        art.rows()
    );
    if art.full() {
        s.push_str("WAIT:   lda KBDCR       ; 24 rows: wait for a key\n        bpl WAIT\n        lda KBD\n");
    }
    s.push_str(
        "        jmp MONITOR\n\
\n\
NEXT:   lda (PTR),y     ; next byte of the table, flags set\n\
        inc PTR\n\
        bne NEXT1\n\
        inc PTR+1\n\
NEXT1:  cmp #0\n\
        rts\n\
\n\
ART:\n",
    );
    for ch in data.chunks(16) {
        let b: Vec<String> = ch.iter().map(|b| format!("${b:02X}")).collect();
        s.push_str(&format!("        .byte {}\n", b.join(",")));
    }
    s
}

/// What the screen should show after the program: the rows of the art at
/// the top of a 40x24 screen (for the tests).
pub fn expected_screen(art: &Art) -> Vec<String> {
    (0..ROWS).map(|r| art.lines.get(r).cloned().unwrap_or_default()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gradient(w: usize, h: usize) -> Grey {
        Grey { w, h, px: (0..w * h).map(|i| (i % w) as f32 / (w - 1) as f32).collect() }
    }

    #[test]
    fn ramp_is_clean_and_sorted() {
        let r = clean_ramp("ax\"_@. ");
        assert_eq!(r[0], b' ');
        assert!(!r.contains(&b'"') && !r.contains(&b'_'));
        assert!(r.contains(&b'A') && r.contains(&b'X'));
        assert!(r.windows(2).all(|w| density(w[0]) < density(w[1])));
        assert_eq!(*clean_ramp(RAMP_FULL).last().unwrap(), b'@');
    }

    #[test]
    fn gradient_goes_from_blank_to_dense() {
        let a = convert(&gradient(400, 300), &Params { rows: 23, ..Params::default() });
        assert_eq!(a.rows(), 23);
        let l = &a.lines[10];
        assert_eq!(l.len(), 40);
        assert!(l.starts_with(' ') && l.ends_with('@'), "{l}");
        let d: Vec<u8> = l.bytes().map(density).collect();
        assert!(d.windows(2).all(|w| w[0] <= w[1]), "{l}");
    }

    #[test]
    fn row_24_keeps_its_last_column_blank() {
        let img = Grey { w: 10, h: 10, px: vec![1.0; 100] };
        let a = convert(&img, &Params::default());
        assert_eq!(a.rows(), 24);
        assert_eq!(a.lines[0].len(), 40);
        assert_eq!(a.lines[23].len(), 39);
        assert_eq!(a.stream().iter().filter(|&&c| c == 0x0D).count(), 0);
    }

    #[test]
    fn dither_and_invert() {
        let img = Grey { w: 40, h: 24, px: vec![0.5; 960] };
        let p = Params { dither: true, ramp: " @".into(), rows: 10, ..Params::default() };
        let a = convert(&img, &p);
        let at = a.text().matches('@').count();
        assert!((150..250).contains(&at), "{at}");
        let inv = convert(&Grey { w: 4, h: 4, px: vec![1.0; 16] }, &Params { invert: true, ..Params::default() });
        assert!(inv.lines.is_empty());
    }

    #[test]
    fn crop_has_the_screen_shape() {
        let img = gradient(1000, 500);
        let (x, y, w, h) = crop(&img, &Params::default());
        assert!((w / h - 40.0 * 0.8 / 24.0).abs() < 0.01);
        assert_eq!((x, y, h), (500.0 - w / 2.0, 0.0, 500.0));
        let (x, _, w2, _) = crop(&img, &Params { zoom: 2.0, cx: 0.0, ..Params::default() });
        assert_eq!((x, w2), (0.0, w / 2.0));
    }

    #[test]
    fn text_is_fitted() {
        let a = Art::from_text("hello \"woz\"_\n\n");
        assert_eq!(a.lines, ["HELLO  WOZ"]);
    }

    #[test]
    fn basic_has_no_quote_inside_and_full_rows_end_with_semicolon() {
        let a = Art { lines: vec!["#".repeat(40), "AB".into(), String::new(), "C".into()] };
        let b = to_basic(&a, "Test \"x\"");
        let l: Vec<&str> = b.lines().collect();
        assert_eq!(l[0], "10 REM TEST X");
        assert_eq!(l[1], format!("100 PRINT \"{}\";", "#".repeat(40)));
        assert_eq!(l[2], "110 PRINT \"AB\"");
        assert_eq!(l[3], "120 PRINT");
        assert_eq!(l[4], "130 PRINT \"C\";");
        assert_eq!(l[5], "140 END");
        for l in b.lines() {
            assert!(l.len() < 60);
        }
    }

    #[test]
    fn rle_round_trip() {
        let a = Art { lines: vec!["    ***".into(), "X".repeat(40), "Y".into()] };
        let r = rle(&a);
        let mut out = Vec::new();
        let mut i = 0;
        while r[i] != 0 {
            if r[i] & 0x80 != 0 {
                out.push(r[i] & 0x7F);
                i += 1;
            } else {
                out.extend(std::iter::repeat_n(r[i + 1] & 0x7F, r[i] as usize));
                i += 2;
            }
        }
        let mut want = vec![0x0D; ROWS];
        want.extend(a.stream());
        assert_eq!(out, want);
        assert!(r.len() < want.len());
    }

    #[test]
    fn asm_assembles() {
        let a = Art { lines: vec!["HELLO WOZ".into(); 24] };
        let src = to_asm(&a, "hello");
        let asm = crate::asm6502::assemble_text(&src, None, &[]).unwrap();
        assert_eq!(asm.origin, 0x300);
        assert!(src.contains("WAIT:"));
        assert!(asm.bytes.ends_with(&[0]));
    }
}
