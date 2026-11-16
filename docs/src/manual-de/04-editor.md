# Der Editor

<p class="lead">Der eingebaute Editor schreibt Integer-BASIC-Programme und 6502-Quelltexte, prüft sie bei jedem Tastendruck, startet sie mit einem Klick im emulierten Apple-1 und exportiert sie als Kassetten für die echte Maschine.</p>

## Tabs und Dateien

**📝 Editor** öffnet den Bereich, links vom Bildschirm angedockt. Seine Werkzeugleiste ist kompakt, damit möglichst viele Zeilen für den Text bleiben. Sie beginnt mit Symbolen, die ihre Hilfeblasen erklären:

| Symbol | Aktion |
|---|---|
| **Neu** (erstes Symbol) | Neuer Tab: „BASIC-Programm“ oder „6502-Assembler-Quelltext“ |
| **📂** / **💾** / **💾…** | Öffnen (`.bas`, `.txt`, `.s`, `.asm`, `.a65`, `.inc`) / Speichern (<kbd>Strg</kbd>+<kbd>S</kbd>) / Speichern unter… |
| **✂**, Kopieren, **📋** | Ausschneiden, kopieren, einfügen |
| **🔍** | Suchen und ersetzen (<kbd>Strg</kbd>+<kbd>F</kbd>, <kbd>Strg</kbd>+<kbd>H</kbd>) |
| **⟲ ⟳** | Rückgängig (<kbd>Strg</kbd>+<kbd>Z</kbd>), wiederherstellen (<kbd>Strg</kbd>+<kbd>Umschalt</kbd>+<kbd>Z</kbd>) |
| **A▲▼** | Textgröße: ein kompaktes Bedienelement, der Buchstabe A mit zwei kleinen übereinanderliegenden Dreiecken; ▲ vergrößert, ▼ verkleinert |

Danach folgen in derselben Zeile (oder in der nächsten, wenn der Bereich schmal ist) die Aktionen des Tabs: **▶ Starten**, **📼 WAV exportieren** und **Speicher** für BASIC, oder **📃 Listing**, **⏺ Haltepunkt** und **⏩ Bis zum Cursor** für einen 6502-Quelltext.

Jede Datei ist ein Tab. Ein Punkt • markiert einen geänderten Tab; × schließt ihn (ist er nicht gespeichert, ist ein zweiter Klick nötig). Ein mit **📚** markierter Tab wurde aus der Bibliothek geöffnet: Ein Listing oder ein Quelltext der Bibliothek (▶, ⬇ oder Doppelklick) öffnet seinen Tab, wenn der Editor sichtbar ist, oder zeigt ihn, wenn er schon offen ist. Die Tabs, auch ungespeicherter Text, kommen beim nächsten Start wieder. Ein Rechtsklick auf einen Tab bietet **Speichern unter…** und **Schließen**. Im Text fügt ein **Mittelklick** die primäre Auswahl ein (Linux): Der Cursor geht dorthin, wo Sie klicken, dann wird der zuletzt in irgendeiner Anwendung markierte Text eingefügt.

Die Textgröße stellen Sie mit **A▲▼**, <kbd>Strg</kbd>+Mausrad oder <kbd>Strg</kbd>+<kbd>+</kbd> / <kbd>Strg</kbd>+<kbd>−</kbd> / <kbd>Strg</kbd>+<kbd>0</kbd> ein, wenn der Text den Tastaturfokus hat.

**Abdocken** legt den Editor in ein eigenes Fenster, zum Beispiel auf einem zweiten Bildschirm; **⊟ Andocken** holt ihn zurück. In diesem Fenster getippte Tasten gehen an den Editor, nie an den Apple-1.

## Suchen, Ersetzen und das Rechtsklickmenü

**🔍** (oder <kbd>Strg</kbd>+<kbd>F</kbd>) öffnet über dem Text eine Suchleiste: <kbd>Eingabe</kbd> geht zum nächsten Treffer, <kbd>Umschalt</kbd>+<kbd>Eingabe</kbd> zum vorigen, **Aa** beachtet Groß- und Kleinschreibung, **Wort** findet ganze Wörter, und der Zähler zeigt „3 von 7“. <kbd>Strg</kbd>+<kbd>H</kbd> fügt das Feld **Ersetzen durch** hinzu, mit **Ersetzen** (der aktuelle Treffer) und **Alle ersetzen**. <kbd>Esc</kbd> schließt die Leiste.

![Die Suchleiste, geöffnet auf ECHO, und das Rechtsklickmenü eines 6502-Quelltexts.](img/de/editeur-menu.png "medium")

Ein **Rechtsklick** in den Text öffnet ein Menü mit allen Befehlen und ihren Tastenkürzeln. Das Menü wirkt auf die **Auswahl**, die beim Rechtsklick darauf erhalten bleibt: **Ausschneiden** und **Kopieren** arbeiten mit dem markierten Text. Die wortbezogenen Befehle (zur Definition gehen, Hilfe) verwenden das **Wort unter dem Klick**, nicht das unter dem Cursor.

