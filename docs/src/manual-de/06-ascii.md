# Foto → ASCII

<p class="lead">Das Werkzeug Foto → ASCII macht aus einem Foto ein Apple-1-Bild aus 40 × 24 Zeichen und daraus ein BASIC- oder 6502-Programm, bereit zum Starten oder zum Export auf Kassette.</p>

![Das Werkzeug Foto → ASCII: das Foto mit seinem Zuschnittrahmen, die Einstellungen und die mit der Schrift des 2513 gezeichnete Vorschau.](img/de/brut/ascii-outil.png "large")

## Ein Bild öffnen

**🎨 ASCII** öffnet das Werkzeug, einen andockbaren Bereich wie die anderen. **Ziehen Sie** ein Foto `.jpg` oder `.png` auf das Fenster, oder nehmen Sie **📂 Öffnen…** (ein Foto oder eine fertige ASCII-Art als `.txt`).

Der grüne Rahmen auf dem Foto zeigt den umgewandelten Teil: Ziehen Sie ihn, um ihn zu verschieben, das Mausrad zoomt.

## Einstellungen

| Einstellung | Rolle |
|---|---|
| **Spalten**, **Zeilen** | höchstens 40 × 24. **23 Zeilen** lässt die letzte Zeile der Eingabeaufforderung, damit das Bild nicht rollt. |
| **Zoom**, **Mitte** | der beibehaltene Teil des Fotos |
| **Helligkeit**, **Kontrast**, **Gamma** | die üblichen Einstellungen (Gamma > 1 hellt auf) |
| **Invertieren** | dunkel = viel Tinte |
| **Auto-Tonwert** | streckt die Graustufen (2 % bis 98 %) vor den anderen Einstellungen |
| **Dithering** | Floyd-Steinberg: mehr Abstufungen, mehr Rauschen |
| **Zeichen** | die **vollständige** Rampe (26 Stufen), **Einfach** (10), die aus der **Diashow von Schmenk** oder **Eigene** |

Nur die Zeichen des 2513 sind möglich: Großbuchstaben, Ziffern und Zeichen, ohne Kleinbuchstaben, ohne `"` und ohne `_`. Sie sind nach der Zahl der leuchtenden Punkte in der Schrift des Apple-1 geordnet: Diese Zahl, nicht die Form, ergibt die Graustufe.

!!! astuce "Für ein Porträt"
    40 × 24 Zeichen sind wenig: Schneiden Sie auf das Gesicht zu (Zoom 1,3 bis 1,6), erhöhen Sie den Kontrast etwas und versuchen Sie bei einem hellen Foto ein Gamma unter 1. Ein einfarbiger dunkler Hintergrund gibt die besten Ergebnisse.

## Das erzeugte Programm

- **BASIC**: eine Zeile `PRINT "…"` je Zeile des Bildes. Eine volle Zeile von 40 Zeichen endet mit `;`, weil der Bildschirm von selbst umbricht. Das Eintippen des Programms dauert (etwa 2 s pro Zeile).
- **6502**: ein ca65-Quelltext bei `$0300`, gestartet mit `300R`. Nach dem Löschen des Bildschirms sendet eine Schleife eine komprimierte Tabelle an ECHO (Folgen gleicher Zeichen als „Anzahl + Zeichen“ codiert). Die Anzeige geht sofort.

Mit 24 Zeilen wartet das Programm vor der Rückkehr auf eine Taste; mit 23 landet die Eingabeaufforderung auf der letzten Zeile, und das Bild bleibt vollständig.

Die Schaltflächen: **✎ In den Editor einfügen** (neuer BASIC- oder 6502-Tab), **▶ Starten**, **📼 WAV exportieren…**, **💾 In der Bibliothek speichern** (eine `.txt` im Ordner „Art ASCII“), **📋 Text kopieren**, **Code anzeigen**.

![Vom Foto zum echten Apple-1: Umwandlung, Programm, Kassette.](diagrams/de/workflow-ascii.svg "diagram large")

## Auf der Befehlszeile

```sh
apple1-emu ascii foto.jpg                        # nur das Bild, auf dem Bildschirm
apple1-emu ascii woz.jpg --zoom 1.3 --cx 0.45 --cy 0.32 \
          --contrast 1.3 --gamma 0.45 --rows 23 -o woz.txt
apple1-emu ascii woz.jpg -o woz.bas              # BASIC-Programm
apple1-emu ascii woz.jpg -o woz.s                # 6502-Quelltext
apple1-emu ascii woz.jpg -o woz.wav              # 6502-Kassette, gestartet mit 300R
apple1-emu ascii woz.jpg -o woz.wav --format basic
```

Kapitel 12 nennt alle Optionen.

!!! apple1 "Die Diashow von Dave Schmenk"
    2014 schrieb Dave Schmenk eine Porträt-Diashow für den Apple-1. Die Bibliothek fügt sie als GitHub-Projekt hinzu (`dschmenk/Apple1-Slideshow`): Sie wird heruntergeladen, nicht mitgeliefert, weil das Repository keine Lizenz hat. **▶ Neuesten Build testen** assembliert sie (Syntax von xa, automatisch übersetzt; dieselben 3456 Bytes wie xa 2.4.1) und startet sie mit `280R`.
