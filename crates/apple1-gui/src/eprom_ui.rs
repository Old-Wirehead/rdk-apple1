//! « Programmateur d'EPROM » window: read, write or verify a chip with a
//! `minipro` programmer (TL866II+ / T48 / T56). The programmer runs in a
//! thread; its messages come back through a channel and fill the log.

use crate::app::Hint;
use crate::i18n::{t, tf};
use apple1_core::eprom::{self, Chip, CHIPS};
use eframe::egui::{self, Color32, RichText};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};

enum Msg {
    Line(String),
    Done,
}

/// What a job does.
#[derive(Clone)]
enum Job {
    Read { save_to: Option<PathBuf> },
    Write,
    Verify,
}

pub struct EpromUi {
    pub open: bool,
    prog: usize,
    chip: usize,
    detect: Option<Result<String, String>>,
    from_mem: bool,
    from: String,
    to: String,
    pub file: String,
    rx: Option<Receiver<Msg>>,
    log: Vec<String>,
    pub finished: bool,
}

impl Default for EpromUi {
    fn default() -> Self {
        let chip = CHIPS.iter().position(|c| c.name.starts_with("28C256")).unwrap_or(0);
        EpromUi { open: false, prog: 0, chip, detect: None, from_mem: true, from: "E000".into(), to: "EFFF".into(), file: String::new(), rx: None, log: Vec::new(), finished: false }
    }
}

/// What the window asks the application.
#[derive(Default)]
pub struct Req {
    /// Open the file chooser (for the « … » button).
    pub browse: bool,
}

fn chip_label(c: &Chip) -> String {
    tf("ep_chip_item", &[&c.name, &(c.size / 1024)])
}

/// The bytes of a file as written on the chip: a raw image (.bin) or a
/// program flattened from its first address to its last ($FF in the gaps).
pub fn file_image(path: &Path) -> Result<Vec<u8>, String> {
    let ext = path.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
    if ext == "bin" || ext == "rom" {
        return std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()));
    }
    let p = apple1_core::program::load_file(path, None)?;
    let lo = p.blocks.iter().map(|b| b.0 as usize).min().ok_or("empty file")?;
    let hi = p.blocks.iter().map(|b| b.0 as usize + b.1.len()).max().unwrap_or(lo);
    let mut img = vec![0xFFu8; hi - lo];
    for (a, d) in &p.blocks {
        let o = *a as usize - lo;
        img[o..o + d.len()].copy_from_slice(d);
    }
    Ok(img)
}

impl EpromUi {
    pub fn show_open(&mut self) {
        self.open = true;
        self.redetect();
    }

    fn redetect(&mut self) {
        self.detect = eprom::all().get(self.prog).map(|p| p.detect());
    }

    fn busy(&self) -> bool {
        self.rx.is_some()
    }

    fn data(&self, peek: &dyn Fn(u16) -> u8) -> Result<(Vec<u8>, String), String> {
        if self.from_mem {
            let a = apple1_core::program::parse_addr(&self.from).map_err(|_| tf("ep_range_err", &[&format!("{}–{}", self.from, self.to)]))?;
            let b = apple1_core::program::parse_addr(&self.to).map_err(|_| tf("ep_range_err", &[&format!("{}–{}", self.from, self.to)]))?;
            if b < a {
                return Err(tf("ep_range_err", &[&format!("{a:04X}–{b:04X}")]));
            }
            Ok(((a..=b).map(peek).collect(), format!("${a:04X}–${b:04X}")))
        } else {
            if self.file.trim().is_empty() {
                return Err(t("ep_no_file").into());
            }
            let p = PathBuf::from(self.file.trim());
            Ok((file_image(&p)?, p.display().to_string()))
        }
    }

    /// Apple-1 address of the first byte of the data (memory source), 0 for a
    /// file: differences are reported at `base + position in the chip`.
    fn base(&self) -> usize {
        if self.from_mem { apple1_core::program::parse_addr(&self.from).map(|a| a as usize).unwrap_or(0) } else { 0 }
    }

