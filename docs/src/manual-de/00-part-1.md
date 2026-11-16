---
version: 1.0
date: November 2026
---

# Der Emulator <span class="part-sub">Das Fenster, der Bildschirm, die Bibliothek, die Kassettenschnittstelle ACI, die Farbkarte GEN2, der Debugger und die Befehlszeile</span> {: .part data-part="Teil I" }

# Über dieses Handbuch

<p class="lead"><strong>Retro Dev Kit Apple 1</strong> — „Die Erbsünde“ — bildet den Apple-1 von 1976 taktgenau nach: den 6502, das Videoterminal, die Tastatur, die Kassettenschnittstelle ACI und, als Optionen, die Farbkarte GEN2 von Uncle Bernie und die Grafikkarte P-LAB mit TMS9918A. Es bringt sein eigenes freies BASIC mit, <strong>RDK G2 Basic</strong>. Rund um die Maschine bietet es eine vollständige Werkstatt: eine Programmbibliothek, einen Editor, einen Assembler, einen Debugger und den Export auf einen echten Apple-1.</p>

Dieses Handbuch beschreibt den ganzen Emulator, Version 1.0 (die erste öffentliche Version; die Nummern 7.x waren die Testversionen). Es ergänzt die *Schnellstartanleitung*, die zum Installieren des Programms und für eine erste Kassette genügt. Hier werden jedes Fenster, jede Schaltfläche und jeder Befehl erklärt, mit Beispielen, die Sie so eintippen können, wie sie dastehen.

## Wie dieses Handbuch aufgebaut ist

<div class="cards">
<div class="card"><h4>Teil I — Der Emulator</h4>Das Fenster und seine Bereiche, der Vintage-Bildschirm, die Bibliothek, die Kassette und die ACI, die Grafikkarten GEN2 und P-LAB, der Editor, der Debugger, ASCII-Art, die Einstellungen, die Befehlszeile und die Fehlersuche.</div>
<div class="card"><h4>Teil II — Integer BASIC</h4>Steve Wozniaks BASIC und sein freier Klon RDK G2 Basic: jeder Befehl, jede Funktion und jeder Operator, der Speicher, die Fehlermeldungen und die bekannten Fehler, mit ihren Umgehungen.</div>
<div class="card"><h4>Teil III — Der 6502-Assembler</h4>Der 6502 von Grund auf, der eingebaute Assembler und seine Direktiven, die Routinen des Woz Monitors, vollständige Beispiele und die Fallen, die man meiden sollte.</div>
<div class="card"><h4>Anhänge</h4>Speicherbelegung, Befehle des Woz Monitors, das ACI-Format, der Zeichensatz des 2513, Tastenkürzel und der Befehlsindex.</div>
</div>

## Konventionen

Tasten werden so geschrieben: <kbd>Strg</kbd>+<kbd>R</kbd>, <kbd>RETURN</kbd>. Schaltflächen der Oberfläche stehen fett: **▶ Starten**. Adressen sind hexadezimal und, wie in den Unterlagen der Zeit, mit `$` versehen: `$FFEF`. Der Woz Monitor selbst schreibt sie ohne `$`: `FFEF`.

Sitzungen auf dem Apple-1-Bildschirm haben einen dunklen Hintergrund. Was **Sie** tippen, ist weiß, die Antwort der Maschine grün:

```mon
» 300R

0300: A2
HALLO, APPLE-1!
```

Unterwegs begegnen Ihnen vier Arten von Kästen:

!!! astuce "Tipp"
    Eine Abkürzung oder ein besserer Weg, etwas zu tun.

!!! attention "Achtung"
    Eine Falle, die Zeit oder Daten kostet.

!!! apple1 "An der echten Maschine"
    Was auf einem echten Apple-1 geschieht und was sich vom Emulator unterscheidet.

!!! bug "Bekannter Fehler"
    Ein Fehler des Original-ROMs, mit seiner Umgehung. Jeder Fehler hat einen Status: <span class="verified">geprüft</span> (im Emulator mit dem echten Integer BASIC nachvollzogen) oder <span class="unverified">nicht geprüft</span> (aus einer Quelle zitiert, nicht ausprobiert).

