//! ▶ and double-click on a library program, without the window: find and
//! load Integer BASIC when the program needs it, then decode the program
//! with its ⚙ parameters (one range per tape record).

use std::path::{Path, PathBuf};

use apple1_core::aciwav;
use apple1_core::program::{self, Kind, Program};

use crate::config::home;
use crate::i18n::{t, tf};
use crate::params::Plan;
use crate::session::{self, Session};

/// Names tried for Integer BASIC (lower case, without extension).
const BASIC_NAMES: [&str; 5] = ["basic", "integer_basic", "integer-basic", "integerbasic", "intbasic"];

fn file_name(p: &Path) -> String {
    p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default()
}

/// Files that may hold Integer BASIC, best first: in the library
/// `BASIC.wav`, `integer_basic.bin`… (any case), then any file with
/// "basic" in its name, then `~/apple1/integer_basic.bin`, `basic.bin`.
pub fn basic_candidates(lib_dirs: &[PathBuf]) -> Vec<PathBuf> {
    let mut found: Vec<(usize, u8, u8, String, PathBuf)> = Vec::new();
    for (di, lib_dir) in lib_dirs.iter().enumerate() {
        let Ok(rd) = std::fs::read_dir(lib_dir) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            if !p.is_file() || !crate::library::is_program(&p) || crate::library::source_kind(&p, None).is_some() {
                continue;
            }
            let stem = p.file_stem().map(|s| s.to_string_lossy().to_lowercase()).unwrap_or_default();
            let ext = p.extension().map(|s| s.to_string_lossy().to_lowercase()).unwrap_or_default();
            let rank = if BASIC_NAMES.contains(&stem.as_str()) {
                0
            } else if stem.contains("basic") {
                1
            } else {
                continue;
            };
            // a binary or a dump loads faster than a tape
            let ext_rank = match ext.as_str() {
                "bin" => 0,
                "txt" | "apl" => 1,
                _ => 2,
            };
            // a good name anywhere first, then the folder order
            found.push((rank as usize * 1000 + di, ext_rank, 0, stem, p));
        }
    }
    found.sort();
    let mut v: Vec<PathBuf> = found.into_iter().map(|f| f.4).collect();
    v.extend(["integer_basic.bin", "basic.bin", "BASIC.bin"].iter().map(|n| home().join("apple1").join(n)).filter(|p| p.is_file()));
    v
}

/// Integer BASIC to use: the chosen file if any (errors are reported),
/// else the first valid candidate. Returns (file, program, found automatically).
pub fn find_basic(configured: Option<&Path>, lib_dir: &[PathBuf]) -> Result<(PathBuf, Program, bool), String> {
    if let Some(c) = configured.filter(|p| p.is_file()) {
        return session::read_basic(c).map(|p| (c.to_path_buf(), p, false)).map_err(|e| tf("e_basic_failed", &[&c.display(), &e]));
    }
    for c in basic_candidates(lib_dir) {
        if let Ok(p) = session::read_basic(&c) {
            return Ok((c, p, true));
        }
    }
    Err(t("e_basic_missing").into())
}

/// Integer BASIC that was just loaded.
pub struct BasicLoaded {
    pub path: PathBuf,
    /// Found in the library (to remember it).
    pub auto: bool,
    pub msg: String,
}

/// Make sure Integer BASIC is at $E000, loading it (instantly) if needed.
pub fn ensure_basic(s: &mut Session, configured: Option<&Path>, lib_dir: &[PathBuf]) -> Result<Option<BasicLoaded>, String> {
    if s.basic_present() {
        return Ok(None);
    }
    load_original(s, configured, lib_dir).map(Some)
}

/// Load the original Integer BASIC (the user's copy) at $E000, whatever is there now.
pub fn load_original(s: &mut Session, configured: Option<&Path>, lib_dir: &[PathBuf]) -> Result<BasicLoaded, String> {
    if !s.m.board.is_ram(0xE000) {
        return Err(t("s_no_ram_e000").into());
    }
    let (path, p, auto) = find_basic(configured, lib_dir)?;
    s.load_built(&file_name(&path), &p);
    let msg = if auto { tf("s_basic_auto", &[&file_name(&path)]) } else { tf("s_basic_loaded", &[&p.total_len()]) };
    Ok(BasicLoaded { path, auto, msg })
}

