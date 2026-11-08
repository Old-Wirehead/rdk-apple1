//! P-LAB graphic card: runs the public demos of nippur72/apple1-videocard-lib
//! when they are present (set PLAB_DIR; the binaries are not redistributable
//! and are not part of the repository). Writes PPM images to PLAB_OUT.

use apple1_core::machine::{Config, Machine};
use std::path::PathBuf;

fn run(file: &str, jukebox: bool, frames: u64) -> Option<Machine> {
    let dir = PathBuf::from(std::env::var("PLAB_DIR").ok()?);
    let bin = std::fs::read(dir.join(file)).ok()?;
    let mut cfg = Config { plab_card: true, ram_kb: 32, ..Config::default() };
    if jukebox {
        cfg.codetank = Some(bin.clone());
    }
    let mut m = Machine::new(cfg);
    m.run_cycles(200_000); // the Woz monitor initialises the PIA
    if jukebox {
        m.cpu.pc = 0x4000;
    } else {
        for (i, b) in bin.iter().enumerate() {
            m.board.poke(0x280 + i as u16, *b);
        }
        m.cpu.pc = 0x280;
    }
    m.run_cycles(apple1_core::tms9918::FRAME_CYCLES * frames);
    Some(m)
}

fn dump(m: &Machine, name: &str) {
    let Ok(out) = std::env::var("PLAB_OUT") else { return };
    let rgba = m.board.vdp.render_rgba();
    let mut ppm = format!("P6\n{} {}\n255\n", apple1_core::tms9918::WIDTH, apple1_core::tms9918::HEIGHT).into_bytes();
    for p in rgba.chunks(4) {
        ppm.extend_from_slice(&p[..3]);
    }
    std::fs::write(PathBuf::from(out).join(name), ppm).unwrap();
}

#[test]
fn demos_draw_something() {
    for (f, jb, name, fr) in [("tetris.bin", false, "tetris.ppm", 300), ("picshow.bin", false, "picshow.ppm", 900)] {
        if let Some(m) = run(f, jb, fr) {
            dump(&m, name);
            let nonzero = m.board.vdp.vram.iter().filter(|&&b| b != 0).count();
            eprintln!("{f}: pc={:04X} reg={:?} nonzero={nonzero}", m.cpu.pc, m.board.vdp.reg);
            assert!(nonzero > 100, "{f}: VRAM stayed empty");
        }
    }
    // the menu demo: one key per screen (jukebox and RAM versions)
    for (f, jb) in [("demo.bin", false), ("demo_jukebox.bin", true)] {
        for key in ["1", "2", "3", "A"] {
            let Some(mut m) = run(f, jb, 30) else { return };
            m.type_text(key);
            m.run_cycles(apple1_core::tms9918::FRAME_CYCLES * 400);
            dump(&m, &format!("demo{}-{key}.ppm", if jb { "jb" } else { "" }));
            let nonzero = m.board.vdp.vram.iter().filter(|&&b| b != 0).count();
            eprintln!("{f} {key}: pc={:04X} reg={:?} nonzero={nonzero}", m.cpu.pc, m.board.vdp.reg);
            assert!(nonzero > 100, "{f} {key}: VRAM stayed empty");
        }
    }
}
