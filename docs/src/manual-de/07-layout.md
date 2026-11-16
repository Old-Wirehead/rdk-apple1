# Anordnung der Bereiche

<p class="lead">Jeder Bereich ist ein Tab, den Sie hinlegen, wo Sie möchten: nebeneinander, in Tabs oder in einem eigenen Fenster auf einem zweiten Bildschirm.</p>

## Einen Bereich verschieben

Der Editor, der Apple-1-Bildschirm, der GEN2-Farbmonitor, der Debugger, der Disassembler, der Dump, das ASCII-Werkzeug und die Bibliothek sind alle **andockbare Tabs**. Ziehen Sie einen Tab an seinem Titel: Hilfslinien erscheinen, und Sie legen ihn **links von**, **rechts von**, **über** oder **unter** einem anderen Bereich ab, oder **in dieselbe Tab-Gruppe**. Ziehen Sie die Trenner, um die Größe zu ändern. Die Anordnung wird in `gui.json` gespeichert.

## Die Anfangsanordnung

Der Editor steht links; in der Mitte der Apple-1-Bildschirm, darunter der Farbmonitor, wenn die GEN2-Karte eingesteckt ist; rechts der Debugger mit dem Disassembler darunter, dann der Dump und ganz rechts die Bibliothek. Beim allerersten Start richtet sich die Wahl nach der Breite des **Monitors** (das Fenster öffnet sich klein und wird einen Moment später maximiert): Unter 1400 Punkten (ein Laptop mit 1366 × 768, ein Bildschirm mit 1024 × 768) startet es mit **Editor und Bildschirm**, dem nützlichsten Paar; ab 1400 Punkten mit allen Bereichen. Wählen Sie jederzeit eine andere Anordnung im Menü. Sie folgt der Breite des Fensters: Jeder Bereich behält seinen Anteil, vom Laptop bis zum breiten Monitor. Das Menü **⊞ Anordnung ▾** der Werkzeugleiste stellt sie jederzeit wieder her (**Standard**, alle Bereiche offen) und bietet vier weitere fertige Anordnungen: **Spiel** (Apple-1-Bildschirm und Bibliothek), **BASIC** (Editor und Bildschirm), **Assembler** (Editor, Bildschirm und Debugger) und **Debugging** (Editor, Debugger, Disassembler und Dump). **Aktuelle Anordnung speichern…** fragt nach einem Namen und behält die aktuelle Anordnung, mit ihren abgedockten Fenstern und der Größe des Hauptfensters (beim Laden öffnen sich diese Fenster wieder dort, wo sie waren); Ihre Anordnungen stehen unter einem Trenner in alphabetischer Reihenfolge, und ein Rechtsklick erlaubt **Umbenennen** oder **Löschen**. Sie werden in `gui.json` gespeichert.

## Der Startassistent

Beim ersten Start ersetzt ein Assistent aus drei Seiten das alte Fenster „Ihr Arbeitsbereich“. Er ist bernsteinfarben auf Schwarz gezeichnet, mit dem Röhreneffekt (Rasterzeilen, Leuchten, Vignette, Wölbung 0,07) auf dem Rahmen; der kleine Text und die Vorschaubilder bleiben scharf. Ein kleines **×** oben rechts schließt ihn. **‹ Zurück** und **Weiter ›** wechseln zwischen den Seiten; die letzte Seite hat **Fertig**.

1. **Willkommen**: wozu das Kit dient.
2. **ROM und BASIC**: Der Woz Monitor, das Kassetten-ROM und G2 Basic sind eingebaut. Sonst ist nichts nötig. Puristen installieren ihr eigenes originales Integer BASIC aus einer externen Quelle; die Schaltfläche öffnet die Anleitung („Ein ROM installieren“), die nur Links enthält, nie die Dateien.
3. **Ihr Arbeitsbereich**: die fünf Anordnungen (**Standard**, **Spiel**, **BASIC**, **Assembler**, **Debugging**). Jedes Vorschaubild ist anklickbar (es gibt keine Schaltfläche „Wählen“). Die Rechtecke behalten die Nummern 1 bis 6, und der Farbbildschirm (GEN2 / P-LAB) ist die **7**, in seiner eigenen Farbe, in den Vorschaubildern, die ihn zeigen, und in der Legende.

