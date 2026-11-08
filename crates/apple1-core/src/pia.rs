//! Motorola MC6821 PIA as wired on the Apple-1.
//!
//! * Port A: ASCII keyboard. PA0-PA6 = key code, PA7 tied high, CA1 = key
//!   strobe. `KBDCR` ($D011) bit 7 (IRQA1 flag) goes high when a key is
//!   available and is cleared by reading `KBD` ($D010).
//! * Port B: display. PB0-PB6 = character to display, PB7 (input) = display
//!   busy ("RDA" not ready). Writing `DSP` ($D012) sends a character.

#[derive(Debug, Clone, Default)]
pub struct Pia {
    pub ora: u8,
    pub ddra: u8,
    pub cra: u8,
    pub orb: u8,
    pub ddrb: u8,
    pub crb: u8,
    /// Value present on the port A pins (keyboard).
    pub pa_in: u8,
    /// Level of PB7 (display busy).
    pub pb7_busy: bool,
}

/// What a register access did (for the board to react).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PiaEvent {
    None,
    /// A character was written to the output register B.
    DisplayWrite(u8),
}

impl Pia {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        let pa = self.pa_in;
        *self = Pia::default();
        self.pa_in = pa;
    }

    /// Key strobe on CA1: latch the code and raise IRQA1.
    pub fn key_strobe(&mut self, ascii: u8) {
        self.pa_in = ascii | 0x80; // PA7 tied high on the Apple-1
        self.cra |= 0x80;
    }

    pub fn key_available(&self) -> bool {
        self.cra & 0x80 != 0
    }

    /// Side-effect free view of a register.
    pub fn peek(&self, reg: u8) -> u8 {
        match reg & 3 {
            0 => {
                if self.cra & 0x04 != 0 {
                    (self.ora & self.ddra) | (self.pa_in & !self.ddra)
                } else {
                    self.ddra
                }
            }
            1 => self.cra,
            2 => {
                if self.crb & 0x04 != 0 {
                    let pins = if self.pb7_busy { 0x80 } else { 0x00 };
                    (self.orb & self.ddrb) | (pins & !self.ddrb)
                } else {
                    self.ddrb
                }
            }
            _ => self.crb,
        }
    }

    pub fn read(&mut self, reg: u8) -> u8 {
        let v = self.peek(reg);
        match reg & 3 {
            0 if self.cra & 0x04 != 0 => self.cra &= 0x3F, // clear IRQA1/IRQA2
            2 if self.crb & 0x04 != 0 => self.crb &= 0x3F,
            _ => {}
        }
        v
    }

    pub fn write(&mut self, reg: u8, v: u8) -> PiaEvent {
        match reg & 3 {
            0 => {
                if self.cra & 0x04 != 0 {
                    self.ora = v
                } else {
                    self.ddra = v
                }
                PiaEvent::None
            }
            1 => {
                self.cra = (self.cra & 0xC0) | (v & 0x3F);
                PiaEvent::None
            }
            2 => {
                if self.crb & 0x04 != 0 {
                    self.orb = v;
                    PiaEvent::DisplayWrite(v & 0x7F)
                } else {
                    self.ddrb = v;
                    PiaEvent::None
                }
            }
            _ => {
                self.crb = (self.crb & 0xC0) | (v & 0x3F);
                PiaEvent::None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn keyboard_bit7_semantics() {
        let mut p = Pia::new();
        // Woz monitor init: DDRB=$7F, CRA=CRB=$A7
        p.write(2, 0x7F);
        p.write(1, 0xA7);
        p.write(3, 0xA7);
        assert_eq!(p.read(1) & 0x80, 0);
        p.key_strobe(b'A');
        assert_eq!(p.read(1) & 0x80, 0x80);
        assert_eq!(p.read(0), 0xC1);
        assert_eq!(p.read(1) & 0x80, 0, "reading KBD clears the flag");
        // display busy shows on bit 7 of DSP
        p.pb7_busy = true;
        assert_eq!(p.read(2) & 0x80, 0x80);
        p.pb7_busy = false;
        assert_eq!(p.read(2) & 0x80, 0);
        assert_eq!(p.write(2, 0xC1), PiaEvent::DisplayWrite(0x41));
    }
}
