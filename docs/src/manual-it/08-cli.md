# La riga di comando

<p class="lead">Due programmi: apple1-emu-gui, l'interfaccia grafica, e apple1-emu, l'emulatore da terminale e cassetta degli attrezzi per le conversioni. Tutto ciò che fanno i pulsanti di esportazione si può fare anche dalla riga di comando, per i vostri script.</p>

!!! attention "Gli strumenti da riga di comando parlano francese"
    L'interfaccia grafica esiste in italiano (`--langue it`), ma `apple1-emu` mostra i suoi messaggi e il suo aiuto solo in francese, e le opzioni dell'interfaccia, i comandi di F2 e le parole chiave del debugger sono parole francesi (`--carte-couleur`, `charger`, `pas`…). Questo capitolo mostra l'output reale e lo traduce dove serve.

## L'emulatore da terminale

`apple1-emu` senza comando avvia l'Apple-1 nel terminale (almeno 42 colonne × 28 righe): 8 KB di RAM più `$E000`, ACI, velocità reale.

```sh
apple1-emu                                  # 8 KB + E000, ACI, velocità reale
apple1-emu --fast                           # avvio in modo rapido
apple1-emu --load samples/hello.txt --run 300
apple1-emu --tape samples/hello.wav         # cassetta nel registratore virtuale
apple1-emu --basic ~/apple1/integer_basic.bin
apple1-emu --ram 32 --no-e000               # 32 KB, niente RAM in E000
```

| Opzione | Effetto |
|---|---|
| `--ram KB` | RAM da `$0000`, da 4 a 48 KB (8 per impostazione predefinita) |
| `--no-e000` | niente RAM in `$E000-$EFFF` (il ponticello «W → E» della scheda) |
| `--no-aci` | nessuna scheda per cassetta |
| `--basic FILE` | il vostro Integer BASIC (binario di 4 KB o dump esadecimale), caricato in `$E000` |
| `--load FILE[@IND]` | un programma in memoria (`.txt`, `.bin@IND`, `.wav`, `.aif`); ripetibile |
| `--tape FILE` | una cassetta nel registratore |
| `--type FILE` | digita un file di testo all'avvio |
| `--run IND` | digita `INDR` dopo il caricamento |
| `--fast`, `--no-throttle` | modo rapido; visualizzazione senza il limite di 60 caratteri/s |
| `--no-refresh` | senza il modello del refresh della DRAM (1,0227 MHz invece di 0,960) |
| `--carte-couleur`, `--couleur-50hz` | la scheda GEN2 (registri e memoria; l'immagine a colori esiste solo nell'interfaccia grafica) |
| `--cursor-hz HZ`, `--cursor-duty F` | lampeggio del cursore (2,18 Hz, visibile per 2/3 per impostazione predefinita) |
| `--monitor-rom`, `--monitor-page` | un'altra immagine del Woz Monitor (256 o 512 byte, pagina 0 o 1) |
| `--aci-rom`, `--aci-page` | un'altra immagine della PROM dell'ACI |

**Tasti del terminale.** <kbd>Ctrl</kbd>+<kbd>R</kbd> RESET, <kbd>Ctrl</kbd>+<kbd>L</kbd> CLEAR SCREEN, <kbd>F1</kbd> aiuto, <kbd>F2</kbd> riga di comando, <kbd>F3</kbd> velocità, <kbd>F4</kbd> limite di visualizzazione, <kbd>F5</kbd>/<kbd>F6</kbd> riproduci/ferma la cassetta, <kbd>F10</kbd> o <kbd>Ctrl</kbd>+<kbd>Q</kbd> esci. Il testo incollato nel terminale viene digitato sulla tastiera dell'Apple-1. Backspace digita `_`, la cancellazione del Woz Monitor.

**Comandi di F2** (le parole dei comandi sono francesi).

