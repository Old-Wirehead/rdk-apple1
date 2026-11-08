//! Emulated machine plus the actions of the toolbar (load, save, tape...).
//! Same behaviour as the terminal interface (`apple1-emu`).

use std::path::{Path, PathBuf};
use std::time::Instant;

use apple1_core::aciwav::{self, ExportOptions};
use apple1_core::machine::Machine;
use apple1_core::program::{self, Kind, Program};
use apple1_core::tape::{self, RenderOptions, TapePlayer};
use apple1_core::{wav, Config};

use crate::i18n::{t, tf};

/// Recording is saved after this much emulated silence (s).
const RECORD_IDLE_SECS: f64 = 1.5;

pub struct Session {
    pub m: Machine,
    pub fast: bool,
    pub record_path: Option<PathBuf>,
    /// Library file the inserted tape comes from (progress bar).
    pub tape_source: Option<PathBuf>,
    sync_real: Instant,
    sync_cycles: u64,
    was_fast: bool,
    /// Debugger (used when `dbg_on`: open panel or breakpoints set).
    pub dbg: apple1_core::debug::Debugger,
    pub dbg_on: bool,
    /// Last stop not yet shown by the interface.
    pub dbg_stopped: Option<apple1_core::debug::Stop>,
}

/// What the user should type after a load.
pub fn run_hint(p: &Program) -> String {
    match p.run {
        Some(r) => format!("{r:X}R"),
        // no run address: first block outside zero page / stack (4A.00FF
        // holds BASIC pointers, for example)
        None => p.blocks.iter().find(|(a, _)| *a >= 0x200).or(p.blocks.first()).map(|(a, _)| format!("{a:X}R")).unwrap_or_default(),
    }
}

/// Read and check an Integer BASIC file: a `.bin` (4096 bytes), a hex dump,
/// or a tape (`.wav`/`.aiff`). A tape or binary without addresses goes to
/// $E000-$EFFF. The byte at $E000 must be `JMP` ($4C, cold start).
pub fn read_basic(path: &Path) -> Result<Program, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("{} : {e}", path.display()))?;
    let kind = program::kind_of(path, Some(&bytes));
    let at_e000 = |len: usize| if len == 4096 { Ok(Some(0xE000)) } else { Err(tf("e_basic_size", &[&len])) };
    let p = match kind {
        Kind::Bin => program::load_bytes(&bytes, kind, at_e000(bytes.len())?)?,
        Kind::Hex | Kind::Prg => program::load_bytes(&bytes, kind, None)?,
        Kind::Wav => {
            let imp = aciwav::import_wav_bytes(&bytes)?;
            if imp.embedded.is_empty() {
                let n: usize = imp.segments.iter().map(|s| s.data.len()).sum();
                program::wav_to_program(&imp, at_e000(n)?)?
            } else {
                program::wav_to_program(&imp, None)?
            }
        }
    };
    let b = p.blocks.iter().find(|(a, d)| *a <= 0xE000 && (*a as usize + d.len()) > 0xE000).ok_or_else(|| t("e_basic_no_e000").to_string())?;
    let op = b.1[(0xE000 - b.0) as usize];
    if op != 0x4C {
        return Err(tf("e_basic_no_jmp", &[&format!("{op:02X}")]));
    }
    Ok(p)
}

pub fn ranges_text(p: &Program) -> String {
    p.ranges().iter().map(|(a, b)| format!("{a:04X}.{b:04X}")).collect::<Vec<_>>().join(" ")
}

impl Session {
    pub fn new(config: Config) -> Self {
        let m = Machine::new(config);
        Session { sync_cycles: m.board.cycles, m, fast: false, record_path: None, tape_source: None, sync_real: Instant::now(), was_fast: false, dbg: Default::default(), dbg_on: false, dbg_stopped: None }
    }

    /// Run the emulation up to "now" (real speed) or for `budget` (fast).
    pub fn emulate(&mut self, fast_budget: std::time::Duration) {
        let tick = Instant::now();
        if self.fast {
            while tick.elapsed() < fast_budget {
                if !self.run(20_000) {
                    break;
                }
            }
        } else {
            if self.was_fast {
                self.sync_real = tick;
                self.sync_cycles = self.m.board.cycles;
            }
            let cps = self.m.board.config.timing.effective_cps();
            let target = self.sync_cycles + (tick.duration_since(self.sync_real).as_secs_f64() * cps) as u64;
            if target > self.m.board.cycles + (cps * 0.25) as u64 {
                // fell behind (window hidden, slow machine): resynchronise
                self.sync_real = tick;
                self.sync_cycles = self.m.board.cycles;
            } else if target > self.m.board.cycles {
                self.run(target - self.m.board.cycles);
            }
        }
        // paused: the clock restarts from here when it goes on
        self.was_fast = self.fast || (self.dbg_on && self.dbg.paused);
    }

