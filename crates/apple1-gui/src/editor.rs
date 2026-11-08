//! « Éditeur » window: Integer BASIC listings and 6502 assembler sources in
//! tabs, coloured, tokenized (apple1_core::intbasic) or assembled
//! (apple1_core::asm6502) natively, to be run in the emulator or written to
//! an ACI tape.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use apple1_core::asm6502::{self, AsmError, Assembly, ListRow};
use apple1_core::intbasic::{self, Image, Listing, KEYWORDS};
use eframe::egui::{self, text::LayoutJob, Color32, FontId, RichText, TextFormat};
use serde::{Deserialize, Serialize};

use crate::app::Hint;
use crate::i18n::{t, tf};

pub const FONT_DEFAULT: f32 = 13.0;
pub const FONT_MIN: f32 = 8.0;
pub const FONT_MAX: f32 = 48.0;

/// Saved in gui.json.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct EditorCfg {
    pub open: bool,
    pub font: f32,
    /// Text size of the assembler listing (address, bytes, source), separate from the source.
    #[serde(default = "crate::fontsz::default_size")]
    pub listing_font: f32,
    /// Memory of the target Apple-1 in KB (None = automatic).
    pub ram: Option<u32>,
    pub tabs: Vec<TabCfg>,
    pub current: usize,
    /// Assembler tabs show the listing (address, bytes, source).
    pub listing: bool,
}

impl Default for EditorCfg {
    fn default() -> Self {
        EditorCfg { open: false, font: FONT_DEFAULT, listing_font: FONT_DEFAULT, ram: None, tabs: Vec::new(), current: 0, listing: false }
    }
}

/// A tab: its file, and its text when it is not saved.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TabCfg {
    pub path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Language of a tab without file (else: from its extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<SrcLang>,
    /// Opened from the library (📚 in its title).
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub lib: bool,
}

/// What a tab contains.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SrcLang {
    #[default]
    Basic,
    Asm,
}

/// Assembler sources: .s .asm .a65 .inc .ca65; everything else is BASIC.
pub fn lang_of(p: &Path) -> SrcLang {
    match p.extension().map(|e| e.to_string_lossy().to_lowercase()).as_deref() {
        Some("s" | "asm" | "a65" | "inc" | "ca65") => SrcLang::Asm,
        _ => SrcLang::Basic,
    }
}

pub type AsmResult = Result<Assembly, Vec<AsmError>>;

pub struct Tab {
    pub uid: u64,
    pub path: Option<PathBuf>,
    pub text: String,
    /// Text as last read or written (to know when it is modified).
    pub saved: String,
    pub lang: SrcLang,
    /// Opened from the library.
    pub from_lib: bool,
    cache: Option<(u64, Arc<Listing>)>,
    acache: Option<(u64, Arc<AsmResult>)>,
    rcache: Option<(u64, Arc<Vec<ListRow>>)>,
}

impl Tab {
    pub fn name(&self) -> String {
        match &self.path {
            Some(p) => p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(),
            None => t("ed_untitled").to_string(),
        }
    }
    pub fn modified(&self) -> bool {
        self.text != self.saved
    }
    fn key(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.text.hash(&mut h);
        self.path.hash(&mut h);
        h.finish()
    }
    /// The source assembled (includes read from the disk, next to the file).
    pub fn assembly(&mut self) -> Arc<AsmResult> {
        let k = self.key();
        match &self.acache {
            Some((hk, a)) if *hk == k => a.clone(),
            _ => {
                let a = Arc::new(asm6502::assemble_text(&self.text, self.path.as_deref(), &[]));
                self.acache = Some((k, a.clone()));
                a
            }
        }
    }
    /// Rows of the listing view.
    pub fn list_rows(&mut self) -> Arc<Vec<ListRow>> {
        let k = self.key();
        if let Some((hk, r)) = &self.rcache {
            if *hk == k {
                return r.clone();
            }
        }
        let r = Arc::new(match &*self.assembly() {
            Ok(a) => asm6502::listing_rows(a, Some(&self.text)),
            Err(_) => Vec::new(),
        });
        self.rcache = Some((k, r.clone()));
        r
    }
    pub fn listing(&mut self) -> Arc<Listing> {
        let mut h = DefaultHasher::new();
        self.text.hash(&mut h);
        let k = h.finish();
        match &self.cache {
            Some((hk, l)) if *hk == k => l.clone(),
            _ => {
                let l = Arc::new(intbasic::tokenize(&self.text));
                self.cache = Some((k, l.clone()));
                l
            }
        }
    }
}

/// What the window asks the application to do.
pub enum EdAction {
    Open,
    /// Save the tab (uid) under a new name.
    SaveAs(u64),
    Run,
    Export,
    Info(String),
    Fail(String),
    /// Debugger: toggle the breakpoint of a source line (1-based) of the
    /// current tab, or of an address (listing view).
    BreakLine(usize),
    BreakAddr(u16),
    /// Debugger: run to this source line.
    RunToLine(usize),
    /// Show the code of this source line in the disassembly.
    ShowDisasm(usize),
}

/// The search / replace bar.
#[derive(Default)]
pub struct Find {
    pub open: bool,
    pub query: String,
    pub repl: String,
    pub case: bool,
    pub word: bool,
    /// Current hit.
    pub cur: usize,
    focus_query: bool,
    focus_repl: bool,
}

/// What the debugger shows in the editor (current tab).
#[derive(Default, Clone, PartialEq)]
pub struct EdDbg {
    pub on: bool,
    /// Line about to run (1-based source line), PC for the listing view.
    pub cur: Option<usize>,
    pub pc: Option<u16>,
    /// Lines and addresses with a breakpoint.
    pub marks: Vec<usize>,
    pub addrs: Vec<u16>,
    /// Scroll to `cur` (it changed while paused).
    pub scroll: bool,
}

pub struct Editor {
    pub tabs: Vec<Tab>,
    pub cur: usize,
    pub font: f32,
    pub listing_font: f32,
    pub ram: Option<u32>,
    /// The text has the keyboard focus (Ctrl+plus/minus zoom the text).
    pub focused: bool,
    next_uid: u64,
    confirm_close: Option<u64>,
    goto_line: Option<usize>,
    /// Something to save in gui.json.
    pub changed: bool,
    /// Give the keyboard back to the Apple-1 (after « Lancer »).
    pub release: bool,
    pub show_listing: bool,
    /// Set by the debugger before `contents`.
    pub dbg: EdDbg,
    /// Line of the text cursor in the current tab (1-based).
    pub cursor_line: Option<usize>,
    pub find: Find,
    /// « Aller à la ligne » prompt.
    goto_text: Option<String>,
    /// Events given to the text (cut, copy, undo… from the buttons/menu).
    pending: Vec<egui::Event>,
    /// Give the text the focus (paste).
    focus_text: bool,
    /// Select these characters and show them.
    select: Option<(usize, usize)>,
    /// Selection of the current tab (characters, sorted).
    sel: (usize, usize),
    /// Renumbering: first number, step.
    renum: (u32, u32),
    /// Right-click menu shown (screenshots: --editeur-menu).
    pub force_menu: bool,
    /// Character under the last right click (the menu acts on that word).
    ctx_idx: Option<usize>,
    /// « Aide sur … » window.
    help: Option<crate::help::Entry>,
    /// Visible failure message (« Pas de définition… »), shown a few seconds.
    notice: Option<(String, std::time::Instant)>,
}

/// A tab ready to be put in memory or on tape.
pub enum Built {
    Basic(Image),
    Asm(Assembly),
}

impl Built {
    pub fn blocks(&self) -> Vec<(u16, Vec<u8>)> {
        match self {
            Built::Basic(i) => i.blocks(),
            Built::Asm(a) => vec![(a.origin, a.bytes.clone())],
        }
    }
    /// Ranges to type after C100R.
    pub fn aci(&self) -> String {
        match self {
            Built::Basic(i) => i.aci_command(),
            Built::Asm(a) => a.aci_range(),
        }
    }
    /// What starts it, as typed (lines separated by \r).
    pub fn run(&self) -> String {
        match self {
            Built::Basic(_) => "E2B3R\rRUN".into(),
            Built::Asm(a) => a.run_command(),
        }
    }
    /// First address above it (to size the RAM).
    pub fn top(&self) -> u32 {
        match self {
            Built::Basic(i) => i.himem as u32,
            Built::Asm(a) => a.end() as u32 + 1,
        }
    }
}

/// The current text of a tab, tokenized or assembled.
pub fn build(tab: &mut Tab, ram: Option<u32>) -> Result<Built, String> {
    match tab.lang {
        SrcLang::Basic => image_of(&tab.listing(), ram).map(Built::Basic),
        SrcLang::Asm => match &*tab.assembly() {
            Ok(a) if a.bytes.is_empty() => Err(t("as_empty").into()),
            Ok(a) => Ok(Built::Asm(a.clone())),
            Err(e) => Err(tf("ed_has_errors", &[&e.len(), &asm_error_text(&e[0])])),
        },
    }
}

/// Message of an assembler error, in the current language.
pub fn asm_error_text(e: &AsmError) -> String {
    let what = t(&format!("as_e_{}", e.kind.key()));
    let mut s = if e.file == 0 { tf("as_err_at", &[&e.line, &e.col, &what]) } else { tf("as_err_in", &[&e.file_name, &e.line, &e.col, &what]) };
    if !e.detail.is_empty() {
        s.push(' ');
        s.push_str(&tf("as_err_detail", &[&e.detail]));
    }
    s
}

/// Lowest-to-highest kinds of text, for the colours.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    LineNo,
    Keyword,
    Number,
    Str,
    Rem,
    Op,
    Name,
    Other,
    Label,
    Directive,
}

fn color(lang: SrcLang, k: Kind) -> Color32 {
    crate::syntax::color(lang, k)
}

