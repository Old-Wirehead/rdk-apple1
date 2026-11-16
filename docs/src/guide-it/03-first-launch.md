# Primo avvio

Al primo avvio la finestra si apre ingrandita, e una finestra **Il tuo spazio di lavoro** propone cinque disposizioni: **Predefinita**, **Gioco**, **BASIC**, **Assembler** e **Debug**. Ogni miniatura numerata mostra la disposizione, dice a che cosa serve e indica la dimensione minima dello schermo necessaria: fate clic su quella che volete. Togliete la spunta a **Mostra questa finestra all'avvio** se preferite non rivederla; **⊞ Disposizione ▾ → Scegli uno spazio di lavoro…** la riapre. Potete cambiare disposizione in qualsiasi momento con il menu **⊞ Disposizione ▾**. La disposizione predefinita mostra tutti gli strumenti a colpo d'occhio. Ogni zona è un pannello agganciabile che potete spostare, raggruppare in schede o staccare in una finestra propria. Lo schermo a colori (GEN2 / P-LAB) è il rettangolo numero 7, nelle miniature che lo mostrano e nella legenda; i numeri da 1 a 6 non cambiano. Questa finestra non resta sopra le altre. Se la finestra è stretta, la barra degli strumenti scorre: il pulsante **»** porta il gruppo Cassetta.

![La disposizione predefinita. Lo schermo mostra già il piccolo programma della sezione seguente.](img/it/defaut-annote.png "large")

<ol class="legend">
<li><strong>Carica, Salva, Stato</strong>: dump esadecimali <code>.txt</code>, binari <code>.bin</code> e <code>.prg</code>; Salva scrive anche cassette <code>.wav</code>/<code>.aiff</code>; Stato salva e ripristina la macchina.</li>
<li><strong>RESET, Cancella schermo, Velocità (indicatore), Incolla</strong>: i pulsanti della macchina (Ctrl+R, Ctrl+L, F3, Ctrl+V).</li>
<li><strong>BASIC</strong>: carica l'RDK G2 Basic integrato in <code>$E000</code>.</li>
<li><strong>Finestre</strong>: il menu <strong>⊞ Disposizione ▾</strong>, poi un pulsante per pannello (schermi, Editor, Debugger, Disassemblato, Dump, Libreria, ASCII).</li>
<li><strong>Schede grafiche</strong>: <strong>GEN2</strong> e <strong>P-LAB</strong> collegano o scollegano le schede.</li>
<li><strong>Cassetta</strong>: <strong>📼 Inserisci</strong> una cassetta, poi riproduci, ferma, riavvolgi, registra, espelli; oscilloscopio e suono.</li>
<li><strong>EPROM</strong>, <strong>⚙ Preferenze</strong>, <strong>?</strong> (fumetti di aiuto dettagliati, anche F1) e <strong>ℹ</strong> (informazioni, scorciatoie).</li>
<li><strong>Riga di visualizzazione dello schermo</strong>: zoom (da 1x a 8x o Adatta), fosfori, velocità di visualizzazione <strong>Normale</strong> o <strong>Veloce</strong>, schermo vintage, schermo intero, <strong>Stacca</strong>.</li>
<li><strong>Editor</strong>: programmi BASIC e sorgenti 6502 in schede.</li>
<li><strong>Schermo dell'Apple-1</strong>: 40 × 24 caratteri; fate clic per digitare, clic destro per Copia, Incolla, Cancella schermo e Reset.</li>
<li><strong>Debugger</strong>: pausa, passo singolo, registri, punti di interruzione, variabili.</li>
<li><strong>Disassemblato</strong>: il codice attorno al contatore di programma (PC).</li>
<li><strong>Dump</strong>: la memoria in esadecimale e ASCII, modificabile.</li>
<li><strong>Libreria</strong>: i vostri programmi, ciascuno con le sue piccole icone ▶ ⬇ 📼 ⚙.</li>
<li><strong>Barra di stato</strong>: velocità, ritmo di visualizzazione, cassetta, destinazione della tastiera; sotto, l'ultimo messaggio.</li>
</ol>

!!! astuce "Fare spazio"
    Per giocare soltanto, chiudete l'editor e il debugger con i loro pulsanti nel gruppo Finestre (4): lo schermo si ingrandisce. Nell'elenco dello zoom (8), **Adatta** sceglie lo zoom intero più grande che entra, **F11** passa a schermo intero, **⊞ Disposizione ▾ → Predefinita** (4) ripristina tutto; lo stesso menu offre anche una disposizione **Gioco** (schermo dell'Apple-1 e libreria). Se i pulsanti sono troppo piccoli o troppo grandi per il vostro schermo, cambiate **⚙ Preferenze → Interfaccia → Dimensione di icone e testo**.

!!! note "Accesso alla rete al primo avvio"
    Al primissimo avvio, l'emulatore contatta GitHub (`api.github.com`, `raw.githubusercontent.com`) per scaricare il progetto dimostrativo (`dschmenk/Apple1-Slideshow`, elencato in «Progetti GitHub» delle **⚙ Preferenze**), e ne mostra l'avanzamento nella barra di stato. Nient'altro viene inviato. Senza rete, salta semplicemente questo passo; togliete il progetto da quell'elenco per fermare il download una volta per tutte.

## Il Woz Monitor

La barra rovesciata `\` è il prompt del **Woz Monitor**, il programma di 256 byte che avvia l'Apple-1. Fate clic sullo schermo e digitate: le minuscole diventano maiuscole, **Invio** è RETURN, **Backspace** produce il `_` che cancella l'ultimo carattere, come nel 1976.

Il monitor capisce quattro tipi di comandi:

| Digitate | Effetto |
|---|---|
| `300` | mostra il byte all'indirizzo `$0300` |
| `300.30F` | mostra i byte da `$0300` a `$030F` |
| `300: A9 C1` | scrive `A9` in `$0300`, poi `C1` in `$0301` |
| `300R` | esegue il programma che inizia in `$0300` (*Run*) |

## Il vostro primo programma in linguaggio macchina

Digitate queste due righe, ciascuna seguita da Invio:

```mon
» 300: A9 C1 20 EF FF 4C 1F FF

0300: 00
» 300R

0300: A9A
```

La `A` finale l'ha scritta il vostro programma. Gli otto byte sono tre istruzioni del 6502:

| Byte | Istruzione | Scopo |
|---|---|---|
| `A9 C1` | `LDA #$C1` | carica il codice della lettera «A» (con il bit 7 a 1, come vuole l'Apple-1) |
| `20 EF FF` | `JSR $FFEF` | chiama ECHO, la routine del monitor che stampa un carattere |
| `4C 1F FF` | `JMP $FF1F` | torna al monitor, che attende il comando seguente |

Il monitor mostra prima il byte all'indirizzo di partenza (`0300: A9`), poi il vostro programma scrive la `A`. L'assembler integrato (Manuale completo) vi evita di calcolare questi byte a mano.

!!! apple1 "Sul vero Apple-1"
    I comandi sono identici: l'emulatore esegue la vera ROM del Woz Monitor. La macchina vera mostra circa 60 caratteri al secondo. L'emulatore mantiene questo ritmo quando la riga di visualizzazione dello schermo è su **Normale**; **Veloce** mostra tutto in una volta. L'impostazione è comune allo schermo dell'Apple-1 e allo schermo GEN2.