    /// Run `n` cycles, through the debugger when it is on. False when paused.
    fn run(&mut self, n: u64) -> bool {
        if !self.dbg_on {
            self.m.run_cycles(n);
            return true;
        }
        if self.dbg.paused {
            return false;
        }
        if let Some(s) = self.dbg.run(&mut self.m, n) {
            self.dbg_stopped = Some(s);
            return false;
        }
        true
    }

    fn load_into_memory(&mut self, p: &Program) -> usize {
        p.blocks.iter().map(|(a, d)| self.m.load(*a, d)).sum()
    }

    pub fn load_basic(&mut self, path: &Path) -> Result<String, String> {
        if !self.m.board.is_ram(0xE000) {
            return Err(t("s_no_ram_e000").into());
        }
        let p = read_basic(path)?;
        self.load_into_memory(&p);
        Ok(tf("s_basic_loaded", &[&p.total_len()]))
    }

    /// Load the built-in RDK G2 Basic at $E000.
    pub fn load_g2basic(&mut self) -> Result<String, String> {
        if !self.m.board.is_ram(0xE000) {
            return Err(t("s_no_ram_e000").into());
        }
        let p = apple1_core::g2basic::program();
        self.load_into_memory(&p);
        Ok(tf("s_g2basic_loaded", &[&p.total_len()]))
    }

    /// Load a program file into memory. Returns (message, program).
    pub fn load_program(&mut self, path: &Path, addr: Option<u16>) -> Result<(String, Program), String> {
        let p = program::load_file(path, addr)?;
        let skipped = self.load_into_memory(&p);
        let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let mut msg = tf("s_loaded", &[&name, &ranges_text(&p), &run_hint(&p)]);
        if skipped > 0 {
            msg.push_str(&format!(" {}", tf("s_outside_ram", &[&skipped])));
        }
        for n in &p.notes {
            msg.push_str(&format!(" ; {n}"));
        }
        Ok((msg, p))
    }

    /// Put an already decoded program in memory. Returns the message.
    pub fn load_built(&mut self, name: &str, p: &Program) -> String {
        let skipped = self.load_into_memory(p);
        let mut msg = tf("s_loaded", &[&name, &ranges_text(p), &run_hint(p)]);
        if skipped > 0 {
            msg.push_str(&format!(" {}", tf("s_outside_ram", &[&skipped])));
        }
        for n in &p.notes {
            msg.push_str(&format!(" ; {n}"));
        }
        msg
    }

    pub fn save(&mut self, path: &Path, ranges: &str) -> Result<String, String> {
        let list = program::parse_ranges(ranges)?;
        if list.is_empty() {
            return Err(t("s_need_range").into());
        }
        let mut p = Program::default();
        for (a, b) in list {
            p.blocks.push((a, self.m.dump(a, b)));
        }
        let o = ExportOptions { timing: self.m.board.config.timing, ..Default::default() };
        let k = program::save_file(path, &p, &o)?;
        let name = path.display();
        Ok(match k {
            Kind::Wav => tf("s_written_tape", &[&name, &p.aci_read_command()]),
            _ => tf("s_written", &[&name, &p.total_len()]),
        })
    }

    /// Save the whole machine (RAM, CPU, PIA, display, colour card, tape
    /// deck) to a state file; `title` is shown when loading it back.
    pub fn save_state(&mut self, path: &Path, title: &str) -> Result<String, String> {
        let bytes = self.m.save_state(title);
        std::fs::write(path, bytes).map_err(|e| format!("{} : {e}", path.display()))?;
        Ok(tf("s_state_saved", &[&path.display()]))
    }

    /// Replace the machine by the one stored in a state file. Debugger
    /// breakpoints and the recording are kept or ended as for a reset.
    pub fn load_state(&mut self, path: &Path) -> Result<(String, apple1_core::state::StateInfo), String> {
        let bytes = std::fs::read(path).map_err(|e| format!("{} : {e}", path.display()))?;
        let (m, info) = Machine::load_state(&bytes).map_err(|e| tf("s_state_bad", &[&path.display(), &e]))?;
        self.m = m;
        self.sync_cycles = self.m.board.cycles;
        self.sync_real = Instant::now();
        self.tape_source = None;
        self.dbg_stopped = None;
        let msg = tf("s_state_loaded", &[&path.display(), &format!("{:04X}", info.pc)]);
        Ok((msg, info))
    }

