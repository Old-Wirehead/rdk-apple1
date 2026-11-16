# Messaggi di errore e limiti

## Messaggi

Un errore stampa `*** MESSAGGIO ERR`, seguito, quando un programma è in esecuzione, da `STOPPED AT n`. Una riga rifiutata all'inserimento non viene memorizzata: ridigitatela.

| Messaggio | Causa | Esempio provato |
|---|---|---|
| {{SYNTAX ERR}} | istruzione sconosciuta, nome di variabile non valido, `:` in modo diretto | `CNT=7`, `GET A$`, `X=5:PRINT X` in modo diretto |
| {{>32767 ERR}} | overflow, divisione per zero, numero di riga > 32767 | `200*200`, `10/0`, `RND(0)`, `32768 PRINT` |
| {{TOO LONG ERR}} | riga troppo complessa (espressione annidata troppo) o troppo lunga | 13 parentesi annidate |
| {{MEM FULL ERR}} | non c'è più posto tra variabili e programma | `DIM A(3000)` in 8 KB |
| {{TOO MANY PARENS ERR}} | parentesi (previsto dall'interprete; mascherato dal bug 3) | — |
| {{STRING ERR}} | stringa usata male | — |
| {{END ERR}} | il programma finisce senza `END` | `10 PRINT "X"` poi `RUN` |
| {{BAD BRANCH ERR}} | `GOTO`/`GOSUB` verso una riga inesistente | `GOTO 500` |
| {{>8 GOSUBS ERR}} | più di 8 `GOSUB` annidati | ricorsione: 9 chiamate |
| {{BAD RETURN ERR}} | `RETURN` senza `GOSUB` | `RUN 30` su un `RETURN` |
| {{>8 FORS ERR}} | più di 8 `FOR` annidati, o il bug 1 | 9 cicli su una riga |
| {{BAD NEXT ERR}} | `NEXT` senza il `FOR` corrispondente | `20 NEXT I` da solo |
| {{STOPPED AT}} | è stato premuto un tasto durante l'esecuzione | — |
| {{>255 ERR}} | valore > 255: `POKE`, `DIM` di una stringa | `POKE 768,300`, `DIM B$(256)` |
| {{RANGE ERR}} | indice del vettore fuori dai limiti | `A(0)`, `A(3)` dopo `DIM A(2)` |
| {{DIM ERR}} | vettore o stringa dimensionati di nuovo | — |
| {{STR OVFL ERR}} | stringa più lunga del suo `DIM` | `A$="ABCDE"` con `DIM A$(3)` |
| {{RETYPE LINE}} | risposta non valida a `INPUT` (lettere per un numero): ridigitare | — |

Le righe con «—» vengono dalla tabella dei messaggi dell'interprete; non sono state provocate nei nostri test.

## Limiti

| Limite | Valore |
|---|---|
| interi | da −32767 a 32767 |
| numeri di riga | da 0 a 32767 |
| nomi di variabile | lettera + cifra facoltativa |
| nomi di stringa | una lettera + `$` |
| lunghezza di una stringa | 255 |
| dimensioni di un vettore | 1, indici da 1 a n |
| `FOR` annidati | 8 |
| `GOSUB` annidati | 8 |
| parentesi annidate | 12 |
| lunghezza di una riga digitata | 120 caratteri passano, 141 sono rifiutati |


## Risparmiare memoria

Con 8 KB, il BASIC ha circa 2 KB per il programma e le variabili. Qualche modo per guadagnarne:

<ol class="steps">
<li><strong>Abbassare LOMEM</strong> se non avete codice macchina: <code>LOMEM=768</code> (<code>$0300</code>) dà alle variabili 1280 byte in più. Provato: <code>DIM T(1200)</code> fallisce con <code>MEM FULL</code> con il LOMEM predefinito, e riesce dopo <code>LOMEM=768</code>. I porting di *101 BASIC Computer Games* fatti durante lo sviluppo fanno proprio così.</li>
<li><strong>Scrivere in modo compatto</strong>: il BASIC conserva gli spazi e i <code>REM</code>. <code>LIST</code> reindenta da solo, e una riga lunga con più istruzioni costa meno di più righe.</li>
<li><strong>Riutilizzare vettori e stringhe</strong> invece di dimensionarne di nuovi.</li>
<li><strong>Aggiungere RAM</strong>: le impostazioni ⚙ di un programma, <code>--ram 16</code>, o un'espansione vera. Un programma oltre 8 KB richiede una macchina da 12, 16 o 32 KB: <code>apple1-emu basic --ram 16</code> lo colloca in cima a quella memoria.</li>
</ol>
