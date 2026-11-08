//! « Débogueur » panel: pause, steps (instruction, over a JSR, out of a
//! routine, to the cursor, Integer BASIC line), breakpoints on addresses and
//! BASIC lines, registers and flags (editable), disassembly with the labels
//! of the assembled program, BASIC variables, memory watches and a hex/ASCII
//! dump with the changed bytes highlighted. Docked on the right or in its
//! own window (crate::dock). The stepping itself is apple1_core::debug.

use std::time::Instant;

use apple1_core::debug::{self, Mission, Stop, Symbols, VarValue};
use eframe::egui::{self, Color32, FontId, RichText};
use serde::{Deserialize, Serialize};

use crate::app::Hint;
use crate::i18n::{t, tf};
use crate::session::Session;

/// How a memory watch is shown.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Fmt {
    Byte,
    Word,
    Ascii,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MemWatch {
    pub addr: u16,
    pub len: u16,
    pub fmt: Fmt,
}

/// Saved in gui.json.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct DbgCfg {
    pub open: bool,
    pub dock: crate::dock::DockCfg,
    /// BASIC variables watched (`A`, `B1`, `C$`, `D(2)`).
    pub vars: Vec<String>,
    pub mems: Vec<MemWatch>,
    /// First address of the dump.
    pub dump: u16,
    /// The « Désassemblage » and « Dump » windows.
    pub disasm: WinCfg,
    pub dump_win: WinCfg,
    /// Breakpoints, kept from one session to the next.
    pub breaks: Vec<u16>,
    pub basic_breaks: Vec<u16>,
    /// Text size of the debugger panel.
    #[serde(default = "crate::fontsz::default_size")]
    pub font: f32,
}

/// A window of the debugger: shown or not, docked or separate.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct WinCfg {
    pub open: bool,
    pub dock: crate::dock::DockCfg,
    /// Text size of this window (points), remembered in gui.json.
    #[serde(default = "crate::fontsz::default_size")]
    pub font: f32,
}

impl WinCfg {
    fn new(w: f32, h: f32) -> Self {
        WinCfg { open: true, dock: crate::dock::DockCfg { detached: true, width: w, pos: None, size: Some([w, h]) }, font: crate::fontsz::DEFAULT }
    }
}

impl Default for WinCfg {
    fn default() -> Self {
        WinCfg::new(520.0, 560.0)
    }
}

impl Default for DbgCfg {
    fn default() -> Self {
        DbgCfg {
            open: false,
            dock: crate::dock::DockCfg { width: 440.0, ..Default::default() },
            vars: Vec::new(),
            mems: Vec::new(),
            dump: 0,
            disasm: WinCfg::new(520.0, 560.0),
            dump_win: WinCfg::new(700.0, 520.0),
            breaks: Vec::new(),
            basic_breaks: Vec::new(),
            font: crate::fontsz::DEFAULT,
        }
    }
}

/// `$30`, `0200.020F`, `label`, then ` w` (word) or ` a` (ASCII).
pub fn parse_mem(text: &str, syms: &[(String, i64)]) -> Option<MemWatch> {
    let t = text.trim();
    let (body, fmt) = match t.rsplit_once([' ', ':']) {
        Some((b, f)) if f.eq_ignore_ascii_case("w") || f.eq_ignore_ascii_case("m") => (b, Some(Fmt::Word)),
        Some((b, f)) if f.eq_ignore_ascii_case("a") => (b, Some(Fmt::Ascii)),
        Some((b, f)) if f.eq_ignore_ascii_case("b") || f.eq_ignore_ascii_case("o") => (b, Some(Fmt::Byte)),
        _ => (t, None),
    };
    let (a, len) = match body.split_once('.') {
        Some((a, b)) => {
            let a = debug::parse_addr(a, syms)?;
            let b = debug::parse_addr(b, syms)?;
            if b < a || b - a > 255 {
                return None;
            }
            (a, b - a + 1)
        }
        None => (debug::parse_addr(body, syms)?, 0),
    };
    let fmt = fmt.unwrap_or(Fmt::Byte);
    let len = if len == 0 {
        match fmt {
            Fmt::Word => 2,
            Fmt::Ascii => 16,
            Fmt::Byte => 1,
        }
    } else {
        len
    };
    Some(MemWatch { addr: a, len, fmt })
}

/// Commands of the toolbar (also from the script options).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmd {
    Pause,
    Continue,
    Into,
    Over,
    Out,
    To(u16),
    BasicLine,
}

impl Cmd {
    pub fn parse(s: &str) -> Option<Cmd> {
        Some(match s {
            "pause" => Cmd::Pause,
            "continuer" | "continue" => Cmd::Continue,
            "pas" | "step" => Cmd::Into,
            "dessus" | "over" => Cmd::Over,
            "sortir" | "out" => Cmd::Out,
            "basic" => Cmd::BasicLine,
            _ => return None,
        })
    }
}

pub struct DebugUi {
    /// Symbols of the last program assembled and run.
    pub syms: Vec<(String, i64)>,
    /// Symbols of the source open in the editor (assembled as it is typed): a label can be
    /// used before the first run (#46).
    pub ed_syms: Vec<(String, i64)>,
    /// Labels asked for a breakpoint before they exist: set when they do.
    pub pending: Vec<String>,
    map: Symbols,
    /// Bytes per row of the dump (16, or 8 when narrow).
    dump_per: u16,
    /// First address shown by the disassembly: kept while PC stays in the
    /// window, so the list does not move when PC loops (key wait).
    dis_top: Option<u16>,
    dis_follow: bool,
    dis_goto: String,
    /// Memory when the last command started (changed bytes in colour),
    /// and the copy refreshing it while running.
    base: Vec<u8>,
    last: Vec<u8>,
    base_t: Instant,
    bp_in: String,
    bbp_in: String,
    var_in: String,
    mem_in: String,
    goto_in: String,
    poke_addr: String,
    poke_val: String,
    dump_goto: bool,
    /// Hex digit typed in the dump, waiting for the second one: (address, high nibble).
    dump_half: Option<(u16, u8)>,
    pub msg: Option<String>,
    /// PC / BASIC line when the panel was last drawn (scroll the editor).
    pub seen: Option<(u16, Option<u16>)>,
    /// Bytes (first, last) of the source line(s) selected in the editor.
    pub hl: Vec<(u16, u16)>,
    /// Bring this address into view in the disassembly and in the dump.
    pub jump_dis: Option<u16>,
    pub jump_dump: Option<u16>,
    /// Addresses shown by the disassembly the last time.
    dis_shown: (u16, u16),
}

