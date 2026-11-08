//! EPROM / EEPROM programmers: read, write and verify a chip from the kit.
//!
//! A programmer is a [`Programmer`]. One is provided:
//!
//! * [`Minipro`]: the `minipro` command line tool (XGecu TL866II+, T48, T56…),
//!   used only when it is installed (`minipro` in the `PATH`, or a path
//!   given by the user). Nothing is linked: the tool is run as a program.
//!
//! Adding another programmer = implementing the trait; the GUI only talks to
//! the trait and to [`all`].

use std::path::PathBuf;
use std::process::Command;

/// A chip the user can pick: the name the programmer knows, and its size.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chip {
    /// Label shown to the user, e.g. `27C256`.
    pub name: &'static str,
    /// Name given to `minipro -p` (device name + package).
    pub minipro: &'static str,
    pub size: usize,
}

/// Common chips for an Apple-1 (ROMs, ACI, BASIC EPROMs, GEN2 expansion…).
pub const CHIPS: &[Chip] = &[
    Chip { name: "2716", minipro: "2716@DIP24", size: 2048 },
    Chip { name: "2732", minipro: "2732@DIP24", size: 4096 },
    Chip { name: "2764", minipro: "2764@DIP28", size: 8192 },
    Chip { name: "27C64", minipro: "27C64@DIP28", size: 8192 },
    Chip { name: "27C128", minipro: "27C128@DIP28", size: 16384 },
    Chip { name: "27C256", minipro: "27C256@DIP28", size: 32768 },
    Chip { name: "27C512", minipro: "27C512@DIP28", size: 65536 },
    Chip { name: "28C64", minipro: "AT28C64B@DIP28", size: 8192 },
    Chip { name: "28C256", minipro: "AT28C256@DIP28", size: 32768 },
    Chip { name: "SST27SF256", minipro: "SST27SF256@DIP28", size: 32768 },
    Chip { name: "SST27SF512", minipro: "SST27SF512@DIP28", size: 65536 },
    Chip { name: "23128 (mask ROM, read only)", minipro: "UM23128@DIP28", size: 16384 },
];

/// Look a chip up by its label (case insensitive); `None` = unknown.
pub fn chip(name: &str) -> Option<&'static Chip> {
    CHIPS.iter().find(|c| c.name.eq_ignore_ascii_case(name.trim()))
}

/// Result of comparing the chip with a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verify {
    /// Number of bytes compared.
    pub compared: usize,
    /// Number of different bytes (also counts missing bytes).
    pub different: usize,
    /// First differences: (address, expected, found), at most 16.
    pub first: Vec<(usize, u8, u8)>,
}

impl Verify {
    pub fn ok(&self) -> bool {
        self.different == 0
    }
}

/// Compare what was expected (`want`) with what the chip holds (`got`).
pub fn compare(want: &[u8], got: &[u8]) -> Verify {
    let n = want.len().max(got.len());
    let mut v = Verify { compared: n, different: 0, first: Vec::new() };
    for i in 0..n {
        let (a, b) = (want.get(i).copied(), got.get(i).copied());
        if a != b {
            v.different += 1;
            if v.first.len() < 16 {
                v.first.push((i, a.unwrap_or(0xFF), b.unwrap_or(0xFF)));
            }
        }
    }
    v
}

/// A way to talk to a chip programmer.
pub trait Programmer {
    /// Stable id (`minipro`).
    fn id(&self) -> &'static str;
    /// Name shown to the user.
    fn name(&self) -> &'static str;
    /// Is it usable now? `Ok(details)` or `Err(clean message)`.
    fn detect(&self) -> Result<String, String>;
    /// Can this programmer write a chip?
    fn can_write(&self) -> bool;
    /// Read `chip` (`size` bytes).
    fn read(&self, chip: &Chip) -> Result<Vec<u8>, String>;
    /// Program `data` into `chip`.
    fn write(&self, chip: &Chip, data: &[u8]) -> Result<String, String>;
    /// Compare the chip with `data` (reads it back).
    fn verify(&self, chip: &Chip, data: &[u8]) -> Result<Verify, String> {
        let got = self.read(chip)?;
        Ok(compare(data, &got))
    }
}

/// Every programmer known to the kit, in the order shown.
pub fn all() -> Vec<Box<dyn Programmer>> {
    vec![Box::new(Minipro::default())]
}

// ---------------------------------------------------------------- minipro

/// The `minipro` tool (TL866II+, T48, T56…).
#[derive(Debug, Clone)]
pub struct Minipro {
    /// Program to run (default: `minipro` in the PATH).
    pub exe: PathBuf,
}

impl Default for Minipro {
    fn default() -> Self {
        Minipro { exe: PathBuf::from("minipro") }
    }
}

impl Minipro {
    fn run(&self, args: &[&str]) -> Result<String, String> {
        let out = Command::new(&self.exe).args(args).output().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                "minipro is not installed (see https://gitlab.com/DavidGriffith/minipro : on Debian/Ubuntu/Mint try « sudo apt install minipro »)".to_string()
            } else {
                format!("cannot run minipro: {e}")
            }
        })?;
        let text = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
        if out.status.success() {
            Ok(text)
        } else {
            let last = text.lines().rev().find(|l| !l.trim().is_empty()).unwrap_or("no message");
            Err(format!("minipro: {}", last.trim()))
        }
    }

    /// Arguments of a read (also used by tests).
    pub fn read_args<'a>(device: &'a str, file: &'a str) -> Vec<&'a str> {
        vec!["-p", device, "-r", file]
    }
}

