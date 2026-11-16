# La libreria

<p class="lead">La libreria è il punto di partenza di quasi tutto: conserva le vostre cassette, i dump e i sorgenti, riconosce le cassette dell'epoca e avvia un programma con un clic, BASIC compreso.</p>

![Il pannello della libreria: la cartella principale, la cartella «Art ASCII», cassette riconosciute e un progetto GitHub. BASIC.wav è in riproduzione.](img/it/bibliotheque.png "small")

## Cartelle

La cartella principale è `~/apple1/programmes`. Viene creata al primo avvio, con gli esempi suddivisi in cinque cartelle: **Art ASCII** (undici esempi di arte ASCII in 40 × 24, capitolo 9), **BASIC** (programmi `.bas` e dump BASIC `.hex.txt`), **6502** (sorgenti `.s`/`.asm` con i loro `.bin`/`.hex`), **GEN2 COLOR** (i test della scheda GEN2 di Bernie, `gen2-*.apl`, e i giochi a colori per G2 Basic) e **WAV** (tutte le cassette). Nel pannello queste cinque cartelle compaiono direttamente al livello superiore, nell'ordine **BASIC**, **6502**, **GEN2 COLOR**, **WAV**, **Art ASCII**: la cartella radice (il nome della directory sul disco) non è mostrata sopra di loro, e i file lasciati sciolti in quella cartella non sono elencati. Metteteli in una delle cinque cartelle. Una cartella che aggiungete, o un progetto GitHub, conserva il proprio titolo. Una libreria già esistente non viene mai riorganizzata, salvo che le sue cartelle **Assembleur** e **Uncle Bernie** (i nomi delle versioni da 7.8 a 7.9.1) vengono rinominate **6502** e **GEN2 COLOR**, con il loro contenuto. Per sceglierne un'altra: **⚙ Preferenze → Cartella della libreria → 📂 Sfoglia…** (**⟲ Predefinita** torna a `~/apple1/programmes`).

**+ Cartella**, in cima al pannello, aggiunge altre cartelle; la cartella principale resta sempre. La libreria è un albero: cartelle e sottocartelle si ripiegano, e il loro stato viene ricordato. Ogni cartella conserva le proprie impostazioni in un file `programs.json`. Clic destro su una cartella aggiunta per aprirla nel gestore di file o toglierla dall'elenco (sul disco non viene cancellato nulla).

**Pannello stretto.** La libreria può essere stretta quanto volete (fino a una larghezza minima, sotto la quale il separatore si ferma). Ogni voce allora si adatta: il nome occupa tutta la larghezza, tagliato con «…» se è troppo lungo (il fumetto mostra il nome completo), le piccole icone ▶ ⬇ 📼 ⚙ passano alla riga successiva e la descrizione è limitata a una o due righe, anch'essa tagliata con «…», con il testo completo nel fumetto. Nell'intestazione, **+ Cartella** e **+ Progetto GitHub** diventano icone con fumetti.

I file mostrati sono: `.wav .aiff .bin .hex .txt .apl .bas .s .asm .prg`. L'elenco si aggiorna da solo ogni due secondi: copiate un file nella cartella e compare.

Per ogni programma, l'elenco mostra il nome, il tipo, gli intervalli di indirizzi, la durata della cassetta, il comando di avvio e le vostre note.

### I giochi di Claude IA

