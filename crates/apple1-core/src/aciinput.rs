//! Optional model of the ACI analog input (FROM TAPE -> LM311 comparator).
//!
//! The emulator normally uses an ideal edge detector (`tape::detect_edges`)
//! because the ACI ROM only looks at *changes* of the comparator output.
//! This module models the 1976 schematic instead, to check whether a file
//! should survive the real front end at a given playback level:
//!
//! * FROM TAPE -> series C (0.01 µF on the original board, 0.1 µF suggested
//!   by the Mimeo guide and Uncle Bernie's Gen2) -> comparator input biased
//!   at 2.5 V by 10k/10k 1% (5 kΩ Thévenin). With 0.01 µF this is a 3.2 kHz
//!   high pass: the comparator effectively sees the *derivative* of the
//!   signal, which is why a square wave with sharp edges works best.
//! * Reference input also at 2.5 V (10k/10k), with positive feedback from
//!   the output (3k pull-up + 47k). Fixed hysteresis from those values is
//!   about ±0.22 V (stock 47k), ±0.04 V with Gen2's 300k — an estimate from
//!   the schematic, not a measurement.
//! * Uncle Bernie (Oct 2026): Gen2 ACI has no input filtering, and hysteresis
//!   should not be modelled as constant. Keep a moving average of the min
//!   and max of the audio input and set the thresholds at 1/3 and 2/3 of
//!   that span (so the model tracks human volume setting). `AciInput::gen2`
//!   uses that adaptive mode.
//! * Optional series resistor (e.g. 1.2 kΩ) and source resistance.
//!
//! The digital full scale (1.0) is mapped to `volts_peak` at the jack
//! (unloaded); smartphone headphone outputs at maximum volume are typically
//! in the 0.5 to 1 V peak range (not measured on the Redmi Note 7).

#[derive(Debug, Clone, Copy)]
pub struct AciInput {
    pub coupling_farads: f64,
    pub bias_ohms: f64,
    pub series_ohms: f64,
    /// Fixed half-span of the comparator hysteresis (ignored when `adaptive`).
    pub hysteresis_volts: f64,
    pub volts_peak: f64,
    /// Bernie's adaptive thresholds: moving min/max, trip points at 1/3 and 2/3.
    pub adaptive: bool,
}

impl AciInput {
    /// Original 1976 ACI: 0.01 µF, 47k feedback, fixed hysteresis.
    pub fn stock(volts_peak: f64) -> AciInput {
        AciInput {
            coupling_farads: 10e-9,
            bias_ohms: 5000.0,
            series_ohms: 0.0,
            hysteresis_volts: 0.22,
            volts_peak,
            adaptive: false,
        }
    }
    /// Original board with the Mimeo guide's 0.1 µF coupling capacitor.
    pub fn mimeo(volts_peak: f64) -> AciInput {
        AciInput { coupling_farads: 100e-9, ..AciInput::stock(volts_peak) }
    }
    /// Uncle Bernie's Gen2: 100 nF, no filtering, adaptive 1/3–2/3 hysteresis.
    pub fn gen2(volts_peak: f64) -> AciInput {
        AciInput {
            coupling_farads: 100e-9,
            hysteresis_volts: 0.04,
            adaptive: true,
            ..AciInput::stock(volts_peak)
        }
    }
    pub fn by_name(name: &str, volts_peak: f64) -> Option<AciInput> {
        match name.to_ascii_lowercase().as_str() {
            "stock" | "origine" | "1976" => Some(AciInput::stock(volts_peak)),
            "mimeo" => Some(AciInput::mimeo(volts_peak)),
            "gen2" => Some(AciInput::gen2(volts_peak)),
            _ => None,
        }
    }
    /// High-pass corner (Hz).
    pub fn corner_hz(&self) -> f64 {
        1.0 / (2.0 * std::f64::consts::PI * (self.bias_ohms + self.series_ohms) * self.coupling_farads)
    }
}

