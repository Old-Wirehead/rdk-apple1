//! RDK G2 Basic: the clean-room Integer BASIC clone of the kit (MIT, repository
//! rdk-gen2basic-apple1), built in. 7.9 KB at $E000-$FEFC, with LOMEM=, HIMEM= and KEY.

use crate::program::Program;

/// The binary image, to be loaded at $E000.
pub const IMAGE: &[u8] = include_bytes!("../assets/rdk-g2basic.bin");

/// Load address of the image.
pub const ADDRESS: u16 = 0xE000;

/// The image as a program (one block at $E000).
pub fn program() -> Program {
    Program { blocks: vec![(ADDRESS, IMAGE.to_vec())], run: None, notes: Vec::new() }
}

/// True when RDK G2 Basic (identification bytes "RDK" at $E003) is at $E000.
pub fn present(peek: &dyn Fn(u16) -> u8) -> bool {
    peek(0xE000) == 0x4C && peek(0xE003) == b'R' && peek(0xE004) == b'D' && peek(0xE005) == b'K'
}

#[cfg(test)]
mod tests {
    #[test]
    fn image_is_g2basic() {
        assert!(super::IMAGE.len() > 7000 && super::IMAGE.len() < 0x1F00);
        assert_eq!(&super::IMAGE[..6], &[0x4C, super::IMAGE[1], super::IMAGE[2], b'R', b'D', b'K']);
    }
}
