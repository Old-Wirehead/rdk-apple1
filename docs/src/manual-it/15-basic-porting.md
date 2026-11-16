# Portare un programma da Applesoft o Microsoft BASIC

<p class="lead">La maggior parte dei listati degli anni '70 e '80 è scritta in Microsoft BASIC (Applesoft, il BASIC di «101 BASIC Computer Games»). Si possono portare sull'Apple-1, a patto di conoscere le equivalenze. Ogni riga indicata qui sotto come «rifiutata» è stata digitata nel BASIC vero.</p>

## Equivalenze

| Applesoft / Microsoft | Integer BASIC dell'Apple-1 | Nota |
|---|---|---|
| `ON X GOTO 100,200,300` | `GOTO 100*X` (se i numeri di riga lo permettono) o una serie di `IF X=1 THEN 100` | rifiutata |
| `GET A$` | `INPUT A$` | rifiutata |
| `DATA` / `READ` / `RESTORE` | vettore riempito con assegnazioni, o una stringa tagliata | rifiutata |
| `MID$(A$,I,N)` | `A$(I,I+N-1)` | rifiutata |
| `LEFT$(A$,N)` / `RIGHT$(A$,N)` | `A$(1,N)` / `A$(LEN(A$)-N+1)` | — |
| `A$+B$` | `A$(LEN(A$)+1)=B$` | — |
| `CHR$(N)`, `ASC(A$)` | stringa di riferimento (capitolo 18, bug 6) | rifiutata |
| `STR$(N)`, `VAL(A$)` | direttamente `PRINT N`; `INPUT N` | rifiutata |
| `INT(X)` | inutile: tutto è intero | rifiutata |
| `RND(1)` (reale da 0 a 1) | `RND(N)` (intero da 0 a N−1) | — |
| `X^2` | `X*X` | si blocca (bug 2) |
| `HOME` | 24 × `PRINT`, o CLEAR SCREEN (<kbd>Ctrl</kbd>+<kbd>L</kbd>) a mano | rifiutata |
| `PRINT TAB(10);"X"` | `TAB 10:PRINT "X"` | — |
| `STOP` | `END` | rifiutata |
| `DEF FN` | `GOSUB` | — |
| `A<>B` | `A#B` (anche `<>` è accettato) | — |
| `IF A THEN X:Y` | condizione invertita + salto (bug 4) | trappola |
| `DIM A(10,10)` | `DIM A(121)` e `A(I*11+J+1)` | — |
| `A(0)` | gli indici partono da 1 | `RANGE ERR` |
| `SCORE`, `NAME$` | `S`, `N$`: una lettera (+ cifra) | `SYNTAX ERR` |
| numeri reali | interi ×10 o ×100 (virgola fissa) | — |

{{!porting}}

## Il metodo

<ol class="steps">
<li><strong>Elencate le variabili</strong> del programma originale e assegnate a ciascuna una lettera (+ cifra). Annotate la tabella in <code>REM</code> all'inizio.</li>
<li><strong>Sostituite i DATA</strong> con assegnazioni in una subroutine di inizializzazione, o con una stringa che si taglia.</li>
<li><strong>Trovate ogni <code>IF … THEN … : …</code></strong> e riscrivetelo con un salto (bug 4).</li>
<li><strong>Trovate ogni uscita da un <code>FOR</code> con <code>GOTO</code></strong> e sostituitela con «valore finale + <code>NEXT</code>» (bug 1).</li>
<li><strong>Spostate di 1 gli indici dei vettori</strong> se il programma usa l'indice 0.</li>
<li><strong>Controllate nell'editor</strong>: segnala durante la digitazione le parole che l'Integer BASIC non conosce. <strong>▶ Esegui</strong> digita il programma: ogni riga rifiutata compare sullo schermo con <code>SYNTAX ERR</code>.</li>
<li><strong>Misurate la memoria</strong>: <code>apple1-emu basic prog.bas</code> dà la dimensione e la RAM necessaria. Troppo grande per 8 KB? <code>LOMEM=768</code> (capitolo 17), o una macchina da 16 KB.</li>
<li><strong>Esportate</strong> la cassetta (<strong>📼 Esporta WAV</strong>) e annotate gli intervalli per la macchina vera.</li>
</ol>

## Un esempio di porting

Un frammento tipico in Microsoft BASIC:

```text title="Originale (Microsoft BASIC)"
100 READ N$,S
110 IF S>HI THEN HI=S:BEST$=N$
120 IF N$<>"END" THEN 100
130 PRINT "MIGLIORE: ";BEST$;" (";HI;")"
140 DATA "ANNA",12,"BRUNO",17,"CLARA",9,"END",0
```

Sull'Apple-1: niente `READ`/`DATA`, niente nomi lunghi, e l'`IF` della riga 110 assegnerebbe `BEST$` a ogni passaggio (bug 4).

```basic title="Porting in Integer BASIC (provato)"
10 DIM N$(10),B$(10),S(3)
20 S(1)=12:S(2)=17:S(3)=9
30 H=0
40 FOR I=1 TO 3
50 GOSUB 200
60 IF S(I)<=H THEN 80
70 H=S(I):B$=N$
80 NEXT I
90 PRINT "MIGLIORE: ";B$;" (";H;")"
100 END
200 IF I=1 THEN N$="ANNA"
210 IF I=2 THEN N$="BRUNO"
220 IF I=3 THEN N$="CLARA"
230 RETURN
```

```mon
>» RUN
MIGLIORE: BRUNO (17)
```

## Lezioni dai porting

Otto giochi di *101 BASIC Computer Games* (DEC, 1975) sono stati portati così durante lo sviluppo dell'emulatore: BAGLES, MUGWMP, HEX, BULCOW, STARS, TRAP, CHOMP e WAR. Ciascuno è stato giocato nell'emulatore, esportato su cassetta e ricaricato attraverso la ROM dell'ACI. Questi porting non sono distribuiti (i listati originali non sono liberi), ma hanno fissato le regole di questo capitolo: variabili di una lettera, vettori che partono da 1, niente `READ`/`DATA`, nessun `GOTO` fuori da un `FOR`, una sola istruzione governata da `IF`, e `LOMEM=768` per i più grandi.
