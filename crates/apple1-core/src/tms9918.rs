//! P-LAB "Apple-1 Graphic Card": a Texas Instruments TMS9918A video display
//! processor (VDP) with its own 16 KB of video RAM, plus the optional
//! "CodeTank" EEPROM daughterboard ($4000-$7FFF).
//!
//! Sources
//! - P-LAB, "Apple-1 Graphic Card" description, rev. 0.4 (English PDF on
//!   <https://p-l4b.github.io/graphic/>): memory map, jumpers, NTSC output.
//! - nippur72/apple1-videocard-lib (README, `lib/tms9918.h`): register
//!   layout and the programming model used by the demos.
//! - TMS9918A data manual (Texas Instruments, 1982): modes, sprites, status.
//!
//! Documented by P-LAB: the VDP occupies `$CC00-$CFFF`; address `$CC00` is
//! the data port and `$CC01` the control/status port. The IRQ/NMI jumper
//! routes the VDP interrupt (default: IRQ).
//!
//! NOT documented by P-LAB (assumptions made here, to be confirmed):
//! - how the 1 KB window mirrors: we decode only A0 (even = data, odd =
//!   control), so every pair of addresses in `$CC00-$CFFF` is the same VDP;
//! - the exact bus timing (the VDP is asynchronous; no wait states are
//!   modelled, accesses are instantaneous);
//! - reset behaviour: the card has no reset line to the VDP in the
//!   documentation; at power-up we start with blank VRAM and zeroed
//!   registers (real VRAM contents are random);
//! - whether the CodeTank EEPROM is write-protected from the bus (we make
//!   it read-only) and what the two 16 KB pages/jumpers look like from the
//!   software point of view (we map one 16 KB image at `$4000`).
//!
//! Not emulated: mid-frame register changes (the frame is drawn from the
//! state at the moment it is displayed), the 4 KB DRAM addressing mode
//! (R1 bit 7 = 0; the card has SRAM), exact sprite timing, the external
//! video input, composite artefacts of the NTSC signal.

/// CPU cycles per video frame (262 lines x 342 pixel clocks / 5.369 MHz at
/// 1.022727 MHz).
pub const FRAME_CYCLES: u64 = 17067;
/// The frame flag is set when the beam leaves the 192 visible lines.
const VBLANK_START: u64 = FRAME_CYCLES * 192 / 262;

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 192;

/// TMS9918A palette (index 0 = transparent, displayed as the backdrop).
pub const PALETTE: [[u8; 3]; 16] = [
    [0, 0, 0],
    [0, 0, 0],
    [33, 200, 66],
    [94, 220, 120],
    [84, 85, 237],
    [125, 118, 252],
    [212, 82, 77],
    [66, 235, 245],
    [252, 85, 84],
    [255, 121, 120],
    [212, 193, 84],
    [230, 206, 128],
    [33, 176, 59],
    [201, 91, 186],
    [204, 204, 204],
    [255, 255, 255],
];

#[derive(Clone, PartialEq, Eq)]
pub struct Vdp {
    pub vram: Box<[u8; 16384]>,
    pub reg: [u8; 8],
    pub status: u8,
    pub addr: u16,
    latch: Option<u8>,
    read_buf: u8,
    /// Cycle of the next vertical blank.
    next_vblank: u64,
    /// IRQ/NMI jumper: true = NMI, false = IRQ (default).
    pub nmi_jumper: bool,
    /// Number of frames since power-up.
    pub frames: u64,
}

impl std::fmt::Debug for Vdp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vdp").field("reg", &self.reg).field("status", &self.status).field("addr", &self.addr).finish()
    }
}

impl Default for Vdp {
    fn default() -> Self {
        Self::new()
    }
}

/// Decode a CPU address in `$CC00-$CFFF`: `Some(false)` = data port, `Some(true)` = control port.
pub fn decode(addr: u16) -> Option<bool> {
    if (0xCC00..=0xCFFF).contains(&addr) {
        Some(addr & 1 != 0)
    } else {
        None
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Graphics1,
    Graphics2,
    Multicolor,
    Text,
}

impl Mode {
    pub fn name(self) -> &'static str {
        match self {
            Mode::Graphics1 => "Graphics I",
            Mode::Graphics2 => "Graphics II",
            Mode::Multicolor => "Multicolor",
            Mode::Text => "Text",
        }
    }
}

