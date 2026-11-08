//! A pane (editor, screen, debugger…) detached from the main window in its
//! own OS window (egui viewport) that can go to another monitor, then docked
//! again (crate::layout).

use crate::app::Hint;
use crate::i18n::t;
use eframe::egui;
use serde::{Deserialize, Serialize};

/// Where a tool is, saved in gui.json.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DockCfg {
    /// In its own window rather than docked.
    pub detached: bool,
    /// Width of the docked panel.
    pub width: f32,
    /// Position and inner size of the separate window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<[f32; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<[f32; 2]>,
}

impl Default for DockCfg {
    fn default() -> Self {
        DockCfg { detached: false, width: 640.0, pos: None, size: None }
    }
}

/// Egui points to the logical points of the window builder.
pub fn to_logical(v: [f32; 2], zoom: f32) -> [f32; 2] {
    [(v[0] * zoom).round(), (v[1] * zoom).round()]
}

/// What the user did with the frame of the tool.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct DockOut {
    /// Closed (× or the window's close button).
    pub close: bool,
    /// Something of DockCfg changed (to save).
    pub changed: bool,
}

/// Geometry the separate window was opened with: given unchanged to egui at
/// every frame (a changing builder would move the window).
#[derive(Default)]
pub struct DockState {
    opened_with: Option<([f32; 2], Option<[f32; 2]>)>,
}

impl DockState {
    /// Docked again: the next separate window takes the saved geometry.
    pub fn docked(&mut self) {
        self.opened_with = None;
    }
}

/// Title row of a separate window: title, ⊟ Ancrer, × (if it can close).
pub fn header(ui: &mut egui::Ui, title: &str, cfg: &mut DockCfg, h: bool, out: &mut DockOut, closable: bool, extra: &mut dyn FnMut(&mut egui::Ui)) {
    ui.horizontal(|ui| {
        ui.strong(title);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if closable && ui.button("×").hint(h, t("dk_close_tip"), "h_dk_close").clicked() {
                out.close = true;
            }
            if ui.button(t("dk_dock")).hint(h, t("dk_dock_tip"), "h_dk_dock").clicked() {
                cfg.detached = false;
                out.changed = true;
            }
            extra(ui);
        });
    });
    ui.separator();
}

/// The separate window (egui viewport) of a tool; `body` draws all of it.
/// Its geometry is kept in `cfg`; closing it sets `close`.
pub fn window(ctx: &egui::Context, id: &str, title: &str, cfg: &mut DockCfg, st: &mut DockState, mut body: impl FnMut(&mut egui::Ui, &mut DockCfg, &mut DockOut)) -> DockOut {
    let mut out = DockOut::default();
    // the saved geometry is in egui points (physical / (system scale x interface
    // zoom)), the window builder wants logical points (physical / system scale)
    // (#39: without the zoom factor the window shrank at each restart)
    let zoom = ctx.zoom_factor();
    let (size, pos) = *st.opened_with.get_or_insert_with(|| (to_logical(cfg.size.unwrap_or([cfg.width.max(560.0), 760.0]), zoom), cfg.pos.map(|p| to_logical(p, zoom))));
    let mut b = egui::ViewportBuilder::default().with_title(title).with_inner_size(size).with_min_inner_size([320.0, 240.0]).with_app_id("apple1-emu");
    if let Some(p) = pos {
        b = b.with_position(p);
    }
    let vid = egui::ViewportId::from_hash_of(id);
    ctx.show_viewport_immediate(vid, b, |ui, class| {
        if class == egui::ViewportClass::EmbeddedWindow {
            // no multi-window backend: shown as an egui window instead
            body(ui, cfg, &mut out);
            return;
        }
        egui::CentralPanel::default().show(ui, |ui| body(ui, cfg, &mut out));
        let (close, inner, outer) = ui.ctx().input(|i| (i.viewport().close_requested(), i.viewport().inner_rect, i.viewport().outer_rect));
        if close {
            out.close = true;
        }
        if let Some(r) = inner {
            let s = [r.width().round(), r.height().round()];
            if cfg.size != Some(s) && s[0] > 50.0 {
                cfg.size = Some(s);
                out.changed = true;
            }
        }
        if let Some(r) = outer {
            let p = [r.min.x.round(), r.min.y.round()];
            if cfg.pos != Some(p) {
                cfg.pos = Some(p);
                out.changed = true;
            }
        }
    });
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saved_geometry_is_stable_across_restarts_whatever_the_zoom() {
        for zoom in [0.85_f32, 1.0, 1.35, 1.8] {
            // physical pixels of the window, system scale 1: points = physical / zoom
            let physical = [1280.0_f32, 800.0];
            let mut saved = [(physical[0] / zoom).round(), (physical[1] / zoom).round()];
            for _ in 0..5 {
                let opened = to_logical(saved, zoom); // what the builder gets = physical size
                saved = [(opened[0] / zoom).round(), (opened[1] / zoom).round()];
            }
            assert!((saved[0] * zoom - physical[0]).abs() <= zoom, "zoom {zoom}: {saved:?}");
            assert!((saved[1] * zoom - physical[1]).abs() <= zoom, "zoom {zoom}: {saved:?}");
        }
    }
}
