//! The window: toolbar, screen, program library and status bar.

use crate::layout::Pane;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};

use apple1_core::aciwav::ExportOptions;
use apple1_core::display::{CursorBlink, COLS, ROWS};
use apple1_core::machine::map_key;
use apple1_core::program::{self, Kind, Program};
use eframe::egui::{self, Color32, ColorImage, Key, RichText, TextureHandle, TextureOptions, ViewportCommand};
use egui_file_dialog::{FileDialog, FileDialogLabels};

use crate::build;
use crate::crt;
use crate::github::{self, Meta, Spec};
use crate::config::{home, GuiConfig, Zoom, ZOOM_MAX};
use crate::gen2screen;
use crate::i18n::{self, t, tf, Detected, Lang};
use crate::launch;
use crate::library::{self, Entry, Folder, Library, Root};
use crate::params::{self, Plan, ProgKind, ProgParams};
use crate::screen::{self, Phosphor, CELL_H, CELL_W, HEIGHT, WIDTH};
use crate::session::Session;

/// What a file dialog was opened for.
#[derive(Clone, Debug)]
enum Op {
    Load,
    /// Cassette panel « Insérer »: a .wav/.aiff into the virtual deck.
    InsertTape,
    Save(String),
    Basic,
    Record,
    LibraryDir,
    /// Another library folder.
    AddLibrary,
    /// BASIC editor: open a listing, save a tab (uid), export a tape.
    EdOpen,
    EdSave(u64),
    EdExport(u64),
    /// « Photo → ASCII »: open a picture.
    AsciiOpen,
    /// Machine state: save, load.
    StateSave,
    StateLoad,
    /// EPROM programmer: the file to write or compare.
    EpromFile,
    /// P-LAB card: the 16 KB image of the CodeTank EEPROM.
    PlabRom,
}

/// Small question windows.
enum Ask {
    /// Load address for a .bin (or a tape without addresses).
    Addr { path: PathBuf, text: String, err: Option<String> },
    /// Tape file: into memory or as a virtual cassette?
    Tape { path: PathBuf },
    SaveRanges { text: String, err: Option<String> },
    Rename { path: PathBuf, text: String, err: Option<String> },
    Delete { path: PathBuf },
    /// ⚙ parameters of a library program.
    Params { path: PathBuf, p: ProgParams, err: Option<String>, is_bin: bool, len: usize },
    About,
    /// « Votre espace de travail »: shown at the first launches.
    Welcome,
    Preferences,
    /// « + Projet GitHub »: URL or owner/repo.
    GitHub { text: String, err: Option<String> },
    /// Name of a layout to save (`old` None) or to rename (`old` Some).
    LayoutName { text: String, err: Option<String>, old: Option<String> },
    /// The kit version changed: offer a fresh start (library files kept).
    Fresh { previous: String },
    /// « Réinitialiser les préférences » asked for a confirmation.
    ConfirmReset,
}

/// What the « Disposition » menu asks for.
enum LayoutAct {
    Welcome,
    Preset(crate::layout::Preset),
    Saved(String),
    Save,
    Rename(String),
    Delete(String),
    /// Right-click: open the little menu (Renommer / Supprimer) there.
    Context(String, egui::Pos2),
}

/// Toolbar icons drawn by hand (no font has them).
#[derive(Clone, Copy)]
enum Icon {
    /// Speedometer: fast speed.
    Gauge,
    /// DIP chip with its round UV window: EPROM programmer.
    Chip,
}

/// A toolbar button with a drawn icon (and a text when `text` is given); lit like a
/// selectable label when `selected`.
fn icon_button(ui: &mut egui::Ui, icon: Icon, selected: bool, text: Option<&str>) -> egui::Response {
    let pad = ui.spacing().button_padding;
    let font = egui::TextStyle::Button.resolve(ui.style());
    let galley = text.map(|s| ui.painter().layout_no_wrap(s.to_string(), font, Color32::PLACEHOLDER));
    let icon_w = match icon {
        Icon::Gauge => 17.0,
        Icon::Chip => 21.0,
    };
    let w = pad.x * 2.0 + icon_w + galley.as_ref().map(|g| g.size().x + 4.0).unwrap_or(0.0);
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(w, ui.spacing().interact_size.y), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let v = if selected { ui.style().interact_selectable(&resp, true) } else { *ui.style().interact(&resp) };
        let bg = if selected { v.bg_fill } else { v.weak_bg_fill };
        let p = ui.painter();
        p.rect(rect, v.corner_radius, bg, v.bg_stroke, egui::StrokeKind::Inside);
        let fg = v.fg_stroke.color;
        let c = egui::pos2(rect.left() + pad.x + icon_w / 2.0, rect.center().y);
        let st = egui::Stroke::new(1.4, fg);
        match icon {
            Icon::Gauge => {
                let (r, c) = (7.2, egui::pos2(c.x, c.y + 3.5));
                let at = |a: f32, k: f32| egui::pos2(c.x + a.cos() * r * k, c.y - a.sin() * r * k);
                let arc: Vec<egui::Pos2> = (0..=16).map(|i| at(std::f32::consts::PI * i as f32 / 16.0, 1.0)).collect();
                p.add(egui::Shape::line(arc, st));
                for a in [0.15f32, 0.5, 0.85] {
                    let a = std::f32::consts::PI * (1.0 - a);
                    p.line_segment([at(a, 1.0), at(a, 0.68)], egui::Stroke::new(1.0, fg));
                }
                p.line_segment([c, at(0.8, 0.85)], egui::Stroke::new(1.6, Color32::from_rgb(255, 120, 60)));
                p.circle_filled(c, 1.6, fg);
            }
            Icon::Chip => {
                let body = egui::Rect::from_center_size(c, egui::vec2(icon_w - 3.0, 8.5));
                let pin = Color32::from_gray(200);
                for i in 0..4 {
                    let x = body.left() + (i as f32 + 0.5) * body.width() / 4.0;
                    p.line_segment([egui::pos2(x, body.top() - 2.6), egui::pos2(x, body.top())], egui::Stroke::new(1.2, pin));
                    p.line_segment([egui::pos2(x, body.bottom()), egui::pos2(x, body.bottom() + 2.6)], egui::Stroke::new(1.2, pin));
                }
                p.rect(body, 1.0, Color32::from_rgb(70, 72, 80), egui::Stroke::new(1.0, fg), egui::StrokeKind::Inside);
                p.circle_filled(egui::pos2(body.left(), c.y), 1.6, bg);
                p.circle(egui::pos2(c.x + 0.8, c.y), 2.2, Color32::from_rgb(140, 210, 255), egui::Stroke::new(0.6, Color32::from_rgb(230, 245, 255)));
            }
        }
        if let Some(g) = galley {
            p.galley(egui::pos2(rect.left() + pad.x + icon_w + 4.0, rect.center().y - g.size().y / 2.0), g, fg);
        }
    }
    resp
}

/// A library program being loaded through the virtual cassette.
struct Loading {
    path: PathBuf,
    /// Keys to type once loaded (▶), or None (double-click).
    run: Option<String>,
    /// Emulated time when the tape ended.
    done_at: Option<f64>,
    started: f64,
}

/// Command line options used for automatic tests and demos.
#[derive(Default, Debug)]
pub struct Script {
    pub load: Vec<PathBuf>,
    pub tapes: Vec<PathBuf>,
    pub type_text: Vec<(f64, String)>,
    pub capture: Option<(f64, PathBuf)>,
    pub quit_after_capture: bool,
    pub zoom: Option<Zoom>,
    pub window: Option<[f32; 2]>,
    /// Library programs to load (double-click) / run (▶) / open in ⚙, after S s.
    pub lib_load: Vec<(f64, String)>,
    pub lib_run: Vec<(f64, String)>,
    pub lib_params: Option<(f64, String)>,
    pub preferences: Option<f64>,
    /// Listings opened in the BASIC editor, and when to press « Lancer ».
    pub editor_files: Vec<PathBuf>,
    pub editor_run: Option<f64>,
    /// « 📼 Exporter WAV » of the current tab at this time, into this file
    /// (as the button, then the file dialog, would do).
    pub editor_export: Option<(f64, PathBuf)>,
    /// Debugger open, its breakpoints (applied after « Lancer » for labels),
    /// watches, and commands at given times.
    pub debugger: bool,
    /// Apple-1 screen in its own window.
    pub screen_window: bool,
    pub dbg_breaks: Vec<String>,
    pub dbg_watch: Vec<String>,
    pub dbg_cmds: Vec<(f64, crate::debugger::Cmd)>,
    /// « Photo → ASCII »: picture (or .txt) opened, settings, and when to
    /// press its « ▶ Lancer ».
    pub ascii: Option<PathBuf>,
    pub ascii_settings: Vec<(String, String)>,
    pub ascii_run: Option<f64>,
    /// Tab of the preferences shown by --preferences; the editor's search
    /// bar (text) and right-click menu opened (for screenshots).
    pub prefs_tab: Option<PrefTab>,
    pub editor_find: Option<String>,
    pub editor_menu: bool,
    pub screen_menu: bool,
    /// The « État » menu shown open, and the EPROM window open (screenshots).
    pub state_menu: bool,
    pub eprom: bool,
    /// The « Charger » file window open (screenshots).
    pub dialog: bool,
    /// The workspace window shown (screenshots).
    pub welcome: bool,
    /// The « new version » question shown (screenshots).
    pub fresh: bool,
    /// Scroll the toolbar to the Cassette group (screenshots).
    pub bar_end: bool,
    /// Open the start-up guide on this screen (1..3). Screenshots.
    pub wizard_page: Option<u8>,
}

/// Number and (soft) colour of a pane in the thumbnails of the workspace window.
/// 1–6 stay as they were; the colour screen (GEN2 / P-LAB) is 7, in its own colour.
pub(crate) fn welcome_pane(p: Pane) -> (u8, Color32) {
    match p {
        Pane::Editor => (1, Color32::from_rgb(62, 86, 124)),
        Pane::Screen => (2, Color32::from_rgb(64, 106, 80)),
        Pane::Debugger => (3, Color32::from_rgb(128, 98, 62)),
        Pane::Disasm => (4, Color32::from_rgb(118, 80, 66)),
        Pane::Dump => (5, Color32::from_rgb(94, 76, 118)),
        Pane::Library => (6, Color32::from_rgb(84, 94, 104)),
        Pane::Color | Pane::Plab => (7, Color32::from_rgb(148, 72, 36)),
        Pane::Ascii => (8, Color32::from_rgb(84, 84, 84)),
    }
}

/// Name of a pane in the legend of the workspace window.
pub(crate) fn welcome_pane_key(p: Pane) -> &'static str {
    match p {
        Pane::Editor => "wl_p_editor",
        Pane::Screen => "scr_title",
        Pane::Debugger => "db_title",
        Pane::Disasm => "db_disasm",
        Pane::Dump => "db_dump",
        Pane::Library => "lib_title",
        Pane::Color | Pane::Plab => "wl_p_color",
        Pane::Ascii => "ax_title",
    }
}

/// The real definition of the monitor, in physical pixels (egui gives it in points: ÷ the
/// whole scale, native × interface zoom). (0, 0) when unknown.
pub fn screen_pixels(ctx: &egui::Context) -> (u32, u32) {
    let mon = ctx.input(|i| i.viewport().monitor_size).unwrap_or_default();
    let ppp = ctx.pixels_per_point();
    ((mon.x * ppp).round() as u32, (mon.y * ppp).round() as u32)
}

impl Script {
    /// Something was asked on the command line (a capture, a window to show, a program to load…):
    /// the workspace window must not open on top of it.
    pub fn automated(&self) -> bool {
        self.capture.is_some()
            || self.window.is_some()
            || !self.load.is_empty()
            || !self.tapes.is_empty()
            || !self.type_text.is_empty()
            || !self.lib_load.is_empty()
            || !self.lib_run.is_empty()
            || self.lib_params.is_some()
            || self.preferences.is_some()
            || !self.editor_files.is_empty()
            || self.editor_run.is_some()
            || self.editor_export.is_some()
            || self.debugger
            || self.screen_window
            || self.ascii.is_some()
            || self.editor_find.is_some()
            || self.editor_menu
            || self.screen_menu
            || self.state_menu
            || self.eprom
            || self.dialog
    }
}

/// Tabs of the preferences.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PrefTab {
    #[default]
    Interface,
    Editeur,
    Bibliotheque,
    Cassette,
    Gen2,
    Avance,
}

impl PrefTab {
    pub const ALL: [PrefTab; 6] = [PrefTab::Interface, PrefTab::Editeur, PrefTab::Bibliotheque, PrefTab::Cassette, PrefTab::Gen2, PrefTab::Avance];
    fn key(self) -> &'static str {
        match self {
            PrefTab::Interface => "pr_tab_interface",
            PrefTab::Editeur => "pr_tab_editor",
            PrefTab::Bibliotheque => "pr_tab_library",
            PrefTab::Cassette => "pr_tab_tape",
            PrefTab::Gen2 => "pr_tab_gen2",
            PrefTab::Avance => "pr_tab_advanced",
        }
    }
    pub fn parse(s: &str) -> Option<PrefTab> {
        Some(match s {
            "interface" | "couleurs" | "colors" | "colours" => PrefTab::Interface,
            "editeur" | "editor" => PrefTab::Editeur,
            "bibliotheque" | "library" => PrefTab::Bibliotheque,
            "cassette" | "tape" => PrefTab::Cassette,
            "gen2" => PrefTab::Gen2,
            "avance" | "advanced" => PrefTab::Avance,
            _ => return None,
        })
    }
}

pub struct App {
    s: Session,
    cfg: GuiConfig,
    #[allow(dead_code)]
    first_run: bool,
    lib: Library,
    detected: Detected,
    tex: Option<TextureHandle>,
    drawn: Option<(u64, bool, Phosphor)>,
    /// Colour monitor of the GEN2 card.
    tex2: Option<TextureHandle>,
    tex_plab: Option<TextureHandle>,
    /// Programmer window (EPROM).
    eprom: crate::eprom_ui::EpromUi,
    /// Where the « État » button is (for the menu forced open).
    state_btn: egui::Rect,
    drawn2: Option<u64>,
    /// Pictures before the screening processing (terminal, GEN2).
    raw: Option<ColorImage>,
    raw2: Option<ColorImage>,
    crt: crt::Crt,
    crt2: crt::Crt,
    /// GEN2 picture baked at the screen's pixel size (phosphor stripes).
    tex_bake: Option<TextureHandle>,
    bake_key: Option<(u64, usize, usize, crt::Screening, u8)>,
    /// Settings used for the last processing, and its time.
    crt_cfg: Option<(crt::Screening, Phosphor, bool)>,
    crt_t: Instant,
    blink: CursorBlink,
    message: String,
    error: bool,
    dialog: Option<(FileDialog, Op)>,
    ask: Option<Ask>,
    /// Right-click on one of the user's layouts: its name and where the little menu goes.
    layout_ctx: Option<(String, egui::Pos2)>,
    /// Size (egui points) to give the main window at the next frame (a saved layout, #38).
    pending_win: Option<[f32; 2]>,
    applied_active: Option<[[u8; 3]; 2]>,
    /// Reopen the preferences once the file dialog is closed.
    reopen_prefs: bool,
    /// ⚙ Interface › « Choisir un espace de travail… »: close the Préférences, open the workspace window
    prefs_to_welcome: bool,
    /// After the version question, open the workspace window.
    pending_welcome: bool,
    /// « Écran couleur » with no card plugged: where the choice pops up, and the frame it opened.
    color_pick: Option<(egui::Pos2, u64)>,
    /// The toolbar's left groups are wider than the window (show « » »).
    tb_overflow: bool,
    /// « » » scrolls the toolbar to the Cassette group; « « » scrolls back.
    tb_to_end: bool,
    /// Width of the pinned right-hand toolbar buttons.
    right_w: f32,
    /// Tool window that last had the pointer or the keyboard (text size).
    font_hot: Option<crate::fontsz::Win>,
    /// Screen of the start-up guide (0..2).
    wiz_page: u8,
    selected: Option<PathBuf>,
    /// Library folder to remove once the library tree has been drawn (removing it while
    /// drawing shrinks `lib.roots` under the `for ri in 0..len` loop: index out of bounds).
    pending_lib_remove: Option<PathBuf>,
    loading: Option<Loading>,
    /// Last library program put in memory (path, modification time).
    loaded: Option<(PathBuf, Option<SystemTime>)>,
    keys_to_emu: bool,
    frame_no: u64,
    start: Instant,
    script: Script,
    speed: (Instant, u64, f64),
    last_save_ranges: String,
    cfg_dirty: Option<Instant>,
    screen_rect: egui::Rect,
    screen_rect2: egui::Rect,
    /// Mouse selection on a screen, and the zoom really used by each screen.
    scr_sel: Option<crate::screen::ScreenSel>,
    eff_zoom: [f32; 2],
    /// Width the full toolbar needs (measured), for its language.
    bar_need: (Lang, f32),
    /// Source line the asm highlight was last computed for.
    hl_line: Option<usize>,
    pref_tab: PrefTab,
    /// Theme and interface zoom applied (to set them when they change).
    applied_look: Option<(crate::config::Theme, f32)>,
    /// Window width the default layout was made for; while `lay_pending`
    /// (the first ~1.5 s, until a click) it is remade if the window grew
    /// (maximized by the window manager after the first frame).
    lay_w: f32,
    lay_pending: bool,
    /// --debogueur was given: the first layout keeps the debugger even on a small window
    dbg_forced: bool,
    /// The saved tree as loaded: the window opens small then grows, and egui_dock writes the
    /// fractions it had to clamp for the small width back into the tree, so the saved
    /// layout came back distorted. Put back while the window size settles.
    lay_saved: Option<egui_dock::DockState<Pane>>,
    /// Width of the window in points, to size the panes (see `layout`).
    win_w: f32,
    /// Width of the monitor in points (0: not known yet)
    mon_w: f32,
    /// Separate windows of the panes (geometry they were opened with).
    wins: std::collections::BTreeMap<Pane, crate::dock::DockState>,
    /// The saved layout as last written (to see when it changes).
    layout_json: String,
    /// GitHub projects: network worker, cache, last download, progress.
    gh: github::Worker,
    gh_cache: PathBuf,
    gh_meta: std::collections::HashMap<String, Meta>,
    gh_status: std::collections::HashMap<String, (String, f32)>,
    /// Cassette sound: None = not opened yet, Some(None) = no audio device.
    snd: Option<Option<crate::audio::Audio>>,
    /// Last tape time sent to the sound (writing?, seconds).
    snd_t: Option<(bool, f64)>,
    snd_lp: f32,
    /// « Éditeur BASIC » window.
    ed: crate::editor::Editor,
    ascii: crate::ascii::AsciiTool,
    dbg: crate::debugger::DebugUi,
}

/// Where a source line of the editor runs.
enum LineTarget {
    Addr(u16),
    Basic(u16),
}

/// What the library tree needs while it is drawn.
struct Cx {
    entries: Vec<Entry>,
    progress: Option<(f32, bool, bool)>,
    loading: Option<PathBuf>,
    tape_src: Option<PathBuf>,
}

/// Tooltip of a control: the short one, or in help mode its explanation
/// (bigger text, translated, readable on high DPI screens).
/// « Bulles d'aide » of the preferences: off = no short tooltips (help
/// mode still explains).
static TOOLTIPS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

pub(crate) fn set_tooltips(on: bool) {
    TOOLTIPS.store(on, std::sync::atomic::Ordering::Relaxed);
}

pub(crate) trait Hint {
    fn hint(self, help: bool, short: &str, key: &str) -> Self;
}

impl Hint for egui::Response {
    fn hint(self, help: bool, short: &str, key: &str) -> Self {
        if help {
            let short = short.to_string();
            self.on_hover_ui(|ui| {
                ui.set_max_width(400.0);
                ui.label(RichText::new(t(key)).size(16.0));
                if !short.is_empty() && short != t(key) {
                    ui.add_space(4.0);
                    ui.label(RichText::new(short).size(13.0).weak());
                }
            })
        } else if short.is_empty() || !TOOLTIPS.load(std::sync::atomic::Ordering::Relaxed) {
            self
        } else {
            self.on_hover_text(short)
        }
    }
}

fn dialog_labels() -> FileDialogLabels {
    if i18n::current() == Lang::En {
        return FileDialogLabels::default();
    }
    FileDialogLabels {
        title_select_directory: t("fd_title_dir").into(),
        title_select_file: t("fd_title_file").into(),
        title_select_multiple: t("fd_title_multi").into(),
        title_save_file: t("fd_title_save").into(),
        cancel: t("fd_cancel").into(),
        overwrite: t("fd_overwrite").into(),
        reload: t("fd_reload").into(),
        working_directory: t("fd_workdir").into(),
        select_all: t("fd_select_all").into(),
        show_hidden: t("fd_hidden").into(),
        show_system_files: t("fd_system").into(),
        heading_pinned: t("fd_pinned").into(),
        heading_places: t("fd_places").into(),
        heading_devices: t("fd_devices").into(),
        heading_removable_devices: t("fd_removable").into(),
        home_dir: t("fd_home").into(),
        desktop_dir: t("fd_desktop").into(),
        documents_dir: t("fd_documents").into(),
        downloads_dir: t("fd_downloads").into(),
        audio_dir: t("fd_music").into(),
        pictures_dir: t("fd_pictures").into(),
        videos_dir: t("fd_videos").into(),
        pin_folder: t("fd_pin").into(),
        unpin_folder: t("fd_unpin").into(),
        rename_pinned_folder: t("fd_rename").into(),
        selected_directory: t("fd_sel_dir").into(),
        selected_file: t("fd_sel_file").into(),
        selected_items: t("fd_sel_items").into(),
        file_name: t("fd_file_name").into(),
        file_filter_all_files: t("fd_all_files").into(),
        save_extension_any: t("fd_any").into(),
        open_button: t("fd_open").into(),
        save_button: t("fd_save").into(),
        cancel_button: t("fd_cancel_button").into(),
        overwrite_file_modal_text: t("fd_overwrite_text").into(),
        err_empty_folder_name: t("fd_err_folder").into(),
        err_empty_file_name: t("fd_err_file").into(),
        err_directory_exists: t("fd_err_dir_exists").into(),
        new_folder_button: t("fd_new_folder").into(),
        new_folder_tooltip: t("fd_new_folder_tip").into(),
        err_file_exists: t("fd_err_file_exists").into(),
    }
}

fn file_name(p: &Path) -> String {
    p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default()
}

fn mtime(p: &Path) -> Option<SystemTime> {
    std::fs::metadata(p).and_then(|m| m.modified()).ok()
}

