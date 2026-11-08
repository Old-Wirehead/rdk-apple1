//! The original ACI ROM, driven through the emulated keyboard, reading our
//! WAV files and writing tapes that we decode.
use apple1_core::aciwav::{export_wav_bytes, ExportOptions};
use apple1_core::machine::{Config, Machine};
use apple1_core::tape::{self, TapePlayer};
use apple1_core::wav;

fn machine() -> Machine {
    let mut m = Machine::new(Config { display_throttle: false, ..Config::default() });
    m.run_until_idle(2_000_000);
    m
}

fn load_wav_into_player(m: &mut Machine, bytes: &[u8]) {
    let a = wav::parse_wav(bytes).unwrap();
    m.board.player = Some(TapePlayer::from_samples(a.best_channel(), a.sample_rate, "test"));
}

fn rom_read(m: &mut Machine, cmd: &str) {
    m.type_text("C100R\n");
    m.run_until_idle(2_000_000);
    m.type_text(cmd);
    m.type_text("\n");
    // header 10 s + data: give it up to 30 s of emulated time
    assert!(m.run_until_idle(30_000_000), "ACI read did not return: {}", m.screen_text());
}

#[test]
fn rom_reads_our_wav() {
    let data: Vec<u8> = (0..=255u8).rev().chain(0..64).collect();
    for rate in [22050u32, 48000] {
        let w = export_wav_bytes(&[(0x0300, &data)], &ExportOptions { sample_rate: rate, bits: 8, ..Default::default() }).unwrap();
        let mut m = machine();
        load_wav_into_player(&mut m, &w);
        rom_read(&mut m, &format!("300.{:X}R", 0x300 + data.len() - 1));
        assert_eq!(m.dump(0x300, 0x300 + data.len() as u16 - 1), data, "rate {rate}");
        let screen = m.screen_text();
        assert!(screen.contains("C100: A9*"), "{screen}"); // the "*" prompt of the ACI
    }
}

#[test]
fn rom_reads_c2t_wav() {
    let p = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/data/c2t_hello_44100_8bit.wav");
    let b = std::fs::read(p).unwrap();
    let mut m = machine();
    load_wav_into_player(&mut m, &b);
    rom_read(&mut m, "300.313R");
    m.type_text("300R\n");
    m.run_until_idle(5_000_000);
    assert!(m.screen_text().contains("HI"), "{}", m.screen_text());
}

#[test]
fn rom_write_matches_direct_export_and_decodes() {
    let data: Vec<u8> = (0..100u8).map(|i| i.wrapping_mul(37)).collect();
    let mut m = machine();
    m.load(0x0300, &data);
    m.type_text("C100R\n");
    m.run_until_idle(2_000_000);
    m.type_text("0300.0363W\n");
    // start recording right before the CR is processed
    m.board.recording = Some(Vec::new());
    assert!(m.run_until_idle(30_000_000));
    let rec = m.board.recording.take().unwrap();
    let timing = m.board.config.timing;
    let secs: Vec<f64> = rec.iter().map(|&c| timing.cycles_to_secs(c)).collect();
    let segs = tape::decode_edges(&secs.iter().map(|t| t - secs[0]).collect::<Vec<_>>());
    assert_eq!(segs.len(), 1);
    assert_eq!(segs[0].data, data);
    // same half periods (in cycles) as the direct exporter
    let direct = tape::rom_write_toggles(&[(0x0300, &data)], apple1_core::Timing { clock_hz: 1.0, refresh_stall: false }).unwrap();
    let a: Vec<u64> = rec.windows(2).map(|w| w[1] - w[0]).collect();
    let b: Vec<u64> = direct.windows(2).map(|w| (w[1] - w[0]) as u64).collect();
    assert_eq!(a.len(), b.len());
    assert_eq!(a, b);
}