/// A name that could be a label (not an address).
fn is_label(x: &str) -> bool {
    let mut c = x.chars();
    c.next().is_some_and(|f| f.is_ascii_alphabetic() || f == '_' || f == '.') && x.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
}

impl DebugUi {
    fn in_hl(&self, a: u16, len: u16) -> bool {
        self.hl.iter().any(|&(f, l)| a <= l && a.wrapping_add(len.max(1) - 1) >= f)
    }

    pub fn jump_to(&mut self, a: u16) {
        self.jump_dis = Some(a);
        self.jump_dump = Some(a);
    }
}

impl Default for DebugUi {
    fn default() -> Self {
        DebugUi {
            syms: Vec::new(),
            ed_syms: Vec::new(),
            pending: Vec::new(),
            map: Symbols::new(),
            dump_per: 0,
            dis_top: None,
            dis_follow: true,
            dis_goto: String::new(),
            base: Vec::new(),
            last: Vec::new(),
            base_t: Instant::now(),
            bp_in: String::new(),
            bbp_in: String::new(),
            var_in: String::new(),
            mem_in: String::new(),
            goto_in: String::new(),
            poke_addr: String::new(),
            poke_val: String::new(),
            dump_goto: true,
            dump_half: None,
            msg: None,
            seen: None,
            hl: Vec::new(),
            jump_dis: None,
            jump_dump: None,
            dis_shown: (0, 0),
        }
    }
}

fn snap(s: &Session) -> Vec<u8> {
    (0..=0xFFFFu16).map(|a| s.m.board.peek(a)).collect()
}

const MONO: f32 = 13.0;
/// Rows of the disassembly, always the same number (no jumping layout).
const DIS_ROWS: usize = 24;
/// PC is kept at least this many rows above the bottom.
const DIS_KEEP: usize = 4;
/// A colour for the dark theme, another for the light one (readable on both).
fn pick(dark: Color32, light: Color32) -> Color32 {
    if crate::syntax::dark() { dark } else { light }
}
fn chg_col() -> Color32 {
    crate::uicolors::get(crate::uicolors::Role::Changed)
}
fn dim() -> Color32 {
    pick(Color32::GRAY, Color32::from_rgb(90, 90, 90))
}
fn text_col() -> Color32 {
    crate::uicolors::get(crate::uicolors::Role::DumpAscii)
}
fn addr_col() -> Color32 {
    crate::uicolors::get(crate::uicolors::Role::DumpAddr)
}
fn err_col() -> Color32 {
    pick(Color32::LIGHT_RED, Color32::from_rgb(170, 0, 0))
}
const BRK: Color32 = Color32::from_rgb(230, 90, 90);

fn mono_px() -> f32 {
    MONO_PX.with(|c| c.get())
}

fn set_mono(px: f32) {
    MONO_PX.with(|c| c.set(px.clamp(crate::editor::FONT_MIN, crate::editor::FONT_MAX)));
}

std::thread_local! {
    static MONO_PX: std::cell::Cell<f32> = const { std::cell::Cell::new(MONO) };
}

fn mono(s: impl Into<String>) -> RichText {
    RichText::new(s).font(FontId::monospace(mono_px()))
}

/// Text of a stop, for the panel and the status bar.
pub fn stop_text(s: Stop, syms: &Symbols) -> String {
    let at = |a: u16| match syms.get(&a) {
        Some(n) => format!("${a:04X} ({n})"),
        None => format!("${a:04X}"),
    };
    match s {
        Stop::Pause => t("db_st_pause").into(),
        Stop::Step => t("db_st_step").into(),
        Stop::Break(a) => tf("db_st_break", &[&at(a)]),
        Stop::BasicLine(l) => tf("db_st_basic", &[&l]),
        Stop::BasicBreak(l) => tf("db_st_basic_break", &[&l]),
        Stop::Jam => t("db_st_jam").into(),
    }
}

impl DebugUi {
    pub fn set_symbols(&mut self, syms: Vec<(String, i64)>) {
        self.map = debug::symbols_from(&syms);
        self.syms = syms;
    }

    pub fn symbols(&self) -> &Symbols {
        &self.map
    }

    /// Memory now becomes the reference for the changed bytes.
    fn mark(&mut self, s: &Session) {
        self.base = snap(s);
        self.last = self.base.clone();
        self.base_t = Instant::now();
    }

    pub fn command(&mut self, s: &mut Session, c: Cmd) {
        if c != Cmd::Pause {
            self.mark(s);
        }
        s.dbg_on = true;
        match c {
            Cmd::Pause => s.dbg.pause(),
            Cmd::Continue => s.dbg.cont(),
            Cmd::Into => s.dbg.go(Mission::Into),
            Cmd::Over => s.dbg.over(&s.m),
            Cmd::Out => s.dbg.out(&s.m),
            Cmd::To(a) => s.dbg.go(Mission::To(a)),
            Cmd::BasicLine => s.dbg.go(Mission::BasicLine),
        }
    }

