# Statement and function reference

<p class="lead">Every statement recognized by the Apple-1’s Integer BASIC, taken from the interpreter’s keyword table, with what it really does. Every example was tried.</p>

## Statements

`PRINT` list `[;]`
{: .syntax }

{{PRINT}} displays numbers and strings. `;` joins items with no space (`PRINT A;B` prints `12` for A=1, B=2); `,` moves to the next column, in steps of 8 (`PRINT 1,2,3` prints `1       2       3`). A final `;` suppresses the line break. `PRINT` alone moves to the next line.

`INPUT` `["prompt",]` variable `[,variable…]`
{: .syntax }

{{INPUT}} reads a number or a string. BASIC always prints `?` after the prompt: `INPUT "NAME",N$` prints `NAME?`. A string too long for its `DIM` gives `STR OVFL ERR`.

`LET` variable `=` expression
{: .syntax }

{{LET}} is optional: `A=5` is enough.

`IF` expression `THEN` statement | number
{: .syntax }

{{IF}} executes the statement when the expression is not zero (`IF A THEN …` is allowed). `THEN 100` jumps to line 100. **Caution**: only the first statement depends on the condition (chapter 18, bug 4).

`GOTO` expression — `GOSUB` expression — `RETURN`
{: .syntax }

{{GOTO}} and {{GOSUB}} accept an **expression**: `GOTO K*100`, `GOSUB 300+K` (the “computed GOTO” stands in for the missing `ON … GOTO`). A nonexistent line gives `BAD BRANCH ERR`. At most eight nested {{GOSUB}}s (`>8 GOSUBS ERR` on the ninth); `RETURN` without `GOSUB` gives `BAD RETURN ERR`.

`FOR` var `=` start `TO` end `[STEP step]` … `NEXT` var
{: .syntax }

{{FOR}}/{{NEXT}}: the body is executed **at least once**, even if start is past end (`FOR I=5 TO 1` goes through once). `STEP` may be negative. At most eight nested loops (`>8 FORS ERR`). `NEXT` must name the variable; a `NEXT` without `FOR` gives `BAD NEXT ERR`.

`DIM` var`(n)` `[,var$(n)…]`
{: .syntax }

{{DIM}} creates an integer array of `n` elements, numbered **from 1 to n** (`A(0)` gives `RANGE ERR`), or reserves `n` characters (at most 255) for a string. Every string must be dimensioned before it is assigned.

`REM` text
{: .syntax }

{{REM}}: a comment, up to the end of the line (a `:` after REM is part of the comment).

`END`
{: .syntax }

{{END}} ends the program. A program that ends without `END` prints `*** END ERR`: this is harmless, but end your programs with `END`.

`TAB` n
{: .syntax }

{{TAB}} is a **statement** (not a PRINT function as in Applesoft): `TAB 10:PRINT "*"` prints the star in column 10.

`POKE` address`,`value
{: .syntax }

{{POKE}} writes a byte (0 to 255, otherwise `>255 ERR`). Addresses above 32767 are written as negative numbers: address − 65536.

`CALL` address
{: .syntax }

{{CALL}} calls a machine-language program, which returns with `RTS`. Example: `POKE 768,96 : CALL 768` (96 = `RTS`). `CALL -256` returns cleanly to the Woz Monitor (`$FF00`).

## Functions

| Function | Result | Verified example |
|---|---|---|
| {{PEEK}}`(addr)` | the byte at the address | `PEEK(76)+PEEK(77)*256` gives HIMEM (4096) |
| {{RND}}`(n)` | random integer from 0 to n−1; `RND(0)`: `>32767 ERR`; `RND(-5)` gives a negative number | `RND(6)+1`: a die |
| {{SGN}}`(x)` | −1, 0 or 1 (also for a number of 256 or more) | `SGN(-5);SGN(0)` prints `-10` |
| {{ABS}}`(x)` | absolute value | `ABS(-9)` = 9 |
| `KEY(n)` (G2 Basic only) | code of the key pressed, without waiting (see below) | `K=KEY(0)` |
| {{LEN}}`(a$)` | length of the string | `LEN("ABC")` = 3 |
| {{USR}}`(x)` | calls `$0000`: **crashes** without a prepared program | — |
| {{ASC}}`(`, {{SCRN}}`(` | present in the table, but `SYNTAX ERR` | — |

## Operators

| Precedence | Operators |
|---|---|
| 1 | unary `-` and `+`, `NOT` |
| 2 | `*` `/` `MOD` |
| 3 | `+` `-` |
| 4 | `=` `#` (not equal, also `<>`) `<` `>` `<=` `>=` |
| 5 | `AND` |
| 6 | `OR` |

{{MOD}}, {{AND}}, {{OR}} and {{NOT}}:

- Division **truncates toward zero**: `7/2` = 3, `-7/2` = −3. `MOD` takes the sign of the dividend: `-7 MOD 3` = −1.
- Comparisons give 1 (true) or 0 (false): `PRINT 5>3;5=3` prints `10`. `NOT 0` = 1, `NOT 5` = 0.
- `AND` and `OR` are logical, not bitwise: `3 AND 0` = 0, `3 OR 0` = 1.
- `3*-2` is accepted and equals −6.
- `^` exists in the table, but **hangs the machine** (chapter 18, bug 2).
- Strings can only be compared with `=` and `#`; `"ABC"<"ABD"` is refused (`SYNTAX ERR`).

## Extensions of RDK G2 Basic

These work only with **RDK G2 Basic** (the original Integer BASIC answers `SYNTAX ERR`).

`KEY(n)`
{: .syntax }

`KEY(0)` returns the **code of the key** that was pressed (the 7-bit ASCII code: 65 for `A`, 13 for Return, 32 for the space bar) and consumes it, or `0` if no key was pressed. It never waits, so a program can run while it watches the keyboard. `KEY(1)` (any other non-zero argument) **waits** for a key.

```basic title="Wait for a key without INPUT"
10 K=KEY(0)
20 IF K=0 THEN 10
30 PRINT K
```

In the original, a key pressed while a program runs **stops it** (`STOPPED AT n`), and `INPUT` blocks until Return. With G2 Basic, once a program has called `KEY`, a key press no longer stops it: only `KEY` sees the keyboard, and <kbd>Ctrl</kbd>+<kbd>C</kbd> (code 3) read by `KEY` stops the program at the next call. Call `KEY` in the main loop of the program, then.

`LOMEM=` n — `HIMEM=` n
{: .syntax }

Immediate commands that take an expression (`LOMEM=2048+512`); inside a program they give `SYNTAX ERR`, as in the original. They are described with the memory in chapter 16.

`LIST` `[`a`[,`b`]]` — `DEL` a`[,`b`]`
{: .syntax }

`LIST` lists everything, `LIST n` one line, `LIST a,b` the lines from `a` to `b`. `DEL n` deletes one line, `DEL a,b` the lines from `a` to `b` (if `b` is below `a`, only line `a`). A listed line longer than the screen is wrapped like the original’s (continuation lines are indented by six spaces).

String slices follow the original: `A$(I)` with `I` of 256 or more, or negative, gives `>255 ERR`, and `A$(I,J)` with `J` below `I-1` gives `STRING ERR`.

## Keywords that do nothing

The table contains `COLOR=`, `PLOT`, `HLIN`, `VLIN` and `VTAB`, inherited from the BASIC project for the Apple II. On the Apple-1, `COLOR=` and `PLOT` are accepted and do nothing, `VTAB` gives `SYNTAX ERR`. Keep them out of your programs.