/// Decode a library program with its checked parameters (instant). Tape
/// records go to the ⚙ ranges, one range per record; a tape without
/// addresses and without ranges is placed by `program::guess_layout`.
pub fn build_program(path: &Path, plan: &Plan) -> Result<Program, String> {
    let first = plan.load_addr.or(plan.ranges.first().map(|r| r.0));
    let mut p = match program::kind_of(path, None) {
        Kind::Bin => program::load_file(path, first)?,
        Kind::Hex | Kind::Prg => program::load_file(path, None)?,
        Kind::Wav => {
            let imp = aciwav::import_wav(path).map_err(|e| format!("{} : {e}", path.display()))?;
            let n = imp.segments.len();
            let r = &plan.ranges;
            if n > 0 && r.len() == n {
                program::wav_with_ranges(&imp, r)?
            } else if !imp.embedded.is_empty() || n == 0 {
                program::wav_to_program(&imp, None)?
            } else if r.len() == 1 {
                // one start address: records placed end to end
                program::wav_to_program(&imp, first)?
            } else if r.is_empty() {
                match program::guess_layout(&imp) {
                    Some((g, _)) => program::wav_with_ranges(&imp, &g)?,
                    None => program::wav_to_program(&imp, None).map_err(|e| format!("{} : {e}", path.display()))?,
                }
            } else {
                return Err(tf("e_ranges_records", &[&r.len(), &n]));
            }
        }
    };
    if plan.run_addr.is_some() {
        p.run = plan.run_addr;
    }
    Ok(p)
}