fn temp_file(tag: &str) -> PathBuf {
    let n = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
    std::env::temp_dir().join(format!("apple1-eprom-{}-{n}-{tag}.bin", std::process::id()))
}

impl Programmer for Minipro {
    fn id(&self) -> &'static str {
        "minipro"
    }
    fn name(&self) -> &'static str {
        "minipro (TL866II+ / T48 / T56)"
    }
    fn detect(&self) -> Result<String, String> {
        // `minipro -k` lists the programmers it finds; any output is enough to
        // show the tool is there (older versions: `-V` / `--version`).
        match self.run(&["-k"]) {
            Ok(t) => Ok(t.lines().find(|l| !l.trim().is_empty()).unwrap_or("minipro").trim().to_string()),
            Err(e) if e.contains("not installed") => Err(e),
            Err(_) => self.run(&["-V"]).map(|t| t.lines().next().unwrap_or("minipro").trim().to_string()),
        }
    }
    fn can_write(&self) -> bool {
        true
    }
    fn read(&self, chip: &Chip) -> Result<Vec<u8>, String> {
        let f = temp_file("read");
        let r = self.run(&Self::read_args(chip.minipro, &f.to_string_lossy()));
        let data = r.and_then(|_| std::fs::read(&f).map_err(|e| format!("{}: {e}", f.display())));
        let _ = std::fs::remove_file(&f);
        let data = data?;
        if data.len() != chip.size {
            return Err(format!("minipro returned {} bytes, expected {} for {}", data.len(), chip.size, chip.name));
        }
        Ok(data)
    }
    fn write(&self, chip: &Chip, data: &[u8]) -> Result<String, String> {
        if data.len() > chip.size {
            return Err(format!("the file ({} bytes) is larger than the {} ({} bytes)", data.len(), chip.name, chip.size));
        }
        let f = temp_file("write");
        // pad with $FF (an erased chip) up to the chip size
        let mut full = data.to_vec();
        full.resize(chip.size, 0xFF);
        std::fs::write(&f, &full).map_err(|e| format!("{}: {e}", f.display()))?;
        let r = self.run(&["-p", chip.minipro, "-w", &f.to_string_lossy()]);
        let _ = std::fs::remove_file(&f);
        r.map(|t| t.lines().rev().find(|l| !l.trim().is_empty()).unwrap_or("done").trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_reports_differences() {
        let v = compare(&[1, 2, 3], &[1, 9, 3, 4]);
        assert_eq!((v.compared, v.different), (4, 2));
        assert_eq!(v.first, vec![(1, 2, 9), (3, 0xFF, 4)]);
        assert!(compare(&[5; 8], &[5; 8]).ok());
    }

    #[test]
    fn chips_by_name() {
        assert_eq!(chip("27c256").unwrap().size, 32768);
        assert!(chip("nonsense").is_none());
        assert!(CHIPS.iter().all(|c| c.size.is_power_of_two()));
    }

    #[test]
    fn missing_minipro_gives_a_clean_message() {
        let m = Minipro { exe: PathBuf::from("/nonexistent/minipro-xyz") };
        assert!(m.detect().unwrap_err().contains("not installed"));
        assert!(m.read(chip("27C64").unwrap()).unwrap_err().contains("not installed"));
    }

    #[cfg(unix)]
    #[test]
    fn minipro_with_a_fake_tool_reads_writes_and_verifies() {
        use std::os::unix::fs::PermissionsExt;
        let dir = std::env::temp_dir().join(format!("fake-minipro-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let chipfile = dir.join("chip.bin");
        std::fs::write(&chipfile, vec![0xFFu8; 8192]).unwrap();
        // a fake `minipro`: -r copies the "chip" to the file, -w copies the file to the "chip"
        let script = format!(
            "#!/bin/sh\nwhile [ $# -gt 0 ]; do case \"$1\" in -p) dev=\"$2\"; shift 2;; -r) cp {c} \"$2\"; echo ok; exit 0;; -w) cp \"$2\" {c}; echo 'Verification OK'; exit 0;; -k) echo 'Found TL866II+ 04.2.132'; exit 0;; *) shift;; esac; done\nexit 1\n",
            c = chipfile.display()
        );
        let exe = dir.join("minipro");
        std::fs::write(&exe, script).unwrap();
        std::fs::set_permissions(&exe, std::fs::Permissions::from_mode(0o755)).unwrap();
        let m = Minipro { exe };
        assert!(m.detect().unwrap().contains("TL866II+"));
        let c = chip("27C64").unwrap();
        let prog: Vec<u8> = (0..5000u32).map(|i| (i * 7) as u8).collect();
        assert_eq!(m.write(c, &prog).unwrap(), "Verification OK");
        let back = m.read(c).unwrap();
        assert_eq!(&back[..5000], &prog[..]);
        assert!(back[5000..].iter().all(|b| *b == 0xFF));
        let mut padded = prog.clone();
        padded.resize(8192, 0xFF);
        assert!(m.verify(c, &padded).unwrap().ok());
        padded[10] ^= 0xFF;
        let v = m.verify(c, &padded).unwrap();
        assert_eq!((v.different, v.first[0].0), (1, 10));
        assert!(m.write(c, &vec![0; 9000]).unwrap_err().contains("larger"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn only_minipro_is_listed() {
        let v = all();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].id(), "minipro");
        assert!(v[0].can_write());
    }
}
