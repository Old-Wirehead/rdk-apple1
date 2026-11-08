# First launch

On first launch the window opens maximized, and a **Your workspace** window offers five layouts: **Default**, **Game**, **BASIC**, **Assembly** and **Debugging**. Each numbered thumbnail shows the layout, says what it is for and gives the minimum screen size it needs: click the one you want. Untick **Show this window at start-up** if you prefer not to see it again; **⊞ Layout ▾ → Choose a workspace…** opens it again. You can change the layout at any time with the **⊞ Layout ▾** menu. The default layout makes every tool visible at a glance. Each area is a dockable panel that you can move, group into tabs or detach into its own window. The colour screen (GEN2 / P-LAB) is rectangle number 7, in the thumbnails that show it and in the legend; numbers 1 to 6 do not change. This window does not stay above the others. If the window is narrow, the toolbar scrolls: the **»** button brings the Cassette group.

![The default layout. The screen already shows the small program from the next section.](img/en/defaut-annote.png "large")

<ol class="legend">
<li><strong>Load, Save, State</strong>: <code>.txt</code> hex dumps, <code>.bin</code> and <code>.prg</code> binaries; Save also writes <code>.wav</code>/<code>.aiff</code> tapes; State saves and restores the machine.</li>
<li><strong>RESET, Clear screen, Speed (gauge), Paste</strong>: the machine’s buttons (Ctrl+R, Ctrl+L, F3, Ctrl+V).</li>
<li><strong>BASIC</strong>: loads the built-in RDK G2 Basic at <code>$E000</code>.</li>
<li><strong>Windows</strong>: the <strong>⊞ Layout ▾</strong> menu, then one button per panel (screens, Editor, Debugger, Disassembly, Dump, Library, ASCII).</li>
<li><strong>Graphics cards</strong>: <strong>GEN2</strong> and <strong>P-LAB</strong> plug the cards in or out.</li>
<li><strong>Tape</strong>: <strong>📼 Insert</strong> a tape, then play, stop, rewind, record, eject; oscilloscope and sound.</li>
<li><strong>EPROM</strong>, <strong>⚙ Preferences</strong>, <strong>?</strong> (detailed tooltips, also F1) and <strong>ℹ</strong> (about, shortcuts).</li>
<li><strong>Display row of the screen</strong>: zoom (1x to 8x or Fit), phosphor, display speed <strong>Normal</strong> or <strong>Fast</strong>, vintage screen, full screen, <strong>Detach</strong>.</li>
<li><strong>Editor</strong>: BASIC programs and 6502 sources in tabs.</li>
<li><strong>Apple-1 screen</strong>: 40 × 24 characters; click it to type, right-click for Copy, Paste, Clear screen and Reset.</li>
<li><strong>Debugger</strong>: pause, single step, registers, breakpoints, variables.</li>
<li><strong>Disassembly</strong>: the code around the program counter (PC).</li>
<li><strong>Dump</strong>: memory in hex and ASCII, editable.</li>
<li><strong>Library</strong>: your programs, each with its small icons ▶ ⬇ 📼 ⚙.</li>
<li><strong>Status bar</strong>: speed, display rate, tape, keyboard target; below it, the last message.</li>
</ol>

!!! astuce "Making room"
    To just play, close the editor and debugger with their buttons in the Windows group (4): the screen grows. In the zoom list (8), **Fit** picks the largest whole zoom that fits, **F11** goes full screen, **⊞ Layout ▾ → Default** (4) restores everything; the same menu also offers a **Game** layout (Apple-1 screen and library). If the buttons are too small or too large for your screen, change **⚙ Preferences → Interface → Icon and text size**.

!!! note "Network access at first launch"
    The first time it starts, the emulator contacts GitHub (`api.github.com`, `raw.githubusercontent.com`) to fetch the demo project (`dschmenk/Apple1-Slideshow`, listed under “GitHub projects” in **⚙ Preferences**), and shows its progress in the status bar. Nothing else is sent. Offline, it simply skips it; remove the project from that list to stop the download for good.

## The Woz Monitor

The backslash `\` is the prompt of the **Woz Monitor**, the 256-byte program that starts the Apple-1. Click the screen and type: lower case becomes upper case, **Enter** is RETURN, **Backspace** gives the `_` that rubs out the last character, as in 1976.

The monitor understands four kinds of commands:

| You type | Effect |
|---|---|
| `300` | shows the byte at address `$0300` |
| `300.30F` | shows the bytes from `$0300` to `$030F` |
| `300: A9 C1` | writes `A9` at `$0300`, then `C1` at `$0301` |
| `300R` | runs the program that starts at `$0300` (*Run*) |

## Your first machine-language program

Type these two lines, each followed by Enter:

```mon
» 300: A9 C1 20 EF FF 4C 1F FF

0300: 00
» 300R

0300: A9A
```

The final `A` was printed by your program. The eight bytes are three 6502 instructions:

| Bytes | Instruction | Purpose |
|---|---|---|
| `A9 C1` | `LDA #$C1` | loads the code of the letter “A” (with bit 7 set, as the Apple-1 expects) |
| `20 EF FF` | `JSR $FFEF` | calls ECHO, the monitor routine that prints one character |
| `4C 1F FF` | `JMP $FF1F` | returns to the monitor, which waits for the next command |

The monitor first shows the byte at the start address (`0300: A9`), then your program prints the `A`. The built-in assembler (Complete Manual) saves you from working out these bytes by hand.

!!! apple1 "On the real Apple-1"
    The commands are identical: the emulator runs the real Woz Monitor ROM. The real machine displays about 60 characters per second. The emulator keeps that pace when the display row of the screen is set to **Normal**; **Fast** shows everything at once. The setting is shared by the Apple-1 screen and the GEN2 screen.