    pub fn insert_tape(&mut self, path: &Path) -> Result<String, String> {
        let a = wav::read_wav(path).map_err(|e| format!("{} : {e}", path.display()))?;
        let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let p = TapePlayer::from_samples(a.best_channel(), a.sample_rate, name.clone());
        let d = p.duration();
        self.m.board.player = Some(p);
        // Suggest the read command when the file carries its addresses.
        let cmd = program::load_file(path, None).map(|p| p.aci_read_command()).unwrap_or_else(|_| t("s_start_end").into());
        self.tape_source = Some(path.to_path_buf());
        Ok(tf("s_tape_inserted", &[&name, &format!("{d:.1}"), &cmd]))
    }

    pub fn play(&mut self) -> Result<String, String> {
        let now = self.m.board.now_secs();
        match self.m.board.player.as_mut() {
            Some(p) => {
                p.start(now);
                Ok(tf("s_playing", &[&p.name]))
            }
            None => Err(t("s_no_tape_load").into()),
        }
    }

    pub fn stop(&mut self) -> String {
        let mut msg = t("s_stopped").to_string();
        if let Some(p) = self.m.board.player.as_mut() {
            p.rewind();
            p.autostart = true;
        }
        if let Some(m) = self.finish_recording() {
            msg = m;
        }
        msg
    }

    pub fn rewind(&mut self) -> Result<String, String> {
        match self.m.board.player.as_mut() {
            Some(p) => {
                p.rewind();
                Ok(t("s_rewound").into())
            }
            None => Err(t("s_no_tape").into()),
        }
    }

    pub fn eject(&mut self) -> String {
        self.m.board.player = None;
        self.tape_source = None;
        t("s_ejected").into()
    }

    pub fn start_recording(&mut self, path: &Path) -> String {
        self.record_path = Some(path.to_path_buf());
        self.m.board.recording = Some(Vec::new());
        tf("s_rec_armed", &[&path.display()])
    }

    /// Save the recording (if any toggles were captured).
    pub fn finish_recording(&mut self) -> Option<String> {
        let path = self.record_path.take()?;
        let rec = self.m.board.recording.take().unwrap_or_default();
        if rec.len() < 100 {
            return Some(t("s_rec_cancelled").into());
        }
        let t = self.m.board.config.timing;
        let first = rec[0];
        let secs: Vec<f64> = rec.iter().map(|&c| t.cycles_to_secs(c - first)).collect();
        let o = ExportOptions::for_format(wav::AudioFormat::from_path(&path));
        let samples = tape::render_toggles(
            &secs,
            &RenderOptions {
                sample_rate: o.sample_rate,
                amplitude: o.amplitude,
                lead_silence: o.lead_silence,
                tail_silence: o.tail_silence,
            },
        );
        let segs = tape::decode_edges(&secs);
        let bytes = wav::encode_audio(o.format, &samples, o.sample_rate, o.bits, o.channels, None);
        Some(match std::fs::write(&path, bytes) {
            Ok(()) => {
                let n: usize = segs.iter().map(|s| s.data.len()).sum();
                tf("s_rec_saved", &[&path.display(), &format!("{:.1}", secs.last().unwrap_or(&0.0)), &segs.len(), &n])
            }
            Err(e) => tf("s_error", &[&format!("{} : {e}", path.display())]),
        })
    }

    /// Auto-save a finished recording. Returns a message when saved.
    pub fn housekeeping(&mut self) -> Option<String> {
        self.record_path.as_ref()?;
        let now = self.m.board.cycles;
        let t = self.m.board.config.timing;
        let last = self.m.board.recording.as_ref().and_then(|r| r.last().copied())?;
        if t.cycles_to_secs(now - last) > RECORD_IDLE_SECS {
            return self.finish_recording();
        }
        None
    }

    pub fn toggle_throttle(&mut self) -> String {
        let mut c = self.m.board.config.clone();
        c.display_throttle = !c.display_throttle;
        let on = c.display_throttle;
        self.m.board.apply_config(c);
        if on {
            t("s_throttle_on").into()
        } else {
            t("s_throttle_off").into()
        }
    }

    /// Integer BASIC seems to be in memory: loaded by the BASIC button, or a
    /// `JMP` (cold start) at $E000 followed by non-empty code (heuristic).
    pub fn basic_present(&self) -> bool {
        let m = &self.m.board.mem;
        self.m.board.is_ram(0xE000) && m[0xE000] == 0x4C && (0xE2B3..0xE2C0).any(|a| m[a] != 0)
    }

    /// RDK G2 Basic (identification bytes "RDK" at $E003) is in memory.
    pub fn g2_present(&self) -> bool {
        apple1_core::g2basic::present(&|a| self.m.board.peek(a))
    }

