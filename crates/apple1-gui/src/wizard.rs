//! Start-up guide: three screens, amber on a near-black panel.
//!
//! The frame is a thin, slightly curved amber-grey rim, a soft vignette,
//! and a faint raster only in that rim and across the large titles. Body
//! text, the legend and the thumbnails are not screened, so they stay crisp.

use eframe::egui::{self, Color32, Mesh, Pos2, Rect, RichText, Stroke};

use crate::crt;
use crate::i18n::{t, tf};
use crate::layout::{Pane, Preset};

/// Panel fill asked for by the validated mockups.
const BG: Color32 = Color32::from_rgb(0x0C, 0x07, 0x02);
const AMBER: Color32 = Color32::from_rgb(255, 176, 40);
const AMBER_SOFT: Color32 = Color32::from_rgb(212, 150, 48);
const AMBER_DIM: Color32 = Color32::from_rgb(168, 124, 52);
/// Thin rim: amber mixed with grey, not a bright orange band.
const RIM: Color32 = Color32::from_rgb(128, 104, 64);
/// Barrel bow of the rim. The vintage slider’s 0.07 maps to a small
/// factor; this pulls the corners in a little and leaves the middle of
/// each edge in place (`crt::warp`).
const BOW: f32 = 0.028;

pub struct Out {
    pub close: bool,
    pub preset: Option<Preset>,
    pub dirty: bool,
}

fn at(rect: Rect, u: f32, v: f32) -> Pos2 {
    let (a, b) = crt::warp(u * 2.0 - 1.0, v * 2.0 - 1.0, BOW);
    Pos2::new(rect.center().x + a * rect.width() * 0.5, rect.center().y + b * rect.height() * 0.5)
}

/// Walk the rectangle’s edge, `t` in 0..1, clockwise from the top-left.
fn edge(t: f32) -> (f32, f32) {
    let x = (t * 4.0).rem_euclid(4.0);
    if x < 1.0 {
        (x, 0.0)
    } else if x < 2.0 {
        (1.0, x - 1.0)
    } else if x < 3.0 {
        (3.0 - x, 1.0)
    } else {
        (0.0, 4.0 - x)
    }
}

fn inset_edge(t: f32, d: f32) -> (f32, f32) {
    let (u, v) = edge(t);
    (u.clamp(d, 1.0 - d), v.clamp(d, 1.0 - d))
}

fn grid(rect: Rect, n: usize, color_at: impl Fn(f32, f32) -> Color32) -> Mesh {
    let mut m = Mesh::default();
    for j in 0..=n {
        for i in 0..=n {
            let (u, v) = (i as f32 / n as f32, j as f32 / n as f32);
            m.colored_vertex(at(rect, u, v), color_at(u, v));
        }
    }
    for j in 0..n {
        for i in 0..n {
            let a = (j * (n + 1) + i) as u32;
            let b = a + 1;
            let c = a + n as u32 + 1;
            let d = c + 1;
            m.add_triangle(a, b, d);
            m.add_triangle(a, d, c);
        }
    }
    m
}

/// Dark panel, soft corner shade, thin curved rim, faint raster in the rim only.
fn paint_frame(ui: &egui::Ui, rect: Rect) {
    let p = ui.painter();
    p.add(grid(rect, 28, |_, _| BG));
    // vignette: darker only towards the edges
    p.add(grid(rect, 18, |u, v| {
        let (a, b) = (u * 2.0 - 1.0, v * 2.0 - 1.0);
        let r = (a * a * 0.55 + b * b * 0.7).sqrt();
        let alpha = ((r - 0.78).max(0.0) / 0.35).powf(1.8) * 64.0;
        Color32::from_black_alpha(alpha.clamp(0.0, 64.0) as u8)
    }));
    // thin rim
    let segs = 72;
    let mut rim = Mesh::default();
    let d = 0.005;
    for i in 0..=segs {
        let t = i as f32 / segs as f32;
        let (u, v) = edge(t);
        let (ui_, vi) = inset_edge(t, d);
        rim.colored_vertex(at(rect, u, v), RIM);
        rim.colored_vertex(at(rect, ui_, vi), Color32::from_rgba_unmultiplied(RIM.r(), RIM.g(), RIM.b(), 0));
    }
    for i in 0..segs as u32 {
        let a = i * 2;
        rim.add_triangle(a, a + 1, a + 3);
        rim.add_triangle(a, a + 3, a + 2);
    }
    p.add(rim);
    // faint scanlines, only in the top and bottom of the rim, bowed with it
    let ppp = ui.ctx().pixels_per_point().max(1.0);
    let col = Color32::from_black_alpha(28);
    let band = 14.0 / rect.height();
    let mut y = (rect.top() * ppp).ceil();
    let bot = rect.bottom() * ppp;
    while y < bot {
        let v = ((y / ppp) - rect.top()) / rect.height();
        if v < band || v > 1.0 - band {
            let mut pts = Vec::with_capacity(25);
            for i in 0..=24 {
                pts.push(at(rect, i as f32 / 24.0, v.clamp(0.0, 1.0)));
            }
            p.add(egui::Shape::line(pts, Stroke::new(1.0 / ppp, col)));
        }
        y += 4.0;
    }
}