impl Vdp {
    pub fn new() -> Self {
        Vdp { vram: Box::new([0; 16384]), reg: [0; 8], status: 0, addr: 0, latch: None, read_buf: 0, next_vblank: VBLANK_START, nmi_jumper: false, frames: 0 }
    }

    pub fn mode(&self) -> Mode {
        if self.reg[1] & 0x10 != 0 {
            Mode::Text
        } else if self.reg[1] & 0x08 != 0 {
            Mode::Multicolor
        } else if self.reg[0] & 0x02 != 0 {
            Mode::Graphics2
        } else {
            Mode::Graphics1
        }
    }

    pub fn display_on(&self) -> bool {
        self.reg[1] & 0x40 != 0
    }
    pub fn interrupt_enabled(&self) -> bool {
        self.reg[1] & 0x20 != 0
    }
    pub fn backdrop(&self) -> u8 {
        self.reg[7] & 0x0F
    }

    /// Level of the interrupt line (frame flag AND interrupt enable).
    pub fn int_level(&self) -> bool {
        self.status & 0x80 != 0 && self.interrupt_enabled()
    }

    /// Advance the frame clock. Returns true when a new frame flag was raised.
    pub fn advance(&mut self, cycles: u64) -> bool {
        let mut raised = false;
        while cycles >= self.next_vblank {
            self.next_vblank += FRAME_CYCLES;
            self.frames += 1;
            self.status |= 0x80;
            self.eval_sprites();
            raised = true;
        }
        raised
    }

    pub fn read_data(&mut self) -> u8 {
        self.latch = None;
        let v = self.read_buf;
        self.read_buf = self.vram[self.addr as usize & 0x3FFF];
        self.addr = (self.addr + 1) & 0x3FFF;
        v
    }

    pub fn write_data(&mut self, v: u8) {
        self.latch = None;
        self.vram[self.addr as usize & 0x3FFF] = v;
        self.read_buf = v;
        self.addr = (self.addr + 1) & 0x3FFF;
    }

    pub fn read_status(&mut self) -> u8 {
        self.latch = None;
        let s = self.status;
        self.status &= 0x1F;
        s
    }

    pub fn write_control(&mut self, v: u8) {
        match self.latch.take() {
            None => self.latch = Some(v),
            Some(lo) => {
                if v & 0x80 != 0 {
                    self.reg[(v & 7) as usize] = lo;
                } else {
                    self.addr = ((v as u16 & 0x3F) << 8) | lo as u16;
                    if v & 0x40 == 0 {
                        self.read_buf = self.vram[self.addr as usize];
                        self.addr = (self.addr + 1) & 0x3FFF;
                    }
                }
            }
        }
    }

    /// Side effect free status (debugger).
    pub fn peek_status(&self) -> u8 {
        self.status
    }

    fn sprite_size(&self) -> (i32, bool) {
        let big = self.reg[1] & 0x02 != 0;
        let mag = self.reg[1] & 0x01 != 0;
        let s = if big { 16 } else { 8 };
        (if mag { s * 2 } else { s }, big)
    }

