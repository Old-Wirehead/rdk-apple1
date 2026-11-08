//! Per-program parameters of the library (⚙ button), saved in
//! `programs.json` in the library folder (replaces `descriptions.json`,
//! which is migrated on first use).

use std::collections::BTreeMap;
use std::path::Path;

use apple1_core::aciwav;
use apple1_core::program::{self, Guess, Kind, Program};
use apple1_core::tape;
use serde::{Deserialize, Serialize};

use crate::i18n::{t, tf};

pub const FILE: &str = "programs.json";
pub const OLD_FILE: &str = "descriptions.json";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgKind {
    /// Machine code, started from the Woz monitor (`300R`).
    #[default]
    Machine,
    /// BASIC program (`4A.00FF 800.FFF`) for RDK G2 Basic (the default BASIC of the kit),
    /// started by `E2B3R` + `RUN`.
    Basic,
    /// The same, for the original Integer BASIC (purists; the user's own copy of it).
    BasicInteger,
}

impl ProgKind {
    /// A BASIC program, G2 Basic or the original.
    pub fn is_basic(self) -> bool {
        self != ProgKind::Machine
    }
}

/// What the user can edit. Text fields keep what was typed; empty = "from
/// the file".
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ProgParams {
    /// Memory ranges, ACI style: `300.31E`, `4A.00FF 800.FFF`.
    pub ranges: String,
    /// Load address of a raw .bin (hex).
    pub load_addr: String,
    /// Run address (hex), for machine code.
    pub run_addr: String,
    /// Command typed to start it (`\n` = RETURN); empty = default.
    pub run_cmd: String,
    pub kind: ProgKind,
    /// Integer BASIC must be in memory at $E000.
    pub needs_basic: bool,
    pub notes: String,
    /// Title of the recognised tape when these parameters were filled in
    /// automatically (see `known`); empty = set by the user, never
    /// overwritten.
    pub auto: String,
}

/// Validated, ready-to-use parameters.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Plan {
    pub ranges: Vec<(u16, u16)>,
    pub load_addr: Option<u16>,
    pub run_addr: Option<u16>,
    pub kind: ProgKind,
    pub needs_basic: bool,
    /// Text typed to start the program (with `\r` for RETURN).
    pub run_keys: String,
}

/// Integer BASIC: a program saved with its zero page pointers `4A.00FF`.
pub fn looks_basic(ranges: &[(u16, u16)]) -> bool {
    ranges.iter().any(|&(a, b)| a == 0x4A && b == 0xFF)
}

pub fn ranges_aci(r: &[(u16, u16)]) -> String {
    r.iter().map(|&(a, b)| tape::aci_range_text(a, b)).collect::<Vec<_>>().join(" ")
}

/// Default parameters read from the file itself.
pub fn from_file(path: &Path) -> ProgParams {
    let mut p = ProgParams::default();
    let bytes = std::fs::read(path).unwrap_or_default();
    let kind = program::kind_of(path, Some(&bytes));
    if kind == Kind::Bin {
        p.load_addr = "300".into();
        p.ranges = format!("300.{:X}", (0x300 + bytes.len().max(1) - 1).min(0xFFFF));
        p.run_addr = "300".into();
        p.run_cmd = "300R".into();
        return p;
    }
    let imp = (kind == Kind::Wav).then(|| aciwav::import_wav_bytes(&bytes).ok()).flatten();
    if let Some(m) = imp.as_ref().and_then(|i| crate::known::identify(i, Some(path))) {
        return m.known.params();
    }
    if let Ok(prog) = program::load_bytes(&bytes, kind, None) {
        fill_from_program(&mut p, &prog);
    } else if let Some((r, g)) = imp.as_ref().and_then(program::guess_layout) {
        fill_from_guess(&mut p, &r, g);
    }
    p
}

/// Tape without addresses (Apple1Cassettes...): see `program::guess_layout`.
pub fn fill_from_guess(p: &mut ProgParams, r: &[(u16, u16)], g: Guess) {
    p.ranges = ranges_aci(r);
    p.run_addr.clear();
    match g {
        Guess::IntegerBasic => {
            p.kind = ProgKind::Machine;
            p.needs_basic = false;
            p.run_cmd.clear();
            p.notes = t("p_guess_basic_rom").into();
        }
        Guess::BasicPointers | Guess::BasicDefault => {
            p.kind = ProgKind::Basic;
            p.needs_basic = true;
            p.run_cmd = "E2B3R\\nRUN".into();
            p.notes = t(if g == Guess::BasicPointers { "p_guess_pointers" } else { "p_guess_default" }).into();
        }
    }
}

