# The devkit assembler

<p class="lead">The emulator contains a complete two-pass 6502 assembler, compatible with ca65. It is used in the editor, which assembles on every keystroke, on the command line, and for GitHub projects. There is nothing to install.</p>

## The work cycle

![The assembly work cycle: write, assemble, run, debug, then export to the real Apple-1.](diagrams/en/workflow-asm.svg "diagram large")

<ol class="steps">
<li><strong>Write</strong>: <strong>📝 Editor</strong>, then <strong>New</strong> or <strong>Open</strong> a <code>.s</code>, <code>.asm</code> or <code>.a65</code> file. Syntax coloring distinguishes instructions, directives, labels, numbers and comments.</li>
<li><strong>Assemble</strong>: nothing to do, assembly follows every change. The bottom of the panel gives the size, the addresses and the line to type on the real machine; errors are shown with their line and column.</li>
<li><strong>Run</strong>: <strong>▶ Run</strong> puts the bytes in memory at the <code>.org</code> address and types the Woz Monitor command (<code>300R</code>).</li>
<li><strong>Debug</strong>: <strong>⏺ Breakpoint</strong> on the cursor line, then step through in the debugger (chapter 8), with your labels in the disassembly.</li>
<li><strong>Export</strong>: <strong>📼 Export WAV</strong> writes the tape; on the real Apple-1, type <code>C100R</code>, the range shown (<code>0300.0320R</code>), then <code>300R</code>.</li>
</ol>

![A source in the editor: on the left the colored text, at the bottom the assembly result; on the right the Apple-1 has run the program.](img/en/brut/editeur-asm.png "large")

## The listing and the symbols

**📃 Listing** shows, for each line, the address, the bytes produced and the source; lines produced by a macro are marked `+`, included files are expanded, and the symbol table follows. Clicking a line takes you back to the source; clicking in the margin sets a breakpoint. Here is the listing of `hello.s` (chapter 23), produced by the command line:

```text title="hello.lst"
0300  A2 00       start:  ldx #0
0302  BD 10 03    @next:  lda message,x   ; next character
0305  F0 06               beq @done       ; 0 = end of the string
0307  20 EF FF            jsr ECHO
030A  E8                  inx
030B  D0 F5               bne @next
030D  4C 1F FF    @done:  jmp GETLINE

                  message:
0310  8D                  .byte CR
0311  48 45 4C 4C         .asciiz "HELLO, APPLE-1!"
0315  4F 2C 20 41
…
; ---- Symboles (13)
; ECHO        $FFEF
; GETLINE     $FF1F
; message     $0310
; start       $0300
; start@done  $030D
; start@next  $0302
```

The local labels `@next` and `@done` belong to `start`: the table names them `start@next` and `start@done`, and the debugger accepts these names as breakpoint addresses. (The listing written by the command line keeps its French heading, *Symboles*.)

## The command line

{{asm (command line)|`apple1-emu asm`}} does the same without a window:

```sh
apple1-emu asm hello.s                    # check only
apple1-emu asm hello.s -l hello.lst       # with the listing
apple1-emu asm hello.s -o hello.wav       # tape (.wav or .aif)
apple1-emu asm hello.s -o hello.bin       # raw binary
apple1-emu asm hello.s -o hello.hex       # Woz dump, to paste into the monitor
apple1-emu asm jeu.s -I ../common         # another folder for .include
```

Each command summarizes the assembly and gives the line for the real Apple-1. The summary is printed in French (“33 bytes … 2 file(s), 13 symbols / Tape (after C100R): 0300.0320R then 300R / written: hello.wav”):

```text
33 octets, $0300-$0320 ; 2 fichier(s), 13 symboles
Cassette (après C100R) : 0300.0320R   puis 300R
écrit : hello.wav
```

Without `.org`, the program is placed at `$0300`. The `.hex` dump has the form the Woz Monitor accepts from the keyboard, ending with the run command:

