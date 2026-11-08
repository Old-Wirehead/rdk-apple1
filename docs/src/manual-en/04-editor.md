# The editor

<p class="lead">The built-in editor writes Integer BASIC programs and 6502 sources, checks them at every keystroke, runs them in the emulated Apple-1 in one click, and exports them as tapes for the real machine.</p>

## Tabs and files

**📝 Editor** opens the panel, docked to the left of the screen. Its toolbar is compact, to leave as many lines as possible to the text. It starts with icons, explained by their tooltips:

| Icon | Action |
|---|---|
| **New** (first icon) | New tab: “BASIC program” or “6502 assembler source” |
| **📂** / **💾** / **💾…** | Open (`.bas`, `.txt`, `.s`, `.asm`, `.a65`, `.inc`) / Save (<kbd>Ctrl</kbd>+<kbd>S</kbd>) / Save as… |
| **✂**, copy, **📋** | Cut, copy, paste |
| **🔍** | Find and replace (<kbd>Ctrl</kbd>+<kbd>F</kbd>, <kbd>Ctrl</kbd>+<kbd>H</kbd>) |
| **⟲ ⟳** | Undo (<kbd>Ctrl</kbd>+<kbd>Z</kbd>), redo (<kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd>) |
| **A▲▼** | Text size: one compact control, the letter A with two small stacked triangles; ▲ enlarges, ▼ reduces |

After them, on the same row (or the next one when the panel is narrow), come the actions of the tab: **▶ Run**, **📼 Export WAV**, and **Memory** for BASIC, or **📃 Listing**, **⏺ Breakpoint** and **⏩ To cursor** for a 6502 source.

Each file is a tab. A dot • marks a modified tab; × closes it (a second click is required if it is not saved). A tab marked **📚** was opened from the library: opening a listing or a source of the library (▶, ⬇ or a double-click) opens its tab when the editor is visible, or shows it if it is already open. Tabs, unsaved text included, come back at the next launch. A right-click on a tab offers **Save as…** and **Close**. In the text, a **middle click** pastes the primary selection (Linux): the cursor goes where you click, then the text last highlighted in any application is inserted.

The text size is set with the **A▲▼** control, <kbd>Ctrl</kbd>+wheel, or <kbd>Ctrl</kbd>+<kbd>+</kbd> / <kbd>Ctrl</kbd>+<kbd>−</kbd> / <kbd>Ctrl</kbd>+<kbd>0</kbd> when the text has the keyboard focus.

**Detach** puts the editor in its own window, for example on a second screen; **⊟ Dock** puts it back. Keys typed in that window go to the editor, never to the Apple-1.

## Find, replace and the right-click menu

**🔍** (or <kbd>Ctrl</kbd>+<kbd>F</kbd>) opens a search bar above the text: <kbd>Enter</kbd> goes to the next match, <kbd>Shift</kbd>+<kbd>Enter</kbd> to the previous one, **Aa** matches the case, **Word** finds whole words, and the counter shows “3 of 7”. <kbd>Ctrl</kbd>+<kbd>H</kbd> adds the **Replace with** field, with **Replace** (the current match) and **Replace all**. <kbd>Esc</kbd> closes the bar.

![The search bar open on ECHO, and the right-click menu of a 6502 source.](img/en/editeur-menu.png "medium")

**Right-clicking** the text opens a menu with every command and its shortcut. The menu acts on the **selection**, which is kept when you right-click on it: **Cut** and **Copy** work on the selected text. The word-based commands (go to the definition, help) use the **word under the click**, not the one under the cursor.



