# Known bugs and their workarounds

<p class="lead">The Apple-1’s Integer BASIC was written without an assembler, in 1976, and was never fixed. Here are its known defects, each reproduced in the emulator with the real BASIC, and how to avoid them.</p>

<span class="verified">verified</span>: reproduced in the emulator with the real Integer BASIC, with a transcript to back it up. <span class="unverified">not verified</span>: reported by a source, not reproduced. Bug n corresponds to section 18.n. Since the emulator runs the original code instruction by instruction, a bug reproduced here also exists on a real Apple-1.

## Leaving a FOR loop with GOTO <span class="verified">verified</span>

A loop left with `GOTO` stays “open”. If the program goes through the `FOR` again, a new entry is pushed; on the ninth pass: `>8 FORS ERR`.

```basic
10 N=0
20 FOR I=1 TO 10
30 IF I=2 THEN GOTO 50
40 NEXT I
50 N=N+1:IF N<20 THEN 20
60 PRINT "DONE ";N
70 END
```

```mon
>» RUN
*** >8 FORS ERR
STOPPED AT 20
```

!!! astuce "Workaround"
    Never leave a `FOR` with `GOTO`: give the variable its final value and go through `NEXT`. Here: `30 IF I=2 THEN I=10:GOTO 40`. The program then prints `DONE 20`. The ports of *101 BASIC Computer Games* follow this rule.

## The ^ operator hangs the machine <span class="verified">verified</span>

`^` (power) is in the interpreter’s table, but `PRINT 2^3` never returns: only RESET gets you out.

!!! astuce "Workaround"
    Multiply: `2*2*2`, or a loop `P=1:FOR I=1 TO N:P=P*X:NEXT I`.

## The “PPED AT” message <span class="verified">verified</span>

When the interpreter’s operand stack overflows (an expression nested too deeply in direct mode), BASIC should print an error message, but it points **3 bytes too far** into the message table (Eric Smith’s disassembly notes it: `ldy #erri_stopped_at+3 ; The "+3" is a bug!`). What you read is the end of `STOPPED AT`:

```mon
>» PRINT 1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1)))))))))))))
14
*** PPED AT  ERR
STOPPED AT 65535
```

!!! astuce "Workaround"
    Break long expressions up with intermediate variables. Beyond 12 levels of parentheses, a program line is refused anyway (`TOO LONG ERR`).

## IF … THEN governs only the first statement <span class="verified">verified</span>

In `IF A=2 THEN PRINT "X":PRINT "Y"`, only `PRINT "X"` depends on the condition: `PRINT "Y"` is **always** executed (with A=1, the program prints `Y`). This is the most common trap when porting a program written for another BASIC.

!!! astuce "Workaround"
    Invert the condition and jump over the block:

    ```basic
    80 IF A#2 THEN 90
    85 PRINT "X":PRINT "Y"
    90 REM CONTINUE
    ```

## −32768 <span class="verified">verified</span>

`X=-32767-1` prints a `-` then `*** >32767 ERR`. The smallest usable value is −32767.

!!! astuce "Workaround"
    Stay within −32767 … 32767. To read a 16-bit word with `PEEK`, split it: `PEEK(A)+PEEK(A+1)*256` overflows above 32767; test the high byte first (`IF PEEK(A+1)>127 THEN …`).

## ASC( does not work <span class="verified">verified</span>

`ASC("A")` is in the keyword table, but any line that uses it is refused: `SYNTAX ERR`. The same goes for `SCRN(`.

!!! astuce "Workaround (tried)"
    Look the character up in a reference string. Line 60 closes the loop (`K=26:NEXT K`) to avoid bug 1:

```basic title="The position of a letter, without ASC"
10 DIM C$(26),L$(1)
20 C$="ABCDEFGHIJKLMNOPQRSTUVWXYZ"
30 INPUT "LETTER",L$
40 FOR K=1 TO 26:IF C$(K,K)=L$ THEN 60:NEXT K
50 PRINT "NOT A LETTER":END
60 PRINT "POSITION ";K:K=26:NEXT K
70 GOTO 30
```

```mon
LETTER?» W
POSITION 23
LETTER?» E
POSITION 5
LETTER?» 5
NOT A LETTER
```

## Graphics keywords that do nothing <span class="verified">verified</span>

`COLOR=`, `PLOT` and `HLIN` are accepted and do nothing; `VLIN` and `VTAB` give `SYNTAX ERR`. They are leftovers of the BASIC intended for the Apple II.

