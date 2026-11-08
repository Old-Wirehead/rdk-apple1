---
version: 1.0
date: November 2026
---

# The emulator <span class="part-sub">The window, the screen, the library, the ACI cassette interface, the GEN2 color card, the debugger and the command line</span> {: .part data-part="Part I" }

# About this manual

<p class="lead"><strong>Retro Dev Kit Apple 1</strong> — “The Original Sin” — reproduces the 1976 Apple-1 cycle for cycle: the 6502, the video terminal, the keyboard, the ACI cassette interface and, as options, Uncle Bernie’s GEN2 color card and the P-LAB TMS9918A graphics card. It comes with its own free BASIC, <strong>RDK G2 Basic</strong>. Around the machine it adds a complete workshop: a program library, an editor, an assembler, a debugger and export to a real Apple-1.</p>

This manual describes the whole emulator, version 1.0 (the first public release; the 7.x numbers were the test versions). It complements the *Quick Start Guide*, which is enough to install the program and play a first tape. Here every panel, every button and every command is explained, with examples you can type as they are.

## How this manual is organized

<div class="cards">
<div class="card"><h4>Part I — The emulator</h4>The window and its panels, the vintage screen, the library, the tape and the ACI, the GEN2 and P-LAB graphics cards, the editor, the debugger, ASCII art, the preferences, the command line and troubleshooting.</div>
<div class="card"><h4>Part II — Integer BASIC</h4>Steve Wozniak’s BASIC and its free clone RDK G2 Basic: every statement, function and operator, memory, the error messages and the known bugs, with their workarounds.</div>
<div class="card"><h4>Part III — The 6502 assembler</h4>The 6502 from the ground up, the built-in assembler and its directives, the Woz Monitor routines, complete examples and the pitfalls to avoid.</div>
<div class="card"><h4>Appendices</h4>Memory map, Woz Monitor commands, the ACI format, the 2513 character set, keyboard shortcuts, and the command index.</div>
</div>

## Conventions

Keys are written like this: <kbd>Ctrl</kbd>+<kbd>R</kbd>, <kbd>RETURN</kbd>. Interface buttons are in bold: **▶ Run**. Addresses are in hexadecimal, prefixed with `$` as in the documents of the time: `$FFEF`. The Woz Monitor itself writes them without `$`: `FFEF`.

Sessions on the Apple-1 screen have a dark background. What **you** type is white, the machine’s answer is green:

```mon
» 300R

0300: A2
HELLO, APPLE-1!
```

Four kinds of boxes appear along the way:

!!! astuce "Tip"
    A shortcut, or a better way of doing something.

!!! attention "Caution"
    A trap that costs time or data.

!!! apple1 "On the real machine"
    What happens on a real Apple-1, and what differs from the emulator.

!!! bug "Known bug"
    A defect of the original ROM, with its workaround. Each bug has a status: <span class="verified">verified</span> (reproduced in the emulator with the real Integer BASIC) or <span class="unverified">not verified</span> (quoted from a source, not tried).

Every example in this manual was tried in the emulator. The 6502 programs of Part III are in the `docs/exemples/en/asm/` folder of the distribution. They produce exactly the same bytes with the built-in assembler and with ca65.

## What you need

The emulator runs on Linux with nothing to install. The Woz Monitor and the ACI ROM are built in, and so is **RDK G2 Basic**, a free BASIC that is compatible with Integer BASIC (chapter 14): the **BASIC** button works at once, on a fresh install, with no file to find. The 1976 **Integer BASIC** of Apple is **not** supplied, because it is still under copyright. If you want the original, use your own copy (for example the `BASIC.wav` tape from the “Apple1Cassettes” collection on archive.org) and pick it in **⚙ Preferences → Advanced → BASIC**.

!!! attention "ROMs and tapes stay with you"
    The emulator ships neither Apple’s Integer BASIC nor commercial tapes. Its recognition table holds only sizes, fingerprints and parameters. Keep your copies in your personal library.

!!! note "Licence"
    Retro Dev Kit Apple 1 is free software under the **GNU GPL 3.0** (no warranty). Some parts keep their own MIT licence: RDK G2 Basic and Uncle Bernie’s GEN2 tests and glyphs. See `LICENSE` and `NOTICE` in the archive, and the appendix “Licence and credits”.

## The main paths

Four ways of using the emulator come up again and again. Each is described later with its own diagram:

