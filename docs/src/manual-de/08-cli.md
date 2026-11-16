# Die Befehlszeile

<p class="lead">Zwei Programme: apple1-emu-gui, die grafische Oberfläche, und apple1-emu, der Terminal-Emulator und Werkzeugkasten für Umwandlungen. Alles, was die Exportschaltflächen tun, geht auch auf der Befehlszeile, für Ihre Skripte.</p>

!!! attention "Die Befehlszeilenwerkzeuge sprechen Französisch"
    Die grafische Oberfläche gibt es auf Deutsch (`--langue de`), aber `apple1-emu` gibt seine Meldungen und seine Hilfe nur auf Französisch aus, und die Optionen der Oberfläche, die Befehle von F2 und die Schlüsselwörter des Debuggers sind französische Wörter (`--carte-couleur`, `charger`, `pas`…). Dieses Kapitel zeigt die echte Ausgabe und übersetzt sie, wo es nötig ist.

## Der Terminal-Emulator

`apple1-emu` ohne Befehl startet den Apple-1 im Terminal (mindestens 42 Spalten × 28 Zeilen): 8 KB RAM plus `$E000`, ACI, Originalgeschwindigkeit.

```sh
apple1-emu                                  # 8 KB + E000, ACI, Originalgeschwindigkeit
apple1-emu --fast                           # im schnellen Modus starten
apple1-emu --load samples/hello.txt --run 300
apple1-emu --tape samples/hello.wav         # Kassette im virtuellen Rekorder
apple1-emu --basic ~/apple1/integer_basic.bin
apple1-emu --ram 32 --no-e000               # 32 KB, kein RAM bei E000
```

| Option | Wirkung |
|---|---|
| `--ram KB` | RAM ab `$0000`, 4 bis 48 KB (standardmäßig 8) |
| `--no-e000` | kein RAM bei `$E000-$EFFF` (der Jumper „W → E“ der Platine) |
| `--no-aci` | keine Kassettenkarte |
| `--basic DATEI` | Ihr Integer BASIC (4-KB-Binärdatei oder Hex-Dump), nach `$E000` geladen |
| `--load DATEI[@ADR]` | ein Programm in den Speicher (`.txt`, `.bin@ADR`, `.wav`, `.aif`); wiederholbar |
| `--tape DATEI` | eine Kassette im Rekorder |
| `--type DATEI` | tippt beim Start eine Textdatei |
| `--run ADR` | tippt nach dem Laden `ADRR` |
| `--fast`, `--no-throttle` | schneller Modus; Anzeige ohne die Grenze von 60 Zeichen/s |
| `--no-refresh` | ohne das Modell der DRAM-Auffrischung (1,0227 MHz statt 0,960) |
| `--carte-couleur`, `--couleur-50hz` | die GEN2-Karte (Register und Speicher; das Farbbild gibt es nur in der grafischen Oberfläche) |
| `--cursor-hz HZ`, `--cursor-duty F` | Blinken des Cursors (standardmäßig 2,18 Hz, 2/3 sichtbar) |
| `--monitor-rom`, `--monitor-page` | ein anderes Abbild des Woz Monitors (256 oder 512 Bytes, Seite 0 oder 1) |
| `--aci-rom`, `--aci-page` | ein anderes Abbild des ACI-PROMs |

**Tasten im Terminal.** <kbd>Strg</kbd>+<kbd>R</kbd> RESET, <kbd>Strg</kbd>+<kbd>L</kbd> CLEAR SCREEN, <kbd>F1</kbd> Hilfe, <kbd>F2</kbd> Befehlszeile, <kbd>F3</kbd> Geschwindigkeit, <kbd>F4</kbd> Anzeigegrenze, <kbd>F5</kbd>/<kbd>F6</kbd> Kassette abspielen/anhalten, <kbd>F10</kbd> oder <kbd>Strg</kbd>+<kbd>Q</kbd> beenden. Ins Terminal eingefügter Text wird auf der Tastatur des Apple-1 getippt. Die Rücktaste tippt `_`, das Löschzeichen des Woz Monitors.

**Befehle von F2** (die Befehlswörter sind französisch).

