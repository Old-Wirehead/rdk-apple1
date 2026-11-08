//! Tool definitions (tools/list) and their implementation (tools/call).

use std::path::Path;

use apple1_core::program::{self, Program};
use apple1_core::{asm6502, debug, hexdump, intbasic};
use serde_json::{json, Value};

use crate::session::{self, Session, CLOCK};

const ADDR: &str = "address: hex string ('0300', '$0300', '0x300') or integer";

fn tool(name: &str, desc: &str, props: Value, required: &[&str]) -> Value {
    json!({"name": name, "description": desc, "inputSchema": {"type": "object", "properties": props, "required": required}})
}

pub fn list() -> Vec<Value> {
    let addr = json!({"type": ["string", "integer"], "description": ADDR});
    vec![
        tool("reset", "Press RESET (back to the Woz Monitor '\\' prompt). With any option, or hard=true, power-cycle with a fresh configuration (memory cleared). Integer BASIC is not built in: give basic_path (or start the server with --basic) to load it at $E000.",
            json!({"hard": {"type": "boolean"}, "basic_path": {"type": "string", "description": "Integer BASIC image (.wav/.aiff, 4 KB .bin, or hex dump)"},
                   "start_basic": {"type": "boolean", "description": "type E000R after loading BASIC (default true)"},
                   "color_card": {"type": "boolean", "description": "Uncle Bernie's GEN2 colour card (implies 48 KB RAM)"},
                   "ram_kb": {"type": "integer", "description": "RAM from $0000, 4..48 KB (default 8)"}}), &[]),
        tool("type_text", "Type on the Apple-1 keyboard (lower case is upper-cased, \\n becomes RETURN) and run until the input is consumed and the machine waits for a key. Returns the screen.",
            json!({"text": {"type": "string"}, "max_cycles": {"type": "integer", "description": "default 20000000 (about 20 s of Apple-1 time)"}}), &["text"]),
        tool("read_screen", "The 40x24 text screen.", json!({}), &[]),
        tool("run", "Run at full speed for some cycles or milliseconds of Apple-1 time (stops early on a breakpoint or a jam). Returns the stop reason and the registers.",
            json!({"cycles": {"type": "integer"}, "ms": {"type": "number"}, "until_idle": {"type": "boolean", "description": "also stop when waiting for a key"}}), &[]),
        tool("run_until_break", "Run until a breakpoint, a jam, or max_cycles (default 50000000).", json!({"max_cycles": {"type": "integer"}}), &[]),
        tool("step", "Execute n instructions (default 1) and disassemble the next one.", json!({"n": {"type": "integer"}}), &[]),
        tool("registers", "CPU registers and flags.", json!({}), &[]),
        tool("read_memory", "Hex/ASCII dump (Woz Monitor format) of len bytes (default 128, max 4096). No side effects on I/O.",
            json!({"addr": addr, "len": {"type": "integer"}}), &["addr"]),
        tool("write_memory", "Write bytes into RAM. bytes: hex string ('A9 00 8D') or an array of integers.",
            json!({"addr": addr, "bytes": {"type": ["string", "array"], "items": {"type": "integer"}}}), &["addr", "bytes"]),
        tool("assemble", "Assemble 6502 source (ca65 syntax; .org sets the origin, default $0300). Loads into memory unless load=false; run=true then jumps to the origin and runs until idle. Returns the listing (with the symbol table) or the errors.",
            json!({"source": {"type": "string"}, "org": addr, "load": {"type": "boolean"}, "run": {"type": "boolean"}}), &["source"]),
        tool("load_basic_listing", "Tokenize an Integer BASIC listing like the real interpreter, put it in memory (zero page + program below HIMEM), warm start BASIC (RESET, E2B3R) and RUN it unless run=false. Needs BASIC loaded.",
            json!({"text": {"type": "string"}, "run": {"type": "boolean"}, "max_cycles": {"type": "integer"}}), &["text"]),
        tool("load_file", "Load a program file into memory: .wav/.aiff (ACI cassette), Woz hex dump (.txt/.hex), or raw binary (needs addr).",
            json!({"path": {"type": "string"}, "addr": addr}), &["path"]),
        tool("set_breakpoint", "Add a breakpoint on an instruction address.", json!({"addr": addr}), &["addr"]),
        tool("clear_breakpoint", "Remove one breakpoint, or all when addr is omitted. Returns the remaining ones.", json!({"addr": addr}), &[]),
        tool("disassemble", "Disassemble count instructions (default 16) from addr (default PC). Symbols from the last assembly are used.",
            json!({"addr": addr, "count": {"type": "integer"}}), &[]),
    ]
}

