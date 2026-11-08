# Porting an Applesoft or Microsoft BASIC program

<p class="lead">Most listings from the 1970s and 1980s are written in Microsoft BASIC (Applesoft, the BASIC of “101 BASIC Computer Games”). They can be ported to the Apple-1, provided you know the equivalents. Every “refused” line below was typed into the real BASIC.</p>

## Equivalents

| Applesoft / Microsoft | Apple-1 Integer BASIC | Note |
|---|---|---|
| `ON X GOTO 100,200,300` | `GOTO 100*X` (if the line numbers allow it) or a series of `IF X=1 THEN 100` | refused |
| `GET A$` | `INPUT A$` | refused |
| `DATA` / `READ` / `RESTORE` | array filled by assignments, or a sliced string | refused |
| `MID$(A$,I,N)` | `A$(I,I+N-1)` | refused |
| `LEFT$(A$,N)` / `RIGHT$(A$,N)` | `A$(1,N)` / `A$(LEN(A$)-N+1)` | — |
| `A$+B$` | `A$(LEN(A$)+1)=B$` | — |
| `CHR$(N)`, `ASC(A$)` | reference string (chapter 18, bug 6) | refused |
| `STR$(N)`, `VAL(A$)` | `PRINT N` directly; `INPUT N` | refused |
| `INT(X)` | unnecessary: everything is an integer | refused |
| `RND(1)` (real from 0 to 1) | `RND(N)` (integer from 0 to N−1) | — |
| `X^2` | `X*X` | hangs (bug 2) |
| `HOME` | 24 × `PRINT`, or CLEAR SCREEN (<kbd>Ctrl</kbd>+<kbd>L</kbd>) by hand | refused |
| `PRINT TAB(10);"X"` | `TAB 10:PRINT "X"` | — |
| `STOP` | `END` | refused |
| `DEF FN` | `GOSUB` | — |
| `A<>B` | `A#B` (`<>` is also accepted) | — |
| `IF A THEN X:Y` | inverted condition + jump (bug 4) | trap |
| `DIM A(10,10)` | `DIM A(121)` and `A(I*11+J+1)` | — |
| `A(0)` | indices start at 1 | `RANGE ERR` |
| `SCORE`, `NAME$` | `S`, `N$`: one letter (+ digit) | `SYNTAX ERR` |
| real numbers | integers ×10 or ×100 (fixed point) | — |

{{!porting}}

## The method

<ol class="steps">
<li><strong>List the variables</strong> of the original program and assign each one a letter (+ digit). Record the table in <code>REM</code>s at the top.</li>
<li><strong>Replace the DATA</strong> with assignments in an initialization subroutine, or with a string you slice.</li>
<li><strong>Find every <code>IF … THEN … : …</code></strong> and rewrite it with a jump (bug 4).</li>
<li><strong>Find every exit from a <code>FOR</code> by <code>GOTO</code></strong> and replace it with “final value + <code>NEXT</code>” (bug 1).</li>
<li><strong>Shift array indices</strong> by 1 if the program uses index 0.</li>
<li><strong>Check in the editor</strong>: it flags words unknown to Integer BASIC as you type. <strong>▶ Run</strong> types the program in: any refused line appears on screen with <code>SYNTAX ERR</code>.</li>
<li><strong>Measure the memory</strong>: <code>apple1-emu basic prog.bas</code> gives the size and the RAM needed. Too big for 8 KB? <code>LOMEM=768</code> (chapter 17), or a 16 KB machine.</li>
<li><strong>Export</strong> the tape (<strong>📼 Export WAV</strong>) and note the ranges for the real machine.</li>
</ol>

## A porting example

A typical fragment in Microsoft BASIC:

```text title="Original (Microsoft BASIC)"
100 READ N$,S
110 IF S>HI THEN HI=S:BEST$=N$
120 IF N$<>"END" THEN 100
130 PRINT "BEST: ";BEST$;" (";HI;")"
140 DATA "ANNE",12,"BOB",17,"CLAIRE",9,"END",0
```

On the Apple-1: no `READ`/`DATA`, no long names, and the `IF` on line 110 would assign `BEST$` on every pass (bug 4).

```basic title="Integer BASIC port (tried)"
10 DIM N$(10),B$(10),S(3)
20 S(1)=12:S(2)=17:S(3)=9
30 H=0
40 FOR I=1 TO 3
50 GOSUB 200
60 IF S(I)<=H THEN 80
70 H=S(I):B$=N$
80 NEXT I
90 PRINT "BEST: ";B$;" (";H;")"
100 END
200 IF I=1 THEN N$="ANNE"
210 IF I=2 THEN N$="BOB"
220 IF I=3 THEN N$="CLAIRE"
230 RETURN
```

```mon
>» RUN
BEST: BOB (17)
```

## Lessons from the ports

Eight games from *101 BASIC Computer Games* (DEC, 1975) were ported this way during the emulator’s development: BAGLES, MUGWMP, HEX, BULCOW, STARS, TRAP, CHOMP and WAR. Each was played in the emulator, exported to tape and reloaded through the ACI ROM. These ports are not distributed (the original listings are not free), but they set the rules of this chapter: one-letter variables, arrays starting at 1, no `READ`/`DATA`, no `GOTO` out of a `FOR`, a single statement governed by `IF`, and `LOMEM=768` for the larger ones.
