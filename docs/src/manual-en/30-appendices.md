# Appendices <span class="part-sub">Memory map, Woz Monitor commands, ACI timings, character set, keyboard shortcuts, pitfalls, licence and credits</span> {: .part data-part="Appendices" }

# Memory map {: .appendix }

![Apple-1 memory in the emulator, from the monitor vectors down to zero page.](diagrams/en/memoire.svg "diagram large")

| Addresses | Contents | Details |
|---|---|---|
| `$0000-$00FF` | zero page | Woz Monitor `$24-$2B`; Integer BASIC `$4A-$FF` (pointers LOMEM `$4A`, HIMEM `$4C`, PP `$CA`, PV `$CC`) |
| `$0100-$01FF` | 6502 stack | |
| `$0200-$027F` | monitor line buffer | 128 characters |
| `$0280-$0FFF` | base RAM (4 KB) | machine-language programs; BASIC between LOMEM (`$0800`) and HIMEM (`$1000`) |
| `$1000-$BFFF` | expansion RAM | 8 KB by default in the emulator (up to `$1FFF`), up to 48 KB with `--ram` |
| `$C000-$C0FF` | ACI: output toggle, tape input | any access toggles the output |
| `$C100-$C1FF` | ACI: PROM (the ACI program) | started with `C100R` |
| `$4000-$7FFF` | P-LAB CodeTank | 16 KB EPROM, read-only, when a ROM is plugged in (over the RAM) |
| `$C250-$C257` | GEN2 card: switches | read; text/LORES pages `$0400`, `$0800`, HIRES `$2000`, `$4000` |
| `$CC00-$CC01` | P-LAB card: TMS9918A | `$CC00` data, `$CC01` control and status; repeated over `$CC00-$CFFF` |
| `$D010-$D013` | PIA: `KBD`, `KBDCR`, `DSP`, `DSPCR` | chapter 22 |
| `$E000-$EFFF` | BASIC | RDK G2 Basic (7,933 bytes, needs RAM up to `$FEFF`), or Apple’s Integer BASIC (4 KB of RAM, loaded from tape or an image) |
| `$FF00-$FFFF` | Woz Monitor | vectors: NMI `$0F00`, RESET `$FF00`, IRQ/BRK `$0000` |

# Woz Monitor commands {: .appendix }

The monitor waits for a line ended by <kbd>RETURN</kbd>. Addresses and bytes are written in hexadecimal, without `$`; a line may contain several commands separated by spaces.

