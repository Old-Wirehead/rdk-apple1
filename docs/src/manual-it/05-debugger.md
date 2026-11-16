# Il debugger, il disassemblato e il dump

<p class="lead">Il debugger ferma l'Apple-1 e lo fa avanzare passo passo, un'istruzione alla volta in 6502 o una riga alla volta in BASIC, mostrando i registri, le variabili e la memoria.</p>

## Il pannello del debugger

**🐛 Debugger** apre il pannello, agganciato a destra o staccabile come l'editor. I suoi comandi:

| Comando | Effetto |
|---|---|
| **⏸ Pausa** / **▶ Continua** | ferma o fa ripartire l'Apple-1 |
| **⬇ Passo** | esegue un'istruzione |
| **↪ Scavalca** | esegue un `JSR` in un colpo solo e si ferma al ritorno |
| **⤴ Esci** | esegue fino all'`RTS` della subroutine corrente |
| **⏭ Riga BASIC** | avanza di una riga del programma BASIC |
| **⏩ Fino al cursore** | (nell'editor) esegue fino alla riga del cursore |

{{!Passo (debugger)}}{{!Scavalca}}{{!Esci}}

Il riquadro **Processore** mostra A, X, Y, S, PC e i flag N V B D I Z C; si possono tutti modificare facendo clic sul valore. Sotto è indicato il numero di cicli e di istruzioni eseguiti.

## Punti di arresto

Un punto di arresto si imposta in quattro modi:

- **⏺ Punto di arresto** nell'editor (o <kbd>F9</kbd>, o il menu del clic destro), sulla riga del cursore: in un sorgente `.s` (grazie alla tabella degli indirizzi dell'assembler) o in un programma BASIC (numero di riga);
- un clic nel margine del **📃 Listato**;
- un clic su una riga del disassemblato;
- la digitazione di un indirizzo o di un'etichetta: `$0368`, `ASK`, `PUTS+3`, oppure `L190` per la riga BASIC 190. Un'etichetta che non esiste ancora non viene rifiutata: il punto di arresto viene conservato e impostato appena l'etichetta esiste (quando assemblate il sorgente dell'editor o lo avviate), con un messaggio che lo dice.

I punti di arresto, le variabili osservate e le osservazioni della memoria vengono ricordati da una sessione all'altra, ma caricare un altro programma cancella i punti di arresto e le etichette del precedente. Quando è caricato il BASIC integrato, la sezione BASIC del debugger si intitola **RDK G2 Basic**; senza alcun BASIC in `$E000` lo dice in modo neutro. I colori della riga corrente, degli indirizzi del dump e della colonna ASCII si cambiano in **⚙ Preferenze → Interfaccia** («Colori dell'interfaccia»). Nei pannelli stretti la colonna dei byte del disassemblato sparisce e la riga di scrittura va a capo invece di essere tagliata.

## Fare il debug di un programma BASIC

![boucle.bas fermo a un punto di arresto alla riga 30: la riga corrente in verde nell'editor, le variabili I e S osservate nel debugger.](img/it/brut/debogueur-basic.png "large")

La riga corrente è evidenziata in verde nell'editor, i punti di arresto in viola. Il riquadro BASIC (**RDK G2 Basic** o **Integer BASIC**, a seconda di quello caricato) mostra la riga corrente, i punti di arresto sui numeri di riga e le **variabili osservate**: intere (`A`, `B1`), elementi di array (`D(2)`), array interi (`D`), stringhe (`C$`), più **Tutte le variabili**.

Il debug funziona con **RDK G2 Basic**, il BASIC integrato nell'emulatore, così come con l'originale: il vero Integer BASIC non serve più. Il debugger rileva l'inizio di ogni riga (la riga corrente, i punti di arresto di riga, **⏭ Riga BASIC**) e legge le variabili nelle tabelle dell'interprete. Per l'originale legge la riga dai puntatori in pagina zero `$DC`/`$DD`; G2 Basic la tiene altrove, e il debugger conosce entrambe le disposizioni.

!!! attention "Un tasto ferma il programma"
    Come sulla macchina vera, un tasto premuto mentre il programma BASIC gira lo ferma (`STOPPED AT`). Durante una sessione di debug, tenete il fuoco della tastiera nel pannello del debugger, non sullo schermo dell'Apple-1. Un programma G2 Basic può leggere la tastiera senza fermarsi con `KEY(0)` (capitolo 15).

## Fare il debug di un programma 6502

![decimal.s fermo all'etichetta prdec: registri, disassemblato con le etichette del programma, osservazione della memoria $06.$08.](img/it/brut/debogueur-asm.png "large")

Il **disassemblato** mostra 24 righe attorno a PC, con le etichette del programma assemblato in una colonna propria. Si ricentra solo quando PC esce dalla zona visibile, così la visualizzazione non salta a ogni passo. **Vai** fissa l'elenco su un indirizzo o un'etichetta; **Segui PC** gli fa di nuovo seguire l'esecuzione.

Quando selezionate un'istruzione nel sorgente 6502 dell'editor, i suoi byte vengono anche **evidenziati** nel disassemblato e nel dump (capitolo 7), ed entrambi i pannelli scorrono fino a loro se non sono visibili; il disassemblato smette allora di seguire PC finché non premete **Segui PC**.

L'**osservazione della memoria** mostra un byte, una parola o ASCII (`$30`, `$30 w`, `0200.020F`, un'etichetta), modificabile, in giallo quando il valore cambia.

## Il dump

Il **Dump** mostra la memoria in esadecimale e ASCII, 16 byte per riga (8 se il pannello è stretto), con il bit 7 tolto per l'ASCII. **Vai** salta a un indirizzo o alle scorciatoie ZP, `$0200`, `$0300`, PC, `$E000`. I byte modificati sono in giallo, il byte di PC su fondo verde. **Scrivi** deposita dei byte (`0300 = A9 00`); un clic su un byte compila la riga di scrittura.

Il disassemblato e il dump sono pannelli a tutti gli effetti: staccati per impostazione predefinita, si possono agganciare accanto al debugger (capitolo 10).

## Sulla riga di comando

Per dimostrazioni e test automatici: `--debogueur` apre il pannello, `--dbg-arret ASK,L190` imposta punti di arresto, `--dbg-surveiller 'A,C$,$30 w'` osserva variabili e memoria, e `--dbg-commande 5:pas` invia un comando dopo 5 secondi (`pause`, `continuer`, `pas`, `dessus`, `sortir`, `basic`). Questi nomi di opzioni e parole chiave sono in francese.