/// Escape sequences of --taper: \n (RETURN), \\ (backslash).
pub fn unescape(s: &str) -> String {
    let mut out = String::new();
    let mut it = s.chars();
    while let Some(c) = it.next() {
        if c == '\\' {
            match it.next() {
                Some('n') | Some('r') => out.push('\r'),
                Some(o) => out.push(o),
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Name of the GEN2 display mode in the current language.
fn mode_name(m: apple1_core::gen2::Modes) -> String {
    let p = if m.page2 { 2 } else { 1 };
    if m.text {
        tf("m_text", &[&p])
    } else {
        let g = if m.hires { "HIRES" } else { "LORES" };
        if m.mixed {
            tf("m_mixed", &[&g, &p])
        } else {
            tf("m_graphics", &[&g, &p])
        }
    }
}

/// Language from the settings ("auto" = detected).
pub fn apply_language(cfg: &GuiConfig, detected: &Detected) {
    i18n::set(Lang::from_code(&cfg.langue).unwrap_or(detected.lang));
}

impl App {
    pub fn new(cc: &eframe::CreationContext, session: Session, cfg: GuiConfig, first_run: bool, script: Script, detected: Detected) -> Self {
        let ctx = &cc.egui_ctx;
        // Ctrl+plus / Ctrl+minus / Ctrl+0 are ours (zoom of the Apple-1 screen).
        ctx.options_mut(|o| o.zoom_with_keyboard = false);
        ctx.set_visuals(egui::Visuals::dark());
        // light theme: the selected buttons (white text on a strong blue, not
        // dark text on pale blue) and links readable
        ctx.style_mut_of(egui::Theme::Light, |s| {
            s.visuals.selection.bg_fill = Color32::from_rgb(0, 92, 170);
            s.visuals.selection.stroke.color = Color32::WHITE;
            s.visuals.hyperlink_color = Color32::from_rgb(0, 70, 160);
        });
        apply_language(&cfg, &detected);
        let mut cfg = cfg;
        if first_run {
            cfg.kit_version = crate::config::KIT_VERSION.into();
        }
        let offer_fresh = script.fresh || (!first_run && cfg.kit_version != crate::config::KIT_VERSION && !script.automated());
        let show_welcome = (!cfg.welcome_hide && !script.automated()) || script.welcome;
        let mut message = t("welcome").to_string();
        if !cfg.library.exists() {
            match library::populate(&cfg.library) {
                Ok(n) => message = format!("{} {message}", tf("lib_created", &[&cfg.library.display(), &n])),
                Err(e) => message = tf("lib_create_failed", &[&e]),
            }
        }
        let lib = Library::new(cfg.library.clone());
        let gh_cache = github::cache_root();
        let mut app = App {
            gh: github::Worker::new(std::sync::Arc::new(github::Ureq::new()), gh_cache.clone()),
            gh_cache,
            gh_meta: Default::default(),
            gh_status: Default::default(),
            snd: None,
            snd_t: None,
            snd_lp: 0.0,
            ed: crate::editor::Editor::from_cfg(&cfg.editor),
            ascii: Default::default(),
            wins: Default::default(),
            layout_json: String::new(),
            dbg: Default::default(),
            s: {
                let mut session = session;
                session.dbg.breaks = cfg.debugger.breaks.iter().copied().collect();
                session.dbg.basic_breaks = cfg.debugger.basic_breaks.iter().copied().collect();
                session
            },
            first_run,
            lib,
            detected,
            tex: None,
            drawn: None,
            tex2: None,
            tex_plab: None,
            drawn2: None,
            raw: None,
            raw2: None,
            crt: crt::Crt::default(),
            crt2: crt::Crt::default(),
            tex_bake: None,
            bake_key: None,
            crt_cfg: None,
            crt_t: Instant::now(),
            blink: CursorBlink::apple1(),
            message,
            error: false,
            dialog: None,
            eprom: crate::eprom_ui::EpromUi::default(),
            state_btn: egui::Rect::NOTHING,
            ask: if offer_fresh { Some(Ask::Fresh { previous: cfg.kit_version.clone() }) } else if show_welcome { Some(Ask::Welcome) } else { None },
            layout_ctx: None,
            pending_win: None,
            applied_active: None,
            reopen_prefs: false,
            prefs_to_welcome: false,
            pending_welcome: offer_fresh && show_welcome,
            color_pick: None,
            tb_overflow: false,
            tb_to_end: script.bar_end,
            font_hot: None,
            wiz_page: script.wizard_page.unwrap_or(0),
            right_w: 150.0,
            selected: None,
            pending_lib_remove: None,
            loading: None,
            loaded: None,
            keys_to_emu: true,
            frame_no: 0,
            start: Instant::now(),
            speed: (Instant::now(), 0, 0.0),
            last_save_ranges: "300.3FF".into(),
            cfg_dirty: None,
            screen_rect: egui::Rect::NOTHING,
            screen_rect2: egui::Rect::NOTHING,
            scr_sel: None,
            eff_zoom: [1.0; 2],
            bar_need: (Lang::En, 0.0),
            hl_line: None,
            pref_tab: PrefTab::Interface,
            applied_look: None,
            lay_w: 0.0,
            lay_pending: false,
            dbg_forced: false,
            lay_saved: None,
            win_w: 1366.0,
            mon_w: 0.0,
            cfg,
            script,
        };
        app.extras();
        app.refresh_roots();
        // projects never downloaded (the others are updated on demand)
        for s in app.cfg.github.clone() {
            if let Ok(spec) = Spec::parse(&s) {
                if !app.gh_meta.contains_key(&s) {
                    app.gh_status.insert(s.clone(), (t("gh_waiting").into(), 0.0));
                    app.gh.send(github::Job::Sync(spec), app.cfg.token());
                }
            }
        }
        if app.lib.migrated > 0 {
            app.message = format!("{} {}", tf("lib_migrated", &[&app.lib.migrated]), app.message);
        }
        if let Some(z) = app.script.zoom {
            app.cfg.zoom = z;
        }
        app.s.fast = app.cfg.fast;
        if app.s.m.board.config.display_throttle != app.cfg.throttle {
            app.s.toggle_throttle();
        }
        app
    }

    pub fn set_blink(&mut self, b: CursorBlink) {
        self.blink = b;
    }

    fn info(&mut self, m: impl Into<String>) {
        self.message = m.into();
        self.error = false;
    }

    fn fail(&mut self, e: impl std::fmt::Display) {
        self.message = tf("s_error", &[&e]);
        self.error = true;
    }

    fn result(&mut self, r: Result<String, String>) {
        match r {
            Ok(m) => self.info(m),
            Err(e) => self.fail(e),
        }
    }

    fn dirty(&mut self) {
        self.cfg_dirty = Some(Instant::now());
    }

    // ---------------------------------------------------------------- files

    fn open_dialog(&mut self, op: Op) {
        let start_dir = match &op {
            Op::Load | Op::InsertTape | Op::Basic | Op::EdOpen | Op::PlabRom => self.cfg.last_dir.clone().filter(|d| d.is_dir()).unwrap_or_else(|| self.cfg.library.clone()),
            Op::StateSave | Op::StateLoad => self.states_dir(),
            // next to the source (#33): the WAV or the new name stays in its folder tree
            Op::EdSave(uid) | Op::EdExport(uid) => self.ed.tab(*uid).and_then(|t| t.path.as_ref().and_then(|p| p.parent().map(Path::to_path_buf))).filter(|d| d.is_dir()).unwrap_or_else(|| self.cfg.library.clone()),
            Op::LibraryDir | Op::AddLibrary => self.cfg.library.parent().map(Path::to_path_buf).unwrap_or_else(home),
            _ => self.cfg.library.clone(),
        };
        let start_dir = if start_dir.is_dir() { start_dir } else { home() };
        let mut d = FileDialog::new()
            .id(egui::Id::new("dialogue-fichier"))
            .labels(dialog_labels())
            .initial_directory(start_dir)
            .default_size([820.0, 520.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]);
        match &op {
            Op::Load => {
                let f = t("fd_filter_programs");
                d = d.title(t("fd_load_title")).add_file_filter_extensions(f, vec!["txt", "apl", "bin", "hex"]).default_file_filter(f);
                d.pick_file();
            }
            Op::PlabRom => {
                let f = t("fd_filter_programs");
                d = d.title(t("plab_rom")).add_file_filter_extensions(f, vec!["bin", "rom"]).default_file_filter(f);
                d.pick_file();
            }
            Op::InsertTape => {
                let f = t("fd_filter_tapes");
                d = d.title(t("fd_insert_title")).add_file_filter_extensions(f, vec!["wav", "aiff", "aif"]).default_file_filter(f);
                d.pick_file();
            }
            Op::Basic => {
                let f = t("fd_filter_basic");
                d = d.title(t("fd_basic_title")).add_file_filter_extensions(f, vec!["wav", "aiff", "aif", "txt", "apl", "hex", "bin"]).default_file_filter(f);
                d.pick_file();
            }
            Op::Save(_) => {
                d = d
                    .title(t("fd_save_title"))
                    .add_save_extension(t("fd_ext_wav"), "wav")
                    .add_save_extension(t("fd_ext_aiff"), "aiff")
                    .add_save_extension(t("fd_ext_txt"), "txt")
                    .add_save_extension(t("fd_ext_bin"), "bin")
                    .default_save_extension(t("fd_ext_wav"))
                    .default_file_name("programme.wav");
                d.save_file();
            }
            Op::Record => {
                d = d
                    .title(t("fd_record_title"))
                    .add_save_extension(t("fd_ext_wav"), "wav")
                    .add_save_extension(t("fd_ext_aiff"), "aiff")
                    .default_save_extension(t("fd_ext_wav"))
                    .default_file_name("enregistrement.wav");
                d.save_file();
            }
            Op::LibraryDir => {
                d = d.title(t("fd_library_title"));
                d.pick_directory();
            }
            Op::AddLibrary => {
                d = d.title(t("fd_add_library_title"));
                d.pick_directory();
            }
            Op::EdOpen => {
                let all = t("fd_filter_ed_all");
                d = d
                    .title(t("fd_ed_open_title"))
                    .add_file_filter_extensions(all, vec!["bas", "txt", "s", "asm", "a65", "inc", "ca65"])
                    .add_file_filter_extensions(t("fd_filter_bas"), vec!["bas", "txt"])
                    .add_file_filter_extensions(t("fd_filter_asm"), vec!["s", "asm", "a65", "inc", "ca65"])
                    .default_file_filter(all);
                d.pick_file();
            }
            Op::EdSave(uid) => {
                let asm = self.ed.tab(*uid).map(|t| t.lang == crate::editor::SrcLang::Asm).unwrap_or(false);
                let (fx, ext) = if asm { (t("fd_ext_asm"), "s") } else { (t("fd_ext_bas"), "bas") };
                let name = self.ed.tab(*uid).and_then(|t| t.path.as_ref().map(|p| file_name(p))).unwrap_or_else(|| format!("programme.{ext}"));
                d = d.title(t("fd_ed_save_title")).add_save_extension(fx, ext).default_save_extension(fx).default_file_name(&name);
                d.save_file();
            }
            Op::AsciiOpen => {
                let f = t("fd_filter_images");
                d = d.title(t("fd_ascii_title")).add_file_filter_extensions(f, vec!["jpg", "jpeg", "png", "txt"]).default_file_filter(f);
                d.pick_file();
            }
            Op::StateSave => {
                let fx = t("fd_ext_state");
                d = d.title(t("fd_state_save_title")).add_save_extension(fx, "a1state").default_save_extension(fx).default_file_name("etat.a1state");
                d.save_file();
            }
            Op::StateLoad => {
                let f = t("fd_ext_state");
                d = d.title(t("fd_state_load_title")).add_file_filter_extensions(f, vec!["a1state"]).default_file_filter(f);
                d.pick_file();
            }
            Op::EpromFile => {
                d = d.title(t("ep_title"));
                d.pick_file();
            }
            Op::EdExport(uid) => {
                let stem = self.ed.tab(*uid).and_then(|t| t.path.as_ref().and_then(|p| p.file_stem()).map(|s| s.to_string_lossy().to_string())).unwrap_or_else(|| "programme".into());
                d = d
                    .title(t("fd_ed_export_title"))
                    .add_save_extension(t("fd_ext_wav"), "wav")
                    .add_save_extension(t("fd_ext_aiff"), "aiff")
                    .default_save_extension(t("fd_ext_wav"))
                    .default_file_name(&format!("{stem}.wav"));
                d.save_file();
            }
        }
        self.dialog = Some((d, op));
    }

    fn picked(&mut self, op: Op, path: PathBuf) {
        if let Some(parent) = path.parent() {
            if !matches!(op, Op::LibraryDir | Op::AddLibrary) {
                self.cfg.last_dir = Some(parent.to_path_buf());
                self.dirty();
            }
        }
        match op {
            Op::Load => self.load_path(&path, false),
            Op::PlabRom => self.set_plab_rom(Some(path)),
            Op::InsertTape => {
                let r = self.s.insert_tape(&path);
                self.result(r);
            }
            Op::Basic => {
                let r = self.s.load_basic(&path);
                if r.is_ok() {
                    self.cfg.basic = Some(path);
                    self.cfg.basic_g2 = false;
                    self.dirty();
                }
                self.result(r);
            }
            Op::Save(ranges) => {
                let path = if path.extension().is_none() { path.with_extension("wav") } else { path };
                let r = self.s.save(&path, &ranges);
                self.result(r);
                self.lib.scan();
            }
            Op::Record => {
                let path = if path.extension().is_none() { path.with_extension("wav") } else { path };
                let m = self.s.start_recording(&path);
                self.info(m);
            }
            Op::LibraryDir => self.set_library(path),
            Op::AddLibrary => self.add_library(path),
            Op::EdOpen => {
                let r = self.ed.open(&path);
                self.result(r);
            }
            Op::EdSave(uid) => {
                let asm = self.ed.tab(uid).map(|t| t.lang == crate::editor::SrcLang::Asm).unwrap_or(false);
                let path = if path.extension().is_none() { path.with_extension(if asm { "s" } else { "bas" }) } else { path };
                let r = self.ed.save(uid, Some(path));
                self.result(r);
                self.lib.scan();
            }
            Op::EdExport(uid) => self.editor_export(uid, path),
            Op::AsciiOpen => self.ascii_open(&path),
            Op::StateSave => {
                let path = if path.extension().is_none() { path.with_extension("a1state") } else { path };
                self.save_state_file(&path);
            }
            Op::StateLoad => self.load_state_file(&path),
            Op::EpromFile => self.eprom.file = path.display().to_string(),
        }
    }

    // --------------------------------------------------------- machine state

    /// Folder proposed for the .a1state files (next to the library).
    fn states_dir(&self) -> PathBuf {
        let d = self.cfg.library.parent().map(|p| p.join("etats")).unwrap_or_else(home);
        let _ = std::fs::create_dir_all(&d);
        if d.is_dir() { d } else { home() }
    }

    fn remember_state(&mut self, path: &Path, title: &str) {
        let when = std::process::Command::new("date").arg("+%d/%m %H:%M").output().ok().filter(|o| o.status.success()).map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default();
        self.cfg.states.retain(|s| s.path != path);
        self.cfg.states.insert(0, crate::config::RecentState { title: title.to_string(), path: path.to_path_buf(), when });
        self.cfg.states.truncate(8);
        self.dirty();
    }

    fn save_state_file(&mut self, path: &Path) {
        let title = path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
        let r = self.s.save_state(path, &title);
        if r.is_ok() {
            self.remember_state(path, &title);
        }
        self.result(r);
    }

    fn load_state_file(&mut self, path: &Path) {
        if !path.exists() {
            self.cfg.states.retain(|s| s.path != path);
            self.dirty();
            self.result(Err(tf("st_missing", &[&path.display()])));
            return;
        }
        match self.s.load_state(path) {
            Ok((msg, info)) => {
                // the settings follow the machine that was loaded
                self.cfg.carte_couleur = info.color_card;
                self.cfg.couleur_50hz = self.s.m.board.config.color_50hz;
                self.loading = None;
                let title = if info.title.is_empty() { path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default() } else { info.title.clone() };
                self.remember_state(path, &title);
                self.info(msg);
            }
            Err(e) => self.result(Err(e)),
        }
    }

    /// Shortcuts of the machine state, the menu forced open (screenshots) and
    /// the EPROM programmer window.
    fn state_ui(&mut self, ctx: &egui::Context) {
        let cs = egui::Modifiers::COMMAND | egui::Modifiers::SHIFT;
        if ctx.input_mut(|i| i.consume_key(cs, egui::Key::S)) {
            self.open_dialog(Op::StateSave);
        }
        if ctx.input_mut(|i| i.consume_key(cs, egui::Key::O)) {
            self.open_dialog(Op::StateLoad);
        }
        if self.script.state_menu {
            let at = self.state_btn.left_bottom() + egui::vec2(0.0, 3.0);
            egui::Area::new(egui::Id::new("menu-etat-script")).order(egui::Order::Foreground).fixed_pos(at).show(ctx, |ui| {
                egui::Frame::menu(ui.style()).show(ui, |ui| {
                    self.state_menu_items(ui);
                });
            });
        }
        if self.script.dialog {
            self.script.dialog = false;
            self.open_dialog(Op::Load);
        }
        if self.script.eprom {
            self.script.eprom = false;
            self.eprom.show_open();
        }
        let s = &self.s.m.board;
        let req = self.eprom.show(ctx, self.cfg.help_mode, &|a| s.peek(a));
        if req.browse {
            self.open_dialog(Op::EpromFile);
        }
        if self.eprom.finished {
            self.info(t("s_eprom_done"));
        }
    }

    /// Items of the « État » menu; true = close the menu.
    fn state_menu_items(&mut self, ui: &mut egui::Ui) -> bool {
        let h = self.cfg.help_mode;
        let mut close = false;
        ui.set_min_width(260.0);
        if ui.add(egui::Button::new(t("st_save")).shortcut_text(format!("Ctrl+{}+S", t("st_shift")))).hint(h, t("st_save_tip"), "h_tb_state").clicked() {
            self.open_dialog(Op::StateSave);
            close = true;
        }
        if ui.add(egui::Button::new(t("st_load")).shortcut_text(format!("Ctrl+{}+O", t("st_shift")))).hint(h, t("st_load_tip"), "h_tb_state").clicked() {
            self.open_dialog(Op::StateLoad);
            close = true;
        }
        ui.separator();
        ui.label(RichText::new(t("st_recent")).weak());
        if self.cfg.states.is_empty() {
            ui.label(RichText::new(t("st_none")).small().weak());
        }
        for st in self.cfg.states.clone() {
            let label = if st.when.is_empty() { format!("🕘 {}", st.title) } else { format!("🕘 {} — {}", st.title, st.when) };
            if ui.button(label).on_hover_text(shown_path(&st.path)).clicked() {
                self.load_state_file(&st.path);
                close = true;
            }
        }
        close
    }

    // ---------------------------------------------------------- BASIC editor

    fn editor_pane(&mut self, ui: &mut egui::Ui) {
        let mut acts = Vec::new();
        let h = self.cfg.help_mode;
        self.ed.dbg = self.editor_dbg();
        self.dbg.resolve_pending(&mut self.s);
        if let Some(q) = self.script.editor_find.take() {
            self.ed.find.open = true;
            self.ed.find.query = q;
        }
        self.ed.force_menu = self.script.editor_menu;
        self.ed.contents(ui, h, &mut acts);
        for a in acts {
            match a {
                crate::editor::EdAction::Open => self.open_dialog(Op::EdOpen),
                crate::editor::EdAction::SaveAs(uid) => self.open_dialog(Op::EdSave(uid)),
                crate::editor::EdAction::Run => self.editor_run(),
                crate::editor::EdAction::Export => {
                    if let Some(uid) = self.editor_built().map(|(uid, _, _)| uid) {
                        self.open_dialog(Op::EdExport(uid));
                    }
                }
                crate::editor::EdAction::BreakAddr(a) => crate::debugger::DebugUi::toggle_break(&mut self.s, a),
                crate::editor::EdAction::ShowDisasm(l) => match self.line_target(l) {
                    Some(LineTarget::Addr(a)) => {
                        self.set_pane_wanted(Pane::Debugger, true);
                        self.set_pane_wanted(Pane::Disasm, true);
                        self.dbg.show_disasm(a);
                    }
                    _ => self.fail(tf("db_no_code", &[&l])),
                },
                crate::editor::EdAction::BreakLine(l) => match self.line_target(l) {
                    Some(LineTarget::Addr(a)) => {
                        self.set_pane_wanted(Pane::Debugger, true);
                        crate::debugger::DebugUi::toggle_break(&mut self.s, a)
                    }
                    Some(LineTarget::Basic(n)) => {
                        self.set_pane_wanted(Pane::Debugger, true);
                        crate::debugger::DebugUi::toggle_basic_break(&mut self.s, n)
                    }
                    None => self.fail(tf("db_no_code", &[&l])),
                },
                crate::editor::EdAction::RunToLine(l) => match self.line_target(l) {
                    Some(LineTarget::Addr(a)) => self.dbg.command(&mut self.s, crate::debugger::Cmd::To(a)),
                    Some(LineTarget::Basic(n)) => {
                        self.dbg.command(&mut self.s, crate::debugger::Cmd::Continue);
                        self.s.dbg.go(apple1_core::debug::Mission::BasicTo(n));
                    }
                    None => self.fail(tf("db_no_code", &[&l])),
                },
                crate::editor::EdAction::Info(m) => self.info(m),
                crate::editor::EdAction::Fail(m) => {
                    self.message = m;
                    self.error = true;
                }
            }
        }
    }

    /// What the debugger shows in the current tab: line being run,
    /// breakpoints.
    fn editor_dbg(&mut self) -> crate::editor::EdDbg {
        use crate::editor::{EdDbg, SrcLang};
        if !self.cfg.debugger.open {
            return EdDbg::default();
        }
        let pc = self.s.m.cpu.pc;
        let paused = self.s.dbg.paused;
        let peek = |a: u16| self.s.m.board.peek(a);
        let bline = if apple1_core::debug::basic_present(&peek) { self.s.dbg.basic_line.or_else(|| apple1_core::debug::basic_current_line(&peek)) } else { None };
        let mut d = EdDbg { on: true, pc: Some(pc), ..Default::default() };
        let now = (pc, bline);
        d.scroll = paused && self.dbg.seen != Some(now);
        self.dbg.seen = Some(now);
        let (breaks, bbreaks) = (self.s.dbg.breaks.clone(), self.s.dbg.basic_breaks.clone());
        let Some(tab) = self.ed.current() else { return d };
        match tab.lang {
            SrcLang::Basic => {
                let l = tab.listing();
                d.cur = bline.and_then(|n| l.lines.iter().find(|x| x.number == n)).map(|x| x.src_line);
                d.marks = l.lines.iter().filter(|x| bbreaks.contains(&x.number)).map(|x| x.src_line).collect();
            }
            SrcLang::Asm => {
                if let Ok(a) = &*tab.assembly() {
                    if self.dbg.ed_syms != a.symbols {
                        self.dbg.ed_syms = a.symbols.clone();
                    }
                    let code = |x: &&apple1_core::asm6502::ListLine| x.file == 0 && !x.bytes.is_empty();
                    d.cur = a.listing.iter().filter(code).find(|x| x.addr == pc).map(|x| x.line);
                    d.marks = a.listing.iter().filter(code).filter(|x| breaks.contains(&x.addr)).map(|x| x.line).collect();
                    d.addrs = breaks.into_iter().collect();
                }
            }
        }
        d
    }

    /// Address (6502) or BASIC line number of a source line of the current
    /// tab: that line, or the next one that produces code.
    fn line_target(&mut self, line: usize) -> Option<LineTarget> {
        use crate::editor::SrcLang;
        let tab = self.ed.current()?;
        match tab.lang {
            SrcLang::Basic => tab.listing().lines.iter().filter(|x| x.src_line >= line).min_by_key(|x| x.src_line).map(|x| LineTarget::Basic(x.number)),
            SrcLang::Asm => match &*tab.assembly() {
                Ok(a) => a.listing.iter().filter(|x| x.file == 0 && !x.bytes.is_empty() && x.line >= line).min_by_key(|x| x.line).map(|x| LineTarget::Addr(x.addr)),
                Err(_) => None,
            },
        }
    }

    // ---------------------------------------------------------- debugger

    // ---------------------------------------------------------- panes

    /// Is the pane open?
    fn pane_wanted(&self, p: Pane) -> bool {
        match p {
            Pane::Editor => self.cfg.editor.open,
            Pane::Screen => self.cfg.screen_open,
            Pane::Color => self.color_card() && self.cfg.color_open,
            Pane::Plab => self.plab_card() && self.cfg.plab_open,
            Pane::Debugger => self.cfg.debugger.open,
            Pane::Disasm => self.cfg.debugger.disasm.open,
            Pane::Dump => self.cfg.debugger.dump_win.open,
            Pane::Library => self.cfg.show_library,
            Pane::Ascii => self.cfg.ascii.open,
        }
    }

    fn set_pane_wanted(&mut self, p: Pane, on: bool) {
        match p {
            Pane::Editor => {
                self.cfg.editor.open = on;
                self.ed.changed = true;
                if !on {
                    self.ed.focused = false;
                }
            }
            Pane::Screen => self.cfg.screen_open = on,
            Pane::Color => self.cfg.color_open = on,
            Pane::Plab => self.cfg.plab_open = on,
            Pane::Debugger => {
                self.cfg.debugger.open = on;
                if !on {
                    self.s.dbg.cont();
                }
            }
            Pane::Disasm => self.cfg.debugger.disasm.open = on,
            Pane::Dump => self.cfg.debugger.dump_win.open = on,
            Pane::Library => self.cfg.show_library = on,
            Pane::Ascii => self.cfg.ascii.open = on,
        }
        self.dirty();
    }

    /// Separate window or not, and its geometry.
    fn pane_dock(&mut self, p: Pane) -> &mut crate::dock::DockCfg {
        match p {
            Pane::Editor => &mut self.cfg.editor_dock,
            Pane::Screen => &mut self.cfg.screen_dock,
            Pane::Color => &mut self.cfg.color_dock,
            Pane::Plab => &mut self.cfg.plab_dock,
            Pane::Debugger => &mut self.cfg.debugger.dock,
            Pane::Disasm => &mut self.cfg.debugger.disasm.dock,
            Pane::Dump => &mut self.cfg.debugger.dump_win.dock,
            Pane::Library => &mut self.cfg.library_dock,
            Pane::Ascii => &mut self.cfg.ascii.dock,
        }
    }

    /// Default layout: every tool open and docked in the main window.
    fn reset_layout(&mut self) {
        for p in Pane::ALL {
            self.pane_dock(p).detached = false;
        }
        self.cfg.editor.open = true;
        self.ed.changed = true;
        self.cfg.debugger.open = true;
        self.cfg.debugger.disasm.open = true;
        self.cfg.debugger.dump_win.open = true;
        self.cfg.show_library = true;
        self.cfg.layout = Some(crate::layout::default_layout(self.color_card(), self.cfg.library_right, self.first_run_width()));
        self.cfg.layout_version = crate::layout::VERSION;
        if self.frame_no <= 3 {
            self.lay_w = self.win_w;
            self.lay_pending = true;
        }
        self.dirty();
    }

    /// Panes open now: editor, debugger, disassembly, dump, library, ASCII.
    fn open_panes(&self) -> [bool; 6] {
        [Pane::Editor, Pane::Debugger, Pane::Disasm, Pane::Dump, Pane::Library, Pane::Ascii].map(|p| self.pane_wanted(p))
    }

    /// Open exactly these panes, docked in the main window, in this tree.
    fn apply_layout(&mut self, open: [bool; 6], tree: egui_dock::DockState<Pane>) {
        for p in Pane::ALL {
            self.pane_dock(p).detached = false;
        }
        for (p, on) in [Pane::Editor, Pane::Debugger, Pane::Disasm, Pane::Dump, Pane::Library, Pane::Ascii].into_iter().zip(open) {
            self.set_pane_wanted(p, on);
        }
        self.cfg.layout = Some(tree);
        self.cfg.layout_version = crate::layout::VERSION;
        self.dirty();
    }

    /// Width the very first layout is made for. The window opens small and is maximized a few
    /// frames later: judging it by its first size gave the editor + screen layout even on a
    /// 2560-point monitor, so the monitor counts as soon as it is known.
    fn first_run_width(&self) -> f32 {
        width_for_first_layout(self.frame_no, self.script.window.is_some(), self.mon_w, self.win_w)
    }

    fn preset_names() -> Vec<String> {
        crate::layout::Preset::ALL.iter().map(|p| t(p.title_key()).to_string()).collect()
    }

    /// « ⊞ Disposition »: a provided layout, one of the user's, or saving / renaming / deleting.
    fn layout_act(&mut self, a: LayoutAct) {
        use crate::layout::Preset;
        match a {
            LayoutAct::Preset(pr) => {
                if pr == Preset::Default {
                    self.reset_layout();
                } else {
                    let tree = crate::layout::preset_layout(pr, self.color_card(), self.cfg.library_right, self.win_w);
                    self.apply_layout(pr.open(), tree);
                }
                self.cfg.layout_choice = pr.key().into();
                self.info(tf("ly_applied", &[&t(pr.title_key())]));
            }
            LayoutAct::Saved(name) => {
                if let Some(l) = self.cfg.saved_layouts.iter().find(|l| l.name == name).cloned() {
                    if let Some(tree) = l.tree {
                        self.apply_layout(l.open, tree);
                        // the windows that were detached come back where they were
                        self.wins.clear();
                        for (p, d) in &l.detached {
                            self.set_pane_wanted(*p, true);
                            *self.pane_dock(*p) = d.clone();
                            self.pane_dock(*p).detached = true;
                        }
                        if !self.cfg.maximized {
                            self.pending_win = l.window;
                        }
                        self.cfg.layout_choice = format!("u:{name}");
                        self.info(tf("ly_applied", &[&name]));
                    }
                }
            }
            LayoutAct::Welcome => {
                self.wiz_page = 0;
                self.ask = Some(Ask::Welcome);
            },
            LayoutAct::Context(name, pos) => self.layout_ctx = Some((name, pos)),
            LayoutAct::Save => self.ask = Some(Ask::LayoutName { text: String::new(), err: None, old: None }),
            LayoutAct::Rename(old) => self.ask = Some(Ask::LayoutName { text: old.clone(), err: None, old: Some(old) }),
            LayoutAct::Delete(name) => {
                self.cfg.saved_layouts.retain(|l| l.name != name);
                if self.cfg.layout_choice == format!("u:{name}") {
                    self.cfg.layout_choice.clear();
                }
                self.dirty();
                self.info(tf("ly_deleted", &[&name]));
            }
        }
    }

    /// Keep the current layout under `name` (saving), or rename `old` to it.
    fn layout_name_done(&mut self, name: &str, old: Option<&str>) -> Result<(), String> {
        let name = name.trim().to_string();
        if !crate::layout::name_ok(&name, &Self::preset_names()) {
            return Err(t("ly_bad_name").into());
        }
        match old {
            Some(old) => {
                if name != old && self.cfg.saved_layouts.iter().any(|l| l.name.eq_ignore_ascii_case(&name)) {
                    return Err(t("ly_taken").into());
                }
                if let Some(l) = self.cfg.saved_layouts.iter_mut().find(|l| l.name == old) {
                    l.name = name.clone();
                }
                if self.cfg.layout_choice == format!("u:{old}") {
                    self.cfg.layout_choice = format!("u:{name}");
                }
                self.info(tf("ly_renamed", &[&name]));
            }
            None => {
                let tree = self.cfg.layout.clone().ok_or_else(|| t("ly_bad_name").to_string())?;
                let mut detached: Vec<(Pane, crate::dock::DockCfg)> = Vec::new();
                for p in Pane::ALL {
                    if self.pane_wanted(p) && self.pane_dock(p).detached {
                        detached.push((p, self.pane_dock(p).clone()));
                    }
                }
                self.cfg.saved_layouts.retain(|l| !l.name.eq_ignore_ascii_case(&name));
                self.cfg.saved_layouts.push(crate::layout::SavedLayout { name: name.clone(), open: self.open_panes(), tree: Some(tree), detached, window: self.cfg.window });
                self.cfg.layout_choice = format!("u:{name}");
                self.info(tf("ly_saved", &[&name]));
            }
        }
        self.dirty();
        Ok(())
    }

    /// The tree in line with what is open and detached; debugger refreshed,
    /// its breakpoints saved.
    fn layout_sync(&mut self) {
        if self.cfg.layout.is_none() || self.cfg.layout_version != crate::layout::VERSION {
            // first run, or a gui.json of v7.5 or before
            self.reset_layout();
            let w = self.first_run_width();
            // (a size given on the command line keeps the 7.9.1 threshold: scripted captures)
            let limit = if self.script.window.is_some() { 1200.0 } else { crate::layout::SMALL_WINDOW };
            if w > 1.0 && w < limit {
                // five panes would leave the Apple-1 screen a few dozen points:
                // editor + screen on a small window (the Layout menu has the others)
                let tree = crate::layout::preset_layout(crate::layout::Preset::Basic, self.color_card(), self.cfg.library_right, w);
                self.apply_layout(crate::layout::Preset::Basic.open(), tree);
                self.cfg.layout_choice = crate::layout::Preset::Basic.key().into();
                if self.dbg_forced {
                    self.cfg.debugger.open = true;
                }
            }
        }
        let mut st = self.cfg.layout.take().unwrap_or_else(|| crate::layout::default_layout(false, true, self.win_w));
        for p in Pane::ALL {
            let want = self.pane_wanted(p) && !self.pane_dock(p).detached;
            let inside = crate::layout::contains(&st, p);
            if want && !inside {
                crate::layout::insert(&mut st, p, self.cfg.library_right);
            } else if !want && inside {
                crate::layout::remove(&mut st, p);
            }
        }
        self.cfg.layout = Some(st);
        // set again by the editor when it is drawn with the focus
        self.ed.focused = false;
        if self.cfg.debugger.open {
            let (dbg, s, cfg) = (&mut self.dbg, &mut self.s, &mut self.cfg.debugger);
            dbg.refresh(s);
            // breakpoints saved in gui.json
            let (b, bb): (Vec<u16>, Vec<u16>) = (s.dbg.breaks.iter().copied().collect(), s.dbg.basic_breaks.iter().copied().collect());
            if b != cfg.breaks || bb != cfg.basic_breaks {
                cfg.breaks = b;
                cfg.basic_breaks = bb;
                self.dirty();
            }
        }
    }

    /// The main window below the toolbar: the dock tree, then the panes in
    /// their own windows.
    fn panes_ui(&mut self, ui: &mut egui::Ui) {
        self.layout_sync();
        let ctx = ui.ctx().clone();
        let mut st = self.cfg.layout.take().unwrap_or_else(|| crate::layout::default_layout(false, true, self.win_w));
        st.translations.tab_context_menu.close_button = t("dk_close_menu").to_string();
        let style = egui_dock::Style::from_egui(ui.style());
        let mut v = Panes { app: self, closed: Vec::new(), detach: Vec::new() };
        egui::CentralPanel::default().frame(egui::Frame::NONE).show(ui, |ui| {
            egui_dock::DockArea::new(&mut st).id(egui::Id::new("disposition")).style(style).show_leaf_collapse_buttons(false).show_leaf_close_all_buttons(false).show_inside(ui, &mut v);
        });
        let Panes { closed, detach, .. } = v;
        self.cfg.layout = Some(st);
        for p in closed {
            self.set_pane_wanted(p, false);
        }
        for p in detach {
            self.pane_dock(p).detached = true;
            self.dirty();
        }
        self.windows_ui(&ctx);
        // saved when it changes (tab moved, split dragged)
        let json = serde_json::to_string(&self.cfg.layout).unwrap_or_default();
        if json != self.layout_json {
            if !self.layout_json.is_empty() {
                self.dirty();
            }
            self.layout_json = json;
        }
        self.editor_sync();
    }

    /// One pane, in the tree or in its own window.
    fn pane_ui(&mut self, ui: &mut egui::Ui, p: Pane, detached: bool) {
        let h = self.cfg.help_mode;
        let rect = ui.max_rect();
        // in a narrow pane the « Détacher » icon (top right, over the content) would hide the
        // end of the first toolbar row (the Run button of the editor, #36): it gets its own row
        if !detached && rect.width() < 460.0 && !matches!(p, Pane::Screen | Pane::Color | Pane::Plab) {
            ui.add_space(22.0);
        }
        if ui.ui_contains_pointer() {
            self.font_hot = match p {
                Pane::Editor => Some(if self.ed.show_listing { crate::fontsz::Win::Listing } else { crate::fontsz::Win::Editor }),
                Pane::Debugger => Some(crate::fontsz::Win::Debugger),
                Pane::Disasm => Some(crate::fontsz::Win::Disasm),
                Pane::Dump => Some(crate::fontsz::Win::Dump),
                Pane::Library => Some(crate::fontsz::Win::Library),
                Pane::Screen | Pane::Color | Pane::Plab | Pane::Ascii => None,
            };
        }
        match p {
            Pane::Editor => {
                self.editor_pane(ui);
                if self.ed.focused {
                    self.font_hot = Some(if self.ed.show_listing { crate::fontsz::Win::Listing } else { crate::fontsz::Win::Editor });
                }
            }
            Pane::Screen | Pane::Color => {
                let color = p == Pane::Color;
                if self.display_row(ui, color, detached) {
                    self.pane_dock(p).detached = true;
                    self.dirty();
                }
                self.screen_ui(ui, color);
                return;
            }
            Pane::Plab => {
                self.plab_pane(ui, detached);
                return;
            }
            Pane::Debugger => {
                if self.dbg.contents(ui, h, &mut self.s, &mut self.cfg.debugger) {
                    self.dirty();
                }
            }
            Pane::Disasm => {
                if self.dbg.disasm_contents(ui, h, &mut self.s, &mut self.cfg.debugger.disasm.font) {
                    self.dirty();
                }
            }
            Pane::Dump => {
                if self.dbg.dump_contents(ui, h, &mut self.s, &mut self.cfg.debugger) {
                    self.dirty();
                }
            }
            Pane::Library => self.library_ui(ui),
            Pane::Ascii => self.ascii_pane(ui),
        }
        if detached {
            return;
        }
        // « Détacher », on top of the pane, top right (the tools: the icon,
        // its tab's menu has the text too; the screens: in their display row)
        let full = t("dk_detach");
        let label = full.split_whitespace().next().unwrap_or("⧉").to_string();
        let strip = egui::Rect::from_min_max(egui::pos2(rect.min.x, rect.min.y + 3.0), egui::pos2(rect.max.x - 4.0, rect.min.y + 24.0));
        let mut go = false;
        ui.scope_builder(egui::UiBuilder::new().max_rect(strip).layout(egui::Layout::right_to_left(egui::Align::Min)), |ui| {
            go = ui.add(egui::Button::new(label).small()).hint(h, t("dk_detach_tip"), "h_dk_detach").clicked();
        });
        if go {
            self.pane_dock(p).detached = true;
            self.dirty();
        }
    }

    /// The panes detached in their own OS window.
    fn windows_ui(&mut self, ctx: &egui::Context) {
        let h = self.cfg.help_mode;
        for p in Pane::ALL {
            if !self.pane_wanted(p) || !self.pane_dock(p).detached {
                if let Some(w) = self.wins.get_mut(&p) {
                    w.docked();
                }
                continue;
            }
            let mut st = self.wins.remove(&p).unwrap_or_default();
            let mut dcfg = self.pane_dock(p).clone();
            let title = p.title();
            let screen = matches!(p, Pane::Screen | Pane::Color | Pane::Plab);
            let mut int_changed = false;
            let out = crate::dock::window(ctx, p.id(), &title, &mut dcfg, &mut st, |ui, dcfg, out| {
                if screen {
                    // this window has its own keyboard: to the Apple-1 unless
                    // a widget of it has the focus
                    let to_emu = !ui.ctx().egui_wants_keyboard_input() && self.ask.is_none() && self.dialog.is_none();
                    let mut ev = ui.ctx().input_mut(|i| std::mem::take(&mut i.events));
                    self.handle_keys(ctx, &mut ev, to_emu);
                    ui.ctx().input_mut(|i| i.events = ev);
                }
                let integer = &mut self.cfg.integer_scale;
                crate::dock::header(ui, &title, dcfg, h, out, p.closable(), &mut |ui| {
                    if screen && ui.checkbox(integer, t("scr_integer")).hint(h, t("scr_integer_tip"), "h_scr_integer").changed() {
                        int_changed = true;
                    }
                });
                self.pane_ui(ui, p, true);
            });
            self.wins.insert(p, st);
            if out.close {
                if screen {
                    // closing the window of a screen docks it again
                    dcfg.detached = false;
                } else {
                    self.set_pane_wanted(p, false);
                }
            }
            if out.changed || out.close || int_changed || dcfg != *self.pane_dock(p) {
                *self.pane_dock(p) = dcfg;
                self.dirty();
            }
        }
    }

    /// Settings of the editor into gui.json (tabs, text not saved yet…).
    /// Bytes of the asm source line(s) selected in the editor, marked in
    /// the Disassembly and the Hex dump; moving to another line brings them
    /// into view.
    fn sync_highlight(&mut self) {
        use crate::editor::SrcLang;
        let mut hl: Vec<(u16, u16)> = Vec::new();
        let lines = self.ed.src_lines();
        if let (true, Some((l0, l1))) = (self.cfg.editor.open, lines) {
            if let Some(tab) = self.ed.current() {
                if tab.lang == SrcLang::Asm {
                    if let Ok(a) = &*tab.assembly() {
                        for x in a.listing.iter().filter(|x| x.file == 0 && !x.bytes.is_empty() && x.line >= l0 && x.line <= l1) {
                            hl.push((x.addr, x.addr.wrapping_add(x.bytes.len() as u16 - 1)));
                        }
                    }
                }
            }
        }
        let line = lines.map(|l| l.0);
        if (self.ed.focused || self.hl_line.is_none()) && line != self.hl_line {
            if let Some(&(first, _)) = hl.first() {
                self.dbg.jump_to(first);
            }
        }
        self.hl_line = line;
        if hl != self.dbg.hl {
            self.dbg.hl = hl;
        }
    }

    fn editor_sync(&mut self) {
        self.sync_highlight();
        if std::mem::take(&mut self.ed.changed) {
            self.cfg.editor = self.ed.to_cfg(self.cfg.editor.open);
            self.dirty();
        }
    }

    /// The current tab tokenized or assembled (uid, name, result), or an
    /// error shown in the status bar.
    fn editor_built(&mut self) -> Option<(u64, String, crate::editor::Built)> {
        let ram = self.ed.ram;
        let tab = self.ed.current()?;
        let (uid, name) = (tab.uid, tab.name());
        match crate::editor::build(tab, ram) {
            Ok(b) => Some((uid, name, b)),
            Err(e) => {
                self.fail(e);
                None
            }
        }
    }

    /// « Lancer »: BASIC: interpreter loaded if needed, the tokenized
    /// program put in memory (zero page + program below HIMEM), then E2B3R
    /// and RUN. 6502: the assembled code put at its origin, then ORIGR.
    fn editor_run(&mut self) {
        use crate::editor::Built;
        let Some((_, name, b)) = self.editor_built() else { return };
        let mut extra = Vec::new();
        let c = self.s.m.board.config.clone();
        let need = b.top().div_ceil(4096) * 4;
        if !c.color_card && c.ram_kb < need && need <= 48 {
            let mut c2 = c;
            c2.ram_kb = need;
            self.s.m.board.apply_config(c2);
            extra.push(tf("s_ram_extended", &[&need]));
        }
        if matches!(b, Built::Basic(_)) {
            match self.ensure_basic() {
                Ok(Some(m)) => extra.push(m),
                Ok(None) => {}
                Err(e) => return self.fail(e),
            }
        }
        self.loading = None;
        self.loaded = None;
        let prog = Program { blocks: b.blocks(), run: None, notes: vec![] };
        if let Built::Asm(a) = &b {
            self.dbg.set_symbols(a.symbols.clone());
        }
        self.dbg.resolve_pending(&mut self.s);
        for x in std::mem::take(&mut self.script.dbg_breaks) {
            if !self.dbg.add_break_text(&mut self.s, &x) {
                eprintln!("--dbg-arret : {x} ?");
            }
        }
        self.s.load_built(&name, &prog);
        self.type_command(&b.run());
        extra.push(match &b {
            Built::Basic(img) => tf("ed_ran", &[&name, &img.aci_command()]),
            Built::Asm(a) => tf("as_ran", &[&name, &a.bytes.len(), &format!("{:04X}", a.origin), &a.run_command()]),
        });
        self.info(extra.join(" — "));
        // the Apple-1 takes the keyboard
        self.ed.release = true;
    }

    /// « Exporter WAV »: ACI tape (BASIC: zero page + program; 6502: the
    /// code at its origin), with its ⚙ parameters in the library.
    fn editor_export(&mut self, uid: u64, path: PathBuf) {
        use crate::editor::Built;
        let path = match path.extension().map(|e| e.to_string_lossy().to_lowercase()) {
            Some(e) if e == "wav" || e == "aif" || e == "aiff" => path,
            _ => path.with_extension("wav"),
        };
        let ram = self.ed.ram;
        let Some(tab) = self.ed.tab(uid) else { return };
        let b = match crate::editor::build(tab, ram) {
            Ok(b) => b,
            Err(e) => return self.fail(e),
        };
        let src = tab.name();
        let prog = Program { blocks: b.blocks(), run: None, notes: vec![] };
        let mut o = ExportOptions::for_format(apple1_core::wav::AudioFormat::from_path(&path));
        o.timing = self.s.m.board.config.timing;
        if let Err(e) = program::save_file(&path, &prog, &o) {
            return self.fail(e);
        }
        let secs = std::fs::read(&path).ok().and_then(|b| apple1_core::wav::parse_audio(&b).ok()).map(|a| a.duration_secs()).unwrap_or(0.0);
        let p = match &b {
            Built::Basic(img) => ProgParams {
                ranges: params::ranges_aci(&prog.ranges()),
                kind: ProgKind::Basic,
                needs_basic: true,
                run_cmd: "E2B3R\\nRUN".into(),
                notes: tf("ed_export_note", &[&src, &img.ram_kb()]),
                ..Default::default()
            },
            Built::Asm(a) => ProgParams {
                ranges: params::ranges_aci(&prog.ranges()),
                kind: ProgKind::Machine,
                needs_basic: false,
                run_cmd: a.run_command(),
                notes: tf("as_export_note", &[&src]),
                ..Default::default()
            },
        };
        let _ = self.lib.set_params(&path, p);
        self.lib.scan();
        self.info(match &b {
            Built::Basic(img) => tf("ed_exported", &[&path.display(), &format!("{secs:.0}"), &img.aci_command(), &img.ram_kb()]),
            Built::Asm(a) => tf("as_exported", &[&path.display(), &format!("{secs:.0}"), &a.aci_range(), &a.run_command()]),
        });
    }

    fn set_library(&mut self, path: PathBuf) {
        let mut msg = tf("lib_now", &[&path.display()]);
        if !path.exists() {
            match library::populate(&path) {
                Ok(n) => msg = tf("lib_created", &[&path.display(), &n]),
                Err(e) => return self.fail(e),
            }
        }
        self.cfg.library = path.clone();
        self.cfg.libraries.retain(|d| *d != path);
        self.lib.set_dir(path);
        self.refresh_roots();
        self.selected = None;
        self.dirty();
        self.info(msg);
    }

    fn add_library(&mut self, path: PathBuf) {
        if self.cfg.library_dirs().contains(&path) {
            return self.fail(t("lib_folder_exists"));
        }
        self.cfg.libraries.push(path.clone());
        self.cfg.folders_open.insert(path.display().to_string(), true);
        self.refresh_roots();
        self.dirty();
        self.info(tf("lib_folder_added", &[&path.display()]));
    }

    fn remove_library(&mut self, path: &Path) {
        self.cfg.libraries.retain(|d| d != path);
        self.refresh_roots();
        self.dirty();
        self.info(tf("lib_folder_removed", &[&path.display()]));
    }

    /// Library folders: the main one, the others, then the GitHub projects
    /// already downloaded.
    fn refresh_roots(&mut self) {
        let mut roots: Vec<Root> = self.cfg.library_dirs().into_iter().map(Root::local).collect();
        for s in &self.cfg.github {
            let Ok(spec) = Spec::parse(s) else { continue };
            if let Some(m) = Meta::load(&spec, &self.gh_cache) {
                roots.push(Root { path: m.dir(&spec, &self.gh_cache), label: spec.label(), github: Some(s.clone()), error: None });
                self.gh_meta.insert(s.clone(), m);
            }
        }
        self.lib.set_roots(roots);
    }

    // --------------------------------------------------------------- GitHub

    fn add_github(&mut self, text: &str) -> Result<(), String> {
        let spec = Spec::parse(text)?;
        let key = spec.text();
        if self.cfg.github.contains(&key) {
            return Err(tf("gh_already", &[&key]));
        }
        self.cfg.github.push(key.clone());
        self.dirty();
        self.github_job(github::Job::Sync(spec));
        Ok(())
    }

    fn remove_github(&mut self, key: &str) {
        self.cfg.github.retain(|g| g != key);
        self.gh_meta.remove(key);
        self.gh_status.remove(key);
        self.refresh_roots();
        self.dirty();
        self.info(tf("gh_removed", &[&key]));
    }

    fn github_job(&mut self, job: github::Job) {
        let spec = match &job {
            github::Job::Sync(s) | github::Job::Latest(s) => s.clone(),
        };
        self.gh_status.insert(spec.text(), (t("gh_waiting").into(), 0.0));
        self.info(format!("⎇ {} : {}", spec.label(), t("gh_waiting")));
        // the token goes to the worker only, never into a message
        self.gh.send(job, self.cfg.token());
    }

    fn github_tick(&mut self) -> bool {
        let mut any = false;
        while let Ok(ev) = self.gh.rx.try_recv() {
            any = true;
            match ev {
                github::Event::Progress(s, m, f) => {
                    self.info(format!("⎇ {} : {m}", s.label()));
                    self.gh_status.insert(s.text(), (m, f));
                }
                github::Event::Synced(s, Ok(m)) => {
                    self.gh_status.remove(&s.text());
                    self.refresh_roots();
                    self.lib.scan();
                    self.info(tf("gh_synced", &[&s.label(), &m.short(), &m.date, &m.files]));
                }
                github::Event::Synced(s, Err(e)) | github::Event::Built(s, Err(e)) => {
                    self.gh_status.insert(s.text(), (e.clone(), -1.0));
                    self.fail(format!("⎇ {} : {e}", s.label()));
                }
                github::Event::Built(s, Ok(b)) => {
                    self.gh_status.remove(&s.text());
                    self.lib.scan();
                    self.lib_load(&b.path, true);
                    if !self.error {
                        self.message = format!("⎇ {} — {}", b.origin, self.message);
                    }
                }
            }
        }
        any
    }

    /// The cassette is moving: (writing?, time of the head in seconds).
    /// Reading: time on the tape; writing: emulated time of the toggles.
    fn tape_now(&self) -> Option<(bool, f64)> {
        let b = &self.s.m.board;
        if b.recording.is_some() && self.s.record_path.is_some() {
            return Some((true, b.now_secs()));
        }
        let p = b.player.as_ref()?;
        let now = b.now_secs();
        (p.started_at.is_some() && !p.finished(now)).then(|| (false, p.position(now)))
    }

    fn tape_window(&self, write: bool, t0: f64, t1: f64) -> crate::tapesig::Window {
        let b = &self.s.m.board;
        if write {
            let r = b.recording.as_deref().unwrap_or(&[]);
            let tm = b.config.timing;
            let a = r.partition_point(|&c| tm.cycles_to_secs(c) <= t0);
            let z = r.partition_point(|&c| tm.cycles_to_secs(c) <= t1);
            crate::tapesig::Window { edges: r[a..z].iter().map(|&c| tm.cycles_to_secs(c)).collect(), level0: a % 2 == 1 }
        } else {
            b.player.as_ref().map(|p| crate::tapesig::window(&p.edges, t0, t1)).unwrap_or_default()
        }
    }

    /// Send the signal that went under the head since the last frame to
    /// the sound card (real-time speed only).
    fn sound_tick(&mut self) {
        let now = self.tape_now();
        let (Some((w, t1)), true, false) = (now, self.cfg.tape_sound, self.s.fast) else {
            self.snd_t = None;
            return;
        };
        if self.snd.is_none() {
            self.snd = Some(crate::audio::Audio::open());
        }
        let rate = crate::audio::RATE as f64;
        let t0 = match self.snd_t {
            Some((w0, t0)) if w0 == w && t0 <= t1 && t1 - t0 < 0.3 => t0,
            _ => t1 - 0.02,
        };
        let n = ((t1 - t0) * rate) as usize;
        let t1 = t0 + n as f64 / rate;
        self.snd_t = Some((w, t1));
        let win = self.tape_window(w, t0, t1);
        let samples = crate::tapesig::render(&win, t0, t1, n, crate::tapesig::alpha(5000.0, rate as f32), &mut self.snd_lp);
        if let Some(Some(a)) = &self.snd {
            a.push(&samples, self.cfg.tape_volume);
        }
    }

    /// Oscilloscope: the cassette signal around the head, green trace on a
    /// dark grid (10 ms wide; while writing, the last 10 ms).
    fn scope_ui(&mut self, ctx: &egui::Context) {
        let Some((w, head)) = self.tape_now().filter(|_| self.cfg.tape_scope) else { return };
        let h = self.cfg.help_mode;
        let mut open = true;
        let mut changed = false;
        let (t0, t1) = if w { (head - 0.010, head) } else { (head - 0.005, head + 0.005) };
        let win = self.tape_window(w, t0, t1);
        egui::Window::new(t("sc_title"))
            .id(egui::Id::new("oscilloscope"))
            .open(&mut open)
            .default_size([380.0, 170.0])
            .default_pos([24.0, (ctx.content_rect().bottom() - 240.0).max(80.0)])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(if w { t("sc_write") } else { t("sc_read") }).small());
                    changed |= ui.checkbox(&mut self.cfg.tape_sound, t("sc_sound")).hint(h, t("sc_sound_tip"), "h_sc_sound").changed();
                    changed |= ui.add(egui::Slider::new(&mut self.cfg.tape_volume, 0.0..=1.0).fixed_decimals(2)).hint(h, t("sc_volume_tip"), "h_sc_volume").changed();
                    if self.s.fast && self.cfg.tape_sound {
                        ui.label(RichText::new(t("sc_muted")).small().weak());
                    } else if matches!(self.snd, Some(None)) && self.cfg.tape_sound {
                        ui.label(RichText::new(t("sc_no_audio")).small().weak());
                    }
                });
                let size = egui::vec2(ui.available_width().max(200.0), ui.available_height().max(90.0));
                let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::hover());
                let p = ui.painter_at(rect);
                p.rect_filled(rect, 3.0, Color32::from_rgb(6, 14, 8));
                let grid = egui::Stroke::new(1.0, Color32::from_rgb(22, 52, 28));
                for i in 1..10 {
                    let x = rect.left() + rect.width() * i as f32 / 10.0;
                    p.line_segment([egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())], grid);
                }
                for i in 1..4 {
                    let y = rect.top() + rect.height() * i as f32 / 4.0;
                    p.line_segment([egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)], grid);
                }
                let n = (rect.width() as usize * 2).max(2);
                let mut lp = if win.level0 { 1.0 } else { -1.0 };
                let a = crate::tapesig::alpha(4000.0, n as f32 / 0.010);
                let ys = crate::tapesig::render(&win, t0, t1, n, a, &mut lp);
                let pts: Vec<egui::Pos2> = ys
                    .iter()
                    .enumerate()
                    .map(|(i, y)| egui::pos2(rect.left() + rect.width() * i as f32 / (n - 1) as f32, rect.center().y - y * rect.height() * 0.36))
                    .collect();
                p.add(egui::Shape::line(pts, egui::Stroke::new(1.6, Color32::from_rgb(70, 255, 110))));
                // the head
                let hx = if w { rect.right() - 1.0 } else { rect.center().x };
                p.line_segment([egui::pos2(hx, rect.top()), egui::pos2(hx, rect.bottom())], egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 200, 80, 90)));
                p.text(rect.left_bottom() + egui::vec2(4.0, -3.0), egui::Align2::LEFT_BOTTOM, t("sc_div"), egui::FontId::monospace(11.0), Color32::from_rgb(70, 140, 80));
                resp.hint(h, t("sc_tip"), "h_sc_scope");
            });
        if !open {
            self.cfg.tape_scope = false;
            changed = true;
        }
        if changed {
            self.dirty();
        }
    }

    fn toggle_help(&mut self) {
        self.cfg.help_mode = !self.cfg.help_mode;
        self.dirty();
        self.info(if self.cfg.help_mode { t("help_on") } else { t("help_off") });
    }

    /// Load any file. `memory` = tapes go into memory without asking.
    fn load_path(&mut self, path: &Path, memory: bool) {
        let bytes = std::fs::read(path).ok();
        match program::kind_of(path, bytes.as_deref()) {
            Kind::Bin => self.ask = Some(Ask::Addr { path: path.into(), text: "300".into(), err: None }),
            Kind::Wav if !memory => self.ask = Some(Ask::Tape { path: path.into() }),
            _ => self.load_memory(path, None),
        }
    }

    /// Loading another program: the previous one's breakpoints and labels no longer mean anything (#30).
    fn forget_debug_marks(&mut self) {
        self.s.dbg.breaks.clear();
        self.s.dbg.basic_breaks.clear();
        self.dbg.set_symbols(Vec::new());
    }

    fn load_memory(&mut self, path: &Path, addr: Option<u16>) {
        self.forget_debug_marks();
        match self.s.load_program(path, addr) {
            Ok((m, _)) => {
                self.loaded = Some((path.to_path_buf(), mtime(path)));
                self.info(m)
            }
            Err(e) if e.contains("adresse n'est pas connue") => self.ask = Some(Ask::Addr { path: path.into(), text: "300".into(), err: Some(t("addr_unknown").into()) }),
            Err(e) => self.fail(e),
        }
    }

    /// BASIC button: the chosen file, or one found in the library
    /// (BASIC.wav, integer_basic.bin…), else ask for it.
    fn basic(&mut self) {
        if self.cfg.basic_g2 {
            let r = self.s.load_g2basic();
            return self.result(r);
        }
        match launch::find_basic(self.cfg.basic.as_deref(), &self.lib.folders()) {
            Ok((p, _, auto)) => {
                let r = self.s.load_basic(&p);
                if r.is_ok() && self.cfg.basic.as_ref() != Some(&p) {
                    self.cfg.basic = Some(p.clone());
                    self.dirty();
                }
                match r {
                    Ok(m) if auto => self.info(format!("{} {m}", tf("s_basic_auto", &[&file_name(&p)]))),
                    r => self.result(r),
                }
            }
            Err(_) if self.cfg.basic.as_ref().map(|p| !p.is_file()).unwrap_or(true) => self.open_dialog(Op::Basic),
            Err(e) => self.fail(e),
        }
    }

    /// Make sure a BASIC is in memory (the one of the Preferences: G2 Basic, or the original
    /// Integer BASIC loaded from the known file or one found in the library, which is then
    /// remembered). Returns a message when it was loaded.
    fn ensure_basic(&mut self) -> Result<Option<String>, String> {
        self.ensure_basic_as(None)
    }

    /// Same, for a program that wants its own BASIC (⚙ « Programme G2 Basic » / « Programme
    /// Integer BASIC »): `Some(true)` G2 Basic, `Some(false)` the original, `None` the Preferences'.
    fn ensure_basic_as(&mut self, want_g2: Option<bool>) -> Result<Option<String>, String> {
        let g2 = want_g2.unwrap_or(self.cfg.basic_g2);
        if g2 {
            let present = if want_g2.is_some() { self.s.g2_present() } else { self.s.basic_present() };
            return if present { Ok(None) } else { self.s.load_g2basic().map(Some) };
        }
        let r = if want_g2.is_some() && self.s.g2_present() {
            // G2 Basic is there, the program wants the original
            launch::load_original(&mut self.s, self.cfg.basic.as_deref(), &self.lib.folders()).map(Some)?
        } else {
            launch::ensure_basic(&mut self.s, self.cfg.basic.as_deref(), &self.lib.folders())?
        };
        Ok(r.map(|b| {
            if b.auto || self.cfg.basic.as_ref() != Some(&b.path) {
                self.cfg.basic = Some(b.path);
                self.dirty();
            }
            b.msg
        }))
    }

    fn reset(&mut self) {
        self.s.m.reset();
        self.info(t("s_reset"));
    }

    fn clear(&mut self) {
        self.s.m.clear_screen();
        self.info(t("s_cleared"));
    }

    fn toggle_fast(&mut self) {
        self.s.fast = !self.s.fast;
        self.cfg.fast = self.s.fast;
        self.dirty();
        let m = if self.s.fast { t("s_fast") } else { t("s_real") };
        self.info(m);
    }

    // ------------------------------------------------------ library loading

    fn lib_entry(&self, name: &str) -> Option<PathBuf> {
        self.lib.entries.iter().find(|e| e.name == name).or_else(|| self.lib.entries.iter().find(|e| e.path.ends_with(name))).map(|e| e.path.clone())
    }

    /// Checked parameters of a library program.
    fn plan_of(&self, path: &Path) -> Result<Plan, String> {
        let p = self.lib.params_of(path);
        let is_bin = program::kind_of(path, None) == Kind::Bin;
        let len = std::fs::metadata(path).map(|m| m.len() as usize).unwrap_or(0);
        p.validate(is_bin, len).map_err(|e| tf("e_params", &[&file_name(path), &e]))
    }

    /// Decode a library program with its stored parameters (instant).
    fn build_program(&self, path: &Path, plan: &Plan) -> Result<Program, String> {
        launch::build_program(path, plan)
    }

    /// Double-click on a library program: load and run it when it can be
    /// run (instant), else load it; a source opens in the editor.
    fn lib_open(&mut self, path: &Path) {
        let run = library::source_kind(path, None).is_some() || self.plan_of(path).map(|p| !p.run_keys.is_empty()).unwrap_or(false);
        self.lib_load(path, run);
        self.lib_to_editor(path);
    }

    /// A source of the library opened in a tab of the editor (or its tab
    /// shown), when the editor is visible.
    fn lib_to_editor(&mut self, path: &Path) {
        if self.cfg.editor.open && library::source_kind(path, None).is_some() {
            if let Err(e) = self.ed.open_lib(path) {
                self.fail(e);
            }
        }
    }

    /// ⬇ (run = false) or ▶ (run = true) on a library program: instant
    /// load (the command line can ask for the cassette: --mode-cassette).
    fn lib_load(&mut self, path: &Path, run: bool) {
        self.lib_load_via(path, run, self.cfg.cassette_mode);
    }

    /// `cassette`: read through the simulated ACI (📼), at real speed.
    fn lib_load_via(&mut self, path: &Path, run: bool, cassette: bool) {
        self.selected = Some(path.to_path_buf());
        if let Some(src) = library::source_kind(path, None) {
            return self.run_source(path, src, run);
        }
        let plan = match self.plan_of(path) {
            Ok(p) => p,
            Err(e) => return self.fail(e),
        };
        if run && plan.run_keys.is_empty() {
            return self.fail(tf("e_no_run", &[&file_name(path)]));
        }
        let ram_msg = self.ensure_ram(&plan);
        // BASIC first (instant), then the program (zero page + code), then
        // the warm start E2B3R + RUN.
        let basic_msg = if plan.needs_basic {
            let want = match plan.kind {
                ProgKind::Basic => Some(true),
                ProgKind::BasicInteger => Some(false),
                ProgKind::Machine => None,
            };
            match self.ensure_basic_as(want) {
                Ok(m) => m,
                Err(e) => return self.fail(e),
            }
        } else {
            None
        };
        let already = basic_msg.is_none() && self.loaded.as_ref() == Some(&(path.to_path_buf(), mtime(path)));
        if run && already {
            return self.run_plan(path, &plan);
        }
        let prog = match self.build_program(path, &plan) {
            Ok(p) => p,
            Err(e) => return self.fail(e),
        };
        if cassette {
            self.cassette_load(path, &prog, run.then(|| plan.run_keys.clone()));
        } else {
            self.forget_debug_marks();
            let m = self.s.load_built(&file_name(path), &prog);
            self.loaded = Some((path.to_path_buf(), mtime(path)));
            if run {
                self.run_plan(path, &plan);
            } else {
                self.info(format!("⚡ {m}"));
            }
        }
        if let (Some(b), false) = (basic_msg, self.error) {
            self.message = format!("{b} — {}", self.message);
        }
        if let (Some(m), false) = (ram_msg, self.error) {
            self.message = format!("{m} — {}", self.message);
        }
    }

    /// A source program: a BASIC listing is typed into Integer BASIC
    /// (cold start E000R, the lines, RUN), assembly is built with ca65/ld65.
    fn run_source(&mut self, path: &Path, src: library::Source, run: bool) {
        match src {
            library::Source::Basic => {
                let text = match std::fs::read_to_string(path) {
                    Ok(t) => t,
                    Err(e) => return self.fail(format!("{} : {e}", path.display())),
                };
                let basic_msg = match self.ensure_basic() {
                    Ok(m) => m,
                    Err(e) => return self.fail(e),
                };
                self.s.m.reset();
                self.s.m.type_text(&build::basic_keys(&text, false));
                if run {
                    // random pause before RUN: RND does not start from the same seed every time (#32)
                    self.s.m.type_pause();
                    self.s.m.type_text("RUN\r");
                }
                self.loaded = None;
                let n = text.lines().filter(|l| !l.trim().is_empty()).count();
                self.info(tf("l_typing", &[&file_name(path), &n]));
                if let Some(b) = basic_msg {
                    self.message = format!("{b} — {}", self.message);
                }
            }
            library::Source::Asm => match build::assemble(path) {
                Ok((p, at)) => {
                    self.forget_debug_marks();
                    let m = self.s.load_built(&file_name(path), &p);
                    self.loaded = None;
                    if run {
                        self.type_command(&format!("{at:X}R"));
                        self.info(tf("l_running", &[&file_name(path), &format!("{at:X}R")]));
                    } else {
                        self.info(format!("⚙ {m}"));
                    }
                }
                Err(e) => self.fail(e),
            },
            // ASCII art: shown in the tool; ▶ runs its 6502 program
            library::Source::Art => {
                let r = self.ascii.load_art(path);
                let ok = r.is_ok();
                self.result(r);
                if ok {
                    self.set_pane_wanted(Pane::Ascii, true);
                    if run {
                        let code = self.ascii.code(&self.cfg.ascii, true);
                        self.ascii_insert(code, true, crate::ascii::Then::Run);
                    }
                }
            }
        }
    }

    // ------------------------------------------------------ Photo → ASCII

    /// Additions to an existing setup (once): « Art ASCII » examples in the
    /// library, the Apple1-Slideshow GitHub project.
    fn extras(&mut self) {
        if self.cfg.extras >= crate::config::EXTRAS {
            return;
        }
        if self.cfg.library.is_dir() {
            // 7.9.2: the 5 folders (renamed 6502 and GEN2 COLOR) with their examples, Art ASCII included
            library::ensure_layout(&self.cfg.library);
        }
        for g in crate::config::DEFAULT_GITHUB {
            if !self.cfg.github.iter().any(|x| x.eq_ignore_ascii_case(g)) {
                self.cfg.github.push(g.to_string());
            }
        }
        self.cfg.extras = crate::config::EXTRAS;
        self.dirty();
    }

    fn ascii_open(&mut self, path: &Path) {
        let r = if crate::ascii::is_image(path) { self.ascii.load_image(path) } else { self.ascii.load_art(path) };
        self.result(r);
        self.set_pane_wanted(Pane::Ascii, true);
    }

    fn ascii_pane(&mut self, ui: &mut egui::Ui) {
        let mut acts = Vec::new();
        let h = self.cfg.help_mode;
        let ph = self.cfg.phosphor;
        if self.ascii.contents(ui, h, &mut self.cfg.ascii, ph, &mut acts) {
            self.dirty();
        }
        for a in acts {
            match a {
                crate::ascii::AsciiAction::Open => self.open_dialog(Op::AsciiOpen),
                crate::ascii::AsciiAction::Insert { code, asm, then } => self.ascii_insert(code, asm, then),
                crate::ascii::AsciiAction::Save { text, name } => {
                    let r = self.ascii_save(&text, &name);
                    self.result(r);
                }
            }
        }
    }

    /// The generated program in a new editor tab, then ▶ or 📼 as the
    /// editor's buttons.
    fn ascii_insert(&mut self, code: String, asm: bool, then: crate::ascii::Then) {
        let lang = if asm { crate::editor::SrcLang::Asm } else { crate::editor::SrcLang::Basic };
        let uid = self.ed.new_tab_with(code, lang);
        self.set_pane_wanted(Pane::Editor, true);
        match then {
            crate::ascii::Then::Nothing => self.info(t("ax_inserted").to_string()),
            crate::ascii::Then::Run => self.editor_run(),
            crate::ascii::Then::Export => self.open_dialog(Op::EdExport(uid)),
        }
    }

    /// The art as a .txt of the « Art ASCII » folder of the library.
    fn ascii_save(&mut self, text: &str, name: &str) -> Result<String, String> {
        let dir = self.cfg.library.join(crate::ascii::ART_DIR);
        std::fs::create_dir_all(&dir).map_err(|e| format!("{} : {e}", dir.display()))?;
        let base: String = name.chars().map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '-' }).collect();
        let base = if base.trim_matches('-').is_empty() { "art".to_string() } else { base };
        let mut p = dir.join(format!("{base}.txt"));
        let mut i = 2;
        while p.exists() {
            p = dir.join(format!("{base}-{i}.txt"));
            i += 1;
        }
        std::fs::write(&p, text).map_err(|e| format!("{} : {e}", p.display()))?;
        self.lib.scan();
        self.selected = Some(p.clone());
        Ok(tf("ax_saved", &[&p.display()]))
    }

    /// Programs above the 8 KB of a basic Apple-1 (Football, Star Trek
    /// 2003, Life...): add RAM (as a memory board would) up to the end of
    /// the highest range, rounded to 4 KB.
    fn ensure_ram(&mut self, plan: &Plan) -> Option<String> {
        let c = self.s.m.board.config.clone();
        if c.color_card {
            return None;
        }
        let kb = launch::ram_needed(plan, c.ram_kb)?;
        let mut c2 = c;
        c2.ram_kb = kb;
        self.s.m.board.apply_config(c2);
        Some(tf("s_ram_extended", &[&kb]))
    }

    fn run_plan(&mut self, path: &Path, plan: &Plan) {
        self.type_command(&plan.run_keys);
        self.info(tf("l_running", &[&file_name(path), &plan.run_keys.replace('\r', " ⏎ ")]));
    }

    /// RESET (back to the Woz monitor), then type `keys` and RETURN.
    fn type_command(&mut self, keys: &str) {
        self.s.m.reset();
        self.s.m.type_text(&format!("{}\r", keys.trim_end_matches('\r')));
    }

    /// Simulated loading: the program goes on the virtual cassette and the
    /// ACI reads it (C100R then the ranges, typed automatically).
    fn cassette_load(&mut self, path: &Path, prog: &Program, run: Option<String>) {
        let tape = if program::kind_of(path, None) == Kind::Wav {
            path.to_path_buf()
        } else {
            let dir = crate::config::cache_base().join("cassettes");
            let f = dir.join(format!("{}.wav", file_name(path)));
            let _ = std::fs::create_dir_all(&dir);
            let o = ExportOptions { timing: self.s.m.board.config.timing, ..Default::default() };
            if let Err(e) = program::save_file(&f, prog, &o) {
                return self.fail(e);
            }
            f
        };
        if let Err(e) = self.s.insert_tape(&tape) {
            return self.fail(e);
        }
        self.s.tape_source = Some(path.to_path_buf());
        let cmd = prog.aci_read_command();
        self.type_command(&format!("C100R\r{cmd}"));
        self.loading = Some(Loading { path: path.to_path_buf(), run, done_at: None, started: self.s.m.board.now_secs() });
        self.info(tf("l_cassette", &[&file_name(path), &cmd]));
    }

    /// Follow a cassette loading: when the tape has ended, the program is
    /// in memory (and started for ▶).
    fn loading_tick(&mut self) {
        let Some(l) = &mut self.loading else { return };
        let now = self.s.m.board.now_secs();
        match self.s.tape_progress() {
            None => {
                self.loading = None;
                return;
            }
            Some((_, false, _)) if now - l.started > 20.0 => {
                let n = file_name(&l.path);
                self.loading = None;
                return self.fail(tf("e_aci_not_reading", &[&n]));
            }
            Some((_, _, true)) if l.done_at.is_none() => l.done_at = Some(now),
            _ => {}
        }
        if l.done_at.map(|d| now - d > 0.3).unwrap_or(false) {
            let l = self.loading.take().unwrap();
            self.loaded = Some((l.path.clone(), mtime(&l.path)));
            match l.run {
                Some(keys) => {
                    self.type_command(&keys);
                    self.info(tf("l_running", &[&file_name(&l.path), &keys.replace('\r', " ⏎ ")]));
                }
                None => self.info(tf("l_cassette_done", &[&file_name(&l.path)])),
            }
        }
    }

    fn open_params(&mut self, path: &Path) {
        let len = std::fs::metadata(path).map(|m| m.len() as usize).unwrap_or(0);
        self.selected = Some(path.to_path_buf());
        self.ask = Some(Ask::Params { path: path.into(), p: self.lib.params_of(path), err: None, is_bin: program::kind_of(path, None) == Kind::Bin, len });
    }

    // ----------------------------------------------------------------- zoom

    fn font_hot_step(&mut self, d: f32) {
        use crate::fontsz::Win;
        match self.font_hot {
            Some(Win::Editor) => self.ed.font_step(d),
            Some(Win::Listing) => self.ed.listing_step(d),
            Some(Win::Debugger) => {
                self.cfg.debugger.font = crate::fontsz::step(self.cfg.debugger.font, d);
                self.dirty();
            }
            Some(Win::Disasm) => {
                self.cfg.debugger.disasm.font = crate::fontsz::step(self.cfg.debugger.disasm.font, d);
                self.dirty();
            }
            Some(Win::Dump) => {
                self.cfg.debugger.dump_win.font = crate::fontsz::step(self.cfg.debugger.dump_win.font, d);
                self.dirty();
            }
            Some(Win::Library) => {
                self.cfg.library_font = crate::fontsz::step(self.cfg.library_font, d);
                self.dirty();
            }
            None => self.zoom_step(if d > 0.0 { 1 } else { -1 }),
        }
    }

    fn font_hot_reset(&mut self) {
        use crate::fontsz::Win;
        match self.font_hot {
            Some(Win::Editor) => self.ed.font_reset(),
            Some(Win::Listing) => {
                self.ed.listing_font = crate::fontsz::reset();
                self.ed.changed = true;
            }
            Some(Win::Debugger) => {
                self.cfg.debugger.font = crate::fontsz::reset();
                self.dirty();
            }
            Some(Win::Disasm) => {
                self.cfg.debugger.disasm.font = crate::fontsz::reset();
                self.dirty();
            }
            Some(Win::Dump) => {
                self.cfg.debugger.dump_win.font = crate::fontsz::reset();
                self.dirty();
            }
            Some(Win::Library) => {
                self.cfg.library_font = crate::fontsz::reset();
                self.dirty();
            }
            None => {
                self.cfg.zoom = Zoom::Ajuster;
                self.dirty();
            }
        }
    }

    fn zoom_step(&mut self, delta: i32) {
        let cur = match self.cfg.zoom {
            Zoom::Fixe(z) => z as i32,
            Zoom::Ajuster => self.fit_zoom().floor().max(1.0) as i32,
        };
        let wanted = (cur + delta).clamp(1, ZOOM_MAX as i32) as u32;
        self.cfg.zoom = Zoom::Fixe(wanted);
        // a fixed zoom bigger than the pane scrolls: say so (#36)
        let fit = self.fit_zoom();
        let shown = if fit < 1.0 { fit } else { fit.floor() };
        if delta > 0 && (wanted as f32) > shown + 0.01 {
            self.info(tf("zoom_capped", &[&fmt_zoom(wanted as f32)]));
        }
        self.dirty();
    }

    fn color_card(&self) -> bool {
        self.s.m.board.config.color_card
    }

    fn set_color_card(&mut self, on: bool, hz50: bool) {
        if on == self.cfg.carte_couleur && hz50 == self.cfg.couleur_50hz {
            return;
        }
        self.cfg.carte_couleur = on;
        self.cfg.couleur_50hz = hz50;
        if on {
            self.cfg.color_open = true;
        }
        self.dirty();
        let mut c = self.s.m.board.config.clone();
        c.color_card = on;
        c.color_50hz = hz50;
        self.s.m.board.apply_config(c);
        self.info(if on { t("gen2_on") } else { t("gen2_off") });
    }

    fn plab_card(&self) -> bool {
        self.s.m.board.config.plab_card
    }

    fn set_plab_card(&mut self, on: bool) {
        if on == self.cfg.carte_plab && on == self.plab_card() {
            // already plugged, but the screen was hidden: a click must show it
            if on && !self.cfg.plab_open {
                self.cfg.plab_open = true;
                self.dirty();
            }
            return;
        }
        self.cfg.carte_plab = on;
        if on {
            self.cfg.plab_open = true;
        }
        self.dirty();
        let mut c = self.s.m.board.config.clone();
        c.plab_card = on;
        c.plab_nmi = self.cfg.plab_nmi;
        self.s.m.board.apply_config(c);
        self.s.m.board.vdp_changed_config();
        self.info(if on { t("plab_on") } else { t("plab_off") });
    }

    /// Apply the card switches of `cfg` to the machine (a reset of the
    /// preferences must not leave a card plugged in the running session).
    fn sync_cards(&mut self) {
        let mut c = self.s.m.board.config.clone();
        c.color_card = self.cfg.carte_couleur;
        c.color_50hz = self.cfg.couleur_50hz;
        c.plab_card = self.cfg.carte_plab;
        c.plab_nmi = self.cfg.plab_nmi;
        c.codetank = self.cfg.plab_rom.as_ref().and_then(|p| std::fs::read(p).ok());
        self.s.m.board.apply_config(c);
        self.s.m.board.vdp_changed_config();
    }

    /// Default settings again. The library folders, the GitHub projects and
    /// the token stay; files on disk are not deleted.
    fn start_fresh(&mut self) {
        self.cfg = GuiConfig::fresh_keeping_library(&self.cfg);
        self.ed = crate::editor::Editor::from_cfg(&self.cfg.editor);
        self.s.dbg.breaks.clear();
        self.s.dbg.basic_breaks.clear();
        self.sync_cards();
        apply_language(&self.cfg, &self.detected);
        self.applied_look = None;
        self.applied_active = None;
        self.reset_layout();
        self.refresh_roots();
        self.dirty();
    }

    /// The version question was answered (or dismissed): remember the kit version so it
    /// is not asked again, then open the workspace window if it was waiting.
    fn accept_version(&mut self, fresh: bool) {
        if fresh {
            self.start_fresh();
        } else if self.cfg.kit_version != crate::config::KIT_VERSION {
            self.cfg.kit_version = crate::config::KIT_VERSION.into();
            self.dirty();
        }
        if std::mem::take(&mut self.pending_welcome) {
            self.ask = Some(Ask::Welcome);
        }
    }

    /// Install (or remove) the CodeTank EEPROM image of the P-LAB card.
    fn set_plab_rom(&mut self, path: Option<PathBuf>) {
        let img = match &path {
            Some(p) => match std::fs::read(p) {
                Ok(b) if !b.is_empty() && b.len() <= 16384 => Some(b),
                Ok(b) => return self.fail(format!("{} : {} octets (16384 max)", p.display(), b.len())),
                Err(e) => return self.fail(format!("{} : {e}", p.display())),
            },
            None => None,
        };
        let n = img.as_ref().map(Vec::len).unwrap_or(0);
        self.cfg.plab_rom = path;
        self.dirty();
        let mut c = self.s.m.board.config.clone();
        c.codetank = img;
        self.s.m.board.apply_config(c);
        self.info(if n > 0 { tf("plab_rom_loaded", &[&n]) } else { t("plab_rom_removed").to_string() });
    }

    /// The P-LAB card screen: what the TMS9918A draws, its status line and tools.
    fn plab_pane(&mut self, ui: &mut egui::Ui, detached: bool) {
        let h = self.cfg.help_mode;
        ui.add_space(2.0);
        ui.horizontal_wrapped(|ui| {
            if ui.button(t("plab_rom")).hint(h, t("plab_rom_tip"), "h_plab_rom").clicked() {
                self.open_dialog(Op::PlabRom);
            }
            if self.s.m.board.config.codetank.is_some() && ui.button(t("plab_eject")).clicked() {
                self.set_plab_rom(None);
            }
            let mut nmi = self.cfg.plab_nmi;
            if ui.checkbox(&mut nmi, t("plab_nmi")).hint(h, t("plab_nmi_tip"), "h_plab_nmi").changed() {
                self.cfg.plab_nmi = nmi;
                self.dirty();
                let mut c = self.s.m.board.config.clone();
                c.plab_nmi = nmi;
                self.s.m.board.apply_config(c);
            }
            if !detached {
                let full = t("dk_detach");
                let label = full.split_whitespace().next().unwrap_or("⧉").to_string();
                if ui.add(egui::Button::new(label).small()).hint(h, t("dk_detach_tip"), "h_dk_detach").clicked() {
                    self.pane_dock(Pane::Plab).detached = true;
                    self.dirty();
                }
            }
        });
        let v = &self.s.m.board.vdp;
        let regs: String = v.reg.iter().map(|r| format!("{r:02X}")).collect::<Vec<_>>().join(" ");
        let mode = if v.display_on() { v.mode().name().to_string() } else { t("plab_blank").to_string() };
        let info = tf("plab_info", &[&mode, &regs, &format!("{:02X}", v.peek_status()), &v.frames]);
        ui.label(RichText::new(info).small().monospace());
        // the picture
        let img = ColorImage::from_rgba_unmultiplied([apple1_core::tms9918::WIDTH, apple1_core::tms9918::HEIGHT], &v.render_rgba());
        let bd = apple1_core::tms9918::PALETTE[v.backdrop() as usize];
        match &mut self.tex_plab {
            Some(tx) => tx.set(img, TextureOptions::NEAREST),
            None => self.tex_plab = Some(ui.ctx().load_texture("plab-ecran", img, TextureOptions::NEAREST)),
        }
        let avail = ui.available_rect_before_wrap().intersect(ui.clip_rect());
        let area = avail.shrink(8.0);
        let ppp = ui.ctx().pixels_per_point();
        let (w, hh) = (apple1_core::tms9918::WIDTH as f32, apple1_core::tms9918::HEIGHT as f32);
        let mut z = (area.width() / w).min(area.height() / hh).max(0.1);
        if z >= 1.0 && self.cfg.integer_scale {
            z = z.floor();
        }
        let dot = if z * ppp >= 1.0 { (z * ppp).round() / ppp } else { z };
        let size = egui::vec2(w * dot, hh * dot);
        ui.painter().rect_filled(avail, 0.0, Color32::from_rgb(bd[0], bd[1], bd[2]));
        let r = egui::Rect::from_center_size(avail.center(), size);
        ui.allocate_rect(avail, egui::Sense::click());
        if let Some(tx) = &self.tex_plab {
            ui.painter().image(tx.id(), r, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), Color32::WHITE);
        }
    }

    /// Integer zoom that fits the Apple-1 screen pane (fractional if < 1).
    fn fit_zoom(&self) -> f32 {
        self.fit_zoom_in(self.screen_rect)
    }

    fn fit_zoom_in(&self, r: egui::Rect) -> f32 {
        if !r.is_positive() {
            return 1.0;
        }
        let z = (r.width() / WIDTH as f32).min(r.height() / HEIGHT as f32);
        if z >= 1.0 && self.cfg.integer_scale {
            z.floor()
        } else {
            z
        }
    }

    // ------------------------------------------------------------- keyboard

    /// Send keyboard events to the Apple-1 (and remove them so egui does not
    /// also act on them), handle the global shortcuts.
    fn handle_keys(&mut self, ctx: &egui::Context, events: &mut Vec<egui::Event>, to_emu: bool) {
        type Action = Box<dyn FnOnce(&mut App)>;
        if std::env::var_os("APPLE1_DEBUG_KEYS").is_some() {
            for ev in events.iter() {
                if matches!(ev, egui::Event::Key { .. } | egui::Event::Text(_) | egui::Event::Paste(_)) {
                    eprintln!("clavier ({}) : {ev:?}", if to_emu { "Apple-1" } else { "fenetre" });
                }
            }
        }
        let mut fullscreen = false;
        let mut actions: Vec<Action> = Vec::new();
        let mut text = String::new();
        let mut pasted = String::new();
        events.retain(|ev| match ev {
            egui::Event::Key { key, pressed: true, modifiers, .. } => {
                let ctrl = modifiers.ctrl || modifiers.command;
                let act: Option<Action> = match key {
                    Key::F11 => {
                        fullscreen = true;
                        return false;
                    }
                    Key::F1 => Some(Box::new(|a| a.toggle_help())),
                    // in the BASIC editor, Ctrl+plus/minus/0 size its text
                    Key::Plus | Key::Equals if ctrl && self.font_hot.is_some() => Some(Box::new(|a| a.font_hot_step(1.0))),
                    Key::Minus if ctrl && self.font_hot.is_some() => Some(Box::new(|a| a.font_hot_step(-1.0))),
                    Key::Num0 if ctrl && self.font_hot.is_some() => Some(Box::new(|a| a.font_hot_reset())),
                    Key::Plus | Key::Equals if ctrl => Some(Box::new(|a| a.zoom_step(1))),
                    Key::Minus if ctrl => Some(Box::new(|a| a.zoom_step(-1))),
                    Key::Num0 if ctrl => Some(Box::new(|a| {
                        a.cfg.zoom = Zoom::Ajuster;
                        a.dirty()
                    })),
                    _ if !to_emu => return true,
                    Key::V | Key::C | Key::X if ctrl => return true, // -> Paste / Copy events
                    Key::R if ctrl => Some(Box::new(App::reset)),
                    Key::L if ctrl => Some(Box::new(App::clear)),
                    Key::Enter => Some(Box::new(|a| a.latch_key(0x0D))),
                    Key::Backspace => Some(Box::new(|a| a.latch_key(b'_'))),
                    Key::Escape => Some(Box::new(|a| a.latch_key(0x1B))),
                    Key::F3 => Some(Box::new(App::toggle_fast)),
                    _ if ctrl => {
                        // Ctrl+letter: control code, as on the real keyboard
                        let n = key.name().as_bytes();
                        if n.len() == 1 && n[0].is_ascii_uppercase() {
                            let c = n[0] & 0x1F;
                            Some(Box::new(move |a| a.latch_key(c)))
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                if let Some(f) = act {
                    actions.push(f);
                }
                false
            }
            egui::Event::Key { .. } => !to_emu,
            egui::Event::Text(t) if to_emu => {
                text.push_str(t);
                false
            }
            egui::Event::Paste(t) if to_emu => {
                pasted.push_str(t);
                false
            }
            _ => true,
        });
        if fullscreen {
            let fs = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
            ctx.send_viewport_cmd(ViewportCommand::Fullscreen(!fs));
        }
        for a in actions {
            a(self);
        }
        for ch in text.chars() {
            if let Some(c) = map_key(ch) {
                self.latch_key(c);
            }
        }
        if !pasted.is_empty() {
            // typed at keyboard speed by the core
            let t = pasted.replace("\r\n", "\r").replace('\n', "\r");
            self.s.m.type_text(&t);
            self.info(tf("s_pasted", &[&t.chars().count()]));
        }
    }

    // --------------------------------------------------------------- screen

    fn update_texture(&mut self, ctx: &egui::Context) {
        let b = &self.s.m.board;
        let visible = self.blink.visible_at(b.now_secs());
        let key = (b.display.generation, visible, self.cfg.phosphor);
        let fresh = self.tex.is_none() || self.drawn != Some(key);
        if fresh {
            self.draw_terminal(ctx, key);
        }
        let fresh2 = self.update_color_texture(ctx);
        self.update_screening(ctx, fresh, fresh2);
    }

    /// Upload the pictures, processed when the vintage screen is on.
    fn update_screening(&mut self, ctx: &egui::Context, fresh: bool, fresh2: bool) {
        let scr = self.cfg.screening;
        let now = Instant::now();
        let dt = now.duration_since(self.crt_t).as_secs_f32().min(0.1);
        self.crt_t = now;
        let mut scr2 = scr;
        scr2.on = self.cfg.screening2_on;
        let key = (scr, self.cfg.phosphor, scr2.on);
        let changed = self.crt_cfg != Some(key);
        self.crt_cfg = Some(key);
        let ph = self.cfg.phosphor;
        for second in [false, true] {
            let (raw, tex, crt, fresh, bg, tau, name) = if second {
                (&self.raw2, &mut self.tex2, &mut self.crt2, fresh2, Color32::BLACK, crt::tau(None), "gen2-couleur")
            } else {
                (&self.raw, &mut self.tex, &mut self.crt, fresh, ph.bg(), crt::tau(Some(ph)), "apple1-ecran")
            };
            let Some(raw) = raw else { continue };
            let scr = if second { scr2 } else { scr };
            let img = if scr.on {
                if !(fresh || changed || crt.active) {
                    continue;
                }
                let img = crt.process(raw, bg, &scr, tau, dt);
                if second && scr.rgb_mask {
                    crt.last = Some(img.clone());
                    crt.gen += 1;
                }
                img
            } else {
                if !(fresh || changed) {
                    continue;
                }
                raw.clone()
            };
            match tex {
                Some(t) => t.set(img, TextureOptions::NEAREST),
                None => *tex = Some(ctx.load_texture(name, img, TextureOptions::NEAREST)),
            }
        }
    }

    fn draw_terminal(&mut self, ctx: &egui::Context, key: (u64, bool, Phosphor)) {
        let b = &self.s.m.board;
        let visible = key.1;
        let mut cells = [[b' '; COLS]; ROWS];
        for (r, line) in cells.iter_mut().enumerate() {
            for (c, v) in line.iter_mut().enumerate() {
                *v = b.display.cell(r, c);
            }
        }
        let cursor = visible.then_some((b.display.row, b.display.col.min(COLS - 1)));
        let _ = ctx;
        self.raw = Some(screen::render(&cells, cursor, self.cfg.phosphor));
        self.drawn = Some(key);
    }

    /// True when the colour picture changed.
    fn update_color_texture(&mut self, ctx: &egui::Context) -> bool {
        let _ = ctx;
        if !self.color_card() {
            self.tex2 = None;
            self.raw2 = None;
            self.drawn2 = None;
            return false;
        }
        let b = &self.s.m.board;
        // flashing characters: 1.5 Hz (Bernie, Oct 2026)
        let flash = (b.now_secs() * 2.0 * FLASH_HZ) as u64 & 1 == 0;
        let fat = self.cfg.gen2_fat_braces;
        let lines = b.gen2.frame_lines(b.config.timing.cycles_to_ticks(b.cycles));
        let sig = gen2screen::signature_ex(&lines, &b.mem, flash, fat);
        if self.raw2.is_some() && self.drawn2 == Some(sig) {
            return false;
        }
        self.raw2 = Some(gen2screen::render_lines_ex(&lines, &b.mem, flash, fat));
        self.drawn2 = Some(sig);
        true
    }

    /// The Apple-1 screen (terminal) or the GEN2 colour screen.
    fn screen_ui(&mut self, ui: &mut egui::Ui, color: bool) {
        // never wider than what is visible: a too wide row above must not
        // push the picture off centre
        let avail = ui.available_rect_before_wrap().intersect(ui.clip_rect());
        // room for the caption under the colour screen
        let cap = if color { 18.0 } else { 0.0 };
        let area = egui::Rect::from_min_max(avail.min, egui::pos2(avail.max.x, (avail.max.y - cap).max(avail.min.y))).shrink(8.0);
        if color {
            self.screen_rect2 = area;
        } else {
            self.screen_rect = area;
        }
        let ppp = ui.ctx().pixels_per_point();
        let fit = self.fit_zoom_in(area);
        let z = match if color { self.cfg.zoom2 } else { self.cfg.zoom } {
            // the chosen zoom is kept: a screen bigger than its pane scrolls (#36)
            Zoom::Fixe(z) => z as f32,
            Zoom::Ajuster => fit,
        };
        self.eff_zoom[color as usize] = z;
        // snap the dot size to whole physical pixels when possible
        let dot = if z * ppp >= 1.0 { (z * ppp).round() / ppp } else { z };
        let size = egui::vec2(WIDTH as f32 * dot, HEIGHT as f32 * dot);
        let bg = self.cfg.phosphor.bg();
        // around the picture: dark with the dark theme, light with the light one
        let around = if ui.visuals().dark_mode { Color32::from_rgb(bg.r() / 2, bg.g() / 2, bg.b() / 2) } else { Color32::from_gray(214) };
        ui.painter().rect_filled(avail, 0.0, around);
        let fits = size.x <= avail.width() + 0.5 && size.y + cap <= avail.height() + 0.5;
        let (tex, sbg) = if color { (self.tex2.clone(), Color32::BLACK) } else { (self.tex.clone(), bg) };
        let caption = if color {
            mode_name(self.s.m.board.gen2.modes()).to_string()
        } else {
            String::new()
        };
        // no scan lines option in the window any more (command line only)
        let scan = self.cfg.scanlines;
        let mut scr = self.cfg.screening;
        if color {
            scr.on = self.cfg.screening2_on;
        }
        let phase = self.start.elapsed().as_secs_f64();
        let sel = self.scr_sel.filter(|s| s.color == color);
        let picture = std::cell::Cell::new(egui::Rect::NOTHING);
        // GEN2 vintage: the picture made at the pixel size of the screen,
        // with the fine red/green/blue phosphor stripes (needs 3+ pixels per dot)
        let mut baked_id = None;
        if color && scr.on && scr.rgb_mask && fits && dot * ppp >= 3.0 {
            if let Some(img) = &self.crt2.last {
                let (w, h) = ((size.x * ppp).round() as usize, (size.y * ppp).round() as usize);
                let tg = crt::tint_gray(&scr, phase);
                let key = (self.crt2.gen, w, h, scr, tg);
                if self.bake_key != Some(key) || self.tex_bake.is_none() {
                    let out = crt::bake(img, w, h, &scr, tg as f32 / 255.0);
                    match &mut self.tex_bake {
                        Some(t) => t.set(out, TextureOptions::NEAREST),
                        None => self.tex_bake = Some(ui.ctx().load_texture("gen2-masque", out, TextureOptions::NEAREST)),
                    }
                    self.bake_key = Some(key);
                }
                baked_id = self.tex_bake.as_ref().map(|t| t.id());
            }
        }
        let paint = |ui: &mut egui::Ui| {
            let (rect, resp) = if fits {
                // picture + caption, centred horizontally and vertically
                let mut r = egui::Rect::from_center_size(avail.center() - egui::vec2(0.0, cap / 2.0), size);
                r = r.translate(egui::vec2((r.min.x * ppp).round() / ppp - r.min.x, (r.min.y * ppp).round() / ppp - r.min.y));
                let resp = ui.allocate_rect(avail, egui::Sense::click_and_drag());
                (r, resp)
            } else {
                let (all, resp) = ui.allocate_exact_size(size + egui::vec2(0.0, cap), egui::Sense::click_and_drag());
                (egui::Rect::from_min_size(all.min, size), resp)
            };
            picture.set(rect);
            let p = ui.painter();
            p.rect_filled(rect.expand(dot * 4.0), dot * 3.0, sbg);
            if let Some(t) = &tex {
                if scr.on {
                    crt::paint(p, t.id(), baked_id, rect, &scr, ppp, phase);
                } else {
                    p.image(t.id(), rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), Color32::WHITE);
                }
            }
            if scan && !scr.on && dot * ppp >= 2.0 {
                let h = (dot * 0.4).max(1.0 / ppp);
                let shade = Color32::from_black_alpha(120);
                for y in 0..HEIGHT {
                    let y0 = rect.min.y + (y as f32 + 1.0) * dot - h;
                    p.rect_filled(egui::Rect::from_min_max(egui::pos2(rect.min.x, y0), egui::pos2(rect.max.x, y0 + h)), 0.0, shade);
                }
            }
            if let Some(s) = sel {
                let (cw, ch) = (CELL_W as f32 * dot, CELL_H as f32 * dot);
                for (r, a, b) in s.spans() {
                    let x0 = rect.min.x + a as f32 * cw;
                    let x1 = rect.min.x + (b + 1) as f32 * cw;
                    let y0 = rect.min.y + r as f32 * ch;
                    p.rect_filled(egui::Rect::from_min_max(egui::pos2(x0, y0), egui::pos2(x1, y0 + ch)), 0.0, Color32::from_rgba_unmultiplied(120, 170, 255, 90));
                }
            }
            if !caption.is_empty() {
                // centred under the picture, on one line (cut with … if too long)
                let mut job = egui::text::LayoutJob::simple_singleline(caption.clone(), egui::FontId::proportional(12.0), Color32::from_gray(150));
                job.wrap = egui::text::TextWrapping::truncate_at_width(avail.width() - 8.0);
                let g = p.layout_job(job);
                let at = egui::pos2(rect.center().x - g.size().x / 2.0, rect.bottom() + dot * 4.0 + 2.0);
                p.galley(at, g, Color32::from_gray(150));
            }
            resp
        };
        let resp = if fits {
            paint(ui)
        } else {
            egui::ScrollArea::both().id_salt(if color { "defil-couleur" } else { "defil-ecran" }).auto_shrink([false, false]).show(ui, paint).inner
        };
        // mouse selection (characters)
        let pic = picture.get();
        if let Some(pos) = resp.interact_pointer_pos().filter(|_| pic.is_positive()) {
            let cell = crate::screen::cell_at(pic, dot, pos);
            if resp.drag_started_by(egui::PointerButton::Primary) {
                // the drag starts where the button went down, not where the pointer is once egui sees a drag
                let from = ui.input(|i| i.pointer.press_origin()).map(|p| crate::screen::cell_at(pic, dot, p)).unwrap_or(cell);
                self.scr_sel = Some(crate::screen::ScreenSel { color, from, to: cell });
            } else if resp.dragged_by(egui::PointerButton::Primary) {
                if let Some(s) = self.scr_sel.as_mut().filter(|s| s.color == color) {
                    s.to = cell;
                }
            }
        }
        if resp.clicked() {
            self.scr_sel = None;
            // the Apple-1 takes the keyboard: drop egui's focus
            ui.memory_mut(|m| {
                if let Some(id) = m.focused() {
                    m.surrender_focus(id)
                }
            });
        }
        resp.context_menu(|ui| {
            if self.screen_menu_items(ui, color) {
                ui.close();
            }
        });
        // forced open (screenshots: --ecran-menu)
        if self.script.screen_menu && !color {
            let at = avail.min + egui::vec2(avail.width() * 0.35, avail.height() * 0.3);
            egui::Area::new(egui::Id::new("menu-ecran-script")).order(egui::Order::Foreground).fixed_pos(at).show(ui.ctx(), |ui| {
                egui::Frame::menu(ui.style()).show(ui, |ui| {
                    ui.set_width(220.0);
                    self.screen_menu_items(ui, false);
                });
            });
        }
    }

    /// Text of the screen `color` (whole screen, or the mouse selection).
    fn screen_text(&self, color: bool, only_selection: bool) -> String {
        let sel = self.scr_sel.filter(|s| s.color == color && only_selection);
        let sel = sel.unwrap_or(crate::screen::ScreenSel { color, from: (0, 0), to: (ROWS - 1, COLS - 1) });
        if color {
            let b = &self.s.m.board;
            let lines = b.gen2.frame_lines(b.config.timing.cycles_to_ticks(b.cycles));
            sel.text(|r| gen2screen::row_text(&lines, &b.mem, r))
        } else {
            sel.text(|r| self.s.m.board.display.row_text(r))
        }
    }

    /// Right click on a screen: Copier, Coller, Effacer l'écran, Reset.
    /// True when an item was chosen.
    fn screen_menu_items(&mut self, ui: &mut egui::Ui, color: bool) -> bool {
        let item = |ui: &mut egui::Ui, label: &str, key: &str| {
            ui.horizontal(|ui| {
                let b = ui.add(egui::Button::new(label).frame(false)).clicked();
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| ui.label(RichText::new(key).weak()));
                b
            })
            .inner
        };
        let has_sel = self.scr_sel.is_some_and(|s| s.color == color);
        if item(ui, if has_sel { t("sm_copy_sel") } else { t("sm_copy") }, "") {
            let text = self.screen_text(color, true);
            let n = text.lines().count();
            ui.ctx().copy_text(text);
            self.info(tf("sm_copied", &[&n]));
            return true;
        }
        if item(ui, t("sm_paste"), "Ctrl+V") {
            // as the toolbar's « Coller »: typed at keyboard speed
            ui.ctx().send_viewport_cmd(ViewportCommand::RequestPaste);
            self.keys_to_emu = true;
            return true;
        }
        ui.separator();
        if item(ui, t("tb_clear"), "Ctrl+L") {
            self.clear();
            return true;
        }
        if item(ui, t("tb_reset"), "Ctrl+R") {
            self.reset();
            return true;
        }
        false
    }

    /// The display row at the top of a screen pane: zoom, phosphor (Apple-1),
    /// display speed (shared by both screens), vintage screen, full screen,
    /// and « Détacher » at its right end. True: detach it.
    fn display_row(&mut self, ui: &mut egui::Ui, color: bool, detached: bool) -> bool {
        let h = self.cfg.help_mode;
        let mut detach = false;
        ui.add_space(2.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(t("tb_zoom"));
            let area = if color { self.screen_rect2 } else { self.screen_rect };
            let fit = tf("tb_fit", &[&fmt_zoom(self.fit_zoom_in(area))]);
            let before = if color { self.cfg.zoom2 } else { self.cfg.zoom };
            let mut z = before;
            let name = |z: Zoom| match z {
                Zoom::Ajuster => fit.clone(),
                Zoom::Fixe(n) => format!("{n}x"),
            };
            // a fixed zoom that does not fit is shown as used: « 4x → 3x »
            let eff = self.eff_zoom[color as usize];
            let shown = match z {
                Zoom::Fixe(n) if (eff - n as f32).abs() > 0.01 => format!("{n}x -> {}", fmt_zoom(eff)),
                _ => name(z),
            };
            egui::ComboBox::from_id_salt(("zoom-ecran", color))
                .selected_text(shown)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut z, Zoom::Ajuster, name(Zoom::Ajuster)).hint(h, t("tb_fit_tip"), "h_tb_fit");
                    for n in 1..=ZOOM_MAX {
                        ui.selectable_value(&mut z, Zoom::Fixe(n), name(Zoom::Fixe(n)));
                    }
                })
                .response
                .hint(h, t("dr_zoom_tip"), "h_tb_zoom");
            if z != before {
                if color {
                    self.cfg.zoom2 = z;
                } else {
                    self.cfg.zoom = z;
                }
                self.dirty();
            }
            if !color {
                ui.label(t("tb_phosphor"));
                self.phosphor_combo(ui, "phosphore");
            }
            ui.label(t("dr_display"));
            let thr = self.s.m.board.config.display_throttle;
            let a = ui.selectable_label(thr, t("dr_normal")).hint(h, t("dr_normal_tip"), "h_dr_display").clicked() && !thr;
            let b = ui.selectable_label(!thr, t("dr_fast")).hint(h, t("dr_fast_tip"), "h_dr_display").clicked() && thr;
            if a || b {
                self.toggle_throttle();
            }
            let on = if color { &mut self.cfg.screening2_on } else { &mut self.cfg.screening.on };
            if ui.checkbox(on, t("tb_screening")).hint(h, t("tb_screening_tip"), "h_tb_screening").changed() {
                self.dirty();
            }
            let tip = format!("{} (F11)", t("tb_fullscreen").trim_start_matches(|c: char| !c.is_alphanumeric()).trim());
            if ui.button("⛶").hint(h, &tip, "h_tb_fullscreen").clicked() {
                let fs = ui.ctx().input(|i| i.viewport().fullscreen.unwrap_or(false));
                ui.ctx().send_viewport_cmd(ViewportCommand::Fullscreen(!fs));
            }
            if !detached {
                // « Détacher » at the right end of the row (or alone, at the right, on the next one)
                let label = t("dk_detach");
                let tw = ui.painter().layout_no_wrap(label.to_string(), egui::TextStyle::Button.resolve(ui.style()), Color32::WHITE).size().x;
                let bw = tw + 2.0 * ui.spacing().button_padding.x;
                let room = |ui: &egui::Ui| ui.max_rect().right() - ui.cursor().min.x - bw;
                if room(ui) < ui.spacing().item_spacing.x {
                    ui.end_row();
                }
                let sp = room(ui) - ui.spacing().item_spacing.x;
                ui.add_space(sp.max(0.0));
                detach = ui.button(label).hint(h, t("dk_detach_tip"), "h_dk_detach").clicked();
            }
        });
        detach
    }

    /// Display speed of the terminal (~60 characters/s or instant).
    fn toggle_throttle(&mut self) {
        let m = self.s.toggle_throttle();
        self.cfg.throttle = self.s.m.board.config.display_throttle;
        self.dirty();
        self.info(m);
    }

    // -------------------------------------------------------------- toolbar

    /// The toolbar, on one row: files | machine | panels | cassette, and the
    /// window tools on the right. Icons only when the window is too narrow
    /// (Préférences › Interface › Barre compacte), else on two rows.
    /// « ⊞ Disposition ⏷ »: provided layouts, the user's own, saving.
    /// A key typed by the user. While the debugger is paused the machine does
    /// not read the keyboard: like the real single latch (KBD/KBDCR), only the
    /// last key stays pending, the others are lost (#29).
    fn latch_key(&mut self, c: u8) {
        if self.s.dbg.paused {
            self.s.m.board.key_queue.clear();
        }
        self.s.m.press_key(c);
    }

    fn layout_menu(&mut self, ui: &mut egui::Ui, label: &str, h: bool) {
        // « ⊞ Disposition ⏷ »: provided layouts, the user's own, saving
        let mut act = None;
        let names = crate::layout::sorted_names(&self.cfg.saved_layouts);
        let choice = self.cfg.layout_choice.clone();
        let label = format!("{} ⏷", label);
        // stays open on a right-click (Renommer / Supprimer); a left click on an item closes it
        let cfg = egui::containers::menu::MenuConfig::new().close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside);
        let (m, _) = egui::containers::menu::MenuButton::new(label).config(cfg).ui(ui, |ui| {
            let mark = |on: bool, s: &str| egui::Button::new(format!("{} {s}", if on { "✔" } else { "   " })).selected(on);
            if ui.button(t("wl_menu")).clicked() {
                act = Some(LayoutAct::Welcome);
                ui.close();
            }
            ui.separator();
            for pr in crate::layout::Preset::ALL {
                if ui.add(mark(choice == pr.key(), t(pr.title_key()))).clicked() {
                    act = Some(LayoutAct::Preset(pr));
                    ui.close();
                }
            }
            ui.separator();
            if ui.button(t("ly_save")).clicked() {
                act = Some(LayoutAct::Save);
                ui.close();
            }
            if !names.is_empty() {
                ui.separator();
            }
            for n in &names {
                let r = ui.add(mark(choice == format!("u:{n}"), n));
                if r.clicked() {
                    act = Some(LayoutAct::Saved(n.clone()));
                    ui.close();
                }
                if r.secondary_clicked() {
                    // egui closes a menu when another opens: the little menu is ours
                    act = Some(LayoutAct::Context(n.clone(), ui.input(|i| i.pointer.interact_pos()).unwrap_or(r.rect.left_bottom())));
                    ui.close();
                }
            }
        });
        m.hint(h, t("tb_layout_tip"), "h_tb_layout");
        if let Some(a) = act {
            self.layout_act(a);
        }
    }

    /// The single « Écran couleur » button: shows or hides the screen of the plugged colour card
    /// (GEN2, P-LAB or both). With no card it opens a small choice (not a menu inside the
    /// scrolling bar: that menu never appeared, so the P-LAB entry did nothing).
    fn color_screen_button(&mut self, ui: &mut egui::Ui, label: &str, h: bool) {
        let cards = [(Pane::Color, self.color_card() || self.cfg.carte_couleur), (Pane::Plab, self.plab_card() || self.cfg.carte_plab)];
        let active: Vec<Pane> = cards.iter().filter(|(_, on)| *on).map(|(p, _)| *p).collect();
        if active.is_empty() {
            let r = ui.button(label).hint(h, t("tb_colorscreen_none"), "h_tb_colorscreen");
            if r.clicked() {
                self.color_pick = Some((r.rect.left_bottom(), self.frame_no));
            }
            return;
        }
        let on = active.iter().any(|p| self.pane_wanted(*p));
        if ui.add(egui::Button::selectable(on, label)).hint(h, t("tb_colorscreen_tip"), "h_tb_colorscreen").clicked() {
            for p in active {
                if p == Pane::Plab && !self.plab_card() {
                    self.set_plab_card(true);
                } else if p == Pane::Color && !self.color_card() {
                    self.set_color_card(true, self.cfg.couleur_50hz);
                }
                self.set_pane_wanted(p, !on);
            }
        }
    }

    /// The GEN2 / P-LAB choice, drawn outside the toolbar so a narrow window cannot clip it.
    fn color_pick_ui(&mut self, ctx: &egui::Context) {
        let Some((pos, opened)) = self.color_pick else { return };
        let mut act: Option<bool> = None;
        let area = egui::Area::new(egui::Id::new("choix-couleur")).order(egui::Order::Foreground).fixed_pos(pos).show(ctx, |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.set_min_width(240.0);
                if ui.button(t("tb_colorscreen_gen2")).clicked() {
                    act = Some(false);
                }
                if ui.button(t("tb_colorscreen_plab")).clicked() {
                    act = Some(true);
                }
            });
        });
        let dismiss = self.frame_no != opened && ctx.input(|i| i.key_pressed(Key::Escape) || (i.pointer.any_pressed() && !area.response.contains_pointer()));
        if act.is_some() || dismiss {
            self.color_pick = None;
        }
        match act {
            Some(false) => self.set_color_card(true, self.cfg.couleur_50hz),
            Some(true) => self.set_plab_card(true),
            None => {}
        }
    }

    fn toolbar(&mut self, ui: &mut egui::Ui) {
        let h = self.cfg.help_mode;
        let lang = i18n::current();
        if self.bar_need.0 != lang {
            self.bar_need = (lang, 0.0);
        }
        let avail = ui.available_width();
        let need = self.bar_need.1.max(if self.cfg.compact_bar { 1400.0 } else { 0.0 });
        let compact = self.cfg.compact_bar && avail < need;
        // the label, or only its icon (« 📂 Charger » → « 📂 »)
        let l = |s: &str| -> String {
            match s.split_once(' ') {
                Some((i, _)) if compact && !i.chars().next().is_some_and(|c| c.is_alphanumeric()) => i.to_string(),
                _ => s.to_string(),
            }
        };
        let mut left_w = 0.0;
        // right-hand buttons stay pinned; the groups scroll (1024 px used to clip Cassette)
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let mut right = |ui: &mut egui::Ui| {
                    if ui.button("ℹ").hint(h, t("tb_about"), "h_tb_about").clicked() {
                        self.ask = Some(Ask::About);
                    }
                    if ui.selectable_label(h, RichText::new(" ? ").strong()).hint(h, t("tb_help_toggle"), "h_tb_help").clicked() {
                        self.toggle_help();
                    }
                    let tip = format!("{} — {}", t("tb_prefs").trim_start_matches(|c: char| !c.is_alphanumeric()).trim(), t("tb_prefs_tip"));
                    if ui.button("⚙").hint(h, &tip, "h_tb_prefs").clicked() {
                        self.ask = Some(Ask::Preferences);
                    }
                    ui.separator();
                    if icon_button(ui, Icon::Chip, false, (!compact).then_some("EPROM")).hint(h, t("tb_eprom_tip"), "h_tb_eprom").clicked() {
                        self.eprom.show_open();
                    }
                };
                self.right_w = ui.horizontal(|ui| right(ui)).response.rect.width();
                if self.tb_overflow {
                    let label = if self.tb_to_end { "«" } else { "»" };
                    if ui.button(label).hint(h, t("tb_overflow_tip"), "h_tb_overflow").clicked() {
                        self.tb_to_end = !self.tb_to_end;
                    }
                }
                let w = ui.available_width();
                let out = egui::ScrollArea::horizontal()
                    .id_salt("barre-outils-scroll")
                    .auto_shrink([false, true])
                    .max_width(w)
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            let start = ui.cursor().min.x;
            if ui.button(l(t("tb_load"))).hint(h, t("tb_load_tip"), "h_tb_load").clicked() {
                self.open_dialog(Op::Load);
            }
            if ui.button(l(t("tb_save"))).hint(h, t("tb_save_tip"), "h_tb_save").clicked() {
                self.ask = Some(Ask::SaveRanges { text: self.last_save_ranges.clone(), err: None });
            }
            // machine state: one button, one menu
            let label = format!("{} ⏷", l(t("tb_state")));
            let st = ui.menu_button(label, |ui| {
                if self.state_menu_items(ui) {
                    ui.close();
                }
            });
            self.state_btn = st.response.rect;
            st.response.hint(h, t("tb_state_tip"), "h_tb_state");
            ui.separator();
            // red like the RESET key of a real Apple-1 keyboard; not an arrow (the editor's Undo is one)
            let reset = egui::Button::new(RichText::new("RESET").strong().size(11.0).color(Color32::from_rgb(255, 235, 235))).fill(Color32::from_rgb(150, 40, 40));
            if ui.add(reset).hint(h, t("tb_reset_tip"), "h_tb_reset").clicked() {
                self.reset();
            }
            if ui.button(l(t("tb_clear"))).hint(h, t("tb_clear_tip"), "h_tb_clear").clicked() {
                self.clear();
            }
            // a gauge (the many ▶ of the tape would be confused with it); lit = fast
            let word = t(if self.s.fast { "tb_fast" } else { "tb_real" }).split_once(' ').map(|(_, w)| w).unwrap_or("");
            if icon_button(ui, Icon::Gauge, self.s.fast, (!compact).then_some(word)).hint(h, t("tb_speed_tip"), "h_tb_speed").clicked() {
                self.toggle_fast();
            }
            if ui.button(l(t("tb_paste"))).hint(h, t("tb_paste_tip"), "h_tb_paste").clicked() {
                ui.ctx().send_viewport_cmd(ViewportCommand::RequestPaste);
                self.keys_to_emu = true;
            }
            let tip = match &self.cfg.basic {
                _ if self.cfg.basic_g2 => t("tb_basic_g2_tip").into(),
                Some(p) => tf("tb_basic_tip", &[&p.display()]),
                None => t("tb_basic_choose_tip").into(),
            };
            let b = ui.button("BASIC").hint(h, &tip, "h_tb_basic");
            if b.clicked() {
                self.basic();
            }
            b.context_menu(|ui| {
                if ui.button(t("tb_basic_other")).clicked() {
                    self.open_dialog(Op::Basic);
                    ui.close();
                }
            });
            ui.separator();
            egui::Frame::group(ui.style()).inner_margin(egui::Margin::symmetric(4, 1)).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(t("tb_windows")).weak());
                    let lbl = l(t("tb_layout"));
                    self.layout_menu(ui, &lbl, h);
                    for (p, key, tip, hid) in [
                        (Pane::Screen, "tb_screen", "tb_screen_tip", "h_tb_screen"),
                        // the colour screen: one button for GEN2 and P-LAB (see below)
                        (Pane::Color, "tb_colorscreen", "tb_colorscreen_tip", "h_tb_colorscreen"),
                        (Pane::Editor, "tb_editor", "tb_editor_tip", "h_tb_editor"),
                        (Pane::Debugger, "tb_debugger", "tb_debugger_tip", "h_tb_debugger"),
                        (Pane::Disasm, "tb_disasm", "tb_disasm_tip", "h_tb_disasm"),
                        (Pane::Dump, "tb_dump", "tb_dump_tip", "h_tb_dump"),
                        (Pane::Library, "tb_lib", "tb_lib_tip", "h_tb_library"),
                        (Pane::Ascii, "tb_ascii", "tb_ascii_tip", "h_tb_ascii"),
                    ] {
                        if p == Pane::Color {
                            self.color_screen_button(ui, &l(t(key)), h);
                            continue;
                        }
                        let on = self.pane_wanted(p);
                        if ui.add(egui::Button::selectable(on, l(t(key)))).hint(h, t(tip), hid).clicked() {
                            self.set_pane_wanted(p, !on);
                        }
                    }
                });
            });
            ui.separator();
            let gen2 = self.cfg.carte_couleur;
            // its icon is the one of ASCII: the text, short anyway
            let g = if compact { "GEN2".to_string() } else { t("tb_gen2").to_string() };
            egui::Frame::group(ui.style()).inner_margin(egui::Margin::symmetric(4, 1)).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(t("tb_cards")).weak());
                    if ui.selectable_label(gen2, g).hint(h, t("tb_gen2_tip"), "h_tb_gen2").clicked() {
                        // same setting as Préférences › GEN2, applied at once
                        let hz50 = self.cfg.couleur_50hz;
                        self.set_color_card(!gen2, hz50);
                    }
                    let plab = self.cfg.carte_plab;
                    if ui.selectable_label(plab, "P-LAB").hint(h, t("tb_plab_tip"), "h_tb_plab").clicked() {
                        self.set_plab_card(!plab);
                    }
                });
            });
            ui.separator();
            let tape_group = egui::Frame::group(ui.style()).inner_margin(egui::Margin::symmetric(4, 1)).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(t("tb_tape"));
                    if ui.button(l(t("tb_insert"))).hint(h, t("tb_insert_tip"), "h_tb_insert").clicked() {
                        self.open_dialog(Op::InsertTape);
                    }
                    if ui.button("▶").hint(h, t("tb_play"), "h_tb_play").clicked() {
                        let r = self.s.play();
                        self.result(r);
                    }
                    if ui.button("⏹").hint(h, t("tb_stop"), "h_tb_stop").clicked() {
                        let m = self.s.stop();
                        self.info(m);
                        self.lib.scan();
                    }
                    if ui.button("⏪").hint(h, t("tb_rewind"), "h_tb_rewind").clicked() {
                        let r = self.s.rewind();
                        self.result(r);
                    }
                    let rec = self.s.record_path.is_some();
                    if ui.selectable_label(rec, RichText::new("⏺").color(Color32::from_rgb(230, 60, 60))).hint(h, t("tb_record"), "h_tb_record").clicked() {
                        if rec {
                            let m = self.s.finish_recording().unwrap_or_default();
                            self.info(m);
                            self.lib.scan();
                        } else {
                            self.open_dialog(Op::Record);
                        }
                    }
                    if ui.button("⏏").hint(h, t("tb_eject"), "h_tb_eject").clicked() {
                        let m = self.s.eject();
                        self.loading = None;
                        self.info(m);
                    }
                    let (text, frac) = match (self.s.tape_state(), &self.s.record_path) {
                        (_, Some(p)) => {
                            let n = self.s.m.board.recording.as_ref().map(|r| r.len()).unwrap_or(0);
                            (tf("tb_rec_edges", &[&file_name(p), &n]), 0.0)
                        }
                        (Some((t, f)), None) => (t, f),
                        (None, None) => (t("tb_empty").into(), 0.0),
                    };
                    let w = if compact { 110.0 } else { 150.0 };
                    // the whole text in the tip: the bar is short
                    ui.add(egui::ProgressBar::new(frac).desired_width(w).text(text.clone())).hint(h, &text, "h_tb_tape");
                    if ui.selectable_label(self.cfg.tape_scope, "📈").hint(h, t("tb_scope_tip"), "h_tb_scope").clicked() {
                        self.cfg.tape_scope = !self.cfg.tape_scope;
                        self.dirty();
                    }
                    if ui.selectable_label(self.cfg.tape_sound, if self.cfg.tape_sound { "🔊" } else { "🔇" }).hint(h, t("tb_sound_tip"), "h_tb_sound").clicked() {
                        self.cfg.tape_sound = !self.cfg.tape_sound;
                        self.dirty();
                    }
                });
            });
            if self.tb_to_end {
                ui.scroll_to_rect(tape_group.response.rect, Some(egui::Align::RIGHT));
            }
            left_w = ui.min_rect().max.x - start;
                        });
                    });
                self.tb_overflow = out.content_size.x > out.inner_rect.width() + 2.0;
            });
        });
        if !compact {
            self.bar_need = (lang, left_w + self.right_w + 24.0);
        }
    }

    fn screening_prefs(&mut self, ui: &mut egui::Ui) {
        let h = self.cfg.help_mode;
        let before = self.cfg.screening;
        let s = &mut self.cfg.screening;
        ui.checkbox(&mut s.on, t("tb_screening")).hint(h, t("tb_screening_tip"), "h_tb_screening");
        ui.add_enabled_ui(s.on, |ui| {
            ui.indent("screening", |ui| {
                ui.add(egui::Slider::new(&mut s.intensity, 0.0..=1.0).fixed_decimals(2).text(t("scr_intensity"))).hint(h, "", "h_scr_intensity");
                ui.checkbox(&mut s.gaps, t("scr_gaps")).hint(h, t("scr_gaps_tip"), "h_scr_gaps");
                ui.checkbox(&mut s.soften, t("scr_soften")).hint(h, "", "h_scr_soften");
                ui.add(egui::Slider::new(&mut s.afterglow, 0.0..=1.0).fixed_decimals(2).text(t("scr_persistence"))).hint(h, t("scr_persistence_tip"), "h_scr_persistence");
                ui.checkbox(&mut s.glow, t("scr_glow")).hint(h, "", "h_scr_glow");
                ui.checkbox(&mut s.curvature, t("scr_curvature")).hint(h, "", "h_scr_curvature");
                ui.add_enabled_ui(s.curvature, |ui| {
                    ui.add(egui::Slider::new(&mut s.curve, 0.0..=1.0).fixed_decimals(2).text(t("scr_curve"))).hint(h, "", "h_scr_curve");
                });
                ui.checkbox(&mut s.flicker, t("scr_flicker")).hint(h, "", "h_scr_flicker");
                ui.checkbox(&mut s.rgb_mask, t("scr_rgb_mask")).on_hover_text(t("scr_rgb_mask_tip"));
            });
        });
        if before != self.cfg.screening {
            self.dirty();
        }
    }

    fn phosphor_combo(&mut self, ui: &mut egui::Ui, salt: &str) {
        let before = self.cfg.phosphor;
        let h = self.cfg.help_mode;
        egui::ComboBox::from_id_salt(salt)
            .selected_text(self.cfg.phosphor.label())
            .show_ui(ui, |ui| {
                for p in Phosphor::ALL {
                    ui.selectable_value(&mut self.cfg.phosphor, p, p.label());
                }
            })
            .response
            .hint(h, "", "h_tb_phosphor");
        if before != self.cfg.phosphor {
            self.dirty();
        }
    }

    // -------------------------------------------------------------- library

    fn library_ui(&mut self, ui: &mut egui::Ui) {
        let h = self.cfg.help_mode;
        if self.cfg.library_font < crate::editor::FONT_MIN {
            self.cfg.library_font = crate::fontsz::DEFAULT;
        }
        let f = self.cfg.library_font;
        {
            let s = ui.style_mut();
            for (style, k) in [(egui::TextStyle::Body, 1.0), (egui::TextStyle::Button, 1.0), (egui::TextStyle::Monospace, 1.0), (egui::TextStyle::Small, 0.85)] {
                if let Some(id) = s.text_styles.get_mut(&style) {
                    id.size = (f * k).max(8.0);
                }
            }
        }
        ui.add_space(4.0);
        ui.heading(t("lib_title"));
        ui.horizontal_wrapped(|ui| {
            // narrow panel: icons (the text goes to the tooltip)
            let narrow = ui.available_width() < 300.0;
            let (l_folder, l_gh) = if narrow { ("📁+", "GH") } else { (t("lib_add_folder"), t("lib_add_github")) };
            let (tip_folder, tip_gh) = if narrow { (t("lib_add_folder"), t("lib_add_github")) } else { ("", "") };
            if ui.button(l_folder).hint(h, tip_folder, "h_lib_add_folder").clicked() {
                self.open_dialog(Op::AddLibrary);
            }
            if ui.button(l_gh).hint(h, tip_gh, "h_lib_add_github").clicked() {
                self.ask = Some(Ask::GitHub { text: String::new(), err: None });
            }
            if ui.button("⟲").hint(h, t("lib_refresh"), "h_lib_refresh").clicked() {
                self.lib.scan();
            }
            if crate::fontsz::control(ui, "lib", &mut self.cfg.library_font, h) {
                self.dirty();
            }
        });
        if crate::fontsz::wheel(ui, ui.max_rect(), &mut self.cfg.library_font) {
            self.dirty();
        }
        ui.label(RichText::new(t("lib_dbl_note")).small().weak());
        if let Some(e) = &self.lib.error {
            ui.colored_label(Color32::LIGHT_RED, e);
            if ui.button(t("lib_create")).clicked() {
                let r = library::populate(&self.lib.dir).map(|n| tf("lib_copied", &[&n]));
                self.result(r);
                self.lib.scan();
            }
        }
        ui.separator();
        let cx = Cx { entries: self.lib.entries.clone(), progress: self.s.tape_progress(), loading: self.loading.as_ref().map(|l| l.path.clone()), tape_src: self.s.tape_source.clone() };
        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            for ri in 0..self.lib.roots.len() {
                let root = self.lib.roots[ri].clone();
                // the main library: the five folders (and any other subfolder) directly,
                // with no root node and no loose files of that root
                if ri == 0 && root.github.is_none() {
                    let folders = self.lib.top_folders(ri);
                    for folder in &folders {
                        self.folder_ui(ui, folder, None, &cx);
                    }
                } else {
                    let tree = self.lib.tree(ri);
                    self.folder_ui(ui, &tree, Some(&root), &cx);
                }
            }
            // projects being downloaded for the first time (or failed)
            let shown: Vec<String> = self.lib.roots.iter().filter_map(|r| r.github.clone()).collect();
            for key in self.cfg.github.clone() {
                if shown.contains(&key) {
                    continue;
                }
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("⎇ {key}")).strong()).hint(h, "", "h_gh_node");
                    if ui.small_button("✖").hint(h, t("gh_remove"), "h_gh_remove").clicked() {
                        self.remove_github(&key);
                    }
                });
                self.github_status_ui(ui, &key);
                if ui.small_button(t("gh_update")).hint(h, "", "h_gh_update").clicked() {
                    if let Ok(spec) = Spec::parse(&key) {
                        self.github_job(github::Job::Sync(spec));
                    }
                }
            }
        });
        if let Some(p) = self.pending_lib_remove.take() {
            self.remove_library(&p);
        }
    }

    fn github_status_ui(&mut self, ui: &mut egui::Ui, key: &str) {
        if let Some((m, f)) = self.gh_status.get(key) {
            if *f < 0.0 {
                ui.colored_label(Color32::LIGHT_RED, RichText::new(m).small());
            } else {
                ui.add(egui::ProgressBar::new(*f).desired_width(ui.available_width()).desired_height(12.0).text(RichText::new(m).small()).animate(true));
            }
        }
    }

    /// A folder of the tree (a library folder, a GitHub project or a
    /// subfolder); its open/closed state is saved in gui.json.
    fn folder_ui(&mut self, ui: &mut egui::Ui, f: &Folder, root: Option<&Root>, cx: &Cx) {
        let h = self.cfg.help_mode;
        let key = f.path.display().to_string();
        let mut open = *self.cfg.folders_open.get(&key).unwrap_or(&root.is_some());
        let gh = root.and_then(|r| r.github.clone());
        let dir_name = f.path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| key.clone());
        let title = match (&gh, root) {
            (Some(_), Some(r)) => format!("⎇ {}  ({})", r.label, f.count()),
            (None, Some(_)) => format!("📁 {dir_name}  ({})", f.count()),
            _ => format!("📂 {}  ({})", f.name, f.count()),
        };
        let resp = egui::CollapsingHeader::new(RichText::new(title).strong()).id_salt(&key).open(Some(open)).show(ui, |ui| {
            if let Some(g) = &gh {
                if let Some(m) = self.gh_meta.get(g).cloned() {
                    ui.label(RichText::new(format!("{} · {} · {}", m.short(), m.date.replace('T', " ").trim_end_matches('Z'), m.message)).small().weak())
                        .hint(h, &tf("gh_commit_tip", &[&m.sha, &m.branch]), "h_gh_commit");
                }
                ui.horizontal_wrapped(|ui| {
                    let busy = self.gh_status.get(g).map(|s| s.1 >= 0.0).unwrap_or(false);
                    if ui.add_enabled(!busy, egui::Button::new(RichText::new(t("gh_latest")).color(if ui.visuals().dark_mode { Color32::from_rgb(90, 220, 110) } else { Color32::from_rgb(20, 120, 40) }))).hint(h, "", "h_gh_latest").clicked() {
                        if let Ok(spec) = Spec::parse(g) {
                            self.github_job(github::Job::Latest(spec));
                        }
                    }
                    if ui.add_enabled(!busy, egui::Button::new(t("gh_update"))).hint(h, "", "h_gh_update").clicked() {
                        if let Ok(spec) = Spec::parse(g) {
                            self.github_job(github::Job::Sync(spec));
                        }
                    }
                    if ui.button("🌐").hint(h, t("gh_open_page"), "h_gh_page").clicked() {
                        if let Ok(spec) = Spec::parse(g) {
                            let _ = std::process::Command::new("xdg-open").arg(spec.url()).spawn();
                        }
                    }
                    if ui.button("✖").hint(h, t("gh_remove"), "h_gh_remove").clicked() {
                        self.remove_github(g);
                    }
                });
                self.github_status_ui(ui, g);
            }
            if let Some(e) = root.and_then(|r| r.error.as_ref()) {
                ui.colored_label(Color32::LIGHT_RED, RichText::new(e).small());
            }
            for sub in &f.folders {
                self.folder_ui(ui, sub, None, cx);
            }
            for &i in &f.files {
                if let Some(e) = cx.entries.get(i) {
                    self.entry_ui(ui, e, cx);
                }
            }
            if root.is_some() && f.count() == 0 && root.and_then(|r| r.error.as_ref()).is_none() {
                ui.label(RichText::new(t("lib_empty")).small().weak());
            }
        });
        let tip = if root.is_some() { f.path.display().to_string() } else { String::new() };
        let hr = resp.header_response.hint(h, &tip, if gh.is_some() { "h_gh_node" } else { "h_lib_folder" });
        if hr.clicked() {
            open = !open;
            self.cfg.folders_open.insert(key, open);
            self.dirty();
        }
        let path = f.path.clone();
        let local_root = root.filter(|r| r.github.is_none()).map(|r| r.path.clone());
        hr.context_menu(|ui| {
            if ui.button(t("ctx_open_folder")).clicked() {
                self.open_folder(&path);
                ui.close();
            }
            if let Some(r) = &local_root {
                if *r != self.cfg.library && ui.button(t("pr_remove_folder")).clicked() {
                    self.pending_lib_remove = Some(r.clone());
                    ui.close();
                }
            }
        });
    }

    /// The four icons of a library entry: ▶ run, ⬇ load, 📼 via the cassette,
    /// ⚙ settings. `rtl`: the layout is right-to-left (drawn in reverse).
    fn entry_icons(&mut self, ui: &mut egui::Ui, e: &Entry, h: bool, fast_ok: bool, rtl: bool) {
        let dark = ui.visuals().dark_mode;
        let icon = |ui: &mut egui::Ui, g: &str, c: [u8; 3], c_light: [u8; 3], on: bool| {
            let c = if dark { c } else { c_light };
            ui.add_enabled(on, egui::Button::new(RichText::new(g).size(14.0).color(Color32::from_rgb(c[0], c[1], c[2]))).min_size(egui::vec2(22.0, 22.0)))
        };
        let mut order = [0usize, 1, 2, 3];
        if rtl {
            order.reverse();
        }
        for k in order {
            match k {
                0 => {
                    if icon(ui, "▶", [110, 210, 110], [20, 120, 30], true).hint(h, t("lib_play_tip"), "h_lib_play").clicked() {
                        self.lib_load(&e.path, true);
                        self.lib_to_editor(&e.path);
                    }
                }
                1 => {
                    if icon(ui, "⬇", [120, 185, 255], [20, 80, 180], true).hint(h, t("lib_load_tip"), "h_lib_load").clicked() {
                        self.lib_load(&e.path, false);
                        self.lib_to_editor(&e.path);
                    }
                }
                2 => {
                    let tip = if fast_ok { t("lib_tape_tip") } else { t("lib_tape_no") };
                    if icon(ui, "📼", [240, 190, 90], [150, 90, 0], fast_ok).hint(h, tip, "h_lib_tape").clicked() {
                        self.lib_load_via(&e.path, false, true);
                    }
                }
                _ => {
                    if icon(ui, "⚙", [225, 225, 225], [60, 60, 60], true).hint(h, t("lib_gear_tip"), "h_lib_gear").clicked() {
                        self.open_params(&e.path);
                    }
                }
            }
        }
    }

    fn entry_ui(&mut self, ui: &mut egui::Ui, e: &Entry, cx: &Cx) {
        let h = self.cfg.help_mode;
        let selected = self.selected.as_deref() == Some(e.path.as_path());
        let mut lines = String::new();
        let mut tip = String::new();
        let saved = self.lib.has_params(&e.path);
        let pp = saved.then(|| self.lib.params_of(&e.path));
        match &e.info {
            None => lines.push_str(t("lib_analysing")),
            Some(i) => {
                lines.push_str(t(i.kind));
                if let Some(d) = i.duration {
                    lines.push_str(&format!(" · {d:.1} s"));
                }
                let ranges = match &pp {
                    Some(p) if !p.ranges.trim().is_empty() => p.ranges.clone(),
                    _ if i.is_bin => tf("lib_bin_size", &[&i.size]),
                    _ if i.ranges.is_empty() => t("lib_unknown_addr").into(),
                    _ => i.ranges.clone(),
                };
                lines.push_str(&format!("\n{ranges}"));
                let run = match &pp {
                    Some(p) if p.kind == ProgKind::Basic => Some("G2 Basic".to_string()),
                    Some(p) if p.kind == ProgKind::BasicInteger => Some("Integer BASIC".to_string()),
                    Some(p) if !p.run_cmd.trim().is_empty() => Some(p.run_cmd.replace("\\n", " ⏎ ")),
                    _ => i.run.clone(),
                };
                if let Some(r) = run {
                    lines.push_str(&format!(" · {} {r}", t("lib_run_label")));
                }
                tip = tf("lib_size", &[&i.size]);
                if let Some(er) = &i.error {
                    tip.push_str(&format!("\n{er}"));
                }
            }
        }
        let desc = self.lib.description(&e.path).cloned();
        let mut known_tip = String::new();
        let mut job = egui::text::LayoutJob::default();
        let style = ui.style();
        let strong = egui::TextFormat { font_id: egui::TextStyle::Body.resolve(style), color: style.visuals.strong_text_color(), ..Default::default() };
        let weak = egui::TextFormat { font_id: egui::TextStyle::Small.resolve(style), color: style.visuals.weak_text_color(), ..Default::default() };
        let normal = egui::TextFormat { font_id: egui::TextStyle::Small.resolve(style), color: style.visuals.text_color(), ..Default::default() };
        job.append(&e.name, 0.0, strong);
        if let Some(m) = e.info.as_ref().and_then(|i| i.known) {
            let green = egui::TextFormat { font_id: egui::TextStyle::Small.resolve(style), color: if style.visuals.dark_mode { Color32::from_rgb(90, 200, 120) } else { Color32::from_rgb(20, 120, 50) }, ..Default::default() };
            job.append(&format!("  ✔ {}", t("lib_known")), 0.0, green);
            let how = if m.how == crate::known::How::Content { t("lib_known_content") } else { t("lib_known_name") };
            let whose = if self.lib.is_auto(&e.path) { t("lib_known_auto") } else { t("lib_known_user") };
            known_tip = format!("{}\n{how}\n{whose}", tf("lib_known_tip", &[&m.known.title]));
        }
        job.append(&format!("\n{lines}"), 0.0, weak);
        if let Some(d) = &desc {
            job.append(&format!("\n{d}"), 0.0, normal);
        }
        let mut r = None;
        let fast_ok = library::source_kind(&e.path, None).is_none();
        let narrow = ui.available_width() < 300.0;
        if narrow {
            // narrow panel: the name alone on a full-width line (truncated with « … »),
            // the icons on the next line, then the details and the description, truncated
            // (the full text is in the tooltip)
            let w = ui.available_width();
            let pad = ui.spacing().button_padding.x;
            let green = egui::TextFormat { font_id: egui::TextStyle::Small.resolve(ui.style()), color: if ui.visuals().dark_mode { Color32::from_rgb(90, 200, 120) } else { Color32::from_rgb(20, 120, 50) }, ..Default::default() };
            let mut nj = egui::text::LayoutJob::default();
            nj.append(&e.name, 0.0, egui::TextFormat { font_id: egui::TextStyle::Body.resolve(ui.style()), color: ui.visuals().strong_text_color(), ..Default::default() });
            if known_tip.is_empty() {
                nj.wrap = egui::text::TextWrapping { max_width: w - 2.0 * pad, max_rows: 1, break_anywhere: true, overflow_character: Some('…') };
            } else {
                nj.append("  ✔", 0.0, green);
                nj.wrap = egui::text::TextWrapping { max_width: w - 2.0 * pad, max_rows: 1, break_anywhere: true, overflow_character: Some('…') };
            }
            let rn = ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| ui.add(egui::Button::selectable(selected, nj))).inner;
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 3.0;
                self.entry_icons(ui, e, h, fast_ok, false);
            });
            let small = |color: Color32, rows: usize| {
                let j = egui::text::LayoutJob { wrap: egui::text::TextWrapping { max_width: w, max_rows: rows, break_anywhere: true, overflow_character: Some('…') }, ..Default::default() };
                (j, color)
            };
            let (mut ij, c) = small(ui.visuals().weak_text_color(), 1);
            ij.append(&lines.replace('\n', " · "), 0.0, egui::TextFormat { font_id: egui::TextStyle::Small.resolve(ui.style()), color: c, ..Default::default() });
            let ri = ui.add(egui::Label::new(ij).sense(egui::Sense::click()));
            let mut rr = rn.union(ri);
            if let Some(d) = &desc {
                let (mut dj, c) = small(ui.visuals().text_color(), 2);
                dj.append(d, 0.0, egui::TextFormat { font_id: egui::TextStyle::Small.resolve(ui.style()), color: c, ..Default::default() });
                rr = rr.union(ui.add(egui::Label::new(dj).sense(egui::Sense::click())));
            }
            // full text in the tooltip
            let mut full = e.name.clone();
            full.push_str(&format!("\n{lines}"));
            if let Some(d) = &desc {
                full.push_str(&format!("\n{d}"));
            }
            tip = if tip.is_empty() { full } else { format!("{full}\n{tip}") };
            r = Some(rr);
        } else {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                ui.spacing_mut().item_spacing.x = 3.0;
                self.entry_icons(ui, e, h, fast_ok, true);
                let w = ui.available_width().max(60.0);
                job.wrap.max_width = w - 2.0 * ui.spacing().button_padding.x;
                let b = ui.allocate_ui_with_layout(egui::vec2(w, 0.0), egui::Layout::top_down_justified(egui::Align::Min), |ui| ui.add(egui::Button::selectable(selected, job))).inner;
                r = Some(b);
            });
        }
        // ASCII art: a preview and the conversions
        if selected && e.info.as_ref().map(|i| i.kind == "k_ascii_art").unwrap_or(false) {
            if let Ok(text) = std::fs::read_to_string(&e.path) {
                let mut art = apple1_core::asciiart::Art::from_text(&text);
                art.fit();
                egui::Frame::new().fill(Color32::from_rgb(8, 14, 8)).inner_margin(4.0).corner_radius(3.0).show(ui, |ui| {
                    let size = ((ui.available_width() - 8.0) / 40.0 / 0.6).clamp(4.0, 11.0);
                    ui.label(RichText::new(art.text()).monospace().size(size).color(Color32::from_rgb(110, 230, 120)));
                });
                ui.horizontal_wrapped(|ui| {
                    let to_basic = ui.button(t("ax_to_basic")).hint(h, t("ax_to_basic_tip"), "h_ax_to_basic").clicked();
                    let to_asm = ui.button(t("ax_to_asm")).hint(h, t("ax_to_asm_tip"), "h_ax_to_asm").clicked();
                    for asm in [false, true] {
                        if (asm && to_asm) || (!asm && to_basic) {
                            let mut cfg = self.cfg.ascii.clone();
                            cfg.asm = asm;
                            let title = e.path.file_stem().map(|s| s.to_string_lossy().to_uppercase()).unwrap_or_default();
                            let code = if asm { apple1_core::asciiart::to_asm(&art, &title) } else { apple1_core::asciiart::to_basic(&art, &title) };
                            let _ = self.ascii.load_art(&e.path);
                            self.ascii_insert(code, asm, crate::ascii::Then::Nothing);
                        }
                    }
                    if ui.button(t("ax_to_tool")).hint(h, t("ax_to_tool_tip"), "h_ax_to_tool").clicked() {
                        self.ascii_open(&e.path);
                    }
                });
            }
        }
        // progress bar while this program is read from the cassette
        let here = cx.loading.as_deref() == Some(e.path.as_path()) || cx.tape_src.as_deref() == Some(e.path.as_path());
        if let (true, Some((frac, started, done))) = (here, cx.progress) {
            let text = if done {
                t("lib_progress_done").to_string()
            } else if started {
                tf("lib_progress", &[&format!("{:.0}", frac * 100.0)])
            } else {
                t("lib_progress_wait").to_string()
            };
            ui.add(egui::ProgressBar::new(frac).desired_width(ui.available_width()).desired_height(12.0).text(RichText::new(text).small()));
        }
        let Some(r) = r else { return };
        if !known_tip.is_empty() {
            tip = if tip.is_empty() { known_tip } else { format!("{known_tip}\n{tip}") };
        }
        let known = e.info.as_ref().and_then(|i| i.known).is_some();
        let r = r.hint(h, &tip, if known { "h_lib_known" } else { "h_lib_entry" });
        if r.clicked() {
            self.selected = Some(e.path.clone());
        }
        if r.double_clicked() {
            self.lib_open(&e.path);
        }
        let is_tape = e.info.as_ref().map(|i| i.is_tape).unwrap_or(false);
        r.context_menu(|ui| {
            self.selected = Some(e.path.clone());
            if ui.button(t("ctx_load")).clicked() {
                self.lib_load(&e.path, false);
                ui.close();
            }
            if ui.button(t("ctx_run")).clicked() {
                self.lib_load(&e.path, true);
                ui.close();
            }
            if is_tape && ui.button(t("ctx_insert")).clicked() {
                let r = self.s.insert_tape(&e.path);
                self.result(r);
                ui.close();
            }
            if ui.button(t("ctx_params")).clicked() {
                self.open_params(&e.path);
                ui.close();
            }
            if ui.button(t("ctx_rename")).clicked() {
                self.ask = Some(Ask::Rename { path: e.path.clone(), text: e.name.clone(), err: None });
                ui.close();
            }
            if ui.button(t("ctx_delete")).clicked() {
                self.ask = Some(Ask::Delete { path: e.path.clone() });
                ui.close();
            }
            ui.separator();
            if ui.button(t("ctx_open_folder")).clicked() {
                self.open_folder(&e.dir);
                ui.close();
            }
        });
    }

    fn open_folder(&mut self, dir: &Path) {
        match std::process::Command::new("xdg-open").arg(dir).spawn() {
            Ok(_) => self.info(tf("lib_opening", &[&dir.display()])),
            Err(e) => self.fail(format!("xdg-open : {e}")),
        }
    }

    // --------------------------------------------------------------- modals

    /// The little menu of a right-clicked user layout.
    fn layout_ctx_ui(&mut self, ctx: &egui::Context) {
        let Some((name, pos)) = self.layout_ctx.clone() else { return };
        let mut act = None;
        let area = egui::Area::new(egui::Id::new("layout-ctx")).order(egui::Order::Foreground).fixed_pos(pos).show(ctx, |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.set_min_width(120.0);
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                if ui.button(t("ly_rename")).clicked() {
                    act = Some(LayoutAct::Rename(name.clone()));
                }
                if ui.button(t("ly_delete")).clicked() {
                    act = Some(LayoutAct::Delete(name.clone()));
                }
                });
            });
        });
        let outside = ctx.input(|i| i.pointer.any_pressed()) && !area.response.contains_pointer();
        if act.is_some() || outside || ctx.input(|i| i.key_pressed(Key::Escape)) {
            self.layout_ctx = None;
        }
        if let Some(a) = act {
            self.layout_act(a);
        }
    }

    fn ask_ui(&mut self, ctx: &egui::Context) {
        self.layout_ctx_ui(ctx);
        if matches!(self.ask, Some(Ask::Welcome)) {
            self.welcome_window(ctx);
            return;
        }
        let Some(mut ask) = self.ask.take() else { return };
        let mut keep = true;
        let modal = egui::Modal::new(egui::Id::new("question")).show(ctx, |ui| {
            let wide = matches!(ask, Ask::Preferences | Ask::Params { .. });
            ui.set_max_width(if matches!(ask, Ask::Welcome) { 880.0 } else if wide { 640.0 } else { 520.0 });
            let enter = ui.input(|i| i.key_pressed(Key::Enter));
            match &mut ask {
                Ask::Addr { path, text, err } => {
                    ui.heading(t("q_addr_title"));
                    ui.label(tf("q_addr_text", &[&file_name(path)]));
                    let r = ui.text_edit_singleline(text);
                    r.request_focus();
                    if let Some(e) = err {
                        ui.colored_label(Color32::LIGHT_RED, e.as_str());
                    }
                    ui.horizontal(|ui| {
                        if ui.button(t("b_load")).clicked() || enter {
                            match program::parse_addr(text.trim()) {
                                Ok(a) => {
                                    let p = path.clone();
                                    keep = false;
                                    self.load_memory(&p, Some(a));
                                }
                                Err(e) => *err = Some(e),
                            }
                        }
                        if ui.button(t("b_cancel")).clicked() {
                            keep = false;
                        }
                    });
                }
                Ask::Tape { path } => {
                    ui.heading(t("q_tape_title"));
                    ui.label(tf("q_tape_text", &[&file_name(path)]));
                    ui.add_space(6.0);
                    if ui.button(t("q_tape_memory")).on_hover_text(t("q_tape_memory_tip")).clicked() {
                        let p = path.clone();
                        keep = false;
                        self.load_memory(&p, None);
                    }
                    if ui.button(t("q_tape_insert")).on_hover_text(t("q_tape_insert_tip")).clicked() {
                        let r = self.s.insert_tape(path);
                        keep = false;
                        self.result(r);
                    }
                    if ui.button(t("b_cancel")).clicked() {
                        keep = false;
                    }
                }
                Ask::SaveRanges { text, err } => {
                    ui.heading(t("q_save_title"));
                    ui.label(t("q_save_text"));
                    let r = ui.text_edit_singleline(text);
                    r.request_focus();
                    ui.label(RichText::new(t("q_save_hint")).small().weak());
                    if let Some(e) = err {
                        ui.colored_label(Color32::LIGHT_RED, e.as_str());
                    }
                    ui.horizontal(|ui| {
                        if ui.button(t("q_save_choose")).clicked() || enter {
                            match program::parse_ranges(text) {
                                Ok(l) if !l.is_empty() => {
                                    keep = false;
                                    self.last_save_ranges = text.clone();
                                    self.open_dialog(Op::Save(text.clone()));
                                }
                                Ok(_) => *err = Some(t("q_save_need").into()),
                                Err(e) => *err = Some(e),
                            }
                        }
                        if ui.button(t("b_cancel")).clicked() {
                            keep = false;
                        }
                    });
                }
                Ask::Rename { path, text, err } => {
                    ui.heading(t("q_rename_title"));
                    let r = ui.text_edit_singleline(text);
                    r.request_focus();
                    if let Some(e) = err {
                        ui.colored_label(Color32::LIGHT_RED, e.as_str());
                    }
                    ui.horizontal(|ui| {
                        if ui.button(t("q_rename_title")).clicked() || enter {
                            match self.lib.rename(path, text) {
                                Ok(p) => {
                                    keep = false;
                                    self.selected = Some(p);
                                    self.info(t("q_renamed"));
                                }
                                Err(e) => *err = Some(e),
                            }
                        }
                        if ui.button(t("b_cancel")).clicked() {
                            keep = false;
                        }
                    });
                }
                Ask::Delete { path } => {
                    ui.heading(t("q_delete_title"));
                    ui.label(tf("q_delete_text", &[&file_name(path)]));
                    ui.horizontal(|ui| {
                        if ui.button(RichText::new(t("q_delete_button")).color(Color32::LIGHT_RED)).clicked() {
                            let p = path.clone();
                            keep = false;
                            let r = self.lib.delete(&p).map(|_| tf("q_deleted", &[&file_name(&p)]));
                            self.result(r);
                        }
                        if ui.button(t("b_cancel")).clicked() {
                            keep = false;
                        }
                    });
                }
                Ask::Params { path, p, err, is_bin, len } => {
                    let name = file_name(path);
                    ui.heading(tf("p_title", &[&name]));
                    ui.label(RichText::new(tf("p_saved_in", &[&params::FILE])).small().weak());
                    ui.add_space(4.0);
                    egui::Grid::new("parametres").num_columns(2).spacing([10.0, 6.0]).show(ui, |ui| {
                        ui.label(t("p_ranges"));
                        ui.add(egui::TextEdit::singleline(&mut p.ranges).desired_width(360.0).hint_text("300.31E   4A.00FF 800.FFF")).on_hover_text(t("p_ranges_tip"));
                        ui.end_row();
                        ui.label(t("p_load"));
                        ui.add_enabled(*is_bin, egui::TextEdit::singleline(&mut p.load_addr).desired_width(80.0).hint_text("300")).on_hover_text(t("p_load_tip"));
                        ui.end_row();
                        ui.label(t("p_kind"));
                        ui.horizontal(|ui| {
                            let before = p.kind;
                            ui.radio_value(&mut p.kind, ProgKind::Machine, t("p_machine"));
                            ui.radio_value(&mut p.kind, ProgKind::Basic, t("p_basic"));
                            ui.radio_value(&mut p.kind, ProgKind::BasicInteger, t("p_basic_int"));
                            if before != p.kind {
                                if p.kind.is_basic() {
                                    p.needs_basic = true;
                                    p.run_cmd = "E2B3R\\nRUN".into();
                                } else {
                                    p.run_cmd = if p.run_addr.trim().is_empty() { String::new() } else { format!("{}R", p.run_addr.trim()) };
                                }
                            }
                        });
                        ui.end_row();
                        ui.label(t("p_run"));
                        let before = p.run_addr.clone();
                        ui.add_enabled(p.kind == ProgKind::Machine, egui::TextEdit::singleline(&mut p.run_addr).desired_width(80.0).hint_text("300"));
                        if before != p.run_addr && (p.run_cmd.trim().is_empty() || p.run_cmd == format!("{}R", before.trim())) {
                            p.run_cmd = if p.run_addr.trim().is_empty() { String::new() } else { format!("{}R", p.run_addr.trim()) };
                        }
                        ui.end_row();
                        ui.label(t("p_cmd"));
                        ui.add(egui::TextEdit::singleline(&mut p.run_cmd).desired_width(360.0).hint_text("300R   E2B3R\\nRUN")).on_hover_text(t("p_cmd_tip"));
                        ui.end_row();
                        ui.label("");
                        ui.add_enabled(p.kind == ProgKind::Machine, egui::Checkbox::new(&mut p.needs_basic, t("p_needs_basic")));
                        ui.end_row();
                        ui.label(t("p_notes"));
                        ui.add(egui::TextEdit::multiline(&mut p.notes).desired_rows(3).desired_width(360.0));
                        ui.end_row();
                    });
                    let preview = p.validate(*is_bin, *len);
                    match (&preview, &err) {
                        (_, Some(e)) => {
                            ui.colored_label(Color32::LIGHT_RED, e.as_str());
                        }
                        (Ok(plan), None) => {
                            ui.label(RichText::new(tf("p_preview", &[&plan.run_keys.replace('\r', " ⏎ ")])).small().weak());
                        }
                        (Err(e), None) => {
                            ui.colored_label(Color32::from_rgb(230, 170, 60), e.as_str());
                        }
                    }
                    if !p.auto.is_empty() {
                        ui.label(RichText::new(tf("p_auto_info", &[&p.auto])).small().color(if ui.visuals().dark_mode { Color32::from_rgb(90, 200, 120) } else { Color32::from_rgb(20, 120, 50) }));
                    }
                    ui.horizontal(|ui| {
                        if ui.button(t("b_save")).clicked() {
                            match p.validate(*is_bin, *len) {
                                Ok(_) => {
                                    keep = false;
                                    // edited by hand: never replaced by the table of known tapes
                                    if !p.auto.is_empty() && *p != self.lib.params_of(path) {
                                        p.auto.clear();
                                    }
                                    let r = self.lib.set_params(path, p.clone()).map(|_| tf("p_saved", &[&name]));
                                    if self.loaded.as_ref().map(|l| &l.0) == Some(path) {
                                        self.loaded = None;
                                    }
                                    self.result(r);
                                }
                                Err(e) => *err = Some(e),
                            }
                        } else if err.is_some() && preview.is_ok() {
                            *err = None;
                        }
                        if ui.button(t("p_from_file")).on_hover_text(t("p_from_file_tip")).clicked() {
                            let notes = std::mem::take(&mut p.notes);
                            *p = params::from_file(path);
                            p.notes = notes;
                            *err = None;
                        }
                        if ui.button(t("b_cancel")).clicked() {
                            keep = false;
                        }
                    });
                }
                Ask::Preferences => {
                    keep = self.preferences_ui(ui, enter);
                }
                Ask::GitHub { text, err } => {
                    ui.heading(t("gh_add_title"));
                    ui.label(t("gh_add_text"));
                    let r = ui.add(egui::TextEdit::singleline(text).desired_width(480.0).hint_text("https://github.com/owner/repo"));
                    r.request_focus();
                    ui.label(RichText::new(t("gh_add_hint")).small().weak());
                    if let Some(e) = err {
                        ui.colored_label(Color32::LIGHT_RED, e.as_str());
                    }
                    ui.horizontal(|ui| {
                        if ui.button(t("gh_add")).clicked() || enter {
                            match self.add_github(text) {
                                Ok(()) => keep = false,
                                Err(e) => *err = Some(e),
                            }
                        }
                        if ui.button(t("b_cancel")).clicked() {
                            keep = false;
                        }
                    });
                }
                Ask::LayoutName { text, err, old } => {
                    ui.heading(t("ly_name_title"));
                    let r = ui.add(egui::TextEdit::singleline(text).desired_width(320.0));
                    r.request_focus();
                    if let Some(e) = err {
                        ui.colored_label(Color32::LIGHT_RED, e.as_str());
                    }
                    ui.horizontal(|ui| {
                        if ui.button(t(if old.is_some() { "ly_rename" } else { "ly_save_btn" })).clicked() || enter {
                            match self.layout_name_done(text, old.as_deref()) {
                                Ok(()) => keep = false,
                                Err(e) => *err = Some(e),
                            }
                        }
                        if ui.button(t("b_cancel")).clicked() {
                            keep = false;
                        }
                    });
                }
                Ask::Welcome => {
                    // drawn as an ordinary window (welcome_window), never as this modal
                }
                Ask::Fresh { previous } => {
                    ui.heading(t("ver_title"));
                    let old = if previous.is_empty() { t("ver_old").to_string() } else { previous.clone() };
                    ui.label(tf("ver_text", &[&old, &crate::config::KIT_VERSION]));
                    ui.label(RichText::new(t("ver_keep_files")).small().weak());
                    ui.horizontal(|ui| {
                        if ui.button(t("ver_fresh")).clicked() {
                            self.accept_version(true);
                            keep = false;
                        }
                        if ui.button(t("ver_keep")).clicked() {
                            self.accept_version(false);
                            keep = false;
                        }
                    });
                }
                Ask::ConfirmReset => {
                    ui.heading(t("pr_reset"));
                    ui.label(t("pr_reset_confirm"));
                    ui.horizontal(|ui| {
                        if ui.button(t("pr_reset")).clicked() || enter {
                            self.start_fresh();
                            self.info(t("pr_reset_done"));
                            self.ask = Some(Ask::Preferences);
                            keep = false;
                        }
                        if ui.button(t("b_cancel")).clicked() {
                            self.ask = Some(Ask::Preferences);
                            keep = false;
                        }
                    });
                }
                Ask::About => {
                    ui.heading(t("about_title"));
                    ui.label(tf("about_version", &[&crate::config::KIT_VERSION]));
                    ui.hyperlink(env!("CARGO_PKG_REPOSITORY"));
                    ui.label(RichText::new(t("about_license")).small());
                    ui.label(t("help"));
                    if ui.button(t("b_close")).clicked() || enter {
                        keep = false;
                    }
                }
            }
        });
        if modal.should_close() {
            keep = false;
        }
        let was_fresh = matches!(ask, Ask::Fresh { .. });
        if keep && self.ask.is_none() {
            self.ask = Some(ask);
        } else if !keep && was_fresh && self.ask.is_none() {
            // dismissed: keep the current settings, and do not ask again
            self.accept_version(false);
        }
    }

    /// Start-up guide: an ordinary window (it does not stay above the others).
    fn welcome_window(&mut self, ctx: &egui::Context) {
        let mut result = crate::wizard::Out { close: false, preset: None, dirty: false };
        let (mw, mh) = screen_pixels(ctx);
        let zoom = ctx.zoom_factor();
        let page = &mut self.wiz_page;
        let hide = &mut self.cfg.welcome_hide;
        // never larger than the main window (1100 × 760 at first launch, 1024 × 768 screens: #54)
        let room = ctx.content_rect().size() - egui::vec2(16.0, 16.0);
        let size = egui::vec2(room.x.clamp(640.0, 1180.0), room.y.clamp(480.0, 760.0));
        egui::Window::new(t("wiz_title"))
            .id(egui::Id::new("assistant"))
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .fixed_size(size)
            .order(egui::Order::Middle)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                ui.set_min_size(size);
                result = crate::wizard::contents(ui, page, hide, (mw, mh), zoom);
            });
        if result.dirty {
            self.dirty();
        }
        if let Some(pr) = result.preset {
            self.layout_act(LayoutAct::Preset(pr));
        }
        if result.close {
            self.ask = None;
        }
    }


    /// The ⚙ Préférences window. Returns false to close it.
    fn preferences_ui(&mut self, ui: &mut egui::Ui, enter: bool) -> bool {
        let h = self.cfg.help_mode;
        let mut keep = true;
        ui.heading(t("pr_title"));
        ui.horizontal_wrapped(|ui| {
            for tab in PrefTab::ALL {
                ui.selectable_value(&mut self.pref_tab, tab, t(tab.key()));
            }
        });
        ui.separator();
        let scroll_h = (ui.ctx().content_rect().height() * 0.55).clamp(220.0, 520.0);
        egui::ScrollArea::vertical().max_height(scroll_h).show(ui, |ui| {
            ui.set_min_width(560.0);
            match self.pref_tab {
                PrefTab::Interface => {
            // language
            ui.add_space(4.0);
            ui.strong(t("pr_language"));
            let auto = tf("pr_auto", &[&self.detected.lang.native(), &self.detected.source]);
            let cur = Lang::from_code(&self.cfg.langue);
            let mut choice = cur;
            egui::ComboBox::from_id_salt("langue")
                .width(360.0)
                .selected_text(cur.map(|l| l.native().to_string()).unwrap_or(auto.clone()))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut choice, None, auto.clone());
                    for l in i18n::ALL {
                        ui.selectable_value(&mut choice, Some(l), l.native());
                    }
                })
                .response
                .hint(h, "", "h_pr_language");
            if choice != cur {
                self.cfg.langue = choice.map(|l| l.code().to_string()).unwrap_or_else(|| "auto".into());
                apply_language(&self.cfg, &self.detected);
                self.dirty();
                self.info(t("pr_language_set"));
            }
            ui.label(RichText::new(t("pr_language_tip")).small().weak());

                    self.interface_prefs(ui);
                }
                PrefTab::Editeur => self.syntax_prefs(ui),
                PrefTab::Bibliotheque => {
            // library
            ui.label(t("pr_main_folder"));
            ui.label(RichText::new(shown_path(&self.cfg.library)).monospace()).on_hover_text(self.cfg.library.display().to_string());
            ui.horizontal(|ui| {
                if ui.button(t("pr_browse")).hint(h, "", "h_pr_browse").clicked() {
                    keep = false;
                    self.reopen_prefs = true;
                    self.open_dialog(Op::LibraryDir);
                }
                let def = GuiConfig::default().library;
                if ui.add_enabled(self.cfg.library != def, egui::Button::new(t("pr_default"))).hint(h, &def.display().to_string(), "h_pr_default").clicked() {
                    self.set_library(def);
                }
                if ui.button(t("lib_open_folder")).hint(h, "", "h_lib_open_folder").clicked() {
                    let d = self.cfg.library.clone();
                    self.open_folder(&d);
                }
            });
            ui.label(t("pr_other_folders"));
            for d in self.cfg.libraries.clone() {
                ui.horizontal(|ui| {
                    if ui.small_button("✖").hint(h, t("pr_remove_folder"), "h_pr_remove_folder").clicked() {
                        self.remove_library(&d);
                    }
                    ui.label(RichText::new(shown_path(&d)).monospace()).on_hover_text(d.display().to_string());
                });
            }
            if ui.button(t("lib_add_folder")).hint(h, "", "h_lib_add_folder").clicked() {
                keep = false;
                self.reopen_prefs = true;
                self.open_dialog(Op::AddLibrary);
            }
            ui.label(t("pr_github"));
            for g in self.cfg.github.clone() {
                ui.horizontal(|ui| {
                    if ui.small_button("✖").hint(h, t("gh_remove"), "h_gh_remove").clicked() {
                        self.remove_github(&g);
                    }
                    ui.label(RichText::new(format!("⎇ {g}")).monospace());
                });
            }
            if ui.button(t("lib_add_github")).hint(h, "", "h_lib_add_github").clicked() {
                keep = false;
                self.ask = Some(Ask::GitHub { text: String::new(), err: None });
            }
            ui.horizontal(|ui| {
                ui.label(t("pr_token"));
                let r = ui.add(egui::TextEdit::singleline(&mut self.cfg.github_token).password(true).desired_width(260.0).hint_text("ghp_…")).hint(h, t("pr_token_tip"), "h_pr_token");
                if r.changed() {
                    self.dirty();
                }
            });
            if std::env::var("GITHUB_TOKEN").map(|v| !v.trim().is_empty()).unwrap_or(false) {
                ui.label(RichText::new(t("pr_token_env")).small().weak());
            }
            ui.horizontal(|ui| {
                ui.label(t("pr_position"));
                let mut right = self.cfg.library_right;
                ui.radio_value(&mut right, false, t("pr_left")).hint(h, "", "h_pr_position");
                ui.radio_value(&mut right, true, t("pr_right")).hint(h, "", "h_pr_position");
                if right != self.cfg.library_right {
                    self.cfg.library_right = right;
                    // put again on that side
                    if let Some(l) = self.cfg.layout.as_mut() {
                        crate::layout::remove(l, Pane::Library);
                    }
                    self.dirty();
                }
            });
                }
                PrefTab::Cassette => {
            // cassette: sound and oscilloscope
            let mut ch = ui.checkbox(&mut self.cfg.tape_sound, t("sc_sound")).hint(h, t("sc_sound_tip"), "h_sc_sound").changed();
            ui.horizontal(|ui| {
                ui.label(t("sc_volume"));
                ch |= ui.add(egui::Slider::new(&mut self.cfg.tape_volume, 0.0..=1.0)).hint(h, t("sc_volume_tip"), "h_sc_volume").changed();
            });
            ch |= ui.checkbox(&mut self.cfg.tape_scope, t("pr_scope")).hint(h, t("tb_scope_tip"), "h_tb_scope").changed();
            if ch {
                self.dirty();
            }

                }
                PrefTab::Gen2 => {
            // GEN2 colour card
            let mut on = self.cfg.carte_couleur;
            let mut hz50 = self.cfg.couleur_50hz;
            ui.checkbox(&mut on, t("gen2_checkbox")).hint(h, "", "h_gen2");
            ui.add_enabled_ui(on, |ui| {
                ui.horizontal(|ui| {
                    ui.label(t("gen2_scan"));
                    ui.radio_value(&mut hz50, false, t("gen2_60")).hint(h, "", "h_gen2_scan");
                    ui.radio_value(&mut hz50, true, t("gen2_50")).hint(h, "", "h_gen2_scan");
                });
                ui.horizontal(|ui| {
                    ui.label(t("gen2_braces"));
                    if ui.radio_value(&mut self.cfg.gen2_fat_braces, true, t("gen2_braces_fat")).hint(h, "", "h_gen2_braces").changed() {
                        self.dirty();
                        self.drawn2 = None;
                    }
                    if ui.radio_value(&mut self.cfg.gen2_fat_braces, false, t("gen2_braces_thin")).hint(h, "", "h_gen2_braces").changed() {
                        self.dirty();
                        self.drawn2 = None;
                    }
                });
            });
            ui.label(RichText::new(t("gen2_help")).weak().small());
            ui.label(RichText::new(t("gen2_palette")).small());
            egui::Grid::new("palette-gen2").spacing([6.0, 2.0]).show(ui, |ui| {
                for i in 0..16u8 {
                    let (r, _) = ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());
                    ui.painter().rect_filled(r, 2.0, gen2screen::color(i));
                    ui.label(RichText::new(format!("{i:X} {}", color_name(i))).small());
                    if i % 4 == 3 {
                        ui.end_row();
                    }
                }
            });
            self.set_color_card(on, hz50);
                }
                PrefTab::Avance => {
                    ui.strong(t("pr_display"));
            if ui.checkbox(&mut self.cfg.show_library, t("pr_show_library")).hint(h, "", "h_tb_library").changed() {
                self.dirty();
            }
            let mut thr = self.s.m.board.config.display_throttle;
            if ui.checkbox(&mut thr, t("pr_throttle")).hint(h, "", "h_pr_throttle").changed() {
                self.toggle_throttle();
            }
            let mut fast = self.s.fast;
            if ui.checkbox(&mut fast, t("pr_fast")).hint(h, "", "h_pr_fast").changed() {
                self.toggle_fast();
            }
                    ui.separator();
                    ui.strong(t("pr_vintage_fine"));
                    self.screening_prefs(ui);
                    ui.separator();
            // BASIC
            ui.strong(t("pr_basic"));
            if ui.radio(self.cfg.basic_g2, t("pr_basic_g2")).clicked() {
                self.cfg.basic_g2 = true;
                self.dirty();
            }
            if ui.radio(!self.cfg.basic_g2, t("pr_basic_orig")).clicked() {
                self.cfg.basic_g2 = false;
                self.dirty();
            }
            let b = self.cfg.basic.as_ref().map(|p| shown_path(p)).unwrap_or_else(|| t("pr_basic_none").into());
            ui.label(RichText::new(b).monospace());
            ui.horizontal(|ui| {
                if ui.button(t("pr_basic_choose")).hint(h, "", "h_pr_basic_choose").clicked() {
                    keep = false;
                    self.reopen_prefs = true;
                    self.open_dialog(Op::Basic);
                }
                if ui.add_enabled(self.cfg.basic.is_some(), egui::Button::new(t("pr_basic_forget"))).hint(h, "", "h_pr_basic_forget").clicked() {
                    self.cfg.basic = None;
                    self.dirty();
                }
            });

                }
            }
            ui.add_space(6.0);
            ui.label(RichText::new(tf("pr_saved_in", &[&shown_path(&GuiConfig::path())])).small().weak());
        });
        ui.horizontal(|ui| {
            if ui.button(t("pr_reset")).hint(h, t("pr_reset_tip"), "h_pr_reset").clicked() {
                self.ask = Some(Ask::ConfirmReset);
                keep = false;
            }
            if ui.button(t("b_close")).clicked() || enter {
                keep = false;
            }
        });
        if std::mem::take(&mut self.prefs_to_welcome) {
            self.ask = Some(Ask::Welcome);
            keep = false;
        }
        keep
    }

    /// Préférences › Interface (after the language): theme, icon size,
    /// compact toolbar, tooltips.
    fn interface_prefs(&mut self, ui: &mut egui::Ui) {
        use crate::config::{IconSize, Theme};
        let h = self.cfg.help_mode;
        let before = (self.cfg.theme, self.cfg.icon_size, self.cfg.compact_bar, self.cfg.tooltips);
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.strong(t("pr_theme"));
            for (th, k) in [(Theme::Sombre, "pr_theme_dark"), (Theme::Clair, "pr_theme_light"), (Theme::Systeme, "pr_theme_system")] {
                ui.radio_value(&mut self.cfg.theme, th, t(k)).hint(h, "", "h_pr_theme");
            }
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.strong(t("pr_icons"));
            egui::ComboBox::from_id_salt("taille-icones")
                .selected_text(t(self.cfg.icon_size.key()))
                .show_ui(ui, |ui| {
                    for z in IconSize::ALL {
                        ui.selectable_value(&mut self.cfg.icon_size, z, t(z.key()));
                    }
                })
                .response
                .hint(h, "", "h_pr_icons");
        });
        let ctx = ui.ctx().clone();
        let native = ctx.native_pixels_per_point().unwrap_or(1.0);
        let (px, py) = screen_pixels(&ctx);
        let (px, py) = (px as f32, py as f32);
        let scale = native * ctx.zoom_factor();
        ui.label(RichText::new(tf("pr_scale_note", &[&format!("{:.0} %", scale * 100.0), &format!("{px:.0}×{py:.0}"), &format!("{:.0} %", native * 100.0)])).small().weak());
        ui.add_space(4.0);
        ui.checkbox(&mut self.cfg.compact_bar, t("pr_compact")).hint(h, t("pr_compact_tip"), "h_pr_compact");
        ui.checkbox(&mut self.cfg.tooltips, t("pr_tooltips")).hint(h, t("pr_tooltips_tip"), "h_pr_tooltips");
        if before != (self.cfg.theme, self.cfg.icon_size, self.cfg.compact_bar, self.cfg.tooltips) {
            self.dirty();
        }
        // the « Votre espace de travail » window: shown at start-up, or on demand
        ui.add_space(6.0);
        ui.separator();
        ui.strong(t("wl_title"));
        let mut show = !self.cfg.welcome_hide;
        if ui.checkbox(&mut show, t("wl_show_start")).changed() {
            self.cfg.welcome_hide = !show;
            self.dirty();
        }
        if ui.button(t("wl_menu")).hint(h, "", "h_pr_welcome_open").clicked() {
            self.prefs_to_welcome = true;
        }
        // colours of the dump, the debugger and the buttons
        ui.add_space(6.0);
        ui.separator();
        self.colors_prefs(ui);
    }

    /// Préférences › Couleurs: dump, debugger and button colours, one set for
    /// the dark theme and one for the light theme, with a preview and a reset.
    fn colors_prefs(&mut self, ui: &mut egui::Ui) {
        use crate::uicolors::Role;
        let h = self.cfg.help_mode;
        ui.strong(t("pr_colors"));
        ui.label(RichText::new(t("pr_colors_tip")).small().weak());
        ui.add_space(6.0);
        let before = self.cfg.ui_colors.clone();
        egui::Grid::new("couleurs-interface").num_columns(3).spacing([18.0, 6.0]).show(ui, |ui| {
            ui.label("");
            ui.label(RichText::new(t("pr_colors_dark")).strong());
            ui.label(RichText::new(t("pr_colors_light")).strong());
            ui.end_row();
            for r in Role::ALL {
                ui.label(t(r.key()));
                for dark in [true, false] {
                    let mut c = self.cfg.ui_colors.rgb(r, dark);
                    if ui.color_edit_button_srgb(&mut c).hint(h, "", "h_pr_colors").changed() {
                        self.cfg.ui_colors.set(r, dark, c);
                    }
                }
                ui.end_row();
            }
        });
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            for (dark, key) in [(true, "pr_colors_reset_dark"), (false, "pr_colors_reset_light")] {
                if ui.add_enabled(!self.cfg.ui_colors.is_default(dark), egui::Button::new(t(key))).hint(h, "", "h_pr_colors_reset").clicked() {
                    self.cfg.ui_colors.reset(dark);
                }
            }
            if ui.add_enabled(!self.cfg.ui_colors.is_default(true) || !self.cfg.ui_colors.is_default(false), egui::Button::new(t("pr_colors_reset_all"))).hint(h, "", "h_pr_colors_reset").clicked() {
                self.cfg.ui_colors = Default::default();
            }
        });
        ui.add_space(8.0);
        // previews: what a dump row, the current line of the disassembly and an active button look like
        ui.horizontal(|ui| {
            for dark in [true, false] {
                let uc = &self.cfg.ui_colors;
                let rgb = |r: Role| {
                    let c = uc.rgb(r, dark);
                    Color32::from_rgb(c[0], c[1], c[2])
                };
                let (bg, fg) = if dark { (Color32::from_gray(27), Color32::from_gray(220)) } else { (Color32::from_gray(248), Color32::from_gray(30)) };
                egui::Frame::new().fill(bg).inner_margin(6.0).corner_radius(3.0).show(ui, |ui| {
                    ui.set_width(250.0);
                    let mono = |s: &str, c: Color32| RichText::new(s).monospace().color(c);
                    ui.label(RichText::new(t(if dark { "pr_colors_dark" } else { "pr_colors_light" })).small().color(fg));
                    ui.horizontal(|ui| {
                        ui.label(mono("0300: ", rgb(Role::DumpAddr)));
                        ui.label(mono("A9 00 8D ", fg));
                        ui.label(mono("1F", rgb(Role::Changed)).strong());
                        ui.label(mono(" .#..", rgb(Role::DumpAscii)));
                    });
                    // labels with a background extend to the right in a horizontal row:
                    // truncate them to the frame, or the dialog grows wider than the window (#53)
                    ui.add(egui::Label::new(mono(" ▶ FF2C  AD 10 D0  LDA $D010", fg).background_color(rgb(Role::CurLine))).truncate());
                    ui.add(egui::Label::new(mono("   FF2F  20 EF FF  JSR $FFEF", fg).background_color(rgb(Role::HlLine))).truncate());
                    ui.add(egui::Label::new(mono(" ● FF32  4C 00 03  JMP $0300", fg).background_color(rgb(Role::BrkLine))).truncate());
                    ui.add(egui::Label::new(mono("   START:", rgb(Role::Label))).truncate());
                    ui.horizontal(|ui| {
                        let (r, _) = ui.allocate_exact_size(egui::vec2(70.0, 20.0), egui::Sense::hover());
                        ui.painter().rect_filled(r, 3.0, rgb(Role::Active));
                        ui.painter().text(r.center(), egui::Align2::CENTER_CENTER, t("pr_colors_button"), egui::FontId::proportional(13.0), Color32::WHITE);
                        let (r, _) = ui.allocate_exact_size(egui::vec2(70.0, 20.0), egui::Sense::hover());
                        ui.painter().rect_filled(r, 3.0, rgb(Role::SelCell));
                        ui.painter().text(r.center(), egui::Align2::CENTER_CENTER, "$0300", egui::FontId::monospace(12.0), fg);
                    });
                });
            }
        });
        if before != self.cfg.ui_colors {
            self.dirty();
        }
    }

    /// Préférences › Éditeur: syntax colours of BASIC and of the 6502
    /// assembler, with a preview.
    fn syntax_prefs(&mut self, ui: &mut egui::Ui) {
        use crate::editor::SrcLang;
        use crate::syntax::Scheme;
        let h = self.cfg.help_mode;
        let dark = ui.visuals().dark_mode;
        let before = self.cfg.syntax;
        // text size of the editor, with its default value back
        ui.horizontal(|ui| {
            ui.strong(t("pr_ed_font"));
            let mut f = self.ed.font;
            if ui.add(egui::Slider::new(&mut f, crate::editor::FONT_MIN..=crate::editor::FONT_MAX).integer().suffix(" pt")).hint(h, t("pr_ed_font_tip"), "h_pr_ed_font").changed() {
                self.ed.font = f;
                self.ed.changed = true;
            }
            if ui.add_enabled(self.ed.font != crate::editor::FONT_DEFAULT, egui::Button::new(t("pr_ed_font_default"))).hint(h, t("pr_ed_font_default_tip"), "h_pr_ed_font_default").clicked() {
                self.ed.font_reset();
            }
        });
        ui.add_space(6.0);
        ui.strong(t("pr_syntax"));
        ui.label(RichText::new(t("pr_syntax_tip")).small().weak());
        for lang in [SrcLang::Basic, SrcLang::Asm] {
            ui.add_space(6.0);
            let lc = if lang == SrcLang::Asm { &mut self.cfg.syntax.asm } else { &mut self.cfg.syntax.basic };
            ui.horizontal(|ui| {
                ui.label(RichText::new(t(if lang == SrcLang::Asm { "pr_syntax_asm" } else { "pr_syntax_basic" })).strong());
                let was = lc.scheme;
                egui::ComboBox::from_id_salt(("schema", lang == SrcLang::Asm))
                    .selected_text(t(lc.scheme.key()))
                    .show_ui(ui, |ui| {
                        for sc in Scheme::ALL {
                            ui.selectable_value(&mut lc.scheme, sc, t(sc.key()));
                        }
                    })
                    .response
                    .hint(h, "", "h_pr_syntax");
                if lc.scheme == Scheme::Perso && was != Scheme::Perso {
                    // start from the colours shown
                    lc.custom = was.palette(dark);
                }
                if ui.button(t("pr_default_colors")).hint(h, "", "h_pr_syntax").clicked() {
                    *lc = Default::default();
                }
            });
            // the swatches: changing one makes the scheme « personnalisé »
            let mut pal = lc.palette(dark);
            let shown = pal;
            ui.horizontal_wrapped(|ui| {
                for (k, c) in pal.fields(lang) {
                    ui.color_edit_button_srgb(c);
                    ui.label(RichText::new(t(k)).small());
                    ui.add_space(6.0);
                }
            });
            if pal != shown {
                lc.scheme = Scheme::Perso;
                lc.custom = pal;
            }
            // preview
            crate::syntax::set_active(&self.cfg.syntax, dark);
            let hello = t("pr_sample_hello");
            let sample = if lang == SrcLang::Asm {
                format!("; {hello}\nstart:  ldx #0\n        lda msg,x\n        .byte \"HI\", $0D\n")
            } else {
                format!("10 REM {}\n20 FOR I=1 TO 10\n30 PRINT \"HI \";I\n40 NEXT I: GOTO 20\n", hello.to_uppercase())
            };
            let job = crate::editor::highlight_as(&sample, 14.0, &[], lang, &Default::default());
            let bg = if dark { Color32::from_gray(14) } else { Color32::from_gray(250) };
            egui::Frame::new().fill(bg).inner_margin(6.0).corner_radius(3.0).show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.label(job);
            });
        }
        if before != self.cfg.syntax {
            self.dirty();
        }
    }

    /// Theme, interface zoom (the icon size setting, a constant: never read
    /// from the monitor), tooltips and editor colours, applied when they change.
    fn apply_look(&mut self, ctx: &egui::Context) {
        use crate::config::{IconSize, Theme};
        if self.cfg.icon_size == IconSize::Auto {
            self.cfg.icon_size = IconSize::Normale;
        }
        let z = self.cfg.icon_size.zoom();
        let look = (self.cfg.theme, z);
        if self.applied_look != Some(look) {
            ctx.set_theme(match self.cfg.theme {
                Theme::Sombre => egui::ThemePreference::Dark,
                Theme::Clair => egui::ThemePreference::Light,
                Theme::Systeme => egui::ThemePreference::System,
            });
            if (ctx.zoom_factor() - z).abs() > 1e-3 {
                ctx.set_zoom_factor(z);
            }
            self.applied_look = Some(look);
        }
        set_tooltips(self.cfg.tooltips);
        let dark = ctx.global_style().visuals.dark_mode;
        crate::syntax::set_active(&self.cfg.syntax, dark);
        crate::uicolors::set_active(&self.cfg.ui_colors, dark);
        // active (selected) buttons: the colour of each theme, set when it changes
        let act = [true, false].map(|d| self.cfg.ui_colors.rgb(crate::uicolors::Role::Active, d));
        if self.applied_active != Some(act) {
            for (theme, c) in [(egui::Theme::Dark, act[0]), (egui::Theme::Light, act[1])] {
                ctx.style_mut_of(theme, |s| s.visuals.selection.bg_fill = Color32::from_rgb(c[0], c[1], c[2]));
            }
            self.applied_active = Some(act);
        }
    }

    fn status_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let mhz = self.speed.2 / 1e6;
            let sp = if self.s.fast {
                tf("st_fast", &[&format!("{mhz:.1}"), &format!("{:.0}", self.speed.2 / self.s.m.board.config.timing.effective_cps())])
            } else {
                tf("st_real", &[&format!("{mhz:.3}")])
            };
            ui.label(sp).on_hover_text(t("st_speed_tip"));
            ui.separator();
            let thr = if self.s.m.board.config.display_throttle { t("st_throttle_on") } else { t("st_throttle_off") };
            if ui.link(thr).on_hover_text(t("st_throttle_tip")).clicked() {
                self.toggle_throttle();
            }
            ui.separator();
            let tape = match (&self.s.record_path, self.s.tape_state()) {
                (Some(p), _) => tf("st_recording", &[&file_name(p)]),
                (None, Some((t, _))) => format!("📼 {t}"),
                _ => t("st_no_tape").into(),
            };
            ui.label(tape);
            ui.separator();
            let kb = if self.keys_to_emu { t("st_kb_apple") } else { t("st_kb_window") };
            ui.label(RichText::new(kb).weak());
            ui.separator();
            // the message takes what is left of the one line; all of it in the tip
            let color = if self.error { ui.visuals().error_fg_color } else { ui.visuals().text_color() };
            ui.add(egui::Label::new(RichText::new(&self.message).color(color)).truncate()).on_hover_text(&self.message);
        });
    }

    fn script_tick(&mut self, ctx: &egui::Context) {
        let t = self.start.elapsed().as_secs_f64();
        for p in std::mem::take(&mut self.script.load) {
            self.load_path(&p, true);
        }
        for p in std::mem::take(&mut self.script.tapes) {
            let r = self.s.insert_tape(&p);
            self.result(r);
        }
        let due: Vec<String> = {
            let (now, later): (Vec<_>, Vec<_>) = std::mem::take(&mut self.script.type_text).into_iter().partition(|(at, _)| *at <= t);
            self.script.type_text = later;
            now.into_iter().map(|(_, s)| s).collect()
        };
        for s in due {
            self.s.m.type_text(&s);
        }
        // library actions (wait until the library has been read)
        if !self.lib.entries.is_empty() {
            for run in [false, true] {
                let list = if run { &mut self.script.lib_run } else { &mut self.script.lib_load };
                let (now, later): (Vec<_>, Vec<_>) = std::mem::take(list).into_iter().partition(|(at, _)| *at <= t);
                *list = later;
                for (_, name) in now {
                    match self.lib_entry(&name) {
                        Some(p) => self.lib_load(&p, run),
                        None => self.fail(format!("{name} ?")),
                    }
                }
            }
            if let Some((at, name)) = self.script.lib_params.clone() {
                if at <= t {
                    self.script.lib_params = None;
                    if let Some(p) = self.lib_entry(&name) {
                        self.open_params(&p);
                    }
                }
            }
        }
        if let Some(p) = self.script.ascii.take() {
            for (k, v) in std::mem::take(&mut self.script.ascii_settings) {
                let c = &mut self.cfg.ascii;
                let f = v.replace(',', ".").parse::<f32>().unwrap_or(1.0);
                match k.as_str() {
                    "cols" | "colonnes" => c.cols = f as usize,
                    "rows" | "lignes" => c.rows = f as usize,
                    "zoom" => c.zoom = f,
                    "cx" => c.cx = f,
                    "cy" => c.cy = f,
                    "luminosite" | "brightness" => c.brightness = f,
                    "contraste" | "contrast" => c.contrast = f,
                    "gamma" => c.gamma = f,
                    "inverser" | "invert" => c.invert = v != "0",
                    "tramage" | "dither" => c.dither = v != "0",
                    "asm" | "6502" => c.asm = v != "0",
                    "basic" => c.asm = v == "0",
                    "rampe" | "ramp" => c.ramp = f as usize,
                    _ => eprintln!("--ascii-reglages : {k} ?"),
                }
            }
            self.ascii_open(&p);
        }
        if self.script.ascii_run.map(|at| at <= t).unwrap_or(false) {
            self.script.ascii_run = None;
            let asm = self.cfg.ascii.asm;
            let code = self.ascii.code(&self.cfg.ascii, asm);
            self.ascii_insert(code, asm, crate::ascii::Then::Run);
        }
        if std::mem::take(&mut self.script.screen_window) {
            self.cfg.screen_dock.detached = true;
        }
        if std::mem::take(&mut self.script.debugger) {
            self.cfg.debugger.open = true;
            self.dbg_forced = true;
            for w in std::mem::take(&mut self.script.dbg_watch) {
                if w.starts_with('$') || w.contains('.') {
                    match crate::debugger::parse_mem(&w, &[]) {
                        Some(m) if self.cfg.debugger.mems.contains(&m) => {}
                        Some(m) => self.cfg.debugger.mems.push(m),
                        None => eprintln!("--dbg-surveiller : {w} ?"),
                    }
                } else if !self.cfg.debugger.vars.contains(&w.to_ascii_uppercase()) {
                    self.cfg.debugger.vars.push(w.to_ascii_uppercase());
                }
            }
        }
        let (now, later): (Vec<_>, Vec<_>) = std::mem::take(&mut self.script.dbg_cmds).into_iter().partition(|(at, _)| *at <= t);
        self.script.dbg_cmds = later;
        for (_, c) in now {
            self.dbg.command(&mut self.s, c);
        }
        for p in std::mem::take(&mut self.script.editor_files) {
            self.cfg.editor.open = true;
            let r = self.ed.open(&p);
            self.result(r);
        }
        if self.script.editor_run.map(|at| at <= t).unwrap_or(false) && !self.lib.entries.is_empty() {
            self.script.editor_run = None;
            self.editor_run();
        }
        if self.script.editor_export.as_ref().map(|(at, _)| *at <= t).unwrap_or(false) && !self.lib.entries.is_empty() {
            let (_, path) = self.script.editor_export.take().unwrap();
            // same path as the button: build the current tab, then what the
            // save dialog calls with the chosen file
            if let Some(uid) = self.editor_built().map(|(uid, _, _)| uid) {
                self.picked(Op::EdExport(uid), path);
            }
        }
        if self.script.preferences.map(|at| at <= t).unwrap_or(false) {
            self.script.preferences = None;
            self.ask = Some(Ask::Preferences);
            if let Some(tab) = self.script.prefs_tab.take() {
                self.pref_tab = tab;
            }
        }
        if let Some((at, _)) = &self.script.capture {
            if *at <= t && self.frame_no > 3 {
                ctx.send_viewport_cmd(ViewportCommand::Screenshot(egui::UserData::default()));
                self.script.capture = self.script.capture.take().map(|(_, p)| (f64::INFINITY, p));
            }
        }
        let shot = ctx.input(|i| {
            i.raw.events.iter().find_map(|e| match e {
                egui::Event::Screenshot { image, .. } => Some(image.clone()),
                _ => None,
            })
        });
        if let (Some(img), Some((_, path))) = (shot, self.script.capture.clone()) {
            match save_png(&path, &img) {
                Ok(()) => eprintln!("capture : {} ({}x{})", path.display(), img.size[0], img.size[1]),
                Err(e) => eprintln!("capture : {e}"),
            }
            self.script.capture = None;
            if self.script.quit_after_capture {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
        }
    }
}

