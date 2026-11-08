//! apple1-mcp (Retro Dev Kit Apple 1): a minimal Model Context Protocol server (stdio transport,
//! newline-delimited JSON-RPC 2.0) holding one headless Apple-1 session.
//!
//! Usage: `apple1-mcp [--basic PATH]` (PATH: Integer BASIC as .wav/.aiff,
//! raw 4 KB binary or Woz hex dump, loaded at $E000 on every power-on).
//! The environment variable `APPLE1_BASIC` is used when `--basic` is absent.

mod session;
mod tools;

use std::io::{BufRead, Write};

use serde_json::{json, Value};

const SUPPORTED: [&str; 3] = ["2025-06-18", "2025-03-26", "2024-11-05"];

fn main() {
    let mut basic = std::env::var_os("APPLE1_BASIC").map(Into::into);
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--basic" => basic = args.next().map(Into::into),
            "-h" | "--help" => {
                println!("Retro Dev Kit Apple 1 — The Original Sin\napple1-mcp [--basic PATH]   MCP server on stdin/stdout (see README.md)");
                return;
            }
            "-V" | "--version" => {
                println!("apple1-mcp (Retro Dev Kit Apple 1) {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            x => eprintln!("apple1-mcp: argument ignored: {x}"),
        }
    }
    let mut s = session::Session::new(basic);
    let stdin = std::io::stdin();
    let mut out = std::io::stdout().lock();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        let reply = match serde_json::from_str::<Value>(&line) {
            Err(e) => Some(error(Value::Null, -32700, &format!("parse error: {e}"))),
            Ok(Value::Array(batch)) => {
                let r: Vec<Value> = batch.into_iter().filter_map(|m| handle(&mut s, m)).collect();
                (!r.is_empty()).then_some(Value::Array(r))
            }
            Ok(m) => handle(&mut s, m),
        };
        if let Some(r) = reply {
            let _ = writeln!(out, "{r}");
            let _ = out.flush();
        }
    }
}

fn error(id: Value, code: i64, msg: &str) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": msg}})
}

fn handle(s: &mut session::Session, m: Value) -> Option<Value> {
    let id = m.get("id").cloned();
    let method = m.get("method").and_then(Value::as_str).unwrap_or("");
    let params = m.get("params").cloned().unwrap_or(json!({}));
    let Some(id) = id else {
        // notification (notifications/initialized, cancelled...): no reply
        return None;
    };
    if method.is_empty() {
        // a response from the client (we never send requests): ignore
        return if m.get("result").is_some() || m.get("error").is_some() { None } else { Some(error(id, -32600, "invalid request")) };
    }
    let result = match method {
        "initialize" => {
            let asked = params.get("protocolVersion").and_then(Value::as_str).unwrap_or("");
            let v = if SUPPORTED.contains(&asked) { asked } else { SUPPORTED[1] };
            json!({
                "protocolVersion": v,
                "capabilities": {"tools": {"listChanged": false}},
                "serverInfo": {"name": "retro-dev-kit-apple1", "title": "Retro Dev Kit Apple 1 — The Original Sin", "version": env!("CARGO_PKG_VERSION")},
                "instructions": "One emulated Apple-1 (Woz Monitor at $FF00, ACI at $C100, optional Integer BASIC at $E000), running at full speed. Type monitor commands with type_text (e.g. 'E000R' to start BASIC), then read_screen."
            })
        }
        "ping" => json!({}),
        "tools/list" => json!({"tools": tools::list()}),
        "tools/call" => {
            let name = params.get("name").and_then(Value::as_str).unwrap_or("");
            let args = params.get("arguments").cloned().unwrap_or(json!({}));
            match tools::call(s, name, &args) {
                Some(Ok(text)) => json!({"content": [{"type": "text", "text": text}], "isError": false}),
                Some(Err(e)) => json!({"content": [{"type": "text", "text": e}], "isError": true}),
                None => return Some(error(id, -32602, &format!("unknown tool: {name}"))),
            }
        }
        _ => return Some(error(id, -32601, &format!("method not found: {method}"))),
    };
    Some(json!({"jsonrpc": "2.0", "id": id, "result": result}))
}