**Dieses Fenster beim Start anzeigen** steht auf allen drei Seiten und ist standardmäßig angekreuzt. Dasselbe Kästchen steht unter **⚙ Einstellungen → Oberfläche**. **⊞ Anordnung ▾ → Arbeitsbereich wählen…** öffnet ihn erneut. Es ist ein gewöhnliches Fenster: Es bleibt nicht über den anderen. Es öffnet sich nicht, wenn die Befehlszeile eine Größe vorgibt (`--taille`), und nicht bei Bildschirmfotos.

## Textgröße jedes Fensters

Der Editor, das Assembler-Listing, der Debugger, der Disassembler, der Dump und die Bibliothek haben in ihrer Leiste je ein kompaktes Bedienelement **A 13 ▲▼**: Die Dreiecke ändern die Größe, die für dieses Fenster gespeichert wird. Strg+Mausrad und Strg+Plus / Strg+Minus wirken auf das Fenster unter dem Mauszeiger (über dem Apple-1-Bildschirm ändern Strg+Plus / Strg+Minus den Zoom). **Einstellungen zurücksetzen** setzt jede dieser Größen auf 13 zurück.

## Was zwischen zwei Starts gespeichert wird

Die Anordnung wird beim Beenden von selbst gespeichert und so wiedergefunden, wie sie war: Position und Größe jedes Bereichs, die abgedockten Fenster, der Zoom und die gewählte Anordnung. Das Fenster wird in seiner Größe und an seinem Platz wiederhergestellt, dann werden die Bereiche in denselben Proportionen zurückgelegt, welcher Bildschirm es auch sei: Die Anordnung wurde bei 2880 × 1620, 2560 × 1440, 1920 × 1080 und 1366 × 768 geprüft, mit weniger als 1 % abweichendem Bild nach einem Neustart. Eine unter einem Namen gespeicherte Anordnung (**Aktuelle Anordnung speichern…**) behält zusätzlich die abgedockten Fenster. Die Oberfläche zoomt nie von selbst: Nur die Einstellung „Größe von Symbolen und Text“ ändert die Größe der Schaltflächen.

## Wenn der Bildschirm größer ist als sein Bereich

Mit einem **festen Zoom** (2x, 3x… oder <kbd>Strg</kbd>+<kbd>+</kbd>) behält der Apple-1-Bildschirm die verlangte Größe, auch wenn der Bereich kleiner ist: Er **rollt** dann (Mausrad oder Bildlaufleisten), statt verkleinert zu werden, und die Statusleiste sagt es, mit dem Hinweis, den Bereich zu vergrößern. Nur **Anpassen** passt den Zoom an den Bereich an. In einem schmalen Fenster (zum Beispiel 1024 × 768) rollt die Werkzeugleiste seitlich, statt abgeschnitten zu werden: Die Schaltfläche **»** holt die Gruppe Kassette, ein zweiter Klick geht zurück zum Anfang. Die Schaltfläche **Starten** des Editors bleibt immer sichtbar.

## Einen Bereich abdocken

Jeder Bereich hat oben rechts eine Schaltfläche **Abdocken** (auch per Rechtsklick auf den Tab), die ihn in **einem eigenen Fenster** öffnet, das Sie auf einen zweiten Bildschirm verschieben können; seine Größe und sein Platz werden gespeichert. **⊟ Andocken** legt ihn an seinen ursprünglichen Platz zurück.