/// A few faint lines across a large title. Too light and too far apart to moiré.
fn title_raster(ui: &egui::Ui, rect: Rect) {
    let ppp = ui.ctx().pixels_per_point().max(1.0);
    let p = ui.painter();
    let col = Color32::from_black_alpha(42);
    let mut y = (rect.top() * ppp).ceil();
    let bot = rect.bottom() * ppp;
    while y < bot {
        let yy = y / ppp;
        p.line_segment([Pos2::new(rect.left(), yy), Pos2::new(rect.right(), yy)], Stroke::new(1.0 / ppp, col));
        y += 5.0;
    }
}

fn title(ui: &mut egui::Ui, text: &str) {
    let galley = ui.painter().layout(text.to_string(), egui::FontId::proportional(28.0), AMBER, ui.available_width());
    let (rect, _) = ui.allocate_exact_size(galley.size(), egui::Sense::hover());
    ui.painter().galley(rect.min, galley, AMBER);
    title_raster(ui, rect);
}

fn heading(ui: &mut egui::Ui, text: &str) {
    ui.add_space(14.0);
    ui.label(RichText::new(text).color(AMBER).size(20.0).strong());
}

fn amber_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    let galley = ui.painter().layout_no_wrap(label.to_string(), egui::FontId::proportional(15.0), AMBER);
    let pad = egui::vec2(16.0, 8.0);
    let (rect, resp) = ui.allocate_exact_size(galley.size() + pad * 2.0, egui::Sense::click());
    let fill = if resp.hovered() { Color32::from_rgb(36, 22, 8) } else { Color32::TRANSPARENT };
    ui.painter().rect(rect, 6.0, fill, Stroke::new(1.0, AMBER_SOFT), egui::StrokeKind::Inside);
    ui.painter().galley(rect.min + pad, galley, AMBER);
    resp
}

fn checkbox(ui: &mut egui::Ui, hide: &mut bool) -> bool {
    let mut show = !*hide;
    let changed = ui.checkbox(&mut show, RichText::new(t("wl_show_start")).color(AMBER_SOFT).size(15.0)).changed();
    if changed {
        *hide = !show;
    }
    changed
}

/// One screen. `page` is 0, 1 or 2.
pub fn contents(ui: &mut egui::Ui, page: &mut u8, hide: &mut bool, screen: (u32, u32), zoom: f32) -> Out {
    let mut out = Out { close: false, preset: None, dirty: false };
    let full = ui.max_rect();
    paint_frame(ui, full);
    // Extra side inset so the title, the × and the long lines stay inside
    // the bowed corners rather than sitting on the rim.
    let side = if full.width() < 1080.0 { 40.0 } else { 72.0 };
    let inner = full.shrink2(egui::vec2(side, 36.0));
    let footer = Rect::from_min_max(Pos2::new(inner.left(), inner.bottom() - 46.0), inner.right_bottom());
    let body = Rect::from_min_max(inner.min, Pos2::new(inner.right(), footer.top() - 8.0));
    ui.scope_builder(egui::UiBuilder::new().max_rect(body), |ui| {
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(4.0);
                if ui.add(egui::Button::new(RichText::new("×").color(AMBER).size(26.0)).frame(false)).clicked() {
                    out.close = true;
                }
                let w = ui.available_width();
                ui.allocate_ui_with_layout(egui::vec2(w, 36.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    let label = match *page {
                        0 => t("wiz_s1_title"),
                        1 => t("wiz_s2_title"),
                        _ => t("wiz_s3_title"),
                    };
                    title(ui, label);
                });
            });
        });
        ui.label(RichText::new(tf("wiz_step", &[&(*page as u32 + 1)])).color(AMBER_DIM).size(14.0));
        match *page {
            0 => page_welcome(ui),
            1 => page_rom(ui),
            _ => {
                if let Some(p) = page_layouts(ui, screen, zoom) {
                    out.preset = Some(p);
                }
            }
        }
    });
    ui.scope_builder(egui::UiBuilder::new().max_rect(footer), |ui| {
        ui.horizontal(|ui| {
            if checkbox(ui, hide) {
                out.dirty = true;
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let forward = if *page >= 2 { t("wiz_done") } else { t("wiz_next") };
                if amber_button(ui, forward).clicked() {
                    if *page >= 2 {
                        out.close = true;
                    } else {
                        *page += 1;
                    }
                }
                if *page > 0 {
                    ui.add_space(8.0);
                    if amber_button(ui, t("wiz_prev")).clicked() {
                        *page -= 1;
                    }
                }
            });
        });
    });
    out
}

