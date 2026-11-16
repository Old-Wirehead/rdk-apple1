# Bug noti e come aggirarli

<p class="lead">L'Integer BASIC dell'Apple-1 fu scritto senza assembler, nel 1976, e non fu mai corretto. Ecco i suoi difetti noti, ciascuno riprodotto nell'emulatore con il BASIC vero, e come evitarli.</p>

<span class="verified">verificato</span>: riprodotto nell'emulatore con il vero Integer BASIC, con una trascrizione a sostegno. <span class="unverified">non verificato</span>: segnalato da una fonte, non riprodotto. Il bug n corrisponde alla sezione 18.n. Poiché l'emulatore esegue il codice originale istruzione per istruzione, un bug riprodotto qui esiste anche su un vero Apple-1.

## Uscire da un ciclo FOR con GOTO <span class="verified">verificato</span>

Un ciclo da cui si esce con `GOTO` resta «aperto». Se il programma ripassa dal `FOR`, viene impilata una nuova voce; al nono passaggio: `>8 FORS ERR`.

```basic
10 N=0
20 FOR I=1 TO 10
30 IF I=2 THEN GOTO 50
40 NEXT I
50 N=N+1:IF N<20 THEN 20
60 PRINT "FATTO ";N
70 END
```

```mon
>» RUN
*** >8 FORS ERR
STOPPED AT 20
```

!!! astuce "Aggiramento"
    Non uscite mai da un `FOR` con `GOTO`: date alla variabile il suo valore finale e passate da `NEXT`. Qui: `30 IF I=2 THEN I=10:GOTO 40`. Il programma stampa allora `FATTO 20`. I porting di *101 BASIC Computer Games* seguono questa regola.

## L'operatore ^ blocca la macchina <span class="verified">verificato</span>

`^` (potenza) è nella tabella dell'interprete, ma `PRINT 2^3` non ritorna mai: solo RESET vi tira fuori.

!!! astuce "Aggiramento"
    Moltiplicate: `2*2*2`, o un ciclo `P=1:FOR I=1 TO N:P=P*X:NEXT I`.

## Il messaggio «PPED AT» <span class="verified">verificato</span>

Quando lo stack degli operandi dell'interprete trabocca (un'espressione annidata troppo in modo diretto), il BASIC dovrebbe stampare un messaggio di errore, ma punta **3 byte troppo avanti** nella tabella dei messaggi (il disassemblato di Eric Smith lo annota: `ldy #erri_stopped_at+3 ; The "+3" is a bug!`). Quello che leggete è la fine di `STOPPED AT`:

```mon
>» PRINT 1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1)))))))))))))
14
*** PPED AT  ERR
STOPPED AT 65535
```

!!! astuce "Aggiramento"
    Spezzate le espressioni lunghe con variabili intermedie. Oltre 12 livelli di parentesi, una riga di programma viene comunque rifiutata (`TOO LONG ERR`).

## IF … THEN governa solo la prima istruzione <span class="verified">verificato</span>

In `IF A=2 THEN PRINT "X":PRINT "Y"`, solo `PRINT "X"` dipende dalla condizione: `PRINT "Y"` viene **sempre** eseguito (con A=1 il programma stampa `Y`). È la trappola più comune quando si fa il porting di un programma scritto per un altro BASIC.

!!! astuce "Aggiramento"
    Invertite la condizione e saltate il blocco:

    ```basic
    80 IF A#2 THEN 90
    85 PRINT "X":PRINT "Y"
    90 REM CONTINUA
    ```

## −32768 <span class="verified">verificato</span>

`X=-32767-1` stampa un `-` poi `*** >32767 ERR`. Il valore più piccolo utilizzabile è −32767.

!!! astuce "Aggiramento"
    Restate entro −32767 … 32767. Per leggere una parola a 16 bit con `PEEK`, scomponetela: `PEEK(A)+PEEK(A+1)*256` trabocca sopra 32767; controllate prima il byte alto (`IF PEEK(A+1)>127 THEN …`).

## ASC( non funziona <span class="verified">verificato</span>

`ASC("A")` è nella tabella delle parole chiave, ma ogni riga che lo usa viene rifiutata: `SYNTAX ERR`. Lo stesso vale per `SCRN(`.

!!! astuce "Aggiramento (provato)"
    Cercate il carattere in una stringa di riferimento. La riga 60 chiude il ciclo (`K=26:NEXT K`) per evitare il bug 1:

```basic title="La posizione di una lettera, senza ASC"
10 DIM C$(26),L$(1)
20 C$="ABCDEFGHIJKLMNOPQRSTUVWXYZ"
30 INPUT "LETTERA",L$
40 FOR K=1 TO 26:IF C$(K,K)=L$ THEN 60:NEXT K
50 PRINT "NON E UNA LETTERA":END
60 PRINT "POSIZIONE ";K:K=26:NEXT K
70 GOTO 30
```

```mon
LETTERA?» W
POSIZIONE 23
LETTERA?» E
POSIZIONE 5
LETTERA?» 5
NON E UNA LETTERA
```

## Parole chiave grafiche che non fanno nulla <span class="verified">verificato</span>

`COLOR=`, `PLOT` e `HLIN` sono accettate e non fanno nulla; `VLIN` e `VTAB` danno `SYNTAX ERR`. Sono resti del BASIC pensato per l'Apple II.

