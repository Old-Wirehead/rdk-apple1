//! Minimal AIFF / AIFF-C reader and writer (no external dependency).
//!
//! AIFF is what Mike Willegal recommends for playing Apple-1 programs from
//! an iPod / smartphone into the ACI (Mimeo ACI guide, appendix A).
//! Reader: AIFF PCM 8/16/24/32-bit (big endian, signed), AIFF-C `NONE`,
//! `sowt` (little endian), `fl32`/`FL32`, `fl64`/`FL64`.
//! Writer: PCM 16 (or 8) bit, mono or several identical channels, optional
//! `ANNO` chunk (used to embed the load addresses, like ICMT for WAV).

use crate::wav::{Audio, WavError};

fn be16(b: &[u8], o: usize) -> u16 {
    u16::from_be_bytes([b[o], b[o + 1]])
}
fn be32(b: &[u8], o: usize) -> u32 {
    u32::from_be_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}

/// IEEE 754 80-bit extended (big endian) -> f64.
pub fn ext80_to_f64(b: &[u8]) -> f64 {
    let exp = (((b[0] as u16) << 8 | b[1] as u16) & 0x7FFF) as i32;
    let sign = if b[0] & 0x80 != 0 { -1.0 } else { 1.0 };
    let mut mant: u64 = 0;
    for &x in &b[2..10] {
        mant = mant << 8 | x as u64;
    }
    if exp == 0 && mant == 0 {
        return 0.0;
    }
    sign * (mant as f64) * 2f64.powi(exp - 16383 - 63)
}

/// f64 (positive integer sample rates) -> IEEE 754 80-bit extended.
pub fn f64_to_ext80(v: f64) -> [u8; 10] {
    let mut out = [0u8; 10];
    if v <= 0.0 {
        return out;
    }
    let e = v.log2().floor() as i32;
    let mant = (v / 2f64.powi(e) * 2f64.powi(63)) as u64; // integer bit explicit
    let exp = (e + 16383) as u16;
    out[0..2].copy_from_slice(&exp.to_be_bytes());
    out[2..10].copy_from_slice(&mant.to_be_bytes());
    out
}

pub fn is_aiff(bytes: &[u8]) -> bool {
    bytes.len() >= 12 && &bytes[0..4] == b"FORM" && (&bytes[8..12] == b"AIFF" || &bytes[8..12] == b"AIFC")
}

pub fn parse_aiff(bytes: &[u8]) -> Result<Audio, WavError> {
    let fmt = |s: &str| WavError::Format(format!("AIFF : {s}"));
    if !is_aiff(bytes) {
        return Err(fmt("en-tête FORM/AIFF absent"));
    }
    let aifc = &bytes[8..12] == b"AIFC";
    let mut pos = 12;
    let mut comm: Option<(usize, usize, u16, f64, [u8; 4])> = None;
    let mut ssnd: Option<(usize, usize)> = None;
    let mut comment: Option<String> = None;
    while pos + 8 <= bytes.len() {
        let id = &bytes[pos..pos + 4];
        let len = be32(bytes, pos + 4) as usize;
        let body = pos + 8;
        let end = (body + len).min(bytes.len());
        match id {
            b"COMM" => {
                if end - body < 18 {
                    return Err(fmt("chunk COMM trop court"));
                }
                let ch = be16(bytes, body) as usize;
                let frames = be32(bytes, body + 2) as usize;
                let bits = be16(bytes, body + 6);
                let rate = ext80_to_f64(&bytes[body + 8..body + 18]);
                let mut comp = *b"NONE";
                if aifc && end - body >= 22 {
                    comp.copy_from_slice(&bytes[body + 18..body + 22]);
                }
                comm = Some((ch, frames, bits, rate, comp));
            }
            b"SSND" => {
                if end - body < 8 {
                    return Err(fmt("chunk SSND trop court"));
                }
                let offset = be32(bytes, body) as usize;
                ssnd = Some((body + 8 + offset, end));
            }
            b"ANNO" | b"COMT" | b"NAME" => {
                let raw = if id == b"COMT" && end - body > 10 { &bytes[body + 10..end] } else { &bytes[body..end] };
                let t = String::from_utf8_lossy(raw).trim_end_matches('\0').trim().to_string();
                if t.starts_with("APPLE1 ACI") || comment.is_none() {
                    comment = Some(t);
                }
            }
            _ => {}
        }
        pos = body + len + (len & 1);
    }
    let (ch, frames, bits, rate, comp) = comm.ok_or_else(|| fmt("chunk COMM absent"))?;
    let (start, end) = ssnd.ok_or_else(|| fmt("chunk SSND absent"))?;
    if ch == 0 || !(1000.0..=1_000_000.0).contains(&rate) {
        return Err(fmt("paramètres invalides"));
    }
    let (float, little) = match &comp {
        b"NONE" | b"twos" => (false, false),
        b"sowt" => (false, true),
        b"fl32" | b"FL32" | b"fl64" | b"FL64" => (true, false),
        c => return Err(fmt(&format!("compression « {} » non gérée (exportez en PCM)", String::from_utf8_lossy(c)))),
    };
    let bits = if float && (comp == *b"fl64" || comp == *b"FL64") { 64 } else if float { 32 } else { bits };
    let bps = (bits as usize).div_ceil(8);
    if !(1..=8).contains(&bps) {
        return Err(fmt("taille d'échantillon invalide"));
    }
    let data = &bytes[start.min(end)..end];
    let n = frames.min(data.len() / (bps * ch));
    let mut channels = vec![Vec::with_capacity(n); ch];
    for f in 0..n {
        for (c, chan) in channels.iter_mut().enumerate() {
            let o = (f * ch + c) * bps;
            let s = &data[o..o + bps];
            let v = if float {
                if bps == 4 {
                    f32::from_be_bytes([s[0], s[1], s[2], s[3]])
                } else {
                    f64::from_be_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]) as f32
                }
            } else {
                let mut acc: i64 = 0;
                for k in 0..bps {
                    let byte = if little { s[bps - 1 - k] } else { s[k] };
                    acc = acc << 8 | byte as i64;
                }
                let shift = 64 - 8 * bps as u32;
                let signed = (acc << shift) >> shift;
                (signed as f64 / (1i64 << (8 * bps - 1)) as f64) as f32
            };
            chan.push(v);
        }
    }
    Ok(Audio { sample_rate: rate.round() as u32, channels, bits, comment })
}

