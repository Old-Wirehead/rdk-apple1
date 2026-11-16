# Appendici <span class="part-sub">Mappa della memoria, comandi del Woz Monitor, tempi dell'ACI, set di caratteri, scorciatoie da tastiera, trappole, licenza e riconoscimenti</span> {: .part data-part="Appendici" }

# Mappa della memoria {: .appendix }

![La memoria dell'Apple-1 nell'emulatore, dai vettori del monitor alla pagina zero.](diagrams/it/memoire.svg "diagram large")

| Indirizzi | Contenuto | Dettagli |
|---|---|---|
| `$0000-$00FF` | pagina zero | Woz Monitor `$24-$2B`; Integer BASIC `$4A-$FF` (puntatori LOMEM `$4A`, HIMEM `$4C`, PP `$CA`, PV `$CC`) |
| `$0100-$01FF` | stack del 6502 | |
| `$0200-$027F` | buffer di riga del monitor | 128 caratteri |
| `$0280-$0FFF` | RAM di base (4 KB) | programmi in linguaggio macchina; il BASIC tra LOMEM (`$0800`) e HIMEM (`$1000`) |
| `$1000-$BFFF` | RAM di espansione | 8 KB per impostazione predefinita nell'emulatore (fino a `$1FFF`), fino a 48 KB con `--ram` |
| `$C000-$C0FF` | ACI: inversione dell'uscita, ingresso cassetta | ogni accesso inverte l'uscita |
| `$C100-$C1FF` | ACI: PROM (il programma dell'ACI) | avviato con `C100R` |
| `$4000-$7FFF` | P-LAB CodeTank | EPROM da 16 KB, sola lettura, quando è inserita una ROM (sopra la RAM) |
| `$C250-$C257` | scheda GEN2: interruttori | lettura; pagine testo/LORES `$0400`, `$0800`, HIRES `$2000`, `$4000` |
| `$CC00-$CC01` | scheda P-LAB: TMS9918A | `$CC00` dati, `$CC01` controllo e stato; ripetuto su `$CC00-$CFFF` |
| `$D010-$D013` | PIA: `KBD`, `KBDCR`, `DSP`, `DSPCR` | capitolo 22 |
| `$E000-$EFFF` | BASIC | RDK G2 Basic (7933 byte, richiede RAM fino a `$FEFF`), o l'Integer BASIC di Apple (4 KB di RAM, caricato da cassetta o da un'immagine) |
| `$FF00-$FFFF` | Woz Monitor | vettori: NMI `$0F00`, RESET `$FF00`, IRQ/BRK `$0000` |

# Comandi del Woz Monitor {: .appendix }

Il monitor attende una riga terminata da <kbd>RETURN</kbd>. Indirizzi e byte si scrivono in esadecimale, senza `$`; una riga può contenere più comandi separati da spazi.