Jedes Beispiel dieses Handbuchs wurde im Emulator ausprobiert. Die 6502-Programme aus Teil III liegen im Ordner `docs/exemples/de/asm/` der Distribution. Sie ergeben mit dem eingebauten Assembler und mit ca65 genau dieselben Bytes.

## Was Sie brauchen

Der Emulator läuft unter Linux, ohne dass etwas zu installieren wäre. Der Woz Monitor und das ACI-ROM sind eingebaut, ebenso **RDK G2 Basic**, ein freies, zum Integer BASIC kompatibles BASIC (Kapitel 14): Die Schaltfläche **BASIC** funktioniert sofort, auch bei einer frischen Installation, ohne dass eine Datei gesucht werden muss. Apples **Integer BASIC** von 1976 wird **nicht** mitgeliefert, weil es noch urheberrechtlich geschützt ist. Wenn Sie das Original möchten, verwenden Sie Ihre eigene Kopie (zum Beispiel die Kassette `BASIC.wav` aus der Sammlung „Apple1Cassettes“ auf archive.org) und wählen Sie sie unter **⚙ Einstellungen → Erweitert → BASIC**.

!!! attention "ROMs und Kassetten bleiben bei Ihnen"
    Der Emulator liefert weder Apples Integer BASIC noch kommerzielle Kassetten. Seine Erkennungstabelle enthält nur Größen, Prüfsummen und Parameter. Bewahren Sie Ihre Kopien in Ihrer persönlichen Bibliothek auf.

!!! note "Lizenz"
    Retro Dev Kit Apple 1 ist freie Software unter der **GNU GPL 3.0** (ohne Gewährleistung). Einige Teile behalten ihre eigene MIT-Lizenz: RDK G2 Basic sowie die GEN2-Tests und -Glyphen von Uncle Bernie. Siehe `LICENSE` und `NOTICE` im Archiv sowie den Anhang „Lizenz und Danksagungen“.

## Die wichtigsten Wege

Vier Arten, den Emulator zu nutzen, kehren immer wieder. Jede wird später mit einem eigenen Schaubild beschrieben:

| Ich möchte… | Der Weg | Siehe |
|---|---|---|
| ein Programm von damals spielen | Bibliothek → **▶** | Kapitel 4 |
| ein BASIC-Programm schreiben | **📝 Editor** → **▶ Starten** → **📼 WAV exportieren** | Teil II |
| ein 6502-Programm schreiben | **📝 Editor** (`.s`) → **📃 Listing** → **🐛 Debugger** | Teil III |
| ein Programm an einen echten Apple-1 senden | **📼 WAV exportieren** (`.aiff`) → Smartphone → ACI | Kapitel 5 |

# Das Fenster und die Werkzeugleiste

## Überblick

![Die Standardanordnung: alle Bereiche offen und angedockt.](img/de/defaut-annote.png "large")

