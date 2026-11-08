//! BASIC test harness: loads a BASIC image at $E000 (REAL=path to a raw
//! image), cold starts it (E000R) and types a session file line by line,
//! printing the transcript (every character written to the display).
//!
//!   BASIC_ROM=rdk-basic.bin cargo run --release -p apple1-core --example bastest -- SESSION.txt [--gen2]
//!
//! Lines starting with `\x` are not special: each line is typed followed by
//! RETURN. A line `#KEY:text` types text without RETURN. `#WAIT:n` runs n
//! million cycles. Ends when the machine has been idle for a while.
use apple1_core::machine::{Config, Machine};

fn idle(m: &mut Machine, max: u64) -> bool {
    // idle = the program polls the keyboard status ($D011) many times with
    // nothing typed and nothing printed in between
    let end = m.board.cycles + max;
    let mut polls = 0u32;
    let mut last_len = m.board.display.log.as_ref().map_or(0, |l| l.len());
    while m.board.cycles < end {
        let pc = m.cpu.pc;
        let poll = m.board.peek(pc.wrapping_add(1)) == 0x11 && m.board.peek(pc.wrapping_add(2)) == 0xD0;
        m.step();
        let len = m.board.display.log.as_ref().map_or(0, |l| l.len());
        if len != last_len || !m.board.key_queue.is_empty() || m.board.pia.key_available() {
            last_len = len;
            polls = 0;
        } else if poll {
            polls += 1;
            if polls > 3000 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let rom = std::fs::read(std::env::var("BASIC_ROM").expect("BASIC_ROM=path")).expect("rom");
    let cfg = Config { display_throttle: false, color_card: a.iter().any(|x| x == "--gen2"), ..Default::default() };
    let mut m = Machine::new(cfg);
    m.board.display.log = Some(Vec::new());
    m.run_cycles(100_000);
    m.load(0xE000, &rom);
    if let Ok(pre) = std::env::var("BASIC_PRE") {
        // BASIC_PRE=addr:file : extra block loaded before the start
        let (ad, f) = pre.split_once(':').unwrap();
        m.load(u16::from_str_radix(ad, 16).unwrap(), &std::fs::read(f).unwrap());
    }
    m.type_text("E000R\n");
    if let Ok(line) = std::env::var("BASIC_TRACE") {
        m.run_cycles(2_000_000);
        m.type_text(&line);
        m.type_text("\n");
        // BASIC_TRACE=1 : stop when the PC leaves ROM/RAM sanity (page 0-1) and print the last PCs
        let mut ring = std::collections::VecDeque::new();
        for _ in 0..3_000_000u64 {
            m.step();
            ring.push_back(m.cpu.pc);
            if ring.len() > 3000 {
                ring.pop_front();
            }
            if m.cpu.pc < 0x0200 || (0xFD81..0xFFEF).contains(&m.cpu.pc) {
                break;
            }
        }
        let v: Vec<u16> = ring.iter().cloned().collect();
        let mut k = v.len().saturating_sub(1);
        while k > 0 && v[k] == v[k - 1].wrapping_add(3) {
            k -= 1;
        }
        let from = k.saturating_sub(30);
        eprintln!("trace (before the sequential run): {:04X?}", &v[from..=k]);
        eprintln!("A={:02X} X={:02X} Y={:02X} SP={:02X}", m.cpu.a, m.cpu.x, m.cpu.y, m.cpu.sp);
    }
    idle(&mut m, 5_000_000);
    let session = std::fs::read_to_string(&a[0]).unwrap();
    for line in session.lines() {
        if let Some(t) = line.strip_prefix('!') {
            // type the line and run 3 million cycles without waiting for idle
            m.type_text(t);
            m.type_text("\n");
            m.run_cycles(3_000_000);
            continue;
        } else if let Some(t) = line.strip_prefix("#KEY:") {
            m.type_text(t);
        } else if let Some(n) = line.strip_prefix("#WAIT:") {
            m.run_cycles(n.parse::<u64>().unwrap() * 1_000_000);
            continue;
        } else if let Some(r) = line.strip_prefix("#MON:") {
            // typed to the Woz monitor after leaving BASIC with a reset
            m.reset();
            m.run_cycles(200_000);
            m.type_text(r);
            m.type_text("\n");
        } else {
            m.type_text(line);
            m.type_text("\n");
        }
        idle(&mut m, std::env::var("BASIC_MAXIDLE").ok().and_then(|v| v.parse().ok()).unwrap_or(400_000_000));
    }
    if let Ok(d) = std::env::var("BASIC_DUMP") {
        let (a, l) = d.split_once(':').unwrap();
        let (a, l) = (u16::from_str_radix(a, 16).unwrap(), usize::from_str_radix(l, 16).unwrap());
        for i in 0..l {
            if i % 16 == 0 {
                eprint!("\n{:04X}:", a as usize + i);
            }
            eprint!(" {:02X}", m.board.peek(a.wrapping_add(i as u16)));
        }
        eprintln!();
    }
    eprintln!("pc={:04X} a={:02X} cycles={}", m.cpu.pc, m.cpu.a, m.board.cycles);
    let l = m.board.display.log.as_ref().unwrap();
    let s: String = l.iter().map(|&c| if c == 0x0D { '\n' } else { c as char }).collect();
    print!("{s}");
}