| Befehl | Wirkung |
|---|---|
| {{charger}} `DATEI[@ADR]` | *laden*: lädt eine `.txt`, eine `.bin` (`@ADR` Pflicht), eine `.wav` oder eine `.aiff` in den Speicher |
| {{sauver}} `DATEI ANFANG.ENDE …` | *sichern*: sichert den Speicher; die Endung wählt das Format. Die Bereiche lassen sich aus der ACI-Zeile kopieren: `sauver st.aiff 004A.00FFR 0300.0FFFR` |
| {{cassette}} `DATEI` | *Kassette*: legt eine Kassette ein, die beim ersten Lesen der ACI startet |
| `lecture`, `stop`, `rembobiner`, `ejecter` | der Rekorder: Wiedergabe, Stopp, Zurückspulen, Auswerfen |
| {{enregistrer}} `DATEI` | *aufnehmen*: schaltet die Aufnahme scharf; dann `C100R` und `0300.03FFW` tippen |
| `basic DATEI` | lädt Ihr Integer BASIC nach `$E000` |
| `taper DATEI` | *tippen*: tippt eine Textdatei auf der Tastatur |
| `reset`, `effacer`, `rapide`, `reel`, `debit`, `aide`, `quitter` | Reset, löschen, schnell, Originalgeschwindigkeit, Anzeigerate, Hilfe, beenden |

## Umwandlungen

Das Ausgabeformat wird durch die **Endung** gewählt: `.aif`/`.aiff` ergibt ein AIFF (Smartphone-Profil), `.wav` ein WAV, `.txt`/`.hex` einen Woz-Dump, `.bin` eine Binärdatei.

{{export}} (Kurzform von `export-wav`) erzeugt eine Kassette. Das Signal erzeugt das echte ACI-ROM, das im emulierten 6502 läuft: Die Bitzeiten sind die eines Apple-1, der `W` ausführt.

```sh
apple1-emu export samples/hello.txt -o hello.aiff
apple1-emu export prog.bin@300 -o prog.wav --rate 48000 --bits 16
apple1-emu export startrek.txt --range "004A.00FFR 0300.0FFFR" -o startrek.aiff
```

| Option | WAV (Standard) | AIFF (Standard) |
|---|---|---|
| `--rate` | 44 100 Hz | 48 000 Hz |
| `--bits 8\|16` | 16 | 16 |
| `--channels 1\|2` | 1 | 2, identisch |
| `--amplitude` (0,05 bis 1) | 0,8 | 0,89 (−1 dBFS) |
| `--silence S` | 0,5 s | 1,0 s |
| `--range` | ein Bereich oder eine ganze ACI-Zeile; wiederholbar | |
| `--no-refresh`, `--no-embed` | Bits 6 % kürzer; keine Adressen in der Datei | |

{{import}} (Kurzform von `import-wav`) decodiert eine Kassette: `-o prog.txt` oder `-o prog.bin`, `--addr ADR` für eine Kassette ohne Adressen, `--range`, um mehrere Aufnahmen zu platzieren.

{{info}} beschreibt eine Datei (Aufnahmen, Adressen, Länge); mit `--aci-input stock|mimeo|gen2`, `--volts` und `--series` simuliert es den analogen Eingang der ACI (Abschnitt 5.8).

{{convert}} `EINGABE AUSGABE` wandelt zwischen `.txt`, `.bin`, `.wav` und `.aiff` um.

## BASIC und Assembler

```sh
apple1-emu basic spiel.bas -o spiel.wav          # tokenisiertes Listing -> Kassette
apple1-emu basic spiel.bas -o spiel.txt --ram 16 # Woz-Dump, für 16 KB
apple1-emu basic spiel.bas                       # nur prüfen
apple1-emu asm spiel.s -o spiel.wav              # 6502-Quelltext -> Kassette
apple1-emu asm spiel.s -o spiel.bin -l spiel.lst # Binärdatei + Listing
apple1-emu asm spiel.s -I ../lib                 # prüfen, zusätzlicher Include-Pfad
```

{{basic (Befehlszeile)|`basic`}} tokenisiert ein Listing wie der Interpreter und schreibt den Block `$004A-$00FF`, dann das Programm; `--ram 8|12|16|32` wählt die Zielmaschine (standardmäßig die kleinste, die genügt). {{asm (Befehlszeile)|`asm`}} assembliert einen ca65-Quelltext (oder einen von xa); ohne `-o` prüft er nur. Beide geben die Zeile aus, die am echten Apple-1 zu tippen ist.

```text title="Ausgabe von apple1-emu asm hello.s -o hello.txt"
33 octets, $0300-$0320 ; 2 fichier(s), 13 symboles
Cassette (après C100R) : 0300.0320R   puis 300R
écrit : hello.txt
```

Das heißt: „33 Bytes, $0300-$0320; 2 Datei(en), 13 Symbole / Kassette (nach C100R): 0300.0320R, dann 300R / geschrieben: hello.txt“.

## ASCII-Art

{{ascii}} `BILD`: `--cols`, `--rows` (23, um die Eingabeaufforderung zu behalten), `--zoom`, `--cx`, `--cy`, `--brightness`, `--contrast`, `--gamma`, `--invert`, `--dither`, `--no-auto-level`, `--ramp simple|full|slideshow|LISTE`, `--format asm|basic`, `--title`, `-o` (`.txt`, `.bas`, `.s`, `.wav`, `.aif`).

