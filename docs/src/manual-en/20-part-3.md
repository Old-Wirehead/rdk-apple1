# 6502 assembly <span class="part-sub">The processor, the built-in assembler and its tools, input/output and the Woz Monitor routines, annotated examples and tips</span> {: .part data-part="Part III" }

# The 6502 in brief

<p class="lead">The Apple-1’s 6502 is an 8-bit processor running at 1.023 MHz, with three registers, 56 instructions and 64 KB of address space. This chapter gives the essentials needed to read and write the examples in this part.</p>

## Registers

| Register | Size | Role |
|---|---|---|
| **A** | 8 bits | the accumulator: arithmetic, comparisons, reads and writes |
| **X**, **Y** | 8 bits | the index registers: loop counters, offset into a table |
| **S** | 8 bits | the stack pointer; the stack occupies page `$0100-$01FF` |
| **PC** | 16 bits | the program counter: the address of the next instruction |
| **P** | 8 bits | the flags N V – B D I Z C |

{{!registers (6502)}}

The flags are updated by almost every instruction:

<dl class="ref">
<dt>N (negative)</dt><dd>copy of bit 7 of the result; tested by <code>BMI</code> / <code>BPL</code>.</dd>
<dt>V (overflow)</dt><dd>signed overflow of an addition or subtraction; <code>BVS</code> / <code>BVC</code>.</dd>
<dt>Z (zero)</dt><dd>the result is zero; <code>BEQ</code> / <code>BNE</code>.</dd>
<dt>C (carry)</dt><dd>carry out of an addition, no borrow from a subtraction, bit shifted out; <code>BCS</code> / <code>BCC</code>.</dd>
<dt>D (decimal)</dt><dd><code>ADC</code> and <code>SBC</code> work in BCD. The Woz Monitor does <code>CLD</code> at start-up.</dd>
<dt>I (interrupts)</dt><dd>masks IRQ. The stock Apple-1 has none.</dd>
</dl>

## Addressing modes

| Mode | Syntax | Bytes | Example | Effect |
|---|---|---|---|---|
| implied | `INX` | 1 | `inx` | X ← X + 1 |
| accumulator | `ASL A` | 1 | `asl a` | shifts A |
| immediate | `#val` | 2 | `lda #$8D` | A ← $8D |
| zero page | `zp` | 2 | `lda $06` | A ← memory $0006 |
| zero page, X | `zp,X` | 2 | `lda $06,x` | |
| absolute | `addr` | 3 | `lda $D010` | |
| absolute, X / Y | `addr,X` | 3 | `lda message,x` | indexed table |
| indirect indexed | `(zp),Y` | 2 | `lda ($06),y` | 16-bit pointer in zero page, plus Y |
| indexed indirect | `(zp,X)` | 2 | `lda ($06,x)` | table of pointers |
| relative | `label` | 2 | `bne loop` | branch of −128 to +127 bytes |
| indirect | `(addr)` | 3 | `jmp ($FFFC)` | `JMP` only |

**Zero page** (`$0000-$00FF`) is the most precious: its instructions are shorter and faster, and only this mode allows `(zp),Y` pointers. The Woz Monitor uses only `$24-$2B`; Integer BASIC uses much more (chapter 16).

## Instructions by family

| Family | Instructions |
|---|---|
| load, store | `LDA LDX LDY STA STX STY` |
| transfers | `TAX TAY TXA TYA TSX TXS` |
| stack | `PHA PLA PHP PLP` |
| arithmetic | `ADC SBC` (with carry: `CLC` before `ADC`, `SEC` before `SBC`) |
| increments | `INC INX INY DEC DEX DEY` |
| logic | `AND ORA EOR BIT` |
| shifts | `ASL LSR ROL ROR` |
| comparisons | `CMP CPX CPY` (C = 1 if register ≥ value, Z = 1 if equal) |
| branches | `BEQ BNE BCC BCS BMI BPL BVC BVS` |
| jumps | `JMP JSR RTS RTI BRK` |
| flags | `CLC SEC CLD SED CLI SEI CLV` |
| nothing | `NOP` |

{{!instructions (6502)}}

!!! astuce "Unsigned comparison"
    After `CMP #n`, `BCC` branches if A < n and `BCS` if A ≥ n. For “A > n”, compare with n + 1: that is what the examples do with `cmp #'9' | $80 + 1`.

## An NMOS 6502, not a 65C02

The Apple-1 has an original (NMOS) 6502, and so does the emulator. Undocumented opcodes execute as `NOP`s of the right length, and the `JAM` opcodes halt the processor. The assembler accepts 65C02 instructions after `.setcpu "65C02"` (`BRA`, `STZ`, `PHX`, `PLX`, `PHY`, `PLY`, `INC A`…), for a board fitted with a 65C02, but they **do not work** in the emulator:

```asm title="c02.s (test)"
        .setcpu "65C02"
        .org $0300
        lda #$C1        ; "A"
        bra next        ; on an NMOS 6502: 2-byte NOP
        lda #$D8        ; "X"
next:   jsr $FFEF
        jmp $FF1F
```

This program prints `A` on a 65C02, and `X` in the emulator: there `BRA` ($80) is a `NOP` that skips its operand, and the `LDA #$D8` is executed.

## The memory map at a glance

| Addresses | Contents |
|---|---|
| `$0000-$00FF` | zero page (Woz Monitor: `$24-$2B`; BASIC: much more) |
| `$0100-$01FF` | stack |
| `$0200-$027F` | Woz Monitor line buffer |
| `$0280-$1FFF` | free: your programs, often at `$0280` or `$0300` (8 KB of RAM by default in the emulator; 4 KB, up to `$0FFF`, on a stock Apple-1) |
| `$C000-$C0FF` | ACI card (cassette); `$C100-$C1FF`: its program |
| `$C250-$C257` | GEN2 card switches |
| `$D010-$D013` | PIA: keyboard and display |
| `$E000-$EFFF` | Integer BASIC (if loaded) |
| `$FF00-$FFFF` | Woz Monitor |

Appendix A gives the complete map.