    /// Disassembly shown from this address (not following the PC).
    pub fn show_disasm(&mut self, a: u16) {
        self.dis_top = Some(a);
        self.dis_follow = false;
    }

    pub fn toggle_break(s: &mut Session, a: u16) {
        if !s.dbg.breaks.remove(&a) {
            s.dbg.breaks.insert(a);
        }
    }

    pub fn toggle_basic_break(s: &mut Session, l: u16) {
        if !s.dbg.basic_breaks.remove(&l) {
            s.dbg.basic_breaks.insert(l);
        }
    }

    /// Breakpoint from text: an address, a label, or `L190` for a BASIC line.
    pub fn add_break_text(&mut self, s: &mut Session, text: &str) -> bool {
        let x = text.trim();
        if let Some(n) = x.strip_prefix(['L', 'l']).and_then(|n| n.parse::<u16>().ok()) {
            s.dbg.basic_breaks.insert(n);
            return true;
        }
        let found = debug::parse_addr(x, &self.syms).or_else(|| debug::parse_addr(x, &self.ed_syms));
        match found {
            Some(a) => {
                s.dbg.breaks.insert(a);
                true
            }
            None if is_label(x) => {
                // not assembled yet: kept until the label exists
                if !self.pending.iter().any(|p| p == x) {
                    self.pending.push(x.to_string());
                }
                self.msg = Some(tf("db_pending", &[&x]));
                true
            }
            None => false,
        }
    }

    /// Breakpoints on labels that were not known when asked: set now if they are.
    pub fn resolve_pending(&mut self, s: &mut Session) {
        if self.pending.is_empty() {
            return;
        }
        let (syms, ed) = (&self.syms, &self.ed_syms);
        let before = self.pending.len();
        self.pending.retain(|x| match debug::parse_addr(x, syms).or_else(|| debug::parse_addr(x, ed)) {
            Some(a) => {
                s.dbg.breaks.insert(a);
                false
            }
            None => true,
        });
        if self.pending.len() < before {
            self.msg = None;
        }
    }

    /// Once per frame, before the windows: reference of the changed bytes.
    pub fn refresh(&mut self, s: &Session) {
        if self.base.is_empty() {
            self.mark(s);
        }
        if !s.dbg.paused && self.base_t.elapsed().as_secs_f32() > 1.0 {
            self.base = std::mem::replace(&mut self.last, snap(s));
            self.base_t = Instant::now();
        }
    }

    /// The main panel. Returns true when the saved settings changed.
    pub fn contents(&mut self, ui: &mut egui::Ui, h: bool, s: &mut Session, cfg: &mut DbgCfg) -> bool {
        let mut changed = false;
        if cfg.font < crate::editor::FONT_MIN {
            cfg.font = crate::fontsz::DEFAULT;
        }
        set_mono(cfg.font);
        ui.horizontal(|ui| {
            if crate::fontsz::control(ui, "dbg", &mut cfg.font, h) {
                changed = true;
            }
        });
        if crate::fontsz::wheel(ui, ui.max_rect(), &mut cfg.font) {
            changed = true;
        }
        set_mono(cfg.font);
        let paused = s.dbg.paused;
        let basic = debug::basic_present(&|a| s.m.board.peek(a));
        // --- commands
        ui.horizontal_wrapped(|ui| {
            if paused {
                if ui.button(RichText::new(t("db_continue")).strong()).hint(h, t("db_continue_tip"), "h_db_continue").clicked() {
                    self.command(s, Cmd::Continue);
                }
            } else if ui.button(t("db_pause")).hint(h, t("db_pause_tip"), "h_db_pause").clicked() {
                self.command(s, Cmd::Pause);
            }
            if ui.button(t("db_step")).hint(h, t("db_step_tip"), "h_db_step").clicked() {
                self.command(s, Cmd::Into);
            }
            if ui.button(t("db_over")).hint(h, t("db_over_tip"), "h_db_over").clicked() {
                self.command(s, Cmd::Over);
            }
            if ui.button(t("db_out")).hint(h, t("db_out_tip"), "h_db_out").clicked() {
                self.command(s, Cmd::Out);
            }
            if ui.add_enabled(basic, egui::Button::new(t("db_basic_step"))).hint(h, t("db_basic_step_tip"), "h_db_basic_step").clicked() {
                self.command(s, Cmd::BasicLine);
            }
        });
        ui.horizontal(|ui| {
            if ui.selectable_label(cfg.disasm.open, t("db_win_disasm")).hint(h, t("db_win_disasm_tip"), "h_db_win_disasm").clicked() {
                cfg.disasm.open = !cfg.disasm.open;
                changed = true;
            }
            if ui.selectable_label(cfg.dump_win.open, t("db_win_dump")).hint(h, t("db_win_dump_tip"), "h_db_win_dump").clicked() {
                cfg.dump_win.open = !cfg.dump_win.open;
                changed = true;
            }
        });
        let state = match (paused, s.dbg.last_stop) {
            (true, Some(st)) => RichText::new(stop_text(st, &self.map)).color(pick(Color32::from_rgb(255, 200, 120), Color32::from_rgb(160, 70, 0))).strong(),
            (true, None) => RichText::new(t("db_st_pause")).strong(),
            _ => RichText::new(t("db_running")).color(pick(Color32::LIGHT_GREEN, Color32::from_rgb(0, 110, 0))),
        };
        ui.add(egui::Label::new(state).truncate()).hint(h, "", "h_db_state");
        // always one row, empty or not: the layout below does not move
        ui.add(egui::Label::new(RichText::new(self.msg.as_deref().unwrap_or(" ")).color(err_col())).truncate());
        ui.separator();
        egui::ScrollArea::vertical()
            .id_salt("debogueur-defile")
            .auto_shrink([false, false])
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .show(ui, |ui| {
                self.cpu_ui(ui, h, s);
                self.breaks_ui(ui, h, s);
                changed |= self.basic_ui(ui, h, s, cfg, basic);
                changed |= self.mems_ui(ui, h, s, cfg);
            });
        if std::env::var_os("APPLE1_DEBUG_LAYOUT").is_some() {
            eprintln!("disposition : PC ${:04X} début désassemblage {:?} hauteur panneau {}", s.m.cpu.pc, self.dis_top, ui.min_rect().height());
        }
        changed
    }