| Befehl | Tastenkürzel |
|---|---|
| Ausschneiden, Kopieren, Einfügen, Alles auswählen | <kbd>Strg</kbd>+<kbd>X</kbd>, <kbd>C</kbd>, <kbd>V</kbd>, <kbd>A</kbd> |
| Suchen, Ersetzen | <kbd>Strg</kbd>+<kbd>F</kbd>, <kbd>Strg</kbd>+<kbd>H</kbd> |
| Gehe zu Zeile… | <kbd>Strg</kbd>+<kbd>G</kbd> |
| Auskommentieren / einkommentieren (6502), REM / REM entfernen (BASIC) | <kbd>Strg</kbd>+<kbd>/</kbd> |
| Einrücken, Ausrücken | |
| Gehe zur Definition einer Marke (6502) oder einer Zeilennummer (BASIC) | <kbd>F12</kbd> |
| Hilfe zu *Wort* (bei einem Schlüsselwort, Befehl oder einer Direktive: öffnet seinen Eintrag in der Referenz) | |
| Im Disassembler zeigen (6502) | |
| Neu nummerieren… (BASIC: `GOTO`, `GOSUB` und `THEN` folgen) | |
| Haltepunkt auf dieser Zeile | <kbd>F9</kbd> |
| Assemblieren und starten (6502), Senden und starten (BASIC) | <kbd>F5</kbd> |

**Farben.** Die Farben des Texts folgen dem Thema der Oberfläche. **⚙ Einstellungen → Editor** bietet für BASIC und für den 6502-Assembler getrennt die Schemata *Automatisch*, *Klassisch (dunkel)*, *Hell*, *Bernstein*, *Hoher Kontrast* oder Ihre eigenen Farben, ein Farbfeld je Wortart (Kapitel 11).

## Ein BASIC-Programm

![Der BASIC-Editor: Fehler auf rotem Grund, mit Zeilennummer und Spalte unten im Bereich.](img/de/editeur-erreur.png "large")

Der Text ist farbig: Zeilennummern, Schlüsselwörter, Zahlen, Zeichenketten, `REM`, Operatoren. Jede Zeile wird so **tokenisiert**, wie Integer BASIC es täte, von einem eigenen Tokenizer, der Byte für Byte mit dem echten verglichen wurde. Eine Zeile, die BASIC ablehnen würde, wird rot, und der Fehler nennt die Textzeile, die BASIC-Zeilennummer und die Spalte; ein Klick setzt den Cursor dorthin. Erkannte Fehler: Syntax, ungültiger Variablenname, Zahl größer als 32767, Direktbefehl (`LIST`, `RUN`…) im Programm, nicht abgeschlossene Zeichenkette, Zeile länger als 255 Bytes, Speicher voll.

- **Speicher**: automatisch (8 KB mit LOMEM `$0800`, oder `$0300`, wenn das Programm groß ist, sonst 12, 16 oder 32 KB) oder fest vorgegeben.
- {{▶ Starten}}: lädt Integer BASIC, wenn es fehlt (wie die Schaltfläche BASIC), legt das tokenisierte Programm in den Speicher, fügt bei Bedarf RAM hinzu und tippt dann `E2B3R` und `RUN`. Das geht sofort: Nichts wird Zeile für Zeile getippt.
- {{📼 WAV exportieren}}: eine Kassette (`.wav`, oder `.aiff` für ein Smartphone) aus zwei Aufnahmen, mit ihren Einstellungen in der Bibliothek. Unten im Bereich steht, was am echten Apple-1 zu tippen ist, zum Beispiel `C100R`, dann `004A.00FFR 0C3B.0FFFR`, dann `E2B3R` und `RUN`; **📋 Kopieren** legt es in die Zwischenablage.

![Vom Editor zum echten Apple-1: schreiben, starten, korrigieren, exportieren.](diagrams/de/workflow-basic.svg "diagram large")

## Ein 6502-Quelltext

![Ein 6502-Quelltext im Editor: die Farben, das Ergebnis der Assemblierung und die Zeile für den echten Apple-1.](img/de/brut/editeur-asm.png "large")

Ein Quelltext wird **bei jeder Änderung** vom eingebauten Zwei-Pass-Assembler assembliert, der die Syntax von ca65 annimmt (und die von xa, automatisch übersetzt). Unten im Bereich stehen die Größe und die Adressen (`33 Bytes, $0300-$0320 — 2 Datei(en), 13 Symbole`), dann die Zeile für die echte Maschine (`C100R`, dann `0300.0320R`, dann `300R`).

- **Fehler**: Datei, Zeile und Spalte; ein Klick führt dorthin und öffnet bei Bedarf die eingebundene Datei. Warnungen (Zero Page als absolut codiert) stehen in Gelb.
- **📃 Listing**: Adresse, Bytes und Quelltext jeder Zeile, mit eingebundenen Dateien und aufgelösten Makros, dann die Symboltabelle; ein Klick führt zur Zeile zurück.
- **▶ Starten**: legt die Bytes beim Ursprung in den Speicher (`.org`, standardmäßig `$0300`), fügt bei Bedarf RAM hinzu und tippt dann zum Beispiel `300R`.
- **📼 WAV exportieren**: die Kassette, wie bei BASIC.
- **⏺ Haltepunkt** und **⏩ Bis zum Cursor**: die Verbindung zum Debugger (Kapitel 8).
- **Verbindung zum Debugger**: Wählen Sie im Quelltext einen Befehl aus (oder setzen Sie den Cursor auf seine Zeile), werden die erzeugten Bytes im **Disassembler** des Debuggers und im **Hex/ASCII-Dump** hervorgehoben, wenn diese Bereiche sichtbar sind; liegen sie anderswo im Speicher, rollen die Bereiche dorthin. Die Tabelle der Quelltextzeilen und Adressen des Assemblers macht das möglich.

![Ein Assemblerfehler: ECO statt ECHO, ein unbekanntes Symbol, gemeldet mit Zeile und Spalte.](img/de/brut/editeur-asm-erreur.png "large")

Teil III ist ganz dem Assembler gewidmet: Syntax, Direktiven, Listing, Beispiele.
