//! Apple-1 clock model.
//!
//! The Apple-1 CPU clock is derived from the 14.31818 MHz video crystal
//! divided by 14, i.e. 1.022727 MHz. The DRAM refresh is *not* transparent:
//! per the Apple-1 Operation Manual "four out of 65 cycles" (one TV scan line
//! = 455 dots / 7) are taken by refresh, during which the 6502 is stopped.
//! The average CPU speed is therefore 1.022727 * 61/65 = 0.9598 MHz.
//!
//! Because *all* ACI timing is done in software, the exact clock matters for
//! the frequencies written to tape. The refresh model can be disabled.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Timing {
    /// Master CPU clock in Hz (default 14.31818 MHz / 14).
    pub clock_hz: f64,
    /// Model the refresh stall (4 of every 65 clock periods).
    pub refresh_stall: bool,
}

pub const APPLE1_CLOCK_HZ: f64 = 14_318_180.0 / 14.0;

impl Default for Timing {
    fn default() -> Self {
        Timing { clock_hz: APPLE1_CLOCK_HZ, refresh_stall: true }
    }
}

impl Timing {
    /// Convert a CPU cycle count to elapsed real time (seconds).
    pub fn cycles_to_secs(&self, cycles: u64) -> f64 {
        // 61 usable CPU cycles per 65 clock periods
        self.cycles_to_ticks(cycles) as f64 / self.clock_hz
    }

    /// Convert CPU cycles to master CHAR clock periods (1.022727 MHz ticks),
    /// adding the refresh stalls.
    pub fn cycles_to_ticks(&self, cycles: u64) -> u64 {
        if self.refresh_stall {
            cycles + (cycles / 61) * 4
        } else {
            cycles
        }
    }

    /// Inverse of [`Self::cycles_to_ticks`] (rounded down).
    pub fn ticks_to_cycles(&self, ticks: u64) -> u64 {
        if self.refresh_stall {
            ticks / 65 * 61 + (ticks % 65).min(61)
        } else {
            ticks
        }
    }

    /// Average number of CPU cycles per second.
    pub fn effective_cps(&self) -> f64 {
        if self.refresh_stall {
            self.clock_hz * 61.0 / 65.0
        } else {
            self.clock_hz
        }
    }

    /// Convert seconds to an (approximate) CPU cycle count.
    pub fn secs_to_cycles(&self, secs: f64) -> u64 {
        (secs * self.effective_cps()).max(0.0) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn refresh_model() {
        let t = Timing::default();
        let one_sec = t.secs_to_cycles(1.0);
        let back = t.cycles_to_secs(one_sec);
        assert!((back - 1.0).abs() < 1e-4, "{back}");
        let t2 = Timing { refresh_stall: false, ..t };
        assert!((t2.cycles_to_secs(1_022_727) - 1.0).abs() < 1e-6);
    }
}
