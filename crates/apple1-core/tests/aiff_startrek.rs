//! AIFF (smartphone profile) and multi-range tapes laid out like the
//! Applefritter Star Trek reference (`4A.00FFR 300.FFFR`, then `E2B3R`).
//!
//! No Star Trek recording is available in the shared folders, so the data
//! here is SYNTHETIC (pseudo-random bytes in the same two ranges), and the
//! "BASIC" at $E2B3 is a 13-byte stub that prints OK: Integer BASIC itself is
//! not distributed with the emulator.
use apple1_core::aciinput::{self, AciInput};
use apple1_core::aciwav::{export_wav_bytes, import_wav_bytes, ExportOptions};
use apple1_core::machine::{Config, Machine};
use apple1_core::program;
use apple1_core::tape::{self, TapePlayer};
use apple1_core::wav::{self, AudioFormat};

fn machine() -> Machine {
    let mut m = Machine::new(Config { display_throttle: false, ..Config::default() });
    m.run_until_idle(2_000_000);
    m
}

fn pseudo(n: usize, seed: u32) -> Vec<u8> {
    let mut x = seed;
    (0..n)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            (x >> 8) as u8
        })
        .collect()
}

fn startrek_like() -> (Vec<u8>, Vec<u8>) {
    // $4A-$FF: Integer BASIC zero page (pointers LOMEM/HIMEM/PP/PV...),
    // $300-$FFF: program text + variables.
    (pseudo(0x100 - 0x4A, 1), pseudo(0x1000 - 0x300, 7))
}

fn phone_aiff(segs: &[(u16, &[u8])]) -> Vec<u8> {
    export_wav_bytes(segs, &ExportOptions::for_format(AudioFormat::Aiff)).unwrap()
}

#[test]
fn aiff_phone_profile_properties() {
    let data = pseudo(64, 3);
    let b = phone_aiff(&[(0x0300, &data)]);
    assert_eq!(&b[0..4], b"FORM");
    assert_eq!(&b[8..12], b"AIFF");
    let a = wav::parse_audio(&b).unwrap();
    assert_eq!(a.sample_rate, 48000);
    assert_eq!(a.bits, 16);
    assert_eq!(a.channels.len(), 2);
    assert_eq!(a.channels[0], a.channels[1], "channels must be identical (L and R are tied together)");
    let s = &a.channels[0];
    let peak = s.iter().fold(0f32, |m, &x| m.max(x.abs()));
    assert!((peak - 0.7).abs() < 0.001, "peak {peak}");
    let dc: f64 = s.iter().map(|&x| x as f64).sum::<f64>() / s.len() as f64;
    assert!(dc.abs() < 1e-3, "DC {dc}");
    // 1 s of digital silence before the header
    assert!(s[..47_900].iter().all(|&x| x == 0.0));
    assert!(s[48_100] != 0.0);
    let imp = import_wav_bytes(&b).unwrap();
    assert_eq!(imp.segments.len(), 1);
    assert_eq!(imp.segments[0].data, data);
    assert_eq!(imp.embedded, vec![(0x0300, 0x033F)]);
}

#[test]
fn aiff_variants_roundtrip() {
    let data = pseudo(300, 9);
    for (rate, bits, ch) in [(44100u32, 16u16, 1u16), (48000, 8, 2), (22050, 16, 2), (96000, 16, 1)] {
        let o = ExportOptions { sample_rate: rate, bits, channels: ch, ..ExportOptions::for_format(AudioFormat::Aiff) };
        let b = export_wav_bytes(&[(0x0280, &data)], &o).unwrap();
        let imp = import_wav_bytes(&b).unwrap();
        assert_eq!(imp.sample_rate, rate);
        assert_eq!(imp.channels, ch as usize);
        assert_eq!(imp.segments.len(), 1, "{rate} {bits} {ch}");
        assert_eq!(imp.segments[0].data, data, "{rate} {bits} {ch}");
        let p = program::load_bytes(&b, program::kind_of(std::path::Path::new("x.aif"), Some(&b)), None).unwrap();
        assert_eq!(p.ranges(), vec![(0x0280, 0x03AB)]);
    }
}

#[test]
fn startrek_layout_export_import() {
    let (zp, prog) = startrek_like();
    let b = phone_aiff(&[(0x004A, &zp), (0x0300, &prog)]);
    let imp = import_wav_bytes(&b).unwrap();
    assert_eq!(imp.segments.len(), 2);
    assert_eq!(imp.segments[0].data, zp);
    assert_eq!(imp.segments[1].data, prog);
    assert_eq!(imp.embedded, vec![(0x004A, 0x00FF), (0x0300, 0x0FFF)]);
    let p = program::wav_to_program(&imp, None).unwrap();
    assert_eq!(p.aci_read_command(), "004A.00FFR 0300.0FFFR"); // the Applefritter line, 4 digits
    // the ranges as typed on the Applefritter thread parse to the same thing
    assert_eq!(program::parse_ranges("4A.00FFR 300.FFFR").unwrap(), vec![(0x4A, 0xFF), (0x300, 0xFFF)]);
}