    fn cpu_ui(&mut self, ui: &mut egui::Ui, h: bool, s: &mut Session) {
        egui::CollapsingHeader::new(t("db_cpu")).id_salt("db-cpu").default_open(true).show(ui, |ui| {
            let c = &mut s.m.cpu;
            ui.horizontal_wrapped(|ui| {
                for (name, r) in [("A", &mut c.a), ("X", &mut c.x), ("Y", &mut c.y), ("S", &mut c.sp)] {
                    ui.label(mono(name));
                    ui.add(egui::DragValue::new(r).hexadecimal(2, false, true).prefix("$")).hint(h, t("db_reg_tip"), "h_db_regs");
                }
                ui.label(mono("PC"));
                ui.add(egui::DragValue::new(&mut c.pc).hexadecimal(4, false, true).prefix("$")).hint(h, t("db_reg_tip"), "h_db_regs");
            });
            ui.horizontal_wrapped(|ui| {
                ui.label(mono("P"));
                ui.label(mono(format!("${:02X}", c.p)));
                for (name, bit) in [("N", 7), ("V", 6), ("B", 4), ("D", 3), ("I", 2), ("Z", 1), ("C", 0)] {
                    let mut on = c.p & (1 << bit) != 0;
                    if ui.checkbox(&mut on, mono(name)).hint(h, t("db_flag_tip"), "h_db_flags").changed() {
                        c.p ^= 1 << bit;
                    }
                }
            });
            ui.add(egui::Label::new(RichText::new(tf("db_cycles", &[&s.m.board.cycles, &s.dbg.steps])).weak()).truncate());
        });
    }

    /// « Désassemblage » window: always DIS_ROWS rows, from a start kept
    /// while PC stays inside (a key-wait loop does not move the list).
    pub fn disasm_contents(&mut self, ui: &mut egui::Ui, h: bool, s: &mut Session, font: &mut f32) -> bool {
        let mut changed = false;
        if *font < crate::editor::FONT_MIN {
            *font = crate::fontsz::DEFAULT;
        }
        set_mono(*font);
        let pc = s.m.cpu.pc;
        ui.horizontal_wrapped(|ui| {
            let r = ui.add(egui::TextEdit::singleline(&mut self.dis_goto).desired_width(90.0).hint_text("$E000, label")).hint(h, t("db_dis_goto_tip"), "h_db_dis_goto");
            let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if ui.button(t("db_goto")).hint(h, t("db_dis_goto_tip"), "h_db_dis_goto").clicked() || enter {
                match debug::parse_addr(&self.dis_goto, &self.syms) {
                    Some(a) => {
                        self.dis_top = Some(a);
                        self.dis_follow = false;
                        self.msg = None;
                    }
                    None => self.msg = Some(tf("db_bad_addr", &[&self.dis_goto])),
                }
            }
            if ui.checkbox(&mut self.dis_follow, t("db_follow")).hint(h, t("db_follow_tip"), "h_db_follow").changed() && self.dis_follow {
                self.dis_top = None;
            }
            if crate::fontsz::control(ui, "dis", font, h) {
                changed = true;
            }
        });
        if crate::fontsz::wheel(ui, ui.max_rect(), font) {
            changed = true;
            set_mono(*font);
        }
        ui.separator();
        // the selected source line is out of the list: show it (and stop following PC)
        if let Some(a) = self.jump_dis.take() {
            if !(self.dis_shown.0 <= a && a <= self.dis_shown.1) {
                let peek = |x: u16| s.m.board.peek(x);
                self.dis_top = Some(debug::disasm_window(&peek, a, 3, 0, &self.map).first().map(|l| l.addr).unwrap_or(a));
                self.dis_follow = false;
            }
        }
        let lines = {
            let peek = |a: u16| s.m.board.peek(a);
            let map = &self.map;
            let decode = |top: u16| {
                let mut a = top;
                (0..DIS_ROWS)
                    .map(|_| {
                        let l = debug::disasm(&peek, a, map);
                        a = a.wrapping_add(l.bytes.len() as u16);
                        l
                    })
                    .collect::<Vec<_>>()
            };
            let mut lines = self.dis_top.map(decode);
            let inside = |v: &Vec<debug::DisLine>| v[..DIS_ROWS - DIS_KEEP].iter().any(|l| l.addr == pc);
            if self.dis_follow && !lines.as_ref().is_some_and(inside) {
                // recentre: a few instructions before PC
                let top = debug::disasm_window(&peek, pc, 5, 0, map).first().map(|l| l.addr).unwrap_or(pc);
                self.dis_top = Some(top);
                lines = Some(decode(top));
            }
            lines.unwrap_or_else(|| decode(pc))
        };
        if let (Some(f), Some(l)) = (lines.first(), lines.last()) {
            self.dis_shown = (f.addr, l.addr.wrapping_add(l.bytes.len() as u16));
        }
        let mut toggle = None;
        let mut run_to = None;
        // narrow pane: the instruction matters most, the bytes column goes first (#15)
        let cw = ui.fonts_mut(|f| f.glyph_width(&FontId::monospace(mono_px()), '0'));
        let avail_chars = ((ui.available_width() - 24.0) / cw).floor();
        egui::ScrollArea::both().id_salt("db-desas").auto_shrink([false, false]).show(ui, |ui| {
            for l in lines {
                let on = s.dbg.breaks.contains(&l.addr);
                let here = l.addr == pc;
                let bytes: Vec<String> = l.bytes.iter().map(|b| format!("{b:02X}")).collect();
                // the label in its own column: no extra row
                let name: String = self.map.get(&l.addr).map(|n| format!("{n}:")).unwrap_or_default().chars().take(14).collect();
                let full = format!("{}{} {:04X}  {:<9} {:<14} {}", if on { "●" } else { " " }, if here { "▶" } else { " " }, l.addr, bytes.join(" "), name, l.text);
                let text = if full.chars().count() as f32 <= avail_chars {
                    full
                } else {
                    format!("{}{} {:04X}  {}", if on { "●" } else { " " }, if here { "▶" } else { " " }, l.addr, l.text)
                };
                let mut rt = mono(text);
                if here {
                    rt = rt.background_color(crate::editor::cur_bg()).color(pick(Color32::WHITE, Color32::from_rgb(0, 70, 0)));
                } else if self.in_hl(l.addr, l.bytes.len() as u16) {
                    rt = rt.background_color(crate::editor::hl_bg());
                } else if on {
                    rt = rt.color(BRK);
                } else if !name.is_empty() {
                    rt = rt.color(crate::uicolors::get(crate::uicolors::Role::Label));
                }
                let r = ui.add(egui::Label::new(rt).extend().sense(egui::Sense::click())).hint(h, t("db_disasm_tip"), "h_db_disasm");
                if r.clicked() {
                    toggle = Some(l.addr);
                }
                r.context_menu(|ui| {
                    if ui.button(t("db_runto_here")).clicked() {
                        run_to = Some(l.addr);
                        ui.close();
                    }
                    if ui.button(t("db_break_here")).clicked() {
                        toggle = Some(l.addr);
                        ui.close();
                    }
                });
            }
        });
        if let Some(a) = toggle {
            Self::toggle_break(s, a);
        }
        if let Some(a) = run_to {
            self.command(s, Cmd::To(a));
        }
        changed
    }

