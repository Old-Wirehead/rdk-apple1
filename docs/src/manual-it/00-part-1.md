---
version: 1.0
date: novembre 2026
---

# L'emulatore <span class="part-sub">La finestra, lo schermo, la libreria, l'interfaccia a cassetta ACI, la scheda a colori GEN2, il debugger e la riga di comando</span> {: .part data-part="Parte I" }

# Questo manuale

<p class="lead"><strong>Retro Dev Kit Apple 1</strong> — «Il peccato originale» — riproduce ciclo per ciclo l'Apple-1 del 1976: il 6502, il terminale video, la tastiera, l'interfaccia a cassetta ACI e, come opzioni, la scheda a colori GEN2 di Uncle Bernie e la scheda grafica P-LAB con TMS9918A. Ha il suo BASIC libero, <strong>RDK G2 Basic</strong>. Attorno alla macchina aggiunge un'officina completa: una libreria di programmi, un editor, un assembler, un debugger e l'esportazione verso un vero Apple-1.</p>

Questo manuale descrive tutto l'emulatore, versione 1.0 (la prima versione pubblica; i numeri 7.x erano le versioni di prova). Completa la *Guida rapida*, che basta per installare il programma e riprodurre una prima cassetta. Qui sono spiegati ogni pannello, ogni pulsante e ogni comando, con esempi che potete digitare così come sono.

## Com'è organizzato questo manuale

<div class="cards">
<div class="card"><h4>Parte I — L'emulatore</h4>La finestra e i suoi pannelli, lo schermo vintage, la libreria, la cassetta e l'ACI, le schede grafiche GEN2 e P-LAB, l'editor, il debugger, l'arte ASCII, le preferenze, la riga di comando e la risoluzione dei problemi.</div>
<div class="card"><h4>Parte II — Integer BASIC</h4>Il BASIC di Steve Wozniak e il suo clone libero RDK G2 Basic: ogni istruzione, funzione e operatore, la memoria, i messaggi di errore e i bug noti, con i rimedi.</div>
<div class="card"><h4>Parte III — L'assembler 6502</h4>Il 6502 dalle basi, l'assembler integrato e le sue direttive, le routine del Woz Monitor, esempi completi e le trappole da evitare.</div>
<div class="card"><h4>Appendici</h4>Mappa della memoria, comandi del Woz Monitor, il formato ACI, il set di caratteri del 2513, le scorciatoie da tastiera e l'indice dei comandi.</div>
</div>

## Convenzioni

I tasti si scrivono così: <kbd>Ctrl</kbd>+<kbd>R</kbd>, <kbd>RETURN</kbd>. I pulsanti dell'interfaccia sono in grassetto: **▶ Esegui**. Gli indirizzi sono in esadecimale, preceduti da `$` come nei documenti dell'epoca: `$FFEF`. Il Woz Monitor stesso li scrive senza `$`: `FFEF`.

Le sessioni sullo schermo dell'Apple-1 hanno lo sfondo scuro. Ciò che digitate **voi** è in bianco, la risposta della macchina in verde:

```mon
» 300R

0300: A2
CIAO, APPLE-1!
```

Lungo il percorso trovate quattro tipi di riquadri:

!!! astuce "Suggerimento"
    Una scorciatoia, o un modo migliore di fare qualcosa.

!!! attention "Attenzione"
    Una trappola che costa tempo o dati.

!!! apple1 "Sulla macchina vera"
    Che cosa succede su un vero Apple-1 e che cosa cambia rispetto all'emulatore.

