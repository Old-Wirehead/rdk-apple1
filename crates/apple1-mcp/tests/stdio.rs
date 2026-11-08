//! Drive the apple1-mcp binary over stdio like an MCP client.

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use serde_json::{json, Value};

struct Client {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    id: u64,
}

impl Client {
    fn start() -> Client {
        let mut child = Command::new(env!("CARGO_BIN_EXE_apple1-mcp"))
            .env_remove("APPLE1_BASIC")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("spawn apple1-mcp");
        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());
        let mut c = Client { child, stdin, stdout, id: 0 };
        let r = c.request("initialize", json!({"protocolVersion": "2025-03-26", "capabilities": {}, "clientInfo": {"name": "test", "version": "0"}}));
        assert_eq!(r["result"]["protocolVersion"], "2025-03-26");
        assert_eq!(r["result"]["serverInfo"]["name"], "retro-dev-kit-apple1");
        assert_eq!(r["result"]["serverInfo"]["title"], "Retro Dev Kit Apple 1 — The Original Sin");
        c.send(json!({"jsonrpc": "2.0", "method": "notifications/initialized"}));
        c
    }
    fn send(&mut self, v: Value) {
        writeln!(self.stdin, "{v}").unwrap();
        self.stdin.flush().unwrap();
    }
    fn request(&mut self, method: &str, params: Value) -> Value {
        self.id += 1;
        self.send(json!({"jsonrpc": "2.0", "id": self.id, "method": method, "params": params}));
        let mut line = String::new();
        self.stdout.read_line(&mut line).unwrap();
        let v: Value = serde_json::from_str(&line).expect("json reply");
        assert_eq!(v["id"], self.id, "reply id ({line})");
        v
    }
    /// Call a tool; returns (text, isError).
    fn tool(&mut self, name: &str, args: Value) -> (String, bool) {
        let r = self.request("tools/call", json!({"name": name, "arguments": args}));
        let res = &r["result"];
        (res["content"][0]["text"].as_str().unwrap_or_default().to_string(), res["isError"].as_bool().unwrap())
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

#[test]
fn handshake_ping_and_tools_list() {
    let mut c = Client::start();
    assert_eq!(c.request("ping", json!({}))["result"], json!({}));
    let r = c.request("tools/list", json!({}));
    let names: Vec<&str> = r["result"]["tools"].as_array().unwrap().iter().map(|t| t["name"].as_str().unwrap()).collect();
    for n in ["reset", "type_text", "read_screen", "run", "step", "registers", "read_memory", "write_memory", "assemble", "load_basic_listing", "load_file", "set_breakpoint", "clear_breakpoint", "run_until_break", "disassemble"] {
        assert!(names.contains(&n), "{n} missing");
    }
    assert_eq!(c.request("nope", json!({}))["error"]["code"], -32601);
    assert_eq!(c.request("tools/call", json!({"name": "nope"}))["error"]["code"], -32602);
}

#[test]
fn monitor_session_and_memory() {
    let mut c = Client::start();
    let (t, _) = c.tool("type_text", json!({"text": "300: A9 41\n300.301\n"}));
    assert!(t.contains("0300: A9 41"), "{t}");
    let (t, _) = c.tool("read_memory", json!({"addr": "$300", "len": 2}));
    assert!(t.contains("A9 41"), "{t}");
    c.tool("write_memory", json!({"addr": 0x302, "bytes": "EA 00"}));
    let (t, _) = c.tool("disassemble", json!({"addr": "0300", "count": 2}));
    assert!(t.contains("LDA #$41") && t.contains("NOP"), "{t}");
    let (t, err) = c.tool("read_memory", json!({"addr": "zz"}));
    assert!(err, "{t}");
}

const HELLO: &str = "ECHO = $FFEF
        .org $0300
start:  ldx #0
loop:   lda msg,x
        beq done
        jsr ECHO
        inx
        bne loop
done:   jmp $FF1F
msg:    .byte \"HELLO\", 0
";

#[test]
fn assemble_run_hello_and_breakpoint() {
    let mut c = Client::start();
    let (t, err) = c.tool("assemble", json!({"source": HELLO, "run": true}));
    assert!(!err, "{t}");
    assert!(t.contains("0300  A2 00"), "{t}");
    let (scr, _) = c.tool("read_screen", json!({}));
    assert!(scr.lines().any(|l| l == "HELLO"), "{scr}");
    // breakpoint on the JSR ECHO: run again from the monitor
    c.tool("set_breakpoint", json!({"addr": "0307"}));
    c.tool("type_text", json!({"text": "300R\n", "max_cycles": 100000}));
    let (r, _) = c.tool("registers", json!({}));
    assert!(r.starts_with("PC=$0307"), "{r}");
    let (t, _) = c.tool("step", json!({}));
    assert!(t.starts_with("PC=$FFEF"), "{t}");
    c.tool("clear_breakpoint", json!({}));
    let (t, _) = c.tool("run_until_break", json!({"max_cycles": 200000}));
    assert!(t.contains("budget"), "{t}");
    let (t, err) = c.tool("assemble", json!({"source": " lda (", "load": false}));
    assert!(err && t.contains("error"), "{t}");
}

#[test]
fn basic_session_if_available() {
    let basic = "/tmp/gt2/lib/BASIC.wav";
    if !std::path::Path::new(basic).exists() {
        eprintln!("skipped: {basic} not present (Integer BASIC is not shipped)");
        return;
    }
    let mut c = Client::start();
    let (t, err) = c.tool("reset", json!({"basic_path": basic}));
    assert!(!err && t.ends_with(">\n```"), "{t}");
    let (t, _) = c.tool("type_text", json!({"text": "PRINT 6*7\n"}));
    assert!(t.contains("\n42\n"), "{t}");
    let (t, err) = c.tool("load_basic_listing", json!({"text": "10 FOR I=1 TO 3\n20 PRINT \"N\";I\n30 NEXT I\n40 END\n"}));
    assert!(!err && t.contains("N1\nN2\nN3\n"), "{t}");
}