    fn breaks_ui(&mut self, ui: &mut egui::Ui, h: bool, s: &mut Session) {
        egui::CollapsingHeader::new(tf("db_breaks", &[&s.dbg.breaks.len()])).id_salt("db-arrets").default_open(true).show(ui, |ui| {
            ui.horizontal(|ui| {
                let r = ui.add(egui::TextEdit::singleline(&mut self.bp_in).desired_width(140.0).hint_text("$0300, label")).hint(h, t("db_break_in_tip"), "h_db_break_in");
                let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                if ui.button(t("db_add")).clicked() || enter {
                    let txt = std::mem::take(&mut self.bp_in);
                    if !self.add_break_text(s, &txt) {
                        self.msg = Some(tf("db_bad_addr", &[&txt]));
                    } else if self.pending.iter().all(|p| p != txt.trim()) {
                        self.msg = None;
                    }
                }
            });
            let mut del = None;
            for a in s.dbg.breaks.iter() {
                ui.horizontal(|ui| {
                    ui.label(mono(format!("● ${a:04X} {}", self.map.get(a).map(|n| n.as_str()).unwrap_or(""))).color(BRK));
                    if ui.small_button("×").hint(h, t("db_del_tip"), "h_db_del").clicked() {
                        del = Some(*a);
                    }
                });
            }
            if let Some(a) = del {
                s.dbg.breaks.remove(&a);
            }
        });
    }

    fn basic_ui(&mut self, ui: &mut egui::Ui, h: bool, s: &mut Session, cfg: &mut DbgCfg, basic: bool) -> bool {
        let mut changed = false;
        let basic_title = if apple1_core::g2basic::present(&|a| s.m.board.peek(a)) { "RDK G2 Basic" } else { t("db_basic") };
        egui::CollapsingHeader::new(basic_title).id_salt("db-basic").default_open(true).show(ui, |ui| {
            if !basic {
                ui.label(RichText::new(t("db_no_basic")).weak());
                return;
            }
            let peek = |a: u16| s.m.board.peek(a);
            let line = s.dbg.basic_line.or_else(|| debug::basic_current_line(&peek));
            ui.label(match line {
                Some(l) => tf("db_basic_line", &[&l]),
                None => t("db_basic_noline").into(),
            })
            .hint(h, "", "h_db_basic_line");
            // BASIC breakpoints
            ui.horizontal(|ui| {
                ui.label(t("db_basic_breaks"));
                let r = ui.add(egui::TextEdit::singleline(&mut self.bbp_in).desired_width(70.0).hint_text("190")).hint(h, t("db_basic_break_tip"), "h_db_basic_break");
                let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                if ui.button(t("db_add")).clicked() || enter {
                    let txt = std::mem::take(&mut self.bbp_in);
                    match txt.trim().parse::<u16>() {
                        Ok(n) => {
                            s.dbg.basic_breaks.insert(n);
                            self.msg = None;
                        }
                        Err(_) => self.msg = Some(tf("db_bad_line", &[&txt])),
                    }
                }
            });
            let mut del = None;
            ui.horizontal_wrapped(|ui| {
                for l in s.dbg.basic_breaks.iter() {
                    ui.label(mono(format!("● {l}")).color(BRK));
                    if ui.small_button("×").hint(h, t("db_del_tip"), "h_db_del").clicked() {
                        del = Some(*l);
                    }
                }
            });
            if let Some(l) = del {
                s.dbg.basic_breaks.remove(&l);
            }
            // variables
            let vars = debug::basic_vars(&|a| s.m.board.peek(a));
            ui.horizontal(|ui| {
                ui.label(t("db_vars"));
                let r = ui.add(egui::TextEdit::singleline(&mut self.var_in).desired_width(90.0).hint_text("A, C$, D(2)")).hint(h, t("db_var_in_tip"), "h_db_var_in");
                let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                if (ui.button(t("db_add")).clicked() || enter) && !self.var_in.trim().is_empty() {
                    cfg.vars.push(std::mem::take(&mut self.var_in).trim().to_ascii_uppercase());
                    changed = true;
                }
            });
            let mut del = None;
            egui::Grid::new("db-vars").num_columns(3).striped(true).show(ui, |ui| {
                for (i, v) in cfg.vars.iter().enumerate() {
                    ui.label(mono(v));
                    ui.label(mono(debug::basic_watch(&vars, v).unwrap_or_else(|| "?".into())));
                    if ui.small_button("×").hint(h, t("db_del_tip"), "h_db_del").clicked() {
                        del = Some(i);
                    }
                    ui.end_row();
                }
            });
            if let Some(i) = del {
                cfg.vars.remove(i);
                changed = true;
            }
            egui::CollapsingHeader::new(tf("db_all_vars", &[&vars.len()])).id_salt("db-allvars").show(ui, |ui| {
                for v in &vars {
                    let val = match &v.value {
                        VarValue::Str(x, n) => format!("\"{x}\"  (DIM {n})"),
                        VarValue::Int(l) if l.len() == 1 => l[0].to_string(),
                        VarValue::Int(_) => debug::basic_watch(&vars, &v.name).unwrap_or_default(),
                    };
                    ui.label(mono(format!("{:<4} ${:04X}  {val}", v.name, v.addr)));
                }
            })
            .header_response
            .hint(h, "", "h_db_all_vars");
        });
        changed
    }