| I want to… | The path | See |
|---|---|---|
| play a program of the time | library → **▶** | chapter 4 |
| write a BASIC program | **📝 Editor** → **▶ Run** → **📼 Export WAV** | Part II |
| write a 6502 program | **📝 Editor** (`.s`) → **📃 Listing** → **🐛 Debugger** | Part III |
| send a program to a real Apple-1 | **📼 Export WAV** (`.aiff`) → smartphone → ACI | chapter 5 |

# The window and the toolbar

## Overview

![The default layout: every panel open and docked.](img/en/defaut-annote.png "large")

<ol class="legend">
<li><strong>Load, Save, State</strong>: <code>.txt</code> hex dumps, <code>.bin</code> and <code>.prg</code> binaries; Save also writes <code>.wav</code>/<code>.aiff</code> tapes; State saves and restores the machine.</li>
<li><strong>RESET, Clear screen, Speed (gauge), Paste</strong>: the machine’s buttons (Ctrl+R, Ctrl+L, F3, Ctrl+V).</li>
<li><strong>BASIC</strong>: loads the built-in RDK G2 Basic at <code>$E000</code> (or your Integer BASIC, if you chose it).</li>
<li><strong>Windows</strong>: the <strong>⊞ Layout ▾</strong> menu, then one button per panel (Monochrome screen, Colour screen, Editor, Debugger, Disassembly, Dump, Library, ASCII).</li>
<li><strong>Graphics cards</strong>: <strong>GEN2</strong> and <strong>P-LAB</strong> plug the cards in or out.</li>
<li><strong>Tape</strong>: <strong>📼 Insert</strong> a tape, then play, stop, rewind, record, eject; oscilloscope and sound.</li>
<li><strong>EPROM</strong>, <strong>⚙ Preferences</strong>, <strong>?</strong> (detailed help bubbles, also F1) and <strong>ℹ</strong> (about, shortcuts).</li>
<li><strong>Display row of the screen</strong>: zoom (1x to 8x or Fit), phosphor, display speed <strong>Normal</strong> or <strong>Fast</strong>, vintage screen, full screen, <strong>Detach</strong>.</li>
<li><strong>Editor</strong>: BASIC programs and 6502 sources in tabs.</li>
<li><strong>Apple-1 screen</strong>: 40 × 24 characters; click it to type, right-click for Copy, Paste, Clear screen and Reset.</li>
<li><strong>Debugger</strong>: pause, single step, registers, breakpoints, variables.</li>
<li><strong>Disassembly</strong>: the code around the program counter (PC).</li>
<li><strong>Dump</strong>: memory in hexadecimal and ASCII, editable.</li>
<li><strong>Library</strong>: your programs, each with its small icons ▶ ⬇ 📼 ⚙.</li>
<li><strong>Status bar</strong>: speed, display rate, tape, keyboard target; below it, the latest message.</li>
</ol>

Every panel (screen, editor, debugger, library…) is a tab that you can move, group or detach into its own window: that is the subject of chapter 10. On first launch the window opens maximized and a **Your workspace** window proposes five layouts (chapter 10); the size and position of the window and the arrangement of its panels are remembered, and found again as they were at the next launch.

## The toolbar, button by button

The toolbar is a single row: the machine and its tools on the left, the tape deck in the middle, the panels in the Windows group, the cards, and the settings on the right. When the window is too narrow, the buttons keep only their icon (the text stays in the tooltip), or the bar wraps onto several lines if you untick **Compact bar** in the preferences.

![The toolbar up to the Graphics cards group: files, machine, windows and cards. The Tape strip follows (chapter 5), then the settings.](img/en/barre-1.png "large")