<ol class="legend">
<li><strong>Laden, Sichern, Zustand</strong>: Hex-Dumps <code>.txt</code>, Binärdateien <code>.bin</code> und <code>.prg</code>; Sichern schreibt auch Kassetten <code>.wav</code>/<code>.aiff</code>; Zustand speichert und lädt den Zustand der Maschine.</li>
<li><strong>RESET, Bildschirm löschen, Geschwindigkeit (Anzeige), Einfügen</strong>: die Tasten der Maschine (Strg+R, Strg+L, F3, Strg+V).</li>
<li><strong>BASIC</strong>: lädt das eingebaute RDK G2 Basic nach <code>$E000</code> (oder Ihr Integer BASIC, wenn Sie es gewählt haben).</li>
<li><strong>Fenster</strong>: das Menü <strong>⊞ Anordnung ▾</strong>, dann eine Schaltfläche je Bereich (Monochrom-Bildschirm, Farbbildschirm, Editor, Debugger, Disassembler, Dump, Bibliothek, ASCII).</li>
<li><strong>Grafikkarten</strong>: <strong>GEN2</strong> und <strong>P-LAB</strong> stecken die Karten ein oder aus.</li>
<li><strong>Kassette</strong>: eine Kassette <strong>📼 Einlegen</strong>, dann Wiedergabe, Stopp, Zurückspulen, Aufnahme, Auswerfen; Oszilloskop und Ton.</li>
<li><strong>EPROM</strong>, <strong>⚙ Einstellungen</strong>, <strong>?</strong> (ausführliche Hilfeblasen, auch F1) und <strong>ℹ</strong> (Über, Tastenkürzel).</li>
<li><strong>Anzeigezeile des Bildschirms</strong>: Zoom (1x bis 8x oder Anpassen), Phosphor, Anzeigegeschwindigkeit <strong>Normal</strong> oder <strong>Schnell</strong>, Vintage-Bildschirm, Vollbild, <strong>Abdocken</strong>.</li>
<li><strong>Editor</strong>: BASIC-Programme und 6502-Quelltexte in Tabs.</li>
<li><strong>Apple-1-Bildschirm</strong>: 40 × 24 Zeichen; anklicken, um zu tippen, Rechtsklick für Kopieren, Einfügen, Bildschirm löschen und Reset.</li>
<li><strong>Debugger</strong>: Pause, Einzelschritt, Register, Haltepunkte, Variablen.</li>
<li><strong>Disassembler</strong>: der Code um den Programmzähler (PC).</li>
<li><strong>Dump</strong>: der Speicher in Hex und ASCII, bearbeitbar.</li>
<li><strong>Bibliothek</strong>: Ihre Programme, jedes mit seinen kleinen Symbolen ▶ ⬇ 📼 ⚙.</li>
<li><strong>Statusleiste</strong>: Geschwindigkeit, Anzeigerate, Kassette, Ziel der Tastatur; darunter die letzte Meldung.</li>
</ol>

Jeder Bereich (Bildschirm, Editor, Debugger, Bibliothek…) ist ein Tab, den Sie verschieben, gruppieren oder in ein eigenes Fenster abdocken können: Darum geht es in Kapitel 10. Beim ersten Start öffnet sich das Fenster maximiert, und ein Fenster **Ihr Arbeitsbereich** schlägt fünf Anordnungen vor (Kapitel 10); Größe und Position des Fensters und die Anordnung seiner Bereiche werden gespeichert und beim nächsten Start so wiederhergestellt, wie sie waren.

## Die Werkzeugleiste, Schaltfläche für Schaltfläche

Die Werkzeugleiste ist eine einzige Zeile: links die Maschine und ihre Werkzeuge, in der Mitte das Kassettendeck, die Bereiche in der Gruppe Fenster, die Karten und rechts die Einstellungen. Ist das Fenster zu schmal, behalten die Schaltflächen nur ihr Symbol (der Text steht in der Hilfeblase), oder die Leiste bricht auf mehrere Zeilen um, wenn Sie in den Einstellungen **Kompakte Leiste** abwählen.

![Die Werkzeugleiste bis zur Gruppe Grafikkarten: Dateien, Maschine, Fenster und Karten. Danach folgt der Streifen Kassette (Kapitel 5), dann die Einstellungen.](img/de/barre-1.png "large")

