# Your first BASIC program

Integer BASIC, Steve Wozniak’s BASIC, fits in 4 KB, and the emulator’s own **RDK G2 Basic**, which runs the same programs, is built in. It only knows integers (−32767 to 32767), but it is fast and pleasant. The built-in editor spares you typing on the Apple-1 screen, where a typo means retyping the whole line.

## Writing the program

Click **📝 Editor** in the toolbar, then the **New** icon (the first one) of the editor and choose “BASIC program”, and type this program (also in `docs/exemples/en/basic/premier.bas`).

```basic title="premier.bas"
10 REM MY FIRST PROGRAM
20 PRINT "HELLO, APPLE-1!"
30 PRINT
40 FOR I=1 TO 8
50 PRINT I;" SQUARED = ";I*I
60 NEXT I
70 PRINT
80 DIM N$(20)
90 INPUT "YOUR FIRST NAME",N$
100 PRINT "NICE TO MEET YOU, ";N$;"!"
110 END
```

Keywords, numbers, strings and comments each get their own color (the colors can be changed in **⚙ Preferences → Editor**); below the text, the editor shows the program size and the memory left for variables.

!!! astuce "Right-click in the text"
    The editor’s menu holds cut, copy and paste, **Find** (Ctrl+F) and **Replace** (Ctrl+H), go to a line (Ctrl+G), REM / un-REM a block, **Renumber…** (`GOTO`, `GOSUB` and `THEN` follow), a breakpoint on the line (F9) and **Send and run** (F5).

## Running it

Click **▶ Run**. The program goes straight into memory (after BASIC, if needed), then the editor types `E2B3R` and `RUN` for you:

![The editor on the left, the running program on the right.](img/en/brut/editeur-premier.png "large")

The program asks for your first name. Click the Apple-1 screen, type it, then Enter:

```mon
YOUR FIRST NAME?WOZ
NICE TO MEET YOU, WOZ!
```

!!! attention "Three quirks of Integer BASIC"
    - **Strings must be dimensioned**: without `DIM N$(20)`, the `INPUT` of a string stops with `*** STR OVFL ERR`.
    - **Variable names** are one letter, optionally followed by a digit: `A`, `B2`, `N$`. `TOTAL` is a syntax error.
    - **The final `END` is mandatory**: without it, the program ends with `*** END ERR`.

## When there is an error

The editor checks each line as you type, with the rules of the real BASIC. A faulty line turns red and the message explains the problem, with the line number and column:

![Two errors reported: an unterminated string and a variable name that is too long.](img/en/editeur-erreur.png "medium")

A program with errors does not run: fix the red lines first. Run-time errors appear on the Apple-1 screen: `*** BAD BRANCH ERR` for a `GOTO` to a missing line, `*** >32767 ERR` for an overflow or a division by zero. The **Debugger** (Complete Manual) then steps line by line while watching the variables.

![The work cycle: write, run, fix, then export to the real machine.](diagrams/en/workflow-basic.svg "diagram")

!!! astuce "Stopping a program"
    During a `RUN`, a key press stops the program with `STOPPED AT 50` (the current line). If it is stuck, **Reset** (Ctrl+R) returns to the monitor and `E2B3R` re-enters BASIC **without erasing** the program.
