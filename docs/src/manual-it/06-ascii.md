# Foto → ASCII

<p class="lead">Lo strumento Foto → ASCII trasforma una foto in un'immagine Apple-1 di 40 × 24 caratteri, poi in un programma BASIC o 6502, pronto da avviare o da esportare su cassetta.</p>

![Lo strumento Foto → ASCII: la foto con il suo riquadro di ritaglio, le impostazioni e l'anteprima disegnata con il carattere del 2513.](img/it/brut/ascii-outil.png "large")

## Aprire un'immagine

**🎨 ASCII** apre lo strumento, un pannello agganciabile come gli altri. **Trascinate** una foto `.jpg` o `.png` sulla finestra, oppure usate **📂 Apri…** (una foto, o un'arte ASCII già pronta in `.txt`).

Il riquadro verde sulla foto mostra la parte che viene convertita: trascinatelo per spostarlo, la rotellina del mouse fa lo zoom.

## Impostazioni

| Impostazione | Ruolo |
|---|---|
| **Colonne**, **Righe** | al massimo 40 × 24. **23 righe** lascia l'ultima riga al prompt, perché l'immagine non scorra. |
| **Zoom**, **Centro** | la parte della foto conservata |
| **Luminosità**, **Contrasto**, **Gamma** | le impostazioni consuete (gamma > 1 schiarisce) |
| **Inverti** | scuro = molto inchiostro |
| **Livelli auto** | allarga i livelli di grigio (dal 2 % al 98 %) prima delle altre impostazioni |
| **Dithering** | Floyd-Steinberg: più sfumature, più rumore |
| **Caratteri** | la scala **completa** (26 livelli), **Semplice** (10), quella della **presentazione di Schmenk**, oppure **Personalizzata** |

Sono possibili solo i caratteri del 2513: maiuscole, cifre e simboli, niente minuscole, niente `"` e niente `_`. Sono ordinati in base al numero di punti accesi nel carattere dell'Apple-1: è quel numero, non la forma, a dare il livello di grigio.

!!! astuce "Per un ritratto"
    40 × 24 caratteri sono pochi: ritagliate sul volto (zoom da 1,3 a 1,6), alzate un po' il contrasto e provate un gamma inferiore a 1 per una foto chiara. Uno sfondo scuro e uniforme dà i risultati migliori.

## Il programma prodotto

- **BASIC**: una riga `PRINT "…"` per ogni riga del disegno. Una riga piena di 40 caratteri termina con `;`, perché lo schermo va a capo da solo. Il programma è lento da digitare (circa 2 s per riga).
- **6502**: un sorgente ca65 a `$0300`, avviato con `300R`. Dopo aver pulito lo schermo, un ciclo invia a ECHO una tabella compressa (sequenze di caratteri identici codificate come «numero + carattere»). La visualizzazione è istantanea.

Con 24 righe, il programma attende un tasto prima di restituire il controllo; con 23, il prompt arriva sull'ultima riga e l'immagine resta intera.

I pulsanti: **✎ Inserisci nell'editor** (nuova scheda BASIC o 6502), **▶ Esegui**, **📼 Esporta WAV…**, **💾 Salva nella libreria** (un `.txt` nella cartella «Art ASCII»), **📋 Copia il testo**, **Mostra il codice**.

![Dalla foto al vero Apple-1: conversione, programma, cassetta.](diagrams/it/workflow-ascii.svg "diagram large")

## Sulla riga di comando

```sh
apple1-emu ascii foto.jpg                        # solo il disegno, sullo schermo
apple1-emu ascii woz.jpg --zoom 1.3 --cx 0.45 --cy 0.32 \
          --contrast 1.3 --gamma 0.45 --rows 23 -o woz.txt
apple1-emu ascii woz.jpg -o woz.bas              # programma BASIC
apple1-emu ascii woz.jpg -o woz.s                # sorgente 6502
apple1-emu ascii woz.jpg -o woz.wav              # cassetta 6502, avviata con 300R
apple1-emu ascii woz.jpg -o woz.wav --format basic
```

Il capitolo 12 dà tutte le opzioni.

!!! apple1 "La presentazione di Dave Schmenk"
    Nel 2014 Dave Schmenk ha scritto una presentazione di ritratti per l'Apple-1. La libreria la aggiunge come progetto GitHub (`dschmenk/Apple1-Slideshow`): viene scaricata, non fornita, perché il repository non ha licenza. **▶ Prova l'ultima build** la assembla (sintassi di xa, tradotta automaticamente; gli stessi 3456 byte di xa 2.4.1) e la avvia con `280R`.