| Comando | Effetto |
|---|---|
| {{charger}} `FILE[@IND]` | *caricare*: carica in memoria un `.txt`, un `.bin` (`@IND` obbligatorio), un `.wav` o un `.aiff` |
| {{sauver}} `FILE INIZIO.FINE …` | *salvare*: salva la memoria; l'estensione sceglie il formato. Gli intervalli si possono copiare dalla riga dell'ACI: `sauver st.aiff 004A.00FFR 0300.0FFFR` |
| {{cassette}} `FILE` | *cassetta*: inserisce una cassetta, che parte alla prima lettura dell'ACI |
| `lecture`, `stop`, `rembobiner`, `ejecter` | il registratore: riproduci, ferma, riavvolgi, espelli |
| {{enregistrer}} `FILE` | *registrare*: arma il registratore; poi digitate `C100R` e `0300.03FFW` |
| `basic FILE` | carica il vostro Integer BASIC in `$E000` |
| `taper FILE` | *digitare*: digita un file di testo sulla tastiera |
| `reset`, `effacer`, `rapide`, `reel`, `debit`, `aide`, `quitter` | reset, cancellare, rapido, velocità reale, ritmo di visualizzazione, aiuto, uscire |

## Conversioni

Il formato di uscita è scelto dall'**estensione**: `.aif`/`.aiff` dà un AIFF (profilo smartphone), `.wav` un WAV, `.txt`/`.hex` un dump Woz, `.bin` un binario.

{{export}} (alias di `export-wav`) produce una cassetta. Il segnale è creato dalla vera ROM dell'ACI eseguita nel 6502 emulato: i tempi dei bit sono quelli di un Apple-1 che esegue `W`.

```sh
apple1-emu export samples/hello.txt -o hello.aiff
apple1-emu export prog.bin@300 -o prog.wav --rate 48000 --bits 16
apple1-emu export startrek.txt --range "004A.00FFR 0300.0FFFR" -o startrek.aiff
```

| Opzione | WAV (predefinito) | AIFF (predefinito) |
|---|---|---|
| `--rate` | 44 100 Hz | 48 000 Hz |
| `--bits 8\|16` | 16 | 16 |
| `--channels 1\|2` | 1 | 2, identici |
| `--amplitude` (da 0,05 a 1) | 0,8 | 0,89 (−1 dBFS) |
| `--silence S` | 0,5 s | 1,0 s |
| `--range` | un intervallo, o una riga ACI intera; ripetibile | |
| `--no-refresh`, `--no-embed` | bit più corti del 6 %; nessun indirizzo nel file | |

{{import}} (alias di `import-wav`) decodifica una cassetta: `-o prog.txt` o `-o prog.bin`, `--addr IND` per una cassetta senza indirizzi, `--range` per collocare più registrazioni.

{{info}} descrive un file (registrazioni, indirizzi, durata); con `--aci-input stock|mimeo|gen2`, `--volts` e `--series` simula l'ingresso analogico dell'ACI (sezione 5.8).

{{convert}} `INGRESSO USCITA` converte tra `.txt`, `.bin`, `.wav` e `.aiff`.

## BASIC e assembler

```sh
apple1-emu basic gioco.bas -o gioco.wav          # listato tokenizzato -> cassetta
apple1-emu basic gioco.bas -o gioco.txt --ram 16 # dump Woz, per 16 KB
apple1-emu basic gioco.bas                       # solo controllo
apple1-emu asm gioco.s -o gioco.wav              # sorgente 6502 -> cassetta
apple1-emu asm gioco.s -o gioco.bin -l gioco.lst # binario + listato
apple1-emu asm gioco.s -I ../lib                 # controllo, percorso di inclusione in più
```

{{basic (riga di comando)|`basic`}} tokenizza un listato come fa l'interprete e scrive il blocco `$004A-$00FF`, poi il programma; `--ram 8|12|16|32` sceglie la macchina di destinazione (per impostazione predefinita, la più piccola che basta). {{asm (riga di comando)|`asm`}} assembla un sorgente ca65 (o xa); senza `-o` controlla soltanto. Entrambi mostrano la riga da digitare sul vero Apple-1.

```text title="Output di apple1-emu asm hello.s -o hello.txt"
32 octets, $0300-$031F ; 2 fichier(s), 13 symboles
Cassette (après C100R) : 0300.031FR   puis 300R
écrit : hello.txt
```

Cioè: «32 byte, $0300-$031F; 2 file, 13 simboli / Cassetta (dopo C100R): 0300.031FR poi 300R / scritto: hello.txt».

## Arte ASCII