- Der Apple-1-Bildschirm und der Farbmonitor werden getrennt abgedockt. Die Tastatur geht von jedem ihrer Fenster und vom Hauptfenster aus zum Apple-1. Das Schließen des Fensters eines Bildschirms dockt ihn wieder an.
- Das Schließen des Fensters eines Werkzeugs (Editor, Debugger…) schließt das Werkzeug; seine Schaltfläche in der Werkzeugleiste öffnet es am selben Ort wieder.
- Dialogfenster (Öffnen, Speichern unter, Exportieren) öffnen sich immer im Hauptfenster. In den Dateidialogen schließt <kbd>Esc</kbd> den Dialog (auch aus dem Feld für den Dateinamen), <kbd>Strg</kbd>+<kbd>L</kbd> bearbeitet den Pfad (sein Text ist markiert, Tippen ersetzt ihn); Speichern und Exportieren eines Tabs beginnen im Ordner seiner Datei.
- Gespeicherte Anordnungen behalten die abgedockten Fenster; das Hauptfenster behält von einem Start zum nächsten dieselbe Größe, unabhängig vom Zoom.
- Das Kassetten-Oszilloskop bleibt ein kleines schwebendes Fenster im Hauptfenster.

<div class="cards">
<div class="card"><h4>Zum Spielen</h4>Schließen Sie Editor und Debugger, behalten Sie die Bibliothek. <strong>Anpassen</strong> vergrößert den Bildschirm; <kbd>F11</kbd> schaltet auf Vollbild, mit dem Vintage-Bildschirm.</div>
<div class="card"><h4>Zum Programmieren</h4>Editor links, Bildschirm in der Mitte, Debugger und Dump rechts. Docken Sie den Editor auf einen zweiten Monitor ab, um ihm die volle Höhe zu geben.</div>
<div class="card"><h4>Für die GEN2</h4>Stecken Sie die Karte ein: Der Farbmonitor kommt unter den Apple-1-Bildschirm. Docken Sie ihn ab, um ihn groß zu haben, wie einen zweiten Monitor.</div>
<div class="card"><h4>Für einen Vortrag</h4><strong>Abdocken</strong> des Apple-1-Bildschirms auf den Projektor, im Vollbild, und das Hauptfenster auf Ihrem Laptop.</div>
</div>

# Einstellungen

<p class="lead">⚙ Einstellungen vereint alle dauerhaften Einstellungen. Alles wird in ~/.config/apple1-emu/gui.json gespeichert (oder im Ordner des Kits, im portablen Modus).</p>

![Der Tab Oberfläche der Einstellungen: Sprache, Thema, Größe von Symbolen und Text, kompakte Leiste, Hilfeblasen, Startfenster und Farben der Oberfläche.](img/de/preferences.png "large")

Die Einstellungen verteilen sich auf sechs Tabs:

