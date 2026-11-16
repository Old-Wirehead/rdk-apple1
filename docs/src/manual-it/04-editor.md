# L'editor

<p class="lead">L'editor integrato scrive programmi Integer BASIC e sorgenti 6502, li controlla a ogni tasto, li esegue nell'Apple-1 emulato con un clic e li esporta come cassette per la macchina vera.</p>

## Schede e file

**📝 Editor** apre il pannello, agganciato a sinistra dello schermo. La sua barra degli strumenti è compatta, per lasciare al testo il maggior numero di righe. Comincia con delle icone, spiegate dai loro fumetti:

| Icona | Azione |
|---|---|
| **Nuovo** (prima icona) | Nuova scheda: «Programma BASIC» o «Sorgente assembler 6502» |
| **📂** / **💾** / **💾…** | Apri (`.bas`, `.txt`, `.s`, `.asm`, `.a65`, `.inc`) / Salva (<kbd>Ctrl</kbd>+<kbd>S</kbd>) / Salva come… |
| **✂**, copia, **📋** | Taglia, copia, incolla |
| **🔍** | Cerca e sostituisci (<kbd>Ctrl</kbd>+<kbd>F</kbd>, <kbd>Ctrl</kbd>+<kbd>H</kbd>) |
| **⟲ ⟳** | Annulla (<kbd>Ctrl</kbd>+<kbd>Z</kbd>), ripeti (<kbd>Ctrl</kbd>+<kbd>Maiusc</kbd>+<kbd>Z</kbd>) |
| **A▲▼** | Dimensione del testo: un comando compatto, la lettera A con due piccoli triangoli sovrapposti; ▲ ingrandisce, ▼ riduce |

Dopo di esse, sulla stessa riga (o sulla successiva se il pannello è stretto), vengono le azioni della scheda: **▶ Esegui**, **📼 Esporta WAV** e **Memoria** per il BASIC, oppure **📃 Listato**, **⏺ Punto di arresto** e **⏩ Fino al cursore** per un sorgente 6502.

Ogni file è una scheda. Un punto • segna una scheda modificata; × la chiude (se non è salvata serve un secondo clic). Una scheda segnata con **📚** è stata aperta dalla libreria: aprire un listato o un sorgente della libreria (▶, ⬇ o doppio clic) apre la sua scheda quando l'editor è visibile, o la mostra se è già aperta. Le schede, compreso il testo non salvato, ritornano all'avvio seguente. Un clic destro su una scheda offre **Salva come…** e **Chiudi**. Nel testo, un **clic centrale** incolla la selezione primaria (Linux): il cursore va dove fate clic, poi viene inserito l'ultimo testo evidenziato in qualsiasi applicazione.

La dimensione del testo si regola con il comando **A▲▼**, <kbd>Ctrl</kbd>+rotellina, oppure <kbd>Ctrl</kbd>+<kbd>+</kbd> / <kbd>Ctrl</kbd>+<kbd>−</kbd> / <kbd>Ctrl</kbd>+<kbd>0</kbd> quando il testo ha il fuoco della tastiera.

**Stacca** mette l'editor in una finestra propria, per esempio su un secondo schermo; **⊟ Aggancia** lo rimette al suo posto. I tasti digitati in quella finestra vanno all'editor, mai all'Apple-1.

## Cercare, sostituire e il menu del clic destro

**🔍** (o <kbd>Ctrl</kbd>+<kbd>F</kbd>) apre una barra di ricerca sopra il testo: <kbd>Invio</kbd> va all'occorrenza seguente, <kbd>Maiusc</kbd>+<kbd>Invio</kbd> a quella precedente, **Aa** distingue le maiuscole, **Parola** trova parole intere, e il contatore mostra «3 di 7». <kbd>Ctrl</kbd>+<kbd>H</kbd> aggiunge il campo **Sostituisci con**, con **Sostituisci** (l'occorrenza corrente) e **Sostituisci tutto**. <kbd>Esc</kbd> chiude la barra.

![La barra di ricerca aperta su ECHO, e il menu del clic destro di un sorgente 6502.](img/it/editeur-menu.png "medium")

Il **clic destro** nel testo apre un menu con tutti i comandi e le loro scorciatoie. Il menu agisce sulla **selezione**, che resta quando ci fate clic destro sopra: **Taglia** e **Copia** lavorano sul testo selezionato. I comandi basati su una parola (vai alla definizione, aiuto) usano la **parola sotto il clic**, non quella sotto il cursore.

