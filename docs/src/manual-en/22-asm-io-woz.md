# The keyboard, the display and the Woz Monitor

<p class="lead">The Apple-1 has only four input/output registers, in a 6820 (or 6821) PIA, and a 256-byte monitor. That is not much, but it is enough: this chapter shows how to read a key, write a character and use Steve Wozniak’s routines.</p>

## The four PIA registers

| Address | Name | Read | Write |
|---|---|---|---|
| `$D010` | `KBD` | key code, bit 7 set; reading **acknowledges** the key | — |
| `$D011` | `KBDCR` | bit 7 = 1: a key is waiting | configuration (done by the monitor) |
| `$D012` | `DSP` | bit 7 = 1: the display is busy | character to display (7 bits) |
| `$D013` | `DSPCR` | — | configuration (done by the monitor) |

{{!KBD}}{{!KBDCR}}{{!DSP}}

These names are defined in `apple1.inc`, included by all the examples:

@@include exemples/en/asm/apple1.inc

## Reading a key

Wait for bit 7 of `KBDCR` to become 1, then read `KBD`:

```asm
read:   lda KBDCR       ; a key?
        bpl read        ; bit 7 = 0: not yet
        lda KBD         ; ASCII code + $80
```

The keyboard produces only upper case (the emulator converts lower case), with bit 7 set: `A` gives `$C1`, RETURN `$8D`, ESC `$9B`. To find out whether a key has been pressed **without waiting**, test `KBDCR` alone:

```asm
        lda KBDCR
        bpl nokey       ; no key: carry on working
        lda KBD         ; a key: read it (and acknowledge it)
```

`jeu.s` (chapter 23) uses this to pick a random number: it counts in a loop until the player presses a key.

## Writing a character

The Apple-1’s terminal takes one character at a time, and only when it is ready: bit 7 of `DSP` stays at 1 until the previous character has been displayed. The monitor’s `ECHO` routine does the waiting for you. Here are its four instructions, read in the emulator’s monitor:

```mon
\
» FFEF.FFFFR

FFEF: 2C
FFF0: 12 D0 30 FB 8D 12 D0 60
FFF8: 00 00 00 0F 00 FF 00 00
```

That is `BIT DSP` / `BMI ECHO` / `STA DSP` / `RTS`. `ECHO` preserves A, X and Y. Since the original terminal displays only **one character per frame**, about 60 per second, a program that prints a lot is limited by the display, not by the processor; the emulator respects this limit by default (<kbd>F4</kbd> lifts it, chapter 2).

What the display does with each code:

- **RETURN** (`$8D`, or `$0D`) starts a new line; at the bottom of the screen, everything scrolls up one line.
- The **other control characters** (`$00-$1F`, with or without bit 7) are ignored: no clear, no backspace, no bell.
- **Lower case** `$60-$7F` is displayed like upper case `$40-$5F`: the terminal keeps only 6 bits.
- After the 40th column, the cursor moves to the next line by itself.
- There is no code to clear the screen: it is a button on the machine (**Clear screen**, <kbd>Ctrl</kbd>+<kbd>L</kbd>).

## The Woz Monitor routines

| Address | Name | Input | Effect | Registers |
|---|---|---|---|---|
| `$FFEF` | `ECHO` | A = character | displays it | A, X, Y preserved |
| `$FFDC` | `PRBYTE` | A = byte | displays it as 2 hex digits | A destroyed; X, Y preserved |
| `$FFE5` | `PRHEX` | A (low nibble) | displays one hex digit | A destroyed; X, Y preserved |
| `$FF1F` | `GETLINE` | — | back to the monitor: new line, then waits for a command | — |
| `$FF1A` | `ESCAPE` | — | like `GETLINE`, printing `\` first | — |
| `$FF00` | `RESET` | — | initializes the PIA and the monitor | — |

{{!ECHO}}{{!PRBYTE}}{{!PRHEX}}{{!GETLINE}}

The monitor uses zero page `$24-$2B` and the line buffer `$0200-$027F`; everything else is yours (unless Integer BASIC must survive, chapter 16). Its commands (`300R`, `0300.031F`, `300: A9 8D`…) are summarized in Appendix B.

## Starting and ending a program

A program is started by the monitor’s `R` command (`300R`), which does a `JMP` to the address, not a `JSR`. There is therefore **no return address** on the stack: end with `jmp GETLINE` (`$FF1F`), not with `RTS`.

!!! attention "RTS and BRK do not return control"
    An `RTS` at the end of a program started by `R` pops whatever is on the stack. A `BRK` jumps through the ROM’s interrupt vector, which is `$0000` (the last three words of the monitor, above: NMI `$0F00`, RESET `$FF00`, IRQ/BRK `$0000`): the processor then executes zero page. In both cases, press RESET (<kbd>Ctrl</kbd>+<kbd>R</kbd>).

A program called from BASIC with `CALL 768`, on the other hand, is called by a `JSR`: it must end with `RTS` to return to BASIC. Chapter 16 shows how to fit machine code below the BASIC program.

```mon
>» POKE 768,96
>» CALL 768
>
```

(`96` = `$60`, the `RTS` opcode: the smallest possible subroutine.)

## The PIA, the tape and the color card

The ACI card occupies `$C000-$C0FF` (every access toggles the output) and its program `$C100-$C1FF`; chapter 5 explains how it works and Appendix C its timings. The GEN2 card is controlled by simply **reading** `$C250-$C257` (`bit SS_GR`); its addresses are in `gen2.inc`:

@@include exemples/en/asm/gen2.inc
