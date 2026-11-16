# Assembly 6502 <span class="part-sub">Il processore, l'assembler integrato e i suoi strumenti, l'input/output e le routine del Woz Monitor, esempi commentati e consigli</span> {: .part data-part="Parte III" }

# Il 6502 in breve

<p class="lead">Il 6502 dell'Apple-1 è un processore a 8 bit a 1,023 MHz, con tre registri, 56 istruzioni e 64 KB di spazio di indirizzamento. Questo capitolo dà l'essenziale per leggere e scrivere gli esempi di questa parte.</p>

## Registri

| Registro | Dimensione | Ruolo |
|---|---|---|
| **A** | 8 bit | l'accumulatore: aritmetica, confronti, letture e scritture |
| **X**, **Y** | 8 bit | i registri indice: contatori di ciclo, spostamento in una tabella |
| **S** | 8 bit | il puntatore dello stack; lo stack occupa la pagina `$0100-$01FF` |
| **PC** | 16 bit | il contatore di programma: l'indirizzo dell'istruzione successiva |
| **P** | 8 bit | i flag N V – B D I Z C |

{{!registri (6502)}}

I flag sono aggiornati da quasi tutte le istruzioni:

<dl class="ref">
<dt>N (negativo)</dt><dd>copia del bit 7 del risultato; testato da <code>BMI</code> / <code>BPL</code>.</dd>
<dt>V (overflow)</dt><dd>overflow con segno di un'addizione o sottrazione; <code>BVS</code> / <code>BVC</code>.</dd>
<dt>Z (zero)</dt><dd>il risultato è zero; <code>BEQ</code> / <code>BNE</code>.</dd>
<dt>C (riporto)</dt><dd>riporto di un'addizione, assenza di prestito in una sottrazione, bit uscito da uno scorrimento; <code>BCS</code> / <code>BCC</code>.</dd>
<dt>D (decimale)</dt><dd><code>ADC</code> e <code>SBC</code> lavorano in BCD. Il Woz Monitor esegue <code>CLD</code> all'avvio.</dd>
<dt>I (interrupt)</dt><dd>maschera gli IRQ. L'Apple-1 di serie non ne ha.</dd>
</dl>

## Modi di indirizzamento

| Modo | Sintassi | Byte | Esempio | Effetto |
|---|---|---|---|---|
| implicito | `INX` | 1 | `inx` | X ← X + 1 |
| accumulatore | `ASL A` | 1 | `asl a` | fa scorrere A |
| immediato | `#val` | 2 | `lda #$8D` | A ← $8D |
| pagina zero | `zp` | 2 | `lda $06` | A ← memoria $0006 |
| pagina zero, X | `zp,X` | 2 | `lda $06,x` | |
| assoluto | `ind` | 3 | `lda $D010` | |
| assoluto, X / Y | `ind,X` | 3 | `lda message,x` | tabella indicizzata |
| indiretto indicizzato | `(zp),Y` | 2 | `lda ($06),y` | puntatore a 16 bit in pagina zero, più Y |
| indicizzato indiretto | `(zp,X)` | 2 | `lda ($06,x)` | tabella di puntatori |
| relativo | `etichetta` | 2 | `bne loop` | salto condizionato da −128 a +127 byte |
| indiretto | `(ind)` | 3 | `jmp ($FFFC)` | solo `JMP` |

La **pagina zero** (`$0000-$00FF`) è la più preziosa: le sue istruzioni sono più corte e veloci, e solo questo modo permette i puntatori `(zp),Y`. Il Woz Monitor usa solo `$24-$2B`; l'Integer BASIC ne usa molta di più (capitolo 16).

## Istruzioni per famiglie

| Famiglia | Istruzioni |
|---|---|
| caricamento, memorizzazione | `LDA LDX LDY STA STX STY` |
| trasferimenti | `TAX TAY TXA TYA TSX TXS` |
| stack | `PHA PLA PHP PLP` |
| aritmetica | `ADC SBC` (con riporto: `CLC` prima di `ADC`, `SEC` prima di `SBC`) |
| incrementi | `INC INX INY DEC DEX DEY` |
| logica | `AND ORA EOR BIT` |
| scorrimenti | `ASL LSR ROL ROR` |
| confronti | `CMP CPX CPY` (C = 1 se registro ≥ valore, Z = 1 se uguali) |
| salti condizionati | `BEQ BNE BCC BCS BMI BPL BVC BVS` |
| salti | `JMP JSR RTS RTI BRK` |
| flag | `CLC SEC CLD SED CLI SEI CLV` |
| nulla | `NOP` |

{{!istruzioni (6502)}}

!!! astuce "Confronto senza segno"
    Dopo `CMP #n`, `BCC` salta se A < n e `BCS` se A ≥ n. Per «A > n», confrontate con n + 1: è ciò che fanno gli esempi con `cmp #'9' | $80 + 1`.

## Un 6502 NMOS, non un 65C02

L'Apple-1 ha un 6502 originale (NMOS), e così l'emulatore. Gli opcode non documentati si eseguono come `NOP` della lunghezza giusta, e gli opcode `JAM` fermano il processore. L'assembler accetta le istruzioni del 65C02 dopo `.setcpu "65C02"` (`BRA`, `STZ`, `PHX`, `PLX`, `PHY`, `PLY`, `INC A`…), per una scheda con un 65C02, ma **non funzionano** nell'emulatore:

```asm title="c02.s (test)"
        .setcpu "65C02"
        .org $0300
        lda #$C1        ; "A"
        bra next        ; su un 6502 NMOS: NOP di 2 byte
        lda #$D8        ; "X"
next:   jsr $FFEF
        jmp $FF1F
```

Questo programma stampa `A` su un 65C02, e `X` nell'emulatore: lì `BRA` ($80) è un `NOP` che salta il suo operando, e viene eseguito `LDA #$D8`.

## La mappa della memoria a colpo d'occhio

| Indirizzi | Contenuto |
|---|---|
| `$0000-$00FF` | pagina zero (Woz Monitor: `$24-$2B`; BASIC: molto di più) |
| `$0100-$01FF` | stack |
| `$0200-$027F` | buffer di riga del Woz Monitor |
| `$0280-$1FFF` | libera: i vostri programmi, spesso in `$0280` o `$0300` (8 KB di RAM per impostazione predefinita nell'emulatore; 4 KB, fino a `$0FFF`, su un Apple-1 di serie) |
| `$C000-$C0FF` | scheda ACI (cassetta); `$C100-$C1FF`: il suo programma |
| `$C250-$C257` | interruttori della scheda GEN2 |
| `$D010-$D013` | PIA: tastiera e visualizzazione |
| `$E000-$EFFF` | Integer BASIC (se caricato) |
| `$FF00-$FFFF` | Woz Monitor |

L'appendice A dà la mappa completa.
