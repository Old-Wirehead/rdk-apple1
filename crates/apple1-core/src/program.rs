//! Loading and saving programs in the supported file formats:
//! Woz monitor hex dumps (text), raw binaries and ACI cassette WAV files.

use std::path::Path;

use crate::aciwav::{self, ExportOptions};
use crate::hexdump;

/// A program: one or more memory blocks, plus an optional run address.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Program {
    pub blocks: Vec<(u16, Vec<u8>)>,
    pub run: Option<u16>,
    /// Human readable notes (decoder warnings...).
    pub notes: Vec<String>,
}

impl Program {
    pub fn ranges(&self) -> Vec<(u16, u16)> {
        self.blocks.iter().map(|(a, d)| (*a, (*a as u32 + d.len() as u32 - 1) as u16)).collect()
    }
    pub fn total_len(&self) -> usize {
        self.blocks.iter().map(|b| b.1.len()).sum()
    }
    /// The Woz monitor commands to type on a real Apple-1 to read the tape.
    pub fn aci_read_command(&self) -> String {
        let mut s = String::new();
        for (i, (a, b)) in self.ranges().iter().enumerate() {
            if i > 0 {
                s.push(' ');
            }
            s.push_str(&crate::tape::aci_range_text(*a, *b));
            s.push('R');
        }
        s
    }
}

/// File kinds, from the extension (then content sniffing).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Wav,
    Hex,
    Bin,
    /// 2-byte little-endian start address, then the code (KickC / P-LAB demos).
    Prg,
}

pub fn kind_of(path: &Path, bytes: Option<&[u8]>) -> Kind {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    match ext.as_str() {
        "wav" | "wave" | "aif" | "aiff" | "aifc" => return Kind::Wav,
        "prg" => return Kind::Prg,
        "txt" | "hex" | "mon" | "woz" | "apl" => return Kind::Hex,
        "bin" | "raw" | "rom" | "o" => return Kind::Bin,
        _ => {}
    }
    if let Some(b) = bytes {
        if b.starts_with(b"RIFF") || crate::aiff::is_aiff(b) {
            return Kind::Wav;
        }
        if let Ok(t) = std::str::from_utf8(b) {
            if hexdump::parse(t).map(|h| h.total_len() > 0).unwrap_or(false) {
                return Kind::Hex;
            }
        }
    }
    Kind::Bin
}

/// Parse an address in hex, with optional `$` / `0x` prefix.
pub fn parse_addr(s: &str) -> Result<u16, String> {
    let t = s.trim().trim_start_matches('$');
    let t = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")).unwrap_or(t);
    u16::from_str_radix(t, 16).map_err(|_| format!("adresse invalide : « {s} » (hexadécimal attendu, ex. 0300)"))
}

/// Parse a Woz style range `300.3FF` (inclusive).
/// Parse `300.3FF` (a trailing Woz Monitor `R` or `W` is accepted, so a
/// range can be copied from the ACI command line, e.g. `4A.00FFR`).
pub fn parse_range(s: &str) -> Result<(u16, u16), String> {
    let s = s.trim().trim_end_matches(['R', 'r', 'W', 'w']);
    let (a, b) = s.split_once('.').ok_or_else(|| format!("plage invalide : « {s} » (ex. 300.3FF)"))?;
    let (a, b) = (parse_addr(a)?, parse_addr(b)?);
    if b < a {
        return Err(format!("plage invalide : « {s} » (fin < début)"));
    }
    Ok((a, b))
}

/// Parse a list of ranges separated by spaces or commas, e.g. the Star Trek
/// ACI line `4A.00FFR 300.FFFR`, or glued after each R/W as printed in the
/// 1976 BASIC manual (`4A.00FFW800.FFFW`).
pub fn parse_ranges(s: &str) -> Result<Vec<(u16, u16)>, String> {
    let mut parts = Vec::new();
    let mut cur = String::new();
    for c in s.chars() {
        if c.is_whitespace() || c == ',' {
            parts.push(std::mem::take(&mut cur));
        } else {
            cur.push(c);
            if matches!(c, 'R' | 'r' | 'W' | 'w') {
                parts.push(std::mem::take(&mut cur));
            }
        }
    }
    parts.push(cur);
    parts.iter().filter(|x| !x.trim().is_empty()).map(|x| parse_range(x)).collect()
}

