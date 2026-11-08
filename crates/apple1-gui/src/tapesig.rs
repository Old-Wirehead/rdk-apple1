//! The cassette signal as heard and seen: the ACI works on edges (the
//! comparator output when reading, the flip-flop toggles when writing), so
//! the signal is rebuilt from the edge times as a square wave, softened by
//! a one-pole low-pass filter like the head and the cable would.

/// Edges inside a window, with the level just before the first one.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Window {
    pub edges: Vec<f64>,
    pub level0: bool,
}

/// Edges of `all` (sorted, in seconds) between `t0` and `t1`. The level is
/// low before the first edge of the tape, then toggles at each edge.
pub fn window(all: &[f64], t0: f64, t1: f64) -> Window {
    let a = all.partition_point(|&e| e <= t0);
    let b = all.partition_point(|&e| e <= t1);
    Window { edges: all[a..b].to_vec(), level0: a % 2 == 1 }
}

/// `n` samples from `t0` to `t1` (exclusive), values in -1..1. `lp` is the
/// filter state, kept between calls for a continuous sound; `alpha` its
/// coefficient (1 = no filter).
pub fn render(w: &Window, t0: f64, t1: f64, n: usize, alpha: f32, lp: &mut f32) -> Vec<f32> {
    let mut out = Vec::with_capacity(n);
    let (mut i, mut level) = (0, w.level0);
    let dt = if n > 0 { (t1 - t0) / n as f64 } else { 0.0 };
    for k in 0..n {
        let t = t0 + k as f64 * dt;
        while i < w.edges.len() && w.edges[i] <= t {
            level = !level;
            i += 1;
        }
        let x = if level { 1.0 } else { -1.0 };
        *lp += alpha * (x - *lp);
        out.push(*lp);
    }
    out
}

/// Filter coefficient for a cut-off frequency at a sample rate.
pub fn alpha(cutoff_hz: f32, rate: f32) -> f32 {
    let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff_hz);
    let dt = 1.0 / rate;
    dt / (rc + dt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_wave_from_edges() {
        // 1 kHz square wave: an edge every 0.5 ms
        let edges: Vec<f64> = (1..=100).map(|i| i as f64 * 0.0005).collect();
        let w = window(&edges, 0.01, 0.02);
        assert_eq!(w.edges.len(), 20);
        assert!(!w.level0, "19 edges before 10 ms: low... after 20 it is low");
        let s = render(&w, 0.01, 0.02, 200, 1.0, &mut 0.0);
        // 10 ms at 20 kHz: 200 samples, 10 periods, half of them high
        assert!((99..=101).contains(&s.iter().filter(|&&x| x > 0.0).count()));
        // the filtered sound stays within -1..1 and keeps the frequency
        let mut lp = 0.0;
        let s = render(&w, 0.01, 0.02, 200, alpha(4000.0, 20000.0), &mut lp);
        assert!(s.iter().all(|x| x.abs() <= 1.0));
        let crossings = s.windows(2).filter(|p| (p[0] < 0.0) != (p[1] < 0.0)).count();
        assert!((18..=21).contains(&crossings), "{crossings}");
        // before and after the tape: silence (constant level)
        let w = window(&edges, 1.0, 1.1);
        assert!(w.edges.is_empty());
    }
}
