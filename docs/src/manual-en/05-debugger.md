# The debugger, the disassembly and the dump

<p class="lead">The debugger stops the Apple-1 and steps through it, one instruction at a time in 6502 or one line at a time in BASIC, showing the registers, the variables and memory.</p>

## The debugger panel

**🐛 Debugger** opens the panel, docked on the right or detachable like the editor. Its commands:

| Command | Effect |
|---|---|
| **⏸ Pause** / **▶ Continue** | stops or resumes the Apple-1 |
| **⬇ Step** | executes one instruction |
| **↪ Over** | executes a `JSR` in one go, and stops on return |
| **⤴ Out** | runs until the `RTS` of the current subroutine |
| **⏭ BASIC line** | advances by one line of the BASIC program |
| **⏩ To cursor** | (in the editor) runs to the cursor line |

{{!Step (debugger)}}{{!Over}}{{!Out}}

The **Processor** frame shows A, X, Y, S, PC and the flags N V B D I Z C; all of them can be changed by clicking the value. The number of cycles and instructions executed is shown below.

## Breakpoints

A breakpoint can be set in four ways:

- **⏺ Breakpoint** in the editor (or <kbd>F9</kbd>, or the right-click menu), on the cursor line: in a `.s` source (thanks to the assembler’s address table) or in a BASIC program (line number);
- a click in the margin of the **📃 Listing**;
- a click on a line of the disassembly;
- typing an address or a label: `$0368`, `ASK`, `PUTS+3`, or `L190` for BASIC line 190. A label that does not exist yet is not refused: the breakpoint is kept and is set as soon as the label exists (when you assemble the editor’s source, or run it), with a message that says so.

Breakpoints, watched variables and memory watches are remembered from one session to the next, but loading another program clears the breakpoints and labels of the previous one. When the built-in BASIC is loaded the BASIC section of the debugger is titled **RDK G2 Basic**; without any BASIC at `$E000` it says so neutrally. The colours of the current line, the dump addresses and the ASCII column can be changed in **⚙ Preferences → Interface** (“Interface colours”). Narrow panels drop the bytes column of the disassembly and wrap the poke row instead of cutting them.

## Debugging a BASIC program

![boucle.bas stopped at a breakpoint on line 30: the current line in green in the editor, the variables I and S watched in the debugger.](img/en/brut/debogueur-basic.png "large")

The current line is highlighted in green in the editor, breakpoints in purple. The BASIC frame (**RDK G2 Basic** or **Integer BASIC**, depending on the one loaded) shows the current line, the breakpoints on line numbers and the **watched variables**: integers (`A`, `B1`), array elements (`D(2)`), whole arrays (`D`), strings (`C$`), plus **All variables**.

Debugging works with **RDK G2 Basic**, the BASIC built into the emulator, as well as with the original: the real Integer BASIC is no longer needed. The debugger notices the start of every line (the current line, the line breakpoints, **⏭ BASIC line**) and reads the variables from the interpreter’s tables. For the original it reads the line from the zero-page pointers `$DC`/`$DD`; G2 Basic keeps it elsewhere, and the debugger knows both layouts.

!!! attention "A key press stops the program"
    As on the real machine, a key pressed while the BASIC program runs stops it (`STOPPED AT`). During a debugging session, keep the keyboard focus in the debugger panel, not on the Apple-1 screen. A G2 Basic program can read the keyboard without being stopped with `KEY(0)` (chapter 15).

## Debugging a 6502 program

![decimal.s stopped at the label prdec: registers, disassembly with the program’s labels, memory watch $06.$08.](img/en/brut/debogueur-asm.png "large")

The **disassembly** shows 24 lines around PC, with the labels of the assembled program in their own column. It only re-centers when PC leaves the visible area, so the display does not jump at every step. **Go** pins the list on an address or a label; **Follow PC** makes it follow execution again.

When you select an instruction in the editor’s 6502 source, its bytes are also **highlighted** in the disassembly and in the dump (chapter 7), and both panels scroll to them if they are out of view; the disassembly then stops following PC until you press **Follow PC**.

The **memory watch** shows a byte, a word or ASCII (`$30`, `$30 w`, `0200.020F`, a label), editable, in yellow when the value changes.

## The dump

The **Dump** shows memory in hexadecimal and ASCII, 16 bytes per line (8 if the panel is narrow), with bit 7 stripped for the ASCII. **Go** jumps to an address, or to the shortcuts ZP, `$0200`, `$0300`, PC, `$E000`. Changed bytes are in yellow, the byte at PC on a green background. **Write** stores bytes (`0300 = A9 00`); clicking a byte fills in the write line.

The disassembly and the dump are panels in their own right: detached by default, they can be docked next to the debugger (chapter 10).

## On the command line

For demonstrations and automated tests: `--debogueur` opens the panel, `--dbg-arret ASK,L190` sets breakpoints, `--dbg-surveiller 'A,C$,$30 w'` watches variables and memory, and `--dbg-commande 5:pas` sends a command after 5 seconds (`pause`, `continuer`, `pas`, `dessus`, `sortir`, `basic`). These option names and keywords are French.