    /// Sprite coverage per line for the frame: for each line the list of
    /// (sprite index) in priority order (max 4) and the flags.
    fn sprite_lines(&self) -> (Vec<Vec<usize>>, Option<(usize, u8)>, bool) {
        let mut lines: Vec<Vec<usize>> = vec![Vec::new(); HEIGHT];
        let mut fifth: Option<(usize, u8)> = None;
        let base = (self.reg[5] as usize & 0x7F) * 0x80;
        let (size, _) = self.sprite_size();
        let mut n = 0;
        while n < 32 {
            let y = self.vram[(base + n * 4) & 0x3FFF];
            if y == 0xD0 {
                break;
            }
            n += 1;
        }
        let mut collision = false;
        let mut masks: Vec<Vec<[u64; 4]>> = vec![Vec::new(); HEIGHT];
        for i in 0..n {
            let a = base + i * 4;
            let y = self.vram[a & 0x3FFF];
            let top = if y > 0xE0 { y as i32 - 255 } else { y as i32 + 1 };
            for r in 0..size {
                let line = top + r;
                if !(0..HEIGHT as i32).contains(&line) {
                    continue;
                }
                let l = line as usize;
                if lines[l].len() < 4 {
                    lines[l].push(i);
                    let m = self.sprite_row_bits(i, r);
                    if masks[l].iter().any(|o| o.iter().zip(&m).any(|(p, q)| p & q != 0)) {
                        collision = true;
                    }
                    masks[l].push(m);
                } else if fifth.is_none() {
                    fifth = Some((i, 0));
                }
            }
        }
        (lines, fifth, collision)
    }

    /// Opaque pixels of row `r` of sprite `i` as a 256-bit mask in screen coordinates.
    fn sprite_row_bits(&self, i: usize, r: i32) -> [u64; 4] {
        let a = (self.reg[5] as usize & 0x7F) * 0x80 + i * 4;
        let x0 = self.vram[(a + 1) & 0x3FFF] as i32;
        let ec = self.vram[(a + 3) & 0x3FFF] & 0x80 != 0;
        let x0 = if ec { x0 - 32 } else { x0 };
        let (_, big) = self.sprite_size();
        let mag = self.reg[1] & 0x01 != 0;
        let rr = if mag { r / 2 } else { r } as usize;
        let name = self.vram[(a + 2) & 0x3FFF] as usize & if big { 0xFC } else { 0xFF };
        let pbase = (self.reg[6] as usize & 7) * 0x800 + name * 8;
        let mut m = [0u64; 4];
        let w = if big { 16 } else { 8 };
        for px in 0..w {
            let (off, bit) = if px < 8 { (rr, 7 - px) } else { (rr + 16, 15 - px) };
            if self.vram[(pbase + off) & 0x3FFF] >> bit & 1 == 0 {
                continue;
            }
            for k in 0..(if mag { 2 } else { 1 }) {
                let x = x0 + px * if mag { 2 } else { 1 } + k;
                if (0..WIDTH as i32).contains(&x) {
                    m[x as usize / 64] |= 1u64 << (x % 64);
                }
            }
        }
        m
    }

    /// Update the 5S and C status bits for the frame (called at vblank).
    fn eval_sprites(&mut self) {
        let base = (self.reg[5] as usize & 0x7F) * 0x80;
        if self.vram[base & 0x3FFF] == 0xD0 || !self.display_on() || self.mode() == Mode::Text {
            return;
        }
        let (_, fifth, collision) = self.sprite_lines();
        if collision {
            self.status |= 0x20;
        }
        if let Some((i, _)) = fifth {
            if self.status & 0x40 == 0 {
                self.status = (self.status & 0xE0) | 0x40 | i as u8;
            }
        }
    }

    /// Render the 256x192 picture as RGBA. Text mode (240x192) is centred
    /// with an 8-pixel backdrop border on both sides.
    pub fn render_rgba(&self) -> Vec<u8> {
        let idx = self.render_indices();
        let mut out = Vec::with_capacity(WIDTH * HEIGHT * 4);
        for c in idx {
            let p = PALETTE[c as usize & 15];
            out.extend_from_slice(&[p[0], p[1], p[2], 255]);
        }
        out
    }

