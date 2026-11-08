# What next? {: .newpage }

You now know how to load a tape, write and run a BASIC program, and transfer it to a real machine. The **Complete Manual** (`docs/Complete-Manual.pdf`) goes much further:

- **Integer BASIC guide**: every statement with examples, the limits, the error messages, the known bugs and their workarounds, adapting Applesoft programs.
- **6502 assembler guide**: the built-in assembler (ca65 syntax, macros, listing), the Woz Monitor routines, examples tested in the emulator.
- **Games to play and read**: the library comes with five small games by Claude IA (Hangman, Matches, Battleship in BASIC; Tunnel and Tic-tac-toe in assembly), each in French and English.
- **The tools**: debugger, library and GitHub projects, GEN2 color card and P-LAB graphics card, photo → ASCII, vintage screen, command line.

## Essential shortcuts

| Key | Effect |
|---|---|
| Ctrl+R | RESET (back to the monitor) |
| Ctrl+L | CLEAR SCREEN |
| F3 | real speed or full speed |
| Ctrl+V | types the clipboard on the Apple-1 keyboard (also: right-click the screen → Paste) |
| Ctrl+plus, Ctrl+minus, Ctrl+0 | screen zoom, and zoom fitted to the window (in the editor: text size) |
| F11 | full screen |
| F1 | detailed tooltips (like the ? button) |
| Backspace | `_`, the monitor’s rub-out |
| Ctrl+F, Ctrl+H, Ctrl+G | in the editor: find, replace, go to a line |
| F5, F9 | in the editor: run the tab, breakpoint on the line |

## Commands to remember

| Command | Effect |
|---|---|
| `E000R` | starts Integer BASIC (cold: erases the program) |
| `E2B3R` | returns to BASIC without erasing the program |
| `C100R` | starts the ACI program (prompt `*`) |
| `start.endR` | reads the tape into that range, for example `0300.031ER` |
| `300R` | runs a machine-language program at `$0300` |
