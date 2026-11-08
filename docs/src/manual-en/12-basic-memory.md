# Variables, strings, arrays and memory

## Numbers

Integer BASIC knows only **signed 16-bit integers**, from −32767 to 32767. A calculation that leaves this range stops with `*** >32767 ERR` (`200*200`, `32767+1`, and also `10/0` and `RND(0)`). The value −32768 cannot be used (chapter 18, bug 5). There is no decimal point and no `INT`: `7/2` is 3.

## Variable names

This is the limit that surprises people most: a variable name is **one letter, optionally followed by one digit**.

| Name | Result (tried) |
|---|---|
| `A`, `Z`, `B1`, `Z9` | accepted |
| `AB`, `CNT`, `SCORE`, `COUNTER` | `SYNTAX ERR` |
| `A12` | `SYNTAX ERR` |
| `A$`, `N$` | string: **one letter** only |
| `B9$`, `Z9$` | `SYNTAX ERR` |

That makes 26 × 11 = 286 possible integer variables, and 26 strings. Get into the habit of noting the role of each variable in a `REM`.

## Strings

A string must be **dimensioned** before use, with its maximum length (1 to 255): `DIM N$(20)`. `DIM B$(256)` gives `>255 ERR`.

| Operation | Written as | Example (N$ = "KAYAK") |
|---|---|---|
| length | `LEN(N$)` | 5 |
| substring | `N$(i,j)`: characters i to j | `N$(1,3)` = "KAY" |
| tail | `N$(i)`: from character i to the end | `N$(4)` = "AK" |
| concatenate | `A$(LEN(A$)+1)=B$` | "APPLE" + "-1" → "APPLE-1" |
| compare | `=` and `#` only | `IF N$="WOZ" THEN …` |

Assigning a string longer than its `DIM` gives `*** STR OVFL ERR`, as does an answer to `INPUT` that is too long. There is no `MID$`, `LEFT$`, `CHR$`, `STR$`, `VAL` or `ASC` (the latter is in the table but does not work).

!!! astuce "Replacing CHR$ and VAL"
    To print a character from its code, keep a reference string and slice it: `DIM C$(26):C$="ABCDEFGHIJKLMNOPQRSTUVWXYZ"` then `PRINT C$(K,K)`. To read a number, use `INPUT N` rather than `INPUT N$`.

## Arrays

`DIM T(10)` creates an array of 10 integers, **numbered from 1 to 10**. `T(0)` and `T(11)` give `*** RANGE ERR`. There is only one dimension: for an 8 × 8 grid, use `G((L-1)*8+C)`. In the emulator, elements are 0 after `DIM`; on a real machine memory is not necessarily cleared: initialize your arrays. An array too large for memory gives `*** MEM FULL ERR` (`DIM A(3000)` in 8 KB).

## BASIC’s memory

![Apple-1 memory: BASIC at $E000, the program below HIMEM, the variables above LOMEM.](diagrams/en/memoire.svg "diagram large")

BASIC stores the **program at the top** of memory, growing down from HIMEM, and the **variables at the bottom**, growing up from LOMEM. The free space lies between the two.

| Pointer | Address | Value in 8 KB | Role |
|---|---|---|---|
| LOMEM | `$4A-$4B` (74-75) | `$0800` (2048) | start of the variables |
| HIMEM | `$4C-$4D` (76-77) | `$1000` (4096) | end of BASIC’s memory |
| PP | `$CA-$CB` (202-203) | moves down with the program | start of the program |
| PV | `$CC-$CD` (204-205) | moves up with the variables | end of the variables |

{{!LOMEM}}{{!HIMEM}}{{!PP (program pointer)}}

`PRINT PEEK(202)+PEEK(203)*256` gives the start of the program and `PRINT PEEK(204)+PEEK(205)*256` the end of the variables: the free space is the difference between the two. An integer variable such as `A` takes 6 bytes.

!!! apple1 "Why a BASIC tape starts at $004A"
    These pointers are in zero page, from `$4A` to `$FF`. A BASIC tape therefore records **two ranges**: `004A.00FF` (the pointers), then the program from PP to HIMEM, for example `0F2C.0FFF`. The `apple1-emu basic` command computes them for you (its output is in French: “10 lines, 212 bytes… 1836 bytes free for the variables… ACI ranges (after C100R)… then E2B3R and RUN”):

    ```text
    10 lignes, 212 octets ; LOMEM $0800 HIMEM $1000 PP $0F2C ; 1836 octets libres pour les variables ; RAM 8 Ko
    Plages ACI (après C100R) : 004A.00FFR 0F2C.0FFFR   puis E2B3R et RUN
    ```

### Changing LOMEM and HIMEM

The syntax is `LOMEM=n` and `HIMEM=n` (with `=`, **not** `:` as on the Apple II, which gives `SYNTAX ERR`).

- `LOMEM=2304` moves the start of the variables and **clears the variables** (the program stays). Useful to reserve `$0800-$08FF` for a machine-language program.
- `HIMEM=` **higher** (with more RAM, for example `HIMEM=8192` in 16 KB) works and keeps the program.
- `HIMEM=` **lower** is **silently ignored** by the original, even after `SCR` (chapter 18, bug 11). **RDK G2 Basic accepts it**: the program in memory is moved down with the limit, or `MEM FULL ERR` is given if it would no longer fit above the variables (`RANGE ERR` for a negative value; `LOMEM=` gives `RANGE ERR` if it is not below the start of the program, and `>32767 ERR` above 32767). With the original, there is a verified workaround: write the pointer directly, then `SCR`:

```basic title="Lowering HIMEM to 3072 ($0C00)"
POKE 76,0 : POKE 77,12 : SCR
```

At the start of G2 Basic, HIMEM is the top of the RAM found and LOMEM is `$0800` (`$2000` with a GEN2 driver). (`76`/`77` = `$4C`/`$4D`, 12 × 256 = 3072.) Type these three statements on a numbered line or separately in direct mode, **before** entering the program: `SCR` erases it.

## Mixing BASIC and machine language

| Area (8 KB) | Usable for 6502 code? |
|---|---|
| `$0000-$00FF` | zero page: the Woz Monitor uses `$24-$2B`, BASIC at least `$4A-$FF`; avoid it, or save and restore it |
| `$0100-$01FF` | 6502 stack: no |
| `$0200-$027F` | Woz Monitor line buffer; overwritten by every line typed |
| `$0280-$07FF` | free as long as BASIC does not touch it: the best place |
| `$0800-…` | BASIC variables (unless LOMEM is raised) |
| `… -$0FFF` | BASIC program |

The machine-language program is loaded from tape or by POKE, and called with `CALL 768` (for `$0300`). It must end with `RTS` and preserve BASIC’s zero page. Minimal example:

```mon
>» POKE 768,96
>» CALL 768
>
```

(96 = `$60` = `RTS`: the call returns at once.)