fn color_name(i: u8) -> &'static str {
    const KEYS: [&str; 16] = ["col0", "col1", "col2", "col3", "col4", "col5", "col6", "col7", "col8", "col9", "col10", "col11", "col12", "col13", "col14", "col15"];
    t(KEYS[(i & 15) as usize])
}

/// A path as the windows show it: the home folder is « ~ », and a path in
/// the temporary folder (/tmp…) is shown from its last two parts only.
fn shown_path(p: &Path) -> String {
    let tmp = std::env::temp_dir();
    if p.starts_with(&tmp) {
        let n: Vec<_> = p.components().rev().take(2).collect();
        let tail: PathBuf = n.into_iter().rev().collect();
        return format!("…/{}", tail.display());
    }
    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from).filter(|h| h.as_os_str().len() > 1) {
        if let Ok(rest) = p.strip_prefix(&home) {
            return format!("~/{}", rest.display());
        }
    }
    p.display().to_string()
}

fn fmt_zoom(z: f32) -> String {
    if z >= 1.0 {
        format!("{z:.0}x")
    } else {
        format!("{z:.2}x")
    }
}

pub fn save_png(path: &Path, img: &egui::ColorImage) -> Result<(), String> {
    if let Some(d) = path.parent() {
        let _ = std::fs::create_dir_all(d);
    }
    let f = std::fs::File::create(path).map_err(|e| format!("{} : {e}", path.display()))?;
    let mut enc = png::Encoder::new(std::io::BufWriter::new(f), img.size[0] as u32, img.size[1] as u32);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    let mut w = enc.write_header().map_err(|e| e.to_string())?;
    let data: Vec<u8> = img.pixels.iter().flat_map(|c| c.to_array()).collect();
    w.write_image_data(&data).map_err(|e| e.to_string())
}

