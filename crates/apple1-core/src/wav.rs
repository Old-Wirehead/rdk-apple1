//! Minimal RIFF/WAVE reader and writer (no external dependency).
//!
//! Reader: PCM 8 (unsigned) / 16 / 24 / 32-bit integer, 32/64-bit float,
//! `WAVE_FORMAT_EXTENSIBLE`, any number of channels, any sample rate.
//! Writer: mono PCM 8 or 16-bit, optional INFO/ICMT comment chunk (used to
//! embed the load address of an exported program).

use std::fmt;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub enum WavError {
    Io(io::Error),
    Format(String),
}

impl fmt::Display for WavError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WavError::Io(e) => write!(f, "erreur E/S : {e}"),
            WavError::Format(s) => write!(f, "fichier audio invalide : {s}"),
        }
    }
}
impl std::error::Error for WavError {}
impl From<io::Error> for WavError {
    fn from(e: io::Error) -> Self {
        WavError::Io(e)
    }
}

/// Decoded audio, one Vec per channel, samples in -1.0..=1.0.
#[derive(Debug, Clone)]
pub struct Audio {
    pub sample_rate: u32,
    pub channels: Vec<Vec<f32>>,
    pub bits: u16,
    /// Content of the INFO/ICMT chunk if present.
    pub comment: Option<String>,
}

impl Audio {
    /// Pick the channel with the most energy (robust against a dead or
    /// phase-inverted channel, which a plain average would not be).
    pub fn best_channel(&self) -> &[f32] {
        let mut best = 0;
        let mut best_e = -1.0f64;
        for (i, ch) in self.channels.iter().enumerate() {
            let e: f64 = ch.iter().map(|&s| (s as f64) * (s as f64)).sum();
            if e > best_e {
                best_e = e;
                best = i;
            }
        }
        &self.channels[best]
    }

    pub fn duration_secs(&self) -> f64 {
        self.channels.first().map(|c| c.len()).unwrap_or(0) as f64 / self.sample_rate as f64
    }
}

fn u16le(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn u32le(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}

pub fn parse_wav(bytes: &[u8]) -> Result<Audio, WavError> {
    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return Err(WavError::Format("en-tete RIFF/WAVE absent".into()));
    }
    let mut pos = 12;
    let mut fmt: Option<(u16, u16, u32, u16)> = None; // tag, channels, rate, bits
    let mut data: Option<&[u8]> = None;
    let mut comment = None;
    while pos + 8 <= bytes.len() {
        let id = &bytes[pos..pos + 4];
        let mut len = u32le(bytes, pos + 4) as usize;
        let body = pos + 8;
        if body + len > bytes.len() {
            len = bytes.len() - body; // truncated file: use what we have
        }
        let chunk = &bytes[body..body + len];
        match id {
            b"fmt " => {
                if len < 16 {
                    return Err(WavError::Format("chunk fmt trop court".into()));
                }
                let mut tag = u16le(chunk, 0);
                let ch = u16le(chunk, 2);
                let rate = u32le(chunk, 4);
                let bits = u16le(chunk, 14);
                if tag == 0xFFFE && len >= 26 {
                    tag = u16le(chunk, 24); // sub format GUID first 2 bytes
                }
                fmt = Some((tag, ch, rate, bits));
            }
            b"data" => data = Some(chunk),
            b"LIST"
                if len >= 4 && &chunk[0..4] == b"INFO" => {
                    let mut p = 4;
                    while p + 8 <= chunk.len() {
                        let sid = &chunk[p..p + 4];
                        let sl = u32le(chunk, p + 4) as usize;
                        let s = p + 8;
                        if s + sl > chunk.len() {
                            break;
                        }
                        if sid == b"ICMT" {
                            let txt = String::from_utf8_lossy(&chunk[s..s + sl]);
                            comment = Some(txt.trim_end_matches('\0').to_string());
                        }
                        p = s + sl + (sl & 1);
                    }
                }
            _ => {}
        }
        pos = body + len + (len & 1);
    }
    let (tag, nch, rate, bits) = fmt.ok_or_else(|| WavError::Format("chunk fmt manquant".into()))?;
    let data = data.ok_or_else(|| WavError::Format("chunk data manquant".into()))?;
    if nch == 0 || rate == 0 {
        return Err(WavError::Format("nombre de canaux ou frequence nul".into()));
    }
    let bps = (bits as usize).div_ceil(8);
    let frame = bps * nch as usize;
    if frame == 0 {
        return Err(WavError::Format("taille d'echantillon nulle".into()));
    }
    let frames = data.len() / frame;
    let mut channels = vec![Vec::with_capacity(frames); nch as usize];
    for f in 0..frames {
        for (c, chv) in channels.iter_mut().enumerate() {
            let o = f * frame + c * bps;
            let s = &data[o..o + bps];
            let v: f32 = match (tag, bps) {
                (1, 1) => (s[0] as f32 - 128.0) / 128.0,
                (1, 2) => i16::from_le_bytes([s[0], s[1]]) as f32 / 32768.0,
                (1, 3) => (((s[0] as i32) << 8 | (s[1] as i32) << 16 | (s[2] as i32) << 24) >> 8) as f32 / 8_388_608.0,
                (1, 4) => i32::from_le_bytes([s[0], s[1], s[2], s[3]]) as f32 / 2_147_483_648.0,
                (3, 4) => f32::from_le_bytes([s[0], s[1], s[2], s[3]]),
                (3, 8) => f64::from_le_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]) as f32,
                _ => return Err(WavError::Format(format!("format non supporte (tag {tag}, {bits} bits)"))),
            };
            chv.push(v);
        }
    }
    Ok(Audio { sample_rate: rate, channels, bits, comment })
}