pub fn fill_from_program(p: &mut ProgParams, prog: &Program) {
    let r = prog.ranges();
    p.ranges = ranges_aci(&r);
    if looks_basic(&r) {
        p.kind = ProgKind::Basic;
        p.needs_basic = true;
        p.run_addr.clear();
        p.run_cmd = "E2B3R\\nRUN".into();
    } else {
        let run = prog.run.or_else(|| r.iter().map(|x| x.0).find(|&a| a >= 0x200)).or(r.first().map(|x| x.0));
        p.run_addr = run.map(|a| format!("{a:X}")).unwrap_or_default();
        p.run_cmd = run.map(|a| format!("{a:X}R")).unwrap_or_default();
    }
}

/// Parse ranges the way the ACI ROM reads them: `4A.FF` would be read as
/// $004A-$4AFF by the real ACI, so it is refused (write `4A.00FF`).
pub fn parse_aci_ranges(s: &str) -> Result<Vec<(u16, u16)>, String> {
    let list = program::parse_ranges(s)?;
    for tok in s.split([' ', ',']).filter(|x| !x.trim().is_empty()) {
        for part in tok.split_inclusive(['R', 'r', 'W', 'w']) {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            let (a, b) = program::parse_range(part)?;
            let typed = format!("{}R", part.trim_end_matches(['R', 'r', 'W', 'w']));
            if let Some((aa, ab)) = tape::aci_parse_range(&typed.to_ascii_uppercase()) {
                if (aa, ab) != (a, b) {
                    return Err(tf("p_err_aci", &[&part, &format!("{aa:04X}.{ab:04X}"), &tape::aci_range_text(a, b)]));
                }
            }
        }
    }
    Ok(list)
}

fn hex(field: &str, s: &str) -> Result<Option<u16>, String> {
    let s = s.trim().trim_start_matches('$');
    if s.is_empty() {
        return Ok(None);
    }
    program::parse_addr(s).map(Some).map_err(|_| tf("p_err_addr", &[&t(field), &s]))
}

impl ProgParams {
    /// Check the fields. `file_len` = size of a .bin, to check it fits.
    pub fn validate(&self, is_bin: bool, file_len: usize) -> Result<Plan, String> {
        let ranges = if self.ranges.trim().is_empty() { Vec::new() } else { parse_aci_ranges(&self.ranges)? };
        let load_addr = hex("p_load", &self.load_addr)?;
        let run_addr = hex("p_run", &self.run_addr)?;
        if is_bin {
            let a = load_addr.or(ranges.first().map(|r| r.0)).ok_or_else(|| t("p_err_bin_addr").to_string())?;
            if a as usize + file_len > 0x10000 {
                return Err(tf("p_err_bin_fit", &[&file_len, &format!("{a:04X}")]));
            }
        }
        let kind = self.kind;
        let needs_basic = self.needs_basic || kind.is_basic();
        if kind.is_basic() && !ranges.is_empty() && !looks_basic(&ranges) {
            return Err(t("p_err_basic_ranges").to_string());
        }
        if let (Some(r), false) = (run_addr, ranges.is_empty()) {
            if kind == ProgKind::Machine && !ranges.iter().any(|&(a, b)| r >= a && r <= b) && r < 0xE000 {
                return Err(tf("p_err_run_outside", &[&format!("{r:04X}"), &ranges_aci(&ranges)]));
            }
        }
        let cmd = self.run_cmd.trim();
        let run_keys = if !cmd.is_empty() {
            cmd.replace("\\n", "\r").replace('\n', "\r")
        } else {
            match (kind, run_addr) {
                (ProgKind::Basic | ProgKind::BasicInteger, _) => "E2B3R\rRUN".into(),
                (_, Some(a)) => format!("{a:X}R"),
                _ => String::new(),
            }
        };
        if run_keys.chars().any(|c| !(c == '\r' || (' '..='~').contains(&c))) {
            return Err(t("p_err_cmd_chars").to_string());
        }
        Ok(Plan { ranges, load_addr, run_addr, kind, needs_basic, run_keys })
    }
}