```text title="t1.hex (excerpt)"
0280: 34 12 34 12 10 05 FF 34
0288: 12 FF FF A8 02 EA EA EA
…
0280R
```

## Syntax

A line contains, in this order and all optional: a label, an instruction or a directive, and a comment after `;`. Mnemonics and directives may be written in lower or upper case; symbols are case-sensitive.

```asm
PTR     = $06           ; a constant (no byte produced)
start:  lda #0          ; a global label, followed by a colon
@loop:  inx             ; a local label: visible up to the next global one
        bne @loop
:       dex             ; an anonymous label
        bne :-          ; :- the previous one, :+ the next one, :-- the one before
```

{{!local labels}}{{!anonymous labels}}

**Numbers**: `255` in decimal, `$FF` in hexadecimal, `%1010` in binary, `'A'` for a character ($41). The keyboard returns its codes with **bit 7 set**: to compare a key, write `'A' | $80`; `apple1.inc` likewise defines `CR = $8D`. `*` stands for the current address.

**Operators**, from lowest to highest precedence:

| Precedence | Operators | Meaning |
|---|---|---|
| 1 | <code>&#124;&#124;</code> | logical or (result 0 or 1) |
| 2 | `&&` | logical and |
| 3 | `=` `<>` `<` `>` `<=` `>=` | comparisons (0 or 1) |
| 4 | `+` `-` <code>&#124;</code> | addition, subtraction, bitwise or |
| 5 | `*` `/` `&` `^` `<<` `>>` | multiplication, division, and, exclusive or, shifts |
| 6 (unary) | `<` `>` `-` `~` `!` | low byte, high byte, negation, complement, logical not |

So `lda #<message` loads the low byte of the address and `lda #>message` the high byte; `#>S+1` is `(>S)+1`. There is no modulo operator (`%` or `.mod` give a syntax error).

**Zero page or absolute.** An address below `$100` that is known at assembly time is encoded as zero page (`lda $0006` gives `A5 06`, as with ca65). To force the absolute form, write `a:` (`lda a:$06` gives `AD 06 00`); to force zero page, `z:`.

!!! attention "Define zero-page constants before using them"
    If `PTR = $06` is written **after** `lda PTR`, the assembler does not yet know the value and encodes the address on 2 bytes. It flags this in yellow: `warning: zero page address coded on 2 bytes (symbol defined further down; define it earlier, or write z: or a:)`. The program works, but it is longer, and `(zp),Y` modes become impossible.

## Directives

| Directive | Effect | Example |
|---|---|---|
| `.org addr` | assembly (and load) address | `.org $0300` |
| `.byte` (`.byt`) | bytes, strings included | `.byte CR, "OK", 0` |
| `.word` (`.addr`) | 16-bit words, low byte first | `.word 10000, end` |
| `.asciiz` | zero-terminated string | `.asciiz "HELLO"` |
| `.res n[, val]` | reserves n bytes (0, or val) | `.res 3, $EA` |
| `.include "f"` | inserts a file | `.include "apple1.inc"` |
| `.macro name args` … `.endmacro` | defines a macro (`.mac`, `.endmac`) | see `macros.s` |
| `.local a, b` | labels private to each macro call | |
| `.repeat n[, var]` … `.endrep` | repeats a block, var = 0 … n−1 | `.repeat 16, I` |
| `.if` `.elseif` `.else` `.endif` | conditional assembly | `.if WIDTH > 30` |
| `.ifdef` `.ifndef` | depending on whether a symbol exists | `.ifdef APPLE1` |
| `.ifblank` `.ifnblank` | macro argument empty or not | |
| `.defined(sym)` | 1 if the symbol exists (in an expression) | `.if .defined(d)` |
| `.define name value` | textual substitution (`.undefine`) | `.define CHARACTER '*'` |
| `.setcpu "65C02"` | accepts 65C02 instructions (`.pc02`, `.p02`) | |
| `.segment` `.code` `.data` `.rodata` `.bss` | accepted for compatibility | |
| `.feature string_escapes` | `\n`, `\"`… in strings | |

