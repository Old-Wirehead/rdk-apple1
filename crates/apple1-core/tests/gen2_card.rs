//! Uncle Bernie's own GEN2 beta test programs (Applefritter, 29 Sept. 2026)
//! running on the emulated colour card.
use apple1_core::machine::{Config, Machine};
use apple1_core::program::{load_bytes, Kind};

fn machine() -> Machine {
    let mut m = Machine::new(Config { color_card: true, display_throttle: false, ..Config::default() });
    m.run_until_idle(2_000_000);
    m
}

fn load(m: &mut Machine, name: &str) -> u16 {
    let path = format!("{}/../../samples/{name}", env!("CARGO_MANIFEST_DIR"));
    let p = load_bytes(&std::fs::read(&path).unwrap(), Kind::Hex, None).unwrap();
    for (a, d) in &p.blocks {
        assert_eq!(m.load(*a, d), 0, "{name}: not RAM at {a:04X}");
    }
    p.run.expect("run address")
}

fn run(m: &mut Machine, addr: u16, cycles: u64) {
    m.type_text(&format!("{addr:X}R\n"));
    m.run_cycles(cycles);
}

#[test]
fn test4_hst0_is_detected() {
    let mut m = machine();
    let a = load(&mut m, "gen2-test4.apl");
    assert_eq!(a, 0x280);
    run(&mut m, a, 3_000_000);
    let s = m.screen_text();
    assert!(!s.contains("HST0 ?"), "TEST4 says HST0 is missing:\n{s}");
    // TEST4 races the beam: thousands of soft switch reads per second
    assert!(m.board.gen2.accesses > 10_000);
}

#[test]
fn test4_without_card_reports_missing_hst0() {
    let mut m = Machine::new(Config { display_throttle: false, ..Config::default() });
    m.run_until_idle(2_000_000);
    let a = load(&mut m, "gen2-test4.apl");
    run(&mut m, a, 5_000_000);
    assert!(m.screen_text().contains("HST0 ?"), "{}", m.screen_text());
}

#[test]
fn test2_lores_diagonal_and_mixed() {
    let mut m = machine();
    let a = load(&mut m, "gen2-test2.apl");
    run(&mut m, a, 20_000_000);
    let s = m.screen_text();
    // the LORES list has scrolled away; its last entry is "2F 07D0 DD":
    // line 47 = bottom nibbles of row 23 ($07D0), yellow ($D) somewhere
    let row23 = m.dump(0x7D0, 0x7F7);
    assert!(row23.iter().any(|b| b >> 4 == 0xD), "{row23:02X?}\n{s}");
    assert!(s.contains("17 07D0"), "{s}"); // TEXT row 23 address (Apple II ref. manual)
    let g = m.board.gen2.clone();
    assert!(!g.text && g.mixed && !g.hires && !g.page2, "{g:?}");
}

#[test]
fn original_demo_bars() {
    let mut m = machine();
    let a = load(&mut m, "gen2-barres.txt");
    run(&mut m, a, 500_000);
    let g = m.board.gen2.clone();
    assert!(!g.text && g.mixed && !g.hires, "{g:?}");
    assert_eq!(m.dump(0x400, 0x400)[0], 0x00);
    assert_eq!(m.dump(0x427, 0x427)[0], 0xFF);
    assert!(m.screen_text().contains("GEN2: 16 BARRES"), "pc={:04X} jam={} cyc={}\n{}", m.cpu.pc, m.cpu.jammed, m.board.cycles, m.screen_text());
}