| Button | What it does |
|---|---|
| {{Load}} | Opens a program and puts it into memory: a `.txt` (Woz Monitor dump) or a `.hex`; for a `.bin`, the address is asked for. Tapes (`.wav`, `.aiff`) do not go through here: they are inserted with **📼 Insert**, in the Tape strip. |
| {{Save}} | Asks for one or more ranges (`0300.03FF`, `004A.00FF 0300.0FFF`…), then a file. The extension selects the format: `.wav`, `.aiff`, `.txt` or `.bin`. |
| **RESET** (red button) | The Apple-1 RESET button (<kbd>Ctrl</kbd>+<kbd>R</kbd>). The 6502 and the PIA restart; memory and screen are kept. |
| **🆑 Clear screen** | The CLEAR SCREEN button (<kbd>Ctrl</kbd>+<kbd>L</kbd>). |
| **Gauge (Real speed / Fast speed)** | Toggles between the original 1 MHz and maximum speed (<kbd>F3</kbd>). A speedometer dial: lit blue while fast speed is on. |
| **📋 Paste** | Types the clipboard on the Apple-1 keyboard, at keyboard pace (<kbd>Ctrl</kbd>+<kbd>V</kbd>). Ideal for a hex dump. |
| {{BASIC|**BASIC**}} | Loads the built-in **RDK G2 Basic** at `$E000` (7,933 bytes, then `E000R` starts it). If you chose the original Integer BASIC in **⚙ Preferences → Advanced → BASIC**, it loads that one instead: a tape, a `.txt` dump or a 4096-byte `.bin`, looked for in the library (`BASIC.wav`, `integer_basic.bin`, any name containing “basic”), otherwise asked for. Right-click: choose another file. |
| **Windows** (group: **⊞ Layout ▾** and one button per panel) | Shows or hides each panel: **Monochrome screen** (the Apple-1 terminal), **Colour screen** (the screen of the graphics card that is plugged in, GEN2 or P-LAB: the same button serves both; with no card plugged in, it opens a choice to plug one, and **P-LAB** then shows its screen), **📝 Editor** (chapter 7), **🐛 Debugger**, **Disassembly**, **Dump** (chapter 8), **📚 Library** and **🎨 ASCII** (chapter 9). A lit button means the panel is visible; a panel that is already shown can also be closed with the cross of its tab. |
| **Graphics cards: GEN2 and P-LAB** | A named group. **GEN2** plugs in or unplugs Uncle Bernie’s color card (chapter 6); **P-LAB** does the same for the P-LAB TMS9918A card (section 6.6). A button is lit while its card is plugged in; the screen of a card appears in its own panel, shown by the **Colour screen** button. |
| **📸 State ▾** | A single button with a menu: **Save state…** (<kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>S</kbd>), **Load a state…** (<kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>O</kbd>), then the recent states, each with its title and date (section 2.5). |
| **EPROM** (chip with a UV window, on the right of the bar, before **⊞ Layout**) | Opens the EPROM programmer: read, write or verify a chip with a `minipro` programmer (section 2.6). |
| **Tape: 📼 Insert ▶ ⏹ ⏪ ⏺ ⏏** | The deck, gathered in one strip. **📼 Insert** picks a `.wav` or `.aiff` and puts it in the deck as a virtual tape; then play, stop, rewind, record, eject. The strip shows the name, position and length. |
| **📈** and **🔊** | The tape oscilloscope, and the sound of the signal (chapter 5). |
| **⊞ Layout ▾** (first button of the Windows group) | A menu: **Default** (every panel, as at the first launch), **Game** (Apple-1 screen and library), **BASIC** (editor and screen), **Assembly** (editor, screen and debugger), **Debugging** (editor, debugger, disassembly and dump); then **Save the current layout…**, which asks for a name, and your saved layouts in alphabetical order (right-click: Rename, Delete). A tick marks the last choice; everything is kept in `gui.json`. |
| **⚙** | The preferences (chapter 11). |
| **?** | Help mode (<kbd>F1</kbd>): detailed bubbles on every control, in the 7 interface languages. |
| **ℹ** | “About”, with the list of keyboard shortcuts. |

**File windows.** Open, Save as and Export use the emulator’s own file window. A blue **📁+ New folder** button, at the top left, creates a folder in the folder being shown, with a tooltip to explain it; type the name, press <kbd>Enter</kbd>, and the new folder is ready to receive your file.

![The Load window, with the 📁+ New folder button at the top left.](img/en/fenetre-fichiers.png "medium")

The display settings are not in the toolbar: each screen has its own **display row**, at the top of its panel (section 3.2).

![The display row of the Apple-1 screen: zoom, phosphor, display speed, vintage screen and full screen; Detach is at the right end of the row.](img/en/barre-2.png "large")

!!! astuce "Help mode"
    When you explore a panel, switch **?** on: every button explains what it does when you hover over it. Off, only the usual short tooltips remain (they can be turned off in **⚙ → Interface**). The setting is remembered.

## The status bar

![The status bar while BASIC.wav is playing: speed, display limit, tape, keyboard, and the latest message below.](img/en/barre-etat.png "large")

- **Measured speed**: about 0.960 MHz at real speed. This is the effective clock of the Apple-1, which halts the 6502 for four cycles out of sixty-five to refresh its dynamic memory.
- **Display ~60 chars/s**: the Apple-1 terminal displays only one character per frame, about 60 per second. Click to switch between this limit and instant display, like the **Normal** / **Fast** buttons of the display rows.
- **Tape**: name, position and length, or “no tape”.
- **Keyboard**: where the keys go (the Apple-1, or a text field of the interface).

