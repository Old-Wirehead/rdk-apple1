//! « Photo → ASCII »: a photo (jpg, png) becomes a 40x24 ASCII art of the
//! Apple-1 characters (apple1_core::asciiart), shown with the 2513 font,
//! then an Integer BASIC or 6502 program for the editor, a tape, or a .txt
//! of the « Art ASCII » library.

use std::path::{Path, PathBuf};

use apple1_core::asciiart::{self, Art, Grey, Params};
use apple1_core::display::{COLS, ROWS};
use eframe::egui::{self, Color32, ColorImage, RichText, TextureHandle, TextureOptions};
use serde::{Deserialize, Serialize};

use crate::app::Hint;
use crate::i18n::{t, tf};
use crate::screen::{self, Phosphor};

/// Folder of the library that holds the ASCII art (.txt).
pub const ART_DIR: &str = "Art ASCII";

/// The examples given with the emulator (original drawings; the two
/// portraits are conversions of CC BY-SA photos, see LICENCES.md).
pub const EXAMPLES: [(&str, &str); 11] = [
    ("pomme.txt", include_str!("../ascii/pomme.txt")),
    ("carte-apple1.txt", include_str!("../ascii/carte-apple1.txt")),
    ("puce-6502.txt", include_str!("../ascii/puce-6502.txt")),
    ("cassette.txt", include_str!("../ascii/cassette.txt")),
    ("fusee.txt", include_str!("../ascii/fusee.txt")),
    ("chat.txt", include_str!("../ascii/chat.txt")),
    ("crane.txt", include_str!("../ascii/crane.txt")),
    ("ordinateur.txt", include_str!("../ascii/ordinateur.txt")),
    ("hello-woz.txt", include_str!("../ascii/hello-woz.txt")),
    ("portrait-wozniak.txt", include_str!("../ascii/portrait-wozniak.txt")),
    ("portrait-jobs.txt", include_str!("../ascii/portrait-jobs.txt")),
];
pub const LICENCES: &str = include_str!("../ascii/LICENCES.md");

/// Write the « Art ASCII » folder in the library (files already there are
/// kept). Returns the number of files written.
pub fn install_examples(library: &Path) -> Result<usize, String> {
    let d = library.join(ART_DIR);
    std::fs::create_dir_all(&d).map_err(|e| format!("{} : {e}", d.display()))?;
    let mut n = 0;
    for (name, text) in EXAMPLES.iter().copied().chain([("LICENCES.md", LICENCES)]) {
        let p = d.join(name);
        if !p.exists() {
            std::fs::write(&p, text).map_err(|e| format!("{} : {e}", p.display()))?;
            n += 1;
        }
    }
    Ok(n)
}