    fn mems_ui(&mut self, ui: &mut egui::Ui, h: bool, s: &mut Session, cfg: &mut DbgCfg) -> bool {
        let mut changed = false;
        egui::CollapsingHeader::new(t("db_mems")).id_salt("db-mem").default_open(true).show(ui, |ui| {
            ui.horizontal(|ui| {
                let r = ui.add(egui::TextEdit::singleline(&mut self.mem_in).desired_width(150.0).hint_text("$30, 0200.020F, $4A w")).hint(h, t("db_mem_in_tip"), "h_db_mem_in");
                let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                if ui.button(t("db_add")).clicked() || enter {
                    let txt = std::mem::take(&mut self.mem_in);
                    match parse_mem(&txt, &self.syms) {
                        Some(w) => {
                            cfg.mems.push(w);
                            changed = true;
                            self.msg = None;
                        }
                        None => self.msg = Some(tf("db_bad_addr", &[&txt])),
                    }
                }
            });
            let mut del = None;
            for (i, w) in cfg.mems.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    let name = self.map.get(&w.addr).map(|n| format!(" {n}")).unwrap_or_default();
                    ui.label(mono(format!("${:04X}{name}", w.addr)));
                    let before = w.fmt;
                    egui::ComboBox::from_id_salt(("db-fmt", i))
                        .width(70.0)
                        .selected_text(t(fmt_key(w.fmt)))
                        .show_ui(ui, |ui| {
                            for f in [Fmt::Byte, Fmt::Word, Fmt::Ascii] {
                                ui.selectable_value(&mut w.fmt, f, t(fmt_key(f)));
                            }
                        })
                        .response
                        .hint(h, t("db_fmt_tip"), "h_db_fmt");
                    if w.fmt != before {
                        changed = true;
                        if w.fmt == Fmt::Word && w.len < 2 {
                            w.len = 2;
                        }
                    }
                    let a = w.addr;
                    let peek = |x: u16| s.m.board.peek(x);
                    let was = |x: u16| self.base.get(x as usize).copied().unwrap_or(0);
                    let hot = (0..w.len).any(|k| peek(a.wrapping_add(k)) != was(a.wrapping_add(k)));
                    match (w.fmt, w.len) {
                        (Fmt::Byte, 1) => {
                            let mut v = peek(a);
                            let r = ui.add(egui::DragValue::new(&mut v).hexadecimal(2, false, true).prefix("$")).hint(h, t("db_edit_tip"), "h_db_edit");
                            if r.changed() {
                                s.m.board.poke(a, v);
                            }
                            ui.label(mono(format!("{v}")).color(if hot { chg_col() } else { dim() }));
                        }
                        (Fmt::Word, 2) => {
                            let mut v = peek(a) as u16 | (peek(a.wrapping_add(1)) as u16) << 8;
                            let r = ui.add(egui::DragValue::new(&mut v).hexadecimal(4, false, true).prefix("$")).hint(h, t("db_edit_tip"), "h_db_edit");
                            if r.changed() {
                                s.m.board.poke(a, v as u8);
                                s.m.board.poke(a.wrapping_add(1), (v >> 8) as u8);
                            }
                            ui.label(mono(format!("{v}")).color(if hot { chg_col() } else { dim() }));
                        }
                        (Fmt::Ascii, n) => {
                            let txt: String = (0..n).map(|k| ascii(peek(a.wrapping_add(k)))).collect();
                            ui.label(mono(format!("\"{txt}\"")).color(if hot { chg_col() } else { text_col() }));
                        }
                        (f, n) => {
                            let step = if f == Fmt::Word { 2 } else { 1 };
                            let mut parts = Vec::new();
                            let mut k = 0;
                            while k < n {
                                let x = a.wrapping_add(k);
                                parts.push(if step == 2 { format!("{:04X}", peek(x) as u16 | (peek(x.wrapping_add(1)) as u16) << 8) } else { format!("{:02X}", peek(x)) });
                                k += step;
                            }
                            ui.label(mono(parts.join(" ")).color(if hot { chg_col() } else { text_col() }));
                        }
                    }
                    if ui.small_button("×").hint(h, t("db_del_tip"), "h_db_del").clicked() {
                        del = Some(i);
                    }
                });
            }
            if let Some(i) = del {
                cfg.mems.remove(i);
                changed = true;
            }
        });
        changed
    }

    /// « Dump hexa/ASCII » window.
    pub fn dump_contents(&mut self, ui: &mut egui::Ui, h: bool, s: &mut Session, cfg: &mut DbgCfg) -> bool {
        let mut changed = false;
        if cfg.dump_win.font < crate::editor::FONT_MIN {
            cfg.dump_win.font = crate::fontsz::DEFAULT;
        }
        set_mono(cfg.dump_win.font);
        {
            ui.horizontal_wrapped(|ui| {
                let r = ui.add(egui::TextEdit::singleline(&mut self.goto_in).desired_width(80.0).hint_text("$0200")).hint(h, t("db_goto_tip"), "h_db_goto");
                let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                let mut go = None;
                if ui.button(t("db_goto")).clicked() || enter {
                    match debug::parse_addr(&self.goto_in, &self.syms) {
                        Some(a) => go = Some(a),
                        None => self.msg = Some(tf("db_bad_addr", &[&self.goto_in])),
                    }
                }
                for (l, a, tip, hid) in [
                    ("ZP", 0x0000u16, "db_jump_zp_tip", "h_db_jump_zp"),
                    ("$0200", 0x0200, "db_jump_0200_tip", "h_db_jump_0200"),
                    ("$0300", 0x0300, "db_jump_0300_tip", "h_db_jump_0300"),
                    ("PC", s.m.cpu.pc, "db_jump_pc_tip", "h_db_jump_pc"),
                    ("$E000", 0xE000, "db_jump_e000_tip", "h_db_jump_e000"),
                ] {
                    if ui.small_button(l).hint(h, t(tip), hid).clicked() {
                        go = Some(a);
                    }
                }
                if let Some(a) = go {
                    cfg.dump = a & 0xFFF0;
                    self.dump_goto = true;
                    changed = true;
                }
                if crate::fontsz::control(ui, "dump", &mut cfg.dump_win.font, h) {
                    changed = true;
                    set_mono(cfg.dump_win.font);
                }
            });
            if crate::fontsz::wheel(ui, ui.max_rect(), &mut cfg.dump_win.font) {
                changed = true;
                set_mono(cfg.dump_win.font);
            }
            ui.horizontal_wrapped(|ui| {
                ui.label(t("db_poke"));
                ui.add(egui::TextEdit::singleline(&mut self.poke_addr).desired_width(60.0).hint_text("$0300")).hint(h, t("db_poke_tip"), "h_db_poke");
                ui.label("=");
                ui.add(egui::TextEdit::singleline(&mut self.poke_val).desired_width(90.0).hint_text("A9 00")).hint(h, t("db_poke_tip"), "h_db_poke");
                if ui.button(t("db_write")).hint(h, t("db_poke_tip"), "h_db_poke").clicked() {
                    self.msg = self.poke(s).err();
                }
            });
            let row_h = ui.fonts_mut(|f| f.row_height(&FontId::monospace(mono_px()))) + ui.spacing().item_spacing.y;
            ui.add(egui::Label::new(RichText::new(self.msg.as_deref().unwrap_or(" ")).color(err_col())).truncate());
            // 16 bytes a row, or 8 when the pane is narrow
            let cw = ui.fonts_mut(|f| f.glyph_width(&FontId::monospace(mono_px()), '0'));
            let per: u16 = if ui.available_width() >= 74.0 * cw + 16.0 { 16 } else { 8 };
            if per != self.dump_per {
                self.dump_per = per;
                self.dump_goto = true;
            }
            let rows = 0x10000 / per as usize;
            if let Some(a) = self.jump_dump.take() {
                if a < cfg.dump || a >= cfg.dump.saturating_add(per * 6) {
                    cfg.dump = (a & 0xFFF0).saturating_sub(0x10);
                    self.dump_goto = true;
                    changed = true;
                }
            }
            let mut area = egui::ScrollArea::both().id_salt("db-dump-rows").auto_shrink([false, false]);
            if std::mem::take(&mut self.dump_goto) {
                area = area.vertical_scroll_offset((cfg.dump / per) as f32 * row_h);
            }
            let pc = s.m.cpu.pc;
            let mut clicked = None;
            let (mut typed_hex, mut typed_asc): (Option<u16>, Option<u16>) = (None, None);
            let out = area.show_rows(ui, row_h, rows, |ui, range| {
                for row in range {
                    let base = row as u16 * per;
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label(mono(format!("{base:04X}: ")).color(addr_col()));
                        for k in 0..per {
                            let a = base + k;
                            let v = s.m.board.peek(a);
                            let hot = self.base.get(a as usize).map(|b| *b != v).unwrap_or(false);
                            let id = egui::Id::new(("dump-hex", a));
                            let focus = ui.memory(|m| m.has_focus(id));
                            let half = self.dump_half.filter(|(x, _)| *x == a && focus);
                            let shown = match half {
                                Some((_, n)) => format!("{n:X}_"),
                                None => format!("{v:02X}"),
                            };
                            let mut rt = mono(format!("{shown}{}", if k == 7 { "  " } else { " " }));
                            if focus {
                                rt = rt.background_color(crate::editor::sel_bg());
                            } else if hot {
                                rt = rt.color(chg_col()).strong();
                            } else if a == pc {
                                rt = rt.background_color(crate::editor::cur_bg());
                            } else if self.in_hl(a, 1) {
                                rt = rt.background_color(crate::editor::hl_bg());
                            }
                            let r = ui.add(egui::Label::new(rt));
                            let r = ui.interact(r.rect, id, egui::Sense::click());
                            if r.hint(h, t("db_byte_tip"), "h_db_dump").clicked() {
                                ui.memory_mut(|m| m.request_focus(id));
                                clicked = Some(a);
                            }
                            if focus {
                                typed_hex = Some(a);
                            }
                        }
                        for k in 0..per {
                            let a = base + k;
                            let v = s.m.board.peek(a);
                            let id = egui::Id::new(("dump-asc", a));
                            let focus = ui.memory(|m| m.has_focus(id));
                            let mut rt = mono(ascii(v).to_string()).color(text_col());
                            if focus {
                                rt = rt.background_color(crate::editor::sel_bg());
                            }
                            let r = ui.add(egui::Label::new(rt));
                            let r = ui.interact(r.rect, id, egui::Sense::click());
                            if r.hint(h, t("db_ascii_tip"), "h_db_ascii").clicked() {
                                ui.memory_mut(|m| m.request_focus(id));
                                clicked = Some(a);
                            }
                            if focus {
                                typed_asc = Some(a);
                            }
                        }
                    });
                }
            });
            let first = ((out.state.offset.y / row_h) as usize).min(rows - 1) as u16;
            let top = first * per;
            if top != cfg.dump && !self.dump_goto {
                cfg.dump = top;
            }
            // typing in the cell that has the focus: hex digits, or characters
            if typed_hex.is_some() || typed_asc.is_some() {
                let text: String = ui.input_mut(|i| {
                    let mut s = String::new();
                    i.events.retain(|e| match e {
                        egui::Event::Text(x) => {
                            s.push_str(x);
                            false
                        }
                        _ => true,
                    });
                    s
                });
                for c in text.chars() {
                    if let Some(a) = typed_hex {
                        let Some(n) = c.to_digit(16) else { continue };
                        match self.dump_half.take().filter(|(x, _)| *x == a) {
                            None => self.dump_half = Some((a, n as u8)),
                            Some((_, hi)) => {
                                self.dump_write(s, a, hi << 4 | n as u8);
                                ui.memory_mut(|m| m.request_focus(egui::Id::new(("dump-hex", a.wrapping_add(1)))));
                                typed_hex = Some(a.wrapping_add(1));
                            }
                        }
                    } else if let Some(a) = typed_asc {
                        if c.is_ascii() && !c.is_ascii_control() {
                            // the high bit is kept as it is (the Apple-1 sets it on text)
                            let old = s.m.board.peek(a);
                            self.dump_write(s, a, (old & 0x80) | c as u8);
                            ui.memory_mut(|m| m.request_focus(egui::Id::new(("dump-asc", a.wrapping_add(1)))));
                            typed_asc = Some(a.wrapping_add(1));
                        }
                    }
                }
            } else {
                self.dump_half = None;
            }
            if let Some(a) = clicked {
                self.dump_half = None;
                self.poke_addr = format!("${a:04X}");
                self.poke_val = format!("{:02X}", s.m.board.peek(a));
            }
        }
        changed
    }

    /// One byte typed in the dump.
    fn dump_write(&mut self, s: &mut Session, a: u16, v: u8) {
        self.msg = if s.m.board.poke(a, v) { None } else { Some(tf("db_not_ram", &[&format!("{a:04X}")])) };
    }

    /// « Écrire »: hex bytes from an address.
    fn poke(&mut self, s: &mut Session) -> Result<(), String> {
        let a = debug::parse_addr(&self.poke_addr, &self.syms).ok_or_else(|| tf("db_bad_addr", &[&self.poke_addr]))?;
        let bytes: Vec<u8> = self
            .poke_val
            .split([' ', ','])
            .filter(|x| !x.is_empty())
            .map(|x| u8::from_str_radix(x.trim_start_matches('$'), 16))
            .collect::<Result<_, _>>()
            .map_err(|_| tf("db_bad_bytes", &[&self.poke_val]))?;
        for (i, b) in bytes.iter().enumerate() {
            let x = a.wrapping_add(i as u16);
            if !s.m.board.poke(x, *b) {
                return Err(tf("db_not_ram", &[&format!("{x:04X}")]));
            }
        }
        Ok(())
    }
}