/// Colour spans of one source line (byte ranges).
pub fn spans(line: &str) -> Vec<(Range<usize>, Kind)> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() && b[i] == b' ' {
        i += 1;
    }
    let s = i;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    if i > s {
        out.push((s..i, Kind::LineNo));
    }
    let mut kws: Vec<&str> = KEYWORDS.to_vec();
    kws.sort_by_key(|k| std::cmp::Reverse(k.len()));
    while i < b.len() {
        let c = b[i];
        let start = i;
        let kind = if c == b'"' {
            i += 1;
            while i < b.len() && b[i] != b'"' {
                i += 1;
            }
            i = (i + 1).min(b.len());
            Kind::Str
        } else if c.is_ascii_alphabetic() {
            match kws.iter().find(|k| b.len() - i >= k.len() && b[i..i + k.len()].eq_ignore_ascii_case(k.as_bytes())) {
                Some(&"REM") => {
                    out.push((i..i + 3, Kind::Keyword));
                    out.push((i + 3..b.len(), Kind::Rem));
                    break;
                }
                Some(k) => {
                    i += k.len();
                    Kind::Keyword
                }
                None => {
                    i += 1;
                    if i < b.len() && b[i].is_ascii_digit() {
                        i += 1;
                    }
                    if i < b.len() && b[i] == b'$' {
                        i += 1;
                    }
                    Kind::Name
                }
            }
        } else if c.is_ascii_digit() {
            while i < b.len() && b[i].is_ascii_digit() {
                i += 1;
            }
            Kind::Number
        } else if b"+-*/=#<>^(),;:".contains(&c) {
            i += 1;
            Kind::Op
        } else {
            // one character (UTF-8 aware)
            i += line[i..].chars().next().map(|ch| ch.len_utf8()).unwrap_or(1);
            Kind::Other
        };
        out.push((start..i, kind));
    }
    out
}

/// Colour spans of one line of 6502 assembler (ca65 syntax).
pub fn spans_asm(line: &str) -> Vec<(Range<usize>, Kind)> {
    let b = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    // is the next word the first of the statement (label or instruction)?
    let mut first = true;
    let word = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    while i < b.len() {
        let c = b[i];
        let start = i;
        let kind = if c == b';' {
            i = b.len();
            Kind::Rem
        } else if c == b' ' || c == b'\t' {
            i += 1;
            continue;
        } else if c == b'"' || c == b'\'' {
            i += 1;
            while i < b.len() && b[i] != c {
                i += 1;
            }
            i = (i + 1).min(b.len());
            Kind::Str
        } else if c == b'$' || c == b'%' || c.is_ascii_digit() {
            i += 1;
            while i < b.len() && b[i].is_ascii_alphanumeric() {
                i += 1;
            }
            if i == start + 1 && !c.is_ascii_digit() {
                Kind::Op
            } else {
                Kind::Number
            }
        } else if c == b'.' || c == b'@' || c.is_ascii_alphabetic() || c == b'_' {
            i += 1;
            while i < b.len() && word(b[i]) {
                i += 1;
            }
            let w = &line[start..i];
            let mut j = i;
            while j < b.len() && (b[j] == b' ' || b[j] == b'\t') {
                j += 1;
            }
            let k = if i < b.len() && b[i] == b':' && first && !w.starts_with('.') {
                i += 1;
                Kind::Label
            } else if first && j < b.len() && b[j] == b'=' || (first && b.get(j..j + 2) == Some(b":=")) {
                Kind::Label
            } else if w.starts_with('.') {
                first = false;
                Kind::Directive
            } else if first && asm6502::is_mnemonic(w) {
                first = false;
                Kind::Keyword
            } else {
                first = false;
                Kind::Name
            };
            k
        } else if c == b':' {
            i += 1;
            if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
                let sg = b[i];
                while i < b.len() && b[i] == sg {
                    i += 1;
                }
                Kind::Name
            } else {
                Kind::Label
            }
        } else if b"+-*/=#<>^(),&|~!".contains(&c) {
            i += 1;
            Kind::Op
        } else {
            i += line[i..].chars().next().map(|ch| ch.len_utf8()).unwrap_or(1);
            Kind::Other
        };
        out.push((start..i, kind));
    }
    out
}

/// The coloured layout of the whole text; `bad` = source lines (1 = first)
/// with an error, shown on a red background.
#[cfg(test)]
pub fn highlight(text: &str, font: f32, bad: &[usize]) -> LayoutJob {
    highlight_as(text, font, bad, SrcLang::Basic, &EdDbg::default())
}

/// Background of the line being run (debugger), of a line with a
/// breakpoint, of a line with an error (dark or light theme).
/// A text colour for the dark theme, or its darker twin for the light one.
fn tone(dark: (u8, u8, u8), light: (u8, u8, u8)) -> Color32 {
    let c = if crate::syntax::dark() { dark } else { light };
    Color32::from_rgb(c.0, c.1, c.2)
}
pub fn cur_bg() -> Color32 {
    crate::uicolors::get(crate::uicolors::Role::CurLine)
}
/// Bytes of the source line(s) selected in the editor, in Disassembly and Hex dump.
pub fn hl_bg() -> Color32 {
    crate::uicolors::get(crate::uicolors::Role::HlLine)
}
/// The cell being edited in the dump.
pub fn sel_bg() -> Color32 {
    crate::uicolors::get(crate::uicolors::Role::SelCell)
}
fn brk_bg() -> Color32 {
    crate::uicolors::get(crate::uicolors::Role::BrkLine)
}
fn bad_bg() -> Color32 {
    if crate::syntax::dark() { Color32::from_rgb(85, 22, 22) } else { Color32::from_rgb(255, 215, 215) }
}

pub fn highlight_as(text: &str, font: f32, bad: &[usize], lang: SrcLang, dbg: &EdDbg) -> LayoutJob {
    let mut job = LayoutJob::default();
    let fid = FontId::monospace(font);
    let bad_bg = bad_bg();
    for (n, line) in text.split_inclusive('\n').enumerate() {
        let body = line.strip_suffix('\n').unwrap_or(line);
        let body = body.strip_suffix('\r').unwrap_or(body);
        let bg = if bad.contains(&(n + 1)) {
            bad_bg
        } else if dbg.on && dbg.cur == Some(n + 1) {
            cur_bg()
        } else if dbg.on && dbg.marks.contains(&(n + 1)) {
            brk_bg()
        } else {
            Color32::TRANSPARENT
        };
        let mut at = 0;
        let sp = if lang == SrcLang::Asm { spans_asm(body) } else { spans(body) };
        for (r, k) in sp {
            if r.start > at {
                job.append(&body[at..r.start], 0.0, TextFormat { font_id: fid.clone(), color: color(lang, Kind::Other), background: bg, ..Default::default() });
            }
            let mut f = TextFormat { font_id: fid.clone(), color: color(lang, k), background: bg, ..Default::default() };
            if k == Kind::Rem {
                f.italics = true;
            }
            job.append(&body[r.clone()], 0.0, f);
            at = r.end;
        }
        let rest = &line[at..];
        if !rest.is_empty() {
            job.append(rest, 0.0, TextFormat { font_id: fid.clone(), color: color(lang, Kind::Other), background: bg, ..Default::default() });
        }
    }
    job
}

/// Search hits shown on a highlighted background (`cur`: the current one).
fn mark_hits(job: &mut LayoutJob, hits: &[Range<usize>], cur: Option<usize>) {
    if hits.is_empty() {
        return;
    }
    let dark = crate::syntax::dark();
    let (hit, now) = if dark { (Color32::from_rgb(110, 85, 20), Color32::from_rgb(200, 140, 20)) } else { (Color32::from_rgb(255, 230, 140), Color32::from_rgb(255, 170, 60)) };
    let mut out = Vec::new();
    for sec in std::mem::take(&mut job.sections) {
        let (mut a, end): (usize, usize) = (sec.byte_range.start.into(), sec.byte_range.end.into());
        let mut first = true;
        while a < end {
            let (b, which) = match hits.iter().enumerate().find(|(_, h)| h.end > a && h.start < end) {
                Some((i, h)) if h.start <= a => (h.end.min(end), Some(i)),
                Some((_, h)) => (h.start, None),
                None => (end, None),
            };
            let mut f = sec.format.clone();
            if let Some(i) = which {
                f.background = if Some(i) == cur { now } else { hit };
                f.color = if dark { Color32::WHITE } else { Color32::BLACK };
            }
            out.push(egui::text::LayoutSection { leading_space: if first { sec.leading_space } else { 0.0 }, byte_range: a.into()..b.into(), format: f });
            first = false;
            a = b;
        }
    }
    job.sections = out;
}

/// Byte ranges of `q` in `text`.
pub fn find_all(text: &str, q: &str, case: bool, word: bool) -> Vec<Range<usize>> {
    if q.is_empty() {
        return Vec::new();
    }
    let (hay, needle) = if case { (text.to_string(), q.to_string()) } else { (text.to_ascii_lowercase(), q.to_ascii_lowercase()) };
    let is_w = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == '_');
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(i) = hay[from..].find(&needle) {
        let (a, b) = (from + i, from + i + needle.len());
        if !word || (!is_w(text[..a].chars().last()) && !is_w(text[b..].chars().next())) {
            out.push(a..b);
        }
        from = b.max(a + 1);
        while from < hay.len() && !hay.is_char_boundary(from) {
            from += 1;
        }
    }
    out
}

fn char_of(text: &str, byte: usize) -> usize {
    text[..byte.min(text.len())].chars().count()
}

/// Text of the X11/Wayland primary selection (what was last selected in any
/// program), None elsewhere or when empty.
#[cfg(target_os = "linux")]
fn primary_selection() -> Option<String> {
    use arboard::{GetExtLinux, LinuxClipboardKind};
    let t = arboard::Clipboard::new().ok()?.get().clipboard(LinuxClipboardKind::Primary).text().ok()?;
    (!t.is_empty()).then_some(t)
}

#[cfg(not(target_os = "linux"))]
fn primary_selection() -> Option<String> {
    None
}

fn byte_of(text: &str, ch: usize) -> usize {
    text.char_indices().nth(ch).map(|(i, _)| i).unwrap_or(text.len())
}