    /// Progress of the inserted tape: (fraction, started, finished).
    pub fn tape_progress(&self) -> Option<(f32, bool, bool)> {
        let p = self.m.board.player.as_ref()?;
        let now = self.m.board.now_secs();
        let d = p.duration().max(0.001);
        Some(match p.started_at {
            None => (0.0, false, false),
            Some(_) if p.finished(now) => (1.0, true, true),
            Some(_) => ((p.position(now) / d) as f32, true, false),
        })
    }

    /// Tape state: (label, position fraction 0..1) if a tape is inserted.
    pub fn tape_state(&self) -> Option<(String, f32)> {
        let p = self.m.board.player.as_ref()?;
        let now = self.m.board.now_secs();
        let d = p.duration().max(0.001);
        Some(if p.started_at.is_none() {
            (tf("s_tape_ready", &[&p.name, &format!("{d:.1}")]), 0.0)
        } else if p.finished(now) {
            (tf("s_tape_done", &[&p.name, &format!("{d:.1}")]), 1.0)
        } else {
            let pos = p.position(now);
            (tf("s_tape_pos", &[&p.name, &format!("{pos:.1}"), &format!("{d:.1}")]), (pos / d) as f32)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn boot() -> Session {
        let mut s = Session::new(Config { display_throttle: false, ..Config::default() });
        s.m.run_cycles(200_000);
        s
    }

    /// A `.prg` (2-byte address, then code), as the P-LAB / KickC demos, given as a file argument.
    #[test]
    fn prg_file_is_loaded_at_its_own_address() {
        let dir = std::env::temp_dir().join(format!("a1gui-prg-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("demo.prg");
        std::fs::write(&f, [0x80, 0x02, 0xA9, 0x41, 0x20, 0xEF, 0xFF, 0x4C, 0x80, 0x02]).unwrap();
        for plab in [false, true] {
            let mut s = Session::new(Config { display_throttle: false, plab_card: plab, ..Config::default() });
            s.m.run_cycles(200_000);
            let (msg, p) = s.load_program(&f, None).unwrap();
            assert_eq!(p.run, Some(0x0280), "{msg}");
            assert_eq!(s.m.dump(0x0280, 0x0285), vec![0xA9, 0x41, 0x20, 0xEF, 0xFF, 0x4C], "plab={plab}: {msg}");
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn state_roundtrip_through_a_file() {
        let dir = std::env::temp_dir().join(format!("a1gui-state-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("a.a1state");
        let mut s = boot();
        s.m.type_text("300: A9 41 20 EF FF 4C 00 03\n300R\n");
        s.m.run_cycles(300_000);
        let before = s.m.screen_text();
        assert!(s.save_state(&f, "essai").unwrap().contains("a.a1state"));
        let mut s2 = boot();
        let (msg, info) = s2.load_state(&f).unwrap();
        assert!(msg.contains("PC"), "{msg}");
        assert_eq!(info.title, "essai");
        assert_eq!(s2.m.screen_text(), before);
        s2.m.run_cycles(100_000);
        s.m.run_cycles(100_000);
        assert_eq!(s2.m.cpu.pc, s.m.cpu.pc);
        std::fs::write(&f, b"not a state").unwrap();
        assert!(s2.load_state(&f).is_err());
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn load_hex_and_run() {
        let dir = std::env::temp_dir().join(format!("a1gui-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("hello.txt");
        std::fs::write(&f, crate::library::HELLO_TXT).unwrap();
        let mut s = boot();
        let (msg, p) = s.load_program(&f, None).unwrap();
        assert!(msg.contains("300R"), "{msg}");
        assert_eq!(run_hint(&p), "300R");
        s.m.type_text("300R\n");
        s.m.run_cycles(3_000_000);
        assert!(s.m.screen_text().contains("HELLO WORLD!"), "{}", s.m.screen_text());
        // save as AIFF with an ACI range list, then read it back
        let a = dir.join("h.aiff");
        let msg = s.save(&a, "300.31E").unwrap();
        assert!(msg.contains("C100R") && msg.contains("0300.031ER"), "{msg}");
        let mut s2 = boot();
        let (_, p2) = s2.load_program(&a, None).unwrap();
        assert_eq!(p2.blocks, p.blocks);
        assert!(s2.insert_tape(&a).unwrap().contains("0300.031ER"));
        crate::i18n::set(crate::i18n::Lang::Fr);
        assert!(s2.tape_state().unwrap().0.contains("prête"));
        std::fs::remove_dir_all(&dir).ok();
    }
}
