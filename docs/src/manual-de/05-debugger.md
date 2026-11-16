# Der Debugger, der Disassembler und der Dump

<p class="lead">Der Debugger hält den Apple-1 an und führt ihn schrittweise aus, einen Befehl nach dem anderen in 6502 oder eine Zeile nach der anderen in BASIC, und zeigt dabei die Register, die Variablen und den Speicher.</p>

## Der Bereich Debugger

**🐛 Debugger** öffnet den Bereich, rechts angedockt oder wie der Editor abdockbar. Seine Befehle:

| Befehl | Wirkung |
|---|---|
| **⏸ Pause** / **▶ Weiter** | hält den Apple-1 an oder lässt ihn weiterlaufen |
| **⬇ Schritt** | führt einen Befehl aus |
| **↪ Überspringen** | führt ein `JSR` in einem Zug aus und hält bei der Rückkehr an |
| **⤴ Verlassen** | läuft bis zum `RTS` des aktuellen Unterprogramms |
| **⏭ BASIC-Zeile** | geht eine Zeile des BASIC-Programms weiter |
| **⏩ Bis zum Cursor** | (im Editor) läuft bis zur Zeile des Cursors |

{{!Schritt (Debugger)}}{{!Überspringen}}{{!Verlassen}}

Der Rahmen **Prozessor** zeigt A, X, Y, S, PC und die Flags N V B D I Z C; alle lassen sich durch Klick auf den Wert ändern. Darunter stehen die Zahl der ausgeführten Takte und Befehle.

## Haltepunkte

Ein Haltepunkt lässt sich auf vier Arten setzen:

- **⏺ Haltepunkt** im Editor (oder <kbd>F9</kbd>, oder das Rechtsklickmenü) auf der Zeile des Cursors: in einem `.s`-Quelltext (dank der Adresstabelle des Assemblers) oder in einem BASIC-Programm (Zeilennummer);
- ein Klick in den Rand des **📃 Listings**;
- ein Klick auf eine Zeile des Disassemblers;
- das Eintippen einer Adresse oder Marke: `$0368`, `ASK`, `PUTS+3`, oder `L190` für die BASIC-Zeile 190. Eine Marke, die es noch nicht gibt, wird nicht abgelehnt: Der Haltepunkt wird aufbewahrt und gesetzt, sobald die Marke existiert (wenn Sie den Quelltext des Editors assemblieren oder starten), mit einer Meldung, die es sagt.

Haltepunkte, beobachtete Variablen und Speicherbeobachtungen werden von einer Sitzung zur nächsten gespeichert, aber das Laden eines anderen Programms löscht die Haltepunkte und Marken des vorigen. Ist das eingebaute BASIC geladen, heißt der BASIC-Abschnitt des Debuggers **RDK G2 Basic**; ohne BASIC bei `$E000` sagt er das neutral. Die Farben der aktuellen Zeile, der Dump-Adressen und der ASCII-Spalte lassen sich unter **⚙ Einstellungen → Oberfläche** („Farben der Oberfläche“) ändern. In schmalen Bereichen fällt die Bytespalte des Disassemblers weg, und die Schreibzeile bricht um, statt abgeschnitten zu werden.

## Ein BASIC-Programm debuggen

![boucle.bas, angehalten an einem Haltepunkt in Zeile 30: die aktuelle Zeile grün im Editor, die Variablen I und S im Debugger beobachtet.](img/de/brut/debogueur-basic.png "large")

Die aktuelle Zeile ist im Editor grün hervorgehoben, Haltepunkte violett. Der BASIC-Rahmen (**RDK G2 Basic** oder **Integer BASIC**, je nach geladenem) zeigt die aktuelle Zeile, die Haltepunkte auf Zeilennummern und die **beobachteten Variablen**: ganze Zahlen (`A`, `B1`), Elemente von Feldern (`D(2)`), ganze Felder (`D`), Zeichenketten (`C$`), dazu **Alle Variablen**.

Das Debuggen funktioniert mit **RDK G2 Basic**, dem im Emulator eingebauten BASIC, ebenso wie mit dem Original: Das echte Integer BASIC wird nicht mehr gebraucht. Der Debugger bemerkt den Beginn jeder Zeile (die aktuelle Zeile, die Zeilenhaltepunkte, **⏭ BASIC-Zeile**) und liest die Variablen aus den Tabellen des Interpreters. Beim Original liest er die Zeile aus den Zero-Page-Zeigern `$DC`/`$DD`; G2 Basic legt sie anderswo ab, und der Debugger kennt beide Anordnungen.

!!! attention "Ein Tastendruck hält das Programm an"
    Wie an der echten Maschine hält eine Taste, die gedrückt wird, während das BASIC-Programm läuft, es an (`STOPPED AT`). Lassen Sie während einer Debugsitzung den Tastaturfokus im Bereich des Debuggers, nicht auf dem Apple-1-Bildschirm. Ein G2-Basic-Programm kann die Tastatur mit `KEY(0)` lesen, ohne angehalten zu werden (Kapitel 15).

## Ein 6502-Programm debuggen

![decimal.s, angehalten bei der Marke prdec: Register, Disassembler mit den Marken des Programms, Speicherbeobachtung $06.$08.](img/de/brut/debogueur-asm.png "large")

Der **Disassembler** zeigt 24 Zeilen um PC, mit den Marken des assemblierten Programms in einer eigenen Spalte. Er zentriert sich nur neu, wenn PC den sichtbaren Bereich verlässt, damit die Anzeige nicht bei jedem Schritt springt. **Gehe zu** heftet die Liste an eine Adresse oder Marke; **PC folgen** lässt sie wieder der Ausführung folgen.

Wählen Sie im 6502-Quelltext des Editors einen Befehl aus, werden seine Bytes auch im Disassembler und im Dump **hervorgehoben** (Kapitel 7), und beide Bereiche rollen dorthin, wenn sie nicht sichtbar sind; der Disassembler folgt PC dann nicht mehr, bis Sie **PC folgen** drücken.

Die **Speicherbeobachtung** zeigt ein Byte, ein Wort oder ASCII (`$30`, `$30 w`, `0200.020F`, eine Marke), bearbeitbar, gelb, wenn sich der Wert ändert.

## Der Dump

Der **Dump** zeigt den Speicher in Hex und ASCII, 16 Bytes pro Zeile (8, wenn der Bereich schmal ist), mit gelöschtem Bit 7 für das ASCII. **Gehe zu** springt zu einer Adresse oder zu den Abkürzungen ZP, `$0200`, `$0300`, PC, `$E000`. Geänderte Bytes sind gelb, das Byte bei PC hat einen grünen Hintergrund. **Schreiben** legt Bytes ab (`0300 = A9 00`); ein Klick auf ein Byte füllt die Schreibzeile aus.

Der Disassembler und der Dump sind eigenständige Bereiche: Standardmäßig abgedockt, lassen sie sich neben dem Debugger andocken (Kapitel 10).

## Auf der Befehlszeile

Für Vorführungen und automatische Tests: `--debogueur` öffnet den Bereich, `--dbg-arret ASK,L190` setzt Haltepunkte, `--dbg-surveiller 'A,C$,$30 w'` beobachtet Variablen und Speicher, und `--dbg-commande 5:pas` sendet nach 5 Sekunden einen Befehl (`pause`, `continuer`, `pas`, `dessus`, `sortir`, `basic`). Diese Optionsnamen und Schlüsselwörter sind französisch.