| Schaltfläche | Was sie tut |
|---|---|
| {{Laden}} | Öffnet ein Programm und legt es in den Speicher: eine `.txt` (Dump des Woz Monitors) oder eine `.hex`; bei einer `.bin` wird nach der Adresse gefragt. Kassetten (`.wav`, `.aiff`) gehen nicht hier durch: Sie werden mit **📼 Einlegen** im Streifen Kassette eingelegt. |
| {{Sichern}} | Fragt nach einem oder mehreren Bereichen (`0300.03FF`, `004A.00FF 0300.0FFF`…), dann nach einer Datei. Die Endung wählt das Format: `.wav`, `.aiff`, `.txt` oder `.bin`. |
| **RESET** (rote Schaltfläche) | Die RESET-Taste des Apple-1 (<kbd>Strg</kbd>+<kbd>R</kbd>). Der 6502 und der PIA starten neu; Speicher und Bildschirm bleiben erhalten. |
| **🆑 Bildschirm löschen** | Die CLEAR-SCREEN-Taste (<kbd>Strg</kbd>+<kbd>L</kbd>). |
| **Anzeige (Echtzeit / Schnelle Geschwindigkeit)** | Schaltet zwischen den originalen 1 MHz und der Höchstgeschwindigkeit um (<kbd>F3</kbd>). Ein Tachometer: blau beleuchtet, solange die schnelle Geschwindigkeit läuft. |
| **📋 Einfügen** | Tippt die Zwischenablage auf der Tastatur des Apple-1, im Tempo der Tastatur (<kbd>Strg</kbd>+<kbd>V</kbd>). Ideal für einen Hex-Dump. |
| {{BASIC|**BASIC**}} | Lädt das eingebaute **RDK G2 Basic** nach `$E000` (7933 Bytes, dann startet `E000R` es). Haben Sie unter **⚙ Einstellungen → Erweitert → BASIC** das originale Integer BASIC gewählt, lädt es stattdessen dieses: eine Kassette, einen `.txt`-Dump oder eine `.bin` von 4096 Bytes, gesucht in der Bibliothek (`BASIC.wav`, `integer_basic.bin`, jeder Name, der „basic“ enthält), sonst wird danach gefragt. Rechtsklick: eine andere Datei wählen. |
| **Fenster** (Gruppe: **⊞ Anordnung ▾** und eine Schaltfläche je Bereich) | Zeigt oder verbirgt jeden Bereich: **Monochrom-Bildschirm** (das Terminal des Apple-1), **Farbbildschirm** (der Bildschirm der eingesteckten Grafikkarte, GEN2 oder P-LAB: dieselbe Schaltfläche dient beiden; ist keine Karte eingesteckt, bietet sie an, eine einzustecken, und **P-LAB** zeigt dann ihren Bildschirm), **📝 Editor** (Kapitel 7), **🐛 Debugger**, **Disassembler**, **Dump** (Kapitel 8), **📚 Bibliothek** und **🎨 ASCII** (Kapitel 9). Eine leuchtende Schaltfläche bedeutet, dass der Bereich sichtbar ist; ein angezeigter Bereich lässt sich auch mit dem Kreuz seines Tabs schließen. |
| **Grafikkarten: GEN2 und P-LAB** | Eine benannte Gruppe. **GEN2** steckt die Farbkarte von Uncle Bernie ein oder aus (Kapitel 6); **P-LAB** tut dasselbe für die P-LAB-Karte mit TMS9918A (Abschnitt 6.6). Eine Schaltfläche leuchtet, solange ihre Karte eingesteckt ist; der Bildschirm einer Karte erscheint in einem eigenen Bereich, den die Schaltfläche **Farbbildschirm** zeigt. |
| **📸 Zustand ▾** | Eine einzige Schaltfläche mit Menü: **Zustand speichern…** (<kbd>Strg</kbd>+<kbd>Umschalt</kbd>+<kbd>S</kbd>), **Zustand laden…** (<kbd>Strg</kbd>+<kbd>Umschalt</kbd>+<kbd>O</kbd>), dann die letzten Zustände, jeder mit Titel und Datum (Abschnitt 2.5). |
| **EPROM** (Chip mit UV-Fenster, rechts in der Leiste, vor **⊞ Anordnung**) | Öffnet das EPROM-Programmiergerät: einen Chip mit einem `minipro`-Programmer lesen, schreiben oder prüfen (Abschnitt 2.6). |
| **Kassette: 📼 Einlegen ▶ ⏹ ⏪ ⏺ ⏏** | Das Deck, in einem Streifen zusammengefasst. **📼 Einlegen** wählt eine `.wav` oder `.aiff` und legt sie als virtuelle Kassette ins Deck; dann Wiedergabe, Stopp, Zurückspulen, Aufnahme, Auswerfen. Der Streifen zeigt Name, Position und Länge. |
| **📈** und **🔊** | Das Kassetten-Oszilloskop und der Ton des Signals (Kapitel 5). |
| **⊞ Anordnung ▾** (erste Schaltfläche der Gruppe Fenster) | Ein Menü: **Standard** (alle Bereiche, wie beim ersten Start), **Spiel** (Apple-1-Bildschirm und Bibliothek), **BASIC** (Editor und Bildschirm), **Assembler** (Editor, Bildschirm und Debugger), **Debugging** (Editor, Debugger, Disassembler und Dump); dann **Aktuelle Anordnung speichern…**, das nach einem Namen fragt, und Ihre gespeicherten Anordnungen in alphabetischer Reihenfolge (Rechtsklick: Umbenennen, Löschen). Ein Häkchen markiert die letzte Wahl; alles wird in `gui.json` gespeichert. |
| **⚙** | Die Einstellungen (Kapitel 11). |
| **?** | Hilfemodus (<kbd>F1</kbd>): ausführliche Blasen zu jedem Bedienelement, in den 7 Sprachen der Oberfläche. |
| **ℹ** | „Über“, mit der Liste der Tastenkürzel. |