## PROMs

{{prom}} bereitet Dateien zum Programmieren echter PROMs mit 256 × 4 Bit vor (82S129, 3601, 6301), im Format der Nibble-Dumps:

```sh
apple1-emu prom split woz -o proms/     # PROM-A2.bin (D7-D4) und PROM-A1.bin (D3-D0)
apple1-emu prom split aci -o proms/     # PROM-A3.bin (D7-D4) und PROM-A4.bin (D3-D0)
apple1-emu prom join A2.bin A1.bin      # einen ausgelesenen Inhalt zusammensetzen und vergleichen
```

`--swap-a1-a7` ist nur für eine Platine, die nach dem Apple-Schaltplan von 1976 verdrahtet ist, der die Adressleitungen 1 und 7 des PROMs A2 kreuzt.

Um ein EPROM vom 2716 bis zum 27C512 oder ein EEPROM 28C aus der grafischen Oberfläche zu brennen (Programmer TL866II+, T48 oder T56 mit `minipro`), siehe das EPROM-Programmiergerät, Abschnitt 2.6.

## Die grafische Oberfläche

`apple1-emu-gui [OPTIONEN] [DATEI…]` lädt die angegebenen Dateien beim Start. Die wichtigsten Optionen (die Optionsnamen sind französisch):

| Option | Wirkung |
|---|---|
| `--zoom N\|ajuster`, `--taille BxH` | Zoom des Bildschirms (`ajuster` = anpassen), Fenstergröße |
| `--phosphore vert\|blanc\|ambre` | Phosphor (grün, weiß, bernstein) |
| `--screening`, `--sans-screening`, `--screening-options LISTE` | Vintage-Bildschirm des Apple-1 und seine Einstellungen |
| `--gen2-screening` | Vintage-Bildschirm auf dem GEN2-Farbmonitor |
| `--theme clair\|sombre\|systeme` | helles, dunkles oder System-Thema |
| `--taille-icones auto\|petite\|normale\|grande\|tres-grande` | Größe von Symbolen und Text |
| `--barre-complete` | die Werkzeugleiste nie auf Symbole verkleinern |
| `--bibliotheque ORDNER`, `--basic DATEI` | Bibliotheksordner, BASIC-Datei |
| `--carte-couleur`, `--sans-carte-couleur`, `--couleur-50hz` | GEN2-Karte |
| `--langue CODE\|auto` | Sprache der Oberfläche (`--langue de` für Deutsch) |
| `--mode-rapide`, `--mode-cassette` | Laden aus der Bibliothek: sofort (Standard) oder über die Kassette in Originalgeschwindigkeit |
| `--cassette DATEI` | legt eine Kassette ein |
| `--ecran-separe`, `--debogueur` | Bildschirm in einem eigenen Fenster, Debugger offen |
| `--editeur DATEI`, `--ascii DATEI` | öffnet eine Datei im Editor oder im ASCII-Werkzeug |

Für Vorführungen und Bildschirmfotos spielen einige Optionen nach einigen Sekunden Aktionen ab: `--lancer-biblio S:NAME`, `--charger-biblio S:NAME`, `--parametres S:NAME`, `--preferences S` (und `--onglet-prefs interface|editeur|bibliotheque|cassette|gen2|avance`), `--editeur-lancer S`, `--editeur-recherche TEXT` (Suchleiste offen), `--editeur-menu` und `--ecran-menu` (Rechtsklickmenüs offen), `--menu-etat` (das Menü Zustand offen), `--eprom` (das EPROM-Programmiergerät offen), `--fenetre-fichiers` (das Fenster Laden offen), `--editeur-exporter S:DATEI`, `--ascii-reglages R`, `--ascii-lancer S`, `--taper TEXT`, `--taper-a S:TEXT`, `--dbg-arret`, `--dbg-surveiller`, `--dbg-commande S:CMD`, `--capture BILD.png` (nach `--delai S`) und `--quitter`.

# Fehlersuche und häufige Fragen

## Das Fenster öffnet sich nicht

Der OpenGL-Treiber fehlt, oder Sie sind in einer virtuellen Maschine. `run.sh` versucht es automatisch erneut mit der Software-Darstellung von Mesa; von Hand: `LIBGL_ALWAYS_SOFTWARE=1 ./apple1-emu-gui`. Der Vintage-Bildschirm bleibt mit Software-Darstellung flüssig.

## Die Tasten erreichen den Apple-1 nicht

