# Consigli, trappole e limiti

<p class="lead">Ciò che si impara scrivendo programmi per l'Apple-1, prima nell'emulatore e poi sulla macchina vera.</p>

## Dove mettere un programma

| Indirizzo | Per |
|---|---|
| `$0280` | il più in basso possibile: subito dopo il buffer del monitor |
| `$0300` | la scelta abituale (e l'indirizzo predefinito dell'assembler) |
| `$0C00` | con la scheda GEN2 in LORES o testo (pagine `$0400-$0BFF` lasciate libere per lo schermo) |
| sotto HIMEM | con l'Integer BASIC: riservate lo spazio (capitolo 16) |

Per la pagina zero, gli esempi usano `$06-$09`, che il monitor lascia liberi. Se il programma deve convivere con il BASIC, scegliete indirizzi che il BASIC non usa, oppure salvateli e ripristinateli.

## Convivere con la portata dei salti

Un salto condizionato arriva solo a 127 byte in avanti o 128 indietro. Quando l'assembler risponde `salto troppo lontano`, invertite la condizione e saltate un `JMP`:

```asm
        beq :+          ; invece di: bne lontano
        jmp lontano
:
```

## Le stringhe e il bit 7

La tastiera restituisce codici con il bit 7 a uno, e il monitor confronta con quei codici. Lo schermo invece ignora il bit 7: `.asciiz "CIAO"` si visualizza benissimo. Per **confrontare** un tasto, mettete il bit 7 nella costante: `cmp #'Q' | $80`. Le minuscole di una stringa sono visualizzate in maiuscolo.

## Attendere un po'

A 1,023 MHz (0,96 MHz effettivi), un ciclo `dex` / `bne` costa 5 cicli macchina per giro:

```asm
pausa:
        ldy #78         ; 78 x 256 giri da 5 cicli: circa 0,1 s
:       ldx #0
:       dex
        bne :-
        dey
        bne :--
        rts
```

L'emulatore a velocità reale rispetta queste durate; a velocità rapida (<kbd>F3</kbd>) spariscono. I cicli di temporizzazione dell'ACI si contano allo stesso modo in cicli (appendice C).

## Fare il debug in modo efficace

- Mettete i punti di arresto su un'**etichetta** (`start`, `prdec`, `start@done`) invece che su un indirizzo: resta giusto quando il codice si sposta.
- Scavalcate `jsr ECHO` con **↪ Scavalca**: altrimenti seguirete il ciclo di attesa dello schermo istruzione per istruzione.
- Se l'Apple-1 non risponde più dopo il vostro programma, guardate PC: in pagina zero, è quasi sempre un `BRK` o un `RTS` sperduto (capitolo 22).
- Il **dump** mostra la pagina zero: lì si vedono i vostri puntatori, byte basso per primo.

![Il debugger fermo in un sorgente 6502: registri, flag, disassemblato con etichette e memoria.](img/it/brut/debogueur-asm.png "large")

## Verso il vero Apple-1

<ol class="steps">
<li>Assemblate e provate nell'emulatore, <strong>a velocità reale</strong> e con la visualizzazione limitata a 60 caratteri al secondo.</li>
<li>Esportate la cassetta (<strong>📼 Esporta WAV</strong>, o <code>-o prog.aif</code>): l'AIFF è il formato più sicuro se riproducete da uno smartphone.</li>
<li>Sull'Apple-1: <code>C100R</code>, poi l'intervallo indicato (<code>0300.031FR</code>), avviate la riproduzione, poi <code>300R</code>.</li>
<li>Senza ACI: il dump <code>.hex</code> si può digitare (o incollare in un terminale seriale) direttamente nel Woz Monitor; la sua ultima riga avvia il programma.</li>
</ol>

## Limiti dell'assembler integrato

- Un solo blocco in uscita, nell'ordine del sorgente: i segmenti non hanno effetto, e non c'è linker. Per codice distribuito su più zone, usate ca65 e ld65.
- Niente operatore modulo, niente `.proc`, `.scope`, `.struct` o `.enum`: sono direttive di ca65 che l'assembler integrato non conosce.
- Le istruzioni del 65C02 vengono assemblate, ma il processore emulato è un 6502 NMOS: nell'emulatore non funzionano (capitolo 20).
- Nel **listato**, una riga prodotta da una macro perde talvolta lo spazio tra istruzione e operando (`ldastring, x` per `lda string,x`). È solo un difetto di visualizzazione del listato: i byte sono giusti.