**Dateifenster.** Öffnen, Speichern unter und Exportieren verwenden das eigene Dateifenster des Emulators. Eine blaue Schaltfläche **📁+ Neuer Ordner** oben links legt im angezeigten Ordner einen Ordner an, mit einer Hilfeblase zur Erklärung; tippen Sie den Namen, drücken Sie <kbd>Eingabe</kbd>, und der neue Ordner ist bereit für Ihre Datei.

![Das Fenster Laden, mit der Schaltfläche 📁+ Neuer Ordner oben links.](img/de/fenetre-fichiers.png "medium")

Die Anzeigeeinstellungen stehen nicht in der Werkzeugleiste: Jeder Bildschirm hat seine eigene **Anzeigezeile** oben in seinem Bereich (Abschnitt 3.2).

![Die Anzeigezeile des Apple-1-Bildschirms: Zoom, Phosphor, Anzeigegeschwindigkeit, Vintage-Bildschirm und Vollbild; Abdocken steht am rechten Ende der Zeile.](img/de/barre-2.png "large")

!!! astuce "Hilfemodus"
    Wenn Sie einen Bereich erkunden, schalten Sie **?** ein: Jede Schaltfläche erklärt, was sie tut, sobald Sie mit der Maus darüberfahren. Ausgeschaltet bleiben nur die üblichen kurzen Hilfeblasen (sie lassen sich unter **⚙ → Oberfläche** abschalten). Die Einstellung wird gespeichert.

## Die Statusleiste

![Die Statusleiste, während BASIC.wav abgespielt wird: Geschwindigkeit, Anzeigegrenze, Kassette, Tastatur und darunter die letzte Meldung.](img/de/barre-etat.png "large")

- **Gemessene Geschwindigkeit**: etwa 0,960 MHz in Echtzeit. Das ist der effektive Takt des Apple-1, der den 6502 vier von fünfundsechzig Takten anhält, um seinen dynamischen Speicher aufzufrischen.
- **Anzeige ~60 Zeichen/s**: Das Terminal des Apple-1 zeigt nur ein Zeichen pro Bild an, etwa 60 pro Sekunde. Ein Klick wechselt zwischen dieser Grenze und der sofortigen Anzeige, wie die Schaltflächen **Normal** / **Schnell** der Anzeigezeilen.
- **Kassette**: Name, Position und Länge, oder „keine Kassette“.
- **Tastatur**: wohin die Tasten gehen (zum Apple-1 oder in ein Textfeld der Oberfläche).

## Die Tastatur des Apple-1

Klicken Sie auf den Bildschirm: Die Tasten gehen zum Apple-1. Die Originaltastatur kennt nur Großbuchstaben, und der Emulator wandelt Kleinbuchstaben um.

| PC-Taste | Für den Apple-1 |
|---|---|
| Buchstaben, Ziffern, Zeichen | das Zeichen, als Großbuchstabe |
| <kbd>Eingabe</kbd> | RETURN (Code `$8D`) |
| <kbd>Rücktaste</kbd> | `_`: Der Woz Monitor und das BASIC löschen das vorige Zeichen (es bleibt auf dem Bildschirm stehen, gefolgt von `_`) |
| <kbd>Esc</kbd> | ESC: verwirft die aktuelle Zeile im Woz Monitor |
| <kbd>Strg</kbd>+Buchstabe | das Steuerzeichen (`Strg+D` = `$84`…) |
| <kbd>Strg</kbd>+<kbd>R</kbd> / <kbd>Strg</kbd>+<kbd>L</kbd> | die Tasten RESET und CLEAR SCREEN |

