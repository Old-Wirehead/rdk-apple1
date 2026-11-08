//! # apple1-core
//!
//! Emulation core of the Apple-1 (1976): NMOS 6502, PIA 6821, video
//! terminal, Woz Monitor, Apple Cassette Interface, and tools to convert
//! memory to/from ACI compatible WAV files. Frontend independent: the
//! terminal UI lives in the `apple1-emu` crate, a GUI could reuse this crate.

pub mod cpu;
pub mod debug;
pub mod display;
pub mod eprom;
pub mod gen2;
pub mod tms9918;
pub mod hexdump;
pub mod asm6502;
pub mod intbasic;
pub mod machine;
pub mod pia;
pub mod g2basic;
pub mod program;
pub mod roms;
pub mod state;
pub mod tape;
pub mod timing;
pub mod wav;
pub mod aciinput;
pub mod aciwav;
pub mod aiff;
pub mod asciiart;
pub mod xa;

pub use machine::{Config, Machine};
pub use timing::Timing;
