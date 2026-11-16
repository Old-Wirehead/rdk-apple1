# Riferimento di istruzioni e funzioni

<p class="lead">Tutte le istruzioni riconosciute dall'Integer BASIC dell'Apple-1, prese dalla tabella delle parole chiave dell'interprete, con quello che fanno davvero. Ogni esempio è stato provato.</p>

## Istruzioni

`PRINT` elenco `[;]`
{: .syntax }

{{PRINT}} mostra numeri e stringhe. `;` unisce gli elementi senza spazio (`PRINT A;B` stampa `12` per A=1, B=2); `,` passa alla colonna successiva, a passi di 8 (`PRINT 1,2,3` stampa `1       2       3`). Un `;` finale sopprime l'a capo. `PRINT` da solo va alla riga successiva.

`INPUT` `["richiesta",]` variabile `[,variabile…]`
{: .syntax }

{{INPUT}} legge un numero o una stringa. Il BASIC stampa sempre `?` dopo la richiesta: `INPUT "NOME",N$` stampa `NOME?`. Una stringa troppo lunga per il suo `DIM` dà `STR OVFL ERR`.

`LET` variabile `=` espressione
{: .syntax }

{{LET}} è facoltativo: basta `A=5`.

`IF` espressione `THEN` istruzione | numero
{: .syntax }

{{IF}} esegue l'istruzione quando l'espressione non è zero (`IF A THEN …` è permesso). `THEN 100` salta alla riga 100. **Attenzione**: solo la prima istruzione dipende dalla condizione (capitolo 18, bug 4).

`GOTO` espressione — `GOSUB` espressione — `RETURN`
{: .syntax }

{{GOTO}} e {{GOSUB}} accettano un'**espressione**: `GOTO K*100`, `GOSUB 300+K` (il «GOTO calcolato» sostituisce l'assente `ON … GOTO`). Una riga inesistente dà `BAD BRANCH ERR`. Al massimo otto {{GOSUB}} annidati (`>8 GOSUBS ERR` al nono); `RETURN` senza `GOSUB` dà `BAD RETURN ERR`.

`FOR` var `=` inizio `TO` fine `[STEP passo]` … `NEXT` var
{: .syntax }

{{FOR}}/{{NEXT}}: il corpo viene eseguito **almeno una volta**, anche se l'inizio è oltre la fine (`FOR I=5 TO 1` passa una volta). `STEP` può essere negativo. Al massimo otto cicli annidati (`>8 FORS ERR`). `NEXT` deve nominare la variabile; un `NEXT` senza `FOR` dà `BAD NEXT ERR`.

`DIM` var`(n)` `[,var$(n)…]`
{: .syntax }

{{DIM}} crea un vettore di `n` interi, numerati **da 1 a n** (`A(0)` dà `RANGE ERR`), oppure riserva `n` caratteri (al massimo 255) per una stringa. Ogni stringa va dimensionata prima di assegnarla.

`REM` testo
{: .syntax }

{{REM}}: un commento, fino alla fine della riga (un `:` dopo REM fa parte del commento).

`END`
{: .syntax }

{{END}} termina il programma. Un programma che finisce senza `END` stampa `*** END ERR`: è innocuo, ma terminate i vostri programmi con `END`.

`TAB` n
{: .syntax }

{{TAB}} è un'**istruzione** (non una funzione di PRINT come in Applesoft): `TAB 10:PRINT "*"` mette l'asterisco nella colonna 10.

`POKE` indirizzo`,`valore
{: .syntax }

{{POKE}} scrive un byte (da 0 a 255, altrimenti `>255 ERR`). Gli indirizzi sopra 32767 si scrivono come numeri negativi: indirizzo − 65536.

`CALL` indirizzo
{: .syntax }

{{CALL}} chiama un programma in linguaggio macchina, che ritorna con `RTS`. Esempio: `POKE 768,96 : CALL 768` (96 = `RTS`). `CALL -256` torna in modo pulito al Woz Monitor (`$FF00`).

## Funzioni