const FLASH_HZ: f64 = 1.5;

impl eframe::App for App {
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        let to_emu = self.keys_to_emu;
        self.handle_keys(ctx, &mut raw.events, to_emu);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.frame_no += 1;
        self.apply_look(&ctx);
        // width the panes are laid out for: the window's own
        self.win_w = ctx.content_rect().width();
        self.mon_w = ctx.input(|i| i.viewport().monitor_size).map_or(0.0, |m| m.x);
        if self.frame_no == 1 && !self.lay_pending && self.script.window.is_none() {
            self.lay_saved = self.cfg.layout.clone();
            self.lay_w = self.win_w;
        }
        if self.lay_pending || self.lay_saved.is_some() {
            if self.frame_no > 90 || ctx.input(|i| i.pointer.any_pressed()) {
                self.lay_pending = false;
                self.lay_saved = None;
            } else if (self.win_w - self.lay_w).abs() > 8.0 {
                self.lay_w = self.win_w;
                if self.lay_pending {
                    self.cfg.layout = Some(crate::layout::default_layout(self.color_card(), self.cfg.library_right, self.win_w));
                } else if let Some(saved) = &self.lay_saved {
                    self.cfg.layout = Some(saved.clone());
                }
            }
        }
        if std::env::var_os("APPLE1_DEBUG_PPP").is_some() {
            let (ppp, nat, win, mon, max) = ctx.input(|i| (i.pixels_per_point, i.viewport().native_pixels_per_point, i.content_rect().size(), i.viewport().monitor_size, i.viewport().maximized));
            eprintln!("ppp frame={} ppp={ppp} native={nat:?} zoom={} content={}x{} monitor_pt={mon:?} maximized={max:?}", self.frame_no, ctx.zoom_factor(), win.x, win.y);
        }
        if self.frame_no == 2 {
            if let Some(w) = self.script.window {
                ctx.send_viewport_cmd(ViewportCommand::InnerSize(w.into()));
            } else if !ctx.input(|i| i.viewport().maximized.unwrap_or(false)) {
                // the window manager did not take the request at creation
                ctx.send_viewport_cmd(ViewportCommand::Maximized(true));
            }
        }
        if self.frame_no == 8 && self.script.window.is_none() {
            // a restored (unmaximized) window never bigger than the monitor
            let (inner, mon, max) = ctx.input(|i| (i.viewport().inner_rect, i.viewport().monitor_size, i.viewport().maximized.unwrap_or(false)));
            if let (Some(r), Some(m), false) = (inner, mon, max) {
                if m.x > 100.0 && (r.width() > m.x || r.height() > m.y) {
                    ctx.send_viewport_cmd(ViewportCommand::InnerSize(egui::vec2(r.width().min(m.x * 0.9), r.height().min(m.y * 0.9))));
                }
            }
        }
        // emulate up to now
        let budget = Duration::from_millis(14);
        self.s.dbg_on = self.cfg.debugger.open;
        self.s.emulate(budget);
        if let Some(st) = self.s.dbg_stopped.take() {
            let m = crate::debugger::stop_text(st, self.dbg.symbols());
            self.info(m);
        }
        self.sound_tick();
        if let Some(m) = self.s.housekeeping() {
            self.info(m);
            self.lib.scan();
        }
        let el = self.speed.0.elapsed().as_secs_f64();
        if el >= 0.5 {
            self.speed.2 = (self.s.m.board.cycles - self.speed.1) as f64 / el;
            self.speed = (Instant::now(), self.s.m.board.cycles, self.speed.2);
        }
        self.script_tick(&ctx);
        self.loading_tick();
        if self.lib.tick() | self.github_tick() {
            ctx.request_repaint();
        }
        self.update_texture(&ctx);
        self.scope_ui(&ctx);