/// Is this text an ASCII art (in a folder named « …ascii… », or mostly
/// symbols), rather than a note? At most 24 rows of 40 characters.
pub fn looks_art(t: &str, path: &Path) -> bool {
    let lines: Vec<&str> = t.lines().map(str::trim_end).collect();
    let n = lines.iter().rposition(|l| !l.is_empty()).map(|i| i + 1).unwrap_or(0);
    if n == 0 || n > ROWS || lines.iter().any(|l| l.chars().count() > COLS) || !t.is_ascii() {
        return false;
    }
    let in_art_dir = path.parent().and_then(|p| p.file_name()).map(|n| n.to_string_lossy().to_ascii_lowercase().contains("ascii")).unwrap_or(false);
    let ink: Vec<char> = t.chars().filter(|c| !c.is_whitespace()).collect();
    let symbols = ink.iter().filter(|c| !c.is_ascii_alphanumeric()).count();
    in_art_dir || symbols * 10 >= ink.len() * 4
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AsciiCfg {
    pub open: bool,
    pub dock: crate::dock::DockCfg,
    pub cols: usize,
    pub rows: usize,
    pub zoom: f32,
    pub cx: f32,
    pub cy: f32,
    pub brightness: f32,
    pub contrast: f32,
    pub gamma: f32,
    pub invert: bool,
    pub auto_level: bool,
    pub dither: bool,
    /// 0 full, 1 simple, 2 Apple1-Slideshow, 3 custom.
    pub ramp: usize,
    pub custom_ramp: String,
    /// Program made: 6502 (else Integer BASIC).
    pub asm: bool,
}

impl Default for AsciiCfg {
    fn default() -> Self {
        let p = Params::default();
        AsciiCfg {
            open: false,
            dock: crate::dock::DockCfg { detached: false, width: 600.0, pos: None, size: Some([760.0, 720.0]) },
            cols: p.cols,
            rows: p.rows,
            zoom: p.zoom,
            cx: p.cx,
            cy: p.cy,
            brightness: p.brightness,
            contrast: p.contrast,
            gamma: p.gamma,
            invert: p.invert,
            auto_level: p.auto_level,
            dither: p.dither,
            ramp: 0,
            custom_ramp: asciiart::RAMP_SIMPLE.into(),
            asm: false,
        }
    }
}

impl AsciiCfg {
    pub fn params(&self) -> Params {
        Params {
            cols: self.cols,
            rows: self.rows,
            zoom: self.zoom,
            cx: self.cx,
            cy: self.cy,
            brightness: self.brightness,
            contrast: self.contrast,
            gamma: self.gamma,
            invert: self.invert,
            auto_level: self.auto_level,
            dither: self.dither,
            ramp: self.ramp_text(),
            ..Params::default()
        }
    }

    pub fn ramp_text(&self) -> String {
        match self.ramp {
            1 => asciiart::RAMP_SIMPLE.into(),
            2 => asciiart::RAMP_SLIDESHOW.into(),
            3 => self.custom_ramp.clone(),
            _ => asciiart::RAMP_FULL.into(),
        }
    }

    /// Settings of the picture back to their defaults (not the output).
    fn reset(&mut self) {
        let d = AsciiCfg::default();
        *self = AsciiCfg { open: self.open, dock: self.dock.clone(), asm: self.asm, custom_ramp: self.custom_ramp.clone(), ..d };
    }
}

/// What the app does for the tool.
pub enum AsciiAction {
    /// « Ouvrir une image… »
    Open,
    /// The program in a new editor tab (6502 or BASIC), then optionally
    /// run it or export it to a tape.
    Insert { code: String, asm: bool, then: Then },
    /// The art in the library (.txt in « Art ASCII »).
    Save { text: String, name: String },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Then {
    Nothing,
    Run,
    Export,
}

struct Photo {
    path: PathBuf,
    grey: Grey,
    thumb: ColorImage,
    tex: Option<TextureHandle>,
}

#[derive(Default)]
pub struct AsciiTool {
    photo: Option<Photo>,
    /// Art opened from the library (no photo).
    text_art: Option<Art>,
    pub name: String,
    art: Art,
    made_for: Option<Params>,
    preview: Option<(TextureHandle, Art, Phosphor)>,
    show_code: bool,
}

/// Decode an image file to grey levels and a thumbnail (at most 360 px).
pub fn decode(path: &Path) -> Result<(Grey, ColorImage), String> {
    let img = image::open(path).map_err(|e| format!("{} : {e}", path.display()))?;
    let rgba = img.to_rgba8();
    let grey = Grey::from_rgba(rgba.width() as usize, rgba.height() as usize, rgba.as_raw(), 4);
    let th = img.thumbnail(360, 360).to_rgba8();
    let thumb = ColorImage::from_rgba_unmultiplied([th.width() as usize, th.height() as usize], th.as_raw());
    Ok((grey, thumb))
}

pub fn is_image(p: &Path) -> bool {
    matches!(p.extension().and_then(|e| e.to_str()).map(|e| e.to_ascii_lowercase()).as_deref(), Some("jpg" | "jpeg" | "png"))
}

fn stem(p: &Path) -> String {
    p.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default()
}

impl AsciiTool {
    pub fn load_image(&mut self, path: &Path) -> Result<String, String> {
        let (grey, thumb) = decode(path)?;
        let (w, h) = (grey.w, grey.h);
        self.photo = Some(Photo { path: path.to_path_buf(), grey, thumb, tex: None });
        self.text_art = None;
        self.made_for = None;
        self.name = stem(path);
        Ok(tf("ax_image_loaded", &[&self.name, &w, &h]))
    }

    pub fn load_art(&mut self, path: &Path) -> Result<String, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{} : {e}", path.display()))?;
        self.set_art(Art::from_text(&text), &stem(path));
        Ok(tf("ax_art_loaded", &[&self.name]))
    }

    pub fn set_art(&mut self, art: Art, name: &str) {
        self.photo = None;
        self.art = art.clone();
        self.text_art = Some(art);
        self.made_for = None;
        self.name = name.to_string();
    }

    /// The art for the current settings.
    pub fn art(&mut self, cfg: &AsciiCfg) -> &Art {
        if let Some(a) = &self.text_art {
            self.art = a.clone();
        } else if let Some(ph) = &self.photo {
            let p = cfg.params();
            if self.made_for.as_ref() != Some(&p) {
                self.art = asciiart::convert(&ph.grey, &p);
                self.made_for = Some(p);
            }
        }
        &self.art
    }

    pub fn code(&mut self, cfg: &AsciiCfg, asm: bool) -> String {
        let name = if self.name.is_empty() { "ASCII ART".to_string() } else { self.name.clone() };
        let art = self.art(cfg).clone();
        if asm {
            asciiart::to_asm(&art, &name)
        } else {
            asciiart::to_basic(&art, &name)
        }
    }

    /// The tool. Returns true when the settings changed.
    pub fn contents(&mut self, ui: &mut egui::Ui, h: bool, cfg: &mut AsciiCfg, ph: Phosphor, acts: &mut Vec<AsciiAction>) -> bool {
        let mut changed = false;
        let before = cfg.clone();
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            ui.heading(t("ax_title"));
            if ui.button(t("ax_open")).hint(h, t("ax_open_tip"), "h_ax_open").clicked() {
                acts.push(AsciiAction::Open);
            }
            ui.label(t("ax_name"));
            ui.add(egui::TextEdit::singleline(&mut self.name).desired_width(140.0)).hint(h, t("ax_name_tip"), "h_ax_name");
        });
        if self.photo.is_none() && self.text_art.is_none() {
            ui.label(RichText::new(t("ax_drop")).weak());
        }
        let art = self.art(cfg).clone();
        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            // --- the photo with the part kept, and the Apple-1 preview
            ui.horizontal_wrapped(|ui| {
                if let Some(photo) = &mut self.photo {
                    let tex = photo.tex.get_or_insert_with(|| ui.ctx().load_texture("ascii-photo", photo.thumb.clone(), TextureOptions::LINEAR)).clone();
                    let sz = tex.size_vec2();
                    let scale = (240.0 / sz.y).min(300.0 / sz.x);
                    let (rect, resp) = ui.allocate_exact_size(sz * scale, egui::Sense::click_and_drag());
                    ui.painter().image(tex.id(), rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), Color32::WHITE);
                    let (x, y, w, hh) = asciiart::crop(&photo.grey, &cfg.params());
                    let (gw, gh) = (photo.grey.w as f32, photo.grey.h as f32);
                    let r = egui::Rect::from_min_size(rect.min + egui::vec2(x / gw * rect.width(), y / gh * rect.height()), egui::vec2(w / gw * rect.width(), hh / gh * rect.height()));
                    ui.painter().rect_stroke(r, 0.0, egui::Stroke::new(2.0, Color32::from_rgb(90, 220, 110)), egui::StrokeKind::Outside);
                    // drag: move the part; wheel: zoom
                    if resp.dragged() {
                        let d = resp.drag_delta();
                        cfg.cx = (cfg.cx + d.x / rect.width()).clamp(0.0, 1.0);
                        cfg.cy = (cfg.cy + d.y / rect.height()).clamp(0.0, 1.0);
                    }
                    if resp.hovered() {
                        let s = ui.ctx().input(|i| i.smooth_scroll_delta.y);
                        if s != 0.0 {
                            cfg.zoom = (cfg.zoom * (1.0 + s / 400.0)).clamp(1.0, 8.0);
                        }
                    }
                    resp.hint(h, t("ax_photo_tip"), "h_ax_photo");
                    let _ = &photo.path;
                }
                self.preview_ui(ui, &art, ph);
            });
            ui.label(RichText::new(tf("ax_size", &[&art.lines.iter().map(|l| l.len()).max().unwrap_or(0), &art.rows()])).small().weak());
            ui.separator();
            // --- settings (photo only)
            ui.add_enabled_ui(self.photo.is_some(), |ui| {
                egui::Grid::new("ascii-reglages").num_columns(4).spacing([10.0, 4.0]).show(ui, |ui| {
                    ui.label(t("ax_cols"));
                    ui.add(egui::Slider::new(&mut cfg.cols, 1..=40)).hint(h, t("ax_cols_tip"), "h_ax_cols");
                    ui.label(t("ax_rows"));
                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(&mut cfg.rows, 1..=24)).hint(h, t("ax_rows_tip"), "h_ax_rows");
                        if ui.selectable_label(cfg.rows == 23, t("ax_rows23")).hint(h, t("ax_rows23_tip"), "h_ax_rows23").clicked() {
                            cfg.rows = if cfg.rows == 23 { 24 } else { 23 };
                        }
                    });
                    ui.end_row();
                    ui.label(t("ax_zoom"));
                    ui.add(egui::Slider::new(&mut cfg.zoom, 1.0..=8.0).logarithmic(true)).hint(h, t("ax_zoom_tip"), "h_ax_zoom");
                    ui.label(t("ax_pan"));
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut cfg.cx).range(0.0..=1.0).speed(0.005).prefix("x ")).hint(h, t("ax_pan_tip"), "h_ax_pan");
                        ui.add(egui::DragValue::new(&mut cfg.cy).range(0.0..=1.0).speed(0.005).prefix("y ")).hint(h, t("ax_pan_tip"), "h_ax_pan");
                    });
                    ui.end_row();
                    ui.label(t("ax_brightness"));
                    ui.add(egui::Slider::new(&mut cfg.brightness, -1.0..=1.0)).hint(h, "", "h_ax_brightness");
                    ui.label(t("ax_contrast"));
                    ui.add(egui::Slider::new(&mut cfg.contrast, 0.2..=4.0).logarithmic(true)).hint(h, "", "h_ax_contrast");
                    ui.end_row();
                    ui.label(t("ax_gamma"));
                    ui.add(egui::Slider::new(&mut cfg.gamma, 0.2..=5.0).logarithmic(true)).hint(h, t("ax_gamma_tip"), "h_ax_gamma");
                    ui.label("");
                    ui.horizontal_wrapped(|ui| {
                        ui.checkbox(&mut cfg.invert, t("ax_invert")).hint(h, t("ax_invert_tip"), "h_ax_invert");
                        ui.checkbox(&mut cfg.auto_level, t("ax_auto")).hint(h, t("ax_auto_tip"), "h_ax_auto");
                        ui.checkbox(&mut cfg.dither, t("ax_dither")).hint(h, t("ax_dither_tip"), "h_ax_dither");
                    });
                    ui.end_row();
                    ui.label(t("ax_ramp"));
                    ui.horizontal(|ui| {
                        let names = [t("ax_ramp_full"), t("ax_ramp_simple"), t("ax_ramp_slideshow"), t("ax_ramp_custom")];
                        egui::ComboBox::from_id_salt("ascii-rampe").selected_text(names[cfg.ramp.min(3)]).show_ui(ui, |ui| {
                            for (i, n) in names.iter().enumerate() {
                                ui.selectable_value(&mut cfg.ramp, i, *n);
                            }
                        })
                        .response
                        .hint(h, t("ax_ramp_tip"), "h_ax_ramp");
                    });
                    ui.label("");
                    if cfg.ramp == 3 {
                        ui.add(egui::TextEdit::singleline(&mut cfg.custom_ramp).desired_width(200.0).font(egui::TextStyle::Monospace)).hint(h, t("ax_custom_tip"), "h_ax_custom");
                    } else {
                        ui.label(RichText::new(String::from_utf8_lossy(&asciiart::clean_ramp(&cfg.ramp_text())).into_owned()).monospace().weak());
                    }
                    ui.end_row();
                });
                if ui.button(t("ax_reset")).hint(h, "", "h_ax_reset").clicked() {
                    cfg.reset();
                }
            });
            ui.separator();
            // --- output
            ui.horizontal_wrapped(|ui| {
                ui.label(t("ax_output"));
                ui.selectable_value(&mut cfg.asm, false, t("ax_basic")).hint(h, t("ax_basic_tip"), "h_ax_basic");
                ui.selectable_value(&mut cfg.asm, true, t("ax_asm")).hint(h, t("ax_asm_tip"), "h_ax_asm");
            });
            let has = !art.lines.is_empty();
            ui.add_enabled_ui(has, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let asm = cfg.asm;
                    if ui.button(t("ax_insert")).hint(h, t("ax_insert_tip"), "h_ax_insert").clicked() {
                        acts.push(AsciiAction::Insert { code: self.code(cfg, asm), asm, then: Then::Nothing });
                    }
                    if ui.button(RichText::new(t("ax_run")).strong()).hint(h, t("ax_run_tip"), "h_ax_run").clicked() {
                        acts.push(AsciiAction::Insert { code: self.code(cfg, asm), asm, then: Then::Run });
                    }
                    if ui.button(t("ax_export")).hint(h, t("ax_export_tip"), "h_ax_export").clicked() {
                        acts.push(AsciiAction::Insert { code: self.code(cfg, asm), asm, then: Then::Export });
                    }
                    if ui.button(t("ax_save")).hint(h, t("ax_save_tip"), "h_ax_save").clicked() {
                        acts.push(AsciiAction::Save { text: self.art(cfg).text(), name: self.name.clone() });
                    }
                    if ui.button(t("ax_copy")).hint(h, "", "h_ax_copy").clicked() {
                        ui.ctx().copy_text(self.art(cfg).text());
                    }
                    ui.toggle_value(&mut self.show_code, t("ax_show_code")).hint(h, "", "h_ax_show_code");
                });
            });
            if self.show_code && has {
                let mut code = self.code(cfg, cfg.asm);
                ui.add(egui::TextEdit::multiline(&mut code).font(egui::TextStyle::Monospace).desired_width(f32::INFINITY).desired_rows(12).interactive(true));
            }
        });
        if *cfg != before {
            changed = true;
        }
        changed
    }

    /// The art drawn by the 2513 font, at twice its size (4:3).
    fn preview_ui(&mut self, ui: &mut egui::Ui, art: &Art, ph: Phosphor) {
        let stale = self.preview.as_ref().map(|(_, a, p)| a != art || *p != ph).unwrap_or(true);
        if stale {
            let img = render_art(art, ph);
            match &mut self.preview {
                Some((tex, a, p)) => {
                    tex.set(img, TextureOptions::NEAREST);
                    *a = art.clone();
                    *p = ph;
                }
                None => self.preview = Some((ui.ctx().load_texture("ascii-apercu", img, TextureOptions::NEAREST), art.clone(), ph)),
            }
        }
        if let Some((tex, _, _)) = &self.preview {
            let w = (ui.available_width() - 8.0).clamp(280.0, 560.0);
            ui.add(egui::Image::new((tex.id(), egui::vec2(w, w * 0.75))));
        }
    }
}