type R = Result<String, String>;

fn opt_addr(a: &Value, k: &str) -> Result<Option<u16>, String> {
    match a.get(k) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(n)) => n.as_u64().filter(|v| *v <= 0xFFFF).map(|v| Some(v as u16)).ok_or(format!("{k}: 0..65535 expected")),
        Some(Value::String(s)) => program::parse_addr(s).map(Some).map_err(|_| format!("{k}: bad address '{s}'")),
        _ => Err(format!("{k}: {ADDR}")),
    }
}
fn addr(a: &Value, k: &str) -> Result<u16, String> {
    opt_addr(a, k)?.ok_or(format!("{k} is required"))
}
fn int(a: &Value, k: &str) -> Option<u64> {
    a.get(k).and_then(Value::as_u64)
}
fn flag(a: &Value, k: &str) -> Option<bool> {
    a.get(k).and_then(Value::as_bool)
}
fn text<'a>(a: &'a Value, k: &str) -> Result<&'a str, String> {
    a.get(k).and_then(Value::as_str).ok_or(format!("{k} (string) is required"))
}

fn screen_block(s: &Session) -> String {
    format!("```\n{}\n```", s.screen().trim_end_matches('\n'))
}

pub fn call(s: &mut Session, name: &str, a: &Value) -> Option<R> {
    Some(match name {
        "reset" => reset(s, a),
        "type_text" => text(a, "text").map(|t| {
            s.m.type_text(t);
            let stop = s.run(int(a, "max_cycles").unwrap_or(20_000_000), true);
            format!("{}\n{}", stop.text(), screen_block(s))
        }),
        "read_screen" => Ok(screen_block(s)),
        "run" => {
            let c = int(a, "cycles").or_else(|| a.get("ms").and_then(Value::as_f64).map(|ms| (ms * CLOCK / 1000.0) as u64)).unwrap_or(1_000_000);
            let stop = s.run(c, flag(a, "until_idle").unwrap_or(false));
            Ok(format!("{}\n{}", stop.text(), s.registers()))
        }
        "run_until_break" => {
            let stop = s.run(int(a, "max_cycles").unwrap_or(50_000_000), false);
            Ok(format!("{}\n{}", stop.text(), s.registers()))
        }
        "step" => {
            let n = int(a, "n").unwrap_or(1).clamp(1, 1_000_000);
            for _ in 0..n {
                s.m.step();
                if s.m.cpu.jammed {
                    break;
                }
            }
            let d = debug::disasm(&|x| s.m.board.peek(x), s.m.cpu.pc, &s.syms);
            Ok(format!("{}\nnext: {:04X}  {}", s.registers(), d.addr, d.text))
        }
        "registers" => Ok(s.registers()),
        "read_memory" => addr(a, "addr").map(|start| {
            let len = int(a, "len").unwrap_or(128).clamp(1, 4096) as u32;
            let end = (start as u32 + len - 1).min(0xFFFF) as u16;
            hexdump::format(start, &s.m.dump(start, end), None)
        }),
        "write_memory" => write_memory(s, a),
        "assemble" => assemble(s, a),
        "load_basic_listing" => load_basic_listing(s, a),
        "load_file" => load_file(s, a),
        "set_breakpoint" => addr(a, "addr").map(|x| {
            s.breaks.insert(x);
            format!("breakpoints: {}", breaks(s))
        }),
        "clear_breakpoint" => opt_addr(a, "addr").map(|x| {
            match x {
                Some(x) => s.breaks.remove(&x),
                None => {
                    s.breaks.clear();
                    true
                }
            };
            format!("breakpoints: {}", breaks(s))
        }),
        "disassemble" => opt_addr(a, "addr").map(|x| {
            let mut pc = x.unwrap_or(s.m.cpu.pc);
            let mut out = String::new();
            for _ in 0..int(a, "count").unwrap_or(16).clamp(1, 256) {
                let d = debug::disasm(&|x| s.m.board.peek(x), pc, &s.syms);
                let hex: Vec<String> = d.bytes.iter().map(|b| format!("{b:02X}")).collect();
                let label = s.syms.get(&pc).map(|n| format!("{n}:")).unwrap_or_default();
                out.push_str(&format!("{:04X}  {:<9} {:<10} {}\n", pc, hex.join(" "), label, d.text));
                pc = pc.wrapping_add(d.bytes.len() as u16);
            }
            out
        }),
        _ => return None,
    })
}