{{!.org}}{{!.byte}}{{!.word}}{{!.asciiz}}{{!.res}}{{!.include}}{{!.macro}}{{!.repeat}}{{!.if}}{{!.setcpu}}

!!! attention "Segments do not move anything"
    With ca65 and ld65, `.code`, `.data` and `.bss` send code to areas defined by the linker configuration. The built-in assembler produces **a single block**, in source order: segment directives are accepted but have no effect, and `.bss` produces zeros. For a project that needs a real `.cfg` file, the library uses ca65 and ld65 (chapter 4).

## Macros, .repeat and conditional assembly

A macro is a template of lines, copied at each call with its arguments. `.local` gives each call its own labels, without conflicts. `.repeat` unrolls a block at assembly time, which is also handy to **compute tables**:

```asm
squares:
.repeat 16, I
        .byte I * I     ; 0, 1, 4, 9 … 225
.endrep
```

Chapter 23 presents `macros.s`, which uses all of this, and `gen2-lores.s`, which computes the addresses of the 24 screen rows with `.repeat`.

## Error messages

Each error gives the file, the line, the column and the offending text; no byte is written while an error remains. The editor shows the messages in the interface language (below, English); the command line prints them in French.

| Message | Usual cause |
|---|---|
| `syntax error` | unknown operator, missing parenthesis |
| `unknown instruction, directive or macro` | typo; 65C02 instruction without `.setcpu`; label without a colon |
| `addressing mode not possible for this instruction` | `sta #5`, `ldx ($06),y`… |
| `undefined symbol` | `jsr ECO` instead of `ECHO`; forgotten `.include` |
| `symbol already defined` | two labels with the same name |
| `value out of range` | `lda #300`, address beyond `$FFFF` |
| `branch too far (-128 to +127 bytes)` | replace `bne far` with `beq :+` / `jmp far` / `:` |
| `file for .include not found` | name, or folder: add `-I` |
| `badly closed block (.macro/.endmacro, .if/.endif)` | forgotten `.endif` or `.endmacro` |
| `division by zero` | in an expression |
| `string or character without closing quote` | `"HELLO` |
| `value not known at this point (.org, .res, .if, .repeat)` | these directives need a value that is already defined |

![An error in the editor: ECO instead of ECHO. Clicking the message takes you to the line.](img/en/brut/editeur-asm-erreur.png "large")

## Compatibility with ca65 and xa

The built-in assembler follows the syntax of **ca65** (cc65 package) for everything a single-block program needs. The eight examples of chapter 23 were assembled with both tools and compared byte for byte:

| Source | Bytes | Built-in = ca65 |
|---|---|---|
| `hello.s` | 33 | <span class="status-ok">identical</span> |
| `clavier.s` | 32 | <span class="status-ok">identical</span> |
| `hexa.s` | 64 | <span class="status-ok">identical</span> |
| `decimal.s` | 117 | <span class="status-ok">identical</span> |
| `jeu.s` | 222 | <span class="status-ok">identical</span> |
| `macros.s` | 135 | <span class="status-ok">identical</span> |
| `gen2-lores.s` | 110 | <span class="status-ok">identical</span> |
| `gen2-hires.s` | 94 | <span class="status-ok">identical</span> |

Sources written for **xa** (xa65) are recognized by their `#include` or `#define` lines, and translated line by line before assembly (error line numbers stay correct): `#define`, `#include`, `*=`, `.BYT`, `.ASC`, `/* */` comments and labels in column 0 without a colon. Dave Schmenk’s slideshow (chapter 9) is assembled this way, to the same 3456 bytes as xa 2.4.1.

For **GitHub projects**, the library prefers ca65 and ld65 if they are installed (or if the project supplies an ld65 configuration), with `-D APPLE1` by default; otherwise it uses the built-in assembler. The repository’s `apple1.toml` file specifies the addresses and symbols (chapter 4).
