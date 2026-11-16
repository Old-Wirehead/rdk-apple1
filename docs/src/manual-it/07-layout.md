# Disposizione dei pannelli

<p class="lead">Ogni pannello è una scheda che mettete dove volete: affiancata, in schede, o in una finestra propria su un secondo schermo.</p>

## Spostare un pannello

L'editor, lo schermo dell'Apple-1, il monitor a colori GEN2, il debugger, il disassemblato, il dump, lo strumento ASCII e la libreria sono tutte **schede agganciabili**. Trascinate una scheda per il titolo: compaiono delle guide, e la lasciate **a sinistra di**, **a destra di**, **sopra** o **sotto** un altro pannello, oppure **nello stesso gruppo di schede**. Trascinate i separatori per ridimensionare. La disposizione viene ricordata in `gui.json`.

## La disposizione iniziale

L'editor è a sinistra; al centro, lo schermo dell'Apple-1, con il monitor a colori sotto quando la scheda GEN2 è collegata; a destra, il debugger con il disassemblato sotto, poi il dump, e la libreria all'estrema destra. Al primissimo avvio la scelta si basa sulla larghezza del **monitor** (la finestra si apre piccola e viene ingrandita un istante dopo): sotto i 1400 punti (un portatile da 1366 × 768, uno schermo da 1024 × 768) parte con **editor e schermo**, la coppia più utile; da 1400 punti in su, con tutti i pannelli. Scegliete un'altra disposizione dal menu quando volete. Segue la larghezza della finestra: ogni pannello conserva la sua parte, dal portatile al monitor largo. Il menu **⊞ Disposizione ▾** della barra degli strumenti la ripristina in qualsiasi momento (**Predefinita**, tutti i pannelli aperti) e offre altre quattro disposizioni pronte: **Gioco** (schermo dell'Apple-1 e libreria), **BASIC** (editor e schermo), **Assembler** (editor, schermo e debugger) e **Debug** (editor, debugger, disassemblato e dump). **Salva la disposizione attuale…** chiede un nome e conserva la disposizione corrente, con le sue finestre staccate e la dimensione della finestra principale (caricandola, quelle finestre si riaprono dov'erano); le vostre disposizioni sono elencate sotto un separatore, in ordine alfabetico, e un clic destro permette di **Rinominarle** o **Eliminarle**. Sono conservate in `gui.json`.

## La guida all'avvio

Al primo avvio, una guida di tre schermate sostituisce la vecchia finestra «Il tuo spazio di lavoro». È disegnata in ambra su nero, con l'effetto tubo (linee di scansione, alone, vignettatura, curvatura 0,07) sulla cornice; il testo piccolo e le miniature restano nitidi. Una piccola **×** in alto a destra la chiude. **‹ Indietro** e **Avanti ›** passano da una schermata all'altra; l'ultima ha **Termina**.

