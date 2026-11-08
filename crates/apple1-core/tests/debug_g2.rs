//! Debugger against the built-in RDK G2 Basic (issue #45): current line, breakpoints on
//! lines, stepping by line, variables.

use apple1_core::debug::{basic_vars, basic_watch, Debugger, Mission, Stop, G2_LINE_HOOKS};
use apple1_core::machine::{Config, Machine};

fn g2_machine(src: &str) -> Machine {
    let mut m = Machine::new(Config::default());
    m.load(apple1_core::g2basic::ADDRESS, apple1_core::g2basic::IMAGE);
    m.reset();
    m.run_cycles(200_000);
    m.type_text("E000R\n");
    m.run_cycles(300_000);
    for l in src.lines() {
        m.type_text(&format!("{l}\n"));
        m.run_cycles(400_000);
    }
    m.type_text("RUN\n");
    m
}

fn until_stop(d: &mut Debugger, m: &mut Machine) -> Option<Stop> {
    for _ in 0..300 {
        if let Some(s) = d.run(m, 100_000) {
            return Some(s);
        }
    }
    None
}

#[test]
fn hook_is_the_end_of_setline() {
    let img = apple1_core::g2basic::IMAGE;
    let at = |h: u16| (h - apple1_core::g2basic::ADDRESS) as usize;
    // LDA PTR+1 ; STA CUR+1
    assert_eq!(&img[at(G2_LINE_HOOKS[0]) - 2..at(G2_LINE_HOOKS[0]) + 2], &[0xA5, 0x09, 0x85, 0x07]);
    // ADC #0 ; STA IP+1 ; JMP EXEC
    assert_eq!(&img[at(G2_LINE_HOOKS[1]) - 2..at(G2_LINE_HOOKS[1]) + 3], &[0x69, 0x00, 0x85, 0x05, 0x4C]);
}

const LOOP: &str = "10 S=0\n20 FOR I=1 TO 500\n30 S=S+1\n40 NEXT I\n50 END\n";

#[test]
fn breakpoint_on_a_line() {
    let mut m = g2_machine(LOOP);
    let mut d = Debugger::default();
    d.basic_breaks.insert(30);
    assert_eq!(until_stop(&mut d, &mut m), Some(Stop::BasicBreak(30)));
    assert_eq!(d.basic_line, Some(30));
    let v = basic_vars(&|a| m.board.peek(a));
    assert_eq!(basic_watch(&v, "S"), Some("0".into()));
    // by line: 40, then back to 30 with S = 1
    d.go(Mission::BasicLine);
    assert_eq!(until_stop(&mut d, &mut m), Some(Stop::BasicLine(40)));
    d.go(Mission::BasicLine);
    assert_eq!(until_stop(&mut d, &mut m), Some(Stop::BasicLine(30)));
    let v = basic_vars(&|a| m.board.peek(a));
    assert_eq!(basic_watch(&v, "S"), Some("1".into()));
    assert_eq!(basic_watch(&v, "I"), Some("2".into()));
}

#[test]
fn variables_of_a_live_program() {
    let mut m = g2_machine("10 A=5:B1=-3:DIM C$(10):C$=\"HI\":DIM D(3):D(2)=7\n20 GOTO 20\n");
    let mut d = Debugger::default();
    d.basic_breaks.insert(20);
    assert_eq!(until_stop(&mut d, &mut m), Some(Stop::BasicBreak(20)));
    let v = basic_vars(&|a| m.board.peek(a));
    assert_eq!(basic_watch(&v, "A"), Some("5".into()));
    assert_eq!(basic_watch(&v, "B1"), Some("-3".into()));
    assert_eq!(basic_watch(&v, "C$"), Some("\"HI\"".into()));
    assert_eq!(basic_watch(&v, "D(2)"), Some("7".into()));
    assert_eq!(basic_watch(&v, "D(1)"), Some("0".into()));
}
