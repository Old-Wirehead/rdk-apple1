//! Byte-for-byte comparison of the embedded ROMs with Wirehead's PROM dumps
//! (`apple1/hardware/roms/dumps/`, outside this crate: skipped if absent),
//! and custom ROM images (e.g. a 512 x 4 PROM page) on the bus.

use std::path::PathBuf;

use apple1_core::cpu::Bus;
use apple1_core::roms::{self, prom, ACI_ROM, WOZ_MONITOR};
use apple1_core::{Config, Machine};

fn dumps() -> Option<PathBuf> {
    let d = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../hardware/roms/dumps");
    if d.is_dir() {
        Some(d)
    } else {
        eprintln!("dossier de vidages absent ({}) : test ignoré", d.display());
        None
    }
}

#[test]
fn embedded_roms_match_wirehead_dumps() {
    let Some(d) = dumps() else { return };
    let rd = |p: &str| std::fs::read(d.join(p)).unwrap_or_else(|e| panic!("{p}: {e}"));
    let p = |n: u8| rd(&format!("proms_A1-A4/APPLE-A{n}.BIN"));
    assert_eq!(&prom::join(&p(2), &p(1)).unwrap(), WOZ_MONITOR, "A2 (haut) + A1 (bas) != Woz Monitor");
    assert_eq!(&prom::join(&p(3), &p(4)).unwrap(), ACI_ROM, "A3 (haut) + A4 (bas) != ROM ACI");
    assert_eq!(rd("proms_A1-A4/ACI_C100_reconstitue_A3hi_A4lo.bin"), ACI_ROM.to_vec());
    assert_eq!(rd("emulateur_2001/apple1.rom"), WOZ_MONITOR.to_vec());
    // and the splitter reproduces the dump files exactly
    let (hi, lo) = prom::split(WOZ_MONITOR);
    assert_eq!((hi.to_vec(), lo.to_vec()), (p(2), p(1)));
    let (hi, lo) = prom::split(ACI_ROM);
    assert_eq!((hi.to_vec(), lo.to_vec()), (p(3), p(4)));
}

#[test]
fn custom_rom_pages_are_on_the_bus() {
    let mut big = vec![0u8; 512];
    big[..256].copy_from_slice(ACI_ROM);
    for (i, b) in big[256..].iter_mut().enumerate() {
        *b = i as u8 ^ 0x5A;
    }
    let page1 = roms::rom_page(&big, 1).unwrap();
    let mut m = Machine::new(Config { aci_rom: page1, ..Config::default() });
    assert_eq!(m.board.read(0xC100), 0x5A);
    assert_eq!(m.board.read(0xC1FF), 0xFF ^ 0x5A);
    assert_eq!(m.board.read(0xC010), 0x10 ^ 0x5A); // same PROM mirrored at $C000
    // standard page and monitor unchanged
    let mut m = Machine::new(Config { aci_rom: roms::rom_page(&big, 0).unwrap(), ..Config::default() });
    assert_eq!(m.board.read(0xC100), 0xA9);
    assert_eq!(m.board.read(0xFFFC), 0x00);
    assert_eq!(m.board.read(0xFFFD), 0xFF);
}