fn breaks(s: &Session) -> String {
    if s.breaks.is_empty() {
        return "none".into();
    }
    s.breaks.iter().map(|b| format!("${b:04X}")).collect::<Vec<_>>().join(" ")
}

fn reset(s: &mut Session, a: &Value) -> R {
    let path = a.get("basic_path").and_then(Value::as_str).map(Into::into);
    let reconf = flag(a, "hard").unwrap_or(false) || path.is_some() || a.get("color_card").is_some() || a.get("ram_kb").is_some();
    let mut notes = Vec::new();
    if reconf {
        let old = &s.m.board.config;
        let ram = int(a, "ram_kb").map(|k| k.clamp(4, 48) as u32).unwrap_or(old.ram_kb);
        let card = flag(a, "color_card").unwrap_or(old.color_card);
        s.power_on(session::config(ram, card));
        notes.push(format!("power-on: {} KB RAM, GEN2 colour card {}", if card { 48 } else { ram }, if card { "on" } else { "off" }));
        if let Some(p) = path.or_else(|| s.basic_path.clone()) {
            let n = s.load_basic(&p)?;
            if a.get("basic_path").is_some() {
                s.basic_path = Some(p.clone());
            }
            notes.push(format!("Integer BASIC loaded at $E000 ({n} bytes) from {}", p.display()));
        }
    } else {
        s.m.reset();
        notes.push("RESET".into());
    }
    s.run(2_000_000, true);
    if reconf && s.basic_loaded && flag(a, "start_basic").unwrap_or(true) {
        s.m.type_text("E000R\n");
        s.run(5_000_000, true);
        notes.push("BASIC started (E000R)".into());
    }
    Ok(format!("{}\n{}", notes.join("; "), screen_block(s)))
}

fn write_memory(s: &mut Session, a: &Value) -> R {
    let start = addr(a, "addr")?;
    let bytes: Vec<u8> = match a.get("bytes") {
        Some(Value::String(t)) => {
            let h: String = t.chars().filter(|c| c.is_ascii_hexdigit()).collect();
            if !h.len().is_multiple_of(2) || t.chars().any(|c| !c.is_ascii_hexdigit() && !" ,\n\t$".contains(c)) {
                return Err("bytes: hex pairs expected, e.g. 'A9 00 8D'".into());
            }
            (0..h.len()).step_by(2).map(|i| u8::from_str_radix(&h[i..i + 2], 16).unwrap()).collect()
        }
        Some(Value::Array(v)) => v.iter().map(|x| x.as_u64().filter(|b| *b < 256).map(|b| b as u8)).collect::<Option<_>>().ok_or("bytes: integers 0..255 expected")?,
        _ => return Err("bytes is required".into()),
    };
    let skipped = s.m.load(start, &bytes);
    let mut r = format!("{} byte(s) written at ${start:04X}", bytes.len() - skipped);
    if skipped > 0 {
        r.push_str(&format!("; {skipped} ignored (not RAM)"));
    }
    Ok(r)
}