| Tab | Einstellungen |
|---|---|
| **Oberfläche** | Sprache: automatisch (nach der Tastaturbelegung) oder fest: Französisch, Englisch, Deutsch, Russisch, Spanisch, Italienisch, Portugiesisch; die erkannte Sprache und ihre Quelle werden angezeigt. Thema: dunkel, hell oder System. Größe von Symbolen und Text: klein, normal (Standard), groß, sehr groß. Kompakte Leiste (nur Symbole, wenn das Fenster schmal ist). Hilfeblasen beim Überfahren. **Dieses Fenster beim Start anzeigen** (das Fenster „Ihr Arbeitsbereich“) und die Schaltfläche, die es sofort öffnet. **Farben der Oberfläche**: die Farben der Listen der Oberfläche, **getrennt für das dunkle und das helle Thema** gewählt (aktive Schaltflächen, aktuelle Zeile des Debuggers, hervorgehobene Zeile, ausgewählte Zelle, Haltepunktzeile, Adressen und ASCII-Spalte des Dumps, geänderte Werte, Marken); jede Farbe hat ein Farbfeld mit Live-Vorschau, die Standardwerte sind gut lesbar (Kontrast von mindestens 4,5:1 auf dem Hintergrund des Themas), und Schaltflächen **Zurücksetzen** stellen ein Thema oder beide wieder her. |
| **Editor** | Syntaxfarben, für BASIC und für den 6502-Assembler getrennt (Kapitel 7) |
| **Bibliothek** | der Hauptordner (**Durchsuchen…**, **⟲ Standard**, **Ordner öffnen**), die weiteren Ordner, die GitHub-Projekte und das freiwillige GitHub-Token (nur für private Repositorys und Artefakte von Actions; nie angezeigt, und die Variable `GITHUB_TOKEN` hat Vorrang); Position der Bibliothek, links oder rechts (standardmäßig rechts) |
| **Kassette** | Kassettenton (bei schneller Geschwindigkeit stumm) und seine Lautstärke, Oszilloskop bei Wiedergabe oder Aufnahme |
| **GEN2** | die Farbkarte von Uncle Bernie, Bildwechsel 60 oder 50 Hz, LORES-Palette (Kapitel 6) |
| **Erweitert** | Anzeige und Geschwindigkeit: Bibliothek anzeigen, Anzeige auf ~60 Zeichen/s begrenzt wie beim echten Terminal, schnelle Geschwindigkeit; **Vintage-Bildschirm: Feineinstellungen** (für beide Bildschirme, Kapitel 3); die Integer-BASIC-Datei, die nach `$E000` geladen wird (**📂 Wählen…** / **Vergessen**) |

Jeder Schieberegler der Einstellungen (Stärke, Nachleuchten und Wölbung des Vintage-Bildschirms, Kassettenlautstärke) zeigt daneben seinen **Zahlenwert**. Beim ersten Start ist die Wölbung des Vintage-Bildschirms auf **0,07** gesetzt (eine leichte Rundung der Röhre); eine bestehende Installation behält ihren eigenen Wert. Die Vorschau des Tabs **Editor** folgt der Sprache der Oberfläche.

Der Zoom, der Phosphor und das Kästchen Vintage-Bildschirm stehen nicht in den Einstellungen: Sie stehen in der Anzeigezeile jedes Bildschirms (Kapitel 3).

![Der Tab Editor: ein Farbschema für BASIC und für den 6502-Assembler oder Ihre eigenen Farben.](img/de/preferences-editeur.png "large")

**Automatische Sprache.** Der Emulator folgt der aktiven Tastaturbelegung: `fr` ergibt Französisch, `us` oder `gb` Englisch, `de`, `ch` oder `at` Deutsch, `ru` Russisch und so weiter. Er fragt GNOME, `setxkbmap`, Cinnamon (der Fall von Linux Mint), `localectl`, dann `/etc/default/keyboard` und schließlich die Variablen `LANG`. Auf der Befehlszeile: `--langue fr|en|de|ru|es|it|pt|auto`.

!!! astuce "Von vorn anfangen"
    **Einstellungen zurücksetzen** unten im Fenster stellt nach einer Bestätigung die ursprünglichen Einstellungen wieder her (Sprache, Thema, Anordnung, Vintage-Bildschirm…). Die Dateien Ihrer Bibliothek bleiben, wo sie sind; nicht gespeicherte Editor-Tabs werden geschlossen. Der erste Start einer neuen Kit-Version stellt dieselbe Frage, weil `gui.json` erhalten bleibt, wenn der Programmordner gelöscht wird, um einen anderen zu installieren.

    Für ein portables Kit legen Sie eine leere Datei namens `portable` in das Wurzelverzeichnis des Kits (neben `run.sh`) oder starten es mit `APPLE1_PORTABLE=1`: Die Einstellungen (`config/gui.json`), die Standardbibliothek (`programmes`) und der Cache liegen dann im Ordner des Kits.

    Den Emulator zu schließen und `~/.config/apple1-emu/gui.json` umzubenennen bewirkt beim nächsten Start dasselbe wie die Schaltfläche. Die Programmeinstellungen bleiben in der `programs.json` jedes Ordners.
