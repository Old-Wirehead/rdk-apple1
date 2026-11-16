# Installation unter Linux

## Entpacken und starten

<ol class="steps">
<li><strong>Entpacken Sie das Archiv.</strong> Rechtsklick auf <code>apple1-emu.zip</code>, dann „Hier entpacken“. Ein Ordner <code>apple1-emu</code> erscheint.</li>
<li><strong>Doppelklicken Sie auf <code>run.sh</code>.</strong> Fragt der Dateimanager, was mit der Datei geschehen soll (zum Beispiel unter Linux Mint), wählen Sie <strong>„Ausführen“</strong> (nicht „Anzeigen“). Das Apple-1-Fenster öffnet sich, ohne Terminal.</li>
<li><strong>Auf Wunsch: den Emulator ins Menü aufnehmen.</strong> Öffnen Sie im Ordner ein Terminal (Rechtsklick → „Im Terminal öffnen“) und geben Sie den folgenden Befehl ein. Ein Eintrag <strong>Retro Dev Kit Apple 1</strong> erscheint im Bereich <em>Spiele</em> des Menüs, zusammen mit den Befehlen <code>apple1-emu-gui</code> (Fenster) und <code>apple1-emu</code> (Terminal).</li>
</ol>

```sh
./install.sh             # Anwendungsmenü + Befehle in ~/.local/bin
./install.sh --bureau    # dasselbe, dazu ein Symbol auf dem Schreibtisch
./install.sh --desinstaller
```

Die Deinstallation (`--desinstaller`) entfernt die Programme und den Menüeintrag, behält aber Ihre Programme (`~/apple1/programmes`) und Ihre Einstellungen (`~/.config/apple1-emu`). Kein Befehl braucht `sudo`: Alles wird in Ihrem persönlichen Ordner installiert. Beim ersten Start einer neuen Version wird angeboten, mit den ursprünglichen Einstellungen neu zu beginnen; die Bibliothek bleibt erhalten. Die Schaltfläche **Einstellungen zurücksetzen** tut dasselbe. Eine leere Datei namens `portable` neben dem Programm (oder `APPLE1_PORTABLE=1`) hält Einstellungen, Standardbibliothek und Cache im Ordner des Kits.

!!! astuce "Aktualisieren von einer älteren Version"
    Die Skripte `lancer.sh` und `installer.sh` früherer Versionen heißen jetzt `run.sh` und `install.sh`. Führen Sie `./install.sh` erneut aus: Es ersetzt die alte Installation sowie den Menüeintrag und das Schreibtischsymbol, die auf `lancer.sh` zeigten.

!!! astuce "Die Terminal-Version"
    `./run.sh --terminal` öffnet den Emulator in einem Terminal (mindestens 42 Spalten × 28 Zeilen), ohne grafische Oberfläche. Praktisch über eine SSH-Verbindung. Das vollständige Handbuch beschreibt ihre Tasten und Befehle.

## Wenn sich das Fenster nicht öffnet

`run.sh` erkennt einen sofortigen Fehlstart (Grafiktreiber, virtuelle Maschine ohne Beschleunigung) und versucht es von selbst erneut mit der Software-Darstellung von Mesa (`LIBGL_ALWAYS_SOFTWARE=1`). Schlägt auch das fehl, erscheint eine Meldung, und das Protokoll steht in `~/.cache/apple1-emu/gui.log`.

| Symptom | Was tun |
|---|---|
| Der Dateimanager öffnet `run.sh` im Texteditor | Rechtsklick → Eigenschaften → Zugriffsrechte: „Datei als Programm ausführen“ ankreuzen, dann erneut doppelklicken und „Ausführen“ wählen. |
| „Keine Berechtigung“ im Terminal | `chmod +x run.sh install.sh bin/*/apple1-*` |
| Schwarzes oder sehr langsames Fenster | In einer virtuellen Maschine normal: Die Software-Darstellung bleibt für den Apple-1-Bildschirm flüssig. Schalten Sie bei Bedarf den „Vintage-Bildschirm“ aus. |
| Kein Kassettenton | Der Ton läuft über ALSA; ohne Soundkarte bleibt er einfach stumm. Das Laden funktioniert trotzdem. |

## Wo Ihre Dateien liegen

| Ort | Inhalt |
|---|---|
| `~/apple1/programmes` | Die **Bibliothek**: Ihre Kassetten, Programme und Beispiele (beim ersten Start angelegt). |
| `~/.config/apple1-emu/gui.json` | Einstellungen: Zoom, Farben, Anordnung der Fenster, Editor-Tabs. |
| `~/.cache/apple1-emu/` | Umgewandelte Kassetten, heruntergeladene GitHub-Projekte, das Protokoll `gui.log`. |