    fn start(&mut self, ctx: &egui::Context, job: Job, data: Option<Vec<u8>>) {
        let base = self.base();
        let (tx, rx) = channel();
        self.rx = Some(rx);
        let (pi, ci) = (self.prog, self.chip);
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let say = |s: String| {
                let _ = tx.send(Msg::Line(s));
                ctx.request_repaint();
            };
            let progs = eprom::all();
            let chip = &CHIPS[ci];
            let Some(p) = progs.get(pi) else { return };
            let run = || -> Result<(), String> {
                match &job {
                    Job::Read { save_to } => {
                        say(tf("ep_log_read", &[&chip.name]));
                        let got = p.read(chip)?;
                        say(tf("ep_log_read_done", &[&chip.name, &got.len()]));
                        if let Some(f) = save_to {
                            std::fs::write(f, &got).map_err(|e| format!("{}: {e}", f.display()))?;
                            say(tf("ep_log_saved", &[&f.display()]));
                        }
                        if let Some(d) = data {
                            let n = d.len().min(got.len());
                            let v = eprom::compare(&d[..n], &got[..n]);
                            say(tf("ep_log_cmp", &[&v.compared, &v.different]));
                            for (i, a, b) in v.first.iter().take(8) {
                                say(tf("ep_log_diff", &[&format!("{:04X}", base + i), &format!("{a:02X}"), &format!("{b:02X}")]));
                            }
                        }
                    }
                    Job::Write => {
                        let d = data.unwrap_or_default();
                        say(tf("ep_log_write", &[&chip.name, &d.len()]));
                        let r = p.write(chip, &d)?;
                        say(tf("ep_log_write_done", &[&r]));
                        say(tf("ep_log_verify", &[&chip.name]));
                        let mut full = d;
                        full.resize(chip.size, 0xFF);
                        let v = p.verify(chip, &full)?;
                        say(if v.ok() { tf("ep_log_verify_ok", &[&v.compared]) } else { tf("ep_log_verify_bad", &[&v.compared, &v.different]) });
                    }
                    Job::Verify => {
                        say(tf("ep_log_verify", &[&chip.name]));
                        let mut full = data.unwrap_or_default();
                        full.resize(chip.size, 0xFF);
                        let v = p.verify(chip, &full)?;
                        say(if v.ok() { tf("ep_log_verify_ok", &[&v.compared]) } else { tf("ep_log_verify_bad", &[&v.compared, &v.different]) });
                        for (i, a, b) in v.first.iter().take(8) {
                            say(tf("ep_log_diff", &[&format!("{:04X}", base + i), &format!("{a:02X}"), &format!("{b:02X}")]));
                        }
                    }
                }
                Ok(())
            };
            if let Err(e) = run() {
                say(tf("ep_log_err", &[&e]));
            }
            let _ = tx.send(Msg::Done);
            ctx.request_repaint();
        });
    }

    /// Draw the window. `peek`: side effect free read of the Apple-1 memory.
    pub fn show(&mut self, ctx: &egui::Context, h: bool, peek: &dyn Fn(u16) -> u8) -> Req {
        let mut req = Req::default();
        if !self.open {
            return req;
        }
        // messages of the job
        self.finished = false;
        let mut done = false;
        if let Some(rx) = &self.rx {
            while let Ok(m) = rx.try_recv() {
                match m {
                    Msg::Line(l) => self.log.push(l),
                    Msg::Done => done = true,
                }
            }
        }
        if done {
            self.rx = None;
            self.finished = true;
        }
        let progs = eprom::all();
        let ready = matches!(self.detect, Some(Ok(_)));
        let busy = self.busy();
        let mut open = self.open;
        egui::Window::new(t("ep_title")).id(egui::Id::new("fenetre-eprom")).open(&mut open).collapsible(false).resizable(true).default_width(620.0).anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]).show(ctx, |ui| {
            egui::Grid::new("eprom-grille").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                ui.label(t("ep_prog"));
                ui.vertical(|ui| {
                    let name = progs.get(self.prog).map(|p| p.name()).unwrap_or("?");
                    egui::ComboBox::from_id_salt("eprom-prog").selected_text(name).width(330.0).show_ui(ui, |ui| {
                        for (i, p) in progs.iter().enumerate() {
                            if ui.selectable_value(&mut self.prog, i, p.name()).clicked() {
                                self.detect = None;
                            }
                        }
                    });
                    if self.detect.is_none() {
                        self.detect = progs.get(self.prog).map(|p| p.detect());
                    }
                    match &self.detect {
                        Some(Ok(d)) => {
                            ui.colored_label(Color32::from_rgb(90, 210, 110), tf("ep_found", &[&d]));
                        }
                        _ => {
                            ui.colored_label(Color32::from_rgb(235, 90, 90), t("ep_absent"));
                            ui.label(RichText::new(t("ep_install")).small().weak());
                            if ui.button(t("ep_detect")).clicked() {
                                self.redetect();
                            }
                        }
                    }
                });
                ui.end_row();

                ui.label(t("ep_chip"));
                egui::ComboBox::from_id_salt("eprom-puce").selected_text(chip_label(&CHIPS[self.chip])).show_ui(ui, |ui| {
                    for (i, c) in CHIPS.iter().enumerate() {
                        ui.selectable_value(&mut self.chip, i, chip_label(c));
                    }
                });
                ui.end_row();

                ui.label(t("ep_data"));
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut self.from_mem, true, t("ep_from_mem"));
                        ui.add_enabled_ui(self.from_mem, |ui| {
                            ui.label("$");
                            ui.add(egui::TextEdit::singleline(&mut self.from).desired_width(48.0).font(egui::TextStyle::Monospace));
                            ui.label("–  $");
                            ui.add(egui::TextEdit::singleline(&mut self.to).desired_width(48.0).font(egui::TextStyle::Monospace));
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut self.from_mem, false, t("ep_from_file"));
                    });
                    ui.horizontal(|ui| {
                        ui.add_enabled_ui(!self.from_mem, |ui| {
                            ui.add(egui::TextEdit::singleline(&mut self.file).desired_width(360.0));
                            if ui.button("…").hint(h, t("ep_file_tip"), "h_tb_eprom").clicked() {
                                req.browse = true;
                            }
                        });
                    });
                });
                ui.end_row();
            });
            ui.add_space(6.0);
            let read_only = CHIPS[self.chip].name.contains("read only");
            ui.horizontal(|ui| {
                ui.add_enabled_ui(ready && !busy, |ui| {
                    if ui.button(t("ep_read")).hint(h, t("ep_read_tip"), "h_tb_eprom").clicked() {
                        match self.read_inputs(peek) {
                            Ok((data, save_to)) => {
                                self.log.clear();
                                self.start(ctx, Job::Read { save_to }, data);
                            }
                            Err(e) => self.log.push(tf("ep_log_err", &[&e])),
                        }
                    }
                    let w = ui.add_enabled(!read_only, egui::Button::new(RichText::new(t("ep_write")).strong()).fill(Color32::from_rgb(40, 100, 190))).hint(h, if read_only { t("ep_ro") } else { t("ep_write_tip") }, "h_tb_eprom");
                    if w.clicked() {
                        match self.data(peek) {
                            Ok((d, _)) => {
                                self.log.clear();
                                self.start(ctx, Job::Write, Some(d));
                            }
                            Err(e) => self.log.push(tf("ep_log_err", &[&e])),
                        }
                    }
                    if ui.button(t("ep_verify")).hint(h, t("ep_verify_tip"), "h_tb_eprom").clicked() {
                        match self.data(peek) {
                            Ok((d, _)) => {
                                self.log.clear();
                                self.start(ctx, Job::Verify, Some(d));
                            }
                            Err(e) => self.log.push(tf("ep_log_err", &[&e])),
                        }
                    }
                });
                if ui.add_enabled(!busy, egui::Button::new(t("ep_close"))).clicked() {
                    self.open = false;
                }
            });
            ui.add_space(4.0);
            let frac = if busy { ((ctx.input(|i| i.time) * 0.8) % 1.0) as f32 } else { 0.0 };
            ui.add(egui::ProgressBar::new(frac).desired_width(ui.available_width()).text(if busy { t("ep_busy") } else { t("ep_idle") }));
            if busy {
                ctx.request_repaint();
            }
            ui.add_space(4.0);
            egui::Frame::new().fill(ui.visuals().extreme_bg_color).inner_margin(6.0).corner_radius(3.0).show(ui, |ui| {
                egui::ScrollArea::vertical().max_height(150.0).min_scrolled_height(110.0).stick_to_bottom(true).auto_shrink([false, false]).show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    for l in &self.log {
                        ui.label(RichText::new(l).monospace().size(12.0));
                    }
                    if read_only {
                        ui.label(RichText::new(t("ep_ro")).small().weak());
                    }
                });
            });
        });
        if !open {
            self.open = false;
        }
        req
    }

    /// Data of a read: what to compare with (if any) and where to save.
    fn read_inputs(&self, peek: &dyn Fn(u16) -> u8) -> Result<(Option<Vec<u8>>, Option<PathBuf>), String> {
        if self.from_mem {
            let (d, _) = self.data(peek)?;
            Ok((Some(d), None))
        } else {
            if self.file.trim().is_empty() {
                return Err(t("ep_no_file").into());
            }
            let p = PathBuf::from(self.file.trim());
            if p.exists() {
                // an existing file is never overwritten: the chip is compared with it
                Ok((Some(file_image(&p)?), None))
            } else {
                Ok((None, Some(p)))
            }
        }
    }
}
