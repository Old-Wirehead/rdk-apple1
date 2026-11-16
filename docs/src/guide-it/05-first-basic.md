# Il vostro primo programma BASIC

L'Integer BASIC, il BASIC di Steve Wozniak, sta in 4 KB, e l'**RDK G2 Basic** dell'emulatore, che esegue gli stessi programmi, è integrato. Conosce solo i numeri interi (da −32767 a 32767), ma è veloce e piacevole. L'editor integrato vi evita di digitare sullo schermo dell'Apple-1, dove un errore di battitura costringe a riscrivere l'intera riga.

## Scrivere il programma

Fate clic su **📝 Editor** nella barra degli strumenti, poi sull'icona **Nuovo** (la prima) dell'editor, scegliete «Programma BASIC» e digitate questo programma (anche in `docs/exemples/it/basic/premier.bas`).

```basic title="premier.bas"
10 REM IL MIO PRIMO PROGRAMMA
20 PRINT "CIAO, APPLE-1!"
30 PRINT
40 FOR I=1 TO 8
50 PRINT I;" AL QUADRATO = ";I*I
60 NEXT I
70 PRINT
80 DIM N$(20)
90 INPUT "IL TUO NOME",N$
100 PRINT "PIACERE, ";N$;"!"
110 END
```

Parole chiave, numeri, stringhe e commenti hanno ciascuno il proprio colore (i colori si cambiano in **⚙ Preferenze → Editor**); sotto il testo, l'editor mostra la dimensione del programma e la memoria che resta per le variabili.

!!! astuce "Clic destro nel testo"
    Il menu dell'editor contiene taglia, copia e incolla, **Cerca** (Ctrl+F) e **Sostituisci** (Ctrl+H), vai a una riga (Ctrl+G), REM / togli REM su un blocco, **Rinumera…** (`GOTO`, `GOSUB` e `THEN` seguono), un punto di interruzione sulla riga (F9) e **Invia e avvia** (F5).

## Eseguirlo

Fate clic su **▶ Esegui**. Il programma va direttamente in memoria (dopo il BASIC, se serve), poi l'editor digita per voi `E2B3R` e `RUN`:

![L'editor a sinistra, il programma in esecuzione a destra.](img/it/brut/editeur-premier.png "large")

Il programma chiede il vostro nome. Fate clic sullo schermo dell'Apple-1, digitatelo, poi Invio:

```mon
IL TUO NOME?WOZ
PIACERE, WOZ!
```

!!! attention "Tre particolarità dell'Integer BASIC"
    - **Le stringhe vanno dimensionate**: senza `DIM N$(20)`, l'`INPUT` di una stringa si ferma con `*** STR OVFL ERR`.
    - **I nomi di variabile** sono una lettera, seguita eventualmente da una cifra: `A`, `B2`, `N$`. `TOTALE` è un errore di sintassi.
    - **L'`END` finale è obbligatorio**: senza, il programma termina con `*** END ERR`.

## Quando c'è un errore

L'editor controlla ogni riga mentre digitate, con le regole del vero BASIC. Una riga sbagliata diventa rossa e il messaggio spiega il problema, con il numero di riga e la colonna:

![Due errori segnalati: una stringa non chiusa e un nome di variabile troppo lungo.](img/it/editeur-erreur.png "medium")

Un programma con errori non si avvia: correggete prima le righe rosse. Gli errori di esecuzione compaiono sullo schermo dell'Apple-1: `*** BAD BRANCH ERR` per un `GOTO` verso una riga inesistente, `*** >32767 ERR` per un overflow o una divisione per zero. Il **Debugger** (Manuale completo) procede allora riga per riga osservando le variabili.

![Il ciclo di lavoro: scrivere, eseguire, correggere, poi esportare sulla macchina vera.](diagrams/it/workflow-basic.svg "diagram")

!!! astuce "Fermare un programma"
    Durante un `RUN`, premere un tasto ferma il programma con `STOPPED AT 50` (la riga in corso). Se resta bloccato, **Reset** (Ctrl+R) torna al monitor ed `E2B3R` rientra nel BASIC **senza cancellare** il programma.