fn page_welcome(ui: &mut egui::Ui) {
    heading(ui, t("wiz_s1_h"));
    ui.add_space(16.0);
    for key in ["wiz_s1_b1", "wiz_s1_b2", "wiz_s1_b3"] {
        ui.label(RichText::new(format!("•   {}", t(key))).color(AMBER).size(16.0));
        ui.add_space(16.0);
    }
}

fn page_rom(ui: &mut egui::Ui) {
    heading(ui, t("wiz_s2_h"));
    ui.add_space(12.0);
    for key in ["wiz_s2_p1", "wiz_s2_p2", "wiz_s2_p3"] {
        ui.label(RichText::new(t(key)).color(AMBER).size(16.0));
        ui.add_space(12.0);
    }
    ui.add_space(6.0);
    let wide = ui.available_width();
    let label = format!("{}  ↗", t("wiz_s2_link"));
    let galley = ui.painter().layout(label, egui::FontId::proportional(15.0), AMBER, wide - 28.0);
    let h = galley.size().y + 18.0;
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(wide, h), egui::Sense::click());
    let fill = if resp.hovered() { Color32::from_rgb(36, 22, 8) } else { Color32::TRANSPARENT };
    ui.painter().rect(rect, 6.0, fill, Stroke::new(1.0, AMBER_SOFT), egui::StrokeKind::Inside);
    ui.painter().galley(rect.min + egui::vec2(14.0, 9.0), galley, AMBER);
    if resp.clicked() {
        let _ = std::process::Command::new("xdg-open").arg(t("wiz_s2_url")).spawn();
    }
}

fn page_layouts(ui: &mut egui::Ui, screen: (u32, u32), zoom: f32) -> Option<Preset> {
    ui.add_space(10.0);
    ui.label(RichText::new(t("wl_intro")).color(AMBER).size(15.0));
    ui.add_space(12.0);
    let mut chosen = None;
    ui.horizontal_top(|ui| {
        // the outline is drawn 4 points outside each card: 12 points keep a gap between them (#54)
        ui.spacing_mut().item_spacing.x = 12.0;
        for pr in Preset::ALL {
            let card = ui.vertical(|ui| {
                ui.set_width(168.0);
                let (rect, _) = ui.allocate_exact_size(egui::vec2(168.0, 96.0), egui::Sense::hover());
                let p = ui.painter();
                p.rect_filled(rect, 3.0, Color32::from_rgb(16, 11, 6));
                for (pane, x0, y0, x1, y1) in pr.thumb() {
                    let (n, col) = crate::app::welcome_pane(pane);
                    let r = Rect::from_min_max(rect.lerp_inside(egui::vec2(x0, y0)), rect.lerp_inside(egui::vec2(x1, y1))).shrink(1.5);
                    p.rect_filled(r, 2.0, col);
                    let size = (r.width() * 0.8).min(r.height() * 0.5).clamp(8.0, 14.0);
                    p.text(r.center(), egui::Align2::CENTER_CENTER, n.to_string(), egui::FontId::proportional(size), Color32::from_rgb(255, 236, 210));
                }
                ui.add_space(4.0);
                ui.label(RichText::new(t(pr.title_key())).color(AMBER).strong().size(14.0));
                ui.label(RichText::new(t(pr.desc_key())).color(AMBER_SOFT).size(12.0));
                let ((w0, h0), (w1, h1)) = pr.resolution();
                let sc = |v: u32| (v as f32 * zoom).round() as u32;
                ui.label(RichText::new(tf("wl_min", &[&sc(w0), &sc(h0), &sc(w1), &sc(h1)])).color(AMBER_DIM).size(11.0));
                if screen.0 > 0 && screen.0 < sc(w0) {
                    ui.label(RichText::new(t("wl_too_small")).size(11.0).color(Color32::from_rgb(220, 150, 60)));
                }
            });
            let r = card.response.rect.expand(4.0);
            let resp = ui.interact(r, ui.id().with(("wiz-card", pr.title_key())), egui::Sense::click());
            let resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand);
            let stroke = if resp.hovered() { Stroke::new(1.5, AMBER) } else { Stroke::new(1.0, AMBER_DIM) };
            ui.painter().rect_stroke(r, 5.0, stroke, egui::StrokeKind::Inside);
            if resp.clicked() {
                chosen = Some(pr);
            }
        }
    });
    ui.add_space(10.0);
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 12.0;
        for pane in [Pane::Editor, Pane::Screen, Pane::Debugger, Pane::Disasm, Pane::Dump, Pane::Library, Pane::Color] {
            let (n, col) = crate::app::welcome_pane(pane);
            let (rect, _) = ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::hover());
            ui.painter().rect_filled(rect, 3.0, col);
            ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, n.to_string(), egui::FontId::proportional(11.0), Color32::from_rgb(255, 236, 210));
            ui.label(RichText::new(t(crate::app::welcome_pane_key(pane))).color(AMBER_SOFT).size(13.0));
        }
    });
    chosen
}