fn fmt_key(f: Fmt) -> &'static str {
    match f {
        Fmt::Byte => "db_fmt_byte",
        Fmt::Word => "db_fmt_word",
        Fmt::Ascii => "db_fmt_ascii",
    }
}

/// Character of a byte, high bit stripped (the Apple-1 sets it).
pub fn ascii(b: u8) -> char {
    let c = b & 0x7F;
    if (0x20..0x7F).contains(&c) {
        c as char
    } else {
        '.'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_watch_syntax() {
        let syms = vec![("score".to_string(), 0x30i64)];
        assert_eq!(parse_mem("$30", &[]), Some(MemWatch { addr: 0x30, len: 1, fmt: Fmt::Byte }));
        assert_eq!(parse_mem("0200.020F", &[]), Some(MemWatch { addr: 0x200, len: 16, fmt: Fmt::Byte }));
        assert_eq!(parse_mem("$4A w", &[]), Some(MemWatch { addr: 0x4A, len: 2, fmt: Fmt::Word }));
        assert_eq!(parse_mem("score:a", &syms), Some(MemWatch { addr: 0x30, len: 16, fmt: Fmt::Ascii }));
        assert_eq!(parse_mem("0300.0200", &[]), None);
        assert_eq!(parse_mem("zz", &[]), None);
        assert_eq!(ascii(0xC1), 'A');
        assert_eq!(ascii(0x8D), '.');
        assert_eq!(Cmd::parse("dessus"), Some(Cmd::Over));
    }
}