| Command | Shortcut |
|---|---|
| Cut, Copy, Paste, Select all | <kbd>Ctrl</kbd>+<kbd>X</kbd>, <kbd>C</kbd>, <kbd>V</kbd>, <kbd>A</kbd> |
| Find, Replace | <kbd>Ctrl</kbd>+<kbd>F</kbd>, <kbd>Ctrl</kbd>+<kbd>H</kbd> |
| Go to line… | <kbd>Ctrl</kbd>+<kbd>G</kbd> |
| Comment / uncomment (6502), REM / un-REM (BASIC) | <kbd>Ctrl</kbd>+<kbd>/</kbd> |
| Indent, Unindent | |
| Go to the definition of a label (6502) or of a line number (BASIC) | <kbd>F12</kbd> |
| Help on *word* (on a keyword, an instruction or a directive: opens its entry in the reference) | |
| Show in the disassembly (6502) | |
| Renumber… (BASIC: `GOTO`, `GOSUB` and `THEN` follow) | |
| Breakpoint on this line | <kbd>F9</kbd> |
| Assemble and run (6502), Send and run (BASIC) | <kbd>F5</kbd> |

**Colors.** The colors of the text follow the interface theme. **⚙ Preferences → Editor** offers, for BASIC and for the 6502 assembler separately, the schemes *Automatic*, *Classic (dark)*, *Light*, *Amber*, *High contrast*, or your own colors, one swatch per kind of word (chapter 11).

## A BASIC program

![The BASIC editor: errors shown on a red background, with their line number and column at the bottom of the panel.](img/en/editeur-erreur.png "large")

The text is colored: line numbers, keywords, numbers, strings, `REM`, operators. Each line is **tokenized** as Integer BASIC would do it, by a native tokenizer checked byte for byte against the real one. A line that BASIC would refuse turns red, and the error gives the text line, the BASIC line number and the column; a click puts the cursor there. Errors detected: syntax, invalid variable name, number greater than 32767, direct-mode command (`LIST`, `RUN`…) inside the program, unterminated string, line longer than 255 bytes, memory full.

- **Memory**: automatic (8 KB with LOMEM `$0800`, or `$0300` if the program is large, otherwise 12, 16 or 32 KB), or forced.
- {{▶ Run}}: loads Integer BASIC if it is missing (like the BASIC button), puts the tokenized program into memory, adds RAM if needed, then types `E2B3R` and `RUN`. It is instant: nothing is typed line by line.
- {{📼 Export WAV}}: a tape (`.wav`, or `.aiff` for a smartphone) of two records, with its settings in the library. The bottom of the panel shows what to type on the real Apple-1, for example `C100R` then `004A.00FFR 0C3B.0FFFR`, then `E2B3R` and `RUN`; **📋 Copy** puts it on the clipboard.

![From the editor to the real Apple-1: write, run, fix, export.](diagrams/en/workflow-basic.svg "diagram large")

## A 6502 source

![A 6502 source in the editor: the colors, the assembly result and the line to type on the real Apple-1.](img/en/brut/editeur-asm.png "large")

A source is assembled **at every change** by the built-in two-pass assembler, which accepts ca65 syntax (and xa syntax, translated automatically). The bottom of the panel shows the size and the addresses (`33 bytes, $0300-$0320 — 2 file(s), 13 symbols`), then the line for the real machine (`C100R` then `0300.0320R`, then `300R`).

- **Errors**: file, line and column; a click takes you there, opening the included file if necessary. Warnings (zero page encoded as absolute) are in yellow.
- **📃 Listing**: address, bytes and source of each line, with included files and expanded macros, then the symbol table; a click takes you back to the line.
- **▶ Run**: puts the bytes into memory at the origin (`.org`, `$0300` by default), adds RAM if needed, then types for example `300R`.
- **📼 Export WAV**: the tape, as for BASIC.
- **⏺ Breakpoint** and **⏩ To cursor**: the link with the debugger (chapter 8).
- **Link with the debugger**: when you select an instruction in the source (or place the cursor on its line), the bytes it produced are highlighted in the debugger’s **disassembly** and in the **hex / ASCII dump**, when those panels are visible; if they are elsewhere in memory, the panels scroll to show them. The assembler’s table of source lines and addresses makes this possible.

![An assembly error: ECO instead of ECHO, an unknown symbol, reported with its line and column.](img/en/brut/editeur-asm-erreur.png "large")

Part III is entirely devoted to the assembler: syntax, directives, listing, examples.
