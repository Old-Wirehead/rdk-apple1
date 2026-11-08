# Error messages and limits

## Messages

An error prints `*** MESSAGE ERR`, followed, when a program is running, by `STOPPED AT n`. A line refused on entry is not stored: retype it.

| Message | Cause | Example tried |
|---|---|---|
| {{SYNTAX ERR}} | unknown statement, invalid variable name, `:` in direct mode | `CNT=7`, `GET A$`, `X=5:PRINT X` in direct mode |
| {{>32767 ERR}} | overflow, division by zero, line number > 32767 | `200*200`, `10/0`, `RND(0)`, `32768 PRINT` |
| {{TOO LONG ERR}} | line too complex (expression nested too deeply) or too long | 13 nested parentheses |
| {{MEM FULL ERR}} | no more room between variables and program | `DIM A(3000)` in 8 KB |
| {{TOO MANY PARENS ERR}} | parentheses (provided for by the interpreter; masked by bug 3) | — |
| {{STRING ERR}} | string misused | — |
| {{END ERR}} | the program ends without `END` | `10 PRINT "X"` then `RUN` |
| {{BAD BRANCH ERR}} | `GOTO`/`GOSUB` to a nonexistent line | `GOTO 500` |
| {{>8 GOSUBS ERR}} | more than 8 nested `GOSUB`s | recursion: 9 calls |
| {{BAD RETURN ERR}} | `RETURN` without `GOSUB` | `RUN 30` on a `RETURN` |
| {{>8 FORS ERR}} | more than 8 nested `FOR`s, or bug 1 | 9 loops on one line |
| {{BAD NEXT ERR}} | `NEXT` without a matching `FOR` | `20 NEXT I` alone |
| {{STOPPED AT}} | a key was pressed while running | — |
| {{>255 ERR}} | value > 255: `POKE`, `DIM` of a string | `POKE 768,300`, `DIM B$(256)` |
| {{RANGE ERR}} | array index out of bounds | `A(0)`, `A(3)` after `DIM A(2)` |
| {{DIM ERR}} | array or string dimensioned again | — |
| {{STR OVFL ERR}} | string longer than its `DIM` | `A$="ABCDE"` with `DIM A$(3)` |
| {{RETYPE LINE}} | invalid answer to `INPUT` (letters for a number): retype | — |

The “—” rows come from the interpreter’s message table; they were not provoked in our tests.

## Limits

| Limit | Value |
|---|---|
| integers | −32767 to 32767 |
| line numbers | 0 to 32767 |
| variable names | letter + optional digit |
| string names | one letter + `$` |
| string length | 255 |
| array dimensions | 1, indices from 1 to n |
| nested `FOR`s | 8 |
| nested `GOSUB`s | 8 |
| nested parentheses | 12 |
| length of a typed line | 120 characters get through, 141 are refused |


## Saving memory

In 8 KB, BASIC has about 2 KB for the program and the variables. A few ways to gain some:

<ol class="steps">
<li><strong>Lower LOMEM</strong> if you have no machine code: <code>LOMEM=768</code> (<code>$0300</code>) gives the variables 1280 more bytes. Tried: <code>DIM T(1200)</code> fails with <code>MEM FULL</code> with the default LOMEM, and succeeds after <code>LOMEM=768</code>. The ports of *101 BASIC Computer Games* made during development do just that.</li>
<li><strong>Write tersely</strong>: BASIC keeps spaces and <code>REM</code>s. <code>LIST</code> re-indents by itself, and one long line holding several statements costs less than several lines.</li>
<li><strong>Reuse arrays and strings</strong> rather than dimensioning new ones.</li>
<li><strong>Add RAM</strong>: a program’s ⚙ settings, <code>--ram 16</code>, or a real expansion. A program over 8 KB requires a 12, 16 or 32 KB machine: <code>apple1-emu basic --ram 16</code> places it at the top of that memory.</li>
</ol>
