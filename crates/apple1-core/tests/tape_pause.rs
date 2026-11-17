//! A tape stopped in the middle of a load, then played again: the load goes on (#14).

use apple1_core::machine::{Config, Machine};
use apple1_core::tape::TapePlayer;

#[test]
fn stop_then_play_during_a_load() {
    for at in [6.0, 10.5, 10.65, 10.75] {
        load_with_stop_at(at);
    }
}

fn load_with_stop_at(at: f64) {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../samples/hello.wav");
    let a = apple1_core::wav::read_wav(&root).unwrap();
    let mut m = Machine::new(Config::default());
    m.reset();
    m.run_cycles(200_000);
    m.board.player = Some(TapePlayer::from_samples(a.best_channel(), a.sample_rate, "hello"));
    m.type_text("C100R\n300.31ER\n");
    let secs = |m: &Machine| m.board.now_secs();
    let t0 = secs(&m);
    while secs(&m) < t0 + at {
        m.run_cycles(10_000);
    }
    let now = secs(&m);
    assert!(m.board.player.as_mut().unwrap().pause(now));
    let t1 = secs(&m);
    while secs(&m) < t1 + 4.0 {
        m.run_cycles(10_000);
    }
    let now = secs(&m);
    assert!(m.board.player.as_mut().unwrap().resume(now));
    let t2 = secs(&m);
    while secs(&m) < t2 + 10.0 {
        m.run_cycles(10_000);
    }
    assert_eq!(m.board.mem[0x300], 0xA2, "{}", m.board.display.text());
    assert_eq!(m.board.mem[0x301], 0x00);
}