pub fn read_wav(path: &Path) -> Result<Audio, WavError> {
    parse_audio(&std::fs::read(path)?)
}

/// Audio container formats supported for cassette files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Wav,
    Aiff,
}

impl AudioFormat {
    /// From a file name extension (`.aif`, `.aiff`, `.aifc` = AIFF).
    pub fn from_path(path: &Path) -> AudioFormat {
        match path.extension().and_then(|e| e.to_str()).map(|e| e.to_ascii_lowercase()).as_deref() {
            Some("aif") | Some("aiff") | Some("aifc") => AudioFormat::Aiff,
            _ => AudioFormat::Wav,
        }
    }
}

/// Parse a WAV or AIFF file (detected from its content).
pub fn parse_audio(bytes: &[u8]) -> Result<Audio, WavError> {
    if crate::aiff::is_aiff(bytes) {
        crate::aiff::parse_aiff(bytes)
    } else if bytes.len() >= 4 && &bytes[0..4] == b"RIFF" {
        parse_wav(bytes)
    } else {
        Err(WavError::Format("ni WAV (RIFF) ni AIFF (FORM)".into()))
    }
}

/// Encode samples in the given format; mono data is duplicated on every
/// channel (identical channels, never inverted).
pub fn encode_audio(format: AudioFormat, samples: &[f32], sample_rate: u32, bits: u16, channels: u16, comment: Option<&str>) -> Vec<u8> {
    match format {
        AudioFormat::Aiff => crate::aiff::encode_aiff(samples, sample_rate, bits, channels, comment),
        AudioFormat::Wav if channels <= 1 => encode_wav(samples, sample_rate, bits, comment),
        AudioFormat::Wav => {
            // identical channels, 16-bit
            let _ = bits;
            let mut b = encode_wav_stereo16(samples, samples, sample_rate);
            if let Some(c) = comment {
                // append LIST/INFO/ICMT after data (allowed by RIFF)
                let mut txt = c.as_bytes().to_vec();
                txt.push(0);
                if txt.len() % 2 == 1 {
                    txt.push(0);
                }
                let mut list = b"INFOICMT".to_vec();
                list.extend_from_slice(&(txt.len() as u32).to_le_bytes());
                list.extend_from_slice(&txt);
                b.extend_from_slice(b"LIST");
                b.extend_from_slice(&(list.len() as u32).to_le_bytes());
                b.extend_from_slice(&list);
                let riff = (b.len() - 8) as u32;
                b[4..8].copy_from_slice(&riff.to_le_bytes());
            }
            b
        }
    }
}