## The Apple-1 keyboard

Click the screen: the keys go to the Apple-1. The original keyboard has upper case only, and the emulator converts lower case.

| PC key | For the Apple-1 |
|---|---|
| letters, digits, symbols | the character, in upper case |
| <kbd>Enter</kbd> | RETURN (code `$8D`) |
| <kbd>Backspace</kbd> | `_`: the Woz Monitor and BASIC rub out the previous character (it stays on screen, followed by `_`) |
| <kbd>Esc</kbd> | ESC: cancels the current line in the Woz Monitor |
| <kbd>Ctrl</kbd>+letter | the control character (`Ctrl+D` = `$84`…) |
| <kbd>Ctrl</kbd>+<kbd>R</kbd> / <kbd>Ctrl</kbd>+<kbd>L</kbd> | the RESET and CLEAR SCREEN buttons |

You can **select text with the mouse** on either screen: press and drag over the characters, and the selection is highlighted. **Right-clicking** either screen (the Apple-1’s or the GEN2 monitor) opens a small menu: **Copy** puts the selected text on the clipboard (or the whole text of the screen when nothing is selected), **Paste (type the text)** types the clipboard on the Apple-1 keyboard, as if you were typing it, then **Clear screen** and **Reset**.

![The right-click menu of the Apple-1 screen, during a game of Hamurabi.](img/en/ecran-menu.png "medium")

!!! apple1 "The Apple-1 screen erases nothing"
    The terminal can only write a character and move to the next line. There is no movable cursor and no way to erase a character: the rub-out key prints `_` and the character stays on screen. This is normal.

## Machine state

The **📸 State** button of the toolbar takes a complete snapshot of the machine and puts it back later, exactly at the same point: you can stop a game or a program that is running, close the emulator, and resume it the next day. The button opens a menu:

![The State menu: save, load, then the recent states with their date.](img/en/menu-etat.png "medium")

- **Save state…** (<kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>S</kbd>) asks for a file name and writes a `.a1state` file. The proposed folder is `etats`, next to the library folder.
- **Load a state…** (<kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>O</kbd>) replaces the machine by the chosen state. The status bar shows the file and the value of PC.
- **Recent states** lists the last eight, with their title and the date of the save; a click loads one at once. If the file has been moved or deleted, a message says so.

A state contains the memory, the 6502 registers, the PIA, the screen, the GEN2 card if it is plugged in, and the virtual cassette with its position. It does not contain your preferences or the library. A file that cannot be read (damaged, or not a state) is refused with a message, and the machine is left untouched.

!!! astuce "Typical use"
    Save a state just before a difficult passage of a game, or at the end of a long BASIC session: a state is also a convenient way to share a running program with a friend who uses the same version.

## The EPROM programmer

**EPROM** in the toolbar opens a window to read, write and verify an EPROM or EEPROM chip from the emulator, for example to burn the BASIC ROM, a program for the GEN2 expansion, or the contents of your own memory.

![The EPROM programmer: programmer, chip, data source and the three operations.](img/en/programmateur-eprom.png "medium")

Only the programmers driven by the free **`minipro`** tool are supported: the XGecu **TL866II+**, **T48** and **T56**. On Debian, Ubuntu and Linux Mint, install it with `sudo apt install minipro`, plug the programmer into a USB port, then press **Detect**. If `minipro` is missing, the window says so and explains what to do; nothing else is needed on the emulator’s side.

| Setting | Role |
|---|---|
| **Programmer** | the detected programmer, with its firmware (for example “Found TL866II+ 04.2.132 (USB)”) |
| **Chip** | the common chips of the Apple-1 world: 2716, 2732, 2764, 27C64, 27C128, 27C256, 27C512, 28C64, 28C256, SST27SF256, SST27SF512, and the 23128 mask ROM (read only) |
| **Data** | the **Apple-1 memory** (an address range, `$E000`–`$EFFF` by default), or a **file** (`.bin`, `.hex`, `.aiff`…) |

Three buttons act on the chip, and a progress bar and a log show what is happening:

- **Read the chip** reads the whole chip. With the memory or an existing file as the data, the content is **compared** and the differences are listed: an existing file is never overwritten. With a new file name, the content is saved in it.
- **Write the chip** programs the chip with the chosen data. The chip must be blank or erasable, and a confirmation of the result is shown at the end.
- **Verify** compares the chip with the chosen data, byte for byte, and reports each difference (`$E010: chip $4C, data $A9`).