/// All parameters of a library folder.
#[derive(Clone, Debug, Default)]
pub struct Store {
    pub map: BTreeMap<String, ProgParams>,
}

impl Store {
    /// Read `programs.json`, or migrate `descriptions.json` (left in place).
    /// Returns (store, migrated count).
    pub fn load(dir: &Path) -> (Store, usize) {
        if let Ok(s) = std::fs::read_to_string(dir.join(FILE)) {
            return (Store { map: serde_json::from_str(&s).unwrap_or_default() }, 0);
        }
        let old: BTreeMap<String, String> = std::fs::read_to_string(dir.join(OLD_FILE)).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default();
        if old.is_empty() {
            return (Store::default(), 0);
        }
        let mut st = Store::default();
        for (name, notes) in old {
            let mut p = from_file(&dir.join(&name));
            p.notes = notes;
            st.map.insert(name, p);
        }
        let n = st.map.len();
        let _ = st.save(dir);
        (st, n)
    }

    pub fn save(&self, dir: &Path) -> Result<(), String> {
        let p = dir.join(FILE);
        let s = serde_json::to_string_pretty(&self.map).map_err(|e| e.to_string())?;
        std::fs::write(&p, s).map_err(|e| format!("{} : {e}", p.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation() {
        crate::i18n::set(crate::i18n::Lang::Fr);
        let ok = ProgParams { ranges: "300.31E".into(), run_addr: "300".into(), ..Default::default() };
        let p = ok.validate(false, 0).unwrap();
        assert_eq!(p.ranges, vec![(0x300, 0x31E)]);
        assert_eq!(p.run_keys, "300R");
        // ACI reads 4A.FF as 004A.4AFF
        let e = ProgParams { ranges: "4A.FF 800.FFF".into(), ..Default::default() }.validate(false, 0).unwrap_err();
        assert!(e.contains("4A.00FF"), "{e}");
        let b = ProgParams { ranges: "4A.00FF 800.FFF".into(), kind: ProgKind::Basic, ..Default::default() }.validate(false, 0).unwrap();
        assert!(b.needs_basic && b.run_keys == "E2B3R\rRUN");
        assert!(ProgParams { ranges: "300.3FF".into(), kind: ProgKind::Basic, ..Default::default() }.validate(false, 0).is_err());
        assert!(ProgParams { run_addr: "XYZ".into(), ..Default::default() }.validate(false, 0).is_err());
        assert!(ProgParams { ranges: "300.3FF".into(), run_addr: "800".into(), ..Default::default() }.validate(false, 0).is_err());
        // .bin: address needed, must fit
        assert!(ProgParams::default().validate(true, 16).is_err());
        assert!(ProgParams { load_addr: "FFF0".into(), ..Default::default() }.validate(true, 32).is_err());
        assert_eq!(ProgParams { load_addr: "E000".into(), ..Default::default() }.validate(true, 4096).unwrap().load_addr, Some(0xE000));
        let c = ProgParams { run_cmd: "C100R\\n300.31ER".into(), ..Default::default() }.validate(false, 0).unwrap();
        assert_eq!(c.run_keys, "C100R\r300.31ER");
        assert!(ProgParams { run_cmd: "été".into(), ..Default::default() }.validate(false, 0).is_err());
    }

    #[test]
    fn migrate_descriptions() {
        let dir = std::env::temp_dir().join(format!("a1params-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("hello.txt"), crate::library::HELLO_TXT).unwrap();
        std::fs::write(dir.join(OLD_FILE), r#"{"hello.txt": "Bonjour"}"#).unwrap();
        let (st, n) = Store::load(&dir);
        assert_eq!(n, 1);
        let p = &st.map["hello.txt"];
        assert_eq!((p.notes.as_str(), p.ranges.as_str(), p.run_addr.as_str(), p.run_cmd.as_str()), ("Bonjour", "0300.031E", "300", "300R"));
        assert!(dir.join(FILE).exists() && dir.join(OLD_FILE).exists());
        let (st2, n2) = Store::load(&dir);
        assert_eq!((n2, st2.map.len()), (0, 1));
        std::fs::remove_dir_all(&dir).ok();
    }
}
