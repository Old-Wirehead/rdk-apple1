//! Export -> import round trips under various audio conditions.
use apple1_core::aciwav::{export_wav_bytes, import_wav_bytes, ExportOptions};
use apple1_core::tape::{self, RenderOptions};
use apple1_core::wav;
use apple1_core::Timing;

fn sample_data(n: usize, seed: u32) -> Vec<u8> {
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

#[test]
fn roundtrip_rates_and_bits() {
    let data = sample_data(300, 1);
    for rate in [22050u32, 44100, 48000] {
        for bits in [8u16, 16] {
            let o = ExportOptions { sample_rate: rate, bits, ..Default::default() };
            let w = export_wav_bytes(&[(0x0300, &data)], &o).unwrap();
            let imp = import_wav_bytes(&w).unwrap();
            assert_eq!(imp.segments.len(), 1, "rate {rate} bits {bits}");
            assert_eq!(imp.segments[0].data, data, "rate {rate} bits {bits}");
            assert_eq!(imp.embedded, vec![(0x0300, 0x042B)]);
        }
    }
}

#[test]
fn edge_values_00_ff_and_page_crossing() {
    let mut data = vec![0x00; 40];
    data.extend(vec![0xFF; 40]);
    data.extend([0x55, 0xAA, 0x01, 0x80]);
    let o = ExportOptions::default();
    let w = export_wav_bytes(&[(0x02F0, &data)], &o).unwrap();
    let imp = import_wav_bytes(&w).unwrap();
    assert_eq!(imp.segments[0].data, data);
}

fn degrade(samples: &[f32], gain: f32, noise: f32, invert: bool, seed: u32) -> Vec<f32> {
    let mut x = seed;
    samples
        .iter()
        .map(|&s| {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            let r = (x as f32 / u32::MAX as f32) * 2.0 - 1.0;
            let v = s * gain + r * noise + 0.05; // + DC offset
            if invert {
                -v
            } else {
                v
            }
        })
        .collect()
}

#[test]
fn robust_to_noise_gain_dc_polarity_stereo() {
    let data = sample_data(256, 7);
    let toggles = tape::rom_write_toggles(&[(0x0300, &data)], Timing::default()).unwrap();
    let rate = 44100;
    let clean = tape::render_toggles(&toggles, &RenderOptions { sample_rate: rate, ..Default::default() });
    for (gain, noise, inv) in [(0.05f32, 0.01f32, false), (1.2, 0.15, true), (0.3, 0.05, false)] {
        let d = degrade(&clean, gain, noise, inv, 99);
        // put the signal in the right channel of a stereo file, left is noise
        let left: Vec<f32> = degrade(&vec![0.0; d.len()], 0.0, 0.002, false, 5);
        let bytes = wav::encode_wav_stereo16(&left, &d, rate);
        let imp = import_wav_bytes(&bytes).unwrap();
        assert_eq!(imp.channels, 2);
        assert_eq!(imp.segments.len(), 1, "gain {gain} noise {noise}");
        assert_eq!(imp.segments[0].data, data, "gain {gain} noise {noise} inv {inv}");
    }
}

#[test]
fn robust_to_tape_speed_and_lowpass() {
    let data = sample_data(200, 3);
    let toggles = tape::rom_write_toggles(&[(0x0300, &data)], Timing::default()).unwrap();
    for speed in [0.90f64, 0.95, 1.05, 1.10] {
        let t: Vec<f64> = toggles.iter().map(|x| x / speed).collect();
        let s = tape::render_toggles(&t, &RenderOptions { sample_rate: 22050, ..Default::default() });
        // crude cassette-like low pass (one pole ~4 kHz)
        let mut y = 0.0f32;
        let a = 0.6f32;
        let f: Vec<f32> = s.iter().map(|&x| { y += a * (x - y); y }).collect();
        let segs = tape::decode_samples(&f, 22050);
        assert_eq!(segs.len(), 1, "speed {speed}");
        assert_eq!(segs[0].data, data, "speed {speed}");
    }
}

#[test]
fn multi_segment_like_basic_save() {
    // like "4A.FFW 800.8FFW" (Integer BASIC program save)
    let a = sample_data(0xB6, 11);
    let b = sample_data(0x100, 12);
    let w = export_wav_bytes(&[(0x004A, &a), (0x0800, &b)], &ExportOptions::default()).unwrap();
    let imp = import_wav_bytes(&w).unwrap();
    assert_eq!(imp.segments.len(), 2);
    assert_eq!(imp.segments[0].data, a);
    assert_eq!(imp.segments[1].data, b);
    assert_eq!(imp.embedded, vec![(0x004A, 0x00FF), (0x0800, 0x08FF)]);
}

#[test]
fn aci_rom_timing() {
    // Measured ACI ROM timing, in CPU cycles (independent of the clock model)
    let raw = tape::rom_write_toggles(&[(0x0300, &[0x00, 0xFF][..])], Timing { clock_hz: 1.0, refresh_stall: false }).unwrap();
    let h: Vec<u64> = raw.windows(2).map(|w| (w[1] - w[0]) as u64).collect();
    let n = h.len();
    // "0300.0301W": X = 9 -> the first WHEADER inner loop runs 9 times, not 256
    assert_eq!(n, 16639 - 256 + 9 + 2 + 32);
    assert_eq!(h[100], 592); // header half
    assert_eq!(&h[n - 34..n - 32], &[180, 238]); // start bit
    assert_eq!(h[n - 30], 238); // "0" half
    assert_eq!(h[n - 2], 473); // "1" half
    // header length with the default clock model ≈ 10.26 s
    let secs = tape::rom_write_toggles(&[(0x0300, &[0u8][..])], Timing::default()).unwrap();
    let hdr = secs[16639 - 256 + 7] - secs[0];
    assert!((hdr - 10.1).abs() < 0.05, "{hdr}");
}

#[test]
fn decode_c2t_reference_files() {
    // Generated by c2t (Egan Ford, github.com/datajerk/c2t): `c2t -1 c2t_hello.mon out.wav`
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/data/");
    let expect = [0xA2, 0x00, 0xBD, 0x10, 0x03, 0xF0, 0x06, 0x20, 0xEF, 0xFF, 0xE8, 0xD0, 0xF5, 0x4C, 0x1F, 0xFF];
    for f in ["c2t_hello_11025_8bit.wav", "c2t_hello_44100_8bit.wav"] {
        let b = std::fs::read(format!("{dir}{f}")).unwrap();
        let imp = import_wav_bytes(&b).unwrap();
        assert_eq!(imp.segments.len(), 1, "{f}");
        let d = &imp.segments[0].data;
        assert_eq!(&d[..16], &expect, "{f}");
        // c2t writes the whole range 0300-0313 (gap filled)
        assert_eq!(&d[0x10..0x14], &[0xC8, 0xC9, 0x8D, 0x00], "{f}");
    }
}