/// Split `file@ADDR` into path and address.
pub fn split_at_addr(spec: &str) -> Result<(String, Option<u16>), String> {
    if let Some((p, a)) = spec.rsplit_once('@') {
        if !p.is_empty() && !a.is_empty() && a.chars().all(|c| c.is_ascii_hexdigit() || c == '$' || c == 'x') {
            return Ok((p.to_string(), Some(parse_addr(a)?)));
        }
    }
    Ok((spec.to_string(), None))
}

/// Load a program from bytes. `addr` overrides / supplies the load address
/// (mandatory for raw binaries and for WAV files not made by this emulator).
pub fn load_bytes(bytes: &[u8], kind: Kind, addr: Option<u16>) -> Result<Program, String> {
    match kind {
        Kind::Hex => {
            let t = std::str::from_utf8(bytes).map_err(|_| "le fichier hexadécimal n'est pas du texte".to_string())?;
            let h = hexdump::parse(t)?;
            if h.total_len() == 0 {
                return Err("aucun octet trouvé dans le fichier hexadécimal".into());
            }
            let mut blocks = h.blocks.clone();
            if let Some(a) = addr {
                // relocate: keep the relative layout
                let base = blocks[0].0;
                for b in blocks.iter_mut() {
                    b.0 = a.wrapping_add(b.0.wrapping_sub(base));
                }
            }
            Ok(Program { blocks, run: h.run, notes: vec![] })
        }
        Kind::Prg => {
            if bytes.len() < 3 {
                return Err("fichier .prg trop court (adresse de 2 octets, puis le code)".into());
            }
            let start = u16::from_le_bytes([bytes[0], bytes[1]]);
            let data = &bytes[2..];
            let a = addr.unwrap_or(start);
            if a as usize + data.len() > 0x10000 {
                return Err("le binaire dépasse $FFFF".into());
            }
            // the start address is also the entry point
            Ok(Program { blocks: vec![(a, data.to_vec())], run: Some(a), notes: vec![] })
        }
        Kind::Bin => {
            let a = addr.ok_or("binaire brut : indiquez l'adresse de chargement (FICHIER@ADR, ex. prog.bin@300)")?;
            if bytes.is_empty() {
                return Err("fichier vide".into());
            }
            if a as usize + bytes.len() > 0x10000 {
                return Err("le binaire dépasse $FFFF".into());
            }
            Ok(Program { blocks: vec![(a, bytes.to_vec())], run: None, notes: vec![] })
        }
        Kind::Wav => {
            let imp = aciwav::import_wav_bytes(bytes)?;
            wav_to_program(&imp, addr)
        }
    }
}

/// Assign addresses to decoded tape segments.
pub fn wav_to_program(imp: &aciwav::Imported, addr: Option<u16>) -> Result<Program, String> {
    if imp.segments.is_empty() {
        return Err(format!(
            "aucun enregistrement ACI trouvé ({} Hz, {} canal(aux), {} bits, {:.1} s) : vérifiez le volume, l'en-tête de 10 s et le fichier",
            imp.sample_rate, imp.channels, imp.bits, imp.duration
        ));
    }
    let mut p = Program::default();
    let mut next = addr;
    for (i, s) in imp.segments.iter().enumerate() {
        let a = match (addr, imp.embedded.get(i)) {
            (Some(_), _) => next.unwrap(),
            (None, Some(&(a, _))) => a,
            (None, None) => {
                return Err(format!(
                    "{} octet(s) décodé(s), mais l'adresse n'est pas connue (fichier d'un autre outil) : indiquez-la (ex. --addr 300 ou FICHIER.wav@300)",
                    s.data.len()
                ))
            }
        };
        if let Some(&(ea, eb)) = imp.embedded.get(i) {
            let want = eb as usize - ea as usize + 1;
            if want != s.data.len() {
                p.notes.push(format!("segment {} : {} octets décodés, {} attendus", i + 1, s.data.len(), want));
            }
        }
        if s.weak_bits > 0 {
            p.notes.push(format!("segment {} : {} bit(s) douteux", i + 1, s.weak_bits));
        }
        if s.extra_bits > 1 {
            p.notes.push(format!("segment {} : {} bit(s) en trop ignorés en fin d'enregistrement", i + 1, s.extra_bits));
        }
        next = Some(a.wrapping_add(s.data.len() as u16));
        p.blocks.push((a, s.data.clone()));
    }
    Ok(p)
}