/// Encode mono samples (-1..1) as a PCM WAV file in memory.
pub fn encode_wav(samples: &[f32], sample_rate: u32, bits: u16, comment: Option<&str>) -> Vec<u8> {
    assert!(bits == 8 || bits == 16, "8 ou 16 bits seulement");
    let bps = (bits / 8) as usize;
    let data_len = samples.len() * bps;
    let mut list = Vec::new();
    if let Some(c) = comment {
        let mut txt = c.as_bytes().to_vec();
        txt.push(0);
        if txt.len() % 2 == 1 {
            txt.push(0);
        }
        list.extend_from_slice(b"INFO");
        list.extend_from_slice(b"ICMT");
        list.extend_from_slice(&(txt.len() as u32).to_le_bytes());
        list.extend_from_slice(&txt);
    }
    let mut out = Vec::with_capacity(44 + data_len + list.len() + 8);
    out.extend_from_slice(b"RIFF");
    let riff_len = 4 + 24 + 8 + data_len + (data_len & 1) + if list.is_empty() { 0 } else { 8 + list.len() };
    out.extend_from_slice(&(riff_len as u32).to_le_bytes());
    out.extend_from_slice(b"WAVE");
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&sample_rate.to_le_bytes());
    out.extend_from_slice(&(sample_rate * bps as u32).to_le_bytes());
    out.extend_from_slice(&(bps as u16).to_le_bytes());
    out.extend_from_slice(&bits.to_le_bytes());
    if !list.is_empty() {
        out.extend_from_slice(b"LIST");
        out.extend_from_slice(&(list.len() as u32).to_le_bytes());
        out.extend_from_slice(&list);
    }
    out.extend_from_slice(b"data");
    out.extend_from_slice(&(data_len as u32).to_le_bytes());
    for &s in samples {
        let s = s.clamp(-1.0, 1.0);
        if bits == 8 {
            out.push((s * 127.0 + 128.0).round().clamp(0.0, 255.0) as u8);
        } else {
            out.extend_from_slice(&((s * 32767.0).round() as i16).to_le_bytes());
        }
    }
    if data_len & 1 == 1 {
        out.push(0);
    }
    out
}

pub fn write_wav(path: &Path, samples: &[f32], sample_rate: u32, bits: u16, comment: Option<&str>) -> io::Result<()> {
    std::fs::write(path, encode_wav(samples, sample_rate, bits, comment))
}

/// Encode interleaved multi-channel 16-bit data (used by tests to produce stereo files).
pub fn encode_wav_stereo16(left: &[f32], right: &[f32], sample_rate: u32) -> Vec<u8> {
    let n = left.len().min(right.len());
    let data_len = n * 4;
    let mut out = Vec::new();
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&((36 + data_len) as u32).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&sample_rate.to_le_bytes());
    out.extend_from_slice(&(sample_rate * 4).to_le_bytes());
    out.extend_from_slice(&4u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&(data_len as u32).to_le_bytes());
    for i in 0..n {
        out.extend_from_slice(&((left[i].clamp(-1.0, 1.0) * 32767.0) as i16).to_le_bytes());
        out.extend_from_slice(&((right[i].clamp(-1.0, 1.0) * 32767.0) as i16).to_le_bytes());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip_8_16() {
        let s: Vec<f32> = (0..1000).map(|i| ((i as f32) * 0.05).sin() * 0.7).collect();
        for bits in [8u16, 16] {
            let b = encode_wav(&s, 22050, bits, Some("APPLE1 0300.03FF"));
            let a = parse_wav(&b).unwrap();
            assert_eq!(a.sample_rate, 22050);
            assert_eq!(a.channels.len(), 1);
            assert_eq!(a.channels[0].len(), s.len());
            assert_eq!(a.comment.as_deref(), Some("APPLE1 0300.03FF"));
            let tol = if bits == 8 { 0.02 } else { 0.0002 };
            for (x, y) in s.iter().zip(&a.channels[0]) {
                assert!((x - y).abs() < tol);
            }
        }
    }
    #[test]
    fn stereo_best_channel() {
        let l = vec![0.0f32; 100];
        let r: Vec<f32> = (0..100).map(|i| if i % 2 == 0 { 0.5 } else { -0.5 }).collect();
        let a = parse_wav(&encode_wav_stereo16(&l, &r, 48000)).unwrap();
        assert_eq!(a.channels.len(), 2);
        assert!(a.best_channel()[0] > 0.4);
    }
}
