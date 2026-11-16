# La cassetta e la scheda ACI

<p class="lead">L'Apple-1 non aveva né floppy disk né BASIC in ROM: tutto passava da una cassetta audio e dalla scheda ACI (Apple Cassette Interface). L'emulatore esegue la vera ROM dell'ACI, quindi i suoi file WAV e AIFF si caricano su un vero Apple-1, e le registrazioni fatte su un vero Apple-1 si caricano nell'emulatore.</p>

## Formati dei file

| Estensione | Contenuto | Lettura | Scrittura |
|---|---|---|---|
| `.wav` | cassetta ACI, PCM da 8 a 32 bit o virgola mobile, mono o stereo, qualsiasi frequenza | sì | 44,1 kHz mono a 16 bit per impostazione predefinita |
| `.aif`, `.aiff` | cassetta ACI (AIFF e AIFF-C) | sì | profilo smartphone: 48 kHz, 2 canali identici, 16 bit |
| `.txt`, `.hex`, `.apl` | dump del Woz Monitor: righe `0300: A9 C1 20 …` | sì | sì |
| `.bin` | byte grezzi, senza indirizzo (`FILE.bin@300`) | sì | sì |
| `.bas` | listato Integer BASIC (testo) | nell'editor | nell'editor |
| `.s`, `.asm`, `.a65` | sorgente 6502 (ca65 o xa) | nell'editor | nell'editor |

Le cassette esportate dall'emulatore portano i loro indirizzi in un blocco di commento del file (`APPLE1 ACI 0300.031E`), che i lettori audio ignorano. Una cassetta di altra provenienza non ha indirizzi: la libreria li riconosce o li indovina (capitolo 4), e la riga di comando accetta `--addr` o `--range`.

Il decodificatore tollera rumore, livelli bassi, polarità invertita, un errore di velocità del nastro di ±10 % e più registrazioni in un solo file. Legge anche i WAV prodotti dallo strumento **c2t** di Egan Ford. Un **ronzio di rete** (50 o 60 Hz, anche forte quanto il segnale della cassetta stesso) viene filtrato prima di rilevare i fronti, così anche una lunga registrazione fatta con un cavo poco schermato si legge correttamente.

## Il registratore virtuale

La striscia **Cassetta:** della barra degli strumenti raccoglie tutti i comandi: **📼 Inserisci**, **▶** riproduci, **⏹** ferma, **⏪** riavvolgi, **⏺** registra, **⏏** espelli, poi il nome e la posizione della cassetta, l'oscilloscopio **📈** e il suono **🔊**. Una cassetta si inserisce con **📼 Inserisci** (un `.wav` o un `.aiff`), o con il clic destro su un programma della libreria («Inserisci come cassetta»): digitate allora voi `C100R` e l'intervallo. L'icona **📼** di un programma della libreria fa un passo in più: inserisce la cassetta e digita i comandi per voi.

![La striscia Cassetta: durante la lettura di BASIC.wav: 7,6 secondi su 35,8.](img/it/barre-cassette.png "large")

La cassetta segue il tempo **emulato**: a velocità rapida (<kbd>F3</kbd>) il caricamento dura molto meno dell'intestazione di 10 secondi, e funziona lo stesso. Senza cassetta, i caricamenti dalla libreria sono istantanei (capitolo 4).

Caricare l'Integer BASIC «all'antica» si fa così:

```mon
» C100R
C100: A9*
» E000.EFFFR
» E000R

E000: 4C
>
```

