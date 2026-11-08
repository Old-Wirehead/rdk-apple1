//! Klaus Dormann's 6502_functional_test (documented NMOS opcodes, decimal
//! mode included). The binary is GPL and is NOT shipped: it is taken from
//! $DORMANN_BIN, or target/dormann/, or downloaded with curl
//! (scripts/fetch_dormann.sh). The test is skipped when unavailable.
use apple1_core::cpu::{Bus, Cpu};
use std::path::PathBuf;

const URL: &str = "https://github.com/Klaus2m5/6502_65C02_functional_tests/raw/master/bin_files/6502_functional_test.bin";
const SUCCESS: u16 = 0x3469;

struct Flat(Vec<u8>);
impl Bus for Flat {
    fn read(&mut self, a: u16) -> u8 {
        self.0[a as usize]
    }
    fn write(&mut self, a: u16, v: u8) {
        self.0[a as usize] = v;
    }
}

fn binary() -> Option<Vec<u8>> {
    if let Ok(p) = std::env::var("DORMANN_BIN") {
        return std::fs::read(p).ok();
    }
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/dormann");
    let file = dir.join("6502_functional_test.bin");
    if !file.exists() {
        let _ = std::fs::create_dir_all(&dir);
        let ok = std::process::Command::new("curl")
            .args(["-fsSL", "--max-time", "60", "-o"])
            .arg(&file)
            .arg(URL)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !ok {
            let _ = std::fs::remove_file(&file);
            return None;
        }
    }
    std::fs::read(file).ok()
}

#[test]
fn klaus_dormann_functional_test() {
    let Some(bin) = binary() else {
        eprintln!("SKIP: 6502_functional_test.bin unavailable (offline?) - run scripts/fetch_dormann.sh");
        return;
    };
    assert_eq!(bin.len(), 65536);
    assert_eq!(&bin[SUCCESS as usize..SUCCESS as usize + 3], &[0x4C, 0x69, 0x34], "unexpected binary version");
    let mut bus = Flat(bin);
    let mut cpu = Cpu::new();
    cpu.pc = 0x0400;
    let last;
    let mut instructions: u64 = 0;
    loop {
        let pc = cpu.pc;
        cpu.step(&mut bus);
        instructions += 1;
        if cpu.pc == pc {
            last = pc;
            break;
        }
        assert!(instructions < 200_000_000, "runaway at {:04X}", cpu.pc);
    }
    eprintln!("Dormann: trap at ${last:04X} after {instructions} instructions, {} cycles", cpu.cycles);
    assert_eq!(last, SUCCESS, "failed test trap at ${last:04X} (see 6502_functional_test.lst)");
}