!!! bug "Bug noto"
    Un difetto della ROM originale, con il suo rimedio. Ogni bug ha uno stato: <span class="verified">verificato</span> (riprodotto nell'emulatore con il vero Integer BASIC) o <span class="unverified">non verificato</span> (citato da una fonte, non provato).

Ogni esempio di questo manuale è stato provato nell'emulatore. I programmi 6502 della Parte III si trovano nella cartella `docs/exemples/it/asm/` della distribuzione. Producono esattamente gli stessi byte con l'assembler integrato e con ca65.

## Che cosa serve

L'emulatore funziona su Linux senza installare nulla. Il Woz Monitor e la ROM dell'ACI sono integrati, come pure **RDK G2 Basic**, un BASIC libero compatibile con l'Integer BASIC (capitolo 14): il pulsante **BASIC** funziona subito, su un'installazione nuova, senza cercare alcun file. L'**Integer BASIC** di Apple del 1976 **non** è fornito, perché è ancora protetto dal diritto d'autore. Se volete l'originale, usate una vostra copia (per esempio la cassetta `BASIC.wav` della raccolta «Apple1Cassettes» di archive.org) e sceglietela in **⚙ Preferenze → Avanzate → BASIC**.

!!! attention "ROM e cassette restano a voi"
    L'emulatore non fornisce né l'Integer BASIC di Apple né cassette commerciali. La sua tabella di riconoscimento contiene solo dimensioni, impronte e parametri. Tenete le vostre copie nella vostra libreria personale.

!!! note "Licenza"
    Retro Dev Kit Apple 1 è software libero sotto **GNU GPL 3.0** (senza garanzia). Alcune parti mantengono la propria licenza MIT: RDK G2 Basic e i test e i glifi GEN2 di Uncle Bernie. Vedete `LICENSE` e `NOTICE` nell'archivio, e l'appendice «Licenza e ringraziamenti».

## I percorsi principali

Quattro modi di usare l'emulatore tornano di continuo. Ognuno è descritto più avanti con il proprio schema:

| Voglio… | Il percorso | Vedere |
|---|---|---|
| giocare con un programma dell'epoca | libreria → **▶** | capitolo 4 |
| scrivere un programma BASIC | **📝 Editor** → **▶ Esegui** → **📼 Esporta WAV** | Parte II |
| scrivere un programma 6502 | **📝 Editor** (`.s`) → **📃 Listato** → **🐛 Debugger** | Parte III |
| inviare un programma a un vero Apple-1 | **📼 Esporta WAV** (`.aiff`) → smartphone → ACI | capitolo 5 |

# La finestra e la barra degli strumenti

## Panoramica

![La disposizione predefinita: tutti i pannelli aperti e agganciati.](img/it/defaut-annote.png "large")

<ol class="legend">
<li><strong>Carica, Salva, Stato</strong>: dump esadecimali <code>.txt</code>, binari <code>.bin</code> e <code>.prg</code>; Salva scrive anche cassette <code>.wav</code>/<code>.aiff</code>; Stato salva e ripristina la macchina.</li>
<li><strong>RESET, Cancella schermo, Velocità (indicatore), Incolla</strong>: i pulsanti della macchina (Ctrl+R, Ctrl+L, F3, Ctrl+V).</li>
<li><strong>BASIC</strong>: carica l'RDK G2 Basic integrato in <code>$E000</code> (o il vostro Integer BASIC, se l'avete scelto).</li>
<li><strong>Finestre</strong>: il menu <strong>⊞ Disposizione ▾</strong>, poi un pulsante per pannello (Schermo monocromatico, Schermo a colori, Editor, Debugger, Disassemblato, Dump, Libreria, ASCII).</li>
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

Ogni pannello (schermo, editor, debugger, libreria…) è una scheda che potete spostare, raggruppare o staccare in una finestra propria: è l'argomento del capitolo 10. Al primo avvio la finestra si apre ingrandita e una finestra **Il tuo spazio di lavoro** propone cinque disposizioni (capitolo 10); dimensione e posizione della finestra e disposizione dei pannelli vengono ricordate e ritrovate così com'erano all'avvio seguente.

## La barra degli strumenti, pulsante per pulsante