`C100R` avvia il programma dell'ACI, che risponde con il suo prompt `*`. La riga `E000.EFFFR` le chiede di **leggere** la cassetta in `$E000-$EFFF`. Alla fine della cassetta, l'ACI torna al monitor (`\`); `E000R` avvia il BASIC, il cui prompt è `>`.

## Suono e oscilloscopio

![L'oscilloscopio durante la riproduzione di BASIC.wav: la casella «Suono della cassetta», il volume e il segnale a 1 ms per divisione.](img/it/oscilloscope.png "small")

Durante la riproduzione o la registrazione, **🔊** vi fa sentire il segnale, come l'altoparlante di un registratore, a volume basso per impostazione predefinita. **📈** apre l'**oscilloscopio**: una traccia verde su un reticolo scuro, 1 ms per divisione, che mostra 10 ms attorno alla testina o gli ultimi 10 millisecondi scritti dall'Apple-1. Si distinguono l'intestazione (≈ 800 Hz), i bit 0 (≈ 2 kHz) e i bit 1 (≈ 1 kHz). A velocità rapida il suono è muto.

## Registrare dall'Apple-1 emulato

Per produrre una cassetta come nel 1976, armate il registratore, poi chiedete all'ACI di scrivere:

<ol class="steps">
<li>Fate clic su <strong>⏺</strong> e scegliete il file (<code>.wav</code> o <code>.aiff</code>).</li>
<li>Digitate <code>C100R</code>, poi l'intervallo seguito da <code>W</code>, per esempio <code>0300.03FFW</code>, poi <kbd>RETURN</kbd>.</li>
<li>L'ACI scrive un'intestazione di 10 secondi, poi i dati. Il file viene scritto 1,5 s dopo la fine, o su <strong>⏹</strong>.</li>
</ol>

Il modo più semplice resta **Salva** nella barra degli strumenti, o **📼 Esporta WAV** nell'editor: il segnale è lo stesso, ciclo per ciclo, ed è prodotto all'istante.

## Come funziona l'ACI

![Le due direzioni dell'ACI: in scrittura, qualsiasi accesso a $C0xx inverte l'uscita; in lettura, il comparatore comanda la linea di indirizzo A0 della PROM.](diagrams/it/aci.svg "diagram large")

La scheda è notevolmente semplice: una PROM di 256 byte visibile in `$C000` e `$C100`, un flip-flop e un comparatore.

- **Scrittura.** Ogni accesso a un indirizzo in `$C000-$C0FF` inverte l'uscita (74LS74). Il programma della ROM fissa il tempo tra due accessi: quel tempo codifica i bit.
- **Lettura.** Il livello dell'ingresso del nastro, dopo il comparatore LM311, comanda la linea di indirizzo A0 della PROM quando A7 = 1. La stessa istruzione legge quindi `$C080` o `$C081` a seconda del livello: la ROM misura i semiperiodi con un ciclo di 12 colpi di clock.
- **Codifica.** Un «1» dura circa il doppio di uno «0»: 1 kHz per un 1, 2 kHz per uno 0. I byte escono con il bit più significativo per primo, senza parità né somma di controllo.

L'Appendice C dà i tempi esatti, in cicli e in microsecondi.

## I comandi dell'ACI

Dopo `C100R`, l'ACI attende una riga nel formato del monitor, che termina con `R` (lettura) o `W` (scrittura):

| Riga | Effetto |
|---|---|
| `0300.03FFR` | legge la cassetta in `$0300-$03FF` |
| `0300.03FFW` | scrive `$0300-$03FF` sulla cassetta, con 10 s di intestazione |
| `004A.00FFR 0300.0FFFR` | legge **due** registrazioni consecutive: la pagina zero del BASIC, poi il programma |
| `004A.00FFW800.FFFW` | la forma attaccata del manuale BASIC del 1976, accettata anch'essa |

{{!C100R}}{{!R (lettura)}}{{!W (scrittura)}}

!!! attention "Gli zeri di 004A.00FF sono obbligatori"
    L'analizzatore della ROM non azzera il suo accumulatore esadecimale al punto («Doesn’t clear HEX1!!!», dice il listato). Le cifre finali entrano dietro quelle iniziali: `4A.FF` significa `$004A-$4AFF`, quasi 19 KB! `0300.0FFF` funziona perché le quattro cifre spingono fuori il 3 di `0300`. Scrivete sempre i due indirizzi a quattro cifre. L'emulatore riproduce questo comportamento, poiché esegue la vera ROM.

Un programma BASIC si salva in due registrazioni: la pagina zero `$004A-$00FF` (che contiene i puntatori LOMEM e HIMEM e la fine del programma), poi il programma stesso, che termina sempre a HIMEM. Per ricaricarlo, il BASIC deve essere già in memoria; si rientra poi nel BASIC «a caldo» con `E2B3R`, che conserva il programma.

```mon
» C100R
C100: A9*
» 004A.00FFR 0300.0FFFR
» E2B3R

E2B3: 20
» >RUN
```

## Verso un vero Apple-1

È ciò che giustifica l'emulatore: preparare un programma sul PC, poi riprodurlo nell'ACI di una macchina vera, da uno smartphone o dall'uscita cuffie di un computer.

<ol class="steps">
<li><strong>Preparate il file.</strong> <strong>📼 Esporta WAV</strong> nell'editor, oppure <code>apple1-emu export prog.txt -o prog.aiff</code>. Scegliete <code>.aiff</code> per uno smartphone. Annotate la riga mostrata, per esempio <code>C100R</code> poi <code>0300.031ER</code>.</li>
<li><strong>Preparate il lettore.</strong> Volume al massimo (accettate l'avviso di «volume alto»), tutti gli effetti audio disattivati (equalizzatore, Dolby, «miglioramento audio»), modalità aereo, niente Bluetooth. Un lettore semplice come VLC va bene.</li>
<li><strong>Sull'Apple-1</strong>, digitate <code>C100R</code>: il monitor risponde <code>C100: A9*</code>.</li>
<li>Digitate la riga di lettura, per esempio <code>0300.031ER</code>, ma <strong>non premete ancora RETURN</strong>.</li>
<li>Avviate la riproduzione, poi premete <kbd>RETURN</kbd> <strong>durante l'intestazione</strong> (dura circa 10 s, dopo un secondo di silenzio).</li>
<li>Alla fine, l'ACI torna al monitor (<code>\</code>). Avviate il programma: <code>300R</code>, oppure <code>E2B3R</code> poi <code>RUN</code> per un programma BASIC.</li>
</ol>

**Perché questo profilo AIFF?** Non compresso a 16 bit, nessun codec con perdita: MP3 o AAC arrotonderebbero i fronti. 48 kHz, la frequenza nativa di Android, per evitare ogni ricampionamento. Due canali rigorosamente identici, perché un cavo che unisce sinistro e destro su FROM TAPE non deve mai ricevere due segnali diversi. Un'onda quadra a −1 dBFS: l'ingresso dell'ACI originale è un passa-alto (0,01 µF) che vede poco più dei fronti, e un'onda quadra con fronti ripidi è ciò che gli si addice meglio. Un secondo di silenzio all'inizio e alla fine, perché un telefono a volte taglia l'inizio di un suono.

!!! apple1 "Non fidatevi del LED"
    Il manuale del 1976 chiede un livello per cui il LED dell'ACI sia «appena acceso». Eppure si accende solo verso 1,2 V, molto più di quanto serva al comparatore. Un LED spento non prova quindi che il segnale sia troppo debole. Se il caricamento non finisce mai, la ROM sta ancora aspettando fronti: premete RESET e alzate il volume.

!!! attention "Mai MP3 né AAC"
    Un formato con perdita sposta i fronti e rende illeggibile la cassetta. Usate WAV, AIFF, oppure FLAC se il vostro lettore lo decodifica.

## Quale livello serve? I modelli di ingresso

Il comando `info --aci-input` simula l'ingresso analogico della scheda (condensatore di accoppiamento, polarizzazione, isteresi dell'LM311) per una serie di tensioni di picco, e vi dice da quale livello il file verrebbe letto:

```sh
apple1-emu info prog.aiff --aci-input stock     # scheda del 1976, 0,01 µF
apple1-emu info prog.aiff --aci-input mimeo     # 0,1 µF (consiglio della guida Mimeo)
apple1-emu info prog.aiff --aci-input gen2      # ACI Gen2 di Uncle Bernie
apple1-emu info prog.aiff --aci-input stock --volts 0.5 --series 1200
```

| Modello | Che cosa rappresenta |
|---|---|
| `stock` | l'ACI originale: 0,01 µF verso il comparatore polarizzato a 2,5 V, retroazione 3k/47k |
| `mimeo` | la modifica di Mike Willegal: 0,1 µF, più affidabile |
| `gen2` | l'ACI Gen2 di Uncle Bernie: 100 nF selezionati, 300k invece di 47k (meno isteresi) |

`--volts` fissa la tensione di picco al jack per il fondo scala digitale; `--series` aggiunge una resistenza in serie (per esempio gli 1,2 kΩ di un cavo per smartphone). Con il modello `stock`, i file dell'emulatore vengono letti a partire da **circa 0,25 V di picco**; l'uscita cuffie di uno smartphone al massimo dà di solito da 0,5 a 1 V. Questo modello è semplificato: niente rumore, niente deriva del punto di polarizzazione.

## Digitalizzare una vera cassetta o una registrazione dell'Apple-1

Per recuperare un programma digitato su un vero Apple-1, registrate l'uscita **TO TAPE** dell'ACI tramite un ingresso microfonico (un telefono, o la scheda audio di un PC con Audacity a 44,1 o 48 kHz), con il controllo automatico del guadagno e la riduzione del rumore disattivati. Sull'Apple-1: `C100R`, poi `0300.03FFW` e <kbd>RETURN</kbd>. Fermatevi qualche secondo dopo il ritorno del `\`, poi:

```sh
apple1-emu import registrazione.aiff --addr 300 -o prog.txt
apple1-emu import startrek.aiff --range "004A.00FFR 0300.0FFFR" -o startrek.txt
```

!!! apple1 "TO TAPE è un'uscita a livello microfonico"
    Fornisce solo circa 50 mV picco-picco. Controllate la forma d'onda in Audacity: niente saturazione, niente salti di livello durante l'intestazione. L'importazione dell'emulatore tollera qualsiasi guadagno e uno scostamento continuo.