{{ascii}} `IMMAGINE`: `--cols`, `--rows` (23 per tenere il prompt), `--zoom`, `--cx`, `--cy`, `--brightness`, `--contrast`, `--gamma`, `--invert`, `--dither`, `--no-auto-level`, `--ramp simple|full|slideshow|ELENCO`, `--format asm|basic`, `--title`, `-o` (`.txt`, `.bas`, `.s`, `.wav`, `.aif`).

## PROM

{{prom}} prepara i file per programmare vere PROM da 256 × 4 (82S129, 3601, 6301), nel formato dei dump a nibble:

```sh
apple1-emu prom split woz -o proms/     # PROM-A2.bin (D7-D4) e PROM-A1.bin (D3-D0)
apple1-emu prom split aci -o proms/     # PROM-A3.bin (D7-D4) e PROM-A4.bin (D3-D0)
apple1-emu prom join A2.bin A1.bin      # riunire una lettura e confrontarla
```

`--swap-a1-a7` serve solo per una scheda cablata secondo lo schema Apple del 1976, che incrocia le linee di indirizzo 1 e 7 della PROM A2.

Per programmare una EPROM dalla 2716 alla 27C512 o una EEPROM 28C dall'interfaccia grafica (un programmatore TL866II+, T48 o T56 con `minipro`), vedete il programmatore di EPROM, sezione 2.6.

## L'interfaccia grafica

`apple1-emu-gui [OPZIONI] [FILE…]` carica i file indicati all'avvio. Opzioni principali (i nomi delle opzioni sono francesi):

| Opzione | Effetto |
|---|---|
| `--zoom N\|ajuster`, `--taille LxA` | zoom dello schermo (`ajuster` = adatta), dimensione della finestra |
| `--phosphore vert\|blanc\|ambre` | fosfori (verde, bianco, ambra) |
| `--screening`, `--sans-screening`, `--screening-options ELENCO` | schermo vintage dell'Apple-1 e le sue impostazioni |
| `--gen2-screening` | schermo vintage sul monitor a colori GEN2 |
| `--theme clair\|sombre\|systeme` | tema chiaro, scuro o di sistema |
| `--taille-icones auto\|petite\|normale\|grande\|tres-grande` | dimensione di icone e testo |
| `--barre-complete` | non ridurre mai la barra degli strumenti alle sole icone |
| `--bibliotheque CARTELLA`, `--basic FILE` | cartella della libreria, file BASIC |
| `--carte-couleur`, `--sans-carte-couleur`, `--couleur-50hz` | scheda GEN2 |
| `--langue CODICE\|auto` | lingua dell'interfaccia (`--langue it` per l'italiano) |
| `--mode-rapide`, `--mode-cassette` | caricamento dalla libreria: istantaneo (predefinito) o dalla cassetta a velocità reale |
| `--cassette FILE` | inserisce una cassetta |
| `--ecran-separe`, `--debogueur` | schermo in una finestra propria, debugger aperto |
| `--editeur FILE`, `--ascii FILE` | apre un file nell'editor o nello strumento ASCII |

Per dimostrazioni e schermate, alcune opzioni ripetono azioni dopo qualche secondo: `--lancer-biblio S:NOME`, `--charger-biblio S:NOME`, `--parametres S:NOME`, `--preferences S` (e `--onglet-prefs interface|editeur|bibliotheque|cassette|gen2|avance`), `--editeur-lancer S`, `--editeur-recherche TESTO` (barra di ricerca aperta), `--editeur-menu` e `--ecran-menu` (menu del clic destro aperti), `--menu-etat` (il menu Stato aperto), `--eprom` (il programmatore di EPROM aperto), `--fenetre-fichiers` (la finestra Carica aperta), `--editeur-exporter S:FILE`, `--ascii-reglages R`, `--ascii-lancer S`, `--taper TESTO`, `--taper-a S:TESTO`, `--dbg-arret`, `--dbg-surveiller`, `--dbg-commande S:CMD`, `--capture IMMAGINE.png` (dopo `--delai S`) e `--quitter`.

# Risoluzione dei problemi e domande frequenti

## La finestra non si apre

Manca il driver OpenGL, oppure siete in una macchina virtuale. `run.sh` riprova automaticamente con il rendering software di Mesa; a mano: `LIBGL_ALWAYS_SOFTWARE=1 ./apple1-emu-gui`. Lo schermo vintage resta fluido con il rendering software.