fn assemble(s: &mut Session, a: &Value) -> R {
    let src = text(a, "source")?;
    let full = match opt_addr(a, "org")? {
        Some(o) => format!("        .org ${o:04X}\n{src}"),
        None => src.to_string(),
    };
    let asm = asm6502::assemble_text(&full, None, &[]).map_err(|errs| {
        let mut e = format!("{} error(s):\n", errs.len());
        for x in &errs {
            e.push_str(&format!("{x}\n"));
        }
        e
    })?;
    let mut out = format!("assembled {} byte(s) at ${:04X}-${:04X}\n", asm.bytes.len(), asm.origin, asm.end());
    for w in &asm.warnings {
        out.push_str(&format!("warning: {w}\n"));
    }
    s.syms = debug::symbols_from(&asm.symbols);
    if flag(a, "load").unwrap_or(true) {
        let skipped = s.m.load(asm.origin, &asm.bytes);
        out.push_str(&format!("loaded; start with {} at the monitor\n", asm.run_command()));
        if skipped > 0 {
            out.push_str(&format!("WARNING: {skipped} byte(s) outside RAM ignored\n"));
        }
        if flag(a, "run").unwrap_or(false) {
            s.m.cpu.pc = asm.origin;
            let stop = s.run(20_000_000, true);
            out.push_str(&format!("ran: {}\n{}\n{}\n", stop.text(), s.registers(), screen_block(s)));
        }
    }
    out.push_str("\nlisting:\n");
    for r in asm6502::listing_rows(&asm, Some(&full)) {
        out.push_str(r.text.trim_end());
        out.push('\n');
    }
    Ok(out)
}

fn load_basic_listing(s: &mut Session, a: &Value) -> R {
    let l = intbasic::tokenize(text(a, "text")?);
    if !l.errors.is_empty() {
        return Err(l.errors.iter().map(|e| format!("{e}\n")).collect());
    }
    if l.lines.is_empty() {
        return Err("empty listing".into());
    }
    if !s.basic_present() {
        return Err("Integer BASIC is not in memory: call reset with basic_path first (it is not built into the emulator)".into());
    }
    let (lo, hi) = intbasic::layout(l.size(), None);
    let img = intbasic::Image::build(&l.lines, lo, hi).map_err(|_| format!("program of {} bytes: too big", l.size()))?;
    let need = img.ram_kb();
    let mut out = String::new();
    let c = s.m.board.config.clone();
    if !c.color_card && c.ram_kb < need {
        s.m.board.apply_config(session::config(need, false));
        out.push_str(&format!("RAM extended to {need} KB\n"));
    }
    s.m.reset();
    s.run(2_000_000, true);
    s.load_program(&Program { blocks: img.blocks(), run: None, notes: vec![] });
    s.m.type_text("E2B3R\n");
    let mut stop = s.run(5_000_000, true);
    out.push_str(&format!("{} line(s), {} bytes, LOMEM ${:04X} HIMEM ${:04X} (tape: {})\n", l.lines.len(), l.size(), img.lomem, img.himem, img.aci_command()));
    if flag(a, "run").unwrap_or(true) {
        s.m.type_text("RUN\n");
        stop = s.run(int(a, "max_cycles").unwrap_or(20_000_000), true);
    }
    out.push_str(&format!("{}\n{}", stop.text(), screen_block(s)));
    Ok(out)
}

fn load_file(s: &mut Session, a: &Value) -> R {
    let path = text(a, "path")?;
    let p = program::load_file(Path::new(path), opt_addr(a, "addr")?)?;
    let skipped = s.load_program(&p);
    let ranges: Vec<String> = p.ranges().iter().map(|(a, b)| format!("{a:04X}.{b:04X}")).collect();
    let mut r = format!("loaded {path} at {} ({} bytes)", ranges.join(" "), p.total_len());
    if let Some(x) = p.run {
        r.push_str(&format!("; start with {x:X}R"));
    }
    if skipped > 0 {
        r.push_str(&format!("; WARNING {skipped} byte(s) outside RAM ignored"));
    }
    for n in &p.notes {
        r.push_str(&format!("\nnote: {n}"));
    }
    Ok(r)
}
