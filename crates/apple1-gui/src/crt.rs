//! « Écran vintage (screening) »: imitation of a 1970s CRT monitor.
//!
//! The main effect, as on Wirehead's Apple-1 monitor, is the thin dark
//! gap between raster lines: each row of character dots is a horizontal
//! stripe. It is drawn as thin bands of a fixed size in *physical* pixels,
//! so it stays thin at every zoom. The rest is optional and mild: slight
//! horizontal softening of the dots, phosphor afterglow (time constant of
//! the chosen phosphor), a low glow, barrel curvature + vignette, flicker.
//!
//! Everything cheap runs on the CPU at the Apple-1 resolution (280x192, or
//! 4x wider for the softening), the scaling and the geometry are plain
//! textured meshes: it works the same with the software OpenGL renderer
//! (llvmpipe) at any zoom, with no shader to compile.

use eframe::egui::{self, Color32, ColorImage, Mesh, Pos2, Rect};
use serde::{Deserialize, Serialize};

use crate::screen::{Phosphor, HEIGHT, WIDTH};

/// Horizontal oversampling of the processed picture (for the softening).
pub const SX: usize = 4;

/// Version of the screening settings: older gui.json files are migrated
/// to the new defaults (see `Screening::migrate`).
pub const SCREENING_VERSION: u32 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Screening {
    pub on: bool,
    /// 0..1: depth of the line gaps and strength of the softening and glow.
    pub intensity: f32,
    /// Thin dark separation between raster lines (the main effect).
    pub gaps: bool,
    /// Very slight horizontal softening of the dots.
    pub soften: bool,
    /// Low glow around the characters.
    pub glow: bool,
    /// Phosphor afterglow, 0 (none) .. 1 (long); the decay time also
    /// depends on the chosen phosphor.
    pub afterglow: f32,
    /// Barrel curvature of the tube and darker corners.
    pub curvature: bool,
    /// Amount of curvature, 0..1 (when `curvature` is on).
    pub curve: f32,
    /// Slight flicker of the brightness.
    pub flicker: bool,
    /// GEN2 colour screen: fine vertical red/green/blue phosphor stripes
    /// (a colour TV shadow mask), one stripe per physical pixel. Its
    /// strength follows `intensity`.
    pub rgb_mask: bool,
    /// 0 when missing: settings written before the version 2 (v5.3).
    #[serde(default)]
    pub version: u32,
    /// Afterglow on/off of the v5.3 (read only, for the migration).
    #[serde(default, skip_serializing)]
    pub persistence: Option<bool>,
}

impl Default for Screening {
    fn default() -> Self {
        Screening {
            on: true,
            intensity: 0.6,
            gaps: true,
            soften: true,
            glow: true,
            afterglow: 0.3,
            curvature: true,
            curve: 0.07,
            flicker: false,
            rgb_mask: true,
            version: SCREENING_VERSION,
            persistence: None,
        }
    }
}

impl Screening {
    /// Bring settings of an older version to the new defaults: the halo is
    /// now on and the afterglow shorter (the v5.3 saved every field, so its
    /// "glow off" cannot be told from a choice). An afterglow switched off
    /// by the user stays off.
    pub fn migrate(&mut self) {
        if self.version < 2 {
            let d = Screening::default();
            self.glow = d.glow;
            self.afterglow = if self.persistence == Some(false) { 0.0 } else { d.afterglow };
            self.curve = d.curve;
        }
        self.persistence = None;
        self.version = SCREENING_VERSION;
        self.intensity = self.intensity.clamp(0.0, 1.0);
        self.afterglow = self.afterglow.clamp(0.0, 1.0);
        self.curve = self.curve.clamp(0.0, 1.0);
    }