        egui::Panel::top("barre-outils").show(ui, |ui| {
            ui.add_space(3.0);
            self.toolbar(ui);
            ui.add_space(2.0);
        });
        self.color_pick_ui(&ctx);
        egui::Panel::bottom("etat").show(ui, |ui| {
            ui.add_space(2.0);
            self.status_ui(ui);
            ui.add_space(2.0);
        });
        self.panes_ui(ui);
        self.state_ui(&ctx);
        // a picture dropped on the window: « Photo → ASCII »
        let dropped: Vec<PathBuf> = ctx.input(|i| i.raw.dropped_files.iter().map(|f| f.path().to_path_buf()).filter(|p| !p.as_os_str().is_empty()).collect());
        for p in dropped {
            if crate::ascii::is_image(&p) {
                self.ascii_open(&p);
            } else if library::is_program(&p) {
                self.load_path(&p, false);
            }
        }

        if let Some((d, op)) = &mut self.dialog {
            d.update(&ctx);
            if let Some(p) = d.take_picked() {
                let op = op.clone();
                self.dialog = None;
                self.picked(op, p);
            } else if !matches!(d.state(), egui_file_dialog::DialogState::Open) {
                self.dialog = None;
            }
            if self.dialog.is_none() && std::mem::take(&mut self.reopen_prefs) && self.ask.is_none() {
                self.ask = Some(Ask::Preferences);
            }
        }
        self.ask_ui(&ctx);