/// Encode samples (-1..1) as AIFF, written identically on `channels`
/// channels (the Apple-1 cable ties L and R together, see README).
pub fn encode_aiff(samples: &[f32], sample_rate: u32, bits: u16, channels: u16, comment: Option<&str>) -> Vec<u8> {
    assert!(bits == 8 || bits == 16, "8 ou 16 bits seulement");
    let ch = channels.max(1) as usize;
    let bps = (bits / 8) as usize;
    let data_len = samples.len() * bps * ch;
    let mut chunks = Vec::new();
    // COMM
    chunks.extend_from_slice(b"COMM");
    chunks.extend_from_slice(&18u32.to_be_bytes());
    chunks.extend_from_slice(&(ch as u16).to_be_bytes());
    chunks.extend_from_slice(&(samples.len() as u32).to_be_bytes());
    chunks.extend_from_slice(&bits.to_be_bytes());
    chunks.extend_from_slice(&f64_to_ext80(sample_rate as f64));
    if let Some(c) = comment {
        let t = c.as_bytes();
        chunks.extend_from_slice(b"ANNO");
        chunks.extend_from_slice(&(t.len() as u32).to_be_bytes());
        chunks.extend_from_slice(t);
        if t.len() & 1 == 1 {
            chunks.push(0);
        }
    }
    chunks.extend_from_slice(b"SSND");
    chunks.extend_from_slice(&((8 + data_len) as u32).to_be_bytes());
    chunks.extend_from_slice(&0u32.to_be_bytes());
    chunks.extend_from_slice(&0u32.to_be_bytes());
    let mut out = Vec::with_capacity(12 + chunks.len() + data_len + 1);
    out.extend_from_slice(b"FORM");
    let form_len = 4 + chunks.len() + data_len + (data_len & 1);
    out.extend_from_slice(&(form_len as u32).to_be_bytes());
    out.extend_from_slice(b"AIFF");
    out.extend_from_slice(&chunks);
    for &s in samples {
        let s = s.clamp(-1.0, 1.0);
        for _ in 0..ch {
            if bits == 8 {
                out.push(((s * 127.0).round() as i8) as u8);
            } else {
                out.extend_from_slice(&((s * 32767.0).round() as i16).to_be_bytes());
            }
        }
    }
    if data_len & 1 == 1 {
        out.push(0);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ext80_roundtrip() {
        for r in [8000.0, 11025.0, 22050.0, 44100.0, 48000.0, 96000.0] {
            assert_eq!(ext80_to_f64(&f64_to_ext80(r)), r);
        }
        // 44100 Hz as written by Apple tools
        assert_eq!(f64_to_ext80(44100.0), [0x40, 0x0E, 0xAC, 0x44, 0, 0, 0, 0, 0, 0]);
    }
    #[test]
    fn aiff_roundtrip() {
        let s: Vec<f32> = (0..1001).map(|i| ((i as f32) * 0.01).sin() * 0.9).collect();
        for (bits, ch) in [(16u16, 1u16), (16, 2), (8, 1)] {
            let b = encode_aiff(&s, 48000, bits, ch, Some("APPLE1 ACI 0300.0310"));
            let a = parse_aiff(&b).unwrap();
            assert_eq!(a.sample_rate, 48000);
            assert_eq!(a.channels.len(), ch as usize);
            assert_eq!(a.channels[0].len(), s.len());
            assert_eq!(a.comment.as_deref(), Some("APPLE1 ACI 0300.0310"));
            let tol = if bits == 8 { 0.02 } else { 0.0002 };
            for (x, y) in a.channels[ch as usize - 1].iter().zip(&s) {
                assert!((x - y).abs() < tol);
            }
        }
    }
    #[test]
    fn aifc_sowt() {
        // AIFF-C little-endian 16-bit, 1 channel, 2 frames
        let mut b = Vec::new();
        b.extend_from_slice(b"FORM");
        b.extend_from_slice(&0u32.to_be_bytes());
        b.extend_from_slice(b"AIFCCOMM");
        b.extend_from_slice(&24u32.to_be_bytes());
        b.extend_from_slice(&1u16.to_be_bytes());
        b.extend_from_slice(&2u32.to_be_bytes());
        b.extend_from_slice(&16u16.to_be_bytes());
        b.extend_from_slice(&f64_to_ext80(22050.0));
        b.extend_from_slice(b"sowt");
        b.extend_from_slice(&[0, 0]);
        b.extend_from_slice(b"SSND");
        b.extend_from_slice(&12u32.to_be_bytes());
        b.extend_from_slice(&[0; 8]);
        b.extend_from_slice(&16384i16.to_le_bytes());
        b.extend_from_slice(&(-16384i16).to_le_bytes());
        let a = parse_aiff(&b).unwrap();
        assert_eq!(a.sample_rate, 22050);
        assert_eq!(a.channels[0], vec![0.5, -0.5]);
    }
}
