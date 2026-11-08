//! Interface texte (crossterm) : écran 40x24, clavier, ligne de commande.

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use apple1_core::aciwav::ExportOptions;
use apple1_core::display::{COLS, ROWS};
use apple1_core::machine::{map_key, Machine};
use apple1_core::program::{self, Program};
use apple1_core::tape::{self, RenderOptions, TapePlayer};
use apple1_core::wav;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{cursor, execute, queue, terminal};

use crate::{config_from, cursor_from, RunArgs};

/// Recording is saved after this much emulated silence (s).
const RECORD_IDLE_SECS: f64 = 1.5;

pub const HELP: &[&str] = &[
    "TOUCHES",
    " Ctrl+R RESET        Ctrl+L CLEAR SCREEN",
    " F1 cette aide      F2 ligne de commande",
    " F3  vitesse réelle / rapide",
    " F4  affichage 60 car./s oui / non",
    " F5  cassette : lecture depuis le début",
    " F6  cassette : stop",
    " F10 ou Ctrl+Q  quitter",
    " Retour arrière = '_' (effacement Woz)",
    "COMMANDES (F2)",
    " charger FICHIER[@ADR] txt/bin/wav/aif",
    " sauver FICHIER DEBUT.FIN [DEBUT.FIN..]",
    "   (.wav/.aif = ACI, .txt hex, .bin)",
    " cassette FICHIER.wav|.aif  insère",
    " lecture | stop | rembobiner | ejecter",
    " enregistrer FICHIER.wav|.aif (...W)",
    " basic FICHIER   Integer BASIC en E000",
    " taper FICHIER   tape un fichier texte",
    " reset | effacer | rapide | reel",
    " debit (60 car./s) | aide | quitter",
];

pub struct App {
    pub m: Machine,
    pub fast: bool,
    pub message: String,
    pub quit: bool,
    pub help: bool,
    pub cmdline: Option<String>,
    pub record_path: Option<PathBuf>,
}

impl App {
    pub fn new(args: &RunArgs) -> Result<App, String> {
        let m = Machine::new(config_from(args)?);
        let mut app = App {
            m,
            fast: args.fast,
            message: "F1 : aide   Ctrl+R : RESET   F10 : quitter".into(),
            quit: false,
            help: false,
            cmdline: None,
            record_path: None,
        };
        if let Some(b) = &args.basic {
            let msg = app.load_basic(b)?;
            app.message = msg;
        }
        for spec in &args.load {
            let msg = app.load_program(spec)?;
            app.message = msg;
        }
        if let Some(t) = &args.tape {
            app.message = app.insert_tape(t)?;
        }
        if let Some(f) = &args.type_file {
            let t = std::fs::read_to_string(f).map_err(|e| format!("{} : {e}", f.display()))?;
            app.m.type_text(&t);
        }
        if let Some(r) = &args.run {
            let a = program::parse_addr(r)?;
            app.m.type_text(&format!("{a:X}R\n"));
        }
        Ok(app)
    }

    fn load_basic(&mut self, path: &Path) -> Result<String, String> {
        if !self.m.board.is_ram(0xE000) {
            return Err("pas de RAM en $E000 (option --no-e000) : impossible de charger le BASIC".into());
        }
        let bytes = std::fs::read(path).map_err(|e| format!("{} : {e}", path.display()))?;
        let kind = program::kind_of(path, Some(&bytes));
        let p = program::load_bytes(&bytes, kind, if kind == program::Kind::Hex { None } else { Some(0xE000) })?;
        self.load_into_memory(&p);
        Ok(format!("BASIC chargé ({} octets). Tapez E000R (démarrage à froid) ou E2B3R (à chaud)", p.total_len()))
    }

    fn load_into_memory(&mut self, p: &Program) -> usize {
        let mut skipped = 0;
        for (a, d) in &p.blocks {
            skipped += self.m.load(*a, d);
        }
        skipped
    }

    pub fn load_program(&mut self, spec: &str) -> Result<String, String> {
        let (path, a) = program::split_at_addr(spec)?;
        let p = program::load_file(Path::new(&path), a)?;
        let skipped = self.load_into_memory(&p);
        let ranges: Vec<String> = p.ranges().iter().map(|(a, b)| format!("{a:04X}.{b:04X}")).collect();
        let mut msg = format!("Chargé {} en {}", path, ranges.join(" "));
        if let Some(r) = p.run {
            msg.push_str(&format!(" ; lancez par {r:X}R"));
        }
        if skipped > 0 {
            msg.push_str(&format!(" ; ATTENTION {skipped} octet(s) hors RAM ignoré(s)"));
        }
        for n in &p.notes {
            msg.push_str(&format!(" ; {n}"));
        }
        Ok(msg)
    }

