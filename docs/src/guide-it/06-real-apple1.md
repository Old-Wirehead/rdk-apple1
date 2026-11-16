# Su un vero Apple-1: esportare in WAV

Tutto ciò che scrivete nell'emulatore può andare su un vero Apple-1, su una replica o su un kit dotato della scheda a cassetta ACI. Il file audio sostituisce il nastro; uno smartphone o l'uscita cuffie di un PC fa da registratore.

## Creare la cassetta

Nell'editor, fate clic su **📼 Esporta WAV** e scegliete un nome di file. L'estensione decide il formato: `.wav`, oppure `.aiff` per un telefono (il formato consigliato, vedi sotto). La parte bassa dell'editor mostra allora esattamente che cosa digitare sulla macchina vera:

![Dopo l'esportazione, la parte bassa dell'editor mostra il comando di caricamento.](img/it/editeur-export-bas.png "large")

Per `premier.bas` il comando è `C100R`, poi `004A.00FFR 0F58.0FFFR`. Il programma BASIC occupa due registrazioni consecutive sulla cassetta: prima la pagina zero del BASIC (da `$004A` a `$00FF`, che contiene i puntatori del programma), poi il programma stesso, memorizzato in cima alla memoria. Il pulsante **Copia gli intervalli** mette il comando negli appunti.

!!! attention "Sempre quattro cifre"
    Digitate `004A.00FF`, mai `4A.FF`. La ROM dell'ACI non cancella le cifre già lette quando incontra il punto: `4A.FF` verrebbe inteso come da `$004A` a `$4AFF`, e la macchina aspetterebbe 19 KB che non arrivano mai. L'emulatore mostra sempre la forma corretta, e riproduce questo comportamento perché esegue la vera ROM dell'ACI.

## Caricare sulla macchina vera

Il BASIC deve essere già in memoria, caricato dalla sua cassetta (`C100R`, poi `E000.EFFFR`).

<ol class="steps">
<li><strong>Preparate il lettore.</strong> Copiate il file sul telefono, collegate l'uscita cuffie all'ingresso «FROM TAPE» dell'ACI e mettete il volume al massimo.</li>
<li><strong>Tornate al monitor</strong> con RESET, poi digitate <code>C100R</code>. L'ACI risponde <code>C100: A9*</code> e va a capo; l'asterisco è il suo prompt.</li>
<li><strong>Digitate gli intervalli</strong> <code>004A.00FFR 0F58.0FFFR</code>, ma non premete ancora RETURN.</li>
<li><strong>Avviate la riproduzione</strong>, poi premete <strong>RETURN durante l'intestazione</strong>, il tono costante di circa 10 secondi con cui inizia ogni registrazione.</li>
<li><strong>Aspettate che torni il <code>\</code></strong>, poi digitate <code>E2B3R</code> (avvio a caldo del BASIC, che conserva il programma) e <code>RUN</code>.</li>
</ol>

```mon
» C100R

C100: A9*
» 004A.00FFR 0F58.0FFFR
\
» E2B3R

E2B3: 20
» >RUN
CIAO, APPLE-1!
```

!!! apple1 "Leggere in modo affidabile da un telefono"
    - **Volume al massimo.** Accettate l'avviso di Android sul «volume alto», altrimenti il volume resta limitato. Molti telefoni abbassano il volume ogni volta che il jack viene ricollegato: controllatelo ogni volta.
    - **Nessun effetto sonoro**: disattivate equalizzatore, Dolby, Mi Sound e qualsiasi «miglioramento audio».
    - **Modalità aereo** o «Non disturbare», perché nessuna notifica si mescoli al segnale. Niente Bluetooth.
    - **Un lettore semplice** che riproduca l'AIFF senza elaborarlo; VLC funziona bene.
    - **Mai MP3 né AAC**: la compressione deforma i fronti del segnale. Restate su WAV o AIFF.
    - **Non fidatevi del LED dell'ACI**: si accende solo verso 1,2 V, ben oltre ciò che serve alla scheda.

## Perché AIFF a 48 kHz

I file dell'emulatore sono pensati per l'ingresso dell'ACI. L'AIFF a 16 bit non compresso sfugge a ogni codec; 48 kHz è la frequenza nativa di Android, quindi il telefono non ricampiona. I due canali stereo sono rigorosamente identici, così sinistro e destro possono essere uniti su FROM TAPE senza che le due uscite si contrastino. Il segnale è un'onda quadra con fronti ripidi, ciò che il comparatore dell'ACI rileva meglio.

!!! astuce "Controllare un file prima di riprodurlo"
    Sulla riga di comando, `apple1-emu info premier.aiff --aci-input stock` simula l'ingresso della scheda originale a diversi livelli di tensione e vi dice da quale livello il file viene letto correttamente (circa 0,25 V di picco per i nostri file).
