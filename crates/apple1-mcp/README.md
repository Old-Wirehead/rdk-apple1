# apple1-mcp — Retro Dev Kit Apple 1 MCP server

*Retro Dev Kit Apple 1 — The Original Sin* (<https://github.com/Old-Wirehead/rdk-apple1>). A minimal [Model Context Protocol](https://modelcontextprotocol.io) server
that lets an AI assistant (Cursor, Claude Desktop, Claude Code…) drive one
headless emulated Apple-1: type on its keyboard, read its screen, assemble
and load 6502 code, load Integer BASIC listings, set breakpoints and inspect
memory. Transport: stdio (newline-delimited JSON-RPC 2.0), protocol versions
2024-11-05, 2025-03-26 and 2025-06-18. The machine runs at full speed.

## Build

```sh
cargo build --release -p apple1-mcp
# binary: target/release/apple1-mcp
```

```
apple1-mcp [--basic PATH]
```

Integer BASIC is not built into the emulator (Apple copyright). Pass your own
image with `--basic` (or the `APPLE1_BASIC` environment variable, or the
`basic_path` argument of the `reset` tool): `.wav`/`.aiff` cassette, 4 KB raw
binary, or Woz Monitor hex dump. It is loaded at `$E000` at every power-on.

## Tools

| Tool | What it does |
|------|--------------|
| `reset` | RESET button; with options (`hard`, `basic_path`, `start_basic`, `color_card`, `ram_kb`) a power-on with a new configuration |
| `type_text` | type keys (`\n` = RETURN, upper-cased), run until the machine waits for a key, return the screen |
| `read_screen` | the 40×24 text screen |
| `run` | run `cycles` or `ms` of Apple-1 time (optionally `until_idle`) |
| `run_until_break` | run until a breakpoint, a jam or `max_cycles` |
| `step` | execute `n` instructions, show the registers and the next instruction |
| `registers` | PC, A, X, Y, SP, P and flags |
| `read_memory` / `write_memory` | Woz-style hex/ASCII dump; write hex bytes into RAM |
| `assemble` | ca65-syntax assembler; loads the code, optional `run`; listing + symbols or errors |
| `load_basic_listing` | tokenize an Integer BASIC listing, load it, warm start (`E2B3R`) and `RUN` |
| `load_file` | load a `.wav`/`.aiff` cassette, hex dump or raw binary (`addr`) |
| `set_breakpoint` / `clear_breakpoint` | instruction breakpoints |
| `disassemble` | disassembly (with the last assembly's symbols) |

Addresses are hex strings (`"0300"`, `"$0300"`, `"0x300"`) or integers.

## Client configuration

Use absolute paths.

**Cursor** — `~/.cursor/mcp.json` (or `.cursor/mcp.json` in a project):

```json
{
  "mcpServers": {
    "apple1": {
      "command": "/path/to/apple1-emu/target/release/apple1-mcp",
      "args": ["--basic", "/path/to/BASIC.wav"]
    }
  }
}
```

**Claude Desktop** — `~/.config/Claude/claude_desktop_config.json` (Linux),
`~/Library/Application Support/Claude/claude_desktop_config.json` (macOS):

```json
{
  "mcpServers": {
    "apple1": {
      "command": "/path/to/apple1-emu/target/release/apple1-mcp",
      "args": ["--basic", "/path/to/BASIC.wav"]
    }
  }
}
```

**Claude Code**: `claude mcp add apple1 -- /path/to/apple1-mcp --basic /path/to/BASIC.wav`

Drop `"args"` to run without BASIC (monitor, assembler and debugger only).

## Example

`type_text {"text": "E000R\n"}` → BASIC prompt `>`;
`load_basic_listing {"text": "10 PRINT \"HELLO\"\n20 END\n"}` → `HELLO` on the screen.

## Limits

- One session per server process; no tape recording/export, no GEN2 graphics
  output (only the text screen), no BASIC-line breakpoints yet.
- "Idle" means: keyboard queue empty and the CPU looping within 16 bytes for
  30 000 cycles. A program in a tight delay loop is also reported idle; a
  long computation returns "cycle budget reached" — call `run` again.
- `read_memory` does not trigger I/O side effects (PIA, ACI).