    pub fn save(&mut self, path: &str, ranges: &[&str]) -> Result<String, String> {
        if ranges.is_empty() {
            return Err("usage : sauver FICHIER DEBUT.FIN [DEBUT.FIN ...]".into());
        }
        let mut p = Program::default();
        for r in ranges {
            let (a, b) = program::parse_range(r)?;
            p.blocks.push((a, self.m.dump(a, b)));
        }
        let o = ExportOptions { timing: self.m.board.config.timing, ..Default::default() };
        let k = program::save_file(Path::new(path), &p, &o)?;
        Ok(match k {
            program::Kind::Wav => format!("Écrit {path} ; sur l'Apple-1 : C100R puis {}", p.aci_read_command()),
            _ => format!("Écrit {path} ({} octets)", p.total_len()),
        })
    }

    pub fn insert_tape(&mut self, path: &Path) -> Result<String, String> {
        let a = wav::read_wav(path).map_err(|e| format!("{} : {e}", path.display()))?;
        let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let p = TapePlayer::from_samples(a.best_channel(), a.sample_rate, name.clone());
        let d = p.duration();
        self.m.board.player = Some(p);
        Ok(format!(
            "Cassette « {name} » insérée ({d:.1} s). Tapez C100R puis DEBUT.FINR : la bande démarre à la première lecture"
        ))
    }

    pub fn start_recording(&mut self, path: &str) -> String {
        self.record_path = Some(PathBuf::from(path));
        self.m.board.recording = Some(Vec::new());
        format!("Enregistrement armé vers {path} : tapez C100R puis DEBUT.FINW")
    }