#[test]
fn startrek_layout_read_by_emulated_rom_then_e2b3r() {
    let (zp, prog) = startrek_like();
    let b = phone_aiff(&[(0x004A, &zp), (0x0300, &prog)]);
    let a = wav::parse_audio(&b).unwrap();
    let mut m = machine();
    // stand-in for Integer BASIC's warm entry at $E2B3: print "OK" and
    // return to the monitor (JMP $FF1F)
    m.load(0xE2B3, &[0xA9, 0xCF, 0x20, 0xEF, 0xFF, 0xA9, 0xCB, 0x20, 0xEF, 0xFF, 0x4C, 0x1F, 0xFF]);
    m.board.player = Some(TapePlayer::from_samples(a.best_channel(), a.sample_rate, "startrek-synthetique.aif"));
    m.type_text("C100R\n");
    m.run_until_idle(2_000_000);
    m.type_text("4A.00FFR 300.FFFR\n");
    // two 10 s headers + ~22 s of data
    assert!(m.run_until_idle(60_000_000), "ACI read did not return: {}", m.screen_text());
    assert_eq!(m.dump(0x4A, 0xFF), zp);
    assert_eq!(m.dump(0x300, 0xFFF), prog);
    assert!(m.screen_text().contains("4A.00FFR 300.FFFR"));
    assert!(m.screen_text().trim_end().ends_with('\\'), "{}", m.screen_text());
    m.type_text("E2B3R\n");
    m.run_until_idle(2_000_000);
    assert!(m.screen_text().contains("OK"), "{}", m.screen_text());
}

#[test]
fn rom_write_of_two_ranges_matches_exporter() {
    // Typing "004A.00FFW 0300.0FFFW" on the emulated Apple-1 produces exactly
    // the half-periods of our exporter (header length depends on the
    // position of each 'W' in the typed line). NB: "4A.FFW" would mean
    // $4A..$4AFF for the ACI parser (it does not clear its accumulator at '.').
    let (zp, prog) = startrek_like();
    let mut m = machine();
    m.load(0x4A, &zp);
    m.load(0x300, &prog);
    m.type_text("C100R\n");
    m.run_until_idle(2_000_000);
    m.type_text("004A.00FFW 0300.0FFFW\n");
    m.board.recording = Some(Vec::new());
    assert!(m.run_until_idle(60_000_000), "pc {:04X} toggles {} cycles {} {}", m.cpu.pc, m.board.recording.as_ref().unwrap().len(), m.board.cycles, m.screen_text());
    let rec = m.board.recording.take().unwrap();
    let direct =
        tape::rom_write_toggles(&[(0x4A, &zp), (0x300, &prog)], apple1_core::Timing { clock_hz: 1.0, refresh_stall: false })
            .unwrap();
    let a: Vec<u64> = rec.windows(2).map(|w| w[1] - w[0]).collect();
    let b: Vec<u64> = direct.windows(2).map(|w| (w[1] - w[0]) as u64).collect();
    let d = a.iter().zip(&b).position(|(x, y)| x != y);
    assert!(d.is_none() && a.len() == b.len(), "len {} vs {}, first diff {:?} {:?}", a.len(), b.len(), d, d.map(|i| (&a[i.saturating_sub(3)..(i + 3).min(a.len())], &b[i.saturating_sub(3)..(i + 3).min(b.len())])));
}

/// Simulated 1976 front end: what playback level is needed?
#[test]
fn analog_front_end_levels() {
    let data = pseudo(256, 5);
    let b = phone_aiff(&[(0x0300, &data)]);
    let a = wav::parse_audio(&b).unwrap();
    let ok = |m: AciInput| {
        let e = aciinput::comparator_edges(a.best_channel(), a.sample_rate, &m);
        let s = tape::decode_edges(&e);
        s.len() == 1 && s[0].data == data
    };
    for v in [0.4, 0.5, 1.0, 2.0] {
        assert!(ok(AciInput::stock(v)), "stock {v} V");
        assert!(ok(AciInput { series_ohms: 1200.0, ..AciInput::stock(v) }), "stock+1k2 {v} V");
        assert!(ok(AciInput::mimeo(v)), "mimeo {v} V");
        assert!(ok(AciInput::gen2(v)), "gen2 {v} V");
    }
    // far below the ~0.22 V hysteresis of the stock comparator: nothing
    assert!(!ok(AciInput::stock(0.08)));
}

#[test]
fn sample_multi_aiff_through_rom() {
    let p = concat!(env!("CARGO_MANIFEST_DIR"), "/../../samples/multi.aiff");
    let a = wav::read_wav(std::path::Path::new(p)).unwrap();
    let mut m = machine();
    m.board.player = Some(TapePlayer::from_samples(a.best_channel(), a.sample_rate, "multi.aiff"));
    m.type_text("C100R\n");
    m.run_until_idle(2_000_000);
    m.type_text("4A.005CR 300.30ER\n");
    assert!(m.run_until_idle(40_000_000), "{}", m.screen_text());
    m.type_text("300R\n");
    m.run_until_idle(2_000_000);
    assert!(m.screen_text().contains("MULTI-PLAGES OK !"), "{}", m.screen_text());
}