Sie können auf beiden Bildschirmen **Text mit der Maus markieren**: Taste drücken und über die Zeichen ziehen, die Auswahl wird hervorgehoben. Ein **Rechtsklick** auf einen der Bildschirme (den des Apple-1 oder den GEN2-Monitor) öffnet ein kleines Menü: **Kopieren** legt den markierten Text in die Zwischenablage (oder den ganzen Bildschirmtext, wenn nichts markiert ist), **Einfügen (Text tippen)** tippt die Zwischenablage auf der Tastatur des Apple-1, als würden Sie selbst tippen, dann **Bildschirm löschen** und **Reset**.

![Das Rechtsklickmenü des Apple-1-Bildschirms während einer Partie Hamurabi.](img/de/ecran-menu.png "medium")

!!! apple1 "Der Bildschirm des Apple-1 löscht nichts"
    Das Terminal kann nur ein Zeichen schreiben und in die nächste Zeile springen. Es gibt keinen beweglichen Cursor und keine Möglichkeit, ein Zeichen zu löschen: Die Löschtaste gibt `_` aus, und das Zeichen bleibt auf dem Bildschirm. Das ist normal.

## Zustand der Maschine

Die Schaltfläche **📸 Zustand** der Werkzeugleiste macht eine vollständige Momentaufnahme der Maschine und stellt sie später genau an derselben Stelle wieder her: Sie können ein laufendes Spiel oder Programm anhalten, den Emulator schließen und am nächsten Tag weitermachen. Die Schaltfläche öffnet ein Menü:

![Das Menü Zustand: speichern, laden, dann die letzten Zustände mit Datum.](img/de/menu-etat.png "medium")

- **Zustand speichern…** (<kbd>Strg</kbd>+<kbd>Umschalt</kbd>+<kbd>S</kbd>) fragt nach einem Dateinamen und schreibt eine Datei `.a1state`. Vorgeschlagen wird der Ordner `etats` neben dem Ordner der Bibliothek.
- **Zustand laden…** (<kbd>Strg</kbd>+<kbd>Umschalt</kbd>+<kbd>O</kbd>) ersetzt die Maschine durch den gewählten Zustand. Die Statusleiste zeigt die Datei und den Wert von PC.
- **Letzte Zustände** listet die letzten acht auf, mit Titel und Datum der Speicherung; ein Klick lädt einen davon sofort. Wurde die Datei verschoben oder gelöscht, sagt eine Meldung es.

Ein Zustand enthält den Speicher, die Register des 6502, den PIA, den Bildschirm, die GEN2-Karte, wenn sie eingesteckt ist, und die virtuelle Kassette mit ihrer Position. Er enthält weder Ihre Einstellungen noch die Bibliothek. Eine Datei, die sich nicht lesen lässt (beschädigt oder kein Zustand), wird mit einer Meldung abgelehnt, und die Maschine bleibt unverändert.

!!! astuce "Typische Verwendung"
    Speichern Sie einen Zustand kurz vor einer schwierigen Stelle eines Spiels oder am Ende einer langen BASIC-Sitzung: Ein Zustand ist auch ein bequemer Weg, ein laufendes Programm mit einem Freund zu teilen, der dieselbe Version verwendet.

## Das EPROM-Programmiergerät

**EPROM** in der Werkzeugleiste öffnet ein Fenster, um einen EPROM- oder EEPROM-Chip aus dem Emulator heraus zu lesen, zu schreiben und zu prüfen, zum Beispiel um das BASIC-ROM, ein Programm für die GEN2-Erweiterung oder den Inhalt Ihres eigenen Speichers zu brennen.

![Das EPROM-Programmiergerät: Programmer, Chip, Datenquelle und die drei Vorgänge.](img/de/programmateur-eprom.png "medium")

Unterstützt werden nur die Programmer, die das freie Werkzeug **`minipro`** steuert: XGecu **TL866II+**, **T48** und **T56**. Unter Debian, Ubuntu und Linux Mint installieren Sie es mit `sudo apt install minipro`, stecken den Programmer an einen USB-Anschluss und drücken dann **Erkennen**. Fehlt `minipro`, sagt das Fenster es und erklärt, was zu tun ist; auf der Seite des Emulators ist nichts weiter nötig.

