//! The generated programs show the art: the 6502 one with the Woz
//! monitor, the BASIC one with the real Integer BASIC
//! (test-fixtures/BASIC.wav, not in the repository: skipped without it).

use apple1_core::asciiart::{self, Art};
use apple1_core::machine::{Config, Machine};
use apple1_core::{aciwav, asm6502, intbasic, program};

/// An art using every character of the full ramp, full rows and a blank row.
fn art(rows: usize) -> Art {
    let ramp: Vec<u8> = asciiart::clean_ramp(asciiart::RAMP_FULL);
    let lines = (0..rows)
        .map(|r| match r % 6 {
            0 => (0..40).map(|c| ramp[(c + r) % ramp.len()] as char).collect(),
            1 => String::new(),
            2 => "   @@@@@   HELLO WOZ   #####".into(),
            3 => "@".repeat(40),
            _ => format!("{:>w$}", "*", w = 1 + r),
        })
        .collect();
    let mut a = Art { lines };
    a.fit();
    a
}

fn screen(m: &Machine) -> Vec<String> {
    (0..24).map(|r| m.board.display.row_text(r).trim_end().to_string()).collect()
}

fn basic_machine(src: &str) -> Option<Machine> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../test-fixtures/BASIC.wav");
    let b = std::fs::read(path).ok()?;
    let imp = aciwav::import_wav_bytes(&b).unwrap();
    let p = program::wav_to_program(&imp, Some(0xE000)).unwrap();
    let l = intbasic::tokenize(src);
    assert!(l.errors.is_empty(), "{:?}", l.errors);
    let (lo, hi) = intbasic::layout(l.size(), None);
    let img = intbasic::Image::build(&l.lines, lo, hi).unwrap();
    let mut m = Machine::new(Config::default());
    for (a, d) in &p.blocks {
        m.load(*a, d);
    }
    m.reset();
    m.run_cycles(200_000);
    for (a, d) in img.blocks() {
        m.load(a, &d);
    }
    m.type_text("E2B3R\nRUN\n");
    Some(m)
}

#[test]
fn asm_shows_the_art() {
    for rows in [24, 23] {
        let a = art(rows);
        let src = asciiart::to_asm(&a, "test");
        let asm = asm6502::assemble_text(&src, None, &[]).unwrap();
        let mut m = Machine::new(Config::default());
        m.reset();
        m.run_cycles(200_000);
        m.load(asm.origin, &asm.bytes);
        m.type_text("300R\n");
        m.run_cycles(40_000_000);
        assert_eq!(screen(&m), asciiart::expected_screen(&a), "{rows} rows");
        if rows == 24 {
            // waits for a key, then the monitor (a new line: scrolls)
            m.type_text(" ");
            m.run_cycles(2_000_000);
            assert_eq!(m.cpu.pc & 0xFF00, 0xFF00);
        }
    }
}

#[test]
fn basic_shows_the_art() {
    for rows in [24, 23] {
        let a = art(rows);
        let Some(mut m) = basic_machine(&asciiart::to_basic(&a, "test")) else {
            eprintln!("BASIC.wav missing: skipped");
            return;
        };
        m.run_cycles(60_000_000);
        // 23 rows: the prompt on the 24th
        assert_eq!(screen(&m)[..rows], asciiart::expected_screen(&a)[..rows], "{rows} rows");
        if rows == 23 {
            assert_eq!(screen(&m)[23], ">");
        }
    }
}

/// Typed into BASIC (library start of a listing): '@' is kept.
#[test]
fn basic_typed_keeps_at_sign() {
    let Some(mut m) = basic_machine("10 END\n") else {
        return;
    };
    m.run_cycles(5_000_000);
    m.type_text("PRINT \"#@#\"\n");
    m.run_cycles(10_000_000);
    let t = m.screen_text();
    assert!(t.contains("\n#@#"), "{t}");
}

/// Dave Schmenk's Apple1-Slideshow (xa source, not in the repository: a
/// clone in /tmp/sl or $SLIDESHOW): built by the built-in assembler, 280R
/// shows the first picture and its caption.
#[test]
fn schmenk_slideshow_builds_and_runs() {
    let dir = std::env::var("SLIDESHOW").unwrap_or_else(|_| "/tmp/sl".into());
    let src = std::path::Path::new(&dir).join("slideshow.a65");
    if !src.exists() {
        eprintln!("{} missing: skipped", src.display());
        return;
    }
    let a = asm6502::assemble_file(&src, &[]).map_err(|e| e.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n")).unwrap();
    assert_eq!(a.origin, 0x280);
    let mut m = Machine::new(Config { ram_kb: 8, ..Config::default() });
    m.reset();
    m.run_cycles(200_000);
    assert_eq!(m.load(a.origin, &a.bytes), 0);
    m.type_text("280R\n");
    for _ in 0..40 {
        m.run_cycles(1_000_000);
        if screen(&m).iter().any(|l| l.trim() == "WOZ") {
            break;
        }
    }
    let s = screen(&m);
    assert!(s.iter().any(|l| l.trim() == "WOZ"), "{}", s.join("\n"));
    assert!(s.iter().filter(|l| l.contains("@@@@")).count() > 10);
    eprintln!("{} bytes ${:04X}-${:04X}\n{}", a.bytes.len(), a.origin, a.end(), s.join("\n"));
}