        if let Some(sz) = self.pending_win.take() {
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(sz[0], sz[1])));
        }
        // remember the window size
        let (inner, fs, max) = ctx.input(|i| (i.viewport().inner_rect, i.viewport().fullscreen.unwrap_or(false), i.viewport().maximized.unwrap_or(false)));
        if self.frame_no > 60 && !fs {
            if let Some(r) = inner {
                let sz = [r.width().round(), r.height().round()];
                if self.cfg.window != Some(sz) && !max {
                    self.cfg.window = Some(sz);
                    self.dirty();
                }
                if self.cfg.maximized != max {
                    self.cfg.maximized = max;
                    self.dirty();
                }
            }
        }
        if self.cfg_dirty.map(|t| t.elapsed().as_secs_f32() > 1.0).unwrap_or(false) && self.script.capture.is_none() {
            self.cfg.save();
            self.cfg_dirty = None;
        }
        self.keys_to_emu = self.ask.is_none() && self.dialog.is_none() && !ctx.egui_wants_keyboard_input();
        ctx.request_repaint_after(Duration::from_millis(15));
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if let Some(m) = self.s.finish_recording() {
            eprintln!("{m}");
        }
        if self.script.capture.is_none() && self.script.window.is_none() {
            self.cfg.save();
        }
    }
}

