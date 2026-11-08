//! ROM images included with the emulator.
//!
//! * Woz Monitor (256 bytes, $FF00-$FFFF), Steve Wozniak 1976 — as published
//!   in the Apple-1 Operation Manual and reproduced in many places
//!   (e.g. jefftranter/6502 `wozmon.mon`).
//! * Apple Cassette Interface ROM (256 bytes, $C100-$C1FF), Steve Wozniak
//!   1976 — as published by SB-Projects, Applefritter (`wozaci.txt`), the POM1
//!   distribution and jefftranter/6502 `wozaci.mon` (two independent dumps
//!   were compared byte for byte and are identical).
//!
//! Apple Integer BASIC is *not* included (copyright); it can be loaded from a
//! user-supplied file.

pub static WOZ_MONITOR: &[u8; 256] = include_bytes!("roms/wozmon.bin");
pub static ACI_ROM: &[u8; 256] = include_bytes!("roms/aci.bin");

/// Entry points of the ACI ROM routines (used by the cycle exact WAV writer).
pub mod aci {
    pub const START: u16 = 0xC100;
    pub const WRITE: u16 = 0xC170;
    pub const READ: u16 = 0xC18D;
    pub const HEX1L: u16 = 0x24;
    pub const HEX1H: u16 = 0x25;
    pub const HEX2L: u16 = 0x26;
    pub const HEX2H: u16 = 0x27;
    pub const SAVEINDEX: u16 = 0x28;
}

/// Woz monitor entry points.
pub mod wozmon {
    pub const RESET: u16 = 0xFF00;
    pub const ESCAPE: u16 = 0xFF1A;
    pub const GETLINE: u16 = 0xFF1F;
    pub const ECHO: u16 = 0xFFEF;
    pub const PRBYTE: u16 = 0xFFDC;
}

/// 256 x 4 bit bipolar PROMs (Signetics 82S129 / MMI 6301 / 3601...) of the
/// Apple-1: the 8-bit ROM images are split into two nibble PROMs.
///
/// * Woz Monitor: A2 = high nibble (D7-D4), A1 = low nibble (D3-D0).
/// * ACI monitor: A3 = high nibble, A4 = low nibble (on the ACI card).
///
/// A nibble file holds 256 bytes whose upper four bits are zero, in logical
/// address order (the format of the dumps in `hardware/roms/dumps/`).
pub mod prom {
    /// Split an image into (high nibble PROM, low nibble PROM).
    pub fn split(img: &[u8; 256]) -> ([u8; 256], [u8; 256]) {
        let mut hi = [0u8; 256];
        let mut lo = [0u8; 256];
        for i in 0..256 {
            hi[i] = img[i] >> 4;
            lo[i] = img[i] & 0x0F;
        }
        (hi, lo)
    }

    /// Rebuild an 8-bit image from two nibble PROMs (upper bits ignored).
    pub fn join(hi: &[u8], lo: &[u8]) -> Result<[u8; 256], String> {
        if hi.len() != 256 || lo.len() != 256 {
            return Err(format!("une PROM 256 x 4 fait 256 octets (reçu {} et {})", hi.len(), lo.len()));
        }
        let mut img = [0u8; 256];
        for i in 0..256 {
            img[i] = (hi[i] & 0x0F) << 4 | (lo[i] & 0x0F);
        }
        Ok(img)
    }

    /// Exchange two address lines of a PROM image: the byte at logical
    /// address `a` is stored at the address seen by the chip when lines
    /// `x` and `y` are crossed between the CPU bus and the PROM pins.
    /// (Willegal's errata: on the Apple schematic, A1/A7 of PROM A2 are
    /// drawn crossed with respect to the real board.)
    pub fn swap_address_lines(img: &[u8; 256], x: u32, y: u32) -> [u8; 256] {
        let mut out = [0u8; 256];
        for (a, v) in img.iter().enumerate() {
            let bx = (a >> x) & 1;
            let by = (a >> y) & 1;
            let b = (a & !((1 << x) | (1 << y))) | (bx << y) | (by << x);
            out[b] = *v;
        }
        out
    }
}

/// Load a 256-byte ROM page from a user file: a 256-byte image, or a
/// 512-byte image of a larger PROM (AM27S13 / 82S131, 512 x 4 bits) of which
/// `page` 0 (A8 low, the standard firmware) or 1 (A8 high, e.g. Uncle
/// Bernie's hidden pages) is selected.
pub fn rom_page(bytes: &[u8], page: u8) -> Result<[u8; 256], String> {
    let n = bytes.len();
    if n != 256 && n != 512 {
        return Err(format!("image de ROM de {n} octets : 256 ou 512 attendus"));
    }
    let pages = n / 256;
    if page as usize >= pages {
        return Err(format!("page {page} absente (l'image n'a que {pages} page(s))"));
    }
    let mut out = [0u8; 256];
    out.copy_from_slice(&bytes[page as usize * 256..page as usize * 256 + 256]);
    Ok(out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn prom_split_join_swap() {
        let (hi, lo) = prom::split(WOZ_MONITOR);
        assert!(hi.iter().chain(lo.iter()).all(|b| *b < 16));
        assert_eq!(&prom::join(&hi, &lo).unwrap(), WOZ_MONITOR);
        let s = prom::swap_address_lines(WOZ_MONITOR, 1, 7);
        assert_eq!(s[0x80], WOZ_MONITOR[0x02]);
        assert_eq!(s[0x02], WOZ_MONITOR[0x80]);
        assert_eq!(s[0x82], WOZ_MONITOR[0x82]);
        assert_eq!(&prom::swap_address_lines(&s, 1, 7), WOZ_MONITOR);
        let mut big = vec![0xEAu8; 512];
        big[..256].copy_from_slice(ACI_ROM);
        assert_eq!(&rom_page(&big, 0).unwrap(), ACI_ROM);
        assert_eq!(rom_page(&big, 1).unwrap(), [0xEA; 256]);
        assert!(rom_page(&big[..256], 1).is_err() && rom_page(&big[..100], 0).is_err());
    }

    use super::*;
    #[test]
    fn roms_look_right() {
        assert_eq!(&WOZ_MONITOR[0..4], &[0xD8, 0x58, 0xA0, 0x7F]);
        // reset vector -> $FF00, NMI -> $0F00, IRQ -> $0000
        assert_eq!(&WOZ_MONITOR[0xFA..], &[0x00, 0x0F, 0x00, 0xFF, 0x00, 0x00]);
        // ECHO routine
        assert_eq!(&WOZ_MONITOR[0xEF..0xF8], &[0x2C, 0x12, 0xD0, 0x30, 0xFB, 0x8D, 0x12, 0xD0, 0x60]);
        assert_eq!(&ACI_ROM[0..5], &[0xA9, 0xAA, 0x20, 0xEF, 0xFF]);
        assert_eq!(&ACI_ROM[0x70..0x72], &[0xA9, 0x40]); // WRITE: LDA #$40
        assert_eq!(&ACI_ROM[0x8D..0x90], &[0x20, 0xBC, 0xC1]); // READ: JSR FULLCYCLE
        assert_ne!(ACI_ROM[0x80], ACI_ROM[0x81]); // tape-in sensing relies on this
    }
}
