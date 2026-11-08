//! Woz Monitor style hex dump text files, e.g.
//!
//! ```text
//! 0300: A9 00 20 EF FF
//! : 4C 00 03          (continuation, as accepted by the Woz monitor)
//! 0310: 8D
//! 0300R               (optional run command, recorded as `run`)
//! ```
//!
//! Comments start with `;` or `#` or `//`.

use std::fmt::Write as _;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HexDump {
    /// Contiguous blocks (start address, bytes), in file order.
    pub blocks: Vec<(u16, Vec<u8>)>,
    /// Address given by a trailing `XXXXR` command, if any.
    pub run: Option<u16>,
}

impl HexDump {
    pub fn total_len(&self) -> usize {
        self.blocks.iter().map(|b| b.1.len()).sum()
    }

    /// Lowest and highest address covered.
    pub fn range(&self) -> Option<(u16, u16)> {
        let lo = self.blocks.iter().map(|b| b.0).min()?;
        let hi = self.blocks.iter().map(|b| b.0 as u32 + b.1.len() as u32 - 1).max()?;
        Some((lo, hi as u16))
    }

    /// Flatten into one contiguous image from the lowest to the highest
    /// address (gaps filled with `fill`).
    pub fn flatten(&self, fill: u8) -> Option<(u16, Vec<u8>)> {
        let (lo, hi) = self.range()?;
        let mut v = vec![fill; hi as usize - lo as usize + 1];
        for (a, d) in &self.blocks {
            let o = (*a - lo) as usize;
            v[o..o + d.len()].copy_from_slice(d);
        }
        Some((lo, v))
    }
}

fn is_hex(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit())
}

/// Parse a hex dump. Errors carry the 1-based line number.
pub fn parse(text: &str) -> Result<HexDump, String> {
    let mut hd = HexDump::default();
    let mut addr: Option<u16> = None;
    for (ln, raw) in text.lines().enumerate() {
        let mut line = raw;
        for c in [";", "#", "//"] {
            if let Some(p) = line.find(c) {
                line = &line[..p];
            }
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let upper = line.to_ascii_uppercase();
        // run command "0300R" (alone on a line)
        if upper.len() >= 2 && upper.ends_with('R') && is_hex(&upper[..upper.len() - 1]) && !upper.contains(':') {
            hd.run = u16::from_str_radix(&upper[..upper.len() - 1], 16).ok();
            continue;
        }
        let (head, rest) = match upper.find(':') {
            Some(p) => (upper[..p].trim().to_string(), upper[p + 1..].to_string()),
            None => (String::new(), upper.clone()),
        };
        if !head.is_empty() {
            if !is_hex(&head) || head.len() > 4 {
                return Err(format!("ligne {}: adresse invalide '{}'", ln + 1, head));
            }
            addr = Some(u16::from_str_radix(&head, 16).unwrap());
            hd.blocks.push((addr.unwrap(), Vec::new()));
        } else if upper.find(':').is_some() || addr.is_some() {
            // continuation line ": AA BB" or bare bytes after an address
            if hd.blocks.is_empty() {
                return Err(format!("ligne {}: donnees sans adresse", ln + 1));
            }
        } else {
            return Err(format!("ligne {}: donnees sans adresse", ln + 1));
        }
        for tok in rest.split_whitespace() {
            if !is_hex(tok) || tok.len() > 2 {
                return Err(format!("ligne {}: octet invalide '{}'", ln + 1, tok));
            }
            let b = u8::from_str_radix(tok, 16).unwrap();
            hd.blocks.last_mut().unwrap().1.push(b);
            addr = addr.map(|a| a.wrapping_add(1));
        }
    }
    // merge adjacent blocks
    let mut merged: Vec<(u16, Vec<u8>)> = Vec::new();
    for (a, d) in hd.blocks.into_iter().filter(|b| !b.1.is_empty()) {
        if let Some(last) = merged.last_mut() {
            if last.0 as usize + last.1.len() == a as usize {
                last.1.extend(d);
                continue;
            }
        }
        merged.push((a, d));
    }
    hd.blocks = merged;
    Ok(hd)
}

/// Format memory as a Woz monitor dump, 8 bytes per line (the format the
/// Woz monitor itself prints and accepts back).
pub fn format(start: u16, data: &[u8], run: Option<u16>) -> String {
    let mut s = String::new();
    for (i, chunk) in data.chunks(8).enumerate() {
        let a = start.wrapping_add((i * 8) as u16);
        let _ = write!(s, "{a:04X}:");
        for b in chunk {
            let _ = write!(s, " {b:02X}");
        }
        s.push('\n');
    }
    if let Some(r) = run {
        let _ = writeln!(s, "{r:04X}R");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_formats() {
        let t = "; test\n0300: A9 00 20 EF FF\n: 4C 00 03\n0310:8D 8D # c\n0300R\n";
        let h = parse(t).unwrap();
        assert_eq!(h.blocks, vec![(0x0300, vec![0xA9, 0, 0x20, 0xEF, 0xFF, 0x4C, 0, 3]), (0x0310, vec![0x8D, 0x8D])]);
        assert_eq!(h.run, Some(0x0300));
        let (lo, flat) = h.flatten(0).unwrap();
        assert_eq!(lo, 0x300);
        assert_eq!(flat.len(), 0x12);
    }
    #[test]
    fn roundtrip() {
        let data: Vec<u8> = (0..=255).collect();
        let s = format(0x0280, &data, Some(0x0280));
        let h = parse(&s).unwrap();
        assert_eq!(h.blocks, vec![(0x0280, data)]);
        assert_eq!(h.run, Some(0x0280));
    }
    #[test]
    fn errors() {
        assert!(parse("A9 00").is_err());
        assert!(parse("0300: ZZ").is_err());
    }
}