/// Lines `l0..=l1` (0-based) commented, or uncommented when they all are
/// (BASIC: REM after the line number; assembler: « ; »).
pub fn toggle_comment(text: &str, l0: usize, l1: usize, lang: SrcLang) -> String {
    let lines: Vec<&str> = text.split('\n').collect();
    let split = |l: &str| -> (usize, usize) {
        // (end of the line number, start of the statement)
        let n = l.len() - l.trim_start().len();
        let d = n + l[n..].bytes().take_while(|c| c.is_ascii_digit()).count();
        let st = d + l[d..].len() - l[d..].trim_start().len();
        (d, st)
    };
    let commented = |l: &str| match lang {
        SrcLang::Asm => l.trim_start().starts_with(';'),
        SrcLang::Basic => {
            let (_, st) = split(l);
            l[st..].to_ascii_uppercase().starts_with("REM")
        }
    };
    let range = l0..=l1.min(lines.len().saturating_sub(1));
    let all = lines[range.clone()].iter().filter(|l| !l.trim().is_empty()).all(|l| commented(l));
    let mut out: Vec<String> = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        if !range.contains(&i) || l.trim().is_empty() {
            out.push(l.to_string());
            continue;
        }
        out.push(match (lang, all) {
            (SrcLang::Asm, true) => {
                let p = l.find(';').unwrap();
                let rest = &l[p + 1..];
                format!("{}{}", &l[..p], rest.strip_prefix(' ').unwrap_or(rest))
            }
            (SrcLang::Asm, false) => format!("; {l}"),
            (SrcLang::Basic, true) => {
                let (_, st) = split(l);
                let rest = &l[st + 3..];
                format!("{}{}", &l[..st], rest.strip_prefix(' ').unwrap_or(rest))
            }
            (SrcLang::Basic, false) => {
                let (_, st) = split(l);
                format!("{}REM {}", &l[..st], &l[st..])
            }
        });
    }
    out.join("\n")
}

