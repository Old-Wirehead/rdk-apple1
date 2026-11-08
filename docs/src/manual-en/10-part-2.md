# Integer BASIC <span class="part-sub">Steve Wozniak’s BASIC and its free clone RDK G2 Basic: every statement, memory, the error messages, the known bugs and their workarounds, and porting from Applesoft</span> {: .part data-part="Part II" }

# First steps in BASIC

<p class="lead">The Apple-1’s Integer BASIC fits in 4 KB. It knows only integers, one-letter variable names, and has neither DATA nor string functions: it is small, fast, and has its traps.</p>

!!! apple1 "Where this BASIC comes from"
    Steve Wozniak wrote it in 1976, by hand, in assembly language, without a computer to assemble it. It was sold on tape ($75) and loads at `$E000-$EFFF`. The emulator does not supply Apple’s original: use your own `BASIC.wav` or 4 KB image (chapter 4) if you want it. Every test in this part was done in the emulator, with this original.

## Two BASICs: RDK G2 Basic and the original

Since version 7.9 the emulator has its own BASIC, **RDK G2 Basic**, built into the program. It is a clean-room clone of Integer BASIC, written from scratch in 6502 assembly (it contains no code of Apple’s interpreter) and distributed under the MIT licence. It is dedicated to Uncle Bernie.

- **Compatible**: same syntax, same tokens, same program format in memory, same tape format. Programs, listings and tapes written for the original run unchanged. It was checked against the original with 22 test sessions whose output is identical.
- **Fixed**: the known bugs of the original, described in chapter 18, are corrected.
- **Bigger**: 7,933 bytes instead of 4 KB. It needs RAM from `$E000` to `$FEFF`; the emulator provides it, but it would not fit in the 4 KB EPROM window of the original.
- **GEN2-aware**: at start it looks for the GEN2 card and, when it finds one, starts the card driver (and then puts LOMEM at `$2000`, above the graphics pages). The graphics statements themselves are planned for version 2 of G2 Basic.
- **Banner**: `RDK G2 BASIC 1.0`, then `DEDICATED TO UNCLE BERNIE`.
- **Extended** (version 7.9.2): `LOMEM=` and `HIMEM=` work as in the original, and a new function **`KEY(n)`** reads the keyboard without waiting, so that a game can look at the keys without `INPUT` blocking it and without a key press stopping the program (chapter 15). `LIST a,b` and `DEL a,b` use the original’s ranges, and a listed line longer than the screen wraps as the original does.

The **BASIC** button loads G2 Basic. To use Apple’s original instead, open **⚙ Preferences → Advanced → BASIC**, choose **Original Integer BASIC (your own file, loaded at $E000)** and pick the file; the choice is remembered, and the button then loads it. The two behave alike for everything described in this part; the differences are noted where they matter (HIMEM at start, the bugs of chapter 18).

!!! note "Source and tests"
    G2 Basic has its own repository, `Old-Wirehead/rdk-gen2basic-apple1`, with its manual, its source and its regression tests.

## Starting BASIC

Click **BASIC** in the toolbar: the emulator loads the BASIC you chose (G2 Basic by default) and you start it with `E000R`. The BASIC prompt is `>`, the Woz Monitor’s is `\`.

```mon
\
» E000R
E000: 4C
>
```

- `E000R`: **cold start**, erases the program in memory.
- `E2B3R`: **warm start**, keeps the program. This is the address to use after loading a program from tape, and to return to BASIC after a trip through the monitor (RESET).

{{E000R}}{{E2B3R}}

## Direct mode and program mode

A line that begins with a **number** (0 to 32767) is stored in the program; a line without a number is executed at once.

```mon
>» PRINT 2*3+4*5
26
>» 10 PRINT "HELLO"
>» 20 GOTO 10
>» RUN
HELLO
HELLO
…
```

Press a key to stop the program: BASIC prints `STOPPED AT 10`. Retyping a line with the same number replaces it; a number alone deletes it.

!!! attention "Direct mode is limited"
    In direct mode, BASIC refuses statements separated by `:` (`SYNTAX ERR`), as well as `FOR` and `GOSUB`. To try a loop, write it in a numbered line.

## Correcting a line

The Apple-1 keyboard has no delete key: the emulator’s **Backspace** types `_`, which cancels the previous character (the line on screen is not corrected, but BASIC receives the right one). <kbd>Esc</kbd> cancels the whole line: BASIC prints `\` and waits for the next line. For more comfortable editing, use the editor (chapter 7): it colors the syntax, reports errors before sending, and types the program for you.

## A first program

@@include exemples/en/basic/boucle.bas

```mon
>» RUN
SUM = 55
>
```

In the editor, **▶ Run** types the program and `RUN`; in the debugger you can follow it line by line (figure in chapter 8).

## Commands

| Command | Effect |
|---|---|
| {{LIST}} `[n[,m]]` | lists the program, line `n`, or lines `n` to `m` |
| {{RUN}} `[n]` | clears the variables and runs the program, from line `n` if given |
| {{DEL}} `n[,m]` | deletes line `n`, or lines `n` to `m` |
| {{SCR}} | erases the program and the variables |
| {{CLR}} | clears the variables, keeps the program |
| {{AUTO}} `n[,step]` | automatic numbering from `n` (step 10 by default); <kbd>Ctrl</kbd>+<kbd>D</kbd> then <kbd>RETURN</kbd> leaves it, <kbd>Esc</kbd> does not |
| {{MAN}} | returns to manual numbering |
| {{HIMEM}}`=n`, {{LOMEM}}`=n` | the bounds of BASIC’s memory (chapter 16) |

`GOTO n` in direct mode resumes a stopped program **without** clearing the variables, unlike `RUN n`.

!!! astuce "LIST and the slow display"
    The Apple-1 terminal displays about 60 characters per second: listing a long program takes time. Turn off the display limit (**Fast** in the display row, or in the status bar) to read faster, or list in slices: `LIST 100,200`.

## Three programs to type in

### Guess a number

@@include exemples/en/basic/devine.bas

```mon
>» RUN
I PICKED A NUMBER FROM 1 TO 100.
YOUR GUESS?» 50
HIGHER!
YOUR GUESS?» 75
LOWER!
YOUR GUESS?» 62
LOWER!
YOUR GUESS?» 56
HIGHER!
YOUR GUESS?» 57
WELL DONE, IN 5 TRIES.
```

`RND(100)` gives an integer from 0 to 99; `#` means “not equal”.

### A multiplication table

@@include exemples/en/basic/table.bas

### Playing with strings

@@include exemples/en/basic/chaines.bas

```mon
>» RUN
YOUR NAME?» KAYAK
INITIAL: K
END: AK
BACKWARDS: KAYAK
IT'S A PALINDROME!
```

Strings are sliced with `N$(start,end)` and extended with `R$(LEN(R$)+1)=…`: there is neither `MID$` nor `+` for strings (chapter 16).

The four programs are in `docs/exemples/en/basic/`: open them in the editor, or make a tape with `apple1-emu basic devine.bas -o devine.aiff`.