/// The panes of the dock tree (crate::layout), drawn by the App.
struct Panes<'a> {
    app: &'a mut App,
    closed: Vec<Pane>,
    detach: Vec<Pane>,
}

impl egui_dock::TabViewer for Panes<'_> {
    type Tab = Pane;

    fn id(&mut self, tab: &mut Pane) -> egui::Id {
        egui::Id::new(("volet", tab.id()))
    }

    fn title(&mut self, tab: &mut Pane) -> egui::WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Pane) {
        self.app.pane_ui(ui, *tab, false);
    }

    fn context_menu(&mut self, ui: &mut egui::Ui, tab: &mut Pane, _path: egui_dock::NodePath) {
        if ui.button(t("dk_detach")).clicked() {
            self.detach.push(*tab);
            ui.close();
        }
    }

    fn is_closeable(&self, tab: &Pane) -> bool {
        tab.closable()
    }

    fn on_close(&mut self, tab: &mut Pane) -> egui_dock::tab_viewer::OnCloseResponse {
        self.closed.push(*tab);
        egui_dock::tab_viewer::OnCloseResponse::Close
    }

    fn allowed_in_windows(&self, _tab: &mut Pane) -> bool {
        // own OS windows rather (« Détacher »)
        false
    }

    fn scroll_bars(&self, _tab: &Pane) -> [bool; 2] {
        // every pane scrolls by itself
        [false, false]
    }
}