| Einstellung | Rolle |
|---|---|
| **Programmer** | der erkannte Programmer, mit seiner Firmware (zum Beispiel „Found TL866II+ 04.2.132 (USB)“) |
| **Chip** | die üblichen Chips der Apple-1-Welt: 2716, 2732, 2764, 27C64, 27C128, 27C256, 27C512, 28C64, 28C256, SST27SF256, SST27SF512 und das Masken-ROM 23128 (nur lesen) |
| **Daten** | der **Apple-1-Speicher** (ein Adressbereich, standardmäßig `$E000`–`$EFFF`) oder eine **Datei** (`.bin`, `.hex`, `.aiff`…) |

Drei Schaltflächen wirken auf den Chip, ein Fortschrittsbalken und ein Protokoll zeigen, was geschieht:

- **Chip lesen** liest den ganzen Chip. Sind der Speicher oder eine vorhandene Datei die Daten, wird der Inhalt **verglichen**, und die Unterschiede werden aufgelistet: Eine vorhandene Datei wird nie überschrieben. Bei einem neuen Dateinamen wird der Inhalt darin gespeichert.
- **Chip schreiben** programmiert den Chip mit den gewählten Daten. Der Chip muss leer oder löschbar sein; am Ende wird das Ergebnis bestätigt.
- **Prüfen** vergleicht den Chip Byte für Byte mit den gewählten Daten und meldet jeden Unterschied (`$E010: Chip $4C, Daten $A9`).

**Schließen** schließt das Fenster.

Das Fenster verändert nie den emulierten Speicher: Schreiben und Prüfen lesen ihn, Lesen vergleicht ihn nur oder füllt eine Datei.

# Der Bildschirm: Zoom, Phosphor und Vintage-Effekt

## Der Originalbildschirm

Der Apple-1 zeigt 40 Spalten × 24 Zeilen mit Großbuchstaben. Jedes Zeichen ist eine Punktmatrix von 5 × 7 in einer Zelle von 7 × 8, gelesen aus dem ROM des Zeichengenerators **Signetics 2513**: Der Emulator verwendet den Inhalt dieses ROMs Punkt für Punkt (Anhang D). Das Bild hat daher 280 × 192 Punkte.

Der Cursor ist ein blinkendes `@`, gesteuert von einem NE555: 2,18 Hz, etwa zwei Drittel der Zeit sichtbar. Auf manchen Platinen ergibt die Toleranz des Kondensators stattdessen 1,7 Hz; die Option `--cursor-hz 1.7` gibt das wieder.

## Die Anzeigezeile

Jeder Bildschirm hat oben in seinem Bereich eine eigene Zeile mit Einstellungen: der Apple-1-Bildschirm und der Farbmonitor, wenn die GEN2-Karte eingesteckt ist (Kapitel 6).

| Einstellung | Rolle |
|---|---|
| **Zoom** | eine Liste: `1x` bis `8x` vergrößern das Bild um einen ganzzahligen Faktor, so bleiben die Punkte quadratisch und scharf; **Anpassen** wählt den größten Faktor, der in den Bereich passt (der gewählte Faktor steht in Klammern). Mit der Tastatur: <kbd>Strg</kbd>+<kbd>+</kbd>, <kbd>Strg</kbd>+<kbd>−</kbd>, <kbd>Strg</kbd>+<kbd>0</kbd> (anpassen). |
| **Phosphor** | nur Apple-1-Bildschirm: das Grün der Monitore der Zeit, das Weiß eines Fernsehers oder Bernstein. |
| **Anzeige: Normal / Schnell** | **Normal** zeigt die Zeichen im Tempo des echten Terminals, etwa 60 pro Sekunde; **Schnell** zeigt alles auf einmal. Diese Einstellung ist gemeinsam: Ändern Sie sie an einem Bildschirm, ändert sie sich am anderen. |
| **Vintage-Bildschirm** | der Röhreneffekt der 1970er Jahre (nächster Abschnitt); jeder Bildschirm hat sein eigenes Kästchen. |
| **⛶** | Vollbild (<kbd>F11</kbd>). |
| **Abdocken** | legt den Bildschirm in ein eigenes Fenster (Kapitel 10). |