/// Lines `l0..=l1` indented (8 spaces, the assembler's column) or not.
pub fn indent(text: &str, l0: usize, l1: usize, more: bool) -> String {
    text.split('\n')
        .enumerate()
        .map(|(i, l)| {
            if i < l0 || i > l1 || l.is_empty() {
                l.to_string()
            } else if more {
                format!("        {l}")
            } else if let Some(r) = l.strip_prefix('\t') {
                r.to_string()
            } else {
                l[(l.len() - l.trim_start_matches(' ').len()).min(8)..].to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// BASIC lines numbered again from `start` by `step`; the targets of GOTO,
/// GOSUB and THEN follow. Returns the text and the number of lines.
pub fn renumber(text: &str, start: u32, step: u32) -> (String, usize) {
    let mut map = std::collections::HashMap::new();
    let mut n = start;
    for l in text.lines() {
        let t = l.trim_start();
        let d: String = t.chars().take_while(|c| c.is_ascii_digit()).collect();
        if let Ok(v) = d.parse::<u32>() {
            map.entry(v).or_insert(n);
            n += step.max(1);
        }
    }
    let mut out = Vec::new();
    for l in text.split('\n') {
        let sp = spans(l);
        let mut s = String::new();
        let mut at = 0;
        let mut prev: Option<String> = None;
        for (r, k) in sp {
            let w = &l[r.clone()];
            let new = match k {
                Kind::LineNo => w.parse::<u32>().ok().and_then(|v| map.get(&v)),
                Kind::Number if prev.as_deref().is_some_and(|p| ["GOTO", "GOSUB", "THEN"].contains(&p)) => w.parse::<u32>().ok().and_then(|v| map.get(&v)),
                _ => None,
            };
            if let Some(v) = new {
                s.push_str(&l[at..r.start]);
                s.push_str(&v.to_string());
                at = r.end;
            }
            if !w.trim().is_empty() {
                prev = Some(if k == Kind::Keyword { w.to_ascii_uppercase() } else { String::new() });
            }
        }
        s.push_str(&l[at..]);
        out.push(s);
    }
    (out.join("\n"), map.len())
}

/// The word at a character of the text (labels: with . and @).
fn word_at(text: &str, ch: usize) -> Option<String> {
    let cs: Vec<char> = text.chars().collect();
    let w = |c: char| c.is_alphanumeric() || c == '_' || c == '@' || c == '.';
    let mut a = ch.min(cs.len());
    while a > 0 && w(cs[a - 1]) {
        a -= 1;
    }
    let mut b = ch.min(cs.len());
    while b < cs.len() && w(cs[b]) {
        b += 1;
    }
    (b > a).then(|| cs[a..b].iter().collect())
}

/// Line (1-based) that defines `word`: a label (assembler) or a BASIC line
/// number.
pub fn definition(text: &str, word: &str, lang: SrcLang) -> Option<usize> {
    text.lines().position(|l| match lang {
        SrcLang::Basic => l.trim_start().split(|c: char| !c.is_ascii_digit()).next() == Some(word) && word.chars().all(|c| c.is_ascii_digit()),
        SrcLang::Asm => {
            let t = l.trim_start();
            t.strip_prefix(word).is_some_and(|r| r.starts_with(':') || r.trim_start().starts_with('=') || (r.is_empty() && !l.starts_with([' ', '\t'])))
        }
    })
    .map(|i| i + 1)
}

/// Message of an error, in the current language.
pub fn error_text(e: &intbasic::TokError) -> String {
    let what = t(&format!("ed_e_{}", e.kind.key()));
    let num = e.number.map(|n| n.to_string()).unwrap_or_else(|| "—".into());
    let mut s = tf("ed_err_at", &[&e.src_line, &num, &(e.col + 1), &what]);
    if !e.near.is_empty() {
        s.push(' ');
        s.push_str(&tf("ed_err_near", &[&e.near]));
    }
    s
}

/// The program in memory, laid out for `ram` (None = automatic).
pub fn image_of(l: &Listing, ram: Option<u32>) -> Result<Image, String> {
    if let Some(e) = l.errors.first() {
        return Err(tf("ed_has_errors", &[&l.errors.len(), &error_text(e)]));
    }
    if l.lines.is_empty() {
        return Err(t("ed_empty").into());
    }
    let (lo, hi) = intbasic::layout(l.size(), ram);
    Image::build(&l.lines, lo, hi).map_err(|_| tf("ed_too_big", &[&l.size(), &(hi - lo)]))
}

fn read_text(p: &Path) -> Result<String, String> {
    let b = std::fs::read(p).map_err(|e| format!("{} : {e}", p.display()))?;
    Ok(String::from_utf8_lossy(&b).replace("\r\n", "\n").replace('\r', "\n"))
}

impl Editor {
    pub fn from_cfg(c: &EditorCfg) -> Editor {
        let mut e = Editor {
            tabs: Vec::new(),
            cur: 0,
            font: c.font.clamp(FONT_MIN, FONT_MAX),
            listing_font: if c.listing_font < FONT_MIN { FONT_DEFAULT } else { c.listing_font.clamp(FONT_MIN, FONT_MAX) },
            ram: c.ram,
            focused: false,
            next_uid: 1,
            confirm_close: None,
            goto_line: None,
            changed: false,
            release: false,
            show_listing: c.listing,
            dbg: EdDbg::default(),
            cursor_line: None,
            find: Find::default(),
            goto_text: None,
            pending: Vec::new(),
            focus_text: false,
            select: None,
            sel: (0, 0),
            renum: (10, 10),
            force_menu: false,
            ctx_idx: None,
            help: None,
            notice: None,
        };
        for tc in &c.tabs {
            let saved = tc.path.as_deref().and_then(|p| read_text(p).ok());
            let lang = tc.path.as_deref().map(lang_of).or(tc.lang).unwrap_or_default();
            match (&tc.text, saved) {
                (Some(text), s) => e.push(tc.path.clone(), text.clone(), s.unwrap_or_default(), lang),
                (None, Some(s)) => e.push(tc.path.clone(), s.clone(), s, lang),
                (None, None) => continue,
            }
            if let Some(t) = e.tabs.last_mut() {
                t.from_lib = tc.lib;
            }
        }
        if e.tabs.is_empty() {
            e.push(None, String::new(), String::new(), SrcLang::Basic);
        }
        e.cur = c.current.min(e.tabs.len() - 1);
        e
    }

    pub fn to_cfg(&self, open: bool) -> EditorCfg {
        EditorCfg {
            open,
            font: self.font,
            listing_font: self.listing_font,
            ram: self.ram,
            current: self.cur,
            listing: self.show_listing,
            tabs: self
                .tabs
                .iter()
                .filter(|t| t.path.is_some() || !t.text.is_empty())
                .map(|t| TabCfg { path: t.path.clone(), text: (t.path.is_none() || t.modified()).then(|| t.text.clone()), lang: t.path.is_none().then_some(t.lang), lib: t.from_lib })
                .collect(),
        }
    }

    fn push(&mut self, path: Option<PathBuf>, text: String, saved: String, lang: SrcLang) {
        self.tabs.push(Tab { uid: self.next_uid, path, text, saved, lang, from_lib: false, cache: None, acache: None, rcache: None });
        self.next_uid += 1;
    }

    /// First and last source line (1-based) of the selection, or of the
    /// cursor line when nothing is selected.
    pub fn src_lines(&self) -> Option<(usize, usize)> {
        let tab = self.tabs.get(self.cur)?;
        let line_of = |idx: usize| 1 + tab.text.chars().take(idx).filter(|c| *c == '\n').count();
        let (a, mut b) = self.sel;
        // a selection ending at the start of a line does not include it
        if b > a && tab.text.chars().nth(b - 1) == Some('\n') {
            b -= 1;
        }
        Some((line_of(a), line_of(b)))
    }

    pub fn current(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.cur)
    }

    pub fn tab(&mut self, uid: u64) -> Option<&mut Tab> {
        self.tabs.iter_mut().find(|t| t.uid == uid)
    }

    /// A new tab holding `text` (not saved: « sans titre »).
    pub fn new_tab_with(&mut self, text: String, lang: SrcLang) -> u64 {
        if let Some(i) = self.tabs.iter().position(|t| t.path.is_none() && t.text.is_empty()) {
            self.tabs.remove(i);
        }
        self.push(None, text, String::new(), lang);
        self.cur = self.tabs.len() - 1;
        self.changed = true;
        self.tabs[self.cur].uid
    }

    pub fn new_tab(&mut self, lang: SrcLang) {
        self.push(None, String::new(), String::new(), lang);
        self.cur = self.tabs.len() - 1;
        self.changed = true;
    }

    /// Open a file at a line (for an error in an included file).
    pub fn open_at(&mut self, p: &Path, line: usize) -> Result<String, String> {
        let r = self.open(p)?;
        self.goto_line = Some(line);
        Ok(r)
    }

    /// Open a file (or show its tab when it is already open).
    pub fn open(&mut self, p: &Path) -> Result<String, String> {
        if let Some(i) = self.tabs.iter().position(|t| t.path.as_deref() == Some(p)) {
            self.cur = i;
            return Ok(tf("ed_opened", &[&p.display()]));
        }
        let text = read_text(p)?;
        // replace an empty, untouched « sans titre » tab
        if let Some(i) = self.tabs.iter().position(|t| t.path.is_none() && t.text.is_empty()) {
            self.tabs.remove(i);
        }
        self.push(Some(p.to_path_buf()), text.clone(), text, lang_of(p));
        self.cur = self.tabs.len() - 1;
        self.changed = true;
        Ok(tf("ed_opened", &[&p.display()]))
    }

    /// Open a file of the library (📚 in its title), or show its tab.
    pub fn open_lib(&mut self, p: &Path) -> Result<String, String> {
        let r = self.open(p)?;
        if let Some(t) = self.tabs.get_mut(self.cur) {
            if !t.from_lib {
                t.from_lib = true;
                self.changed = true;
            }
        }
        Ok(r)
    }

    /// Write the tab to `path` (None = its own file).
    pub fn save(&mut self, uid: u64, path: Option<PathBuf>) -> Result<String, String> {
        let tab = self.tab(uid).ok_or("?")?;
        let p = path.or(tab.path.clone()).ok_or("?")?;
        let mut text = tab.text.clone();
        if !text.is_empty() && !text.ends_with('\n') {
            text.push('\n');
        }
        std::fs::write(&p, &text).map_err(|e| format!("{} : {e}", p.display()))?;
        tab.saved = tab.text.clone();
        tab.path = Some(p.clone());
        tab.lang = lang_of(&p);
        // another tab on the same file would be a duplicate of this one (#41)
        let cur_uid = self.tabs.get(self.cur).map(|t| t.uid);
        self.tabs.retain(|t| t.uid == uid || t.path.as_deref() != Some(p.as_path()));
        if let Some(i) = self.tabs.iter().position(|t| t.uid == uid) {
            if cur_uid != Some(uid) && cur_uid.is_some() {
                self.cur = self.tabs.iter().position(|t| Some(t.uid) == cur_uid).unwrap_or(i);
            }
        }
        self.cur = self.cur.min(self.tabs.len() - 1);
        self.changed = true;
        Ok(tf("ed_saved", &[&p.display()]))
    }

    /// « A 13 ▲▼ » for the text on screen: the source, or the assembler listing when it is shown.
    fn font_control(&mut self, ui: &mut egui::Ui, h: bool) {
        let listing = self.show_listing && self.tabs.get(self.cur).is_some_and(|t| t.lang == SrcLang::Asm);
        let slot = if listing { &mut self.listing_font } else { &mut self.font };
        if crate::fontsz::control(ui, if listing { "ed-list" } else { "ed-src" }, slot, h) {
            self.changed = true;
        }
    }

    pub fn listing_step(&mut self, d: f32) {
        self.listing_font = crate::fontsz::step(self.listing_font, d);
        self.changed = true;
    }

    pub fn font_step(&mut self, d: f32) {
        self.font = (self.font + d).clamp(FONT_MIN, FONT_MAX).round();
        self.changed = true;
    }

    pub fn font_reset(&mut self) {
        self.font = FONT_DEFAULT;
        self.changed = true;
    }

    /// The editor, in its panel or window.
    pub fn contents(&mut self, ui: &mut egui::Ui, h: bool, acts: &mut Vec<EdAction>) {
        self.shortcuts(ui, acts);
        if let Some(e) = self.help.clone() {
            let mut open = true;
            egui::Window::new(tf("ed_help_title", &[&e.word])).open(&mut open).default_width(440.0).collapsible(false).show(ui.ctx(), |ui| {
                if !e.syntax.is_empty() {
                    ui.label(RichText::new(&e.syntax).monospace().strong());
                    ui.add_space(4.0);
                }
                ui.label(&e.body);
                ui.add_space(4.0);
                ui.label(RichText::new(t("ed_help_src")).weak().small());
            });
            if !open {
                self.help = None;
            }
        }
        if self.notice.as_ref().is_some_and(|(_, t0)| t0.elapsed().as_secs_f32() > 6.0) {
            self.notice = None;
        }
        // --- one row of commands (icons, with their tooltips)
        let asm = self.tabs.get(self.cur).map(|t| t.lang == SrcLang::Asm).unwrap_or(false);
        let key = |k: egui::Key, m: egui::Modifiers| egui::Event::Key { key: k, physical_key: None, pressed: true, repeat: false, modifiers: m };
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 3.0;
            ui.menu_button("🗋", |ui| {
                if ui.button(t("ed_new_basic")).clicked() {
                    self.new_tab(SrcLang::Basic);
                    ui.close();
                }
                if ui.button(t("ed_new_asm")).clicked() {
                    self.new_tab(SrcLang::Asm);
                    ui.close();
                }
            })
            .response
            .hint(h, t("ed_new_tip"), "h_ed_new");
            if ui.button("📂").hint(h, t("ed_open_tip"), "h_ed_open").clicked() {
                acts.push(EdAction::Open);
            }
            if ui.button("💾").hint(h, t("ed_save_tip"), "h_ed_save").clicked() {
                self.save_current(acts);
            }
            if ui.button("💾…").hint(h, t("ed_save_as_tip"), "h_ed_save_as").clicked() {
                if let Some(tab) = self.tabs.get(self.cur) {
                    acts.push(EdAction::SaveAs(tab.uid));
                }
            }
            ui.separator();
            if ui.button("✂").hint(h, t("ed_cut"), "h_ed_edit").clicked() {
                self.pending.push(egui::Event::Cut);
            }
            if ui.button("🗐").hint(h, t("ed_copy_sel"), "h_ed_edit").clicked() {
                self.pending.push(egui::Event::Copy);
            }
            if ui.button("📋").hint(h, t("ed_paste"), "h_ed_edit").clicked() {
                self.paste(ui);
            }
            ui.separator();
            if ui.selectable_label(self.find.open, "🔍").hint(h, t("ed_find_tip"), "h_ed_find").clicked() {
                self.find.open = !self.find.open;
                self.find.focus_query = self.find.open;
            }
            if ui.button("⟲").hint(h, t("ed_undo"), "h_ed_edit").clicked() {
                self.pending.push(key(egui::Key::Z, egui::Modifiers::COMMAND));
            }
            if ui.button("⟳").hint(h, t("ed_redo"), "h_ed_edit").clicked() {
                self.pending.push(key(egui::Key::Z, egui::Modifiers::COMMAND | egui::Modifiers::SHIFT));
            }
            ui.separator();
            self.font_control(ui, h);
            ui.separator();
            let (rt, rk, et, ek) = if asm { ("as_run_tip", "h_as_run", "as_export_tip", "h_as_export") } else { ("ed_run_tip", "h_ed_run", "ed_export_tip", "h_ed_export") };
            if ui.button(RichText::new(t("ed_run")).strong()).hint(h, t(rt), rk).clicked() {
                acts.push(EdAction::Run);
            }
            if ui.button(t("ed_export")).hint(h, t(et), ek).clicked() {
                acts.push(EdAction::Export);
            }
            {
                if asm {
                    if ui.toggle_value(&mut self.show_listing, t("as_listing")).hint(h, t("as_listing_tip"), "h_as_listing").changed() {
                        self.changed = true;
                    }
                } else {
                    let label = |r: Option<u32>| match r {
                        None => t("ed_ram_auto").to_string(),
                        Some(k) => tf("ed_ram_kb", &[&k]),
                    };
                    let before = self.ram;
                    egui::ComboBox::from_id_salt("editeur-memoire")
                        .selected_text(label(self.ram))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.ram, None, label(None));
                            for (k, _) in intbasic::MEMORY {
                                ui.selectable_value(&mut self.ram, Some(k), label(Some(k)));
                            }
                        })
                        .response
                        .hint(h, t("ed_ram_tip"), "h_ed_ram");
                    if self.ram != before {
                        self.changed = true;
                    }
                }
            }
        });
        if self.dbg.on {
            ui.horizontal_wrapped(|ui| {
                let line = self.cursor_line.unwrap_or(1);
                if ui.button(t("db_ed_break")).hint(h, t("db_ed_break_tip"), "h_db_ed_break").clicked() {
                    acts.push(EdAction::BreakLine(line));
                }
                if ui.button(t("db_ed_runto")).hint(h, t("db_ed_runto_tip"), "h_db_ed_runto").clicked() {
                    acts.push(EdAction::RunToLine(line));
                }
            });
        }
        // --- tabs
        let mut close = None;
        egui::ScrollArea::horizontal().id_salt("editeur-onglets").show(ui, |ui| {
            ui.horizontal(|ui| {
                for (i, tab) in self.tabs.iter().enumerate() {
                    let mark = if tab.modified() { " •" } else { "" };
                    let lib = if tab.from_lib { "📚 " } else { "" };
                    let mut tip = tab.path.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| t("ed_untitled").into());
                    if tab.from_lib {
                        tip = format!("{tip}\n{}", t("ed_from_lib"));
                    }
                    let tab_resp = ui.selectable_label(i == self.cur, format!("{lib}{}{mark}", tab.name())).hint(h, &tip, "h_ed_tab");
                    if tab_resp.clicked() && self.cur != i {
                        self.cur = i;
                        self.changed = true;
                    }
                    let uid = tab.uid;
                    tab_resp.context_menu(|ui| {
                        if ui.button(t("ed_save_as")).clicked() {
                            acts.push(EdAction::SaveAs(uid));
                            ui.close();
                        }
                        if ui.button(t("dk_close_menu")).clicked() {
                            close = Some(uid);
                            ui.close();
                        }
                    });
                    if ui.small_button("×").hint(h, t("ed_close_tip"), "h_ed_close").clicked() {
                        close = Some(tab.uid);
                    }
                    ui.add_space(6.0);
                }
            });
        });
        if let Some(uid) = close {
            self.close(uid, acts);
        }
        // --- search / replace, go to line
        let hits = match (self.find.open, self.tabs.get(self.cur)) {
            (true, Some(tab)) => find_all(&tab.text, &self.find.query, self.find.case, self.find.word),
            _ => Vec::new(),
        };
        if self.find.open {
            self.find_bar(ui, h, &hits);
        }
        self.goto_bar(ui);
        ui.separator();
        // --- the text
        let listing_font = self.show_listing && self.tabs.get(self.cur).is_some_and(|t| t.lang == SrcLang::Asm);
        let font = if listing_font { self.listing_font } else { self.font };
        let row_h = ui.fonts_mut(|f| f.row_height(&FontId::monospace(font)));
        let ram = self.ram;
        let Some(tab) = self.tabs.get_mut(self.cur) else { return };
        let lang = tab.lang;
        // errors: (text line, message, file to open for an included file)
        let mut errors: Vec<(usize, String, Option<PathBuf>)> = Vec::new();
        let mut bad: Vec<usize> = Vec::new();
        let mut warns: Vec<(usize, String, Option<PathBuf>)> = Vec::new();
        match lang {
            SrcLang::Basic => {
                for e in &tab.listing().errors {
                    bad.push(e.src_line);
                    errors.push((e.src_line, error_text(e), None));
                }
            }
            SrcLang::Asm => {
                if let Ok(a) = &*tab.assembly() {
                    for w in &a.warnings {
                        let file = (w.file != 0).then(|| w.path.clone());
                        warns.push((w.line, asm_error_text(w), file));
                    }
                }
                if let Err(es) = &*tab.assembly() {
                    for e in es {
                        let file = (e.file != 0).then(|| e.path.clone());
                        if e.file == 0 {
                            bad.push(e.line);
                        }
                        errors.push((e.line, asm_error_text(e), file));
                    }
                }
            }
        }
        let n_err = errors.len();
        let n_warn = warns.len();
        let status_h = if n_err > 0 { 22.0 + n_err.min(5) as f32 * 20.0 } else { 58.0 + if n_warn > 0 { 22.0 + n_warn.min(3) as f32 * 20.0 } else { 0.0 } };
        let status_h = status_h + if self.notice.is_some() { 24.0 } else { 0.0 };
        let area_h = (ui.available_height() - status_h - 8.0).max(row_h * 3.0);
        let listing_view = lang == SrcLang::Asm && self.show_listing;
        let mut menu_resp: Option<(egui::Response, egui::Rect)> = None;
        let id = egui::Id::new(("editeur-texte", tab.uid));
        let hits = if self.find.open { find_all(&tab.text, &self.find.query, self.find.case, self.find.word) } else { Vec::new() };
        let cur_hit = (!hits.is_empty()).then_some(self.find.cur.min(hits.len().saturating_sub(1)));
        let mut scroll_to = None;
        if let Some((a, b)) = self.select.take() {
            let mut st = egui::text_edit::TextEditState::load(ui.ctx(), id).unwrap_or_default();
            st.cursor.set_char_range(Some(egui::text::CCursorRange::two(egui::text::CCursor::new(a), egui::text::CCursor::new(b))));
            st.store(ui.ctx(), id);
            scroll_to = Some(a);
        }
        if !self.pending.is_empty() || std::mem::take(&mut self.focus_text) {
            ui.ctx().memory_mut(|m| m.request_focus(id));
            let ev = std::mem::take(&mut self.pending);
            ui.ctx().input_mut(|i| i.events.extend(ev));
        }
        if let Some(line) = self.goto_line.take() {
            let idx: usize = tab.text.split_inclusive('\n').take(line.saturating_sub(1)).map(|l| l.chars().count()).sum();
            let mut st = egui::text_edit::TextEditState::load(ui.ctx(), id).unwrap_or_default();
            st.cursor.set_char_range(Some(egui::text::CCursorRange::one(egui::text::CCursor::new(idx))));
            st.store(ui.ctx(), id);
            ui.ctx().memory_mut(|m| m.request_focus(id));
            self.show_listing = false;
        }
        if listing_view {
            let rows = tab.list_rows();
            let mut goto = None;
            let fid = FontId::monospace(font);
            egui::ScrollArea::both()
                .id_salt(("editeur-listing", tab.uid))
                .max_height(area_h)
                .min_scrolled_height(area_h)
                .auto_shrink([false, false])
                .show_rows(ui, row_h, rows.len(), |ui, range| {
                    for r in &rows[range] {
                        if self.dbg.on {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 0.0;
                                let on = r.addr.map(|a| self.dbg.addrs.contains(&a)).unwrap_or(false);
                                let here = r.addr.is_some() && r.addr == self.dbg.pc;
                                let g = if here { "▶" } else if on { "●" } else { " " };
                                let col = if on { Color32::from_rgb(230, 90, 90) } else { Color32::LIGHT_GREEN };
                                let b = ui.add(egui::Label::new(RichText::new(format!("{g} ")).font(fid.clone()).color(col)).sense(egui::Sense::click()));
                                if let Some(a) = r.addr {
                                    if b.hint(h, t("db_list_break_tip"), "h_db_list_break").clicked() {
                                        acts.push(EdAction::BreakAddr(a));
                                    }
                                }
                                let c = if r.line == 0 { color(SrcLang::Asm, Kind::Directive) } else if r.text.starts_with(' ') { color(SrcLang::Asm, Kind::Rem) } else if r.file == 0 { color(SrcLang::Asm, Kind::Name) } else { color(SrcLang::Asm, Kind::Other) };
                                let mut rt = RichText::new(&r.text).font(fid.clone()).color(c);
                                if here {
                                    rt = rt.background_color(cur_bg());
                                } else if on {
                                    rt = rt.background_color(brk_bg());
                                }
                                let l = ui.add(egui::Label::new(rt).extend().sense(egui::Sense::click()));
                                if r.file == 0 && r.line > 0 && l.clicked() {
                                    goto = Some(r.line);
                                }
                                if here && self.dbg.scroll {
                                    l.scroll_to_me(Some(egui::Align::Center));
                                }
                            });
                            continue;
                        }
                        let c = if r.line == 0 {
                            color(SrcLang::Asm, Kind::Directive)
                        } else if r.text.starts_with(' ') {
                            color(SrcLang::Asm, Kind::Rem)
                        } else if r.file == 0 {
                            color(SrcLang::Asm, Kind::Name)
                        } else {
                            color(SrcLang::Asm, Kind::Other)
                        };
                        let l = ui.add(egui::Label::new(RichText::new(&r.text).font(fid.clone()).color(c)).extend().sense(egui::Sense::click()));
                        if r.file == 0 && r.line > 0 && l.clicked() {
                            goto = Some(r.line);
                        }
                    }
                });
            if let Some(l) = goto {
                self.goto_line = Some(l);
                self.show_listing = false;
            }
        } else {
            let dbg = self.dbg.clone();
            // right click: egui moves the cursor on any button; remember the
            // selection to give it back when the click is inside it
            let pre_sel = egui::text_edit::TextEditState::load(ui.ctx(), id).and_then(|st| st.cursor.char_range());
            let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap: f32| {
                let mut job = highlight_as(buf.as_str(), font, &bad, lang, &dbg);
                mark_hits(&mut job, &hits, cur_hit);
                job.wrap.max_width = wrap;
                ui.fonts_mut(|f| f.layout_job(job))
            };
            let out = egui::ScrollArea::both()
                .id_salt(("editeur-defile", tab.uid))
                .max_height(area_h)
                .min_scrolled_height(area_h)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let o = egui::TextEdit::multiline(&mut tab.text)
                        .id(id)
                        .code_editor()
                        .font(FontId::monospace(font))
                        .desired_width(f32::INFINITY)
                        .desired_rows(((area_h / row_h) as usize).max(3))
                        .lock_focus(true)
                        .layouter(&mut layouter)
                        .show(ui);
                    if let (true, Some(l)) = (self.dbg.scroll, self.dbg.cur) {
                        // start of the line, through the galley (lines may wrap)
                        let idx: usize = tab.text.split_inclusive('\n').take(l.saturating_sub(1)).map(|x| x.chars().count()).sum();
                        let r = o.galley.pos_from_cursor(egui::text::CCursor::new(idx)).translate(o.galley_pos.to_vec2());
                        ui.scroll_to_rect(r.expand2(egui::vec2(0.0, row_h)), Some(egui::Align::Center));
                    }
                    if let Some(c) = scroll_to {
                        let r = o.galley.pos_from_cursor(egui::text::CCursor::new(c)).translate(o.galley_pos.to_vec2());
                        ui.scroll_to_rect(r.expand2(egui::vec2(0.0, row_h)), Some(egui::Align::Center));
                    }
                    o
                });
            let (secondary, ptr) = ui.input(|i| (i.pointer.button_pressed(egui::PointerButton::Secondary), i.pointer.interact_pos()));
            let mut keep = None;
            if let (true, Some(pos)) = (secondary, ptr) {
                if out.inner_rect.contains(pos) {
                    let idx = out.inner.galley.cursor_from_pos(pos - out.inner.galley_pos).index.0;
                    self.ctx_idx = Some(idx);
                    if let Some(r) = pre_sel {
                        let (a, b) = (r.primary.index.0.min(r.secondary.index.0), r.primary.index.0.max(r.secondary.index.0));
                        if a < b && idx >= a && idx <= b {
                            keep = Some(r);
                        }
                    }
                }
            } else if ui.input(|i| i.pointer.primary_pressed() || i.events.iter().any(|e| matches!(e, egui::Event::Key { pressed: true, .. }))) {
                self.ctx_idx = None;
            }
            // middle click: paste the X11 primary selection (#28)
            if let (true, Some(pos)) = (ui.input(|i| i.pointer.button_pressed(egui::PointerButton::Middle)), ptr) {
                if out.inner_rect.contains(pos) {
                    if let Some(txt) = primary_selection() {
                        let idx = out.inner.galley.cursor_from_pos(pos - out.inner.galley_pos).index.0;
                        let mut st = egui::text_edit::TextEditState::load(ui.ctx(), id).unwrap_or_default();
                        st.cursor.set_char_range(Some(egui::text::CCursorRange::one(egui::text::CCursor::new(idx))));
                        st.store(ui.ctx(), id);
                        self.focus_text = true;
                        self.pending.push(egui::Event::Paste(txt));
                        ui.ctx().request_repaint();
                    }
                }
            }
            if let Some(r) = keep {
                let mut st = egui::text_edit::TextEditState::load(ui.ctx(), id).unwrap_or_default();
                st.cursor.set_char_range(Some(r));
                st.store(ui.ctx(), id);
            }
            if let Some(c) = keep.or(out.inner.cursor_range) {
                let (a, b) = (c.primary.index.0, c.secondary.index.0);
                self.sel = (a.min(b), a.max(b));
                let i = c.primary;
                self.cursor_line = Some(1 + tab.text.chars().take(i.index.0).filter(|c| *c == '\n').count());
            }
            let resp = out.inner.response.response;
            if std::mem::take(&mut self.release) {
                resp.surrender_focus();
            }
            self.focused = resp.has_focus();
            if resp.changed() {
                self.changed = true;
                self.confirm_close = None;
            }
            let resp = resp.hint(h, "", if lang == SrcLang::Asm { "h_as_text" } else { "h_ed_text" });
            menu_resp = Some((resp.clone(), out.inner_rect));
            // Ctrl + wheel over the text: size of the characters
            if ui.rect_contains_pointer(out.inner_rect) || resp.hovered() {
                let slot = if listing_view { &mut self.listing_font } else { &mut self.font };
                if crate::fontsz::wheel(ui, out.inner_rect, slot) {
                    self.changed = true;
                }
            }
        }
        if self.focused && ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)) {
            self.save_current(acts);
        }
        // right click on the text
        if let Some((resp, rect)) = menu_resp {
            resp.context_menu(|ui| {
                if self.menu_items(ui, acts) {
                    ui.close();
                }
            });
            if self.force_menu {
                egui::Area::new(egui::Id::new("menu-editeur-script")).order(egui::Order::Foreground).fixed_pos(rect.min + egui::vec2(rect.width() * 0.3, 60.0)).show(ui.ctx(), |ui| {
                    egui::Frame::menu(ui.style()).show(ui, |ui| {
                        ui.set_width(300.0);
                        self.menu_items(ui, acts);
                    });
                });
            }
        }
        // --- status (scrolls when the errors or the tape line do not fit)
        egui::ScrollArea::vertical().id_salt("editeur-pied").max_height(status_h + 4.0).auto_shrink([false, true]).show(ui, |ui| {
            if let Some((m, _)) = &self.notice {
                ui.ctx().request_repaint_after(std::time::Duration::from_millis(500));
                ui.label(RichText::new(format!("⚠ {m}")).color(Color32::from_rgb(255, 190, 90)).strong());
            }
            ui.separator();
            if n_err > 0 {
                ui.label(RichText::new(tf(if lang == SrcLang::Asm { "as_n_errors" } else { "ed_n_errors" }, &[&n_err])).color(Color32::LIGHT_RED).strong());
                for (line, msg, file) in errors.iter().take(5) {
                    if ui.link(RichText::new(msg).color(tone((255, 170, 170), (170, 20, 20)))).hint(h, t("ed_goto_tip"), "h_ed_errors").clicked() {
                        match file {
                            Some(p) => {
                                let r = self.open_at(p, *line);
                                acts.push(match r {
                                    Ok(m) => EdAction::Info(m),
                                    Err(e) => EdAction::Fail(e),
                                });
                            }
                            None => {
                            self.goto_line = Some(*line);
                            self.focus_text = true;
                        }
                        }
                    }
                }
                if n_err > 5 {
                    ui.label(RichText::new(tf("ed_more_errors", &[&(n_err - 5)])).weak());
                }
                return;
            }
            if n_warn > 0 {
                ui.label(RichText::new(tf("ed_n_warnings", &[&n_warn])).color(tone((240, 200, 90), (140, 90, 0))).strong());
                for (line, msg, file) in warns.iter().take(3) {
                    if ui.link(RichText::new(msg).color(tone((240, 215, 140), (120, 80, 0)))).hint(h, t("ed_goto_tip"), "h_ed_errors").clicked() {
                        match file {
                            Some(p) => {
                                let r = self.open_at(p, *line);
                                acts.push(match r {
                                    Ok(m) => EdAction::Info(m),
                                    Err(e) => EdAction::Fail(e),
                                });
                            }
                            None => {
                            self.goto_line = Some(*line);
                            self.focus_text = true;
                        }
                        }
                    }
                }
                if n_warn > 3 {
                    ui.label(RichText::new(tf("ed_more_errors", &[&(n_warn - 3)])).weak());
                }
            }
            let Some(tab) = self.tabs.get_mut(self.cur) else { return };
            let empty = match lang {
                SrcLang::Basic => tab.listing().lines.is_empty(),
                SrcLang::Asm => tab.assembly().as_ref().as_ref().map(|a| a.bytes.is_empty()).unwrap_or(false),
            };
            if empty {
                ui.label(RichText::new(t(if lang == SrcLang::Asm { "as_hint_empty" } else { "ed_hint_empty" })).weak());
                return;
            }
            let listing = tab.listing();
            match build(tab, ram) {
                Ok(b) => {
                    ui.horizontal_wrapped(|ui| match &b {
                        Built::Basic(img) => {
                            ui.label(tf("ed_summary", &[&listing.lines.len(), &listing.size(), &format!("{:04X}", img.lomem), &format!("{:04X}", img.himem), &img.ram_kb(), &img.free()]))
                                .hint(h, "", "h_ed_summary");
                        }
                        Built::Asm(a) => {
                            ui.label(tf("as_summary", &[&a.bytes.len(), &format!("{:04X}", a.origin), &format!("{:04X}", a.end()), &a.files.len(), &a.symbols.len()])).hint(h, "", "h_as_summary");
                        }
                    });
                    ui.horizontal_wrapped(|ui| {
                        let cmd = b.aci();
                        ui.label(t("ed_tape_range"));
                        ui.label(RichText::new(format!("C100R  {cmd}")).monospace().color(tone((235, 190, 90), (140, 80, 0)))).hint(h, t("ed_tape_tip"), "h_ed_tape");
                        if ui.small_button(t("ed_copy")).hint(h, t("ed_copy_tip"), "h_ed_copy").clicked() {
                            ui.ctx().copy_text(cmd.clone());
                            acts.push(EdAction::Info(tf("ed_copied", &[&cmd])));
                        }
                        match &b {
                            Built::Basic(_) => ui.label(RichText::new(t("ed_then_run")).weak()),
                            Built::Asm(a) => ui.label(RichText::new(tf("as_then_run", &[&a.run_command()])).weak()),
                        };
                    });
                }
                Err(e) => {
                    ui.label(RichText::new(e).color(Color32::LIGHT_RED));
                }
            }
        });
    }

    fn paste(&mut self, ui: &egui::Ui) {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::RequestPaste);
        self.focus_text = true;
    }

    /// Lines of the selection (0-based, first and last).
    fn sel_lines(&self) -> (usize, usize) {
        let Some(tab) = self.tabs.get(self.cur) else { return (0, 0) };
        let line = |c: usize| tab.text.chars().take(c).filter(|x| *x == '\n').count();
        (line(self.sel.0), line(self.sel.1))
    }

    /// Replace the text of the current tab (an editing command).
    fn set_text(&mut self, text: String) {
        if let Some(tab) = self.tabs.get_mut(self.cur) {
            if tab.text != text {
                tab.text = text;
                self.changed = true;
            }
        }
    }

    fn comment(&mut self) {
        let (l0, l1) = self.sel_lines();
        if let Some(tab) = self.tabs.get(self.cur) {
            let t = toggle_comment(&tab.text, l0, l1, tab.lang);
            self.set_text(t);
        }
    }

    /// F12 / menu: go to the label or the BASIC line number. Anything else
    /// says so, visibly (strip under the text, and the status bar).
    fn goto_definition(&mut self, word: Option<String>, acts: &mut Vec<EdAction>) {
        let Some(tab) = self.tabs.get(self.cur) else { return };
        let Some(w) = word else {
            self.notice(t("ed_no_word").to_string(), acts);
            return;
        };
        match definition(&tab.text, &w, tab.lang) {
            Some(l) => self.goto_line = Some(l),
            None => {
                let m = tf("ed_no_def", &[&w]);
                self.notice(m, acts);
            }
        }
    }

    fn notice(&mut self, m: String, acts: &mut Vec<EdAction>) {
        self.notice = Some((m.clone(), std::time::Instant::now()));
        acts.push(EdAction::Fail(m));
    }

    /// Keyboard shortcuts of the text: Ctrl+F/H/G, Ctrl+/, F12, F9, F5.
    fn shortcuts(&mut self, ui: &egui::Ui, acts: &mut Vec<EdAction>) {
        // `self.focused` is reset by the layout at every frame, before the
        // editor is drawn: ask egui whether the text has the keyboard
        let Some(tab) = self.tabs.get(self.cur) else { return };
        let id = egui::Id::new(("editeur-texte", tab.uid));
        if !ui.memory(|m| m.has_focus(id)) {
            return;
        }
        let k = |m: egui::Modifiers, key: egui::Key| ui.input_mut(|i| i.consume_key(m, key));
        if k(egui::Modifiers::COMMAND, egui::Key::F) {
            self.find.open = true;
            self.find.focus_query = true;
        }
        if k(egui::Modifiers::COMMAND, egui::Key::H) {
            self.find.open = true;
            self.find.focus_repl = true;
        }
        if k(egui::Modifiers::COMMAND, egui::Key::G) {
            self.goto_text = Some(String::new());
        }
        if k(egui::Modifiers::COMMAND, egui::Key::Slash) {
            self.comment();
        }
        if k(egui::Modifiers::NONE, egui::Key::F12) {
            let w = word_at(&self.tabs[self.cur].text, self.sel.0);
            self.goto_definition(w, acts);
        }
        if k(egui::Modifiers::NONE, egui::Key::F9) {
            acts.push(EdAction::BreakLine(self.cursor_line.unwrap_or(1)));
        }
        if k(egui::Modifiers::NONE, egui::Key::F5) {
            acts.push(EdAction::Run);
        }
    }

    /// The right-click menu of the text; true when an item was chosen.
    fn menu_items(&mut self, ui: &mut egui::Ui, acts: &mut Vec<EdAction>) -> bool {
        let Some(tab) = self.tabs.get(self.cur) else { return false };
        let lang = tab.lang;
        let word = word_at(&tab.text, self.ctx_idx.unwrap_or(self.sel.0));
        let line = self.cursor_line.unwrap_or(1);
        let defined = word.as_ref().map(|w| definition(&tab.text, w, lang).is_some()).unwrap_or(false);
        let helped = word.as_ref().and_then(|w| crate::help::entry(w, lang == SrcLang::Asm));
        let item = |ui: &mut egui::Ui, label: &str, key: &str| {
            ui.horizontal(|ui| {
                let b = ui.add(egui::Button::new(label).frame(false)).clicked();
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| ui.label(RichText::new(key).weak()));
                b
            })
            .inner
        };
        let key = |k: egui::Key| egui::Event::Key { key: k, physical_key: None, pressed: true, repeat: false, modifiers: egui::Modifiers::COMMAND };
        let mut done = false;
        if item(ui, &format!("✂ {}", t("ed_cut")), "Ctrl+X") {
            self.pending.push(egui::Event::Cut);
            done = true;
        }
        if item(ui, &format!("🗐 {}", t("ed_copy_sel")), "Ctrl+C") {
            self.pending.push(egui::Event::Copy);
            done = true;
        }
        if item(ui, &format!("📋 {}", t("ed_paste")), "Ctrl+V") {
            self.paste(ui);
            done = true;
        }
        if item(ui, &format!("    {}", t("ed_select_all")), "Ctrl+A") {
            self.pending.push(key(egui::Key::A));
            done = true;
        }
        ui.separator();
        if item(ui, &format!("🔍 {}", t("ed_find")), "Ctrl+F") {
            self.find.open = true;
            self.find.focus_query = true;
            if let (Some(w), true) = (&word, self.find.query.is_empty()) {
                self.find.query = w.clone();
            }
            done = true;
        }
        if item(ui, &format!("    {}", t("ed_replace_menu")), "Ctrl+H") {
            self.find.open = true;
            self.find.focus_repl = true;
            done = true;
        }
        if item(ui, &format!("    {}", t("ed_goto_line")), "Ctrl+G") {
            self.goto_text = Some(String::new());
            done = true;
        }
        ui.separator();
        let ck = if lang == SrcLang::Asm { "ed_comment" } else { "ed_rem" };
        if item(ui, &format!(";  {}", t(ck)), "Ctrl+/") {
            self.comment();
            done = true;
        }
        if lang == SrcLang::Asm {
            let (l0, l1) = self.sel_lines();
            for (more, k) in [(true, "ed_indent"), (false, "ed_unindent")] {
                if item(ui, &format!("    {}", t(k)), "") {
                    let t = indent(&self.tabs[self.cur].text, l0, l1, more);
                    self.set_text(t);
                    done = true;
                }
            }
        }
        if let Some(w) = &word {
            // enabled only for a label / a BASIC line number that exists
            let go = ui.add_enabled_ui(defined, |ui| item(ui, &format!("    {}", tf("ed_goto_def", &[w])), "F12")).inner;
            if go {
                self.goto_definition(Some(w.clone()), acts);
                done = true;
            }
            if let Some(e) = helped {
                if item(ui, &format!("❓ {}", tf("ed_help_on", &[&e.word])), "") {
                    self.help = Some(e);
                    done = true;
                }
            }
        }
        if lang == SrcLang::Asm && item(ui, &format!("🔎 {}", t("ed_show_disasm")), "") {
            acts.push(EdAction::ShowDisasm(line));
            done = true;
        }
        ui.separator();
        if item(ui, &format!("⏺ {}", t("ed_breakpoint")), "F9") {
            acts.push(EdAction::BreakLine(line));
            done = true;
        }
        let rk = if lang == SrcLang::Asm { "ed_asm_run" } else { "ed_basic_run" };
        if item(ui, &format!("▶ {}", t(rk)), "F5") {
            acts.push(EdAction::Run);
            done = true;
        }
        if lang == SrcLang::Basic {
            ui.separator();
            ui.menu_button(format!("    {}", t("ed_renumber")), |ui| {
                ui.horizontal(|ui| {
                    ui.label(t("ed_renum_start"));
                    ui.add(egui::DragValue::new(&mut self.renum.0).range(0..=32767));
                    ui.label(t("ed_renum_step"));
                    ui.add(egui::DragValue::new(&mut self.renum.1).range(1..=1000));
                });
                if ui.button(t("ed_renum_ok")).clicked() {
                    let (text, n) = renumber(&self.tabs[self.cur].text, self.renum.0, self.renum.1);
                    self.set_text(text);
                    acts.push(EdAction::Info(tf("ed_renumbered", &[&n])));
                    ui.close();
                }
            });
        }
        done
    }

    /// Search / replace bar, under the tabs.
    fn find_bar(&mut self, ui: &mut egui::Ui, h: bool, hits: &[Range<usize>]) {
        let fill = if ui.visuals().dark_mode { Color32::from_gray(38) } else { Color32::from_gray(228) };
        let mut go: Option<isize> = None;
        let mut replace = false;
        let mut replace_all = false;
        egui::Frame::NONE.fill(fill).inner_margin(egui::Margin::symmetric(6, 3)).show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("🔍");
                let q = ui.add(egui::TextEdit::singleline(&mut self.find.query).desired_width(130.0).hint_text(t("ed_find"))).hint(h, t("ed_find_tip"), "h_ed_find");
                if std::mem::take(&mut self.find.focus_query) {
                    q.request_focus();
                }
                if q.changed() {
                    // first hit after the cursor
                    go = Some(0);
                }
                if q.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    go = Some(if ui.input(|i| i.modifiers.shift) { -1 } else { 1 });
                    q.request_focus();
                }
                let n = hits.len();
                let txt = if n == 0 { if self.find.query.is_empty() { String::new() } else { t("ed_no_match").to_string() } } else { tf("ed_n_of", &[&(self.find.cur.min(n - 1) + 1), &n]) };
                ui.label(RichText::new(txt).weak());
                if ui.small_button("⏶").hint(h, t("ed_prev"), "h_ed_find").clicked() {
                    go = Some(-1);
                }
                if ui.small_button("⏷").hint(h, t("ed_next"), "h_ed_find").clicked() {
                    go = Some(1);
                }
                if ui.selectable_label(self.find.case, "Aa").hint(h, t("ed_case"), "h_ed_find").clicked() {
                    self.find.case = !self.find.case;
                }
                if ui.selectable_label(self.find.word, t("ed_word_short")).hint(h, t("ed_word"), "h_ed_find").clicked() {
                    self.find.word = !self.find.word;
                }
                // second row: a separator inside a wrapped row drew the buttons over the Find field (#26)
                ui.end_row();
                ui.label(t("ed_replace_with"));
                let r = ui.add(egui::TextEdit::singleline(&mut self.find.repl).desired_width(130.0));
                if std::mem::take(&mut self.find.focus_repl) {
                    r.request_focus();
                }
                replace = ui.add_enabled(n > 0, egui::Button::new(t("ed_replace"))).hint(h, t("ed_replace"), "h_ed_find").clicked();
                replace_all = ui.add_enabled(n > 0, egui::Button::new(t("ed_replace_all"))).hint(h, t("ed_replace_all"), "h_ed_find").clicked();
                if ui.small_button("✖").hint(h, t("ed_find_close"), "h_ed_find").clicked() || (ui.input(|i| i.key_pressed(egui::Key::Escape)) && (q.has_focus() || r.has_focus())) {
                    self.find.open = false;
                }
            });
        });
        let Some(tab) = self.tabs.get(self.cur) else { return };
        let n = hits.len();
        if replace_all {
            let mut text = tab.text.clone();
            for r in hits.iter().rev() {
                text.replace_range(r.clone(), &self.find.repl);
            }
            self.set_text(text);
            self.find.cur = 0;
            self.goto_text = None;
            return;
        }
        if replace && n > 0 {
            let i = self.find.cur.min(n - 1);
            let mut text = tab.text.clone();
            text.replace_range(hits[i].clone(), &self.find.repl);
            let at = hits[i].start + self.find.repl.len();
            self.set_text(text);
            // next hit after the replaced one
            let t = &self.tabs[self.cur].text;
            let next = find_all(t, &self.find.query, self.find.case, self.find.word);
            if let Some(j) = next.iter().position(|r| r.start >= at).or((!next.is_empty()).then_some(0)) {
                self.find.cur = j;
                self.select = Some((char_of(t, next[j].start), char_of(t, next[j].end)));
            }
            return;
        }
        if let (Some(d), true) = (go, n > 0) {
            let i = match d {
                0 => {
                    let c = byte_of(&tab.text, self.sel.0);
                    hits.iter().position(|r| r.start >= c).unwrap_or(0)
                }
                1 => (self.find.cur + 1) % n,
                _ => (self.find.cur + n - 1) % n,
            };
            self.find.cur = i;
            self.select = Some((char_of(&tab.text, hits[i].start), char_of(&tab.text, hits[i].end)));
        }
    }

    /// « Aller à la ligne » prompt.
    fn goto_bar(&mut self, ui: &mut egui::Ui) {
        let Some(g) = &mut self.goto_text else { return };
        let mut close = false;
        let mut to = None;
        ui.horizontal(|ui| {
            ui.label(t("ed_goto_prompt"));
            let r = ui.add(egui::TextEdit::singleline(g).desired_width(70.0));
            if g.is_empty() && !r.has_focus() {
                r.request_focus();
            }
            if r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                to = g.trim().parse::<usize>().ok();
                close = true;
            }
            if ui.small_button("✖").clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                close = true;
            }
        });
        if let Some(l) = to {
            self.goto_line = Some(l.max(1));
        }
        if close {
            self.goto_text = None;
        }
    }

    fn save_current(&mut self, acts: &mut Vec<EdAction>) {
        let Some(tab) = self.tabs.get(self.cur) else { return };
        let uid = tab.uid;
        if tab.path.is_none() {
            acts.push(EdAction::SaveAs(uid));
            return;
        }
        acts.push(match self.save(uid, None) {
            Ok(m) => EdAction::Info(m),
            Err(e) => EdAction::Fail(e),
        });
    }

    fn close(&mut self, uid: u64, acts: &mut Vec<EdAction>) {
        let Some(i) = self.tabs.iter().position(|t| t.uid == uid) else { return };
        if self.tabs[i].modified() && self.confirm_close != Some(uid) {
            self.confirm_close = Some(uid);
            acts.push(EdAction::Fail(tf("ed_close_confirm", &[&self.tabs[i].name()])));
            return;
        }
        self.confirm_close = None;
        self.tabs.remove(i);
        if self.tabs.is_empty() {
            self.push(None, String::new(), String::new(), SrcLang::Basic);
        }
        if self.cur >= self.tabs.len() || self.cur > i {
            self.cur = self.cur.saturating_sub(1).min(self.tabs.len() - 1);
        }
        self.changed = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edit_helpers() {
        assert_eq!(find_all("LDA lda ALDA", "lda", false, false).len(), 3);
        assert_eq!(find_all("LDA lda ALDA", "lda", true, false), vec![4..7]);
        assert_eq!(find_all("LDA lda ALDA", "lda", false, true).len(), 2);
        let b = "10 PRINT 1\n20 GOTO 10";
        let c = toggle_comment(b, 0, 1, SrcLang::Basic);
        assert_eq!(c, "10 REM PRINT 1\n20 REM GOTO 10");
        assert_eq!(toggle_comment(&c, 0, 1, SrcLang::Basic), b);
        let a = "        LDA #1\nloop:   RTS";
        let c = toggle_comment(a, 0, 0, SrcLang::Asm);
        assert_eq!(c, "; " .to_string() + a);
        assert_eq!(toggle_comment(&c, 0, 0, SrcLang::Asm), a);
        assert_eq!(indent("LDA\nRTS", 1, 1, true), "LDA\n        RTS");
        assert_eq!(indent("LDA\n        RTS", 0, 1, false), "LDA\nRTS");
        let (r, n) = renumber("5 PRINT 1\n7 GOTO 5\n9 IF A THEN 7", 100, 10);
        assert_eq!(n, 3);
        assert_eq!(r, "100 PRINT 1\n110 GOTO 100\n120 IF A THEN 110");
        assert_eq!(word_at("  JMP loop", 8).as_deref(), Some("loop"));
        assert_eq!(definition("        JMP loop\nloop:   RTS", "loop", SrcLang::Asm), Some(2));
        assert_eq!(definition("10 PRINT\n20 GOTO 10", "20", SrcLang::Basic), Some(2));
    }

    fn kinds(s: &str) -> Vec<(String, Kind)> {
        spans(s).into_iter().map(|(r, k)| (s[r].to_string(), k)).collect()
    }

    #[test]
    fn colours() {
        let k = kinds("10 PRINT \"HI\";A1+5: REM  note");
        assert_eq!(k[0], ("10".into(), Kind::LineNo));
        assert_eq!(k[2], ("PRINT".into(), Kind::Keyword));
        assert!(k.contains(&("\"HI\"".into(), Kind::Str)));
        assert!(k.contains(&("A1".into(), Kind::Name)));
        assert!(k.contains(&("5".into(), Kind::Number)));
        assert!(k.contains(&("+".into(), Kind::Op)));
        assert_eq!(k.last().unwrap(), &("  note".into(), Kind::Rem));
        let k = kinds("20 if x then 30");
        assert!(k.contains(&("if".into(), Kind::Keyword)) && k.contains(&("then".into(), Kind::Keyword)));
        // unterminated string and non-ASCII text do not panic
        let _ = kinds("30 PRINT \"é");
        let job = highlight("10 A=1\n20 B$=\"X\"\n", 14.0, &[2]);
        assert_eq!(job.text, "10 A=1\n20 B$=\"X\"\n");
    }

    #[test]
    fn colours_asm() {
        let k: Vec<(String, Kind)> = {
            let s = "START:  lda #$1F ; note";
            spans_asm(s).into_iter().map(|(r, k)| (s[r].to_string(), k)).collect()
        };
        assert_eq!(k[0], ("START:".into(), Kind::Label));
        assert_eq!(k[1], ("lda".into(), Kind::Keyword));
        assert!(k.contains(&("$1F".into(), Kind::Number)));
        assert_eq!(k.last().unwrap(), &("; note".into(), Kind::Rem));
        let s = "NMAX = %1010 << 2";
        let k: Vec<_> = spans_asm(s).into_iter().map(|(r, k)| (s[r].to_string(), k)).collect();
        assert_eq!(k[0], ("NMAX".into(), Kind::Label));
        assert!(k.contains(&("%1010".into(), Kind::Number)));
        let s = "  .byte \"AB\", 'c'  ; x";
        let k: Vec<_> = spans_asm(s).into_iter().map(|(r, k)| (s[r].to_string(), k)).collect();
        assert_eq!(k[0], (".byte".into(), Kind::Directive));
        assert_eq!(k[1], ("\"AB\"".into(), Kind::Str));
        // a macro call is not a mnemonic; unterminated and non-ASCII: no panic
        let s = "  PRINTS msg";
        assert_eq!(spans_asm(s)[0].1, Kind::Name);
        let _ = spans_asm("  .asciiz \"é");
        let _ = highlight_as("lda #1\n", 14.0, &[1], SrcLang::Asm, &EdDbg::default());
    }

    #[test]
    fn asm_tab_build() {
        assert_eq!(lang_of(Path::new("jeu.S")), SrcLang::Asm);
        assert_eq!(lang_of(Path::new("jeu.bas")), SrcLang::Basic);
        let mut e = Editor::from_cfg(&EditorCfg::default());
        e.new_tab(SrcLang::Asm);
        let tab = e.current().unwrap();
        tab.text = "  .org $0300\nSTART: lda #'A'\n  jsr $FFEF\n  jmp START\n".into();
        let b = build(tab, None).unwrap();
        assert_eq!(b.aci(), "0300.0307R");
        assert_eq!(b.run(), "300R");
        assert_eq!(b.blocks(), vec![(0x300, vec![0xA9, 0x41, 0x20, 0xEF, 0xFF, 0x4C, 0x00, 0x03])]);
        assert!(!tab.list_rows().is_empty());
        tab.text = "  lda ($10),y\n  ldx (5,x)\n".into();
        let err = build(tab, None).err().unwrap();
        assert!(err.contains('2'), "{err}");
        // the language of an unsaved tab is kept
        let c = e.to_cfg(true);
        let e2 = Editor::from_cfg(&c);
        assert_eq!(e2.tabs.last().unwrap().lang, SrcLang::Asm);
    }

    #[test]
    fn save_as_of_an_untitled_tab_keeps_one_tab() {
        let dir = std::env::temp_dir().join(format!("rdk-saveas-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut e = Editor::from_cfg(&EditorCfg::default());
        let uid = e.current().unwrap().uid;
        e.current().unwrap().text = "10 PRINT 1".into();
        assert_eq!(e.tabs.len(), 1);
        e.save(uid, Some(dir.join("a.bas"))).unwrap();
        assert_eq!(e.tabs.len(), 1);
        assert!(!e.tabs[0].modified());
        assert_eq!(e.tabs[0].name(), "a.bas");
        // a second tab on the file that is overwritten is not kept as a duplicate
        e.new_tab(SrcLang::Basic);
        let uid2 = e.current().unwrap().uid;
        e.current().unwrap().text = "20 END".into();
        e.save(uid2, Some(dir.join("a.bas"))).unwrap();
        assert_eq!(e.tabs.len(), 1);
        assert_eq!(e.tabs[0].uid, uid2);
        // the tab closes at once when it is not modified
        let mut acts = Vec::new();
        e.close(uid2, &mut acts);
        assert_eq!(e.tabs.len(), 1, "an empty untitled tab replaces it");
        assert!(e.tabs[0].path.is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn cfg_round_trip() {
        let mut e = Editor::from_cfg(&EditorCfg::default());
        assert_eq!(e.tabs.len(), 1);
        e.tabs[0].text = "10 PRINT 1\n".into();
        let c = e.to_cfg(true);
        assert_eq!(c.tabs.len(), 1);
        let e2 = Editor::from_cfg(&c);
        assert_eq!(e2.tabs[0].text, "10 PRINT 1\n");
        assert!(e2.tabs[0].modified());
        let img = image_of(&intbasic::tokenize("10 PRINT 1\n"), None).unwrap();
        assert_eq!(img.aci_command(), format!("004A.00FFR {:04X}.0FFFR", img.pp));
    }
}