/// Place each decoded record at the given range (the ACI read line of the
/// program, e.g. `4A.00FF 300.FFF`): one range per record.
pub fn wav_with_ranges(imp: &aciwav::Imported, ranges: &[(u16, u16)]) -> Result<Program, String> {
    if ranges.len() != imp.segments.len() {
        return Err(format!("{} plage(s) indiquée(s) pour {} enregistrement(s) sur la cassette", ranges.len(), imp.segments.len()));
    }
    let mut imp = imp.clone();
    imp.embedded = ranges.to_vec();
    wav_to_program(&imp, None)
}

/// Integer BASIC: 4 KB at $E000, starting with `JMP` (cold start).
pub const INTEGER_BASIC: (u16, u16) = (0xE000, 0xEFFF);

/// How the layout of a tape without addresses was guessed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Guess {
    /// One 4096-byte record starting with `JMP`: Integer BASIC itself.
    IntegerBasic,
    /// Integer BASIC program: zero page `4A.00FF`, then the program, whose
    /// range is given by the saved pointers (LOMEM/HIMEM or PP/HIMEM).
    BasicPointers,
    /// Integer BASIC program whose pointers do not match the second
    /// record: placed at $0300 (the most common layout), to be checked.
    BasicDefault,
}

/// Guess where the records of a tape made by another tool go (for example
/// the archive.org "Apple1Cassettes" recordings, which carry no address).
/// Only Integer BASIC and Integer BASIC programs are recognised; machine
/// code programs need their address from the user.
pub fn guess_layout(imp: &aciwav::Imported) -> Option<(Vec<(u16, u16)>, Guess)> {
    let seg = &imp.segments;
    if seg.len() == 1 && seg[0].data.len() == 4096 && seg[0].data[0] == 0x4C {
        return Some((vec![INTEGER_BASIC], Guess::IntegerBasic));
    }
    // An Integer BASIC SAVE writes $4A-$00FF (182 bytes), then the program.
    if seg.len() != 2 || seg[0].data.len() != 0x100 - 0x4A || seg[1].data.is_empty() {
        return None;
    }
    let z = &seg[0].data;
    let word = |zp: usize| z[zp - 0x4A] as usize | (z[zp - 0x4A + 1] as usize) << 8;
    let (lomem, himem, pp) = (word(0x4A), word(0x4C), word(0xCA));
    let n = seg[1].data.len();
    let zp = (0x4A, 0xFF);
    for start in [lomem, pp] {
        if (0x200..0xE000).contains(&start) && himem <= 0xE000 && himem > start && himem - start == n {
            return Some((vec![zp, (start as u16, (himem - 1) as u16)], Guess::BasicPointers));
        }
    }
    // The saved block always ends at HIMEM and contains the program (PP):
    // Football has LOMEM = $0800 but was saved from $0400.
    if himem <= 0xE000 && himem >= n + 0x200 && himem - n <= pp && pp < himem {
        return Some((vec![zp, ((himem - n) as u16, (himem - 1) as u16)], Guess::BasicPointers));
    }
    (0x300 + n <= 0xE000).then(|| (vec![zp, (0x300, (0x300 + n - 1) as u16)], Guess::BasicDefault))
}

pub fn load_file(path: &Path, addr: Option<u16>) -> Result<Program, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("{} : {e}", path.display()))?;
    if path.extension().and_then(|e| e.to_str()).is_some_and(|e| e.eq_ignore_ascii_case("prg")) {
        if bytes.len() < 3 {
            return Err(format!("{} : fichier .prg trop court", path.display()));
        }
        let start = u16::from_le_bytes([bytes[0], bytes[1]]);
        return Ok(Program { blocks: vec![(start, bytes[2..].to_vec())], run: Some(start), notes: vec![] });
    }
    let k = kind_of(path, Some(&bytes));
    load_bytes(&bytes, k, addr).map_err(|e| format!("{} : {e}", path.display()))
}

