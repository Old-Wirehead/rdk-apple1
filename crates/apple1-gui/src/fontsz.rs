//! Compact « A 13 ▲▼ » text-size control, shared by every tool window.
//! The size is remembered in gui.json and cleared by « Reset preferences ».

use eframe::egui::{self, Color32};

use crate::app::Hint;
use crate::editor::{FONT_MAX, FONT_MIN};
use crate::i18n::t;

pub const DEFAULT: f32 = 13.0;

pub fn default_size() -> f32 {
    DEFAULT
}

pub fn step(font: f32, d: f32) -> f32 {
    (font + d).clamp(FONT_MIN, FONT_MAX).round()
}

pub fn reset() -> f32 {
    DEFAULT
}

/// Ctrl+wheel over `rect` scales `font`. True when it changed.
pub fn wheel(ui: &egui::Ui, rect: egui::Rect, font: &mut f32) -> bool {
    if !ui.rect_contains_pointer(rect) {
        return false;
    }
    let z = ui.input(|i| i.zoom_delta());
    if (z - 1.0).abs() <= 1e-3 {
        return false;
    }
    let next = (*font * z).clamp(FONT_MIN, FONT_MAX);
    if (next - *font).abs() < 0.05 {
        return false;
    }
    *font = next;
    true
}

/// « A 13 » with two stacked triangles. True when the size changed.
pub fn control(ui: &mut egui::Ui, salt: &str, font: &mut f32, h: bool) -> bool {
    let height = ui.spacing().interact_size.y;
    let label = format!("A {}", *font as i32);
    let text_w = ui.painter().layout_no_wrap(label.clone(), egui::FontId::proportional(height * 0.62), Color32::PLACEHOLDER).size().x;
    let arrows_w = height * 0.72;
    let (rect, _) = ui.allocate_exact_size(egui::vec2(text_w + arrows_w + 10.0, height), egui::Sense::hover());
    let arrows = egui::Rect::from_min_max(egui::pos2(rect.right() - arrows_w, rect.top()), rect.max);
    let up_r = egui::Rect::from_min_max(arrows.min, egui::pos2(arrows.right(), rect.center().y));
    let down_r = egui::Rect::from_min_max(egui::pos2(arrows.left(), rect.center().y), arrows.max);
    let up = ui.interact(up_r, ui.id().with((salt, "a-plus")), egui::Sense::click()).hint(h, t("fz_tip"), "h_fz");
    let down = ui.interact(down_r, ui.id().with((salt, "a-minus")), egui::Sense::click()).hint(h, t("fz_tip"), "h_fz");
    let any = up.hovered() || down.hovered();
    let v = if any { ui.visuals().widgets.hovered } else { ui.visuals().widgets.inactive };
    let p = ui.painter();
    p.rect(rect, v.corner_radius, v.weak_bg_fill, v.bg_stroke, egui::StrokeKind::Inside);
    p.text(egui::pos2(rect.left() + 4.0 + text_w / 2.0, rect.center().y), egui::Align2::CENTER_CENTER, label, egui::FontId::proportional(height * 0.62), v.fg_stroke.color);
    for (r, r_resp, upward) in [(up_r, &up, true), (down_r, &down, false)] {
        if r_resp.hovered() {
            p.rect_filled(r.shrink(1.0), 2.0, ui.visuals().widgets.active.weak_bg_fill);
        }
        let (c, s) = (r.center(), height * 0.16);
        let tri = if upward {
            [egui::pos2(c.x, c.y - s), egui::pos2(c.x - s * 1.2, c.y + s * 0.8), egui::pos2(c.x + s * 1.2, c.y + s * 0.8)]
        } else {
            [egui::pos2(c.x, c.y + s), egui::pos2(c.x - s * 1.2, c.y - s * 0.8), egui::pos2(c.x + s * 1.2, c.y - s * 0.8)]
        };
        p.add(egui::Shape::convex_polygon(tri.to_vec(), v.fg_stroke.color, egui::Stroke::NONE));
    }
    let mut changed = false;
    if up.clicked() {
        *font = step(*font, 1.0);
        changed = true;
    }
    if down.clicked() {
        *font = step(*font, -1.0);
        changed = true;
    }
    changed
}

/// Which tool window last had the pointer or the keyboard (Ctrl+plus / Ctrl+minus).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win {
    Editor,
    Listing,
    Debugger,
    Disasm,
    Dump,
    Library,
}