/// 280x192 picture of the art with the Apple-1 font.
pub fn render_art(art: &Art, ph: Phosphor) -> ColorImage {
    let mut cells = [[b' '; COLS]; ROWS];
    for (r, l) in art.lines.iter().enumerate().take(ROWS) {
        for (c, b) in l.bytes().enumerate().take(COLS) {
            cells[r][c] = b;
        }
    }
    screen::render(&cells, None, ph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use apple1_core::intbasic;

    #[test]
    fn examples_are_art_and_their_programs_build() {
        for (name, text) in EXAMPLES {
            let p = Path::new("x").join(ART_DIR).join(name);
            assert!(looks_art(text, &p), "{name}");
            let a = Art::from_text(text);
            assert_eq!(a.text(), text.lines().map(str::trim_end).collect::<Vec<_>>().join("\n").trim_end().to_string() + "\n", "{name}: characters outside the Apple-1 set");
            assert!(a.rows() <= 23, "{name}: {} rows", a.rows());
            let l = intbasic::tokenize(&asciiart::to_basic(&a, name));
            assert!(l.errors.is_empty(), "{name}: {:?}", l.errors);
            apple1_core::asm6502::assemble_text(&asciiart::to_asm(&a, name), None, &[]).unwrap();
        }
        assert!(!looks_art("Ceci est une note.\nRien de plus.\n", Path::new("notes/a.txt")));
        assert!(looks_art("Ceci est une note.\n", Path::new("Art ASCII/a.txt")));
        assert!(!looks_art(&"#\n".repeat(30), Path::new("Art ASCII/a.txt")));
    }

    #[test]
    fn examples_are_installed_once() {
        let d = std::env::temp_dir().join(format!("a1ascii-{}", std::process::id()));
        assert_eq!(install_examples(&d).unwrap(), EXAMPLES.len() + 1);
        assert_eq!(install_examples(&d).unwrap(), 0);
        std::fs::remove_dir_all(&d).ok();
    }

    #[test]
    fn png_is_decoded() {
        let d = std::env::temp_dir().join(format!("a1png-{}", std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        let p = d.join("g.png");
        let img = image::GrayImage::from_fn(80, 48, |x, _| image::Luma([(x * 3) as u8]));
        img.save(&p).unwrap();
        let (g, th) = decode(&p).unwrap();
        assert_eq!((g.w, g.h, th.size[0] <= 360), (80, 48, true));
        let mut tool = AsciiTool::default();
        tool.load_image(&p).unwrap();
        let a = tool.art(&AsciiCfg::default()).clone();
        assert_eq!(a.rows(), 24);
        assert!(a.lines[0].ends_with('@'));
        std::fs::remove_dir_all(&d).ok();
    }
}
