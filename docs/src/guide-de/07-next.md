# Wie geht es weiter? {: .newpage }

Sie wissen jetzt, wie man eine Kassette lädt, ein BASIC-Programm schreibt und startet und es auf eine echte Maschine überträgt. Das **vollständige Handbuch** (auf Englisch: `docs/Complete-Manual.pdf`) geht viel weiter:

- **Anleitung zum Integer BASIC**: jeder Befehl mit Beispielen, die Grenzen, die Fehlermeldungen, die bekannten Fehler und wie man sie umgeht, die Anpassung von Applesoft-Programmen.
- **Anleitung zum 6502-Assembler**: der eingebaute Assembler (Syntax von ca65, Makros, Listing), die Routinen des Woz Monitors, im Emulator getestete Beispiele.
- **Spiele zum Spielen und Lesen**: Die Bibliothek enthält fünf kleine Spiele von Claude IA (Galgenmännchen, Streichhölzer, Schiffe versenken in BASIC; Tunnel und Tic-Tac-Toe in Assembler), jeweils auf Französisch und Englisch.
- **Die Werkzeuge**: Debugger, Bibliothek und GitHub-Projekte, GEN2-Farbkarte und P-LAB-Grafikkarte, Foto → ASCII, Vintage-Bildschirm, Befehlszeile.

## Die wichtigsten Tastenkürzel

| Taste | Wirkung |
|---|---|
| Strg+R | RESET (zurück zum Monitor) |
| Strg+L | CLEAR SCREEN |
| F3 | Originalgeschwindigkeit oder volle Geschwindigkeit |
| Strg+V | tippt die Zwischenablage auf der Tastatur des Apple-1 (auch: Rechtsklick auf den Bildschirm → Einfügen) |
| Strg+Plus, Strg+Minus, Strg+0 | Zoom des Bildschirms und an das Fenster angepasster Zoom (im Editor: Textgröße) |
| F11 | Vollbild |
| F1 | ausführliche Hilfeblasen (wie die Schaltfläche ?) |
| Rücktaste | `_`, das Löschzeichen des Monitors |
| Strg+F, Strg+H, Strg+G | im Editor: suchen, ersetzen, zu einer Zeile gehen |
| F5, F9 | im Editor: den Tab starten, Haltepunkt auf der Zeile |

## Befehle zum Merken

| Befehl | Wirkung |
|---|---|
| `E000R` | startet das Integer BASIC (kalt: löscht das Programm) |
| `E2B3R` | kehrt ins BASIC zurück, ohne das Programm zu löschen |
| `C100R` | startet das Programm der ACI (Eingabeaufforderung `*`) |
| `anfang.endeR` | liest die Kassette in diesen Bereich, zum Beispiel `0300.031ER` |
| `300R` | startet ein Programm in Maschinensprache bei `$0300` |