Jeder Bildschirm steht **zentriert** in seinem Bereich, waagerecht und senkrecht, mit seiner Bildunterschrift zentriert unter dem Bild. Ist der gewählte Zoom größer als der Bereich, wird der Bildschirm mit dem größten passenden Zoom gezeigt, statt abgeschnitten zu werden. Ist das Fenster schmaler als die Anzeigezeile, bricht diese auf eine zweite Zeile um, und **Abdocken** bleibt am rechten Ende der letzten Zeile.

![Bernsteinfarbener Phosphor, mit Vintage-Bildschirm und leichter Wölbung.](img/de/brut/ambre.png "medium")

## Der Vintage-Bildschirm

Das Kästchen **Vintage-Bildschirm** der Anzeigezeile ahmt einen Röhrenmonitor der 1970er Jahre nach. Der Haupteffekt ist die feine dunkle Linie zwischen zwei Rasterzeilen: Jede Punktreihe wird zu einem waagerechten Band, wie auf dem Foto eines echten Apple-1. Diese Linie behält bei jedem Zoom dieselbe Dicke in Pixeln Ihres Bildschirms: Sie bleibt fein und scharf, selbst im Vollbild.

![Der Vintage-Bildschirm mit leichter Röhrenwölbung (0,15) und dunklen Ecken während einer Partie Hamurabi.](img/de/brut/vintage.png "medium")

Unter **⚙ Einstellungen → Erweitert → Vintage-Bildschirm: Feineinstellungen** stellen Sie die **Stärke** ein und wählen die Effekte; diese Einstellungen gelten für beide Bildschirme. Jeder Schieberegler zeigt daneben seinen **Zahlenwert** (Wölbung, Stärke…):

| Effekt | Standard | Rolle |
|---|---|---|
| Lücken zwischen den Rasterzeilen | an | die dunkle Linie zwischen zwei Punktreihen |
| waagerecht weichgezeichnete Punkte | an | eine leichte, nur waagerechte Unschärfe, wie beim Elektronenstrahl |
| Nachleuchten des Phosphors | kurz | die Punkte verblassen in weniger als einer Zehntelsekunde (in Bernstein etwas langsamer); ganz links: keines |
| Leuchten | an, dezent | ein Lichthof um die Punkte, die scharf bleiben |
| gewölbte Röhre und dunkle Ecken | aus | mit einem Schieberegler **Wölbung** |
| Flimmern | aus | eine sehr leichte Helligkeitsschwankung |
| RGB-Phosphormaske | an | nur Farbbildschirm der GEN2-Karte: feine senkrechte rote, grüne und blaue Streifen, getrennt durch dünne dunkle Linien, wie auf der Röhre eines Farbfernsehers. Sie ist ab Zoom 3 sichtbar, und ihre Stärke folgt der **Stärke** |

Der Effekt lässt sich auch auf den Farbmonitor der GEN2-Karte anwenden, mit dem Kästchen seiner eigenen Anzeigezeile. Er wird vom Prozessor berechnet, in der Auflösung des Apple-1 und nur, wenn sich das Bild ändert: Er bleibt auch ohne Grafikbeschleunigung flüssig.

Auf der Befehlszeile: `--screening`, `--sans-screening`, `--gen2-screening` (GEN2-Monitor) oder zum Beispiel `--screening-options intensite=0.7,remanence=0.5,courbure=0.4,sans-halo` (Werte von 0 bis 1; die Optionsnamen sind französisch).

!!! astuce "Abgedockter Bildschirm und zweiter Monitor"
    **Abdocken** am Ende der Anzeigezeile legt den Apple-1-Bildschirm in ein eigenes Fenster. Stellen Sie es auf einen zweiten Monitor, im Vollbild, mit dem Vintage-Effekt: Die Tastatur geht zum Apple-1, sobald dieses Fenster den Fokus hat. Mit **Anpassen** folgt das Bild der Fenstergröße, in ganzzahligen Schritten (**Ganzzahlige Skalierung**, in der Titelleiste des Fensters, das schärfste Bild) oder füllend.
