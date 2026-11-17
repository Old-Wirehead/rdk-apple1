//! Code repris **tel quel** de la v1.0 (`crates/apple1-gui`, intouché) pour
//! les fonctions que la 2.0 n'avait pas encore : son de la cassette (ALSA),
//! signal de la cassette (oscilloscope), réglages par programme
//! (`programs.json`), projets GitHub et « Tester le dernier build »,
//! construction des sources (ca65 / ld65 ou assembleur intégré).
//!
//! Les modules sont **lus dans les sources de la v1.0** par `build.rs`
//! (sans copie dans le dépôt, comme `gen2screen.rs` et la table des
//! cassettes) ; seules les références à d'autres modules de la v1.0 passent
//! par les petites fonctions de ce module ([`library`], [`config`],
//! [`known`], [`ascii`]).
//! Leurs textes viennent de la table de la v1.0 (`i18n/strings.txt`, 7
//! langues), lue par [`crate::i18n`] dans la langue de l'interface commune.

#![allow(clippy::all)]

pub mod audio {
    //! Son de la cassette par ALSA (v1.0 `audio.rs`).
    include!(concat!(env!("OUT_DIR"), "/v1_audio.rs"));
}
pub mod build {
    //! Construction d'un source (v1.0 `build.rs`) : ca65 / ld65 ou assembleur intégré, `apple1.toml`.
    include!(concat!(env!("OUT_DIR"), "/v1_build.rs"));
}
pub mod github {
    //! Projets GitHub et « Tester le dernier build » (v1.0 `github.rs`).
    include!(concat!(env!("OUT_DIR"), "/v1_github.rs"));
}
pub mod params {
    //! Réglages par programme, `programs.json` (v1.0 `params.rs`).
    include!(concat!(env!("OUT_DIR"), "/v1_params.rs"));
}
pub mod tapesig {
    //! Signal de la cassette rebâti à partir des fronts (v1.0 `tapesig.rs`).
    include!(concat!(env!("OUT_DIR"), "/v1_tapesig.rs"));
}

/// Équivalents des fonctions de `apple1-gui/src/library.rs` (v1.0).
pub mod library {
    use std::path::Path;

    /// Extensions des programmes de la bibliothèque de la v1.0.
    pub const EXTENSIONS: [&str; 13] = [
        "wav", "aiff", "aif", "txt", "bin", "apl", "hex", "prg", "bas", "s", "asm", "a65", "inc",
    ];

    /// Exemple de la v1.0 (vidage du Woz Monitor), pour les essais.
    #[cfg(test)]
    pub const HELLO_TXT: &str = "0300: A2 00 BD 0E 03 F0 06 20 EF FF E8 D0 F5 4C 1F FF\n0310: 48 45 4C 4C 4F 2C 20 57 4F 52 4C 44 21 0D 00\n";

    pub fn is_program(p: &Path) -> bool {
        let ext = p
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        EXTENSIONS.contains(&ext.as_str())
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Source {
        Basic,
        Asm,
        Art,
    }

    pub fn looks_basic_listing(t: &str) -> bool {
        let lines: Vec<&str> = t.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
        if lines.is_empty()
            || apple1_core::hexdump::parse(t)
                .map(|h| h.total_len() > 0)
                .unwrap_or(false)
        {
            return false;
        }
        let ok = lines
            .iter()
            .filter(|l| {
                let digits = l.chars().take_while(|c| c.is_ascii_digit()).count();
                digits > 0
                    && digits <= 5
                    && l[digits..]
                        .trim_start()
                        .chars()
                        .next()
                        .map(|c| c.is_ascii_alphabetic())
                        .unwrap_or(false)
            })
            .count();
        ok * 10 >= lines.len() * 8
    }

    pub fn source_kind(p: &Path, bytes: Option<&[u8]>) -> Option<Source> {
        let ext = p
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        match ext.as_str() {
            "bas" => Some(Source::Basic),
            "s" | "asm" | "a65" => Some(Source::Asm),
            "txt" => {
                let owned;
                let b = match bytes {
                    Some(b) => b,
                    None => {
                        owned = std::fs::read(p).ok()?;
                        &owned
                    }
                };
                let t = std::str::from_utf8(b).ok()?;
                if looks_basic_listing(t) {
                    Some(Source::Basic)
                } else if apple1_core::hexdump::parse(t)
                    .map(|h| h.total_len() > 0)
                    .unwrap_or(false)
                {
                    None
                } else {
                    super::ascii::looks_art(t, p).then_some(Source::Art)
                }
            }
            _ => None,
        }
    }
}

/// Équivalent de `looks_art` de `apple1-gui/src/ascii.rs` (v1.0).
pub mod ascii {
    use std::path::Path;

    pub fn looks_art(t: &str, path: &Path) -> bool {
        let lines: Vec<&str> = t.lines().map(str::trim_end).collect();
        let n = lines
            .iter()
            .rposition(|l| !l.is_empty())
            .map(|i| i + 1)
            .unwrap_or(0);
        if n == 0 || n > 24 || lines.iter().any(|l| l.chars().count() > 40) || !t.is_ascii() {
            return false;
        }
        let in_art_dir = path
            .parent()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_ascii_lowercase().contains("ascii"))
            .unwrap_or(false);
        let ink: Vec<char> = t.chars().filter(|c| !c.is_whitespace()).collect();
        let symbols = ink.iter().filter(|c| !c.is_ascii_alphanumeric()).count();
        in_art_dir || symbols * 10 >= ink.len() * 4
    }
}

/// Équivalents de `apple1-gui/src/config.rs` (v1.0).
pub mod config {
    use std::path::PathBuf;

    pub fn home() -> PathBuf {
        let var = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
        std::env::var_os(var)
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
    }

    /// Cache de la v1.0 (`~/.cache/apple1-emu`) ; la 2.0 range les projets
    /// GitHub dans sa bibliothèque et ne s'en sert que par défaut.
    pub fn cache_base() -> PathBuf {
        match std::env::var_os("XDG_CACHE_HOME") {
            Some(d) if !d.is_empty() => PathBuf::from(d),
            _ => home().join(".cache"),
        }
        .join("apple1-emu")
    }
}

/// Cassettes reconnues (table de la v1.0, voir [`crate::reconnus`]).
pub mod known {
    use super::params::{ProgKind, ProgParams};
    pub use crate::reconnus::{identifier as identify, Known};

    impl Known {
        /// Réglages d'une cassette reconnue (v1.0 : `Known::params`).
        pub fn params(&self) -> ProgParams {
            let run_addr = if self.basic {
                String::new()
            } else {
                self.run_cmd.trim_end_matches('R').to_string()
            };
            ProgParams {
                ranges: self.ranges.into(),
                load_addr: String::new(),
                run_addr,
                run_cmd: self.run_cmd.into(),
                kind: if self.basic {
                    ProgKind::Basic
                } else {
                    ProgKind::Machine
                },
                needs_basic: self.basic,
                notes: crate::i18n::tf("p_known_notes", &[&self.title]),
                auto: self.title.into(),
            }
        }
    }
}