## I tasti non arrivano all'Apple-1

Fate clic sullo schermo dell'Apple-1. Quando è attivo un campo di testo o una finestra di dialogo, la tastiera va a quel campo; la barra di stato mostra dove va la tastiera.

## «Integer BASIC introuvable» (Integer BASIC non trovato)

Il BASIC non è fornito. Mettete il vostro `BASIC.wav` (o un `.bin` di 4096 byte) nella libreria, oppure indicatelo con il pulsante **BASIC** o in **⚙ Preferenze → Integer BASIC**. L'emulatore lo trova poi da solo.

## Un programma BASIC si ferma con STOPPED AT

È stato premuto un tasto mentre girava: sull'Apple-1 qualsiasi tasto interrompe un programma BASIC. Digitate `RUN` per ripartire, oppure `GOTO` seguito dal numero di riga per riprendere senza cancellare le variabili.

## Il programma non parte dopo il caricamento

- Programma BASIC: serve `E2B3R` (ingresso a caldo), non `E000R`, che cancella il programma. Il BASIC deve essere in memoria **prima** del programma.
- Linguaggio macchina: controllate l'indirizzo di avvio nelle impostazioni ⚙; non è sempre `300R` (`280R`, `1000R`, `2000R`…).
- Programma più grande di 8 KB: la RAM viene aggiunta da ▶, ma non da un caricamento manuale. Avviatelo con ▶, oppure usate `--ram 16`.

## La cassetta non si carica sul vero Apple-1

<ol class="steps">
<li>Volume al massimo, tutti gli effetti audio disattivati, nessuna notifica.</li>
<li>RETURN durante l'intestazione, né prima dell'inizio della riproduzione né dopo la fine dell'intestazione.</li>
<li>Gli intervalli esatti, a quattro cifre: <code>004A.00FFR</code>, mai <code>4A.FFR</code>.</li>
<li>Provate un altro lettore (VLC), poi <code>--rate 44100</code>, <code>--channels 1</code> o <code>--no-refresh</code>.</li>
<li>Controllate il livello con <code>apple1-emu info prog.aiff --aci-input stock</code> (sezione 5.8).</li>
<li>Se il caricamento non finisce mai: RESET, poi riprovate con più volume.</li>
</ol>

## Lo schermo è pieno di @ in inverso

È il monitor a colori GEN2: la sua memoria parte da zero, e il codice `$00` è una `@` in inverso. Il programma non ha ancora riempito la pagina di testo. Sulla scheda vera il contenuto sarebbe casuale.

## I caratteri escono lentamente

È normale: il terminale dell'Apple-1 mostra circa 60 caratteri al secondo. Fate clic su **Schermo: ~60 car./s** nella barra di stato (o su **Veloce** nella riga di visualizzazione dello schermo) per la visualizzazione istantanea, e passate al modo rapido (<kbd>F3</kbd>) per accelerare tutto.

## Il suono della cassetta è muto

È disattivato a velocità rapida, e quando mancano ALSA o la scheda audio: l'oscilloscopio lo dice («muto a velocità massima», «nessuna uscita audio»). Il volume si regola nell'oscilloscopio; 🔊 nella barra degli strumenti spegne o accende il suono.

## Il progetto GitHub non si compila

«Prova l'ultima build» compila con ca65/ld65 (pacchetto `cc65`) se non trova né una Release né un artefatto. Installate `cc65`, oppure aggiungete un `apple1.toml` che indichi il file giusto. Alcuni sorgenti richiedono un ca65 recente (2.19 o successivo); l'errore di ca65 viene allora mostrato così com'è.

## Che cosa l'emulatore non fa

- Gli opcode non documentati del 6502 sono trattati come NOP della lunghezza giusta, e gli opcode JAM fermano il processore; i loro effetti esatti non sono riprodotti (il software dell'Apple-1 non li usa).
- Il comparatore dell'ACI è ideale durante l'emulazione; solo il comando `info` lo simula come circuito analogico.
- L'emulazione è stata verificata con le ROM, i listati e strumenti indipendenti (c2t, ca65, xa), ma l'autore di questo manuale non ha ancora riprodotto i file audio su un vero Apple-1.