/// Save memory blocks to a file whose format is chosen by extension
/// (.wav / .aif / .aiff = ACI cassette, .txt/.hex = hex dump, anything
/// else = binary). For audio, the per-format defaults of
/// `ExportOptions::for_format` apply when the extension's format differs
/// from `wav.format` (only timing and embed_address are kept).
pub fn save_file(path: &Path, p: &Program, wav: &ExportOptions) -> Result<Kind, String> {
    let k = kind_of(path, None);
    let err = |e: std::io::Error| format!("{} : {e}", path.display());
    match k {
        Kind::Wav => {
            let segs: Vec<(u16, &[u8])> = p.blocks.iter().map(|(a, d)| (*a, d.as_slice())).collect();
            let fmt = crate::wav::AudioFormat::from_path(path);
            let o = if fmt == wav.format {
                *wav
            } else {
                ExportOptions { timing: wav.timing, embed_address: wav.embed_address, ..ExportOptions::for_format(fmt) }
            };
            aciwav::export_wav(path, &segs, &o)?;
        }
        Kind::Hex => {
            let mut s = String::new();
            for (i, (a, d)) in p.blocks.iter().enumerate() {
                let run = if i + 1 == p.blocks.len() { p.run } else { None };
                s.push_str(&hexdump::format(*a, d, run));
            }
            std::fs::write(path, s).map_err(err)?;
        }
        Kind::Prg => {
            if p.blocks.len() != 1 {
                return Err("un .prg ne peut contenir qu'une seule plage".into());
            }
            let mut v = p.blocks[0].0.to_le_bytes().to_vec();
            v.extend_from_slice(&p.blocks[0].1);
            std::fs::write(path, v).map_err(err)?;
        }
        Kind::Bin => {
            if p.blocks.len() != 1 {
                return Err("un binaire brut ne peut contenir qu'une seule plage".into());
            }
            std::fs::write(path, &p.blocks[0].1).map_err(err)?;
        }
    }
    Ok(k)
}