!!! astuce "Aggiramento"
    Per posizionare il testo, usate `TAB n` e `PRINT` vuoti. Per disegnare a colori, la scheda GEN2 si pilota con `POKE` (capitolo 6).

## Il bug dei 251 caratteri dell'Apple II <span class="verified">assente</span>

Una versione successiva dell'Integer BASIC (Apple II) si blocca su certe righe di 251 caratteri o più. Sull'Apple-1 la riga digitata è limitata molto prima (circa 128 caratteri): il bug non può verificarsi.

## STOPPED AT 65535 <span class="verified">verificato</span>

Un errore causato da una riga **rifiutata durante l'inserimento** (annidata troppo) o dal bug 3 in modo diretto è seguito da `STOPPED AT 65535`: il BASIC crede di essere in un programma e stampa un numero di riga assurdo. Non ha conseguenze; il programma in memoria non viene toccato.

## USR blocca la macchina <span class="verified">verificato</span>

`X=USR(1)` salta a `$0000`, dove di norma non c'è codice: la macchina si blocca. RESET poi `E2B3R` riportano al BASIC senza perdere il programma.

!!! astuce "Aggiramento"
    Chiamate i vostri programmi in linguaggio macchina con `CALL indirizzo`; passate i valori con `POKE` e `PEEK`.

## HIMEM= non può scendere <span class="verified">verificato</span>

`HIMEM=3072` (e qualsiasi valore sotto quello attuale) viene **ignorato in silenzio**, anche dopo `SCR`, perché l'interprete confronta il nuovo valore con il puntatore del programma prima di accettarlo.

!!! astuce "Aggiramento (verificato)"
    `POKE 76,0 : POKE 77,12 : SCR` porta HIMEM a 3072; il programma successivo viene memorizzato sotto. Vedete il capitolo 16.

## I due punti rifiutati in modo diretto <span class="verified">verificato</span>

`X=5:Y=X*2:PRINT Y` digitato senza numero dà `SYNTAX ERR`; la stessa riga con un numero funziona. Anche `FOR` e `GOSUB` sono rifiutati in modo diretto.

!!! astuce "Aggiramento"
    Digitate un'istruzione per riga, oppure scrivete una riga provvisoria `0` e `RUN 0`… senza dimenticare di cancellarla (`DEL 0`).

## INPUT aggiunge sempre «?» <span class="verified">verificato</span>

`INPUT "ETA ? ",N` stampa `ETA ? ?`: il BASIC aggiunge il suo punto interrogativo. Scrivete la richiesta senza `?`: `INPUT "ETA",N` stampa `ETA?`.

## Il ciclo FOR gira almeno una volta <span class="verified">verificato</span>

`FOR I=5 TO 1 … NEXT I` esegue il corpo una volta (con I=5), dove altri BASIC lo saltano.

!!! astuce "Aggiramento"
    Controllate prima: `IF N<1 THEN 100` prima di `FOR I=1 TO N`.

## La pressione di un tasto ferma il programma <span class="verified">verificato</span>

Qualsiasi tasto premuto mentre il programma gira lo ferma (`STOPPED AT`), tranne durante un `INPUT`. Non è un bug ma una scelta di progetto: non c'è un tasto BREAK. Conseguenza: un gioco in tempo reale non può attendere un tasto senza fermarsi. Leggere la tastiera con `PEEK(-12271)` (`$D011`) e `PEEK(-12272)` (`$D010`) sembra possibile, ma l'interprete controlla la tastiera anche tra un'istruzione e l'altra e può fermarsi prima: non l'abbiamo provato (<span class="unverified">non verificato</span>). Per un gioco in tempo reale, passate al linguaggio macchina (Parte III).

## Riepilogo

| # | Bug | Stato | Aggiramento |
|---|---|---|---|
| 1 | uscire da FOR con GOTO → `>8 FORS` | <span class="status-ok">verificato</span> | finire il ciclo con NEXT |
| 2 | `^` si blocca | <span class="status-ok">verificato</span> | moltiplicare |
| 3 | «PPED AT» (+3) | <span class="status-ok">verificato</span> | variabili intermedie |
| 4 | `IF…THEN a:b` esegue b | <span class="status-ok">verificato</span> | condizione invertita + salto |
| 5 | −32768 | <span class="status-ok">verificato</span> | restare ≥ −32767 |
| 6 | `ASC(`, `SCRN(` | <span class="status-ok">verificato</span> | stringa di riferimento |
| 7 | COLOR=, PLOT, HLIN non fanno nulla; VLIN, VTAB rifiutati | <span class="status-ok">verificato</span> | TAB, POKE (GEN2) |
| 8 | 251 caratteri (Apple II) | <span class="status-ok">assente</span> | — |
| 9 | STOPPED AT 65535 | <span class="status-ok">verificato</span> | innocuo |
| 10 | USR si blocca | <span class="status-ok">verificato</span> | CALL |
| 11 | HIMEM= non scende | <span class="status-ok">verificato</span> | POKE 76/77 + SCR |
| 12 | `:` rifiutato in modo diretto | <span class="status-ok">verificato</span> | un'istruzione per riga |
| 13 | INPUT aggiunge «?» | <span class="status-ok">verificato</span> | richiesta senza «?» |
| 14 | FOR gira una volta | <span class="status-ok">verificato</span> | controllare prima |
| 15 | leggere la tastiera con PEEK durante RUN | <span class="status-src">non verificato</span> | linguaggio macchina |