1. **Benvenuto**: a che cosa serve il kit.
2. **ROM e BASIC**: il Woz Monitor, la ROM della cassetta e G2 Basic sono integrati. Non serve nient'altro. I puristi installano il proprio Integer BASIC originale da una fonte esterna; il pulsante apre la guida («Installare una ROM»), che contiene solo collegamenti, mai i file.
3. **Il tuo spazio di lavoro**: le cinque disposizioni (**Predefinita**, **Gioco**, **BASIC**, **Assembler**, **Debug**). Ogni miniatura è cliccabile (non c'è un pulsante «Scegli»). I rettangoli mantengono i numeri da 1 a 6, e lo schermo a colori (GEN2 / P-LAB) è il **7**, nel suo colore, nelle miniature che lo mostrano e nella legenda.

**Mostra questa finestra all'avvio** è presente nelle tre schermate, spuntata per impostazione predefinita. La stessa casella si trova in **⚙ Preferenze → Interfaccia**. **⊞ Disposizione ▾ → Scegli uno spazio di lavoro…** la riapre. È una finestra normale: non resta sopra le altre. Non si apre quando la riga di comando fissa una dimensione (`--taille`), né per le schermate.

## Dimensione del testo di ogni finestra

L'editor, il listato dell'assembler, il debugger, il disassemblato, il dump e la libreria hanno ciascuno un comando compatto **A 13 ▲▼** nella loro barra: i triangoli cambiano la dimensione, che viene ricordata per quella finestra. Ctrl+rotellina e Ctrl+più / Ctrl+meno agiscono sulla finestra sotto il puntatore del mouse (sullo schermo dell'Apple-1, Ctrl+più / Ctrl+meno cambiano lo zoom). **Reimposta le preferenze** riporta ognuna di queste dimensioni a 13.

## Che cosa viene ricordato tra due avvii

La disposizione viene salvata da sola all'uscita e ritrovata com'era: posizione e dimensione di ogni pannello, le finestre staccate, lo zoom e la disposizione scelta. La finestra viene ripristinata nella sua dimensione e al suo posto, poi i pannelli vengono rimessi nelle stesse proporzioni, qualunque sia lo schermo: la disposizione è stata verificata a 2880 × 1620, 2560 × 1440, 1920 × 1080 e 1366 × 768, con meno dell'1 % dell'immagine diverso dopo un riavvio. Una disposizione salvata con un nome (**Salva la disposizione attuale…**) conserva in più le finestre staccate. L'interfaccia non fa mai zoom da sola: solo l'impostazione «Dimensione di icone e testo» cambia la dimensione dei pulsanti.

## Quando lo schermo è più grande del suo pannello

Con uno **zoom fisso** (2x, 3x…, oppure <kbd>Ctrl</kbd>+<kbd>+</kbd>), lo schermo dell'Apple-1 conserva la dimensione richiesta, anche se il pannello è più piccolo: allora **scorre** (rotellina o barre) invece di essere rimpicciolito, e la barra di stato lo dice, con un suggerimento per allargare il pannello. Solo **Adatta** adegua lo zoom al pannello. In una finestra stretta (1024 × 768, per esempio) la barra degli strumenti scorre di lato invece di essere tagliata: il pulsante **»** porta il gruppo Cassetta, e un secondo clic torna all'inizio. Il pulsante **Esegui** dell'editor resta sempre visibile.

## Staccare un pannello

Ogni pannello ha un pulsante **Stacca** in alto a destra (anche dal clic destro sulla scheda) che lo apre in **una finestra propria**, che potete spostare su un secondo schermo; dimensione e posizione vengono ricordate. **⊟ Aggancia** lo rimette al suo posto iniziale.

- Lo schermo dell'Apple-1 e il monitor a colori si staccano separatamente. La tastiera va all'Apple-1 da una qualsiasi delle loro finestre e dalla finestra principale. Chiudere la finestra di uno schermo lo riaggancia.
- Chiudere la finestra di uno strumento (editor, debugger…) chiude lo strumento; il suo pulsante della barra degli strumenti lo riapre nello stesso posto.
- Le finestre di dialogo (Apri, Salva come, Esporta) si aprono sempre nella finestra principale. Nelle finestre dei file, <kbd>Esc</kbd> chiude la finestra (anche dal campo del nome del file), <kbd>Ctrl</kbd>+<kbd>L</kbd> modifica il percorso (il suo testo è selezionato, quindi digitando lo si sostituisce); Salva ed Esporta di una scheda partono dalla cartella del suo file.
- Le disposizioni salvate conservano le finestre staccate; la finestra principale mantiene la stessa dimensione da un avvio all'altro, qualunque sia lo zoom.
- L'oscilloscopio della cassetta resta una piccola finestra mobile dentro la finestra principale.

<div class="cards">
<div class="card"><h4>Per giocare</h4>Chiudete l'editor e il debugger, tenete la libreria. <strong>Adatta</strong> ingrandisce lo schermo; <kbd>F11</kbd> passa a schermo intero, con lo schermo vintage.</div>
<div class="card"><h4>Per programmare</h4>Editor a sinistra, schermo al centro, debugger e dump a destra. Staccate l'editor su un secondo monitor per dargli tutta l'altezza.</div>
<div class="card"><h4>Per la GEN2</h4>Collegate la scheda: il monitor a colori va sotto lo schermo dell'Apple-1. Staccatelo per averlo grande, come un secondo monitor.</div>
<div class="card"><h4>Per una presentazione</h4><strong>Staccate</strong> lo schermo dell'Apple-1 sul proiettore, a schermo intero, e la finestra principale sul vostro portatile.</div>
</div>

# Preferenze

<p class="lead">⚙ Preferenze raccoglie tutte le impostazioni permanenti. Tutto è conservato in ~/.config/apple1-emu/gui.json (o nella cartella del kit, in modalità portatile).</p>

![La scheda Interfaccia delle preferenze: lingua, tema, dimensione di icone e testo, barra compatta, fumetti, finestra di avvio e colori dell'interfaccia.](img/it/preferences.png "large")

Le impostazioni sono distribuite su sei schede:

| Scheda | Impostazioni |
|---|---|
| **Interfaccia** | lingua: automatica (dalla disposizione della tastiera) o imposta: francese, inglese, tedesco, russo, spagnolo, italiano, portoghese; vengono mostrate la lingua rilevata e la sua fonte. Tema: scuro, chiaro o di sistema. Dimensione di icone e testo: piccola, normale (predefinita), grande, molto grande. Barra compatta (solo icone quando la finestra è stretta). Fumetti al passaggio del mouse. **Mostra questa finestra all'avvio** (la finestra «Il tuo spazio di lavoro») e il pulsante che la apre subito. **Colori dell'interfaccia**: i colori degli elenchi dell'interfaccia, scelti **separatamente per il tema scuro e per quello chiaro** (pulsanti attivi, riga corrente del debugger, riga evidenziata, cella selezionata, riga di punto di arresto, indirizzi e colonna ASCII del dump, valori modificati, etichette); ogni colore ha un campione con anteprima dal vivo, i valori predefiniti sono leggibili (contrasto di almeno 4,5:1 sullo sfondo del tema), e i pulsanti **Reimposta** ripristinano un tema o entrambi. |
| **Editor** | colori della sintassi, separatamente per il BASIC e per l'assembler 6502 (capitolo 7) |
| **Libreria** | la cartella principale (**Sfoglia…**, **⟲ Predefinita**, **Apri la cartella**), le altre cartelle, i progetti GitHub e il token GitHub facoltativo (solo per repository privati e artefatti di Actions; mai mostrato, e la variabile `GITHUB_TOKEN` ha la precedenza); posizione della libreria, a sinistra o a destra (a destra per impostazione predefinita) |
| **Cassetta** | suono della cassetta (muto a velocità rapida) e il suo volume, oscilloscopio durante la riproduzione o la registrazione |
| **GEN2** | la scheda a colori di Uncle Bernie, scansione a 60 o 50 Hz, tavolozza LORES (capitolo 6) |
| **Avanzate** | visualizzazione e velocità: mostra la libreria, visualizzazione limitata a ~60 caratteri/s come il vero terminale, velocità rapida; **Schermo vintage: regolazioni fini** (comuni ai due schermi, capitolo 3); il file Integer BASIC caricato in `$E000` (**📂 Scegli…** / **Dimentica**) |

Ogni cursore delle preferenze (intensità, persistenza e curvatura dello schermo vintage, volume della cassetta) mostra accanto il suo **valore numerico**. Al primo avvio la curvatura dello schermo vintage vale **0,07** (un leggero arrotondamento del tubo); un'installazione esistente mantiene il proprio valore. L'anteprima della scheda **Editor** segue la lingua dell'interfaccia.

Lo zoom, i fosfori e la casella dello schermo vintage non sono nelle preferenze: si trovano nella riga di visualizzazione di ogni schermo (capitolo 3).

![La scheda Editor: uno schema di colori per il BASIC e per l'assembler 6502, oppure i vostri colori.](img/it/preferences-editeur.png "large")

**Lingua automatica.** L'emulatore segue la disposizione attiva della tastiera: `fr` dà il francese, `us` o `gb` l'inglese, `de`, `ch` o `at` il tedesco, `ru` il russo, e così via. Interroga GNOME, `setxkbmap`, Cinnamon (il caso di Linux Mint), `localectl`, poi `/etc/default/keyboard` e infine le variabili `LANG`. Sulla riga di comando: `--langue fr|en|de|ru|es|it|pt|auto`.

!!! astuce "Ripartire da zero"
    **Reimposta le preferenze**, in fondo alla finestra, ripristina le impostazioni originali dopo una conferma (lingua, tema, disposizione, schermo vintage…). I file della vostra libreria restano dove sono; le schede dell'editor non salvate vengono chiuse. Il primo avvio di una nuova versione del kit pone la stessa domanda, perché `gui.json` sopravvive quando la cartella del programma viene cancellata per installarne un'altra.

    Per un kit portatile, mettete un file vuoto chiamato `portable` nella radice del kit (accanto a `run.sh`), oppure avviatelo con `APPLE1_PORTABLE=1`: le preferenze (`config/gui.json`), la libreria predefinita (`programmes`) e la cache stanno allora nella cartella del kit.

    Chiudere l'emulatore e rinominare `~/.config/apple1-emu/gui.json` ha lo stesso effetto del pulsante, all'avvio seguente. Le impostazioni dei programmi restano nel `programs.json` di ogni cartella.
