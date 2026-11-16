# Esempi commentati

<p class="lead">Otto programmi, dal più semplice al più completo. Si trovano nella cartella <code>docs/exemples/it/asm/</code> della distribuzione; ciascuno è stato assemblato con l'assembler integrato ed eseguito nell'emulatore. Gli output mostrati qui sono quelli dell'emulatore.</p>

Per provarli: aprite il file nell'editor e fate clic su **▶ Esegui**, oppure, dalla riga di comando, `apple1-emu asm hello.s -o hello.wav` e poi riproducete la cassetta (capitolo 5). Tenete `apple1.inc` (e `gen2.inc`) nella stessa cartella dei sorgenti.

| File | Byte | Indirizzo | Che cosa mostra |
|---|---|---|---|
| `hello.s` | 32 | `$0300` | una stringa, `ECHO`, etichette locali, ritorno al monitor |
| `clavier.s` | 32 | `$0300` | leggere la tastiera, confrontare caratteri |
| `hexa.s` | 64 | `$0300` | puntatore in pagina zero, `(zp),Y`, addizione a 16 bit, `PRBYTE` |
| `decimal.s` | 117 | `$0300` | numeri a 16 bit, sottrazioni, tabelle di parole |
| `jeu.s` | 240 | `$0300` | un gioco completo: caso, subroutine, messaggi |
| `macros.s` | 132 | `$0300` | macro, `.local`, `.repeat`, `.if`, tabelle calcolate |
| `gen2-lores.s` | 110 | `$0C00` | la scheda GEN2 in LORES |
| `gen2-hires.s` | 94 | `$0C00` | la scheda GEN2 in HIRES |

Le versioni originali di questi programmi sono state assemblate anche con ca65 (stessi byte, capitolo 21); le versioni italiane differiscono solo nei commenti e nei testi.

## `hello.s`: stampare un messaggio

@@include exemples/it/asm/hello.s

X percorre la stringa fino al byte 0 messo da `.asciiz`. Il `bne @next` dopo `inx` non è mai falso per una stringa di meno di 256 caratteri: è un salto corto e sicuro. Il programma termina con `jmp GETLINE`, mai con `RTS` (capitolo 22).

```mon
\
» 300R
0300: A2
CIAO, APPLE-1!
```

Il monitor stampa prima `0300: A2`, il byte all'indirizzo di partenza, poi il programma prende il controllo.

## `clavier.s`: l'eco dei tasti

@@include exemples/it/asm/clavier.s

`cmp #'0' | $80` confronta con il codice della cifra 0, con il bit 7 a uno come lo fornisce la tastiera. `bcc` («minore») e `bcs` («maggiore o uguale») delimitano le cifre. Con i tasti `A`, `B`, `1`, `2` poi ESC, lo schermo mostra:

```text
0300: ADAB1122
\
```

L'`AD` viene dal monitor (`lda KBDCR` comincia con l'opcode `$AD`); `AB`, poi `11` e `22` vengono dal programma; ESC restituisce il controllo al monitor, che stampa `\`.

## `hexa.s`: un dump come nel monitor

@@include exemples/it/asm/hexa.s

`PTR` e `PTR+1` formano un **puntatore a 16 bit** in pagina zero, byte basso per primo; `lda (PTR),y` legge il byte all'indirizzo PTR + Y. L'addizione `PTR += 8` mostra il riporto all'opera: `clc`, `adc #8`, e se esce un riporto (`bcc` non preso), `inc PTR+1`. Il risultato è l'inizio del Woz Monitor:

```text
FF00: D8 58 A0 7F 8C 12 D0 A9
FF08: A7 8D 11 D0 8D 13 D0 C9
```

## `decimal.s`: stampare un numero in decimale

@@include exemples/it/asm/decimal.s

Il 6502 non sa dividere: `prdec` sottrae 10000 quante più volte possibile (il conteggio dà la cifra delle decine di migliaia), poi 1000, 100 e 10. La sottrazione a 16 bit si fa in due tempi, `sbc` sul byte basso poi su quello alto, collegati dal riporto. `FLAG` sopprime gli zeri iniziali. Output:

```text
0 7 42 1976 6502 65535
```

!!! astuce "Conservare X in una subroutine"
    `prdec` usa X per sé, mentre il ciclo principale ne ha bisogno: lo salva con `txa` / `pha` all'inizio e lo ripristina con `pla` / `tax` alla fine. Il 6502 NMOS non ha `PHX`.

## `jeu.s`: indovinare la cifra

@@include exemples/it/asm/jeu.s

Il caso viene dal giocatore: X conta senza sosta (`@wait`) fino al primo tasto, poi il resto della divisione per 10 dà la cifra segreta. `prmsg` riceve l'indirizzo di un messaggio in **Y:X** (byte alto in Y), lo mette nel puntatore `MSG` e lo stampa con `(MSG),y`. Una partita, digitando `x` poi le cifre in ordine:

```text
HO SCELTO UNA CIFRA. PREMI UN TASTO.
LA TUA CIFRA? 0 PIU ALTO.
LA TUA CIFRA? 1 PIU ALTO.
…
LA TUA CIFRA? 5 PIU ALTO.
LA TUA CIFRA? 6 VINTO IN 7 TENTATIVI!
```

## `macros.s`: lasciar lavorare l'assembler

@@include exemples/it/asm/macros.s

`putc` e `print` generano codice a ogni chiamata; `.local` impedisce che le due chiamate di `print` definiscano due volte `loop`. `.repeat 3` srotola tre volte la riga di asterischi (ciascuna con la sua etichetta anonima), `.if` sceglie il messaggio secondo `WIDTH`, e la tabella `squares` è calcolata al momento dell'assemblaggio. Output:

```text
MACRO E .REPEAT
********************
********************
********************
RIGHE CORTE
```

Cambiate `WIDTH = 20` in `40`: l'editor riassembla subito, le righe si allungano e il messaggio diventa `RIGHE LUNGHE`.

## `gen2-lores.s`: colore in bassa risoluzione

Questi due programmi richiedono la scheda GEN2 (**⚙ Preferenze ▸ Scheda colore Uncle Bernie GEN2**, capitolo 6) e partono da `$0C00`, sopra la pagina LORES.

@@include exemples/it/asm/gen2-lores.s

Le due tabelle `rows_lo` e `rows_hi` sono calcolate da `.repeat` con la formula degli indirizzi di riga del capitolo 6. Ogni byte porta due blocchi, da qui il nibble copiato nella metà alta e in quella bassa. Le istruzioni `bit SS_…` sono **letture**: è così che si cambiano i modi della scheda.

![gen2-lores.s: i sedici colori LORES, due colonne ciascuno, sul monitor a colori.](img/it/brut/gen2-lores.png "large")

## `gen2-hires.s`: alta risoluzione

@@include exemples/it/asm/gen2-hires.s

In HIRES, la pagina `$2000-$3FFF` è divisa in blocchi di 128 byte i cui primi 40 byte vanno nel terzo superiore dello schermo, i 40 successivi al centro e i 40 dopo ancora in basso. Il programma riempie ogni terzo con un motivo diverso, alternando due byte secondo la parità della colonna: il risultato sono tre bande di colore.

![gen2-hires.s: tre bande di colore in HIRES: viola, verde e blu.](img/it/brut/gen2-hires.png "large")