    /// Palette indices (0 = transparent already resolved to the backdrop).
    pub fn render_indices(&self) -> Vec<u8> {
        let bd = self.backdrop();
        let mut img = vec![bd; WIDTH * HEIGHT];
        if !self.display_on() {
            return img;
        }
        let mode = self.mode();
        let name_base = (self.reg[2] as usize & 0x0F) * 0x400;
        let v = |a: usize| self.vram[a & 0x3FFF];
        let resolve = |c: u8| if c == 0 { bd } else { c };
        for y in 0..HEIGHT {
            let row = &mut img[y * WIDTH..(y + 1) * WIDTH];
            match mode {
                Mode::Graphics1 => {
                    let pbase = (self.reg[4] as usize & 7) * 0x800;
                    let cbase = self.reg[3] as usize * 0x40;
                    for col in 0..32 {
                        let n = v(name_base + (y / 8) * 32 + col) as usize;
                        let pat = v(pbase + n * 8 + (y & 7));
                        let c = v(cbase + (n >> 3));
                        for b in 0..8 {
                            let on = pat >> (7 - b) & 1 != 0;
                            row[col * 8 + b] = resolve(if on { c >> 4 } else { c & 15 });
                        }
                    }
                }
                Mode::Graphics2 => {
                    let third = y / 64;
                    let pbase = (self.reg[4] as usize & 0x04) << 11;
                    let pmask = ((self.reg[4] as usize & 3) << 11) | 0x7FF;
                    let cbase = (self.reg[3] as usize & 0x80) << 6;
                    let cmask = ((self.reg[3] as usize & 0x7F) << 6) | 0x3F;
                    for col in 0..32 {
                        let n = v(name_base + (y / 8) * 32 + col) as usize;
                        let off = ((third << 8 | n) * 8 + (y & 7)) & 0x1FFF;
                        let pat = v(pbase + (off & pmask));
                        let c = v(cbase + (off & cmask));
                        for b in 0..8 {
                            let on = pat >> (7 - b) & 1 != 0;
                            row[col * 8 + b] = resolve(if on { c >> 4 } else { c & 15 });
                        }
                    }
                }
                Mode::Multicolor => {
                    let pbase = (self.reg[4] as usize & 7) * 0x800;
                    for col in 0..32 {
                        let n = v(name_base + (y / 8) * 32 + col) as usize;
                        let c = v(pbase + n * 8 + ((y / 8) & 3) * 2 + ((y & 7) >> 2));
                        for b in 0..8 {
                            row[col * 8 + b] = resolve(if b < 4 { c >> 4 } else { c & 15 });
                        }
                    }
                }
                Mode::Text => {
                    let pbase = (self.reg[4] as usize & 7) * 0x800;
                    let fg = resolve(self.reg[7] >> 4);
                    for col in 0..40 {
                        let n = v(name_base + (y / 8) * 40 + col) as usize;
                        let pat = v(pbase + n * 8 + (y & 7));
                        for b in 0..6 {
                            if pat >> (7 - b) & 1 != 0 {
                                row[8 + col * 6 + b] = fg;
                            }
                        }
                    }
                }
            }
        }
        if mode != Mode::Text {
            self.draw_sprites(&mut img);
        }
        img
    }

