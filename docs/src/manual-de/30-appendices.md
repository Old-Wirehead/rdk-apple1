# Anhänge <span class="part-sub">Speicherkarte, Befehle des Woz Monitors, Zeiten der ACI, Zeichensatz, Tastenkürzel, Fallstricke, Lizenz und Danksagungen</span> {: .part data-part="Anhänge" }

# Speicherkarte {: .appendix }

![Der Speicher des Apple-1 im Emulator, von den Vektoren des Monitors bis zur Zero Page.](diagrams/de/memoire.svg "diagram large")

| Adressen | Inhalt | Einzelheiten |
|---|---|---|
| `$0000-$00FF` | Zero Page | Woz Monitor `$24-$2B`; Integer BASIC `$4A-$FF` (Zeiger LOMEM `$4A`, HIMEM `$4C`, PP `$CA`, PV `$CC`) |
| `$0100-$01FF` | 6502-Stapel | |
| `$0200-$027F` | Zeilenpuffer des Monitors | 128 Zeichen |
| `$0280-$0FFF` | Grund-RAM (4 KB) | Maschinenprogramme; BASIC zwischen LOMEM (`$0800`) und HIMEM (`$1000`) |
| `$1000-$BFFF` | Erweiterungs-RAM | im Emulator standardmäßig 8 KB (bis `$1FFF`), bis 48 KB mit `--ram` |
| `$C000-$C0FF` | ACI: Umschalten des Ausgangs, Kassetteneingang | jeder Zugriff schaltet den Ausgang um |
| `$C100-$C1FF` | ACI: PROM (das Programm der ACI) | gestartet mit `C100R` |
| `$4000-$7FFF` | P-LAB CodeTank | EPROM mit 16 KB, nur lesbar, wenn ein ROM eingesteckt ist (über dem RAM) |
| `$C250-$C257` | GEN2-Karte: Schalter | lesen; Seiten Text/LORES `$0400`, `$0800`, HIRES `$2000`, `$4000` |
| `$CC00-$CC01` | P-LAB-Karte: TMS9918A | `$CC00` Daten, `$CC01` Steuerung und Status; wiederholt über `$CC00-$CFFF` |
| `$D010-$D013` | PIA: `KBD`, `KBDCR`, `DSP`, `DSPCR` | Kapitel 22 |
| `$E000-$EFFF` | BASIC | RDK G2 Basic (7933 Bytes, braucht RAM bis `$FEFF`) oder Apples Integer BASIC (4 KB RAM, von Kassette oder aus einem Abbild geladen) |
| `$FF00-$FFFF` | Woz Monitor | Vektoren: NMI `$0F00`, RESET `$FF00`, IRQ/BRK `$0000` |

# Befehle des Woz Monitors {: .appendix }

Der Monitor wartet auf eine mit <kbd>RETURN</kbd> abgeschlossene Zeile. Adressen und Bytes werden hexadezimal geschrieben, ohne `$`; eine Zeile darf mehrere durch Leerzeichen getrennte Befehle enthalten.