Klicken Sie auf den Apple-1-Bildschirm. Ist ein Textfeld oder ein Dialogfenster aktiv, geht die Tastatur in dieses Feld; die Statusleiste zeigt, wohin die Tastatur geht.

## „Integer BASIC introuvable“ (Integer BASIC nicht gefunden)

BASIC wird nicht mitgeliefert. Legen Sie Ihr `BASIC.wav` (oder eine `.bin` von 4096 Bytes) in die Bibliothek, oder zeigen Sie mit der Schaltfläche **BASIC** oder unter **⚙ Einstellungen → Integer BASIC** darauf. Der Emulator findet es dann von selbst.

## Ein BASIC-Programm hält mit STOPPED AT an

Während es lief, wurde eine Taste gedrückt: Beim Apple-1 unterbricht jede Taste ein BASIC-Programm. Tippen Sie `RUN`, um neu zu starten, oder `GOTO` mit der Zeilennummer, um fortzufahren, ohne die Variablen zu löschen.

## Das Programm startet nach dem Laden nicht

- BASIC-Programm: Sie brauchen `E2B3R` (Warmstart), nicht `E000R`, das das Programm löscht. BASIC muss **vor** dem Programm im Speicher sein.
- Maschinensprache: Prüfen Sie die Startadresse in den Einstellungen ⚙; es ist nicht immer `300R` (`280R`, `1000R`, `2000R`…).
- Programm größer als 8 KB: Der RAM wird von ▶ hinzugefügt, aber nicht beim Laden von Hand. Starten Sie es mit ▶, oder nehmen Sie `--ram 16`.

## Die Kassette lädt am echten Apple-1 nicht

<ol class="steps">
<li>Lautstärke auf Maximum, alle Audioeffekte aus, keine Benachrichtigungen.</li>
<li>RETURN während des Vorspanns, nicht vor dem Start der Wiedergabe und nicht nach dem Ende des Vorspanns.</li>
<li>Die genauen Bereiche, vierstellig: <code>004A.00FFR</code>, nie <code>4A.FFR</code>.</li>
<li>Einen anderen Abspieler versuchen (VLC), dann <code>--rate 44100</code>, <code>--channels 1</code> oder <code>--no-refresh</code>.</li>
<li>Den Pegel mit <code>apple1-emu info prog.aiff --aci-input stock</code> prüfen (Abschnitt 5.8).</li>
<li>Endet das Laden nie: RESET, dann mit mehr Lautstärke erneut versuchen.</li>
</ol>

## Der Bildschirm ist voller inverser @

Das ist der GEN2-Farbmonitor: Sein Speicher beginnt bei null, und der Code `$00` ist ein inverses `@`. Das Programm hat die Textseite noch nicht gefüllt. Auf der echten Karte wäre der Inhalt zufällig.

## Die Zeichen erscheinen langsam

Das ist normal: Das Terminal des Apple-1 zeigt etwa 60 Zeichen pro Sekunde. Klicken Sie in der Statusleiste auf **Anzeige: ~60 Zeichen/s** (oder auf **Schnell** in der Anzeigezeile des Bildschirms) für sofortige Anzeige, und schalten Sie in den schnellen Modus (<kbd>F3</kbd>), um alles zu beschleunigen.

## Der Kassettenton ist stumm

Er ist bei schneller Geschwindigkeit aus und wenn ALSA oder die Soundkarte fehlt: Das Oszilloskop sagt es („stumm bei voller Geschwindigkeit“, „keine Audioausgabe“). Die Lautstärke stellen Sie im Oszilloskop ein; 🔊 in der Werkzeugleiste schaltet den Ton aus oder ein.

## Das GitHub-Projekt lässt sich nicht bauen

„Neuesten Build testen“ baut mit ca65/ld65 (Paket `cc65`), wenn es weder eine Release noch ein Artefakt findet. Installieren Sie `cc65`, oder fügen Sie eine `apple1.toml` hinzu, die die richtige Datei nennt. Manche Quelltexte brauchen ein neueres ca65 (2.19 oder später); der Fehler von ca65 wird dann unverändert angezeigt.

## Was der Emulator nicht tut

- Undokumentierte 6502-Opcodes werden als NOPs der richtigen Länge behandelt, und JAM-Opcodes halten den Prozessor an; ihre genauen Wirkungen werden nicht wiedergegeben (die Software des Apple-1 benutzt sie nicht).
- Der Komparator der ACI ist während der Emulation ideal; nur der Befehl `info` simuliert ihn als analoge Schaltung.
- Die Emulation wurde mit den ROMs, den Listings und unabhängigen Werkzeugen (c2t, ca65, xa) geprüft, aber die Audiodateien hat der Autor dieses Handbuchs noch nicht in einen echten Apple-1 eingespielt.
