# 6502-Assembler <span class="part-sub">Der Prozessor, der eingebaute Assembler und seine Werkzeuge, Ein- und Ausgabe und die Routinen des Woz Monitors, kommentierte Beispiele und Tipps</span> {: .part data-part="Teil III" }

# Der 6502 in Kürze

<p class="lead">Der 6502 des Apple-1 ist ein 8-Bit-Prozessor mit 1,023 MHz, mit drei Registern, 56 Befehlen und 64 KB Adressraum. Dieses Kapitel gibt das Wesentliche, um die Beispiele dieses Teils zu lesen und zu schreiben.</p>

## Register

| Register | Größe | Rolle |
|---|---|---|
| **A** | 8 Bit | der Akkumulator: Rechnen, Vergleiche, Lesen und Schreiben |
| **X**, **Y** | 8 Bit | die Indexregister: Schleifenzähler, Versatz in einer Tabelle |
| **S** | 8 Bit | der Stapelzeiger; der Stapel belegt die Seite `$0100-$01FF` |
| **PC** | 16 Bit | der Befehlszähler: die Adresse des nächsten Befehls |
| **P** | 8 Bit | die Flags N V – B D I Z C |

{{!Register (6502)}}

Die Flags werden von fast jedem Befehl aktualisiert:

<dl class="ref">
<dt>N (negativ)</dt><dd>Kopie von Bit 7 des Ergebnisses; geprüft von <code>BMI</code> / <code>BPL</code>.</dd>
<dt>V (Überlauf)</dt><dd>vorzeichenbehafteter Überlauf einer Addition oder Subtraktion; <code>BVS</code> / <code>BVC</code>.</dd>
<dt>Z (null)</dt><dd>das Ergebnis ist null; <code>BEQ</code> / <code>BNE</code>.</dd>
<dt>C (Übertrag)</dt><dd>Übertrag aus einer Addition, kein Borgen bei einer Subtraktion, herausgeschobenes Bit; <code>BCS</code> / <code>BCC</code>.</dd>
<dt>D (dezimal)</dt><dd><code>ADC</code> und <code>SBC</code> rechnen in BCD. Der Woz Monitor führt beim Start <code>CLD</code> aus.</dd>
<dt>I (Interrupts)</dt><dd>sperrt IRQ. Der Apple-1 im Originalzustand hat keine.</dd>
</dl>

## Adressierungsarten

| Art | Schreibweise | Bytes | Beispiel | Wirkung |
|---|---|---|---|---|
| implizit | `INX` | 1 | `inx` | X ← X + 1 |
| Akkumulator | `ASL A` | 1 | `asl a` | schiebt A |
| unmittelbar | `#wert` | 2 | `lda #$8D` | A ← $8D |
| Zero Page | `zp` | 2 | `lda $06` | A ← Speicher $0006 |
| Zero Page, X | `zp,X` | 2 | `lda $06,x` | |
| absolut | `adr` | 3 | `lda $D010` | |
| absolut, X / Y | `adr,X` | 3 | `lda message,x` | indizierte Tabelle |
| indirekt indiziert | `(zp),Y` | 2 | `lda ($06),y` | 16-Bit-Zeiger in der Zero Page, plus Y |
| indiziert indirekt | `(zp,X)` | 2 | `lda ($06,x)` | Zeigertabelle |
| relativ | `Marke` | 2 | `bne loop` | Verzweigung um −128 bis +127 Bytes |
| indirekt | `(adr)` | 3 | `jmp ($FFFC)` | nur `JMP` |

Die **Zero Page** (`$0000-$00FF`) ist die wertvollste: Ihre Befehle sind kürzer und schneller, und nur diese Art erlaubt Zeiger `(zp),Y`. Der Woz Monitor benutzt nur `$24-$2B`; das Integer BASIC deutlich mehr (Kapitel 16).

## Befehle nach Familien

| Familie | Befehle |
|---|---|
| Laden, Speichern | `LDA LDX LDY STA STX STY` |
| Transfers | `TAX TAY TXA TYA TSX TXS` |
| Stapel | `PHA PLA PHP PLP` |
| Arithmetik | `ADC SBC` (mit Übertrag: `CLC` vor `ADC`, `SEC` vor `SBC`) |
| Inkremente | `INC INX INY DEC DEX DEY` |
| Logik | `AND ORA EOR BIT` |
| Schiebebefehle | `ASL LSR ROL ROR` |
| Vergleiche | `CMP CPX CPY` (C = 1, wenn Register ≥ Wert, Z = 1, wenn gleich) |
| Verzweigungen | `BEQ BNE BCC BCS BMI BPL BVC BVS` |
| Sprünge | `JMP JSR RTS RTI BRK` |
| Flags | `CLC SEC CLD SED CLI SEI CLV` |
| nichts | `NOP` |

{{!Befehle (6502)}}

!!! astuce "Vorzeichenloser Vergleich"
    Nach `CMP #n` verzweigt `BCC`, wenn A < n, und `BCS`, wenn A ≥ n. Für „A > n“ vergleichen Sie mit n + 1: Das tun die Beispiele mit `cmp #'9' | $80 + 1`.

## Ein NMOS-6502, kein 65C02

Der Apple-1 hat einen originalen 6502 (NMOS), und der Emulator auch. Undokumentierte Opcodes laufen als `NOP` der richtigen Länge, und die `JAM`-Opcodes halten den Prozessor an. Der Assembler nimmt nach `.setcpu "65C02"` Befehle des 65C02 an (`BRA`, `STZ`, `PHX`, `PLX`, `PHY`, `PLY`, `INC A`…), für eine mit einem 65C02 bestückte Platine, aber sie **funktionieren nicht** im Emulator:

```asm title="c02.s (Test)"
        .setcpu "65C02"
        .org $0300
        lda #$C1        ; "A"
        bra next        ; auf einem NMOS-6502: NOP mit 2 Bytes
        lda #$D8        ; "X"
next:   jsr $FFEF
        jmp $FF1F
```

Dieses Programm gibt auf einem 65C02 `A` aus und im Emulator `X`: Dort ist `BRA` ($80) ein `NOP`, das seinen Operanden überspringt, und `LDA #$D8` wird ausgeführt.

## Die Speicherkarte auf einen Blick

| Adressen | Inhalt |
|---|---|
| `$0000-$00FF` | Zero Page (Woz Monitor: `$24-$2B`; BASIC: viel mehr) |
| `$0100-$01FF` | Stapel |
| `$0200-$027F` | Zeilenpuffer des Woz Monitors |
| `$0280-$1FFF` | frei: Ihre Programme, oft bei `$0280` oder `$0300` (im Emulator standardmäßig 8 KB RAM; 4 KB, bis `$0FFF`, bei einem Apple-1 im Originalzustand) |
| `$C000-$C0FF` | ACI-Karte (Kassette); `$C100-$C1FF`: ihr Programm |
| `$C250-$C257` | Schalter der GEN2-Karte |
| `$D010-$D013` | PIA: Tastatur und Anzeige |
| `$E000-$EFFF` | Integer BASIC (falls geladen) |
| `$FF00-$FFFF` | Woz Monitor |

Anhang A gibt die vollständige Karte.