| Comando | Scorciatoia |
|---|---|
| Taglia, Copia, Incolla, Seleziona tutto | <kbd>Ctrl</kbd>+<kbd>X</kbd>, <kbd>C</kbd>, <kbd>V</kbd>, <kbd>A</kbd> |
| Cerca, Sostituisci | <kbd>Ctrl</kbd>+<kbd>F</kbd>, <kbd>Ctrl</kbd>+<kbd>H</kbd> |
| Vai alla riga… | <kbd>Ctrl</kbd>+<kbd>G</kbd> |
| Commenta / decommenta (6502), REM / togli REM (BASIC) | <kbd>Ctrl</kbd>+<kbd>/</kbd> |
| Indenta, Riduci rientro | |
| Vai alla definizione di un'etichetta (6502) o di un numero di riga (BASIC) | <kbd>F12</kbd> |
| Aiuto su *parola* (su una parola chiave, un'istruzione o una direttiva: apre la sua voce nel riferimento) | |
| Mostra nel disassemblato (6502) | |
| Rinumera… (BASIC: `GOTO`, `GOSUB` e `THEN` seguono) | |
| Punto di arresto su questa riga | <kbd>F9</kbd> |
| Assembla e avvia (6502), Invia e avvia (BASIC) | <kbd>F5</kbd> |

**Colori.** I colori del testo seguono il tema dell'interfaccia. **⚙ Preferenze → Editor** offre, separatamente per il BASIC e per l'assembler 6502, gli schemi *Automatica*, *Classico (scuro)*, *Chiaro*, *Ambra*, *Contrasto elevato*, oppure i vostri colori, un campione per tipo di parola (capitolo 11).

## Un programma BASIC

![L'editor BASIC: errori su fondo rosso, con il numero di riga e la colonna in fondo al pannello.](img/it/editeur-erreur.png "large")

Il testo è colorato: numeri di riga, parole chiave, numeri, stringhe, `REM`, operatori. Ogni riga viene **tokenizzata** come farebbe l'Integer BASIC, da un tokenizzatore nativo confrontato byte per byte con quello vero. Una riga che il BASIC rifiuterebbe diventa rossa, e l'errore indica la riga del testo, il numero di riga BASIC e la colonna; un clic vi porta il cursore. Errori rilevati: sintassi, nome di variabile non valido, numero maggiore di 32767, comando del modo diretto (`LIST`, `RUN`…) nel programma, stringa non chiusa, riga più lunga di 255 byte, memoria piena.

- **Memoria**: automatica (8 KB con LOMEM `$0800`, oppure `$0300` se il programma è grande, altrimenti 12, 16 o 32 KB), o imposta.
- {{▶ Esegui}}: carica l'Integer BASIC se manca (come il pulsante BASIC), mette in memoria il programma tokenizzato, aggiunge RAM se serve, poi digita `E2B3R` e `RUN`. È istantaneo: nulla viene digitato riga per riga.
- {{📼 Esporta WAV}}: una cassetta (`.wav`, oppure `.aiff` per uno smartphone) di due registrazioni, con le sue impostazioni nella libreria. In fondo al pannello compare che cosa digitare sul vero Apple-1, per esempio `C100R`, poi `004A.00FFR 0C3B.0FFFR`, poi `E2B3R` e `RUN`; **📋 Copia** lo mette negli appunti.

![Dall'editor al vero Apple-1: scrivere, eseguire, correggere, esportare.](diagrams/it/workflow-basic.svg "diagram large")

## Un sorgente 6502

![Un sorgente 6502 nell'editor: i colori, il risultato dell'assemblaggio e la riga da digitare sul vero Apple-1.](img/it/brut/editeur-asm.png "large")

Un sorgente viene assemblato **a ogni modifica** dall'assembler integrato a due passate, che accetta la sintassi di ca65 (e quella di xa, tradotta automaticamente). In fondo al pannello compaiono la dimensione e gli indirizzi (`33 byte, $0300-$0320 — 2 file, 13 simboli`), poi la riga per la macchina vera (`C100R`, poi `0300.0320R`, poi `300R`).

- **Errori**: file, riga e colonna; un clic vi porta, aprendo il file incluso se serve. Gli avvisi (pagina zero codificata come assoluta) sono in giallo.
- **📃 Listato**: indirizzo, byte e sorgente di ogni riga, con i file inclusi e le macro espanse, poi la tabella dei simboli; un clic riporta alla riga.
- **▶ Esegui**: mette i byte in memoria all'origine (`.org`, `$0300` per impostazione predefinita), aggiunge RAM se serve, poi digita per esempio `300R`.
- **📼 Esporta WAV**: la cassetta, come per il BASIC.
- **⏺ Punto di arresto** e **⏩ Fino al cursore**: il legame con il debugger (capitolo 8).
- **Legame con il debugger**: quando selezionate un'istruzione nel sorgente (o mettete il cursore sulla sua riga), i byte che ha prodotto vengono evidenziati nel **disassemblato** del debugger e nel **dump esadecimale / ASCII**, se quei pannelli sono visibili; se sono altrove in memoria, i pannelli scorrono per mostrarli. La tabella delle righe del sorgente e degli indirizzi dell'assembler rende possibile tutto questo.

![Un errore di assemblaggio: ECO invece di ECHO, un simbolo sconosciuto, segnalato con la sua riga e colonna.](img/it/brut/editeur-asm-erreur.png "large")

La Parte III è interamente dedicata all'assembler: sintassi, direttive, listato, esempi.
