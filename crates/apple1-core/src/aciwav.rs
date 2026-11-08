//! High level helpers: memory <-> ACI audio files (WAV or AIFF).

use std::path::Path;

use crate::tape::{self, DecodedSegment, RenderOptions};
use crate::timing::Timing;
use crate::wav::{self, AudioFormat};

#[derive(Debug, Clone, Copy)]
pub struct ExportOptions {
    pub format: AudioFormat,
    pub sample_rate: u32,
    pub bits: u16,
    /// Number of (identical) channels written.
    pub channels: u16,
    pub amplitude: f32,
    /// Silence before the first header edge / after the last edge (s).
    pub lead_silence: f64,
    pub tail_silence: f64,
    pub timing: Timing,
    /// Embed the address range in an INFO/ICMT (WAV) or ANNO (AIFF) chunk
    /// (ignored by players).
    pub embed_address: bool,
}

impl Default for ExportOptions {
    fn default() -> Self {
        ExportOptions::for_format(AudioFormat::Wav)
    }
}

impl ExportOptions {
    /// Defaults per format.
    ///
    /// * WAV: 44.1 kHz, 16-bit, mono, ~70% full scale (Bernie, Oct 2026),
    ///   0.5 s of silence (PC / tape recorder use).
    /// * AIFF ("smartphone" profile, see README section 4): 48 kHz (native
    ///   rate of Android audio, no resampling), 16-bit, 2 identical
    ///   channels (the cable ties L and R together), also ~70% full scale,
    ///   1 s of silence at both ends (the phone's output stage wakes up /
    ///   fades without eating the header).
    pub fn for_format(format: AudioFormat) -> ExportOptions {
        let mut o = ExportOptions {
            format,
            sample_rate: 44100,
            bits: 16,
            channels: 1,
            amplitude: 0.7,
            lead_silence: 0.5,
            tail_silence: 0.5,
            timing: Timing::default(),
            embed_address: true,
        };
        if format == AudioFormat::Aiff {
            o.sample_rate = 48000;
            o.channels = 2;
            o.amplitude = 0.7;
            o.lead_silence = 1.0;
            o.tail_silence = 1.0;
        }
        o
    }
}

/// Comment stored in exported files, e.g. `APPLE1 ACI 0300.03FF`.
pub fn address_comment(segments: &[(u16, &[u8])]) -> String {
    let mut s = String::from("APPLE1 ACI");
    for (a, d) in segments {
        s.push_str(&format!(" {:04X}.{:04X}", a, *a as u32 + d.len() as u32 - 1));
    }
    s
}

/// Parse `APPLE1 ACI 0300.03FF E000.EFFF` back into ranges.
pub fn parse_address_comment(c: &str) -> Vec<(u16, u16)> {
    let mut v = Vec::new();
    let mut it = c.split_whitespace();
    if it.next() != Some("APPLE1") || it.next() != Some("ACI") {
        return v;
    }
    for r in it {
        if let Some((a, b)) = r.split_once('.') {
            if let (Ok(a), Ok(b)) = (u16::from_str_radix(a, 16), u16::from_str_radix(b, 16)) {
                v.push((a, b));
            }
        }
    }
    v
}

/// Produce the audio file bytes (format `o.format`) for the given segments.
pub fn export_wav_bytes(segments: &[(u16, &[u8])], o: &ExportOptions) -> Result<Vec<u8>, String> {
    let toggles = tape::rom_write_toggles(segments, o.timing)?;
    let samples = tape::render_toggles(
        &toggles,
        &RenderOptions {
            sample_rate: o.sample_rate,
            amplitude: o.amplitude,
            lead_silence: o.lead_silence,
            tail_silence: o.tail_silence,
        },
    );
    let comment = if o.embed_address { Some(address_comment(segments)) } else { None };
    Ok(wav::encode_audio(o.format, &samples, o.sample_rate, o.bits, o.channels, comment.as_deref()))
}

/// Write an audio file; the format follows `o.format` (callers normally use
/// `AudioFormat::from_path`).
pub fn export_wav(path: &Path, segments: &[(u16, &[u8])], o: &ExportOptions) -> Result<(), String> {
    let b = export_wav_bytes(segments, o)?;
    std::fs::write(path, b).map_err(|e| format!("{}: {e}", path.display()))
}

/// Same as `export_wav` with the per-format defaults chosen from the file
/// extension (`.aif`/`.aiff` -> smartphone AIFF profile, otherwise WAV).
pub fn export_audio(path: &Path, segments: &[(u16, &[u8])]) -> Result<(), String> {
    export_wav(path, segments, &ExportOptions::for_format(AudioFormat::from_path(path)))
}

#[derive(Debug, Clone)]
pub struct Imported {
    pub segments: Vec<DecodedSegment>,
    /// Ranges embedded by this emulator, if the file came from it.
    pub embedded: Vec<(u16, u16)>,
    pub sample_rate: u32,
    pub channels: usize,
    pub bits: u16,
    pub duration: f64,
}

pub fn import_wav_bytes(bytes: &[u8]) -> Result<Imported, String> {
    let a = wav::parse_audio(bytes).map_err(|e| e.to_string())?;
    let segments = tape::decode_samples(a.best_channel(), a.sample_rate);
    let embedded = a.comment.as_deref().map(parse_address_comment).unwrap_or_default();
    Ok(Imported {
        segments,
        embedded,
        sample_rate: a.sample_rate,
        channels: a.channels.len(),
        bits: a.bits,
        duration: a.duration_secs(),
    })
}

pub fn import_wav(path: &Path) -> Result<Imported, String> {
    let b = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    import_wav_bytes(&b)
}