#[cfg(test)]
mod tests {
    #[test]
    fn prg_has_a_start_address_header() {
        use super::*;
        // P-LAB / KickC demos: $0280 header, then the code (not a hex dump)
        let bytes = [0x80, 0x02, 0xA9, 0x41, 0x60];
        assert_eq!(kind_of(Path::new("demo.prg"), Some(&bytes)), Kind::Prg);
        let p = load_bytes(&bytes, Kind::Prg, None).unwrap();
        assert_eq!(p.blocks, vec![(0x0280, vec![0xA9, 0x41, 0x60])]);
        assert_eq!(p.run, Some(0x0280));
        // relocated on request
        let q = load_bytes(&bytes, Kind::Prg, Some(0x0400)).unwrap();
        assert_eq!((q.blocks[0].0, q.run), (0x0400, Some(0x0400)));
        assert!(load_bytes(&[0x80, 0x02], Kind::Prg, None).is_err());
        // saved back with its header
        let dir = std::env::temp_dir().join(format!("rdk-prg-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("x.prg");
        save_file(&f, &p, &Default::default()).unwrap();
        assert_eq!(std::fs::read(&f).unwrap(), bytes);
        assert_eq!(load_file(&f, None).unwrap().blocks, p.blocks);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn ranges_as_in_basic_manual() {
        let r = super::parse_ranges("4A.00FFW800.FFFW").unwrap();
        assert_eq!(r, vec![(0x4A, 0xFF), (0x800, 0xFFF)]);
        let r = super::parse_ranges("4A.00FFR 300.FFFR, E000.EFFF").unwrap();
        assert_eq!(r, vec![(0x4A, 0xFF), (0x300, 0xFFF), (0xE000, 0xEFFF)]);
    }

    use super::*;
    #[test]
    fn addr_and_ranges() {
        assert_eq!(parse_addr("$300").unwrap(), 0x300);
        assert_eq!(parse_addr("0xE000").unwrap(), 0xE000);
        assert_eq!(parse_range("300.3FF").unwrap(), (0x300, 0x3FF));
        assert!(parse_range("3FF.300").is_err());
        assert_eq!(split_at_addr("a.bin@300").unwrap(), ("a.bin".into(), Some(0x300)));
        assert_eq!(split_at_addr("a.bin").unwrap(), ("a.bin".into(), None));
    }
    #[test]
    fn wav_roundtrip_with_embedded_addresses() {
        let d1: Vec<u8> = (0..20).collect();
        let w = aciwav::export_wav_bytes(&[(0x0280, &d1)], &ExportOptions::default()).unwrap();
        let p = load_bytes(&w, Kind::Wav, None).unwrap();
        assert_eq!(p.blocks, vec![(0x0280, d1.clone())]);
        assert_eq!(p.aci_read_command(), "0280.0293R");
        let p = load_bytes(&w, Kind::Wav, Some(0x1000)).unwrap();
        assert_eq!(p.blocks[0].0, 0x1000);
        let w2 = aciwav::export_wav_bytes(&[(0x0280, &d1)], &ExportOptions { embed_address: false, ..Default::default() }).unwrap();
        assert!(load_bytes(&w2, Kind::Wav, None).is_err());
    }
    #[test]
    fn guessed_layouts() {
        use crate::tape::DecodedSegment;
        let seg = |d: Vec<u8>| DecodedSegment { data: d, ..DecodedSegment::default() };
        let imp = |s: Vec<DecodedSegment>| aciwav::Imported { segments: s, embedded: vec![], sample_rate: 44100, channels: 1, bits: 16, duration: 1.0 };
        let mut rom = vec![0u8; 4096];
        rom[0] = 0x4C;
        assert_eq!(guess_layout(&imp(vec![seg(rom.clone())])), Some((vec![INTEGER_BASIC], Guess::IntegerBasic)));
        rom[0] = 0;
        assert_eq!(guess_layout(&imp(vec![seg(rom)])), None);
        // Hamurabi from Apple1Cassettes: LOMEM $0300, HIMEM $1000, 3328 bytes
        let mut zp = vec![0u8; 182];
        zp[0..4].copy_from_slice(&[0x00, 0x03, 0x00, 0x10]);
        zp[0x80..0x82].copy_from_slice(&[0xF3, 0x04]);
        let g = guess_layout(&imp(vec![seg(zp.clone()), seg(vec![0; 3328])])).unwrap();
        assert_eq!(g, (vec![(0x4A, 0xFF), (0x300, 0xFFF)], Guess::BasicPointers));
        // PP..HIMEM saved (program only)
        let g = guess_layout(&imp(vec![seg(zp.clone()), seg(vec![0; 0x1000 - 0x4F3])])).unwrap();
        assert_eq!(g, (vec![(0x4A, 0xFF), (0x4F3, 0xFFF)], Guess::BasicPointers));
        // Football: LOMEM $0800, HIMEM $3000, PP $173B, saved from $0400
        let mut fb = vec![0u8; 182];
        fb[0..4].copy_from_slice(&[0x00, 0x08, 0x00, 0x30]);
        fb[0x80..0x82].copy_from_slice(&[0x3B, 0x17]);
        let g = guess_layout(&imp(vec![seg(fb), seg(vec![0; 11264])])).unwrap();
        assert_eq!(g, (vec![(0x4A, 0xFF), (0x400, 0x2FFF)], Guess::BasicPointers));
        let g = guess_layout(&imp(vec![seg(zp), seg(vec![0; 100])])).unwrap();
        assert_eq!(g, (vec![(0x4A, 0xFF), (0x300, 0x363)], Guess::BasicDefault));
        // ranges given by the user, one per record
        let i = imp(vec![seg(vec![1; 182]), seg(vec![2; 16])]);
        let p = wav_with_ranges(&i, &parse_ranges("4A.00FFR300.30FR").unwrap()).unwrap();
        assert_eq!(p.ranges(), vec![(0x4A, 0xFF), (0x300, 0x30F)]);
        assert!(wav_with_ranges(&i, &[(0x300, 0x30F)]).is_err());
    }
    #[test]
    fn hex_relocation() {
        let p = load_bytes(b"0300: 01 02\n0310: 03\n", Kind::Hex, Some(0x1000)).unwrap();
        assert_eq!(p.blocks, vec![(0x1000, vec![1, 2]), (0x1010, vec![3])]);
    }
}
