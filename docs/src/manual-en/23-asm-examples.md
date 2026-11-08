# Annotated examples

<p class="lead">Eight programs, from the simplest to the most complete. They are in the <code>docs/exemples/en/asm/</code> folder of the distribution; each was assembled by the built-in assembler and by ca65 (same bytes), then run in the emulator. The outputs shown here are the emulator’s.</p>

To try them: open the file in the editor and click **▶ Run**, or, on the command line, `apple1-emu asm hello.s -o hello.wav` then play the tape (chapter 5). Keep `apple1.inc` (and `gen2.inc`) in the same folder as the sources.

| File | Bytes | Address | What it shows |
|---|---|---|---|
| `hello.s` | 33 | `$0300` | a string, `ECHO`, local labels, back to the monitor |
| `clavier.s` | 32 | `$0300` | reading the keyboard, comparing characters |
| `hexa.s` | 64 | `$0300` | zero-page pointer, `(zp),Y`, 16-bit addition, `PRBYTE` |
| `decimal.s` | 117 | `$0300` | 16-bit numbers, subtractions, word tables |
| `jeu.s` | 222 | `$0300` | a complete game: randomness, subroutines, messages |
| `macros.s` | 135 | `$0300` | macros, `.local`, `.repeat`, `.if`, computed tables |
| `gen2-lores.s` | 110 | `$0C00` | the GEN2 card in LORES |
| `gen2-hires.s` | 94 | `$0C00` | the GEN2 card in HIRES |

## `hello.s`: printing a message

@@include exemples/en/asm/hello.s

X walks along the string up to the 0 byte placed by `.asciiz`. The `bne @next` after `inx` is never false for a string shorter than 256 characters: it is a short, safe jump. The program ends with `jmp GETLINE`, never with `RTS` (chapter 22).

```mon
\
» 300R
0300: A2
HELLO, APPLE-1!
```

The monitor first prints `0300: A2`, the byte at the start address, then the program takes over.

## `clavier.s`: echoing keys

@@include exemples/en/asm/clavier.s

`cmp #'0' | $80` compares with the code of the digit 0, with bit 7 set as the keyboard does. `bcc` (“lower”) and `bcs` (“higher or equal”) bracket the digits. With the keys `A`, `B`, `1`, `2` then ESC, the screen shows:

```text
0300: ADAB1122
\
```

The `AD` comes from the monitor (`lda KBDCR` starts with opcode `$AD`); `AB` then `11` and `22` come from the program; ESC returns control to the monitor, which prints `\`.

## `hexa.s`: a monitor-style dump

@@include exemples/en/asm/hexa.s

`PTR` and `PTR+1` form a **16-bit pointer** in zero page, low byte first; `lda (PTR),y` reads the byte at address PTR + Y. The addition `PTR += 8` shows the carry at work: `clc`, `adc #8`, and if a carry comes out (`bcc` not taken), `inc PTR+1`. The result is the beginning of the Woz Monitor:

```text
FF00: D8 58 A0 7F 8C 12 D0 A9
FF08: A7 8D 11 D0 8D 13 D0 C9
```

## `decimal.s`: printing a number in decimal

@@include exemples/en/asm/decimal.s

The 6502 cannot divide: `prdec` subtracts 10000 as many times as possible (the count gives the ten-thousands digit), then 1000, 100 and 10. The 16-bit subtraction is done in two steps, `sbc` on the low byte then on the high byte, with the carry linking them. `FLAG` suppresses leading zeros. Output:

```text
0 7 42 1976 6502 65535
```

!!! astuce "Preserving X in a subroutine"
    `prdec` uses X for itself, while the main loop needs it: it saves it with `txa` / `pha` at the start and restores it with `pla` / `tax` at the end. The NMOS 6502 has no `PHX`.

## `jeu.s`: guess the digit

@@include exemples/en/asm/jeu.s

The randomness comes from the player: X counts non-stop (`@wait`) until the first key, then the remainder of the division by 10 gives the secret digit. `prmsg` receives the address of a message in **Y:X** (high byte in Y), stores it in the pointer `MSG`, and prints it with `(MSG),y`. One game, typing `x` then the digits in order:

```text
I PICKED A DIGIT. PRESS ANY KEY.
YOUR DIGIT? 0 HIGHER.
YOUR DIGIT? 1 HIGHER.
…
YOUR DIGIT? 7 HIGHER.
YOUR DIGIT? 8 WON IN 9 TRIES!
```

## `macros.s`: letting the assembler do the work

@@include exemples/en/asm/macros.s

`putc` and `print` generate code at each call; `.local` prevents the two calls of `print` from defining `loop` twice. `.repeat 3` unrolls the line of stars three times (each with its own anonymous label), `.if` chooses the message according to `WIDTH`, and the `squares` table is computed at assembly time. Output:

```text
MACROS AND .REPEAT
********************
********************
********************
SHORT LINES
```

Change `WIDTH = 20` to `40`: the editor reassembles at once, the lines get longer and the message becomes `LONG LINES`.

## `gen2-lores.s`: low-resolution color

These two programs need the GEN2 card (**⚙ Preferences ▸ Uncle Bernie GEN2 colour card**, chapter 6) and start at `$0C00`, above the LORES page.

@@include exemples/en/asm/gen2-lores.s

The two tables `rows_lo` and `rows_hi` are computed by `.repeat` with the row-address formula from chapter 6. Each byte carries two blocks, hence the nibble copied to the top and bottom halves. The `bit SS_…` instructions are **reads**: that is how the card’s modes are switched.

![gen2-lores.s: the sixteen LORES colors, two columns each, on the color monitor.](img/en/brut/gen2-lores.png "large")

## `gen2-hires.s`: high resolution

@@include exemples/en/asm/gen2-hires.s

In HIRES, the `$2000-$3FFF` page is divided into 128-byte blocks whose first 40 bytes go to the top third of the screen, the next 40 to the middle and the 40 after that to the bottom. The program fills each third with a different pattern, alternating two bytes according to the parity of the column: the result is three color bands.

![gen2-hires.s: three color bands in HIRES: violet, green and blue.](img/en/brut/gen2-hires.png "large")