/// RAM (KB, multiple of 4) needed by a program above `current_kb`, or
/// None when it fits (the I/O and ROM above $C000 do not count).
pub fn ram_needed(plan: &Plan, current_kb: u32) -> Option<u32> {
    let top = plan.ranges.iter().map(|r| r.1 as u32 + 1).filter(|&e| e <= 0xC000).max()?;
    let kb = top.div_ceil(4096) * 4;
    (kb > current_kb).then_some(kb)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::{self, ProgKind, ProgParams};
    use apple1_core::aciwav::ExportOptions;
    use apple1_core::Config;

    /// Stand-in for Integer BASIC (the real one is not included): `JMP`
    /// at $E000; at $E2B3 (warm start) a stub prints "HIMEM=" and the
    /// HIMEM pointer ($4C-$4D, loaded from the program's zero page), then
    /// echoes the keyboard.
    fn fake_basic() -> Vec<u8> {
        let mut rom = vec![0u8; 4096];
        rom[0..3].copy_from_slice(&[0x4C, 0xB3, 0xE2]);
        #[rustfmt::skip]
        let stub = [
            0xA2, 0x00,             // E2B3 LDX #0
            0xBD, 0xD8, 0xE2,       // E2B5 LDA MSG,X
            0xF0, 0x06,             // E2B8 BEQ E2C0
            0x20, 0xEF, 0xFF,       // E2BA JSR ECHO
            0xE8,                   // E2BD INX
            0xD0, 0xF5,             // E2BE BNE E2B5
            0xA5, 0x4D,             // E2C0 LDA $4D
            0x20, 0xDC, 0xFF,       // E2C2 JSR PRBYTE
            0xA5, 0x4C,             // E2C5 LDA $4C
            0x20, 0xDC, 0xFF,       // E2C7 JSR PRBYTE
            0xAD, 0x11, 0xD0,       // E2CA LDA KBDCR
            0x10, 0xFB,             // E2CD BPL E2CA
            0xAD, 0x10, 0xD0,       // E2CF LDA KBD
            0x20, 0xEF, 0xFF,       // E2D2 JSR ECHO
            0x4C, 0xCA, 0xE2,       // E2D5 JMP E2CA
        ];
        rom[0x2B3..0x2B3 + stub.len()].copy_from_slice(&stub);
        let msg = b"HIMEM=\0";
        rom[0x2D8..0x2D8 + msg.len()].copy_from_slice(msg);
        rom
    }

    /// A tape made by another tool: no address comment.
    fn tape(path: &Path, records: &[&[u8]]) {
        let segs: Vec<(u16, &[u8])> = records.iter().map(|r| (0u16, *r)).collect();
        let o = ExportOptions { embed_address: false, ..Default::default() };
        aciwav::export_wav(path, &segs, &o).unwrap();
    }

    /// ⚙ « Programme Integer BASIC »: the original replaces G2 Basic when it is the one in memory.
    #[test]
    fn original_replaces_g2() {
        crate::i18n::set(crate::i18n::Lang::Fr);
        let dir = std::env::temp_dir().join(format!("a1g2o-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        tape(&dir.join("BASIC.wav"), &[&fake_basic()]);
        let mut s = boot();
        s.load_g2basic().unwrap();
        assert!(s.g2_present() && s.basic_present());
        let b = load_original(&mut s, None, std::slice::from_ref(&dir)).unwrap();
        assert!(b.auto);
        assert!(!s.g2_present() && s.basic_present());
        std::fs::remove_dir_all(&dir).ok();
    }

    fn boot() -> Session {
        let mut s = Session::new(Config { display_throttle: false, ..Config::default() });
        s.m.run_cycles(200_000);
        s
    }

    #[test]
    fn candidates_order() {
        let dir = std::env::temp_dir().join(format!("a1cand-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        for n in ["BasicGame.wav", "BASIC.wav", "Integer_Basic.bin", "hello.txt", "notes-basic.pdf"] {
            std::fs::write(dir.join(n), b"x").unwrap();
        }
        let c: Vec<String> = basic_candidates(std::slice::from_ref(&dir)).iter().filter(|p| p.starts_with(&dir)).map(|p| file_name(p)).collect();
        assert_eq!(c, ["Integer_Basic.bin", "BASIC.wav", "BasicGame.wav"]);
        std::fs::remove_dir_all(&dir).ok();
    }

    /// The bug report: Hamurabi.wav from Apple1Cassettes (2 records, no
    /// addresses), BASIC.wav next to it, nothing chosen in the Préférences,
    /// ▶ with the ranges typed as `4A.00FFR300.FFFR`.
    #[test]
    fn play_basic_program_from_tapes() {
        crate::i18n::set(crate::i18n::Lang::Fr);
        let dir = std::env::temp_dir().join(format!("a1play-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let ham = dir.join("Hamurabi.wav");
        // HAMURABI_WAV=… : the real recording (not included in the project)
        match std::env::var_os("HAMURABI_WAV").map(PathBuf::from).filter(|p| p.is_file()) {
            Some(real) => {
                std::fs::copy(real, &ham).unwrap();
            }
            None => {
                let mut zp = vec![0u8; 182];
                zp[0..4].copy_from_slice(&[0x00, 0x03, 0x00, 0x10]); // LOMEM $0300, HIMEM $1000
                zp[0x80..0x84].copy_from_slice(&[0xF3, 0x04, 0x97, 0x03]); // PP, PV
                let prog: Vec<u8> = (0..3328u32).map(|i| (i * 7 + 3) as u8).collect();
                tape(&ham, &[&zp, &prog]);
            }
        }
        // no BASIC yet: clear message
        let mut s = boot();
        let rr = ensure_basic(&mut s, None, std::slice::from_ref(&dir));
        let e = rr.err().unwrap();
        assert!(e.contains("BASIC.wav"), "{e}");
        tape(&dir.join("BASIC.wav"), &[&fake_basic()]);
        // ⚙ prefilled from the tapes
        let pb = params::from_file(&dir.join("BASIC.wav"));
        // (a fake BASIC named BASIC.wav, 4096 bytes: recognised by its name)
        assert_eq!((pb.ranges.as_str(), pb.kind, pb.run_cmd.as_str(), pb.needs_basic, pb.auto.as_str()), ("E000.EFFF", ProgKind::Machine, "E000R", false, "Integer BASIC"));
        assert_eq!(crate::known::identify_file(&dir.join("BASIC.wav")).map(|m| m.how), Some(crate::known::How::Name));
        let ph = params::from_file(&ham);
        assert_eq!((ph.ranges.as_str(), ph.kind, ph.run_cmd.as_str()), ("004A.00FF 0300.0FFF", ProgKind::Basic, "E2B3R\\nRUN"));
        assert_eq!(ph.auto, "Hamurabi", "recognised by its name and record sizes");
        // another name: layout guessed from the BASIC pointers
        std::fs::copy(&ham, dir.join("prog.wav")).unwrap();
        let pg = params::from_file(&dir.join("prog.wav"));
        assert_eq!((pg.ranges.as_str(), pg.auto.as_str()), ("004A.00FF 0300.0FFF", ""));
        assert!(pg.notes.contains("LOMEM"), "{}", pg.notes);
        // what Wirehead typed in ⚙
        let user = ProgParams { ranges: "4A.00FFR300.FFFR".into(), kind: ProgKind::Basic, needs_basic: true, run_cmd: "E2B3R\\nRUN".into(), ..Default::default() };
        let plan = user.validate(false, 0).unwrap();
        assert_eq!(plan.ranges, vec![(0x4A, 0xFF), (0x300, 0xFFF)]);
        // ▶: BASIC first (found in the library), then the program, then E2B3R + RUN
        let b = ensure_basic(&mut s, None, std::slice::from_ref(&dir)).unwrap().unwrap();
        assert!(b.auto && b.path == dir.join("BASIC.wav") && b.msg.contains("BASIC.wav"), "{}", b.msg);
        assert!(s.basic_present());
        assert!(ensure_basic(&mut s, Some(&b.path), std::slice::from_ref(&dir)).unwrap().is_none());
        let p = build_program(&ham, &plan).unwrap();
        assert_eq!(p.ranges(), vec![(0x4A, 0xFF), (0x300, 0xFFF)]);
        s.load_built("Hamurabi.wav", &p);
        assert_eq!(s.m.board.mem[0xE000], 0x4C, "the program must not overwrite BASIC");
        assert_eq!(&s.m.board.mem[0x300..0x1000], &p.blocks[1].1[..]);
        assert_eq!(&s.m.board.mem[0x4A..0x4E], &[0x00, 0x03, 0x00, 0x10]);
        s.m.reset();
        s.m.type_text(&format!("{}\r", plan.run_keys));
        s.m.run_cycles(3_000_000);
        let screen = s.m.screen_text();
        assert!(screen.contains("E2B3R") && screen.contains("HIMEM=1000") && screen.contains("RUN"), "{screen}");
        // without ranges: the guessed layout gives the same
        let auto = ph.validate(false, 0).unwrap();
        assert_eq!(build_program(&ham, &auto).unwrap().ranges(), p.ranges());
        // too few ranges for the records: clear error
        let one = ProgParams { ranges: "4A.00FF".into(), kind: ProgKind::Machine, ..Default::default() }.validate(false, 0).unwrap();
        assert!(build_program(&ham, &one).is_ok()); // one start: end to end
        let three = ProgParams { ranges: "4A.00FF 300.3FF 800.FFF".into(), ..Default::default() }.validate(false, 0).unwrap();
        assert!(build_program(&ham, &three).unwrap_err().contains("3 plage"));
        // a bad BASIC file is refused before touching memory
        std::fs::write(dir.join("bad.bin"), vec![0u8; 4096]).unwrap();
        assert!(session::read_basic(&dir.join("bad.bin")).unwrap_err().contains("JMP"));
        std::fs::write(dir.join("short.bin"), vec![0x4Cu8; 100]).unwrap();
        assert!(session::read_basic(&dir.join("short.bin")).unwrap_err().contains("4096"));
        std::fs::remove_dir_all(&dir).ok();
    }

    /// Real recordings (archive.org "Apple1Cassettes"), kept only in the
    /// git-ignored `test-fixtures/` folder (copyright): BASIC.wav and
    /// Hamurabi.wav. Without them the test is skipped.
    #[test]
    fn hamurabi_with_real_basic() {
        crate::i18n::set(crate::i18n::Lang::Fr);
        let fx = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../test-fixtures");
        let (basic, ham) = (fx.join("BASIC.wav"), fx.join("Hamurabi.wav"));
        if !basic.is_file() || !ham.is_file() {
            eprintln!("test ignoré : {} et Hamurabi.wav absents", basic.display());
            return;
        }
        let mut s = boot();
        let b = ensure_basic(&mut s, None, std::slice::from_ref(&fx)).unwrap().unwrap();
        assert_eq!(b.path, basic);
        let plan = ProgParams { ranges: "4A.00FFR300.FFFR".into(), kind: ProgKind::Basic, needs_basic: true, run_cmd: "E2B3R\\nRUN".into(), ..Default::default() }.validate(false, 0).unwrap();
        let p = build_program(&ham, &plan).unwrap();
        s.load_built("Hamurabi.wav", &p);
        s.m.reset();
        s.m.type_text(&format!("{}\r", plan.run_keys));
        s.m.run_cycles(20_000_000);
        let screen = s.m.screen_text();
        eprintln!("{screen}");
        assert!(screen.contains("TRY YOUR HAND AT GOVERNING ANCIENT"), "{screen}");
        assert!(screen.contains("HAMURABI: I BEG TO REPORT TO YOU,"), "{screen}");
        // v5 bug (21a2285): with one start address the records were placed
        // end to end from $4A, so the program went to $0100 instead of
        // $0300 and RUN printed only " BUSHELS,"
        let mut s2 = boot();
        ensure_basic(&mut s2, Some(&basic), std::slice::from_ref(&fx)).unwrap();
        let old = program::wav_to_program(&aciwav::import_wav(&ham).unwrap(), Some(0x4A)).unwrap();
        assert_eq!(old.ranges(), vec![(0x4A, 0xFF), (0x100, 0xDFF)]);
        s2.load_built("Hamurabi.wav", &old);
        s2.m.reset();
        s2.m.type_text("E2B3R\rRUN\r");
        s2.m.run_cycles(20_000_000);
        let bad = s2.m.screen_text();
        assert!(!bad.contains("TRY YOUR HAND") && bad.contains(" BUSHELS,"), "{bad}");
    }

    #[test]
    fn ram_for_big_programs() {
        let plan = |r: &str| ProgParams { ranges: r.into(), ..Default::default() }.validate(false, 0).unwrap();
        assert_eq!(ram_needed(&plan("300.FFF"), 8), None);
        assert_eq!(ram_needed(&plan("4A.00FF 400.2FFF"), 8), Some(12));
        assert_eq!(ram_needed(&plan("4A.00FF 300.3FFF"), 8), Some(16));
        assert_eq!(ram_needed(&plan("2000.21FF"), 8), Some(12));
        assert_eq!(ram_needed(&plan("E000.EFFF"), 8), None);
    }

    /// Every tape of the table found in test-fixtures/ (copies of
    /// archive.org Apple1Cassettes, never committed) starts with its preset
    /// parameters: BASIC programs print their first lines, machine code
    /// shows its screen.
    #[test]
    fn known_tapes_run() {
        crate::i18n::set(crate::i18n::Lang::Fr);
        let fx = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../test-fixtures");
        let basic = fx.join("BASIC.wav");
        let marks: &[(&str, &str)] = &[
            ("AceyDucey.wav", "ACEY DUCEY CARD GAME"),
            ("AppleiEnhancedCheckers.wav", "APPLE-I CHECKERS"),
            ("Blackjack.wav", "WELCOME TO 21"),
            ("Bowling.wav", "WELCOME TO THE ALLEY"),
            ("Buzzword.wav", "BUZZWORD"),
            ("Craps.wav", "CRAP GAME"),
            ("DealOrNoDeal.wav", "DEAL OR NO DEAL"),
            ("Football.wav", "INSTRUCTIONS"),
            ("Hamurabi.wav", "TRY YOUR HAND AT GOVERNING ANCIENT"),
            ("HiLo.wav", "HI-LO"),
            ("LunarlanderasciiGraphics.wav", "LUNAR LANDING SIMULATION"),
            ("Slots.wav", "SLOTS"),
            ("StarTrek.wav", "TYPE A NUMBER"),
            ("StarTrek2003.wav", "S T A R T R E K  2 0 0 3"),
            ("WordCross.wav", "WORDCROSS"),
            ("Wumpus.wav", "WUMPUS"),
            ("APPLE30TH.wav", "HAPPY 30TH BIRTHDAY APPLE!"),
            ("ASMmchess.wav", "MICROCHESS(C)"),
            ("BASIC.wav", ">"),
            ("Life.wav", "CONWAY'S GAME OF LIFE"),
            ("LunarLander.wav", "MAXIMUM BURN IS 30 UNITS/SEC"),
            ("Mastermind.wav", "READY?"),
            ("Microchess.wav", "!     MICROCHESS     !"),
            ("Pasart.wav", "ROWS?"),
        ];
        let only = std::env::var("KNOWN_ONLY").ok();
        let mut ran = 0;
        for k in crate::known::TABLE {
            let path = fx.join(k.file);
            if !path.is_file() || (k.basic && !basic.is_file()) || only.as_deref().is_some_and(|o| o != k.file) {
                continue;
            }
            let plan = k.params().validate(false, 0).unwrap();
            let mut s = boot();
            if let Some(kb) = ram_needed(&plan, s.m.board.config.ram_kb) {
                let mut c = s.m.board.config.clone();
                c.ram_kb = kb;
                s.m.board.apply_config(c);
            }
            if plan.needs_basic {
                ensure_basic(&mut s, Some(&basic), std::slice::from_ref(&fx)).unwrap();
            }
            let p = build_program(&path, &plan).unwrap();
            s.load_built(k.file, &p);
            s.m.reset();
            s.m.type_text(&format!("{}\r", plan.run_keys));
            let mark = marks.iter().find(|m| m.0 == k.file).map(|m| m.1).unwrap_or_else(|| panic!("{} : pas de repère", k.file));
            // the screen scrolls: look at it every million cycles (1 s)
            let (mut seen, mut screen) = (false, String::new());
            for _ in 0..30 {
                s.m.run_cycles(1_000_000);
                screen = s.m.screen_text();
                seen |= screen.contains(mark);
            }
            eprintln!("===== {} ({})\n{screen}", k.title, plan.run_keys.replace('\r', " ⏎ "));
            assert!(seen, "{} : « {mark} » absent\n{screen}", k.title);
            assert!(!screen.contains("*** SYNTAX ERR") && !screen.contains("ERR\n>"), "{} : erreur BASIC\n{screen}", k.title);
            ran += 1;
        }
        if ran == 0 {
            eprintln!("test ignoré : cassettes absentes de {}", fx.display());
        }
    }

    /// Recognised tapes get their ⚙ parameters; those typed by the user
    /// (no `auto`, e.g. from an older version) are never replaced.
    #[test]
    fn library_presets_known_tapes() {
        crate::i18n::set(crate::i18n::Lang::Fr);
        let dir = std::env::temp_dir().join(format!("a1known-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let zp = vec![0u8; 182];
        let prog = vec![0u8; 3328];
        tape(&dir.join("Hamurabi.wav"), &[&zp, &prog]);
        tape(&dir.join("WUMPUS.wav"), &[&zp, &prog]);
        let mine = ProgParams { ranges: "4A.00FF 300.FFF".into(), kind: ProgKind::Basic, notes: "à moi".into(), ..Default::default() };
        let mut st = params::Store::default();
        st.map.insert("WUMPUS.wav".into(), mine.clone());
        // an old automatic entry is refreshed
        st.map.insert("Hamurabi.wav".into(), ProgParams { ranges: "4A.00FF 800.FFF".into(), auto: "Hamurabi".into(), ..Default::default() });
        st.save(&dir).unwrap();
        let mut l = crate::library::Library::new(dir.clone());
        let t0 = std::time::Instant::now();
        while l.entries.iter().any(|e| e.info.is_none()) && t0.elapsed().as_secs() < 60 {
            std::thread::sleep(std::time::Duration::from_millis(20));
            l.tick();
        }
        for e in &l.entries {
            assert_eq!(e.info.as_ref().unwrap().known.map(|m| m.how), Some(crate::known::How::Name), "{}", e.name);
        }
        let (st, _) = params::Store::load(&dir);
        let h = &st.map["Hamurabi.wav"];
        assert_eq!((h.ranges.as_str(), h.run_cmd.as_str(), h.auto.as_str()), ("004A.00FF 0300.0FFF", "E2B3R\\nRUN", "Hamurabi"));
        assert!(l.is_auto(&dir.join("Hamurabi.wav")) && !l.is_auto(&dir.join("WUMPUS.wav")));
        assert_eq!(st.map["WUMPUS.wav"], mine);
        std::fs::remove_dir_all(&dir).ok();
    }
}