| Comando | Effetto | Esempio |
|---|---|---|
| `ind` | mostra il byte a quell'indirizzo | `FFEF` |
| `ind.ind` | mostra un intervallo, 8 byte per riga | `FF00.FF1F` |
| `.ind` | continua la visualizzazione fino a quell'indirizzo | `.FF3F` |
| `ind: byte byte …` | memorizza byte a partire dall'indirizzo | `300: A9 C1 20 EF FF` |
| `: byte …` | continua a memorizzare | `: 4C 1F FF` |
| `indR` | esegue il programma a quell'indirizzo (`JMP`) | `300R` |
| <kbd>Esc</kbd> | annulla la riga (stampa `\`) | |
| `_` (backspace) | annulla l'ultimo carattere | |

{{!indR}}{{!ind.ind}}

Una riga di più di 127 caratteri viene annullata come con Esc. Dopo `R`, il monitor stampa prima il byte all'indirizzo (`0300: A2`), poi il programma prende il controllo.

| Indirizzo | Comando | Ruolo |
|---|---|---|
| `E000R` | avvio a freddo del BASIC | cancella il programma |
| `E2B3R` | avvio a caldo del BASIC | conserva il programma |
| `C100R` | il programma dell'ACI | poi `inizio.fineR` (lettura) o `inizio.fineW` (scrittura) |
| `FF00R` | RESET software del monitor | |

# Tempi dell'ACI {: .appendix }

Le durate sono prodotte dal programma dell'ACI, contando i cicli tra due inversioni dell'uscita. Valori misurati nell'emulatore, che esegue questo programma sul suo 6502 (clock a 1,022727 MHz, con la sosta di 4 cicli ogni 65 cicli per il refresh della memoria):

| Elemento | Cicli | Durata | Frequenza |
|---|---|---|---|
| semiperiodo dell'intestazione | 592 | 618 µs | ≈ 809 Hz |
| prima metà del bit di start | 180 | 188 µs | |
| semiperiodo di uno «0» | 238 | 248 µs | ≈ 2,0 kHz |
| semiperiodo di un «1» | 473 | 494 µs | ≈ 1,0 kHz |
| intestazione completa | 16 390 semiperiodi | ≈ 10,1 s | |

Formato di una registrazione:

<ol class="steps">
<li>l'<strong>intestazione</strong>: circa 10 secondi di semiperiodi da 618 µs;</li>
<li>il <strong>bit di start</strong>: un semiperiodo corto (188 µs), poi un semiperiodo di «0»;</li>
<li>i <strong>dati</strong>: ogni byte, bit più significativo per primo, un periodo completo per bit;</li>
<li>nient'altro: né lunghezza, né indirizzo, né checksum. È il comando di lettura a fornire l'intervallo.</li>
</ol>

In lettura, il programma dell'ACI misura i semiperiodi con un ciclo di 12 cicli macchina. Prende come bit di start qualsiasi semiperiodo inferiore a 31 giri (≈ 372 cicli, 388 µs), e separa «0» da «1» a circa 0,74 ms per periodo completo. Il decodificatore di file dell'emulatore accetta intestazioni da 0,40 a 0,90 ms per semiperiodo, il che copre lo strumento c2t (1 kHz) e le variazioni di velocità del nastro.

# Il set di caratteri del 2513 {: .appendix }

![I 64 caratteri del Signetics 2513, come li disegna l'emulatore: codici da $20 a $5F (con il bit 7, da $A0 a $DF).](img/charset-2513.png "large")

Il generatore di caratteri conosce solo 64 simboli: spazio, punteggiatura, cifre e maiuscole. Il terminale conserva solo 6 bit del codice che riceve:

| Codice inviato | Visualizzazione |
|---|---|
| `$20-$5F` (o `$A0-$DF`) | il carattere della tabella |
| `$60-$7F` (minuscole, o `$E0-$FF`) | la maiuscola corrispondente (`$40-$5F`) |
| `$0D` (o `$8D`) | nuova riga |
| altri `$00-$1F` | nulla: ignorati |

Ogni carattere è una matrice di 5 × 7 punti in una cella di 7 × 8; lo schermo è di 40 × 24 caratteri, cioè 280 × 192 punti.

# Scorciatoie da tastiera {: .appendix }

| Tasto | Effetto |
|---|---|
| <kbd>Ctrl</kbd>+<kbd>R</kbd> | RESET dell'Apple-1 |
| <kbd>Ctrl</kbd>+<kbd>L</kbd> | CLEAR SCREEN |
| <kbd>Ctrl</kbd>+<kbd>V</kbd> | incolla: digita gli appunti sulla tastiera dell'Apple-1 |
| <kbd>F1</kbd> | modo aiuto (fumetti di aiuto dettagliati) |
| <kbd>F3</kbd> | velocità reale o rapida |
| <kbd>F11</kbd> | schermo intero |
| <kbd>Ctrl</kbd>+<kbd>+</kbd> / <kbd>Ctrl</kbd>+<kbd>−</kbd> / <kbd>Ctrl</kbd>+<kbd>0</kbd> | zoom dello schermo (o del testo dell'editor, quando l'editor ha il focus della tastiera) |
| <kbd>Ctrl</kbd>+<kbd>S</kbd> | salvare il file dell'editor |
| <kbd>Ctrl</kbd>+<kbd>F</kbd> / <kbd>Ctrl</kbd>+<kbd>H</kbd> / <kbd>Ctrl</kbd>+<kbd>G</kbd> | editor: cerca, sostituisci, vai alla riga |
| <kbd>Ctrl</kbd>+<kbd>/</kbd> | editor: commenta / togli commento (6502), metti / togli REM (BASIC) |
| <kbd>F5</kbd> / <kbd>F9</kbd> / <kbd>F12</kbd> | editor: esegui, punto di arresto sulla riga, vai alla definizione (6502) |
| <kbd>Ctrl</kbd>+lettera | carattere di controllo (`Ctrl+D` = `$84`) |
| clic destro su uno schermo | Copia, Incolla (digita il testo), Cancella schermo, Reset |
| <kbd>Esc</kbd> | ESC (`$9B`): annulla la riga nel monitor e nel BASIC |
| backspace | digita `_`, la cancellazione del monitor e del BASIC |

Nel **terminale** (`apple1-emu` senza finestra, capitolo 12):

| Tasto | Effetto |
|---|---|
| <kbd>F2</kbd> | riga di comando dell'emulatore |
| <kbd>F5</kbd> / <kbd>F6</kbd> | cassetta: riproduci, ferma |
| <kbd>F10</kbd> o <kbd>Ctrl</kbd>+<kbd>Q</kbd> | esci |

# Trappole e comportamenti da conoscere {: .appendix }

Piccole cose che sorprendono la prima volta. Nessuna è un bug; ognuna ha una ragione.

- **BASIC e RUN.** **▶ Esegui** dell'editor avvia il BASIC, carica il listato e digita `RUN` per voi. Un programma che chiede un input attende i vostri tasti sullo schermo dell'Apple-1: fate prima clic sullo schermo.
- **RND.** Il primo `RND` di un'esecuzione sarebbe sempre lo stesso su una macchina appena resettata; **▶ Esegui** attende un momento casuale prima di `RUN`, così un gioco non comincia sempre allo stesso modo.
- **Comandi della cassetta.** La riga «Cassetta, sul vero Apple-1:» sotto l'editor è il comando del Woz Monitor da digitare su una macchina vera (`C100R`, poi i comandi `R` degli intervalli) prima di riprodurre la cassetta. È un'indicazione, non qualcosa da digitare nell'emulatore.
- **La tastiera va in un solo posto alla volta.** La riga in fondo alla finestra dice dove: all'Apple-1, all'editor, a un campo. Un clic sullo schermo dell'Apple-1 manda i tasti alla macchina; un clic nell'editor li riprende. Dopo un clic su un errore nell'elenco dell'editor, i tasti tornano all'editor.
- **Debugger in pausa.** I tasti premuti mentre la macchina è in pausa non vanno persi, ma viene conservato solo l'**ultimo** (il registro della tastiera vera tiene un carattere); viene consegnato quando riprendete.
- **I punti di arresto vanno con il programma.** Caricare un altro programma (file, libreria, assembler) cancella i punti di arresto e le etichette del precedente. I punti di arresto che avete messo restano da una sessione all'altra solo finché non caricate altro.
- **Incollare con il tasto centrale.** Nell'editor, un clic centrale porta lì il cursore e incolla la **selezione primaria** di X11/Wayland (il testo appena evidenziato in qualsiasi applicazione). L'editor non *scrive* ancora la propria selezione nella selezione primaria: per copiare dall'editor usate <kbd>Ctrl</kbd>+<kbd>C</kbd>.
- **Finestre piccole.** Su un monitor largo meno di 1400 punti, la disposizione del primo avvio è editor e schermo; dal menu **⊞ Disposizione ▾** potete sceglierne un'altra. Su uno schermo da 1024 × 768 la barra degli strumenti scorre di lato: il pulsante **»** raggiunge il gruppo Cassetta, invece di tagliarlo a destra. Uno zoom fisso (<kbd>Ctrl</kbd>+<kbd>+</kbd>) più grande del pannello dello schermo viene mantenuto, e lo schermo scorre nel suo pannello; la barra di stato ve lo dice.
- **Salvare una scheda senza nome.** **Salva come…** (icona, <kbd>Ctrl</kbd>+<kbd>S</kbd> o il menu contestuale della scheda) non lascia mai due schede per lo stesso file: una scheda già aperta su quel file viene chiusa.
- **Primo avvio e la rete.** Il primo avvio contatta GitHub per scaricare il progetto dimostrativo (capitolo 4); togliete il progetto dall'elenco «Progetti GitHub» in **⚙ Preferenze** per smettere definitivamente.
- **Leggere la tastiera in G2 Basic.** Un tasto premuto mentre un programma gira lo ferma, come nell'originale, e `INPUT` attende. Un gioco che deve sorvegliare i tasti chiama `KEY(0)` nel suo ciclo principale (capitolo 15); da quel momento un tasto non ferma più il programma.
- **Etichette e punti di arresto.** Un punto di arresto su un'etichetta che non esiste ancora (digitato prima di assemblare o eseguire) viene conservato e impostato quando l'etichetta compare.
- **Disposizioni e finestre.** Una disposizione salvata ricorda i pannelli agganciati, le finestre staccate (posizione e dimensione) e la dimensione della finestra principale. Non ricorda quale finestra ha la tastiera.

# Licenza e riconoscimenti {: .appendix }

**Retro Dev Kit Apple 1** è software libero, distribuito con la **GNU General Public License, versione 3** (GPL-3.0), senza alcuna garanzia. Il testo completo è nel file `LICENSE` dell'archivio e del repository.

Alcune parti provengono da altri progetti e conservano la propria licenza, la licenza **MIT**, compatibile con la GPL; sono elencate nel file `NOTICE`:

- **RDK G2 Basic**, il BASIC integrato, scritto per questo progetto (repository `Old-Wirehead/rdk-gen2basic-apple1`, MIT);
- i **test GEN2 di Uncle Bernie** (da `gen2-test1` a `gen2-test4`, le barre) e i **glifi GEN2**, forniti da Uncle Bernie.

I giochi della libreria scritti da **Claude IA** (Impiccato, Fiammiferi, Battaglia navale, Tunnel e Tris) sono distribuiti con il progetto.

Ciò che il kit **non** contiene: l'Integer BASIC di Apple, le cassette commerciali e le demo della scheda P-LAB. Restano ai loro proprietari; portate le vostre copie.

La scheda P-LAB è una creazione di P-LAB; il TMS9918A è un chip di Texas Instruments. Questo progetto non è affiliato ad Apple, Texas Instruments o P-LAB.