    /// Barrel distortion factor used by `paint` (0 = flat).
    pub fn curvature_amount(&self) -> f32 {
        if self.curvature {
            0.01 + 0.11 * self.curve.clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}

/// Afterglow time constant (s) at full strength: P4 white short, P1 green
/// medium, P3 amber long. The `afterglow` setting scales it (0.3 by default).
pub fn tau(ph: Option<Phosphor>) -> f32 {
    match ph {
        Some(Phosphor::Vert) => 0.06,
        Some(Phosphor::Blanc) => 0.03,
        Some(Phosphor::Ambre) => 0.15,
        None => 0.03, // colour monitor of the GEN2 card
    }
}

fn to_lin(c: u8) -> f32 {
    let v = c as f32 / 255.0;
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

struct Luts {
    lin: [f32; 256],
    srgb: Vec<u8>,
}

const SRGB_STEPS: usize = 8192;

fn luts() -> &'static Luts {
    static L: std::sync::OnceLock<Luts> = std::sync::OnceLock::new();
    L.get_or_init(|| {
        let mut lin = [0.0; 256];
        for (i, v) in lin.iter_mut().enumerate() {
            *v = to_lin(i as u8);
        }
        let srgb = (0..SRGB_STEPS)
            .map(|i| {
                let v = i as f32 / (SRGB_STEPS - 1) as f32;
                let s = if v <= 0.003_130_8 { v * 12.92 } else { 1.055 * v.powf(1.0 / 2.4) - 0.055 };
                (s * 255.0 + 0.5).clamp(0.0, 255.0) as u8
            })
            .collect();
        Luts { lin, srgb }
    })
}

fn enc(v: f32) -> u8 {
    let l = luts();
    l.srgb[(v.clamp(0.0, 1.0) * (SRGB_STEPS - 1) as f32 + 0.5) as usize]
}

/// Picture processor of one screen (keeps the afterglow between frames).
#[derive(Default)]
pub struct Crt {
    acc: Vec<[f32; 3]>,
    pub active: bool,
    /// Last picture handed out by `process` (kept when asked, for `bake`)
    /// and a counter that changes with it.
    pub last: Option<ColorImage>,
    pub gen: u64,
}

impl Crt {
    /// Process a 280x192 picture whose background colour is `bg`. Returns a
    /// (280*SX)x192 picture (SX x wider when softening, else 280x192).
    /// `dt` = seconds since the previous call. `active` tells whether the
    /// afterglow is still fading (call again next frame).
    pub fn process(&mut self, src: &ColorImage, bg: Color32, s: &Screening, tau: f32, dt: f32) -> ColorImage {
        let l = luts();
        let n = WIDTH * HEIGHT;
        let bgl = [l.lin[bg.r() as usize], l.lin[bg.g() as usize], l.lin[bg.b() as usize]];
        if self.acc.len() != n {
            self.acc = vec![[0.0; 3]; n];
        }
        let k = s.intensity.clamp(0.0, 1.0);
        let ag = s.afterglow.clamp(0.0, 1.0);
        let decay = if ag > 0.0 { (-dt.max(0.0) / (tau * ag)).exp() } else { 0.0 };
        let mut active = false;
        for (a, p) in self.acc.iter_mut().zip(&src.pixels) {
            let sig = [
                (l.lin[p.r() as usize] - bgl[0]).max(0.0),
                (l.lin[p.g() as usize] - bgl[1]).max(0.0),
                (l.lin[p.b() as usize] - bgl[2]).max(0.0),
            ];
            for c in 0..3 {
                let faded = a[c] * decay;
                if faded > sig[c] + 0.002 {
                    a[c] = faded;
                    active = true;
                } else {
                    a[c] = sig[c];
                }
            }
        }
        self.active = active;
        let mut v = self.acc.clone();
        if s.glow && k > 0.0 {
            let g = blur(&self.acc, WIDTH, HEIGHT, 1.3);
            let amt = 0.05 * k;
            for (o, g) in v.iter_mut().zip(&g) {
                for c in 0..3 {
                    o[c] += amt * g[c];
                }
            }
        }
        let sx = if s.soften { SX } else { 1 };
        let w = WIDTH * sx;
        let bgc = bg;
        let mut out = vec![bgc; w * HEIGHT];
        let px = |c: [f32; 3]| -> Color32 {
            let p = [bgl[0] + c[0], bgl[1] + c[1], bgl[2] + c[2]];
            // very bright dots tend to white (saturated phosphor)
            let over = (p[0].max(p[1]).max(p[2]) - 1.0).max(0.0) * 0.5;
            Color32::from_rgb(enc(p[0] + over), enc(p[1] + over), enc(p[2] + over))
        };
        // horizontal reconstruction: each dot is SX sub-pixels; the outer
        // sub-pixels take a little light from the neighbour dot (a tent of
        // a quarter of a dot), the inner ones keep the dot as it is
        let wt = 0.18 * (0.5 + k);
        let mix = |a: [f32; 3], b: [f32; 3]| [(a[0] + wt * b[0]) / (1.0 + wt), (a[1] + wt * b[1]) / (1.0 + wt), (a[2] + wt * b[2]) / (1.0 + wt)];
        let zero = [0.0f32; 3];
        for y in 0..HEIGHT {
            let row = &v[y * WIDTH..(y + 1) * WIDTH];
            let orow = &mut out[y * w..(y + 1) * w];
            for x in 0..WIDTH {
                let c = row[x];
                let l = if x > 0 { row[x - 1] } else { zero };
                let r = if x + 1 < WIDTH { row[x + 1] } else { zero };
                let lit = |p: [f32; 3]| p[0] + p[1] + p[2] > 0.0;
                if sx == 1 {
                    if lit(c) {
                        orow[x] = px(c);
                    }
                    continue;
                }
                if !(lit(c) || lit(l) || lit(r)) {
                    continue;
                }
                let o = &mut orow[x * sx..(x + 1) * sx];
                let mid = if lit(c) { px(c) } else { bgc };
                for m in o[1..sx - 1].iter_mut() {
                    *m = mid;
                }
                o[0] = px(mix(c, l));
                o[sx - 1] = px(mix(c, r));
            }
        }
        ColorImage::new([w, HEIGHT], out)
    }
}

/// Separable gaussian blur (sigma in dots), skipping the dark pixels.
fn blur(src: &[[f32; 3]], w: usize, h: usize, sigma: f32) -> Vec<[f32; 3]> {
    let r = (sigma * 2.5).ceil() as isize;
    let ker: Vec<f32> = (-r..=r).map(|i| (-(i * i) as f32 / (2.0 * sigma * sigma)).exp()).collect();
    let sum: f32 = ker.iter().sum();
    let pass = |inp: &[[f32; 3]], horiz: bool| {
        let mut out = vec![[0.0f32; 3]; w * h];
        for y in 0..h {
            for x in 0..w {
                let p = inp[y * w + x];
                if p[0] + p[1] + p[2] <= 0.0 {
                    continue;
                }
                for (j, kw) in ker.iter().enumerate() {
                    let d = j as isize - r;
                    let (xx, yy) = if horiz { (x as isize + d, y as isize) } else { (x as isize, y as isize + d) };
                    if xx < 0 || yy < 0 || xx >= w as isize || yy >= h as isize {
                        continue;
                    }
                    let o = &mut out[yy as usize * w + xx as usize];
                    let f = kw / sum;
                    for i in 0..3 {
                        o[i] += f * p[i];
                    }
                }
            }
        }
        out
    };
    pass(&pass(src, true), false)
}

/// Curved tube: a point in -1..1 coordinates is pulled towards the centre
/// the more it is far from it, so the edges of the picture look convex.
/// The middle of each edge stays in place, the corners come in.
pub fn warp(u: f32, v: f32, k: f32) -> (f32, f32) {
    let f = (1.0 - k * (u * u + v * v)) / (1.0 - k);
    (u * f, v * f)
}

/// Thickness (physical pixels) and darkness (0..255) of the gap between
/// two raster lines `line_px` physical pixels high; None = too small to
/// show a gap without hiding the picture.
pub fn gap(line_px: f32, intensity: f32) -> Option<(f32, u8)> {
    if line_px < 2.0 {
        return None;
    }
    let k = intensity.clamp(0.0, 1.0);
    // about a fifth of the line, at least one pixel
    let t = (line_px / 5.0).round().max(1.0);
    // a 1 px gap on a 2-3 px line is lightened, so the picture stays bright
    let frac = t / line_px;
    let dark = (0.45 + 0.5 * k) * if frac > 0.25 { 0.25 / frac } else { 1.0 };
    Some((t, (dark * 255.0).round().clamp(0.0, 255.0) as u8))
}

/// Grey level (255 = off) the picture is multiplied with: the flicker.
pub fn tint_gray(s: &Screening, flicker_phase: f64) -> u8 {
    if s.flicker {
        let k = s.intensity.clamp(0.0, 1.0);
        // 60 Hz beat with the room lighting + a little noise
        let n = ((flicker_phase * 7.3).sin() * 0.5 + (flicker_phase * 59.0).sin() * 0.5) as f32;
        (255.0 - (6.0 + 10.0 * k) * (0.5 + 0.5 * n)).round() as u8
    } else {
        255
    }
}

/// Draw a processed screen into `rect` (physical pixel size `ppp`).
///
/// `baked`: the picture already made at the physical resolution of `rect`
/// by `bake` (curvature, raster gaps and phosphor stripes included): it is
/// drawn as is, one texel per physical pixel.
pub fn paint(p: &egui::Painter, tex: egui::TextureId, baked: Option<egui::TextureId>, rect: Rect, s: &Screening, ppp: f32, flicker_phase: f64) {
    let k = s.intensity.clamp(0.0, 1.0);
    let curv = s.curvature_amount();
    let tint = Color32::from_gray(tint_gray(s, flicker_phase));
    let map = |u: f32, v: f32| -> Pos2 {
        let (a, b) = if curv > 0.0 { warp(u * 2.0 - 1.0, v * 2.0 - 1.0, curv) } else { (u * 2.0 - 1.0, v * 2.0 - 1.0) };
        egui::pos2(rect.center().x + a * rect.width() / 2.0, rect.center().y + b * rect.height() / 2.0)
    };
    // picture: a grid (curved) or one quad
    let n = if curv > 0.0 { 24 } else { 1 };
    if let Some(b) = baked {
        p.image(b, rect, Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), Color32::WHITE);
        vignette(p, rect, s, &map);
        return;
    }
    let mut m = Mesh::with_texture(tex);
    for j in 0..=n {
        for i in 0..=n {
            let (u, v) = (i as f32 / n as f32, j as f32 / n as f32);
            m.vertices.push(egui::epaint::Vertex { pos: map(u, v), uv: egui::pos2(u, v), color: tint });
        }
    }
    let idx = |i: usize, j: usize| (j * (n + 1) + i) as u32;
    for j in 0..n {
        for i in 0..n {
            m.add_triangle(idx(i, j), idx(i + 1, j), idx(i + 1, j + 1));
            m.add_triangle(idx(i, j), idx(i + 1, j + 1), idx(i, j + 1));
        }
    }
    p.add(m);
    // gaps between raster lines, at the bottom of each line
    if s.gaps {
        let line_px = rect.height() / HEIGHT as f32 * ppp;
        if let Some((t, dark)) = gap(line_px, k) {
            let col = Color32::from_black_alpha(dark);
            let th = t / ppp;
            let mut g = Mesh::default();
            let segs = if curv > 0.0 { 24 } else { 1 };
            for y in 1..=HEIGHT {
                let v1 = y as f32 / HEIGHT as f32;
                if curv == 0.0 {
                    // snapped to physical pixels: crisp at every zoom
                    let yb = ((rect.min.y + v1 * rect.height()) * ppp).round() / ppp;
                    g.add_colored_rect(Rect::from_min_max(egui::pos2(rect.min.x, yb - th), egui::pos2(rect.max.x, yb)), col);
                } else {
                    let v0 = v1 - th / rect.height();
                    let base = g.vertices.len() as u32;
                    for i in 0..=segs {
                        let u = i as f32 / segs as f32;
                        g.colored_vertex(map(u, v0), col);
                        g.colored_vertex(map(u, v1), col);
                    }
                    for i in 0..segs as u32 {
                        let a = base + 2 * i;
                        g.add_triangle(a, a + 1, a + 3);
                        g.add_triangle(a, a + 3, a + 2);
                    }
                }
            }
            p.add(g);
        }
    }
    vignette(p, rect, s, &map);
}

fn vignette(p: &egui::Painter, _rect: Rect, s: &Screening, map: &dyn Fn(f32, f32) -> Pos2) {
    let curv = s.curvature_amount();
    // darker corners
    if s.curvature {
        let n = 16;
        let mut vg = Mesh::default();
        for j in 0..=n {
            for i in 0..=n {
                let (u, v) = (i as f32 / n as f32, j as f32 / n as f32);
                let (a, b) = (u * 2.0 - 1.0, v * 2.0 - 1.0);
                let r = (a * a * 0.6 + b * b * 0.6).min(1.2);
                let alpha = ((r * r) * (30.0 + 1000.0 * curv)).clamp(0.0, 200.0) as u8;
                vg.colored_vertex(map(u, v), Color32::from_black_alpha(alpha));
            }
        }
        for j in 0..n {
            for i in 0..n {
                vg.add_triangle(idx2(i, j, n), idx2(i + 1, j, n), idx2(i + 1, j + 1, n));
                vg.add_triangle(idx2(i, j, n), idx2(i + 1, j + 1, n), idx2(i, j + 1, n));
            }
        }
        p.add(vg);
    }
}

/// Inverse of `warp`: the point of the flat picture that is drawn at (a, b).
pub fn unwarp(a: f32, b: f32, k: f32) -> (f32, f32) {
    let (mut u, mut v) = (a, b);
    for _ in 0..8 {
        let f = (1.0 - k * (u * u + v * v)) / (1.0 - k);
        u = a / f;
        v = b / f;
    }
    (u, v)
}

/// Weights of the three stripes of the phosphor mask: `[stripe][channel]`.
/// The stripe keeps its own colour and lets the two others through at
/// `lo`; the whole is lifted so a neutral dot keeps its brightness.
pub fn mask_weights(intensity: f32) -> [[f32; 3]; 3] {
    let lo = 1.0 - 0.7 * intensity.clamp(0.0, 1.0);
    let gain = 3.0 / (1.0 + 2.0 * lo);
    let mut w = [[lo * gain; 3]; 3];
    for (i, row) in w.iter_mut().enumerate() {
        row[i] = gain;
    }
    w
}

/// The colour screen as it is seen on the tube, made at the physical
/// resolution (`w` x `h` pixels) it will be shown at, 1 texel = 1 pixel:
/// curvature, raster gaps and the red/green/blue phosphor stripes (one per
/// pixel, so nothing is ever resampled: no moiré, no grey bands).
/// `src` is the picture of `Crt::process`. Pixels outside the curved tube
/// are transparent. `tint`: 0..1 brightness (flicker).
pub fn bake(src: &ColorImage, w: usize, h: usize, s: &Screening, tint: f32) -> ColorImage {
    let (sw, k) = (src.size[0], s.intensity.clamp(0.0, 1.0));
    let curv = s.curvature_amount();
    let line_px = h as f32 / HEIGHT as f32;
    let gaps = if s.gaps { gap(line_px, k) } else { None };
    let weights = mask_weights(k);
    let mut out = vec![Color32::TRANSPARENT; w * h];
    for y in 0..h {
        let b = (y as f32 + 0.5) / h as f32 * 2.0 - 1.0;
        for x in 0..w {
            let a = (x as f32 + 0.5) / w as f32 * 2.0 - 1.0;
            let (u, v) = if curv > 0.0 { unwarp(a, b, curv) } else { (a, b) };
            if u.abs() >= 1.0 || v.abs() >= 1.0 {
                continue;
            }
            let line = (((v * 0.5 + 0.5) * HEIGHT as f32) as usize).min(HEIGHT - 1);
            let col = (((u * 0.5 + 0.5) * sw as f32) as usize).min(sw - 1);
            let p = src.pixels[line * sw + col];
            let mut f = tint;
            if let Some((t, dark)) = gaps {
                let in_gap = if curv == 0.0 {
                    let yb = ((line + 1) as f32 * h as f32 / HEIGHT as f32).round() as usize;
                    y < yb && y + t as usize >= yb
                } else {
                    (v * 0.5 + 0.5) * HEIGHT as f32 - line as f32 > 1.0 - t / line_px
                };
                if in_gap {
                    f *= 1.0 - dark as f32 / 255.0;
                }
            }
            let wt = if s.rgb_mask { weights[x % 3] } else { [1.0; 3] };
            let c = |v: u8, w: f32| (v as f32 * w * f).round().clamp(0.0, 255.0) as u8;
            out[y * w + x] = Color32::from_rgb(c(p.r(), wt[0]), c(p.g(), wt[1]), c(p.b(), wt[2]));
        }
    }
    ColorImage::new([w, h], out)
}

fn idx2(i: usize, j: usize, n: usize) -> u32 {
    (j * (n + 1) + i) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn picture(ph: Phosphor) -> ColorImage {
        let mut cells = [[b' '; 40]; 24];
        for (i, c) in b"0000".iter().enumerate() {
            cells[0][i] = *c;
        }
        crate::screen::render(&cells, None, ph)
    }

    #[test]
    fn gaps_stay_thin() {
        assert_eq!(gap(1.0, 0.6), None);
        for px in [2.0, 3.0, 4.0, 8.0, 16.0, 24.0, 40.0] {
            let (t, d) = gap(px, 0.6).unwrap();
            assert!(t >= 1.0 && t <= (px / 4.0).max(1.0), "{px}: {t}");
            assert!(d > 0);
        }
        // deeper with the intensity
        assert!(gap(12.0, 1.0).unwrap().1 > gap(12.0, 0.0).unwrap().1);
    }

    #[test]
    fn process_keeps_the_picture() {
        let src = picture(Phosphor::Vert);
        let s = Screening { on: true, ..Default::default() };
        let mut crt = Crt::default();
        let out = crt.process(&src, Phosphor::Vert.bg(), &s, tau(Some(Phosphor::Vert)), 0.016);
        assert_eq!(out.size, [WIDTH * SX, HEIGHT]);
        // background unchanged, lit dots stay bright, a bit of light beside them
        assert_eq!(out.pixels[100 * WIDTH * SX + 500], Phosphor::Vert.bg());
        let lit = (0..WIDTH).find(|&x| src.pixels[2 * WIDTH + x] != Phosphor::Vert.bg()).unwrap();
        let o = out.pixels[2 * WIDTH * SX + lit * SX + 1];
        assert!(o.g() >= 0xF0, "{o:?}");
        // without softening: exactly the source
        let crisp = Screening { soften: false, glow: false, afterglow: 0.0, ..s };
        let out = Crt::default().process(&src, Phosphor::Vert.bg(), &crisp, 0.05, 0.016);
        let diff = out.pixels.iter().zip(&src.pixels).filter(|(a, b)| a.g().abs_diff(b.g()) > 2).count();
        assert_eq!(diff, 0);
    }

    #[test]
    fn afterglow_fades() {
        let s = Screening { on: true, ..Default::default() };
        let mut crt = Crt::default();
        let lit = picture(Phosphor::Ambre);
        let dark = crate::screen::render(&[[b' '; 40]; 24], None, Phosphor::Ambre);
        crt.process(&lit, Phosphor::Ambre.bg(), &s, tau(Some(Phosphor::Ambre)), 0.016);
        assert!(!crt.active);
        let a = crt.process(&dark, Phosphor::Ambre.bg(), &s, tau(Some(Phosphor::Ambre)), 0.016);
        assert!(crt.active, "the characters glow after being erased");
        assert!(a.pixels.iter().any(|p| p.r() > 0x80));
        for _ in 0..200 {
            crt.process(&dark, Phosphor::Ambre.bg(), &s, tau(Some(Phosphor::Ambre)), 0.016);
        }
        assert!(!crt.active);
    }

    /// Frames (16 ms) until an erased character is gone.
    fn fade_frames(s: &Screening, ph: Phosphor) -> usize {
        let mut crt = Crt::default();
        let dark = crate::screen::render(&[[b' '; 40]; 24], None, ph);
        crt.process(&picture(ph), ph.bg(), s, tau(Some(ph)), 0.016);
        (1..1000).find(|_| {
            crt.process(&dark, ph.bg(), s, tau(Some(ph)), 0.016);
            !crt.active
        })
        .unwrap()
    }

    #[test]
    fn afterglow_strength() {
        let d = Screening { on: true, ..Default::default() };
        // short by default: fully gone (under 0.2 % of the light) in less
        // than 0.35 s even in amber, shorter in green and white
        let (w, g, a) = (fade_frames(&d, Phosphor::Blanc), fade_frames(&d, Phosphor::Vert), fade_frames(&d, Phosphor::Ambre));
        assert!(w <= g && g < a && a * 16 < 350, "{w} {g} {a}");
        // 0 = none, the maximum lasts longer
        let off = Screening { afterglow: 0.0, ..d };
        assert_eq!(fade_frames(&off, Phosphor::Ambre), 1);
        let max = Screening { afterglow: 1.0, ..d };
        assert!(fade_frames(&max, Phosphor::Ambre) > 2 * a);
    }

    #[test]
    fn curvature_amount() {
        let mut s = Screening::default();
        // first launch: a slight curvature, slider at 0.07
        assert!(s.curvature && (s.curve - 0.07).abs() < 1e-6);
        s.curvature = false;
        assert_eq!(s.curvature_amount(), 0.0);
        s.curvature = true;
        let mild = s.curvature_amount();
        assert!(mild > 0.0 && mild < 0.06, "{mild}");
        s.curve = 1.0;
        assert!(s.curvature_amount() > 2.0 * mild);
        s.curve = 0.0;
        assert!(s.curvature_amount() > 0.0);
    }

    #[test]
    fn fast_enough() {
        // worst case: the 960 characters on screen
        let mut cells = [[b'@'; 40]; 24];
        cells[3][3] = b'#';
        let src = crate::screen::render(&cells, None, Phosphor::Vert);
        let s = Screening { on: true, glow: true, ..Default::default() };
        let mut crt = Crt::default();
        let t0 = std::time::Instant::now();
        for _ in 0..20 {
            crt.process(&src, Phosphor::Vert.bg(), &s, 0.05, 0.016);
        }
        let per = t0.elapsed().as_secs_f64() / 20.0;
        eprintln!("screening : {:.2} ms par image", per * 1000.0);
        // generous bound (debug builds are much slower than release)
        assert!(per < if cfg!(debug_assertions) { 0.5 } else { 0.02 }, "{per}");
    }

    #[test]
    fn warp_keeps_centre_and_corners_inside() {
        assert_eq!(warp(0.0, 0.0, 0.1), (0.0, 0.0));
        let (e, f) = warp(1.0, 0.0, 0.1);
        assert!((e - 1.0).abs() < 1e-5 && f == 0.0);
        let (a, b) = warp(1.0, 1.0, 0.1);
        assert!(a < 1.0 && a > 0.8 && a == b, "corners come in: {a}");
    }

    #[test]
    fn unwarp_inverts_warp() {
        for k in [0.01, 0.08, 0.12] {
            for (u, v) in [(0.0, 0.0), (0.5, -0.3), (0.9, 0.9), (-1.0, 0.2)] {
                let (a, b) = warp(u, v, k);
                let (u2, v2) = unwarp(a, b, k);
                assert!((u - u2).abs() < 1e-3 && (v - v2).abs() < 1e-3, "{k} {u},{v} -> {u2},{v2}");
            }
        }
    }

    #[test]
    fn phosphor_stripes_are_one_pixel_wide_and_black_stays_black() {
        let mut img = ColorImage::new([WIDTH, HEIGHT], vec![Color32::BLACK; WIDTH * HEIGHT]);
        // a white dot block in the middle
        for y in 90..100 {
            for x in 100..120 {
                img.pixels[y * WIDTH + x] = Color32::WHITE;
            }
        }
        let s = Screening { on: true, curvature: false, gaps: false, rgb_mask: true, ..Screening::default() };
        let out = bake(&img, WIDTH * 3, HEIGHT * 3, &s, 1.0);
        let at = |x: usize, y: usize| out.pixels[y * out.size[0] + x];
        // inside the white block: R, G, B stripes cycle with the pixel column
        let (r, g, b) = (at(300, 280), at(301, 280), at(302, 280));
        assert!(r.r() > r.g() && r.r() > r.b(), "{r:?}");
        assert!(g.g() > g.r() && g.g() > g.b(), "{g:?}");
        assert!(b.b() > b.r() && b.b() > b.g(), "{b:?}");
        assert_eq!(at(303, 280), r);
        // the average is close to the white of the dot (no dimming, no grey bands)
        let mean = |f: fn(Color32) -> u8| (f(r) as u32 + f(g) as u32 + f(b) as u32) / 3;
        assert!(mean(|c| c.r()) > 200 && mean(|c| c.g()) > 200 && mean(|c| c.b()) > 200);
        // the black background has no stripes at all
        assert!((0..30).all(|x| at(x, 20) == Color32::BLACK));
        // without the mask the white is plain white
        let plain = bake(&img, WIDTH * 3, HEIGHT * 3, &Screening { rgb_mask: false, ..s }, 1.0);
        assert_eq!(plain.pixels[280 * plain.size[0] + 300], Color32::WHITE);
    }

    #[test]
    fn baked_curved_picture_has_transparent_corners_and_no_panic() {
        let img = ColorImage::new([WIDTH * SX, HEIGHT], vec![Color32::from_gray(200); WIDTH * SX * HEIGHT]);
        let s = Screening { on: true, curvature: true, curve: 0.64, gaps: true, ..Screening::default() };
        let out = bake(&img, 840, 576, &s, 1.0);
        assert_eq!(out.pixels[0], Color32::TRANSPARENT);
        assert_ne!(out.pixels[288 * 840 + 420], Color32::TRANSPARENT);
    }
}