    /// Save the recording (if any toggles were captured).
    pub fn finish_recording(&mut self) -> Option<String> {
        let path = self.record_path.take()?;
        let rec = self.m.board.recording.take().unwrap_or_default();
        if rec.len() < 100 {
            return Some("Enregistrement annulé (rien n'a été écrit sur la cassette)".into());
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
        match std::fs::write(&path, bytes) {
            Ok(()) => {
                let n: usize = segs.iter().map(|s| s.data.len()).sum();
                Some(format!(
                    "Cassette enregistrée dans {} ({:.1} s, {} enregistrement(s), {n} octets)",
                    path.display(),
                    secs.last().unwrap_or(&0.0),
                    segs.len()
                ))
            }
            Err(e) => Some(format!("Erreur : {} : {e}", path.display())),
        }
    }

    /// Execute a command typed after F2. Returns the message to display.
    pub fn command(&mut self, line: &str) -> Result<String, String> {
        let mut it = line.split_whitespace();
        let Some(cmd) = it.next() else { return Ok(String::new()) };
        let args: Vec<&str> = it.collect();
        let arg1 = || args.first().copied().ok_or_else(|| format!("« {cmd} » : il manque le nom de fichier"));
        match cmd.to_lowercase().as_str() {
            "charger" | "load" => self.load_program(arg1()?),
            "sauver" | "save" => self.save(arg1()?, &args[1..]),
            "cassette" | "tape" => self.insert_tape(Path::new(arg1()?)),
            "lecture" | "play" => {
                let now = self.m.board.now_secs();
                match self.m.board.player.as_mut() {
                    Some(p) => {
                        p.start(now);
                        Ok(format!("Lecture de « {} »", p.name))
                    }
                    None => Err("pas de cassette (commande : cassette FICHIER.wav)".into()),
                }
            }
            "stop" => {
                let mut msg = String::from("Lecteur arrêté");
                if let Some(p) = self.m.board.player.as_mut() {
                    p.rewind();
                    p.autostart = true;
                }
                if let Some(m) = self.finish_recording() {
                    msg = m;
                }
                Ok(msg)
            }
            "rembobiner" | "rewind" => match self.m.board.player.as_mut() {
                Some(p) => {
                    p.rewind();
                    Ok("Cassette rembobinée (redémarre à la prochaine lecture de l'ACI)".into())
                }
                None => Err("pas de cassette".into()),
            },
            "ejecter" | "éjecter" | "eject" => {
                self.m.board.player = None;
                Ok("Cassette éjectée".into())
            }
            "enregistrer" | "record" => Ok(self.start_recording(arg1()?)),
            "basic" => self.load_basic(Path::new(arg1()?)),
            "taper" | "type" => {
                let f = arg1()?;
                let t = std::fs::read_to_string(f).map_err(|e| format!("{f} : {e}"))?;
                self.m.type_text(&t);
                Ok(format!("Frappe de {f} ({} caractères)", t.len()))
            }
            "reset" => {
                self.m.reset();
                Ok("RESET".into())
            }
            "effacer" | "clear" => {
                self.m.clear_screen();
                Ok("Écran effacé".into())
            }
            "rapide" | "fast" => {
                self.fast = true;
                Ok("Vitesse : rapide".into())
            }
            "reel" | "réel" | "real" => {
                self.fast = false;
                Ok("Vitesse : réelle".into())
            }
            "debit" | "débit" | "throttle" => Ok(self.toggle_throttle()),
            "aide" | "help" => {
                self.help = true;
                Ok(String::new())
            }
            "quitter" | "quit" => {
                self.quit = true;
                Ok(String::new())
            }
            _ => Err(format!("commande inconnue : « {cmd} » (F1 : aide)")),
        }
    }

    fn toggle_throttle(&mut self) -> String {
        let mut c = self.m.board.config.clone();
        c.display_throttle = !c.display_throttle;
        let on = c.display_throttle;
        self.m.board.apply_config(c);
        if on {
            "Affichage limité à 1 caractère par image (~60 car./s)".into()
        } else {
            "Affichage instantané".into()
        }
    }

    /// Called regularly: auto-save a finished recording.
    pub fn housekeeping(&mut self) {
        if self.record_path.is_some() {
            let now = self.m.board.cycles;
            let t = self.m.board.config.timing;
            if let Some(last) = self.m.board.recording.as_ref().and_then(|r| r.last().copied()) {
                if t.cycles_to_secs(now - last) > RECORD_IDLE_SECS {
                    if let Some(m) = self.finish_recording() {
                        self.message = m;
                    }
                }
            }
        }
    }

    pub fn status(&self) -> String {
        let c = &self.m.board.config;
        let speed = if self.fast {
            "rapide".to_string()
        } else {
            format!("réelle {:.3} MHz", c.timing.effective_cps() / 1e6)
        };
        let kb = if c.color_card { 48 } else { c.ram_kb };
        let ram = format!("{kb}K{}", if c.ram_e000 { "+E000" } else { "" });
        let mut s = format!(
            "Vitesse {speed} | écran {} | RAM {ram}{}{}",
            if c.display_throttle { "60 car/s" } else { "instant." },
            if c.aci { " | ACI" } else { "" },
            if c.color_card { format!(" | GEN2 {}", self.m.board.gen2.mode_name()) } else { String::new() }
        );
        let now = self.m.board.now_secs();
        if let Some(p) = &self.m.board.player {
            let state = if p.started_at.is_none() {
                "prête".to_string()
            } else if p.finished(now) {
                "finie".to_string()
            } else {
                format!("{:.1}/{:.1} s", p.position(now), p.duration())
            };
            s.push_str(&format!(" | K7 {} {}", p.name, state));
        }
        if self.record_path.is_some() {
            s.push_str(" | ENR");
        }
        s
    }

    pub fn handle_key(&mut self, k: KeyEvent) {
        if k.kind == KeyEventKind::Release {
            return;
        }
        if let Some(line) = self.cmdline.as_mut() {
            match k.code {
                KeyCode::Enter => {
                    let l = line.clone();
                    self.cmdline = None;
                    self.message = match self.command(&l) {
                        Ok(m) => m,
                        Err(e) => format!("Erreur : {e}"),
                    };
                }
                KeyCode::Esc | KeyCode::F(2) => self.cmdline = None,
                KeyCode::Backspace => {
                    line.pop();
                }
                KeyCode::Char(c) => line.push(c),
                _ => {}
            }
            return;
        }
        if self.help {
            self.help = false;
            if !matches!(k.code, KeyCode::F(_)) {
                return;
            }
        }
        let ctrl = k.modifiers.contains(KeyModifiers::CONTROL);
        match k.code {
            KeyCode::Char('r') | KeyCode::Char('R') if ctrl => {
                self.m.reset();
                self.message = "RESET".into();
            }
            KeyCode::Char('l') | KeyCode::Char('L') if ctrl => self.m.clear_screen(),
            KeyCode::Char('q') | KeyCode::Char('Q') if ctrl => self.quit = true,
            KeyCode::Char('c') | KeyCode::Char('C') if ctrl => self.quit = true,
            KeyCode::F(10) => self.quit = true,
            KeyCode::F(1) => self.help = true,
            KeyCode::F(2) => self.cmdline = Some(String::new()),
            KeyCode::F(3) => {
                self.fast = !self.fast;
                self.message = if self.fast { "Vitesse : rapide".into() } else { "Vitesse : réelle".into() };
            }
            KeyCode::F(4) => self.message = self.toggle_throttle(),
            KeyCode::F(5) => self.message = self.command("lecture").unwrap_or_else(|e| format!("Erreur : {e}")),
            KeyCode::F(6) => self.message = self.command("stop").unwrap_or_else(|e| format!("Erreur : {e}")),
            KeyCode::Enter => self.m.press_key(0x0D),
            KeyCode::Esc => self.m.press_key(0x1B),
            KeyCode::Backspace | KeyCode::Delete => self.m.press_key(b'_'),
            KeyCode::Tab => self.m.press_key(b' '),
            KeyCode::Char(c) if ctrl => {
                // other control keys: ASCII control codes, like the real keyboard
                let u = c.to_ascii_uppercase();
                if u.is_ascii_uppercase() {
                    self.m.press_key(u as u8 - b'@');
                }
            }
            KeyCode::Char(c) => {
                if let Some(k) = map_key(c) {
                    self.m.press_key(k);
                }
            }
            _ => {}
        }
    }
}

struct TermGuard;
impl Drop for TermGuard {
    fn drop(&mut self) {
        let mut out = io::stdout();
        let _ = execute!(out, event::DisableBracketedPaste, ResetColor, cursor::Show, terminal::LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

fn draw(out: &mut impl Write, app: &App, blink_on: bool) -> io::Result<()> {
    let green = Color::Rgb { r: 0x33, g: 0xFF, b: 0x66 };
    let dim = Color::Rgb { r: 0x22, g: 0x88, b: 0x44 };
    queue!(out, cursor::MoveTo(0, 0), SetBackgroundColor(Color::Black), SetForegroundColor(dim))?;
    queue!(out, Print(format!("┌{:─^40}┐", " APPLE-1 ")))?;
    let d = &app.m.board.display;
    for r in 0..ROWS {
        let mut line = String::with_capacity(COLS);
        for c in 0..COLS {
            let ch = if r == d.row && c == d.col && blink_on { '@' } else { d.cell(r, c) as char };
            line.push(ch);
        }
        if app.help {
            if let Some(h) = HELP.get(r) {
                line = format!("{:<40}", h.chars().take(40).collect::<String>());
            } else if r == ROWS - 1 {
                line = format!("{:<40}", "(une touche pour fermer l'aide)");
            } else {
                line = " ".repeat(40);
            }
        }
        queue!(
            out,
            cursor::MoveTo(0, r as u16 + 1),
            SetForegroundColor(dim),
            Print("│"),
            SetForegroundColor(if app.help { Color::White } else { green }),
            Print(line),
            SetForegroundColor(dim),
            Print("│")
        )?;
    }
    queue!(out, cursor::MoveTo(0, ROWS as u16 + 1), Print(format!("└{}┘", "─".repeat(40))))?;
    let (w, _) = terminal::size().unwrap_or((80, 30));
    let w = w as usize;
    let fit = |s: &str| {
        let mut t: String = s.chars().take(w.saturating_sub(1)).collect();
        let n = t.chars().count();
        t.push_str(&" ".repeat(w.saturating_sub(1).saturating_sub(n)));
        t
    };
    queue!(out, cursor::MoveTo(0, ROWS as u16 + 2), SetForegroundColor(Color::Grey), Print(fit(&app.status())))?;
    let last = match &app.cmdline {
        Some(l) => format!("commande> {l}_"),
        None => app.message.clone(),
    };
    queue!(out, cursor::MoveTo(0, ROWS as u16 + 3), SetForegroundColor(Color::Yellow), Print(fit(&last)), ResetColor)?;
    out.flush()
}

pub fn run(args: RunArgs) -> Result<(), String> {
    let mut app = App::new(&args)?;
    // NE555 D13: 2.18 Hz, visible 2/3 of the time (INTERFACES.md), or --cursor-hz.
    let blink = cursor_from(&args)?;
    terminal::enable_raw_mode().map_err(|e| format!("terminal : {e}"))?;
    let _guard = TermGuard;
    let mut out = io::stdout();
    execute!(out, terminal::EnterAlternateScreen, cursor::Hide, event::EnableBracketedPaste, terminal::Clear(terminal::ClearType::All))
        .map_err(|e| e.to_string())?;

    let frame = Duration::from_micros(16_667);
    let mut sync_real = Instant::now();
    let mut sync_cycles = app.m.board.cycles;
    let mut was_fast = app.fast;
    let started = Instant::now();
    let mut last_drawn: Option<(u64, bool, String, bool)> = None;
    while !app.quit {
        let tick = Instant::now();
        // --- emulate ---
        if app.fast {
            let budget = Duration::from_millis(14);
            while tick.elapsed() < budget {
                app.m.run_cycles(20_000);
            }
        } else {
            if was_fast {
                sync_real = tick;
                sync_cycles = app.m.board.cycles;
            }
            let cps = app.m.board.config.timing.effective_cps();
            let target = sync_cycles + (tick.duration_since(sync_real).as_secs_f64() * cps) as u64;
            if target > app.m.board.cycles + (cps * 0.25) as u64 {
                // we fell behind (terminal suspended...): resynchronise
                sync_real = tick;
                sync_cycles = app.m.board.cycles;
            } else if target > app.m.board.cycles {
                app.m.run_cycles(target - app.m.board.cycles);
            }
        }
        was_fast = app.fast;
        app.housekeeping();
        // --- draw ---
        let blink_on = blink.visible_at(started.elapsed().as_secs_f64());
        let key = (app.m.board.display.generation, blink_on, app.status() + &app.message + &format!("{:?}", app.cmdline), app.help);
        if last_drawn.as_ref() != Some(&key) {
            draw(&mut out, &app, blink_on).map_err(|e| e.to_string())?;
            last_drawn = Some(key);
        }
        // --- input ---
        let mut timeout = frame.saturating_sub(tick.elapsed());
        loop {
            if !event::poll(timeout).map_err(|e| e.to_string())? {
                break;
            }
            match event::read().map_err(|e| e.to_string())? {
                Event::Key(k) => app.handle_key(k),
                Event::Paste(s) => {
                    if let Some(l) = app.cmdline.as_mut() {
                        l.push_str(s.lines().next().unwrap_or(""));
                    } else {
                        app.m.type_text(&s);
                    }
                }
                Event::Resize(_, _) => {
                    let _ = execute!(out, terminal::Clear(terminal::ClearType::All));
                    last_drawn = None;
                }
                _ => {}
            }
            timeout = Duration::ZERO;
        }
    }
    if let Some(m) = app.finish_recording() {
        drop(_guard);
        eprintln!("{m}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> App {
        let args = RunArgs { ram: 8, ..Default::default() };
        App::new(&args).unwrap()
    }

    #[test]
    fn save_and_load_roundtrip_via_commands() {
        let dir = std::env::temp_dir().join(format!("a1emu-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut a = app();
        a.m.load(0x300, &[1, 2, 3, 4]);
        for ext in ["txt", "bin", "wav", "aif", "aiff"] {
            let f = dir.join(format!("t.{ext}"));
            let fs = f.to_str().unwrap();
            a.command(&format!("sauver {fs} 300.303")).unwrap();
            let mut b = app();
            let spec = if ext == "bin" { format!("{fs}@300") } else { fs.to_string() };
            b.command(&format!("charger {spec}")).unwrap();
            assert_eq!(b.m.dump(0x300, 0x303), vec![1, 2, 3, 4], "{ext}");
        }
        // two ranges in zero page + $300, typed like the Star Trek line
        a.m.load(0x4A, &[9, 8, 7]);
        let f = dir.join("multi.aiff");
        let msg = a.command(&format!("sauver {} 4A.004CR 300.303R", f.display())).unwrap();
        assert!(msg.contains("004A.004CR 0300.0303R"), "{msg}");
        let bytes = std::fs::read(&f).unwrap();
        assert_eq!(&bytes[0..4], b"FORM");
        let mut b = app();
        b.command(&format!("charger {}", f.display())).unwrap();
        assert_eq!(b.m.dump(0x4A, 0x4C), vec![9, 8, 7]);
        assert_eq!(b.m.dump(0x300, 0x303), vec![1, 2, 3, 4]);
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn record_via_rom_and_tape_read_back() {
        let dir = std::env::temp_dir().join(format!("a1emu-rec-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("rec.aiff");
        let mut a = app();
        a.m.board.config.display_throttle = false;
        a.m.load(0x300, b"HELLO");
        a.m.run_until_idle(1_000_000);
        a.command(&format!("enregistrer {}", f.display())).unwrap();
        a.m.type_text("C100R\n300.304W\n");
        a.m.run_until_idle(30_000_000);
        a.m.run_cycles(2_000_000);
        a.housekeeping();
        assert!(a.record_path.is_none(), "{}", a.message);
        let mut b = app();
        b.command(&format!("cassette {}", f.display())).unwrap();
        b.m.type_text("C100R\n300.304R\n");
        b.m.run_until_idle(30_000_000);
        assert_eq!(b.m.dump(0x300, 0x304), b"HELLO".to_vec());
        std::fs::remove_dir_all(dir).ok();
    }
}
