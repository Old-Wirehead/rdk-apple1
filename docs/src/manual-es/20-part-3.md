# Ensamblador 6502 <span class="part-sub">El procesador, el ensamblador integrado y sus herramientas, la entrada/salida y las rutinas del Woz Monitor, ejemplos comentados y consejos</span> {: .part data-part="Parte III" }

# El 6502 en resumen

<p class="lead">El 6502 del Apple-1 es un procesador de 8 bits a 1,023 MHz, con tres registros, 56 instrucciones y 64 KB de espacio de direcciones. Este capítulo da lo esencial para leer y escribir los ejemplos de esta parte.</p>

## Registros

| Registro | Tamaño | Papel |
|---|---|---|
| **A** | 8 bits | el acumulador: aritmética, comparaciones, lecturas y escrituras |
| **X**, **Y** | 8 bits | los registros índice: contadores de bucle, desplazamiento en una tabla |
| **S** | 8 bits | el puntero de pila; la pila ocupa la página `$0100-$01FF` |
| **PC** | 16 bits | el contador de programa: la dirección de la instrucción siguiente |
| **P** | 8 bits | los indicadores N V – B D I Z C |

{{!registros (6502)}}

Casi todas las instrucciones actualizan los indicadores:

<dl class="ref">
<dt>N (negativo)</dt><dd>copia del bit 7 del resultado; lo prueban <code>BMI</code> / <code>BPL</code>.</dd>
<dt>V (desbordamiento)</dt><dd>desbordamiento con signo de una suma o resta; <code>BVS</code> / <code>BVC</code>.</dd>
<dt>Z (cero)</dt><dd>el resultado es cero; <code>BEQ</code> / <code>BNE</code>.</dd>
<dt>C (acarreo)</dt><dd>acarreo de una suma, ausencia de préstamo en una resta, bit desplazado hacia fuera; <code>BCS</code> / <code>BCC</code>.</dd>
<dt>D (decimal)</dt><dd><code>ADC</code> y <code>SBC</code> trabajan en BCD. El Woz Monitor hace <code>CLD</code> al arrancar.</dd>
<dt>I (interrupciones)</dt><dd>enmascara IRQ. El Apple-1 de serie no tiene.</dd>
</dl>

## Modos de direccionamiento

| Modo | Sintaxis | Bytes | Ejemplo | Efecto |
|---|---|---|---|---|
| implícito | `INX` | 1 | `inx` | X ← X + 1 |
| acumulador | `ASL A` | 1 | `asl a` | desplaza A |
| inmediato | `#val` | 2 | `lda #$8D` | A ← $8D |
| página cero | `zp` | 2 | `lda $06` | A ← memoria $0006 |
| página cero, X | `zp,X` | 2 | `lda $06,x` | |
| absoluto | `dir` | 3 | `lda $D010` | |
| absoluto, X / Y | `dir,X` | 3 | `lda message,x` | tabla indexada |
| indirecto indexado | `(zp),Y` | 2 | `lda ($06),y` | puntero de 16 bits en página cero, más Y |
| indexado indirecto | `(zp,X)` | 2 | `lda ($06,x)` | tabla de punteros |
| relativo | `etiqueta` | 2 | `bne loop` | bifurcación de −128 a +127 bytes |
| indirecto | `(dir)` | 3 | `jmp ($FFFC)` | solo `JMP` |

La **página cero** (`$0000-$00FF`) es la más valiosa: sus instrucciones son más cortas y rápidas, y solo este modo permite los punteros `(zp),Y`. El Woz Monitor solo usa `$24-$2B`; el Integer BASIC usa mucho más (capítulo 16).

## Instrucciones por familias

| Familia | Instrucciones |
|---|---|
| carga, almacenamiento | `LDA LDX LDY STA STX STY` |
| transferencias | `TAX TAY TXA TYA TSX TXS` |
| pila | `PHA PLA PHP PLP` |
| aritmética | `ADC SBC` (con acarreo: `CLC` antes de `ADC`, `SEC` antes de `SBC`) |
| incrementos | `INC INX INY DEC DEX DEY` |
| lógica | `AND ORA EOR BIT` |
| desplazamientos | `ASL LSR ROL ROR` |
| comparaciones | `CMP CPX CPY` (C = 1 si registro ≥ valor, Z = 1 si son iguales) |
| bifurcaciones | `BEQ BNE BCC BCS BMI BPL BVC BVS` |
| saltos | `JMP JSR RTS RTI BRK` |
| indicadores | `CLC SEC CLD SED CLI SEI CLV` |
| nada | `NOP` |

{{!instrucciones (6502)}}

!!! astuce "Comparación sin signo"
    Tras `CMP #n`, `BCC` bifurca si A < n y `BCS` si A ≥ n. Para «A > n», compare con n + 1: es lo que hacen los ejemplos con `cmp #'9' | $80 + 1`.

## Un 6502 NMOS, no un 65C02

El Apple-1 tiene un 6502 original (NMOS), y el emulador también. Los opcodes no documentados se ejecutan como `NOP` de la longitud correcta, y los opcodes `JAM` detienen el procesador. El ensamblador acepta las instrucciones del 65C02 tras `.setcpu "65C02"` (`BRA`, `STZ`, `PHX`, `PLX`, `PHY`, `PLY`, `INC A`…), para una placa equipada con un 65C02, pero **no funcionan** en el emulador:

```asm title="c02.s (prueba)"
        .setcpu "65C02"
        .org $0300
        lda #$C1        ; "A"
        bra next        ; en un 6502 NMOS: NOP de 2 bytes
        lda #$D8        ; "X"
next:   jsr $FFEF
        jmp $FF1F
```

Este programa muestra `A` en un 65C02, y `X` en el emulador: allí `BRA` ($80) es un `NOP` que salta su operando, y se ejecuta el `LDA #$D8`.

## El mapa de memoria de un vistazo

| Direcciones | Contenido |
|---|---|
| `$0000-$00FF` | página cero (Woz Monitor: `$24-$2B`; BASIC: mucho más) |
| `$0100-$01FF` | pila |
| `$0200-$027F` | búfer de línea del Woz Monitor |
| `$0280-$1FFF` | libre: sus programas, a menudo en `$0280` o `$0300` (8 KB de RAM por defecto en el emulador; 4 KB, hasta `$0FFF`, en un Apple-1 de serie) |
| `$C000-$C0FF` | tarjeta ACI (casete); `$C100-$C1FF`: su programa |
| `$C250-$C257` | conmutadores de la tarjeta GEN2 |
| `$D010-$D013` | PIA: teclado y pantalla |
| `$E000-$EFFF` | Integer BASIC (si está cargado) |
| `$FF00-$FFFF` | Woz Monitor |

El apéndice A da el mapa completo.