| Funzione | Risultato | Esempio verificato |
|---|---|---|
| {{PEEK}}`(ind)` | il byte all'indirizzo | `PEEK(76)+PEEK(77)*256` dà HIMEM (4096) |
| {{RND}}`(n)` | intero casuale da 0 a n−1; `RND(0)`: `>32767 ERR`; `RND(-5)` dà un numero negativo | `RND(6)+1`: un dado |
| {{SGN}}`(x)` | −1, 0 o 1 (anche per un numero da 256 in su) | `SGN(-5);SGN(0)` stampa `-10` |
| {{ABS}}`(x)` | valore assoluto | `ABS(-9)` = 9 |
| `KEY(n)` (solo G2 Basic) | codice del tasto premuto, senza attendere (vedi sotto) | `K=KEY(0)` |
| {{LEN}}`(a$)` | lunghezza della stringa | `LEN("ABC")` = 3 |
| {{USR}}`(x)` | chiama `$0000`: **si blocca** senza un programma preparato | — |
| {{ASC}}`(`, {{SCRN}}`(` | presenti nella tabella, ma `SYNTAX ERR` | — |

## Operatori

| Precedenza | Operatori |
|---|---|
| 1 | `-` e `+` unari, `NOT` |
| 2 | `*` `/` `MOD` |
| 3 | `+` `-` |
| 4 | `=` `#` (diverso, anche `<>`) `<` `>` `<=` `>=` |
| 5 | `AND` |
| 6 | `OR` |

{{MOD}}, {{AND}}, {{OR}} e {{NOT}}:

- La divisione **tronca verso zero**: `7/2` = 3, `-7/2` = −3. `MOD` prende il segno del dividendo: `-7 MOD 3` = −1.
- I confronti danno 1 (vero) o 0 (falso): `PRINT 5>3;5=3` stampa `10`. `NOT 0` = 1, `NOT 5` = 0.
- `AND` e `OR` sono logici, non bit a bit: `3 AND 0` = 0, `3 OR 0` = 1.
- `3*-2` è accettato e vale −6.
- `^` esiste nella tabella, ma **blocca la macchina** (capitolo 18, bug 2).
- Le stringhe si possono confrontare solo con `=` e `#`; `"ABC"<"ABD"` viene rifiutato (`SYNTAX ERR`).

## Estensioni di RDK G2 Basic

Funzionano solo con **RDK G2 Basic** (l'Integer BASIC originale risponde `SYNTAX ERR`).

`KEY(n)`
{: .syntax }

`KEY(0)` restituisce il **codice del tasto** premuto (il codice ASCII a 7 bit: 65 per `A`, 13 per Invio, 32 per la barra spaziatrice) e lo consuma, oppure `0` se non è stato premuto nessun tasto. Non attende mai, quindi un programma può continuare a girare mentre sorveglia la tastiera. `KEY(1)` (qualsiasi altro argomento non nullo) **attende** un tasto.

```basic title="Attendere un tasto senza INPUT"
10 K=KEY(0)
20 IF K=0 THEN 10
30 PRINT K
```

Nell'originale, un tasto premuto mentre un programma gira **lo ferma** (`STOPPED AT n`), e `INPUT` blocca fino a Invio. Con G2 Basic, una volta che un programma ha chiamato `KEY`, la pressione di un tasto non lo ferma più: solo `KEY` vede la tastiera, e un <kbd>Ctrl</kbd>+<kbd>C</kbd> (codice 3) letto da `KEY` ferma il programma alla chiamata successiva. Chiamate dunque `KEY` nel ciclo principale del programma.

`LOMEM=` n — `HIMEM=` n
{: .syntax }

Comandi immediati che accettano un'espressione (`LOMEM=2048+512`); dentro un programma danno `SYNTAX ERR`, come nell'originale. Sono descritti con la memoria nel capitolo 16.

`LIST` `[`a`[,`b`]]` — `DEL` a`[,`b`]`
{: .syntax }

`LIST` lista tutto, `LIST n` una riga, `LIST a,b` le righe da `a` a `b`. `DEL n` cancella una riga, `DEL a,b` le righe da `a` a `b` (se `b` è minore di `a`, solo la riga `a`). Una riga listata più lunga dello schermo va a capo come nell'originale (le righe di continuazione sono rientrate di sei spazi).

Le sottostringhe seguono l'originale: `A$(I)` con `I` da 256 in su, o negativo, dà `>255 ERR`, e `A$(I,J)` con `J` minore di `I-1` dà `STRING ERR`.

## Parole chiave che non fanno nulla

La tabella contiene `COLOR=`, `PLOT`, `HLIN`, `VLIN` e `VTAB`, ereditate dal progetto di BASIC per l'Apple II. Sull'Apple-1, `COLOR=` e `PLOT` sono accettate e non fanno nulla, `VTAB` dà `SYNTAX ERR`. Lasciatele fuori dai vostri programmi.
