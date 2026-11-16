# Der erste Start

Beim ersten Start öffnet sich das Fenster maximiert, und ein Fenster **Ihr Arbeitsbereich** bietet fünf Anordnungen an: **Standard**, **Spiel**, **BASIC**, **Assembler** und **Debugging**. Jedes nummerierte Vorschaubild zeigt die Anordnung, sagt, wofür sie gedacht ist, und nennt die nötige Mindestgröße des Bildschirms: Klicken Sie auf die gewünschte. Entfernen Sie das Häkchen bei **Dieses Fenster beim Start anzeigen**, wenn Sie es nicht wieder sehen möchten; **⊞ Anordnung ▾ → Arbeitsbereich wählen…** öffnet es erneut. Die Anordnung lässt sich jederzeit über das Menü **⊞ Anordnung ▾** ändern. Die Standardanordnung zeigt alle Werkzeuge auf einen Blick. Jeder Bereich ist ein andockbares Fenster, das Sie verschieben, in Tabs gruppieren oder in ein eigenes Fenster abdocken können. Der Farbbildschirm (GEN2 / P-LAB) ist das Rechteck Nummer 7, in den Vorschaubildern, die ihn zeigen, und in der Legende; die Nummern 1 bis 6 ändern sich nicht. Dieses Fenster bleibt nicht über den anderen. Ist das Fenster schmal, lässt sich die Werkzeugleiste verschieben: Die Schaltfläche **»** holt die Gruppe Kassette.

![Die Standardanordnung. Der Bildschirm zeigt bereits das kleine Programm aus dem nächsten Abschnitt.](img/en/defaut-annote.png "large")

<ol class="legend">
<li><strong>Laden, Sichern, Zustand</strong>: Hex-Dumps <code>.txt</code>, Binärdateien <code>.bin</code> und <code>.prg</code>; Sichern schreibt auch Kassetten <code>.wav</code>/<code>.aiff</code>; Zustand speichert und lädt den Zustand der Maschine.</li>
<li><strong>RESET, Bildschirm löschen, Geschwindigkeit (Anzeige), Einfügen</strong>: die Tasten der Maschine (Strg+R, Strg+L, F3, Strg+V).</li>
<li><strong>BASIC</strong>: lädt das eingebaute RDK G2 Basic nach <code>$E000</code>.</li>
<li><strong>Fenster</strong>: das Menü <strong>⊞ Anordnung ▾</strong>, dann eine Schaltfläche je Fenster (Bildschirme, Editor, Debugger, Disassembler, Dump, Bibliothek, ASCII).</li>
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

!!! astuce "Platz schaffen"
    Um nur zu spielen, schließen Sie Editor und Debugger mit ihren Schaltflächen in der Gruppe Fenster (4): Der Bildschirm wird größer. In der Zoomliste (8) wählt **Anpassen** den größten ganzzahligen Zoom, der passt, **F11** schaltet auf Vollbild, **⊞ Anordnung ▾ → Standard** (4) stellt alles wieder her; dasselbe Menü bietet auch die Anordnung **Spiel** (Apple-1-Bildschirm und Bibliothek). Sind die Schaltflächen für Ihren Bildschirm zu klein oder zu groß, ändern Sie **⚙ Einstellungen → Oberfläche → Größe von Symbolen und Text**.

!!! note "Netzwerkzugriff beim ersten Start"
    Beim allerersten Start nimmt der Emulator Kontakt mit GitHub auf (`api.github.com`, `raw.githubusercontent.com`), um das Vorführprojekt zu holen (`dschmenk/Apple1-Slideshow`, aufgeführt unter „GitHub-Projekte“ in **⚙ Einstellungen**), und zeigt den Fortschritt in der Statusleiste. Sonst wird nichts gesendet. Ohne Netz überspringt er diesen Schritt einfach; entfernen Sie das Projekt aus dieser Liste, um das Herunterladen dauerhaft abzustellen.

## Der Woz Monitor

Der umgekehrte Schrägstrich `\` ist die Eingabeaufforderung des **Woz Monitors**, des 256 Byte großen Programms, das den Apple-1 startet. Klicken Sie auf den Bildschirm und tippen Sie: Kleinbuchstaben werden zu Großbuchstaben, **Eingabe** ist RETURN, die **Rücktaste** erzeugt das `_`, das wie 1976 das letzte Zeichen löscht.

Der Monitor versteht vier Arten von Befehlen:

| Sie tippen | Wirkung |
|---|---|
| `300` | zeigt das Byte an der Adresse `$0300` |
| `300.30F` | zeigt die Bytes von `$0300` bis `$030F` |
| `300: A9 C1` | schreibt `A9` nach `$0300`, dann `C1` nach `$0301` |
| `300R` | startet das Programm, das bei `$0300` beginnt (*Run*) |

## Ihr erstes Programm in Maschinensprache

Tippen Sie diese beiden Zeilen, jede gefolgt von Eingabe:

```mon
» 300: A9 C1 20 EF FF 4C 1F FF

0300: 00
» 300R

0300: A9A
```

Das letzte `A` hat Ihr Programm ausgegeben. Die acht Bytes sind drei Befehle des 6502:

| Bytes | Befehl | Zweck |
|---|---|---|
| `A9 C1` | `LDA #$C1` | lädt den Code des Buchstabens „A“ (mit gesetztem Bit 7, wie der Apple-1 es erwartet) |
| `20 EF FF` | `JSR $FFEF` | ruft ECHO auf, die Routine des Monitors, die ein Zeichen ausgibt |
| `4C 1F FF` | `JMP $FF1F` | kehrt zum Monitor zurück, der auf den nächsten Befehl wartet |

Der Monitor zeigt zuerst das Byte an der Startadresse (`0300: A9`), dann gibt Ihr Programm das `A` aus. Der eingebaute Assembler (vollständiges Handbuch) erspart Ihnen, diese Bytes von Hand auszurechnen.

!!! apple1 "Am echten Apple-1"
    Die Befehle sind identisch: Der Emulator führt das echte ROM des Woz Monitors aus. Die echte Maschine zeigt etwa 60 Zeichen pro Sekunde an. Der Emulator hält dieses Tempo, wenn die Anzeigezeile des Bildschirms auf **Normal** steht; **Schnell** zeigt alles auf einmal. Die Einstellung gilt für den Apple-1-Bildschirm und den GEN2-Bildschirm gemeinsam.