!!! astuce "Workaround"
    To position text, use `TAB n` and empty `PRINT`s. To draw in color, the GEN2 card is driven with `POKE` (chapter 6).

## The Apple II 251-character bug <span class="verified">absent</span>

A later version of Integer BASIC (Apple II) crashes on certain lines of 251 characters or more. On the Apple-1 the typed line is limited well before that (about 128 characters): the bug cannot occur.

## STOPPED AT 65535 <span class="verified">verified</span>

An error caused by a line **refused while it is being entered** (nested too deeply) or by bug 3 in direct mode is followed by `STOPPED AT 65535`: BASIC believes it is in a program and prints an absurd line number. It has no consequences; the program in memory is not touched.

## USR crashes the machine <span class="verified">verified</span>

`X=USR(1)` jumps to `$0000`, where there is normally no code: the machine hangs. RESET then `E2B3R` return to BASIC without losing the program.

!!! astuce "Workaround"
    Call your machine-language programs with `CALL address`; pass values with `POKE` and `PEEK`.

## HIMEM= cannot go down <span class="verified">verified</span>

`HIMEM=3072` (and any value below the current one) is **silently ignored**, even after `SCR`, because the interpreter compares the new value with the program pointer before accepting it.

!!! astuce "Workaround (verified)"
    `POKE 76,0 : POKE 77,12 : SCR` sets HIMEM to 3072; the next program is stored below it. See chapter 16.

## The colon refused in direct mode <span class="verified">verified</span>

`X=5:Y=X*2:PRINT Y` typed without a number gives `SYNTAX ERR`; the same line with a number works. `FOR` and `GOSUB` are also refused in direct mode.

!!! astuce "Workaround"
    Type one statement per line, or write a temporary line `0` and `RUN 0`… without forgetting to delete it (`DEL 0`).

## INPUT always adds “?” <span class="verified">verified</span>

`INPUT "AGE ? ",N` prints `AGE ? ?`: BASIC adds its own question mark. Write the prompt without `?`: `INPUT "AGE",N` prints `AGE?`.

## The FOR loop runs at least once <span class="verified">verified</span>

`FOR I=5 TO 1 … NEXT I` executes the body once (with I=5), where other BASICs skip it.

!!! astuce "Workaround"
    Test first: `IF N<1 THEN 100` before `FOR I=1 TO N`.

## A key press stops the program <span class="verified">verified</span>

Any key pressed while the program runs stops it (`STOPPED AT`), except during an `INPUT`. This is not a bug but a design choice: there is no BREAK key. Consequence: a real-time game cannot wait for a key without stopping. Reading the keyboard with `PEEK(-12271)` (`$D011`) and `PEEK(-12272)` (`$D010`) looks possible, but the interpreter also tests the keyboard between statements and may stop first: we have not tried it (<span class="unverified">not verified</span>). For a real-time game, move to machine language (Part III).

## Summary

| # | Bug | Status | Workaround |
|---|---|---|---|
| 1 | leaving FOR with GOTO → `>8 FORS` | <span class="status-ok">verified</span> | end the loop with NEXT |
| 2 | `^` hangs | <span class="status-ok">verified</span> | multiply |
| 3 | “PPED AT” (+3) | <span class="status-ok">verified</span> | intermediate variables |
| 4 | `IF…THEN a:b` executes b | <span class="status-ok">verified</span> | inverted condition + jump |
| 5 | −32768 | <span class="status-ok">verified</span> | stay ≥ −32767 |
| 6 | `ASC(`, `SCRN(` | <span class="status-ok">verified</span> | reference string |
| 7 | COLOR=, PLOT, HLIN do nothing; VLIN, VTAB refused | <span class="status-ok">verified</span> | TAB, POKE (GEN2) |
| 8 | 251 characters (Apple II) | <span class="status-ok">absent</span> | — |
| 9 | STOPPED AT 65535 | <span class="status-ok">verified</span> | harmless |
| 10 | USR crashes | <span class="status-ok">verified</span> | CALL |
| 11 | HIMEM= does not go down | <span class="status-ok">verified</span> | POKE 76/77 + SCR |
| 12 | `:` refused in direct mode | <span class="status-ok">verified</span> | one statement per line |
| 13 | INPUT adds “?” | <span class="status-ok">verified</span> | prompt without “?” |
| 14 | FOR runs once | <span class="status-ok">verified</span> | test first |
| 15 | reading the keyboard with PEEK during RUN | <span class="status-src">not verified</span> | machine language |
