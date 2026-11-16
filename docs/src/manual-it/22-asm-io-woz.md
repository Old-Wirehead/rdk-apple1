# La tastiera, lo schermo e il Woz Monitor

<p class="lead">L'Apple-1 ha solo quattro registri di input/output, in un PIA 6820 (o 6821), e un monitor di 256 byte. Non è molto, ma basta: questo capitolo mostra come leggere un tasto, scrivere un carattere e usare le routine di Steve Wozniak.</p>

## I quattro registri del PIA

| Indirizzo | Nome | Lettura | Scrittura |
|---|---|---|---|
| `$D010` | `KBD` | codice del tasto, bit 7 a uno; la lettura **conferma** il tasto | — |
| `$D011` | `KBDCR` | bit 7 = 1: un tasto è in attesa | configurazione (fatta dal monitor) |
| `$D012` | `DSP` | bit 7 = 1: lo schermo è occupato | carattere da visualizzare (7 bit) |
| `$D013` | `DSPCR` | — | configurazione (fatta dal monitor) |

{{!KBD}}{{!KBDCR}}{{!DSP}}

Questi nomi sono definiti in `apple1.inc`, incluso da tutti gli esempi:

@@include exemples/it/asm/apple1.inc

## Leggere un tasto

Attendere che il bit 7 di `KBDCR` diventi 1, poi leggere `KBD`:

```asm
leggi:  lda KBDCR       ; un tasto?
        bpl leggi       ; bit 7 = 0: non ancora
        lda KBD         ; codice ASCII + $80
```

La tastiera produce solo maiuscole (l'emulatore converte le minuscole), con il bit 7 a uno: `A` dà `$C1`, RETURN `$8D`, ESC `$9B`. Per sapere se è stato premuto un tasto **senza attendere**, testate solo `KBDCR`:

```asm
        lda KBDCR
        bpl niente      ; nessun tasto: continuare a lavorare
        lda KBD         ; un tasto: leggerlo (e confermarlo)
```

`jeu.s` (capitolo 23) lo usa per estrarre un numero a caso: conta in un ciclo finché il giocatore non preme un tasto.

## Scrivere un carattere

Il terminale dell'Apple-1 accetta un carattere alla volta, e solo quando è pronto: il bit 7 di `DSP` resta a 1 finché il carattere precedente non è stato visualizzato. La routine `ECHO` del monitor attende per voi. Ecco le sue quattro istruzioni, lette nel monitor dell'emulatore:

```mon
\
» FFEF.FFFFR

FFEF: 2C
FFF0: 12 D0 30 FB 8D 12 D0 60
FFF8: 00 00 00 0F 00 FF 00 00
```

Cioè `BIT DSP` / `BMI ECHO` / `STA DSP` / `RTS`. `ECHO` conserva A, X e Y. Poiché il terminale originale mostra **un solo carattere per fotogramma**, circa 60 al secondo, un programma che stampa molto è limitato dallo schermo, non dal processore; l'emulatore rispetta questo limite per impostazione predefinita (<kbd>F4</kbd> lo toglie, capitolo 2).

Che cosa fa lo schermo con ogni codice:

- **RETURN** (`$8D`, o `$0D`) comincia una nuova riga; in fondo allo schermo, tutto scorre di una riga verso l'alto.
- Gli **altri caratteri di controllo** (`$00-$1F`, con o senza bit 7) vengono ignorati: niente cancellazione, niente ritorno indietro, niente campanello.
- Le **minuscole** `$60-$7F` sono visualizzate come le maiuscole `$40-$5F`: il terminale conserva solo 6 bit.
- Dopo la 40ª colonna, il cursore passa da solo alla riga successiva.
- Non esiste un codice per cancellare lo schermo: è un pulsante della macchina (**Cancella schermo**, <kbd>Ctrl</kbd>+<kbd>L</kbd>).

## Le routine del Woz Monitor

| Indirizzo | Nome | Ingresso | Effetto | Registri |
|---|---|---|---|---|
| `$FFEF` | `ECHO` | A = carattere | lo visualizza | A, X, Y conservati |
| `$FFDC` | `PRBYTE` | A = byte | lo visualizza in 2 cifre esadecimali | A distrutto; X, Y conservati |
| `$FFE5` | `PRHEX` | A (nibble basso) | visualizza una cifra esadecimale | A distrutto; X, Y conservati |
| `$FF1F` | `GETLINE` | — | ritorno al monitor: nuova riga, poi attesa di un comando | — |
| `$FF1A` | `ESCAPE` | — | come `GETLINE`, stampando prima `\` | — |
| `$FF00` | `RESET` | — | inizializza il PIA e il monitor | — |

{{!ECHO}}{{!PRBYTE}}{{!PRHEX}}{{!GETLINE}}

Il monitor usa la pagina zero `$24-$2B` e il buffer di riga `$0200-$027F`; tutto il resto è vostro (salvo che l'Integer BASIC debba sopravvivere, capitolo 16). I suoi comandi (`300R`, `0300.031F`, `300: A9 8D`…) sono riassunti nell'appendice B.

## Avviare e terminare un programma

Un programma si avvia con il comando `R` del monitor (`300R`), che fa un `JMP` all'indirizzo, non un `JSR`. Non c'è quindi **nessun indirizzo di ritorno** sullo stack: terminate con `jmp GETLINE` (`$FF1F`), non con `RTS`.

!!! attention "RTS e BRK non restituiscono il controllo"
    Un `RTS` alla fine di un programma avviato con `R` preleva ciò che si trova sullo stack. Un `BRK` salta attraverso il vettore di interrupt della ROM, che vale `$0000` (le ultime tre parole del monitor, sopra: NMI `$0F00`, RESET `$FF00`, IRQ/BRK `$0000`): il processore esegue allora la pagina zero. In entrambi i casi, premete RESET (<kbd>Ctrl</kbd>+<kbd>R</kbd>).

Un programma chiamato dal BASIC con `CALL 768`, invece, è chiamato da un `JSR`: deve terminare con `RTS` per tornare al BASIC. Il capitolo 16 mostra come sistemare del codice macchina sotto il programma BASIC.

```mon
>» POKE 768,96
>» CALL 768
>
```

(`96` = `$60`, l'opcode di `RTS`: la subroutine più piccola possibile.)

## Il PIA, la cassetta e la scheda a colori

La scheda ACI occupa `$C000-$C0FF` (ogni accesso inverte l'uscita) e il suo programma `$C100-$C1FF`; il capitolo 5 ne spiega il funzionamento e l'appendice C i tempi. La scheda GEN2 si comanda semplicemente **leggendo** `$C250-$C257` (`bit SS_GR`); i suoi indirizzi sono in `gen2.inc`:

@@include exemples/it/asm/gen2.inc
