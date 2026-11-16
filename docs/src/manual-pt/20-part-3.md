# Assembly 6502 <span class="part-sub">O processador, o assembler integrado e as suas ferramentas, as entradas/saídas e as rotinas do Woz Monitor, exemplos comentados e conselhos</span> {: .part data-part="Parte III" }

# O 6502 em resumo

<p class="lead">O 6502 do Apple-1 é um processador de 8 bits a 1,023 MHz, com três registos, 56 instruções e 64 KB de espaço de endereçamento. Este capítulo dá o essencial para ler e escrever os exemplos desta parte.</p>

## Registos

| Registo | Tamanho | Papel |
|---|---|---|
| **A** | 8 bits | o acumulador: aritmética, comparações, leituras e escritas |
| **X**, **Y** | 8 bits | os registos de índice: contadores de ciclo, deslocamento numa tabela |
| **S** | 8 bits | o ponteiro da pilha; a pilha ocupa a página `$0100-$01FF` |
| **PC** | 16 bits | o contador de programa: o endereço da instrução seguinte |
| **P** | 8 bits | os indicadores N V – B D I Z C |

{{!registos (6502)}}

Os indicadores são atualizados por quase todas as instruções:

<dl class="ref">
<dt>N (negativo)</dt><dd>cópia do bit 7 do resultado; testado por <code>BMI</code> / <code>BPL</code>.</dd>
<dt>V (transbordo)</dt><dd>transbordo com sinal de uma soma ou subtração; <code>BVS</code> / <code>BVC</code>.</dd>
<dt>Z (zero)</dt><dd>o resultado é zero; <code>BEQ</code> / <code>BNE</code>.</dd>
<dt>C (transporte)</dt><dd>transporte de uma soma, ausência de empréstimo numa subtração, bit que sai num deslocamento; <code>BCS</code> / <code>BCC</code>.</dd>
<dt>D (decimal)</dt><dd><code>ADC</code> e <code>SBC</code> trabalham em BCD. O Woz Monitor faz <code>CLD</code> ao arrancar.</dd>
<dt>I (interrupções)</dt><dd>mascara as IRQ. O Apple-1 de origem não tem nenhuma.</dd>
</dl>

## Modos de endereçamento

| Modo | Sintaxe | Bytes | Exemplo | Efeito |
|---|---|---|---|---|
| implícito | `INX` | 1 | `inx` | X ← X + 1 |
| acumulador | `ASL A` | 1 | `asl a` | desloca A |
| imediato | `#val` | 2 | `lda #$8D` | A ← $8D |
| página zero | `zp` | 2 | `lda $06` | A ← memória $0006 |
| página zero, X | `zp,X` | 2 | `lda $06,x` | |
| absoluto | `end` | 3 | `lda $D010` | |
| absoluto, X / Y | `end,X` | 3 | `lda message,x` | tabela indexada |
| indireto indexado | `(zp),Y` | 2 | `lda ($06),y` | ponteiro de 16 bits na página zero, mais Y |
| indexado indireto | `(zp,X)` | 2 | `lda ($06,x)` | tabela de ponteiros |
| relativo | `etiqueta` | 2 | `bne loop` | salto condicional de −128 a +127 bytes |
| indireto | `(end)` | 3 | `jmp ($FFFC)` | só `JMP` |

A **página zero** (`$0000-$00FF`) é a mais preciosa: as suas instruções são mais curtas e rápidas, e só este modo permite os ponteiros `(zp),Y`. O Woz Monitor só usa `$24-$2B`; o Integer BASIC usa muito mais (capítulo 16).

## Instruções por famílias

| Família | Instruções |
|---|---|
| carregar, guardar | `LDA LDX LDY STA STX STY` |
| transferências | `TAX TAY TXA TYA TSX TXS` |
| pilha | `PHA PLA PHP PLP` |
| aritmética | `ADC SBC` (com transporte: `CLC` antes de `ADC`, `SEC` antes de `SBC`) |
| incrementos | `INC INX INY DEC DEX DEY` |
| lógica | `AND ORA EOR BIT` |
| deslocamentos | `ASL LSR ROL ROR` |
| comparações | `CMP CPX CPY` (C = 1 se registo ≥ valor, Z = 1 se iguais) |
| saltos condicionais | `BEQ BNE BCC BCS BMI BPL BVC BVS` |
| saltos | `JMP JSR RTS RTI BRK` |
| indicadores | `CLC SEC CLD SED CLI SEI CLV` |
| nada | `NOP` |

{{!instruções (6502)}}

!!! astuce "Comparação sem sinal"
    Depois de `CMP #n`, `BCC` salta se A < n e `BCS` se A ≥ n. Para «A > n», compare com n + 1: é o que os exemplos fazem com `cmp #'9' | $80 + 1`.

## Um 6502 NMOS, não um 65C02

O Apple-1 tem um 6502 original (NMOS), e o emulador também. Os opcodes não documentados executam-se como `NOP` do comprimento certo, e os opcodes `JAM` param o processador. O assembler aceita as instruções do 65C02 depois de `.setcpu "65C02"` (`BRA`, `STZ`, `PHX`, `PLX`, `PHY`, `PLY`, `INC A`…), para uma placa equipada com um 65C02, mas elas **não funcionam** no emulador:

```asm title="c02.s (teste)"
        .setcpu "65C02"
        .org $0300
        lda #$C1        ; "A"
        bra next        ; num 6502 NMOS: NOP de 2 bytes
        lda #$D8        ; "X"
next:   jsr $FFEF
        jmp $FF1F
```

Este programa mostra `A` num 65C02, e `X` no emulador: aí `BRA` ($80) é um `NOP` que salta o seu operando, e o `LDA #$D8` é executado.

## O mapa da memória num relance

| Endereços | Conteúdo |
|---|---|
| `$0000-$00FF` | página zero (Woz Monitor: `$24-$2B`; BASIC: muito mais) |
| `$0100-$01FF` | pilha |
| `$0200-$027F` | buffer de linha do Woz Monitor |
| `$0280-$1FFF` | livre: os seus programas, muitas vezes em `$0280` ou `$0300` (8 KB de RAM por omissão no emulador; 4 KB, até `$0FFF`, num Apple-1 de origem) |
| `$C000-$C0FF` | placa ACI (cassete); `$C100-$C1FF`: o seu programa |
| `$C250-$C257` | comutadores da placa GEN2 |
| `$D010-$D013` | PIA: teclado e visualização |
| `$E000-$EFFF` | Integer BASIC (se carregado) |
| `$FF00-$FFFF` | Woz Monitor |

O apêndice A dá o mapa completo.
