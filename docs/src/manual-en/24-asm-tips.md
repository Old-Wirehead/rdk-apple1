# Tips, traps and limits

<p class="lead">What you learn writing programs for the Apple-1, first in the emulator and then on the real machine.</p>

## Where to put a program

| Address | For |
|---|---|
| `$0280` | as low as possible: just after the monitor’s buffer |
| `$0300` | the usual choice (and the assembler’s default address) |
| `$0C00` | with the GEN2 card in LORES or text (pages `$0400-$0BFF` left free for the display) |
| below HIMEM | with Integer BASIC: reserve the space (chapter 16) |

For zero page, the examples use `$06-$09`, which the monitor leaves free. If the program must coexist with BASIC, choose addresses BASIC does not use, or save and restore them.

## Living within the branch range

A branch reaches only 127 bytes forward or 128 back. When the assembler answers `branch too far`, invert the condition and jump over a `JMP`:

```asm
        beq :+          ; instead of: bne far
        jmp far
:
```

## Strings and bit 7

The keyboard returns codes with bit 7 set, and the monitor compares against those codes. The display, on the other hand, ignores bit 7: `.asciiz "HELLO"` displays perfectly well. To **compare** a key, put bit 7 in the constant: `cmp #'Q' | $80`. Lower-case letters in a string are displayed as upper case.

## Waiting a little

At 1.023 MHz (0.96 MHz effective), a `dex` / `bne` loop costs 5 cycles per iteration:

```asm
delay:
        ldy #78         ; 78 x 256 iterations of 5 cycles: about 0.1 s
:       ldx #0
:       dex
        bne :-
        dey
        bne :--
        rts
```

The emulator at real speed respects these durations; at fast speed (<kbd>F3</kbd>), they vanish. The ACI’s timing loops are counted in cycles in the same way (Appendix C).

## Debugging efficiently

- Set breakpoints on a **label** (`start`, `prdec`, `start@done`) rather than on an address: it stays correct when the code moves.
- Step over `jsr ECHO` with **↪ Over**: otherwise you follow the display’s wait loop, instruction by instruction.
- If the Apple-1 stops responding after your program, look at PC: in zero page, it is almost always a stray `BRK` or `RTS` (chapter 22).
- The **dump** shows zero page: your pointers are visible there, low byte first.

![The debugger stopped in a 6502 source: registers, flags, disassembly with labels, and memory.](img/en/brut/debogueur-asm.png "large")

## On to the real Apple-1

<ol class="steps">
<li>Assemble and try it in the emulator, <strong>at real speed</strong> and with the display limited to 60 characters per second.</li>
<li>Export the tape (<strong>📼 Export WAV</strong>, or <code>-o prog.aif</code>): AIFF is the safest format when playing from a phone.</li>
<li>On the Apple-1: <code>C100R</code>, then the range shown (<code>0300.0320R</code>), start playback, then <code>300R</code>.</li>
<li>Without an ACI: the <code>.hex</code> dump can be typed (or pasted into a serial terminal) straight into the Woz Monitor; its last line runs the program.</li>
</ol>

## Limits of the built-in assembler

- A single output block, in source order: segments have no effect, and there is no linker. For code spread over several areas, use ca65 and ld65.
- No modulo operator, no `.proc`, `.scope`, `.struct` or `.enum`: these are ca65 directives the built-in assembler does not know.
- 65C02 instructions are assembled, but the emulated processor is an NMOS 6502: they do not work in the emulator (chapter 20).
- In the **listing**, a line produced by a macro sometimes loses the space between the instruction and the operand (`ldastring, x` for `lda string,x`). This is a display defect of the listing only: the bytes are correct.