/// See [`App::first_run_width`]: the monitor width counts during the first frames, unless a
/// window size was given on the command line.
fn width_for_first_layout(frame: u64, scripted_size: bool, monitor: f32, window: f32) -> f32 {
    if frame <= 8 && !scripted_size && monitor > 1.0 {
        window.max(monitor)
    } else {
        window
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_layout_uses_the_monitor_not_the_small_opening_window() {
        for mon in [2560.0, 1920.0, 1440.0, 1366.0] {
            assert_eq!(width_for_first_layout(2, false, mon, 1100.0), mon);
        }
        assert_eq!(width_for_first_layout(2, true, 2560.0, 1024.0), 1024.0); // --taille
        assert_eq!(width_for_first_layout(30, false, 2560.0, 1100.0), 1100.0); // later: the real size
        assert_eq!(width_for_first_layout(2, false, 0.0, 1100.0), 1100.0); // monitor unknown
        // the 1366-point laptop gets editor + screen, the 1440 one the full layout
        let small = crate::layout::SMALL_WINDOW;
        assert!(1366.0 < small && 1440.0 >= small);
    }

    #[test]
    fn colours_tab_is_gone_and_old_option_still_parses() {
        assert_eq!(PrefTab::ALL.len(), 6);
        assert_eq!(PrefTab::parse("couleurs"), Some(PrefTab::Interface));
        assert_eq!(PrefTab::parse("colors"), Some(PrefTab::Interface));
    }

    #[test]
    fn workspace_window_does_not_open_over_a_scripted_run() {
        assert!(!Script::default().automated());
        for s in [Script { state_menu: true, ..Default::default() }, Script { dialog: true, ..Default::default() }, Script { preferences: Some(1.0), ..Default::default() }, Script { eprom: true, ..Default::default() }] {
            assert!(s.automated());
        }
    }

    #[test]
    fn workspace_window_shown_by_default() {
        assert!(!crate::config::GuiConfig::default().welcome_hide);
    }

    #[test]
    fn colour_screen_is_number_seven_and_one_to_six_stay() {
        use crate::layout::Preset;
        assert_eq!(welcome_pane(Pane::Editor).0, 1);
        assert_eq!(welcome_pane(Pane::Screen).0, 2);
        assert_eq!(welcome_pane(Pane::Debugger).0, 3);
        assert_eq!(welcome_pane(Pane::Disasm).0, 4);
        assert_eq!(welcome_pane(Pane::Dump).0, 5);
        assert_eq!(welcome_pane(Pane::Library).0, 6);
        let (n, col) = welcome_pane(Pane::Color);
        assert_eq!(n, 7);
        assert_eq!(welcome_pane(Pane::Plab).0, 7);
        assert_ne!(col, welcome_pane(Pane::Screen).1);
        assert_eq!(Preset::ALL.map(|p| p.key()), ["default", "game", "basic", "asm", "debug"]);
        for pr in Preset::ALL {
            let thumb = pr.thumb();
            let screen = thumb.iter().find(|x| x.0 == Pane::Screen).unwrap();
            let color = thumb.iter().find(|x| x.0 == Pane::Color).unwrap();
            assert!(color.2 >= screen.4 - 0.001, "{pr:?}: colour screen under the terminal");
            assert_eq!((color.1, color.3), (screen.1, screen.3), "{pr:?}");
        }
    }
}