**Close** closes the window.

The window never changes the emulated memory: writing and verifying read it, and reading only compares it or fills a file.

# The screen: zoom, phosphor and vintage effect

## The original screen

The Apple-1 displays 40 columns × 24 lines of upper-case characters. Each character is a 5 × 7 dot matrix in a 7 × 8 cell, read from the **Signetics 2513** character generator ROM: the emulator uses the contents of that ROM, dot for dot (Appendix D). The picture is therefore 280 × 192 dots.

The cursor is a blinking `@`, driven by an NE555: 2.18 Hz, visible about two thirds of the time. On some boards the capacitor tolerance gives 1.7 Hz instead; the `--cursor-hz 1.7` option reproduces it.

## The display row

Each screen has its own row of settings at the top of its panel: the Apple-1 screen, and the color monitor when the GEN2 card is plugged in (chapter 6).

| Setting | Role |
|---|---|
| **Zoom** | a list: `1x` to `8x` magnify the picture by an integer factor, so the dots stay square and sharp; **Fit** chooses the largest factor that fits in the panel (the chosen factor is shown in brackets). From the keyboard: <kbd>Ctrl</kbd>+<kbd>+</kbd>, <kbd>Ctrl</kbd>+<kbd>−</kbd>, <kbd>Ctrl</kbd>+<kbd>0</kbd> (fit). |
| **Phosphor** | Apple-1 screen only: the green of the monitors of the time, the white of a TV set, or amber. |
| **Display: Normal / Fast** | **Normal** shows the characters at the pace of the real terminal, about 60 per second; **Fast** shows everything at once. This setting is shared: changing it on one screen changes it on the other. |
| **Vintage screen** | the 1970s tube effect (next section); each screen has its own checkbox. |
| **⛶** | full screen (<kbd>F11</kbd>). |
| **Detach** | puts the screen in its own window (chapter 10). |

Each screen is **centered** in its panel, horizontally and vertically, with its caption centered below the picture. If the chosen zoom is larger than the panel, the screen is shown at the largest zoom that fits instead of being cut off. When the window is narrower than the display row, the row wraps onto a second line, and **Detach** stays at the right end of the last line.

![Amber phosphor, with the vintage screen and a slight curvature.](img/en/brut/ambre.png "medium")

## The vintage screen

The **Vintage screen** checkbox of the display row imitates a 1970s CRT monitor. The main effect is the thin dark line between two scan lines: each row of dots becomes a horizontal band, as in a photograph of a real Apple-1. This line keeps the same thickness in pixels of your display at every zoom: it stays thin and sharp, even full screen.

![The vintage screen, with a slight tube curvature (0.15) and dark corners, during a game of Hamurabi.](img/en/brut/vintage.png "medium")

In **⚙ Preferences → Advanced → Vintage screen: fine settings**, you set the **Intensity** and choose the effects; these settings are shared by both screens. Each slider shows its **numerical value** next to it (curvature, intensity…):

| Effect | Default | Role |
|---|---|---|
| gaps between scan lines | on | the dark line between two rows of dots |
| horizontally softened dots | on | a slight blur, horizontal only, like the beam |
| phosphor afterglow | short | dots fade in less than a tenth of a second (a little more slowly in amber); far left: none |
| glow | on, subtle | a halo around the dots, which stay sharp |
| curved tube and dark corners | off | with a **Curvature** slider |
| flicker | off | a very slight brightness variation |
| RGB phosphor mask | on | colour screen of the GEN2 card only: fine vertical red, green and blue stripes separated by thin dark lines, as on a colour TV tube. It is visible from zoom 3 up, and its strength follows the **Intensity** |

The effect can also be applied to the color monitor of the GEN2 card, with the checkbox of its own display row. It is computed by the CPU, at the Apple-1 resolution, and only when the picture changes: it stays smooth even without graphics acceleration.

On the command line: `--screening`, `--sans-screening`, `--gen2-screening` (GEN2 monitor), or for example `--screening-options intensite=0.7,remanence=0.5,courbure=0.4,sans-halo` (values from 0 to 1; the option names are French).

!!! astuce "Detached screen and second monitor"
    **Detach**, at the end of the display row, puts the Apple-1 screen in its own window. Place it on a second monitor, full screen, with the vintage effect: the keyboard goes to the Apple-1 as soon as that window has the focus. With **Fit**, the picture follows the window size, in integer steps (**Integer scale**, in the window’s title bar, the sharpest) or filling it.