La barra degli strumenti è una sola riga: la macchina e i suoi strumenti a sinistra, il registratore al centro, i pannelli nel gruppo Finestre, le schede e, a destra, le impostazioni. Quando la finestra è troppo stretta, i pulsanti conservano solo l'icona (il testo resta nel fumetto), oppure la barra va su più righe se togliete la spunta a **Barra compatta** nelle preferenze.

![La barra degli strumenti fino al gruppo Schede grafiche: file, macchina, finestre e schede. Seguono la striscia Cassetta (capitolo 5) e poi le impostazioni.](img/it/barre-1.png "large")

| Pulsante | Che cosa fa |
|---|---|
| {{Carica}} | Apre un programma e lo mette in memoria: un `.txt` (dump del Woz Monitor) o un `.hex`; per un `.bin` viene chiesto l'indirizzo. Le cassette (`.wav`, `.aiff`) non passano da qui: si inseriscono con **📼 Inserisci**, nella striscia Cassetta. |
| {{Salva}} | Chiede uno o più intervalli (`0300.03FF`, `004A.00FF 0300.0FFF`…), poi un file. L'estensione sceglie il formato: `.wav`, `.aiff`, `.txt` o `.bin`. |
| **RESET** (pulsante rosso) | Il pulsante RESET dell'Apple-1 (<kbd>Ctrl</kbd>+<kbd>R</kbd>). Il 6502 e il PIA ripartono; memoria e schermo sono conservati. |
| **🆑 Cancella schermo** | Il pulsante CLEAR SCREEN (<kbd>Ctrl</kbd>+<kbd>L</kbd>). |
| **Indicatore (Velocità reale / Velocità rapida)** | Alterna tra l'1 MHz originale e la velocità massima (<kbd>F3</kbd>). Un tachimetro: illuminato in blu mentre la velocità rapida è attiva. |
| **📋 Incolla** | Digita gli appunti sulla tastiera dell'Apple-1, al ritmo della tastiera (<kbd>Ctrl</kbd>+<kbd>V</kbd>). Ideale per un dump esadecimale. |
| {{BASIC|**BASIC**}} | Carica l'**RDK G2 Basic** integrato in `$E000` (7933 byte, poi `E000R` lo avvia). Se avete scelto l'Integer BASIC originale in **⚙ Preferenze → Avanzate → BASIC**, carica quello: una cassetta, un dump `.txt` o un `.bin` di 4096 byte, cercato nella libreria (`BASIC.wav`, `integer_basic.bin`, qualsiasi nome che contenga «basic»), altrimenti viene chiesto. Clic destro: scegliere un altro file. |
| **Finestre** (gruppo: **⊞ Disposizione ▾** e un pulsante per pannello) | Mostra o nasconde ogni pannello: **Schermo monocromatico** (il terminale dell'Apple-1), **Schermo a colori** (lo schermo della scheda grafica collegata, GEN2 o P-LAB: lo stesso pulsante serve per entrambe; senza scheda collegata propone di collegarne una, e **P-LAB** mostra allora il suo schermo), **📝 Editor** (capitolo 7), **🐛 Debugger**, **Disassemblato**, **Dump** (capitolo 8), **📚 Libreria** e **🎨 ASCII** (capitolo 9). Un pulsante illuminato indica che il pannello è visibile; un pannello già mostrato si chiude anche con la crocetta della sua scheda. |
| **Schede grafiche: GEN2 e P-LAB** | Un gruppo con nome. **GEN2** collega o scollega la scheda a colori di Uncle Bernie (capitolo 6); **P-LAB** fa lo stesso per la scheda P-LAB con TMS9918A (sezione 6.6). Un pulsante resta illuminato finché la sua scheda è collegata; lo schermo di una scheda compare in un pannello proprio, mostrato dal pulsante **Schermo a colori**. |
| **📸 Stato ▾** | Un solo pulsante con un menu: **Salva stato…** (<kbd>Ctrl</kbd>+<kbd>Maiusc</kbd>+<kbd>S</kbd>), **Carica uno stato…** (<kbd>Ctrl</kbd>+<kbd>Maiusc</kbd>+<kbd>O</kbd>), poi gli ultimi stati, ciascuno con titolo e data (sezione 2.5). |
| **EPROM** (chip con finestra UV, a destra nella barra, prima di **⊞ Disposizione**) | Apre il programmatore di EPROM: leggere, scrivere o verificare un chip con un programmatore `minipro` (sezione 2.6). |
| **Cassetta: 📼 Inserisci ▶ ⏹ ⏪ ⏺ ⏏** | Il registratore, riunito in una striscia. **📼 Inserisci** sceglie un `.wav` o un `.aiff` e lo mette nel registratore come cassetta virtuale; poi riproduci, ferma, riavvolgi, registra, espelli. La striscia mostra nome, posizione e durata. |
| **📈** e **🔊** | L'oscilloscopio della cassetta e il suono del segnale (capitolo 5). |
| **⊞ Disposizione ▾** (primo pulsante del gruppo Finestre) | Un menu: **Predefinita** (tutti i pannelli, come al primo avvio), **Gioco** (schermo dell'Apple-1 e libreria), **BASIC** (editor e schermo), **Assembler** (editor, schermo e debugger), **Debug** (editor, debugger, disassemblato e dump); poi **Salva la disposizione attuale…**, che chiede un nome, e le vostre disposizioni salvate in ordine alfabetico (clic destro: Rinomina, Elimina). Un segno di spunta indica l'ultima scelta; tutto è conservato in `gui.json`. |
| **⚙** | Le preferenze (capitolo 11). |
| **?** | Modalità aiuto (<kbd>F1</kbd>): fumetti dettagliati su ogni comando, nelle 7 lingue dell'interfaccia. |
| **ℹ** | «Informazioni», con l'elenco delle scorciatoie da tastiera. |

**Finestre dei file.** Apri, Salva con nome ed Esporta usano la finestra dei file propria dell'emulatore. Un pulsante blu **📁+ Nuova cartella**, in alto a sinistra, crea una cartella nella cartella mostrata, con un fumetto che lo spiega; digitate il nome, premete <kbd>Invio</kbd> e la nuova cartella è pronta a ricevere il vostro file.

![La finestra Carica, con il pulsante 📁+ Nuova cartella in alto a sinistra.](img/it/fenetre-fichiers.png "medium")

Le impostazioni di visualizzazione non sono nella barra degli strumenti: ogni schermo ha la propria **riga di visualizzazione**, in cima al suo pannello (sezione 3.2).

![La riga di visualizzazione dello schermo dell'Apple-1: zoom, fosfori, velocità di visualizzazione, schermo vintage e schermo intero; Stacca è all'estremità destra della riga.](img/it/barre-2.png "large")

!!! astuce "Modalità aiuto"
    Quando esplorate un pannello, attivate **?**: ogni pulsante spiega che cosa fa quando ci passate sopra con il mouse. Disattivata, restano solo i soliti fumetti brevi (si possono disattivare in **⚙ → Interfaccia**). L'impostazione viene ricordata.

## La barra di stato

![La barra di stato durante la riproduzione di BASIC.wav: velocità, limite di visualizzazione, cassetta, tastiera e, sotto, l'ultimo messaggio.](img/it/barre-etat.png "large")

- **Velocità misurata**: circa 0,960 MHz a velocità reale. È il clock effettivo dell'Apple-1, che ferma il 6502 per quattro cicli su sessantacinque per rinfrescare la memoria dinamica.
- **Schermo ~60 car./s**: il terminale dell'Apple-1 mostra un solo carattere per fotogramma, circa 60 al secondo. Un clic alterna tra questo limite e la visualizzazione istantanea, come i pulsanti **Normale** / **Veloce** delle righe di visualizzazione.
- **Cassetta**: nome, posizione e durata, oppure «nessuna cassetta».
- **Tastiera**: dove vanno i tasti (all'Apple-1 o a un campo di testo dell'interfaccia).

## La tastiera dell'Apple-1

Fate clic sullo schermo: i tasti vanno all'Apple-1. La tastiera originale ha solo maiuscole, e l'emulatore converte le minuscole.

| Tasto del PC | Per l'Apple-1 |
|---|---|
| lettere, cifre, simboli | il carattere, in maiuscolo |
| <kbd>Invio</kbd> | RETURN (codice `$8D`) |
| <kbd>Backspace</kbd> | `_`: il Woz Monitor e il BASIC cancellano il carattere precedente (resta sullo schermo, seguito da `_`) |
| <kbd>Esc</kbd> | ESC: annulla la riga in corso nel Woz Monitor |
| <kbd>Ctrl</kbd>+lettera | il carattere di controllo (`Ctrl+D` = `$84`…) |
| <kbd>Ctrl</kbd>+<kbd>R</kbd> / <kbd>Ctrl</kbd>+<kbd>L</kbd> | i pulsanti RESET e CLEAR SCREEN |

Potete **selezionare il testo con il mouse** su entrambi gli schermi: premete e trascinate sui caratteri, e la selezione si evidenzia. Un **clic destro** su uno dei due schermi (quello dell'Apple-1 o il monitor GEN2) apre un piccolo menu: **Copia** mette il testo selezionato negli appunti (o tutto il testo dello schermo se non c'è selezione), **Incolla (digitare il testo)** digita gli appunti sulla tastiera dell'Apple-1, come se li scriveste voi, poi **Cancella schermo** e **Reset**.

![Il menu del clic destro dello schermo dell'Apple-1, durante una partita a Hamurabi.](img/it/ecran-menu.png "medium")

!!! apple1 "Lo schermo dell'Apple-1 non cancella nulla"
    Il terminale sa solo scrivere un carattere e andare a capo. Non c'è un cursore mobile né un modo per cancellare un carattere: il tasto di cancellazione stampa `_` e il carattere resta sullo schermo. È normale.

## Stato della macchina

Il pulsante **📸 Stato** della barra degli strumenti scatta un'istantanea completa della macchina e la rimette più tardi esattamente allo stesso punto: potete fermare un gioco o un programma in corso, chiudere l'emulatore e riprendere il giorno dopo. Il pulsante apre un menu:

![Il menu Stato: salva, carica, poi gli ultimi stati con la data.](img/it/menu-etat.png "medium")

- **Salva stato…** (<kbd>Ctrl</kbd>+<kbd>Maiusc</kbd>+<kbd>S</kbd>) chiede un nome di file e scrive un file `.a1state`. La cartella proposta è `etats`, accanto alla cartella della libreria.
- **Carica uno stato…** (<kbd>Ctrl</kbd>+<kbd>Maiusc</kbd>+<kbd>O</kbd>) sostituisce la macchina con lo stato scelto. La barra di stato mostra il file e il valore di PC.
- **Ultimi stati** elenca gli ultimi otto, con titolo e data del salvataggio; un clic ne carica uno subito. Se il file è stato spostato o eliminato, un messaggio lo segnala.

Uno stato contiene la memoria, i registri del 6502, il PIA, lo schermo, la scheda GEN2 se è collegata e la cassetta virtuale con la sua posizione. Non contiene le vostre preferenze né la libreria. Un file illeggibile (danneggiato, o che non è uno stato) viene rifiutato con un messaggio, e la macchina resta intatta.

!!! astuce "Uso tipico"
    Salvate uno stato appena prima di un passaggio difficile di un gioco, o alla fine di una lunga sessione BASIC: uno stato è anche un modo comodo per condividere un programma in esecuzione con un amico che usa la stessa versione.

## Il programmatore di EPROM

**EPROM** nella barra degli strumenti apre una finestra per leggere, scrivere e verificare un chip EPROM o EEPROM dall'emulatore, per esempio per masterizzare la ROM del BASIC, un programma per l'espansione GEN2 o il contenuto della vostra memoria.

![Il programmatore di EPROM: programmatore, chip, origine dei dati e le tre operazioni.](img/it/programmateur-eprom.png "medium")

Sono supportati solo i programmatori gestiti dallo strumento libero **`minipro`**: gli XGecu **TL866II+**, **T48** e **T56**. Su Debian, Ubuntu e Linux Mint si installa con `sudo apt install minipro`; collegate il programmatore a una porta USB, poi premete **Rileva**. Se `minipro` manca, la finestra lo dice e spiega che cosa fare; dal lato dell'emulatore non serve altro.

| Impostazione | Ruolo |
|---|---|
| **Programmatore** | il programmatore rilevato, con il suo firmware (per esempio «Found TL866II+ 04.2.132 (USB)») |
| **Chip** | i chip più comuni del mondo Apple-1: 2716, 2732, 2764, 27C64, 27C128, 27C256, 27C512, 28C64, 28C256, SST27SF256, SST27SF512 e la ROM a maschera 23128 (sola lettura) |
| **Dati** | la **Memoria dell'Apple-1** (un intervallo di indirizzi, `$E000`–`$EFFF` per impostazione predefinita) o un **file** (`.bin`, `.hex`, `.aiff`…) |

Tre pulsanti agiscono sul chip, e una barra di avanzamento e un registro mostrano che cosa succede:

- **Leggi il chip** legge tutto il chip. Se i dati sono la memoria o un file esistente, il contenuto viene **confrontato** e le differenze sono elencate: un file esistente non viene mai sovrascritto. Con un nome di file nuovo, il contenuto viene salvato in esso.
- **Scrivi il chip** programma il chip con i dati scelti. Il chip deve essere vergine o cancellabile; alla fine viene confermato il risultato.
- **Verifica** confronta il chip con i dati scelti, byte per byte, e segnala ogni differenza (`$E010: chip $4C, dati $A9`).

**Chiudi** chiude la finestra.

La finestra non modifica mai la memoria emulata: scrittura e verifica la leggono, la lettura la confronta soltanto o riempie un file.

# Lo schermo: zoom, fosfori ed effetto vintage

## Lo schermo originale

L'Apple-1 mostra 40 colonne × 24 righe di caratteri maiuscoli. Ogni carattere è una matrice di 5 × 7 punti in una cella di 7 × 8, letta dalla ROM del generatore di caratteri **Signetics 2513**: l'emulatore usa il contenuto di questa ROM, punto per punto (Appendice D). L'immagine ha quindi 280 × 192 punti.

Il cursore è una `@` lampeggiante, comandata da un NE555: 2,18 Hz, visibile per circa due terzi del tempo. Su alcune schede la tolleranza del condensatore dà invece 1,7 Hz; l'opzione `--cursor-hz 1.7` lo riproduce.

## La riga di visualizzazione

Ogni schermo ha la propria riga di impostazioni in cima al suo pannello: lo schermo dell'Apple-1 e il monitor a colori quando la scheda GEN2 è collegata (capitolo 6).

| Impostazione | Ruolo |
|---|---|
| **Zoom** | un elenco: da `1x` a `8x` ingrandiscono l'immagine di un fattore intero, così i punti restano quadrati e nitidi; **Adatta** sceglie il fattore più grande che entra nel pannello (il fattore scelto è indicato tra parentesi). Da tastiera: <kbd>Ctrl</kbd>+<kbd>+</kbd>, <kbd>Ctrl</kbd>+<kbd>−</kbd>, <kbd>Ctrl</kbd>+<kbd>0</kbd> (adatta). |
| **Fosfori** | solo schermo dell'Apple-1: il verde dei monitor dell'epoca, il bianco di un televisore o l'ambra. |
| **Visualizzazione: Normale / Veloce** | **Normale** mostra i caratteri al ritmo del vero terminale, circa 60 al secondo; **Veloce** mostra tutto in una volta. L'impostazione è condivisa: cambiarla su uno schermo la cambia sull'altro. |
| **Schermo vintage** | l'effetto tubo degli anni '70 (sezione seguente); ogni schermo ha la propria casella. |
| **⛶** | schermo intero (<kbd>F11</kbd>). |
| **Stacca** | mette lo schermo in una finestra propria (capitolo 10). |

Ogni schermo è **centrato** nel suo pannello, in orizzontale e in verticale, con la didascalia centrata sotto l'immagine. Se lo zoom scelto è più grande del pannello, lo schermo viene mostrato con lo zoom più grande che entra invece di essere tagliato. Quando la finestra è più stretta della riga di visualizzazione, la riga va su una seconda linea, e **Stacca** resta all'estremità destra dell'ultima linea.

![Fosfori ambra, con lo schermo vintage e una leggera curvatura.](img/it/brut/ambre.png "medium")

## Lo schermo vintage

La casella **Schermo vintage** della riga di visualizzazione imita un monitor a tubo degli anni '70. L'effetto principale è la sottile linea scura tra due linee di scansione: ogni riga di punti diventa una banda orizzontale, come nella foto di un vero Apple-1. Questa linea mantiene lo stesso spessore in pixel del vostro schermo a ogni zoom: resta sottile e nitida, anche a schermo intero.

![Lo schermo vintage, con una leggera curvatura del tubo (0,15) e angoli scuri, durante una partita a Hamurabi.](img/it/brut/vintage.png "medium")

In **⚙ Preferenze → Avanzate → Schermo vintage: regolazioni fini** impostate l'**Intensità** e scegliete gli effetti; queste impostazioni sono comuni ai due schermi. Ogni cursore mostra accanto il suo **valore numerico** (curvatura, intensità…):

| Effetto | Predefinito | Ruolo |
|---|---|---|
| separazione delle linee di scansione | attivo | la linea scura tra due righe di punti |
| punti ammorbiditi in orizzontale | attivo | una leggera sfocatura, solo orizzontale, come il fascio |
| persistenza dei fosfori | breve | i punti si spengono in meno di un decimo di secondo (un po' più lentamente in ambra); tutto a sinistra: nessuna |
| alone | attivo, discreto | un alone attorno ai punti, che restano nitidi |
| tubo bombato e angoli scuri | disattivato | con un cursore **Curvatura** |
| sfarfallio | disattivato | una leggerissima variazione di luminosità |
| maschera di fosfori RGB | attiva | solo lo schermo a colori della scheda GEN2: sottili strisce verticali rosse, verdi e blu separate da linee scure, come sul tubo di un televisore a colori. È visibile dallo zoom 3 in su, e la sua forza segue l'**Intensità** |

L'effetto si può applicare anche al monitor a colori della scheda GEN2, con la casella della sua riga di visualizzazione. È calcolato dal processore, alla risoluzione dell'Apple-1 e solo quando l'immagine cambia: resta fluido anche senza accelerazione grafica.

Sulla riga di comando: `--screening`, `--sans-screening`, `--gen2-screening` (monitor GEN2), oppure per esempio `--screening-options intensite=0.7,remanence=0.5,courbure=0.4,sans-halo` (valori da 0 a 1; i nomi delle opzioni sono in francese).

!!! astuce "Schermo staccato e secondo monitor"
    **Stacca**, alla fine della riga di visualizzazione, mette lo schermo dell'Apple-1 in una finestra propria. Mettetelo su un secondo monitor, a schermo intero, con l'effetto vintage: la tastiera va all'Apple-1 appena quella finestra ha il fuoco. Con **Adatta**, l'immagine segue le dimensioni della finestra, a passi interi (**Scala intera**, nella barra del titolo della finestra, la più nitida) o riempiendola.
