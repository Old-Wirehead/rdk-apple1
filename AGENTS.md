## Cursor Cloud specific instructions

Single Rust workspace, one product: an Apple-1 emulator. Crates are `apple1-core` (library), `apple1-emu` (terminal UI and cassette CLI), and `apple1-gui` (window). No database, daemon, or external service is required.

- The VM image also has rustup toolchain **1.83.0**, which cannot compile this lockfile (`eframe` 0.36 needs rustc 1.95+). The default toolchain is **stable** (1.99+) on `PATH` via `/usr/local/cargo/bin`. Leave that default in place.
- Build, test, and run commands are in `README.md` (section « Compiler depuis les sources » and §7). Use the dev profile: `cargo build`, `cargo test`, `cargo run -p apple1-emu -- …`, `cargo run -p apple1-gui -- …`. `[profile.test] opt-level = 3` is already set, so `cargo test` without `--release` is the right suite.
- `cargo clippy --workspace --all-targets` is clean. `cargo fmt --all -- --check` reports existing formatting drift; that is not an environment failure.
- The Klaus Dormann test downloads `target/dormann/6502_functional_test.bin` on first `cargo test` and needs network. Offline, that one test skips (see `README.md` §7).
- `DISPLAY` is available for `apple1-emu-gui`. The terminal UI needs a TTY of at least 42×28. If the window fails to open, use `LIBGL_ALWAYS_SOFTWARE=1` as described in the README. Integer BASIC is not shipped. A working check is `samples/hello.txt` then `300R` (CLI: `cargo run -p apple1-emu -- --fast --load samples/hello.txt --run 300`).