Le cartelle **BASIC** e **6502** ricevono anche cinque piccoli giochi scritti da Claude IA, ciascuno in versione francese e inglese: *Impiccato* (`pendu.bas`, `hangman.bas`), *Fiammiferi* (`allumettes.bas`, `matches.bas`) e *Battaglia navale* (`bataille.bas`, `battleship.bas`) in BASIC; *Tunnel* (`tunnel.s`, `tunnel-en.s`) e *Tris* (`morpion.s`, `tictactoe.s`) in assembler 6502. Un doppio clic su un gioco BASIC lo avvia con RDK G2 Basic (l'emulatore avvia il BASIC, carica il listato e digita `RUN`); i sorgenti in assembler sono assemblati dall'assembler integrato. I sorgenti in assembler includono `apple1.inc`, installato insieme a loro. La cartella **GEN2 COLOR** contiene inoltre *Forza 4 a colori* (`puissance4-couleur.bas`), un gioco per G2 Basic e la scheda GEN2, di Claude IA. Q e S spostano la pedina sopra la griglia, Spazio la lascia cadere. I giochi fanno parte del progetto e sono sotto la sua licenza (appendice «Licenza e ringraziamenti»). Le versioni italiane di questi giochi si trovano in `docs/exemples/it/`.

## Avviare un programma

Ogni programma ha quattro piccole icone alla sua destra:

| Icona | Azione |
|---|---|
| **▶** | **Carica e avvia**: carica il programma se serve, poi lo avvia: RESET, poi il suo comando digitato sulla tastiera (`300R`, oppure `E2B3R` e `RUN` per un programma BASIC). Se il programma richiede l'Integer BASIC e questo non è in `$E000`, l'emulatore lo carica prima. Un listato BASIC viene digitato nell'Integer BASIC, un sorgente 6502 viene assemblato. |
| **⬇** | **Carica**: mette subito il programma in memoria, senza avviarlo. |
| **📼** | **Dalla cassetta**: come sulla macchina vera. L'emulatore preme RESET, digita `C100R` e poi il comando di lettura dell'ACI (per esempio `0300.031ER`), e la cassetta scorre a velocità reale, con il suono e una barra di avanzamento. I file `.txt` e `.bin` vengono prima convertiti in cassetta. Disattivato per un sorgente (`.bas`, `.s`), che non ha registrazione. |
| **⚙** | **Parametri**: intervalli, indirizzo di caricamento di un `.bin`, indirizzo e comando di avvio, tipo, se serve il BASIC, note. |

{{!doppio clic}}Il caricamento è **istantaneo**, a meno che non scegliate la cassetta con 📼. Un **doppio clic** su un programma lo carica e lo avvia se ha un comando di avvio (come ▶). La cassetta si usa solo quando la chiedete; il registratore della barra degli strumenti mostra allora la cassetta e la sua posizione.

Quando l'editor è aperto, ▶, ⬇ o un doppio clic su un listato BASIC o un sorgente 6502 lo **aprono anche in una scheda dell'editor**, segnata con 📚; se è già aperto, viene semplicemente mostrata la sua scheda.

Il **clic destro** su un programma offre il resto: carica, carica e avvia, inserisci come cassetta senza digitare nulla, parametri, rinomina, elimina (con conferma), apri la cartella.

## Le impostazioni di un programma

![Le impostazioni ⚙ di Hamurabi: due intervalli (pagina zero del BASIC, poi il programma), avviato con E2B3R poi RUN.](img/it/brut/bibliotheque-params.png "large")

| Campo | Ruolo |
|---|---|
| **Intervalli** | Le zone di memoria, scritte come per l'ACI: `0300.031E`, oppure `004A.00FF 0800.0FFF` per un programma BASIC. Ogni registrazione della cassetta va nel suo intervallo, in ordine. |
| **Indirizzo di caricamento** | Per un `.bin`, che non contiene indirizzi. |
| **Avvio** | L'indirizzo e il comando digitati da ▶. `\n` sta per RETURN tra due righe: `E2B3R\nRUN`. |
| **Tipo** | **Linguaggio macchina**, **Programma G2 Basic** (il valore predefinito per un listato BASIC, poiché l'emulatore lo fornisce) o **Programma Integer BASIC** (per chi preferisce l'originale di Apple: ▶ carica allora l'Integer BASIC originale scelto nelle preferenze). Il tipo scelto per un programma prevale sull'impostazione generale del BASIC nelle preferenze. |
| **Richiede un BASIC** | ▶ caricherà prima il BASIC del tipo del programma. |
| **Note** | Testo libero, mostrato nell'elenco. |

I valori sono precompilati a partire dal file e verificati al salvataggio; **⟲ Valori dal file** li rilegge.

!!! attention "Sempre quattro cifre: 004A.00FF, mai 4A.FF"
    Le impostazioni rifiutano `4A.FF`. Non è un capriccio: la ROM dell'ACI non azzera il suo accumulatore esadecimale al punto, quindi le cifre continuano a entrare e leggerebbe `$004A-$4AFF` (vedi sezione 5.6). Per questo l'emulatore scrive sempre gli indirizzi a quattro cifre.

## Cassette riconosciute

Quando un file viene **riconosciuto**, le sue impostazioni si compilano da sole e l'elenco mostra **✔ riconosciuto**. L'emulatore decodifica le registrazioni e confronta dimensioni e impronte SHA-256 con una tabella integrata: le 24 cassette della raccolta «Apple1Cassettes» di archive.org. Il nome del file allora non conta affatto. In mancanza di questo, riconosce il **nome** (ignorando maiuscole, spazi e trattini) con le stesse dimensioni delle registrazioni; il fumetto di aiuto segnala allora che il contenuto è diverso.

| Cassetta | Intervalli | Avvio |
|---|---|---|
| `BASIC.wav` (Integer BASIC) | `E000.EFFF` | `E000R` |
| `Hamurabi.wav`, `StarTrek.wav`, `Wumpus.wav`, `Slots.wav`, `Craps.wav`, `Bowling.wav`, `Buzzword.wav`, `WordCross.wav`, `AppleiEnhancedCheckers.wav`, `LunarlanderasciiGraphics.wav` | `004A.00FF 0300.0FFF` | `E2B3R ⏎ RUN` |
| `AceyDucey.wav`, `Blackjack.wav`, `HiLo.wav` | `004A.00FF 0800.0FFF` | `E2B3R ⏎ RUN` |
| `Football.wav` | `004A.00FF 0400.2FFF` | `E2B3R ⏎ RUN` (12 KB) |
| `DealOrNoDeal.wav`, `StarTrek2003.wav` | `004A.00FF 0300.3FFF` | `E2B3R ⏎ RUN` (16 KB) |
| `APPLE30TH.wav` | `0280.0FFF` | `280R` |
| `ASMmchess.wav` | `0300.0BFF` | `300R` |
| `Microchess.wav` | `1000.18FF` | `1000R` |
| `LunarLander.wav` | `0300.09FF` | `300R` |
| `Mastermind.wav` | `0300.03FF` | `300R` |
| `Pasart.wav` | `0300.05FF` | `300R` |
| `Life.wav` | `2000.21FF` | `2000R` (12 KB) |

Le vostre impostazioni non vengono mai sovrascritte: un programma che avete configurato voi conserva le sue impostazioni, compare solo il segno «riconosciuto». I programmi che superano gli 8 KB di base ricevono automaticamente la RAM necessaria all'avvio, come con una scheda di espansione.

**Cassetta sconosciuta.** L'emulatore indovina ciò che può: una registrazione di 4096 byte che inizia con `JMP` è l'Integer BASIC; due registrazioni di cui la prima è lunga 182 byte (`$4A-$FF`) sono un programma BASIC, collocato secondo i suoi puntatori LOMEM e HIMEM. L'elenco mostra allora «(ipotizzati)»; controllate con ⚙. Un programma in linguaggio macchina senza indirizzi non viene indovinato: indicatene l'intervallo.

![Digitalizzare una vera cassetta: dalla cassetta a un file, poi alla libreria.](diagrams/it/workflow-capture.svg "diagram large")

## Progetti GitHub

**+ Progetto GitHub** accetta un indirizzo (`https://github.com/jefftranter/6502/tree/master/asm/jmon`) o semplicemente `proprietario/repository`. I file utili vengono copiati in `~/.cache/apple1-emu/github/<proprietario>/<repository>/<commit>/`, e il progetto diventa un nodo dell'albero, con il suo commit, la data e il messaggio. **⟳ Aggiorna** scarica l'ultimo commit; 🌐 apre la pagina del repository. Tutti gli accessi alla rete avvengono in background.

{{Prova l'ultima build}} prova, in ordine:

<ol class="steps">
<li>i file dell'ultima <strong>Release</strong> del repository (<code>.wav</code>/<code>.aiff</code>, poi <code>.bin</code>, poi <code>.hex</code>/<code>.txt</code>);</li>
<li>l'ultimo <strong>artefatto di GitHub Actions</strong> (serve un token);</li>
<li><strong>la compilazione dei sorgenti</strong>: un listato BASIC viene digitato nell'Integer BASIC e avviato con <code>RUN</code>; un sorgente 6502 viene assemblato con <strong>ca65/ld65</strong> (pacchetto <code>cc65</code>), con <code>-D APPLE1</code> per impostazione predefinita; un dump Woz viene caricato.</li>
</ol>

![Un progetto GitHub, dall'indirizzo del repository al programma in esecuzione.](diagrams/it/workflow-github.svg "diagram large")

Un file `apple1.toml` (o `apple1.cfg`) nella radice del repository dice che cosa costruire:

```toml title="apple1.toml"
file    = "src/game.s"    # programma da avviare
load    = $0300           # indirizzo di caricamento (altrimenti il .org del sorgente)
run     = $0300           # indirizzo di avvio (altrimenti quello di caricamento)
ld65    = "apple1.cfg"    # configurazione di ld65 (altrimenti -t none)
defines = "APPLE1"        # simboli -D per ca65
```

!!! astuce "Il token GitHub è facoltativo"
    I repository pubblici non ne hanno bisogno. Un token (variabile `GITHUB_TOKEN`, o **⚙ Preferenze**) alza il limite delle richieste e dà accesso ai vostri repository privati e agli artefatti. Non viene mai mostrato né scritto nei messaggi, e `gui.json` diventa allora leggibile solo dal vostro account.

## La cartella «Art ASCII»

La libreria contiene anche una cartella **Art ASCII**: disegni nel formato dello schermo (mela, scheda dell'Apple-1, chip 6502, cassetta, razzo…) e ritratti di Steve Wozniak e Steve Jobs convertiti da foto con licenza libera. Un `.txt` di arte ASCII mostra un'anteprima quando è selezionato, con **→ BASIC**, **→ 6502** e **🎨 Apri nello strumento**; **▶** avvia subito la sua versione 6502. Il capitolo 9 descrive lo strumento.