/// Comparator output transitions (seconds) for the given samples.
pub fn comparator_edges(samples: &[f32], sample_rate: u32, m: &AciInput) -> Vec<f64> {
    const OVER: usize = 8;
    let dt = 1.0 / (sample_rate as f64 * OVER as f64);
    let r_tot = m.bias_ohms + m.series_ohms;
    let tau = r_tot * m.coupling_farads;
    let a = tau / (tau + dt);
    let div = m.bias_ohms / r_tot;
    let h_fixed = m.hysteresis_volts;
    // Adaptive followers: rise instantly to a new extremum, decay slowly
    // (~50 ms) so they track cassette volume without locking onto noise.
    let decay = (-dt / 0.050).exp();
    let mut edges = Vec::new();
    let mut y = 0.0f64; // voltage across the resistive part (relative to 2.5 V bias)
    let mut x_prev = 0.0f64;
    let mut v_prev = 0.0f64;
    let mut state = false; // true = input above high threshold last
    let mut t = 0.0f64;
    let mut vmin = 0.0f64;
    let mut vmax = 0.0f64;
    let mut primed = false;
    for i in 0..samples.len() {
        let s0 = samples[i] as f64 * m.volts_peak;
        let s1 = samples.get(i + 1).map(|&s| s as f64 * m.volts_peak).unwrap_or(s0);
        for k in 0..OVER {
            let x = s0 + (s1 - s0) * k as f64 / OVER as f64;
            y = a * (y + x - x_prev);
            x_prev = x;
            let v = y * div;
            if m.adaptive {
                if !primed {
                    vmin = v;
                    vmax = v;
                    primed = true;
                } else {
                    if v < vmin {
                        vmin = v;
                    } else {
                        vmin = vmin * decay + v * (1.0 - decay);
                    }
                    if v > vmax {
                        vmax = v;
                    } else {
                        vmax = vmax * decay + v * (1.0 - decay);
                    }
                }
            }
            let (lo, hi) = if m.adaptive {
                let span = (vmax - vmin).max(1e-6);
                (vmin + span / 3.0, vmin + 2.0 * span / 3.0)
            } else {
                (-h_fixed, h_fixed)
            };
            let flip = if state { v < lo } else { v > hi };
            if flip {
                let thr = if state { lo } else { hi };
                let frac = if (v - v_prev).abs() > 1e-12 {
                    ((thr - v_prev) / (v - v_prev)).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                edges.push(t - dt + frac * dt);
                state = !state;
            }
            v_prev = v;
            t += dt;
        }
    }
    edges
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn corner_frequencies() {
        assert!((AciInput::stock(1.0).corner_hz() - 3183.0).abs() < 5.0);
        assert!((AciInput::mimeo(1.0).corner_hz() - 318.3).abs() < 1.0);
    }
    #[test]
    fn square_wave_edges_and_threshold() {
        // 1 kHz square, 48 kHz
        let s: Vec<f32> = (0..4800).map(|i| if (i / 24) % 2 == 0 { 0.9 } else { -0.9 }).collect();
        let e = comparator_edges(&s, 48000, &AciInput::stock(1.0));
        assert!(e.len() >= 190, "{}", e.len());
        // far too quiet: 0.1 V peak -> step 0.18 V < 0.22 V hysteresis
        let e = comparator_edges(&s, 48000, &AciInput::stock(0.1));
        assert!(e.len() < 3);
    }
    #[test]
    fn gen2_adaptive_tracks_quiet_square() {
        // Adaptive 1/3–2/3 still sees a quiet but clean square wave.
        let s: Vec<f32> = (0..4800).map(|i| if (i / 24) % 2 == 0 { 0.9 } else { -0.9 }).collect();
        let e = comparator_edges(&s, 48000, &AciInput::gen2(0.1));
        assert!(e.len() >= 190, "adaptive gen2 at 0.1 V: {}", e.len());
    }
}