    fn draw_sprites(&self, img: &mut [u8]) {
        let base = (self.reg[5] as usize & 0x7F) * 0x80;
        let (size, big) = self.sprite_size();
        let mag = self.reg[1] & 0x01 != 0;
        let (lines, _, _) = self.sprite_lines();
        for (y, list) in lines.iter().enumerate() {
            // lower index = higher priority: draw from the back
            for &i in list.iter().rev() {
                let a = base + i * 4;
                let sy = self.vram[a & 0x3FFF];
                let top = if sy > 0xE0 { sy as i32 - 255 } else { sy as i32 + 1 };
                let r = y as i32 - top;
                if !(0..size).contains(&r) {
                    continue;
                }
                let sx = self.vram[(a + 1) & 0x3FFF] as i32;
                let name = self.vram[(a + 2) & 0x3FFF] as usize & if big { 0xFC } else { 0xFF };
                let attr = self.vram[(a + 3) & 0x3FFF];
                let color = attr & 15;
                let x0 = if attr & 0x80 != 0 { sx - 32 } else { sx };
                if color == 0 {
                    continue;
                }
                let rr = if mag { r / 2 } else { r } as usize;
                let pbase = (self.reg[6] as usize & 7) * 0x800 + name * 8;
                let w = if big { 16 } else { 8 };
                for px in 0..w {
                    let (off, bit) = if px < 8 { (rr, 7 - px) } else { (rr + 16, 15 - px) };
                    if self.vram[(pbase + off) & 0x3FFF] >> bit & 1 == 0 {
                        continue;
                    }
                    for k in 0..(if mag { 2 } else { 1 }) {
                        let x = x0 + px * if mag { 2 } else { 1 } + k;
                        if (0..WIDTH as i32).contains(&x) {
                            img[y * WIDTH + x as usize] = color;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine::{Config, Machine};

    fn vdp_reg(m: &mut Machine, r: u8, v: u8) {
        use crate::cpu::Bus;
        m.board.write(0xCC01, v);
        m.board.write(0xCC01, 0x80 | r);
    }
    fn vdp_addr(m: &mut Machine, a: u16, write: bool) {
        use crate::cpu::Bus;
        m.board.write(0xCC01, a as u8);
        m.board.write(0xCC01, (a >> 8) as u8 & 0x3F | if write { 0x40 } else { 0 });
    }

    #[test]
    fn vram_write_read_and_registers() {
        use crate::cpu::Bus;
        let mut m = Machine::new(Config { plab_card: true, ..Config::default() });
        vdp_addr(&mut m, 0x1234, true);
        m.board.write(0xCC00, 0x5A);
        m.board.write(0xCC00, 0xA5);
        assert_eq!(m.board.vdp.vram[0x1234], 0x5A);
        assert_eq!(m.board.vdp.vram[0x1235], 0xA5);
        vdp_addr(&mut m, 0x1234, false);
        assert_eq!(m.board.read(0xCC00), 0x5A);
        assert_eq!(m.board.read(0xCC00), 0xA5);
        vdp_reg(&mut m, 7, 0x1F);
        assert_eq!(m.board.vdp.reg[7], 0x1F);
        // mirrors in the 1 KB window
        m.board.write(0xCFFF, 0x82);
        m.board.write(0xCFFF, 0x80 | 3);
        assert_eq!(m.board.vdp.reg[3], 0x82);
    }

    #[test]
    fn frame_flag_and_irq() {
        use crate::cpu::Bus;
        let mut m = Machine::new(Config { plab_card: true, ..Config::default() });
        m.run_cycles(FRAME_CYCLES * 2);
        assert!(m.board.read(0xCC01) & 0x80 != 0, "frame flag set");
        assert_eq!(m.board.read(0xCC01) & 0x80, 0, "cleared by the read");
        vdp_reg(&mut m, 1, 0x60);
        m.run_cycles(FRAME_CYCLES * 2);
        assert!(m.board.vdp_irq, "IRQ requested when IE is on");
        m.board.read(0xCC01);
        assert!(!m.board.vdp_irq);
    }

    #[test]
    fn graphics2_pixel_and_sprite() {
        let mut v = Vdp::new();
        v.reg = [0x02, 0xC0, 0x0E, 0xFF, 0x03, 0x76, 0x03, 0x04];
        // pattern/colour of tile 0: first row 10000000, white on black
        v.vram[0] = 0x80;
        v.vram[0x2000] = 0xF1;
        let img = v.render_indices();
        assert_eq!(img[0], 15);
        assert_eq!(img[1], 1);
        assert_eq!(img[WIDTH * 100 + 100], 4 /* backdrop? colour table is 0 -> transparent */);
    }

    #[test]
    fn sprite_collision() {
        let mut v = Vdp::new();
        v.reg = [0, 0xC0, 0x0E, 0x80, 0x00, 0x76, 0x03, 0x01];
        v.vram[0x1800..0x1808].fill(0xFF);
        let sa = 0x3B00;
        v.vram[sa..sa + 8].copy_from_slice(&[10, 10, 0, 6, 12, 14, 0, 9]);
        v.vram[sa + 8] = 0xD0;
        v.advance(FRAME_CYCLES);
        assert!(v.status & 0x20 != 0, "collision");
        v.vram[sa + 5] = 100;
        v.status = 0;
        v.advance(FRAME_CYCLES * 2);
        assert_eq!(v.status & 0x20, 0);
    }
}
