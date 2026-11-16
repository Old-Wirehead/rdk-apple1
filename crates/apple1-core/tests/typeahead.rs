//! Keys typed ahead (paste, scripted typing): each direct command runs (#11).

use apple1_core::machine::{Config, Machine};

fn screen(m: &Machine) -> String {
    m.board.display.text()
}

#[test]
fn direct_commands_typed_at_once_all_run() {
    let mut m = Machine::new(Config::default());
    m.load(apple1_core::g2basic::ADDRESS, apple1_core::g2basic::IMAGE);
    m.reset();
    m.run_cycles(200_000);
    m.type_text("E000R\n");
    m.run_cycles(500_000);
    m.type_text("PRINT 3+4\nA=5\nPRINT A*2\nGOTO 500\n");
    m.run_until_idle(20_000_000);
    m.run_cycles(2_000_000);
    let s = screen(&m);
    assert!(s.contains("\n7\n") || s.contains("\n7 \n") || s.lines().any(|l| l.trim() == "7"), "{s}");
    assert!(s.lines().any(|l| l.trim() == "10"), "{s}");
    assert!(s.contains("BAD BRANCH"), "{s}");
}
