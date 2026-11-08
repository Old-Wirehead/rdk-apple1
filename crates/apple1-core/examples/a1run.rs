//! Documentation helper: run a session on the emulated Apple-1 and print the
//! transcript (every character written to the display).
//!
//!   cargo run --release -p apple1-core --example a1run -- basic SESSION.txt
//!   cargo run --release -p apple1-core --example a1run -- asm SOURCE.s [KEYS]
//!
//! `basic`: loads the real Integer BASIC from test-fixtures/BASIC.wav (not in
//! the repository), cold starts it (E000R) and types the session file line
//! by line, waiting after each line until the machine waits for a key.
//! `asm`: assembles SOURCE.s with the native assembler, loads it and runs it
//! from its origin, typing KEYS (\n = RETURN) once it waits for input.
use apple1_core::machine::{Config, Machine};
use apple1_core::{aciwav, asm6502, program};

fn machine() -> Machine {
    // A1RUN_GEN2=1: with Uncle Bernie's GEN2 colour card (RAM $0000-$BFFF)
    let cfg = Config { display_throttle: false, color_card: std::env::var_os("A1RUN_GEN2").is_some(), ..Default::default() };
    let mut m = Machine::new(cfg);
    m.board.display.log = Some(Vec::new());
    m.run_cycles(100_000);
    m
}

/// Run until the keyboard queue is empty and the CPU has been polling the
/// keyboard (Woz Monitor $FF29-$FF2F, ACI $C10D-$C111, Integer BASIC
/// $E003-$E006, or a program's own loop on $D011) for 100 000 cycles.
fn idle(m: &mut Machine, max: u64) -> bool {
    let end = m.board.cycles + max;
    let mut since: Option<u64> = None;
    while m.board.cycles < end {
        m.step();
        let pc = m.cpu.pc;
        let waiting = m.board.key_queue.is_empty() && !m.board.pia.key_available();
        let in_loop = (0xFF29..=0xFF2F).contains(&pc) || (0xC10D..=0xC111).contains(&pc) || (0xE003..=0xE006).contains(&pc) || KEYLOOP.with(|k| k.get().is_some_and(|(a, b)| (a..=b).contains(&pc)));
        if waiting && in_loop {
            let s = *since.get_or_insert(m.board.cycles);
            if m.board.cycles - s > 100_000 {
                return true;
            }
        } else if !in_loop {
            since = None;
        }
    }
    false
}

thread_local! { static KEYLOOP: std::cell::Cell<Option<(u16, u16)>> = const { std::cell::Cell::new(None) }; }

/// The real Integer BASIC from test-fixtures/BASIC.wav (never in the repository).
fn load_basic(m: &mut Machine) {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../test-fixtures/BASIC.wav");
    let b = std::fs::read(path).expect("test-fixtures/BASIC.wav");
    let imp = aciwav::import_wav_bytes(&b).unwrap();
    let p = program::wav_to_program(&imp, Some(0xE000)).unwrap();
    for (ad, d) in &p.blocks {
        m.load(*ad, d);
    }
}

fn transcript(m: &Machine) -> String {
    let l = m.board.display.log.as_ref().unwrap();
    l.iter().map(|&c| if c == 0x0D { '\n' } else { c as char }).collect()
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let mode = a.first().map(String::as_str).unwrap_or("");
    let mut m = machine();
    match mode {
        "basic" => {
            load_basic(&mut m);
            let session = std::fs::read_to_string(&a[1]).unwrap();
            m.type_text("E000R\n");
            idle(&mut m, 5_000_000);
            for line in session.lines() {
                m.type_text(line);
                m.type_text("\n");
                idle(&mut m, 400_000_000);
            }
        }
        "asm" => {
            let out = asm6502::assemble_file(std::path::Path::new(&a[1]), &[]).unwrap_or_else(|e| panic!("{e:?}"));
            for w in &out.warnings {
                eprintln!("warning: {w:?}");
            }
            eprintln!("{} bytes at ${:04X}", out.bytes.len(), out.origin);
            m.load(out.origin, &out.bytes);
            m.type_text(&format!("{:X}R\n", out.origin));
            idle(&mut m, 50_000_000);
            if let Some(keys) = a.get(2) {
                for part in keys.split('|') {
                    m.type_text(&part.replace("\\n", "\n"));
                    idle(&mut m, 50_000_000);
                }
            }
        }
        "mon" => {
            let session = std::fs::read_to_string(&a[1]).unwrap();
            for line in session.lines() {
                m.type_text(line);
                m.type_text("\n");
                idle(&mut m, 50_000_000);
            }
        }
        "pcs" => {
            // like `basic`, with a short wait per line, then where the CPU spends its time
            load_basic(&mut m);
            let session = std::fs::read_to_string(&a[1]).unwrap();
            m.type_text("E000R\n");
            idle(&mut m, 5_000_000);
            for line in session.lines() {
                m.type_text(line);
                m.type_text("\n");
                if !idle(&mut m, 20_000_000) {
                    eprintln!("not idle after {line:?}");
                }
            }
            let mut h = std::collections::BTreeMap::new();
            for _ in 0..200_000 {
                m.step();
                *h.entry(m.cpu.pc).or_insert(0u32) += 1;
            }
            let mut v: Vec<_> = h.into_iter().collect();
            v.sort_by_key(|x| std::cmp::Reverse(x.1));
            eprintln!("hot PCs: {:X?}", &v[..v.len().min(12)]);
        }
        _ => {
            eprintln!("usage: a1run basic SESSION.txt | asm SOURCE.s [KEYS]");
            std::process::exit(2);
        }
    }
    if m.board.config.color_card {
        eprintln!("GEN2: {}", m.board.gen2.mode_name());
    }
    // A1RUN_DUMP=ADDR:LEN (hex): checksum and first bytes of a memory range
    if let Ok(d) = std::env::var("A1RUN_DUMP") {
        let (a, l) = d.split_once(':').unwrap();
        let (a, l) = (u16::from_str_radix(a, 16).unwrap(), usize::from_str_radix(l, 16).unwrap());
        let v: Vec<u8> = (0..l).map(|i| m.board.peek(a.wrapping_add(i as u16))).collect();
        let sum = v.iter().fold(0u32, |s, &b| s.wrapping_mul(31).wrapping_add(b as u32));
        eprintln!("dump {a:04X}+{l:X}: sum {sum:08X} first {:02X?}", &v[..v.len().min(16)]);
    }
    print!("{}", transcript(&m));
}
