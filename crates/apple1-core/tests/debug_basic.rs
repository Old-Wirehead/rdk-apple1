//! Debugger against the real Integer BASIC (test-fixtures/BASIC.wav, not
//! in the repository: the test is skipped without it).

use apple1_core::debug::{basic_vars, basic_watch, Debugger, Mission, Stop};
use apple1_core::machine::{Config, Machine};
use apple1_core::{aciwav, intbasic, program};

fn basic_machine(src: &str) -> Option<Machine> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../test-fixtures/BASIC.wav");
    let b = std::fs::read(path).ok()?;
    let imp = aciwav::import_wav_bytes(&b).unwrap();
    let p = program::wav_to_program(&imp, Some(0xE000)).unwrap();
    let l = intbasic::tokenize(src);
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

/// The test program lives in `ports/` (outside the repository): None, and the
/// test skipped, when it is not there.
fn stars() -> Option<String> {
    std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../ports/basic/stars.bas")).ok()
}

#[test]
fn basic_line_steps_follow_the_program() {
    let Some(mut m) = stars().and_then(|s| basic_machine(&s)) else {
        eprintln!("BASIC.wav or ports/basic/stars.bas missing: skipped");
        return;
    };
    let mut d = Debugger::default();
    let mut lines = Vec::new();
    for _ in 0..7 {
        d.go(Mission::BasicLine);
        let mut s = None;
        for _ in 0..200 {
            s = d.run(&mut m, 100_000);
            if s.is_some() {
                break;
            }
        }
        match s {
            Some(Stop::BasicLine(l)) => lines.push(l),
            other => panic!("{other:?} after {lines:?}"),
        }
    }
    assert_eq!(lines, [100, 110, 120, 150, 160, 170, 180]);
    // 180 waits on INPUT Z: answer 0, then 190 and its GOTO 280
    // a key pressed while a line starts stops the program (BIT $D011
    // after the hook): let INPUT wait first, as a user would
    d.cont();
    assert_eq!(d.run(&mut m, 300_000), None);
    d.go(Mission::BasicLine);
    m.type_text("0\n");
    let mut s = None;
    for _ in 0..200 {
        s = d.run(&mut m, 100_000);
        if s.is_some() {
            break;
        }
    }
    assert_eq!(s, Some(Stop::BasicLine(190)));
    d.go(Mission::BasicLine);
    assert_eq!(d.run(&mut m, 100_000), Some(Stop::BasicLine(280)));
    assert_eq!(d.basic_line, Some(280));
    // breakpoint on 360 (INPUT G) with the variables set
    d.basic_breaks.insert(360);
    d.cont();
    for _ in 0..200 {
        s = d.run(&mut m, 100_000);
        if s.is_some() {
            break;
        }
    }
    assert_eq!(s, Some(Stop::BasicBreak(360)));
    let v = basic_vars(&|a| m.board.peek(a));
    assert_eq!(basic_watch(&v, "A"), Some("100".into()));
    assert_eq!(basic_watch(&v, "M"), Some("7".into()));
    assert_eq!(basic_watch(&v, "Z"), Some("0".into()));
    assert_eq!(basic_watch(&v, "K"), Some("1".into()));
    let x: i32 = basic_watch(&v, "X").unwrap().parse().unwrap();
    assert!((1..=100).contains(&x), "{x}");
}

#[test]
fn variables_of_a_live_program() {
    let Some(mut m) = basic_machine("10 A=5:B1=-3:DIM C$(10):C$=\"HI\":DIM D(3):D(2)=7\n20 GOTO 20\n") else {
        return;
    };
    let mut d = Debugger::default();
    d.basic_breaks.insert(20);
    let mut s = None;
    for _ in 0..100 {
        s = d.run(&mut m, 100_000);
        if s.is_some() {
            break;
        }
    }
    assert_eq!(s, Some(Stop::BasicBreak(20)));
    let v = basic_vars(&|a| m.board.peek(a));
    let w: Vec<Option<String>> = ["A", "B1", "C$", "D(2)", "D"].iter().map(|e| basic_watch(&v, e)).collect();
    assert_eq!(w, [Some("5".into()), Some("-3".into()), Some("\"HI\"".into()), Some("7".into()), Some("(0, 7, 0)".into())]);
}
