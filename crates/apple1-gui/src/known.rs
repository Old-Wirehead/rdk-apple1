//! Known tapes: when a file of the library is recognised (same decoded
//! records as one of the Apple1Cassettes tapes of archive.org), its ⚙
//! parameters are filled in automatically.
//!
//! The table holds only sizes and SHA-256 fingerprints of the decoded
//! records, plus the parameters: no program data is shipped.

use std::path::Path;

use apple1_core::aciwav::Imported;
use sha2::{Digest, Sha256};

use crate::params::{ProgKind, ProgParams};

#[derive(Debug, PartialEq, Eq)]
pub struct Known {
    pub title: &'static str,
    /// Name of the file on archive.org (Apple1Cassettes).
    pub file: &'static str,
    /// File names (lower case, letters and digits only) used when the
    /// content differs (another recording of the same tape).
    pub names: &'static [&'static str],
    /// (size, SHA-256) of each record.
    pub records: &'static [(usize, &'static str)],
    pub ranges: &'static str,
    /// Integer BASIC program (needs BASIC, started by E2B3R + RUN).
    pub basic: bool,
    pub run_cmd: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum How {
    /// Same records, byte for byte.
    Content,
    /// Same file name and same record sizes.
    Name,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Match {
    pub known: &'static Known,
    pub how: How,
}

pub static TABLE: &[Known] = &[
    Known { title: "Apple 30th Anniversary", file: "APPLE30TH.wav", names: &["apple30th"], records: &[(3456, "ff0ff5ea82a2b2c4df2ba36d2fc11957b47b073e1ef0dc7ae19683f646e8eeb0")], ranges: "0280.0FFF", basic: false, run_cmd: "280R" },
    Known { title: "Microchess (ASM)", file: "ASMmchess.wav", names: &["asmmchess"], records: &[(2304, "26cec3af6187a0cbf9b28e615356a78f61d0f57cf768ade7efbc02f1eb404e7d")], ranges: "0300.0BFF", basic: false, run_cmd: "300R" },
    Known { title: "Acey Ducey", file: "AceyDucey.wav", names: &["aceyducey"], records: &[(182, "74a14aaaef3df615056dee451ced4110036f353c1c04adc5c38ae6d8f2e53420"), (2048, "6391bd049d09d231af75f2175b72a0debad0fc1a2e9f873a3e65a6d6a0b09267")], ranges: "004A.00FF 0800.0FFF", basic: true, run_cmd: "E2B3R\\nRUN" },
    Known { title: "Apple-1 Enhanced Checkers", file: "AppleiEnhancedCheckers.wav", names: &["appleienhancedcheckers", "appleenhancedcheckers", "checkers"], records: &[(182, "669d232d82367f11355b3fa8482e6f567e0bc1e79f7e54c7714e4b3e4086681f"), (3328, "2e2a65273015efd15db2bb69dfbc30cc52544b39fb191d7cb92f874916050839")], ranges: "004A.00FF 0300.0FFF", basic: true, run_cmd: "E2B3R\\nRUN" },
    Known { title: "Integer BASIC", file: "BASIC.wav", names: &["basic", "integerbasic", "apple1basic"], records: &[(4096, "bf80009454610a1066489da635a8afb51ad42442d307251896a53bedbeaadd46")], ranges: "E000.EFFF", basic: false, run_cmd: "E000R" },
    Known { title: "Blackjack", file: "Blackjack.wav", names: &["blackjack"], records: &[(182, "d6162beb0e47b1a226d963fa728f5d001fa87df534dc81d57d0d07f5a20619a6"), (2048, "d0bff2c90c89c685654738176608bd2df1d095a418cf05270681f9e60ab0a169")], ranges: "004A.00FF 0800.0FFF", basic: true, run_cmd: "E2B3R\\nRUN" },
    Known { title: "Bowling", file: "Bowling.wav", names: &["bowling"], records: &[(182, "4b2827a980efe2ab9e3a91e3b93d55440929b36dc2d3b15e09cab16b468effba"), (3328, "7c3bd27351f735ff208e1c9736516c22aaca15676a2e967e11c8985e2a6752df")], ranges: "004A.00FF 0300.0FFF", basic: true, run_cmd: "E2B3R\\nRUN" },
    Known { title: "Buzzword", file: "Buzzword.wav", names: &["buzzword"], records: &[(182, "2c8fb7bc6b8d93d61739771ddc59c04573d5361cb20d77b84456639865da90eb"), (3328, "5664e7f03e2ed1220ed6c71440f73089bc959fd9194bf57b574a144921b59909")], ranges: "004A.00FF 0300.0FFF", basic: true, run_cmd: "E2B3R\\nRUN" },
    Known { title: "Craps", file: "Craps.wav", names: &["craps"], records: &[(182, "89477a1728f1563cfc513975679d8d7a672e41c87e649299991de1e04329cdfb"), (3328, "198cfb23c63c99063b9935cf9059fea3f8e892ebf781c5b4b3bf762f83ca29de")], ranges: "004A.00FF 0300.0FFF", basic: true, run_cmd: "E2B3R\\nRUN" },
    Known { title: "Deal or No Deal", file: "DealOrNoDeal.wav", names: &["dealornodeal"], records: &[(182, "cd1fbe13d247858cdde23422f8dd364269ac0b61b1d438f687ff61c6728a9fa5"), (15616, "e5c948d3de18ffa6bf92191cefc54748de683264afdee83c4ea11eb4fe204021")], ranges: "004A.00FF 0300.3FFF", basic: true, run_cmd: "E2B3R\\nRUN" },
    Known { title: "Football", file: "Football.wav", names: &["football"], records: &[(182, "1e2a204cdbd201d2316e3d723ac24a086513d1744b10d90f4ffe14d284d0390d"), (11264, "9dc499289a65066290cad0f2ab043677935b624d75db17138c85d90de18367e7")], ranges: "004A.00FF 0400.2FFF", basic: true, run_cmd: "E2B3R\\nRUN" },
    Known { title: "Hamurabi", file: "Hamurabi.wav", names: &["hamurabi"], records: &[(182, "9ac6b64916df3b028b4b2fc4f43aea0c508d19f0fd5e1a34a769345a6c40a6e2"), (3328, "1a23ce668fda30a0d9628b08aa11091ac384986523e4e04a5fc3bef695bdaca5")], ranges: "004A.00FF 0300.0FFF", basic: true, run_cmd: "E2B3R\\nRUN" },
    Known { title: "Hi-Lo", file: "HiLo.wav", names: &["hilo"], records: &[(182, "8e5c819de664028e6295be963efb9be9892c73e703638577de86b17d0caeab44"), (2048, "b2c25e94a4165cdc3e78e6d14f7b56f38f598ed9f01542bb33e42e429e165e95")], ranges: "004A.00FF 0800.0FFF", basic: true, run_cmd: "E2B3R\\nRUN" },
    Known { title: "Life", file: "Life.wav", names: &["life"], records: &[(512, "60dbb3fd33995165a780b8604aa8e878cae2be9ac4cab28cae430eba555cdbb0")], ranges: "2000.21FF", basic: false, run_cmd: "2000R" },
    Known { title: "Lunar Lander", file: "LunarLander.wav", names: &["lunarlander"], records: &[(1792, "3417413ac3e2e03565dfd4dcaf48fd2fb8714c864a2eaefb6371b9096dad254b")], ranges: "0300.09FF", basic: false, run_cmd: "300R" },
    Known { title: "Lunar Lander (ASCII graphics)", file: "LunarlanderasciiGraphics.wav", names: &["lunarlanderasciigraphics", "lunarlanderascii"], records: &[(182, "856658b8ab2cb2f34949c1bdbc10b755a6f598e10b98ca461e542ee8e1bf7540"), (3328, "3ccb0b7fcdbc5242f59156b31b16aa366ffb0a3b458463c833888184cfb0aa6e")], ranges: "004A.00FF 0300.0FFF", basic: true, run_cmd: "E2B3R\\nRUN" },
    Known { title: "Mastermind", file: "Mastermind.wav", names: &["mastermind"], records: &[(256, "7e98e39d2b12bd8f82ce7b6ef13904981a933e9e22e3e2a07b539ac543ad7f9a")], ranges: "0300.03FF", basic: false, run_cmd: "300R" },
    Known { title: "Microchess", file: "Microchess.wav", names: &["microchess"], records: &[(2304, "95d5a6a4c695bbe7f4f2a9462ec426217b38f42f081dd5c5768060d9dcaffaec")], ranges: "1000.18FF", basic: false, run_cmd: "1000R" },
    Known { title: "Pasart", file: "Pasart.wav", names: &["pasart"], records: &[(768, "c9c603ea166edc41a335e711bd436f904829f7d5bfe2d1a8732fe88411d860d7")], ranges: "0300.05FF", basic: false, run_cmd: "300R" },
    Known { title: "Slots", file: "Slots.wav", names: &["slots"], records: &[(182, "48a13e5d98e7e8805ca617a66e9cb8adb705078e730f01acbf322b9271b35483"), (3328, "d88e48222288676506188e6d3f56f5d38b74bc881cc0fcb6c28a4382f6576a23")], ranges: "004A.00FF 0300.0FFF", basic: true, run_cmd: "E2B3R\\nRUN" },
    Known { title: "Star Trek", file: "StarTrek.wav", names: &["startrek"], records: &[(182, "14b9f270422d9e2080e66234a610b201fdaf6b6d4623fea9f61d482f169cff17"), (3328, "787acc078f2a5ac39017c54dcfca4c84ebd103c912415fdf92a14e239ed6d275")], ranges: "004A.00FF 0300.0FFF", basic: true, run_cmd: "E2B3R\\nRUN" },
    Known { title: "Star Trek 2003", file: "StarTrek2003.wav", names: &["startrek2003"], records: &[(182, "3b8fd9c273410050edd78f138dc666c54b16441f5cc22a7054a161efc13f24ab"), (15616, "ff998404f2ab8aee7cd8e0500cae7d29370dc74b8208cd4b7d7c6dfaf75c41e9")], ranges: "004A.00FF 0300.3FFF", basic: true, run_cmd: "E2B3R\\nRUN" },
    Known { title: "WordCross", file: "WordCross.wav", names: &["wordcross"], records: &[(182, "0a89ad333fbc3ca0b93bd6683f59b8eed3229592dfa72ca403f6dc61db9c780a"), (3328, "4f4ac6cf82e88e2e1021965c1e7e395579b8f5403f03139b6daf483919318ccd")], ranges: "004A.00FF 0300.0FFF", basic: true, run_cmd: "E2B3R\\nRUN" },
    Known { title: "Wumpus", file: "Wumpus.wav", names: &["wumpus"], records: &[(182, "5285f3ad309edd9996438688f330cec0a0ed5d43d50ae1a4b147aa8410540d28"), (3328, "aab7cd4434deb4e6dfb77b9adb317aa98a133b5bacdcf54707c16b2108ae068b")], ranges: "004A.00FF 0300.0FFF", basic: true, run_cmd: "E2B3R\\nRUN" },
];

pub fn sha256_hex(data: &[u8]) -> String {
    Sha256::digest(data).iter().map(|b| format!("{b:02x}")).collect()
}

fn norm(name: &str) -> String {
    name.chars().filter(|c| c.is_ascii_alphanumeric()).collect::<String>().to_ascii_lowercase()
}

/// Recognise decoded records (`records` = data of each record).
pub fn identify_records(records: &[&[u8]], path: Option<&Path>) -> Option<Match> {
    if records.is_empty() {
        return None;
    }
    let sizes: Vec<usize> = records.iter().map(|r| r.len()).collect();
    let same_sizes = |k: &Known| k.records.len() == sizes.len() && k.records.iter().zip(&sizes).all(|(r, s)| r.0 == *s);
    let hashes: Vec<String> = records.iter().map(|r| sha256_hex(r)).collect();
    if let Some(k) = TABLE.iter().find(|k| same_sizes(k) && k.records.iter().zip(&hashes).all(|(r, h)| r.1 == h)) {
        return Some(Match { known: k, how: How::Content });
    }
    let stem = norm(&path?.file_stem()?.to_string_lossy());
    TABLE.iter().find(|k| same_sizes(k) && k.names.iter().any(|n| *n == stem)).map(|k| Match { known: k, how: How::Name })
}

pub fn identify(imp: &Imported, path: Option<&Path>) -> Option<Match> {
    let recs: Vec<&[u8]> = imp.segments.iter().map(|s| s.data.as_slice()).collect();
    identify_records(&recs, path)
}

/// Recognise a file (tapes only).
#[cfg(test)]
pub fn identify_file(path: &Path) -> Option<Match> {
    let bytes = std::fs::read(path).ok()?;
    if apple1_core::program::kind_of(path, Some(&bytes)) != apple1_core::program::Kind::Wav {
        return None;
    }
    identify(&apple1_core::aciwav::import_wav_bytes(&bytes).ok()?, Some(path))
}

impl Known {
    /// Parameters to store in programs.json (`auto` = title: they may be
    /// refreshed; the ⚙ dialog clears it when the user saves).
    pub fn params(&self) -> ProgParams {
        let run_addr = if self.basic { String::new() } else { self.run_cmd.trim_end_matches('R').to_string() };
        ProgParams {
            ranges: self.ranges.into(),
            load_addr: String::new(),
            run_addr,
            run_cmd: self.run_cmd.into(),
            kind: if self.basic { ProgKind::Basic } else { ProgKind::Machine },
            needs_basic: self.basic,
            notes: crate::i18n::tf("p_known_notes", &[&self.title]),
            auto: self.title.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_valid() {
        crate::i18n::set(crate::i18n::Lang::Fr);
        for k in TABLE {
            let plan = k.params().validate(false, 0).unwrap_or_else(|e| panic!("{} : {e}", k.title));
            let sizes: Vec<usize> = plan.ranges.iter().map(|&(a, b)| (b - a) as usize + 1).collect();
            let want: Vec<usize> = k.records.iter().map(|r| r.0).collect();
            assert_eq!(sizes, want, "{}", k.title);
            assert!(k.records.iter().all(|r| r.1.len() == 64), "{}", k.title);
            assert_eq!(k.basic, plan.kind == ProgKind::Basic);
            assert!(k.names.contains(&norm(k.file.trim_end_matches(".wav")).as_str()), "{}", k.title);
        }
        let mut f: Vec<_> = TABLE.iter().map(|k| k.file).collect();
        f.sort();
        f.dedup();
        assert_eq!(f.len(), TABLE.len());
    }

    #[test]
    fn match_by_content_then_name() {
        assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        // fake records: only the name and sizes can match
        let zp = vec![0u8; 182];
        let prog = vec![0u8; 3328];
        let recs: [&[u8]; 2] = [&zp, &prog];
        assert_eq!(identify_records(&recs, None), None);
        let m = identify_records(&recs, Some(Path::new("/x/HAMURABI.WAV"))).unwrap();
        assert_eq!((m.known.file, m.how), ("Hamurabi.wav", How::Name));
        assert_eq!(identify_records(&recs, Some(Path::new("Blackjack.wav"))), None, "sizes differ");
        let m = identify_records(&recs, Some(Path::new("Apple Enhanced Checkers.wav"))).unwrap();
        assert_eq!(m.known.file, "AppleiEnhancedCheckers.wav");
    }

    #[test]
    fn real_tapes_are_recognised() {
        let fx = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../test-fixtures");
        let mut seen = 0;
        for k in TABLE {
            let p = fx.join(k.file);
            if !p.is_file() {
                continue;
            }
            let m = identify_file(&p).unwrap_or_else(|| panic!("{} non reconnu", k.file));
            assert_eq!((m.known.file, m.how), (k.file, How::Content));
            // renamed copy: still recognised by content
            let imp = apple1_core::aciwav::import_wav(&p).unwrap();
            assert_eq!(identify(&imp, Some(Path::new("zzz.wav"))).map(|m| m.known.file), Some(k.file));
            seen += 1;
        }
        if seen == 0 {
            eprintln!("test ignoré : aucune cassette dans {}", fx.display());
        }
    }
}