| Command | Effect | Example |
|---|---|---|
| `addr` | displays the byte at that address | `FFEF` |
| `addr.addr` | displays a range, 8 bytes per line | `FF00.FF1F` |
| `.addr` | continues the display up to that address | `.FF3F` |
| `addr: byte byte …` | stores bytes starting at the address | `300: A9 C1 20 EF FF` |
| `: byte …` | continues storing | `: 4C 1F FF` |
| `addrR` | runs the program at that address (`JMP`) | `300R` |
| <kbd>Esc</kbd> | cancels the line (prints `\`) | |
| `_` (backspace) | cancels the last character | |

{{!addrR}}{{!addr.addr}}

A line longer than 127 characters is cancelled as if by Esc. After `R`, the monitor first prints the byte at the address (`0300: A2`), then the program takes over.

| Address | Command | Role |
|---|---|---|
| `E000R` | BASIC cold start | clears the program |
| `E2B3R` | BASIC warm start | keeps the program |
| `C100R` | the ACI program | then `start.endR` (read) or `start.endW` (write) |
| `FF00R` | software RESET of the monitor | |

# ACI timings {: .appendix }

The durations are produced by the ACI program, by counting cycles between two toggles of the output. Values measured in the emulator, which runs this program on its 6502 (1.022727 MHz clock, with the 4-cycle stall every 65 cycles for memory refresh):

| Element | Cycles | Duration | Frequency |
|---|---|---|---|
| header half-period | 592 | 618 µs | ≈ 809 Hz |
| first half of the start bit | 180 | 188 µs | |
| half-period of a “0” | 238 | 248 µs | ≈ 2.0 kHz |
| half-period of a “1” | 473 | 494 µs | ≈ 1.0 kHz |
| complete header | 16,390 half-periods | ≈ 10.1 s | |

Format of a recording:

<ol class="steps">
<li>the <strong>header</strong>: about 10 seconds of 618 µs half-periods;</li>
<li>the <strong>start bit</strong>: one short half-period (188 µs), then a “0” half-period;</li>
<li>the <strong>data</strong>: each byte, most significant bit first, one full period per bit;</li>
<li>nothing else: no length, no address, no checksum. The read command supplies the range.</li>
</ol>

When reading, the ACI program measures half-periods with a 12-cycle loop. It takes as the start bit any half-period shorter than 31 iterations (≈ 372 cycles, 388 µs), and separates “0” from “1” at about 0.74 ms per full period. The emulator’s file decoder accepts headers from 0.40 to 0.90 ms per half-period, which covers the c2t tool (1 kHz) and tape speed variations.

# The 2513 character set {: .appendix }

![The 64 characters of the Signetics 2513, as the emulator draws them: codes $20 to $5F (with bit 7, $A0 to $DF).](img/charset-2513.png "large")

The character generator knows only 64 symbols: space, punctuation, digits and upper-case letters. The terminal keeps only 6 bits of the code it receives:

| Code sent | Display |
|---|---|
| `$20-$5F` (or `$A0-$DF`) | the character from the table |
| `$60-$7F` (lower case, or `$E0-$FF`) | the corresponding upper-case letter (`$40-$5F`) |
| `$0D` (or `$8D`) | new line |
| other `$00-$1F` | nothing: ignored |

Each character is a 5 × 7 dot matrix in a 7 × 8 cell; the screen is 40 × 24 characters, or 280 × 192 dots.

# Keyboard shortcuts {: .appendix }

| Key | Effect |
|---|---|
| <kbd>Ctrl</kbd>+<kbd>R</kbd> | Apple-1 RESET |
| <kbd>Ctrl</kbd>+<kbd>L</kbd> | CLEAR SCREEN |
| <kbd>Ctrl</kbd>+<kbd>V</kbd> | paste: types the clipboard on the Apple-1 keyboard |
| <kbd>F1</kbd> | help mode (detailed tooltips) |
| <kbd>F3</kbd> | real or fast speed |
| <kbd>F11</kbd> | full screen |
| <kbd>Ctrl</kbd>+<kbd>+</kbd> / <kbd>Ctrl</kbd>+<kbd>−</kbd> / <kbd>Ctrl</kbd>+<kbd>0</kbd> | screen zoom (or editor text zoom, when the editor has the keyboard focus) |
| <kbd>Ctrl</kbd>+<kbd>S</kbd> | save the editor’s file |
| <kbd>Ctrl</kbd>+<kbd>F</kbd> / <kbd>Ctrl</kbd>+<kbd>H</kbd> / <kbd>Ctrl</kbd>+<kbd>G</kbd> | editor: find, replace, go to line |
| <kbd>Ctrl</kbd>+<kbd>/</kbd> | editor: comment / uncomment (6502), REM / un-REM (BASIC) |
| <kbd>F5</kbd> / <kbd>F9</kbd> / <kbd>F12</kbd> | editor: run, breakpoint on the line, go to the definition (6502) |
| <kbd>Ctrl</kbd>+letter | control character (`Ctrl+D` = `$84`) |
| right-click on a screen | Copy, Paste (type the text), Clear screen, Reset |
| <kbd>Esc</kbd> | ESC (`$9B`): cancels the line in the monitor and in BASIC |
| backspace | types `_`, the monitor’s and BASIC’s rubout |

In the **terminal** (`apple1-emu` without a window, chapter 12):

| Key | Effect |
|---|---|
| <kbd>F2</kbd> | emulator command line |
| <kbd>F5</kbd> / <kbd>F6</kbd> | tape: play, stop |
| <kbd>F10</kbd> or <kbd>Ctrl</kbd>+<kbd>Q</kbd> | quit |

# Pitfalls and behaviours to know {: .appendix }

Small things that surprise people the first time. None of them is a bug; each one has a reason.

- **BASIC and RUN.** The editor’s **▶ Run** starts BASIC, loads the listing and types `RUN` for you. A program that asks for input waits for your keys on the Apple-1 screen: click the screen first.
- **RND.** The first `RND` of a run would always be the same on a machine that has just been reset; **▶ Run** waits a random moment before `RUN`, so a game does not always begin the same way.
- **Tape commands.** The line “Tape, on the real Apple-1” under the editor is the Woz Monitor command to type on a real machine (`C100R`, then the `R` commands of the ranges) before you play the tape. It is a hint, not something to type in the emulator.
- **The keyboard goes to one place at a time.** The line at the bottom of the window says where: the Apple-1, the editor, a field. Clicking the Apple-1 screen sends the keys to the machine; clicking the editor takes them back. After an error is clicked in the editor’s list, the keys return to the editor.
- **Paused debugger.** Keys typed while the machine is paused are not lost, but only the **last** one is kept (the real keyboard latch holds one character); it is delivered when you continue.
- **Breakpoints go with the program.** Loading another program (file, library, assembler) clears the breakpoints and labels of the previous one. The breakpoints you set stay from one session to the next only as long as you do not load something else.
- **Pasting with the middle button.** In the editor, a middle click moves the cursor there and pastes the X11/Wayland **primary selection** (the text you just highlighted in any application). The editor does not yet *write* its own selection to the primary selection: to copy out of the editor use <kbd>Ctrl</kbd>+<kbd>C</kbd>.
- **Small windows.** On a monitor narrower than 1400 points the first-launch layout is editor and screen; from the **⊞ Layout ▾** menu you can choose another. On a 1024 × 768 screen the toolbar scrolls sideways: the **»** button reaches the Cassette group, instead of cutting it off on the right. A fixed zoom (<kbd>Ctrl</kbd>+<kbd>+</kbd>) larger than the screen pane is kept, and the screen scrolls in its pane; the status bar tells you so.
- **Saving an untitled tab.** **Save as** (icon, <kbd>Ctrl</kbd>+<kbd>S</kbd> or the right-click menu of the tab) never leaves two tabs for the same file: a tab already open on that file is closed.
- **First launch and the network.** The first start contacts GitHub to fetch the demo project (chapter 4); remove the project from the “GitHub projects” list in **⚙ Preferences** to stop that for good.
- **Reading the keyboard in G2 Basic.** A key pressed while a program runs stops it, as on the original, and `INPUT` waits. A game that needs to watch the keys calls `KEY(0)` in its main loop (chapter 15); once it has, a key no longer stops the program.
- **Labels and breakpoints.** A breakpoint on a label that does not exist yet (typed before you assemble or run) is kept and set when the label appears.
- **Layouts and windows.** A saved layout remembers the docked panels, the detached windows (place and size) and the size of the main window. It does not remember which window has the keyboard.

# Licence and credits {: .appendix }

**Retro Dev Kit Apple 1** is free software, released under the **GNU General Public License, version 3** (GPL-3.0), without any warranty. The full text is in the `LICENSE` file of the archive and of the repository.

Some parts come from other projects and keep their own licence, the **MIT** licence, which is compatible with the GPL; they are listed in the `NOTICE` file:

- **RDK G2 Basic**, the built-in BASIC, written for this project (repository `Old-Wirehead/rdk-gen2basic-apple1`, MIT);
- **Uncle Bernie’s GEN2 tests** (`gen2-test1` to `gen2-test4`, the bars) and the **GEN2 glyphs**, supplied by Uncle Bernie.

The games of the library written by **Claude IA** (Hangman, Matches, Battleship, Tunnel and Tic-tac-toe) are distributed with the project.

What the kit does **not** contain: Apple’s Integer BASIC, commercial tapes, and the demos of the P-LAB card. Those stay with their owners; you bring your own copies.

The P-LAB card is a creation of P-LAB; the TMS9918A is a Texas Instruments chip. This project is not affiliated with Apple, Texas Instruments or P-LAB.