| Befehl | Wirkung | Beispiel |
|---|---|---|
| `adr` | zeigt das Byte an dieser Adresse | `FFEF` |
| `adr.adr` | zeigt einen Bereich, 8 Bytes pro Zeile | `FF00.FF1F` |
| `.adr` | setzt die Anzeige bis zu dieser Adresse fort | `.FF3F` |
| `adr: byte byte …` | speichert Bytes ab der Adresse | `300: A9 C1 20 EF FF` |
| `: byte …` | setzt das Speichern fort | `: 4C 1F FF` |
| `adrR` | startet das Programm an dieser Adresse (`JMP`) | `300R` |
| <kbd>Esc</kbd> | verwirft die Zeile (gibt `\` aus) | |
| `_` (Rücktaste) | hebt das letzte Zeichen auf | |

{{!adrR}}{{!adr.adr}}

Eine Zeile über 127 Zeichen wird wie mit Esc verworfen. Nach `R` gibt der Monitor zuerst das Byte an der Adresse aus (`0300: A2`), dann übernimmt das Programm.

| Adresse | Befehl | Rolle |
|---|---|---|
| `E000R` | Kaltstart von BASIC | löscht das Programm |
| `E2B3R` | Warmstart von BASIC | behält das Programm |
| `C100R` | das Programm der ACI | dann `anfang.endeR` (lesen) oder `anfang.endeW` (schreiben) |
| `FF00R` | Software-RESET des Monitors | |

# Zeiten der ACI {: .appendix }

Die Dauern erzeugt das Programm der ACI, indem es Takte zwischen zwei Umschaltungen des Ausgangs zählt. Im Emulator gemessene Werte, der dieses Programm auf seinem 6502 ausführt (Takt 1,022727 MHz, mit dem Halt von 4 Takten alle 65 Takte für die Speicherauffrischung):

| Element | Takte | Dauer | Frequenz |
|---|---|---|---|
| Halbperiode des Vorspanns | 592 | 618 µs | ≈ 809 Hz |
| erste Hälfte des Startbits | 180 | 188 µs | |
| Halbperiode einer „0“ | 238 | 248 µs | ≈ 2,0 kHz |
| Halbperiode einer „1“ | 473 | 494 µs | ≈ 1,0 kHz |
| vollständiger Vorspann | 16 390 Halbperioden | ≈ 10,1 s | |

Aufbau einer Aufnahme:

<ol class="steps">
<li>der <strong>Vorspann</strong>: etwa 10 Sekunden Halbperioden von 618 µs;</li>
<li>das <strong>Startbit</strong>: eine kurze Halbperiode (188 µs), dann eine Halbperiode einer „0“;</li>
<li>die <strong>Daten</strong>: jedes Byte, höchstwertiges Bit zuerst, eine volle Periode pro Bit;</li>
<li>sonst nichts: keine Länge, keine Adresse, keine Prüfsumme. Der Lesebefehl liefert den Bereich.</li>
</ol>

Beim Lesen misst das Programm der ACI Halbperioden mit einer Schleife von 12 Takten. Als Startbit nimmt es jede Halbperiode unter 31 Durchläufen (≈ 372 Takte, 388 µs), und es trennt „0“ und „1“ bei etwa 0,74 ms pro voller Periode. Der Dateidecoder des Emulators nimmt Vorspänne von 0,40 bis 0,90 ms pro Halbperiode an, was das Werkzeug c2t (1 kHz) und Gleichlaufschwankungen abdeckt.

# Der Zeichensatz des 2513 {: .appendix }

![Die 64 Zeichen des Signetics 2513, so wie der Emulator sie zeichnet: Codes $20 bis $5F (mit Bit 7, $A0 bis $DF).](img/charset-2513.png "large")

Der Zeichengenerator kennt nur 64 Symbole: Leerzeichen, Satzzeichen, Ziffern und Großbuchstaben. Das Terminal behält nur 6 Bit des empfangenen Codes:

| Gesendeter Code | Anzeige |
|---|---|
| `$20-$5F` (oder `$A0-$DF`) | das Zeichen aus der Tabelle |
| `$60-$7F` (Kleinbuchstaben, oder `$E0-$FF`) | der entsprechende Großbuchstabe (`$40-$5F`) |
| `$0D` (oder `$8D`) | neue Zeile |
| übrige `$00-$1F` | nichts: ignoriert |

Jedes Zeichen ist eine Punktmatrix von 5 × 7 in einer Zelle von 7 × 8; der Bildschirm hat 40 × 24 Zeichen, also 280 × 192 Punkte.

# Tastenkürzel {: .appendix }

| Taste | Wirkung |
|---|---|
| <kbd>Strg</kbd>+<kbd>R</kbd> | RESET des Apple-1 |
| <kbd>Strg</kbd>+<kbd>L</kbd> | CLEAR SCREEN |
| <kbd>Strg</kbd>+<kbd>V</kbd> | einfügen: tippt die Zwischenablage auf der Tastatur des Apple-1 |
| <kbd>F1</kbd> | Hilfemodus (ausführliche Hilfeblasen) |
| <kbd>F3</kbd> | Originalgeschwindigkeit oder schnell |
| <kbd>F11</kbd> | Vollbild |
| <kbd>Strg</kbd>+<kbd>+</kbd> / <kbd>Strg</kbd>+<kbd>−</kbd> / <kbd>Strg</kbd>+<kbd>0</kbd> | Zoom des Bildschirms (oder des Editortexts, wenn der Editor den Tastaturfokus hat) |
| <kbd>Strg</kbd>+<kbd>S</kbd> | die Datei des Editors speichern |
| <kbd>Strg</kbd>+<kbd>F</kbd> / <kbd>Strg</kbd>+<kbd>H</kbd> / <kbd>Strg</kbd>+<kbd>G</kbd> | Editor: suchen, ersetzen, zu Zeile gehen |
| <kbd>Strg</kbd>+<kbd>/</kbd> | Editor: aus-/einkommentieren (6502), REM setzen/entfernen (BASIC) |
| <kbd>F5</kbd> / <kbd>F9</kbd> / <kbd>F12</kbd> | Editor: starten, Haltepunkt auf der Zeile, zur Definition gehen (6502) |
| <kbd>Strg</kbd>+Buchstabe | Steuerzeichen (`Ctrl+D` = `$84`) |
| Rechtsklick auf einen Bildschirm | Kopieren, Einfügen (den Text tippen), Bildschirm löschen, Reset |
| <kbd>Esc</kbd> | ESC (`$9B`): verwirft die Zeile im Monitor und in BASIC |
| Rücktaste | tippt `_`, das Löschzeichen des Monitors und von BASIC |

Im **Terminal** (`apple1-emu` ohne Fenster, Kapitel 12):

| Taste | Wirkung |
|---|---|
| <kbd>F2</kbd> | Befehlszeile des Emulators |
| <kbd>F5</kbd> / <kbd>F6</kbd> | Kassette: abspielen, anhalten |
| <kbd>F10</kbd> oder <kbd>Strg</kbd>+<kbd>Q</kbd> | beenden |

# Fallstricke und Verhalten, die man kennen sollte {: .appendix }

Kleinigkeiten, die beim ersten Mal überraschen. Keine davon ist ein Fehler; jede hat einen Grund.

- **BASIC und RUN.** **▶ Starten** im Editor startet BASIC, lädt das Listing und tippt `RUN` für Sie. Ein Programm, das eine Eingabe verlangt, wartet auf dem Apple-1-Bildschirm auf Ihre Tasten: Klicken Sie zuerst auf den Bildschirm.
- **RND.** Das erste `RND` eines Laufs wäre auf einer frisch zurückgesetzten Maschine immer dasselbe; **▶ Starten** wartet vor `RUN` einen zufälligen Moment, damit ein Spiel nicht immer gleich beginnt.
- **Kassettenbefehle.** Die Zeile „Kassette, am echten Apple-1:“ unter dem Editor ist der Befehl des Woz Monitors, den Sie an einer echten Maschine tippen (`C100R`, dann die Befehle `R` der Bereiche), bevor Sie die Kassette abspielen. Es ist ein Hinweis, nichts, was im Emulator zu tippen wäre.
- **Die Tastatur geht immer an eine Stelle.** Die Zeile unten im Fenster sagt, wohin: an den Apple-1, den Editor, ein Feld. Ein Klick auf den Apple-1-Bildschirm schickt die Tasten an die Maschine; ein Klick in den Editor holt sie zurück. Nach einem Klick auf einen Fehler in der Liste des Editors kehren die Tasten in den Editor zurück.
- **Angehaltener Debugger.** Tasten, die getippt werden, während die Maschine angehalten ist, gehen nicht verloren, aber nur die **letzte** wird behalten (das Register der echten Tastatur hält ein Zeichen); sie wird geliefert, wenn Sie fortsetzen.
- **Haltepunkte gehören zum Programm.** Das Laden eines anderen Programms (Datei, Bibliothek, Assembler) löscht die Haltepunkte und Marken des vorigen. Ihre Haltepunkte bleiben von einer Sitzung zur nächsten nur, solange Sie nichts anderes laden.
- **Einfügen mit der mittleren Taste.** Im Editor setzt ein Mittelklick den Cursor dorthin und fügt die **primäre Auswahl** von X11/Wayland ein (den Text, den Sie gerade in irgendeiner Anwendung markiert haben). Der Editor *schreibt* seine eigene Auswahl noch nicht in die primäre Auswahl: Um aus dem Editor zu kopieren, nehmen Sie <kbd>Strg</kbd>+<kbd>C</kbd>.
- **Kleine Fenster.** Auf einem Monitor schmaler als 1400 Punkte ist die Anordnung beim ersten Start Editor und Bildschirm; im Menü **⊞ Anordnung ▾** können Sie eine andere wählen. Auf einem Bildschirm von 1024 × 768 rollt die Werkzeugleiste seitwärts: Die Schaltfläche **»** erreicht die Gruppe Kassette, statt sie rechts abzuschneiden. Ein fester Zoom (<kbd>Strg</kbd>+<kbd>+</kbd>), der größer als der Bildschirmbereich ist, bleibt erhalten, und der Bildschirm rollt in seinem Bereich; die Statusleiste sagt es Ihnen.
- **Einen Tab ohne Namen speichern.** **Speichern unter…** (Symbol, <kbd>Strg</kbd>+<kbd>S</kbd> oder das Rechtsklickmenü des Tabs) lässt nie zwei Tabs für dieselbe Datei stehen: Ein bereits auf dieser Datei geöffneter Tab wird geschlossen.
- **Erster Start und das Netz.** Der erste Start kontaktiert GitHub, um das Vorführprojekt zu holen (Kapitel 4); entfernen Sie das Projekt aus der Liste „GitHub-Projekte“ in **⚙ Einstellungen**, um das endgültig abzustellen.
- **Die Tastatur in G2 Basic lesen.** Eine Taste, die gedrückt wird, während ein Programm läuft, hält es an, wie beim Original, und `INPUT` wartet. Ein Spiel, das die Tasten beobachten muss, ruft `KEY(0)` in seiner Hauptschleife auf (Kapitel 15); danach hält eine Taste das Programm nicht mehr an.
- **Marken und Haltepunkte.** Ein Haltepunkt auf einer Marke, die es noch nicht gibt (getippt, bevor Sie assemblieren oder starten), wird behalten und gesetzt, sobald die Marke erscheint.
- **Anordnungen und Fenster.** Eine gespeicherte Anordnung merkt sich die angedockten Bereiche, die abgedockten Fenster (Ort und Größe) und die Größe des Hauptfensters. Sie merkt sich nicht, welches Fenster die Tastatur hat.

# Lizenz und Danksagungen {: .appendix }

**Retro Dev Kit Apple 1** ist freie Software, veröffentlicht unter der **GNU General Public License, Version 3** (GPL-3.0), ohne jede Gewährleistung. Der vollständige Text steht in der Datei `LICENSE` des Archivs und des Repositorys.

Einige Teile stammen aus anderen Projekten und behalten ihre eigene Lizenz, die **MIT**-Lizenz, die mit der GPL vereinbar ist; sie sind in der Datei `NOTICE` aufgeführt:

- **RDK G2 Basic**, das eingebaute BASIC, für dieses Projekt geschrieben (Repository `Old-Wirehead/rdk-gen2basic-apple1`, MIT);
- die **GEN2-Tests von Uncle Bernie** (`gen2-test1` bis `gen2-test4`, die Balken) und die **GEN2-Glyphen**, von Uncle Bernie geliefert.

Die Spiele der Bibliothek, die **Claude IA** geschrieben hat (Galgenmännchen, Streichhölzer, Schiffe versenken, Tunnel und Tic-Tac-Toe), werden mit dem Projekt verbreitet.

Was der Kit **nicht** enthält: Apples Integer BASIC, kommerzielle Kassetten und die Demos der P-LAB-Karte. Diese bleiben bei ihren Eigentümern; Sie bringen Ihre eigenen Kopien mit.

Die P-LAB-Karte ist eine Schöpfung von P-LAB; der TMS9918A ist ein Chip von Texas Instruments. Dieses Projekt ist weder mit Apple noch mit Texas Instruments oder P-LAB verbunden.
