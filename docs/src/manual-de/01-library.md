# Die Bibliothek

<p class="lead">Die Bibliothek ist der Ausgangspunkt für fast alles: Sie bewahrt Ihre Kassetten, Dumps und Quelltexte auf, erkennt die Kassetten von damals und startet ein Programm mit einem Klick, BASIC inbegriffen.</p>

![Der Bereich Bibliothek: der Hauptordner, der Ordner „Art ASCII“, erkannte Kassetten und ein GitHub-Projekt. BASIC.wav wird gerade abgespielt.](img/de/bibliotheque.png "small")

## Ordner

Der Hauptordner ist `~/apple1/programmes`. Er wird beim ersten Start angelegt, mit den Beispielen in fünf Ordnern: **Art ASCII** (elf ASCII-Art-Beispiele im Format 40 × 24, Kapitel 9), **BASIC** (Programme `.bas` und BASIC-Dumps `.hex.txt`), **6502** (Quelltexte `.s`/`.asm` mit ihren `.bin`/`.hex`), **GEN2 COLOR** (die Tests der GEN2-Karte von Bernie, `gen2-*.apl`, und die Farbspiele für G2 Basic) und **WAV** (alle Kassetten). Im Bereich stehen diese fünf Ordner direkt auf der obersten Ebene, in der Reihenfolge **BASIC**, **6502**, **GEN2 COLOR**, **WAV**, **Art ASCII**: Der Wurzelordner (der Name des Verzeichnisses auf der Festplatte) wird darüber nicht angezeigt, und lose Dateien in diesem Ordner werden nicht aufgelistet. Legen Sie sie in einen der fünf Ordner. Ein Ordner, den Sie hinzufügen, oder ein GitHub-Projekt behält seinen eigenen Titel. Eine schon vorhandene Bibliothek wird nie umgeordnet, nur ihre Ordner **Assembleur** und **Uncle Bernie** (die Namen der Versionen 7.8 bis 7.9.1) werden in **6502** und **GEN2 COLOR** umbenannt, mit ihrem Inhalt. Um einen anderen zu wählen: **⚙ Einstellungen → Bibliotheksordner → 📂 Durchsuchen…** (**⟲ Standard** kehrt zu `~/apple1/programmes` zurück).

**+ Ordner** oben im Bereich fügt weitere Ordner hinzu; der Hauptordner bleibt immer. Die Bibliothek ist ein Baum: Ordner und Unterordner klappen zu, und ihr Zustand wird gespeichert. Jeder Ordner hat seine eigenen Einstellungen in einer Datei `programs.json`. Ein Rechtsklick auf einen hinzugefügten Ordner öffnet ihn im Dateimanager oder entfernt ihn aus der Liste (auf der Festplatte wird nichts gelöscht).

**Schmaler Bereich.** Die Bibliothek darf so schmal sein, wie Sie möchten (bis zu einer Mindestbreite, unter der der Trenner anhält). Jeder Eintrag passt sich dann an: Der Name nimmt die ganze Breite ein, mit „…“ gekürzt, wenn er zu lang ist (die Hilfeblase zeigt den ganzen Namen), die kleinen Symbole ▶ ⬇ 📼 ⚙ rücken in die nächste Zeile, und die Beschreibung ist auf ein oder zwei Zeilen begrenzt, ebenfalls mit „…“ gekürzt, der ganze Text steht in der Hilfeblase. In der Kopfzeile werden **+ Ordner** und **+ GitHub-Projekt** zu Symbolen mit Hilfeblasen.

Angezeigt werden die Dateien: `.wav .aiff .bin .hex .txt .apl .bas .s .asm .prg`. Die Liste aktualisiert sich alle zwei Sekunden: Kopieren Sie eine Datei in den Ordner, und sie erscheint.

Für jedes Programm zeigt die Liste den Namen, den Typ, die Adressbereiche, die Länge der Kassette, den Startbefehl und Ihre Notizen.

### Die Spiele von Claude IA

Die Ordner **BASIC** und **6502** erhalten auch fünf kleine Spiele von Claude IA, jedes in einer französischen und einer englischen Fassung: *Galgenmännchen* (`pendu.bas`, `hangman.bas`), *Streichhölzer* (`allumettes.bas`, `matches.bas`) und *Schiffe versenken* (`bataille.bas`, `battleship.bas`) in BASIC; *Tunnel* (`tunnel.s`, `tunnel-en.s`) und *Tic-Tac-Toe* (`morpion.s`, `tictactoe.s`) in 6502-Assembler. Ein Doppelklick auf ein BASIC-Spiel startet es auf RDK G2 Basic (der Emulator startet BASIC, lädt das Listing und tippt `RUN`); Assembler-Quelltexte übersetzt der eingebaute Assembler. Die Assembler-Quelltexte binden `apple1.inc` ein, das mit ihnen installiert wird. Der Ordner **GEN2 COLOR** enthält außerdem *Vier gewinnt in Farbe* (`puissance4-couleur.bas`), ein Spiel für G2 Basic und die GEN2-Karte, von Claude IA. Q und S bewegen den Stein über dem Gitter, die Leertaste lässt ihn fallen. Die Spiele gehören zum Projekt und stehen unter seiner Lizenz (Anhang „Lizenz und Danksagungen“). Deutsche Fassungen dieser Spiele liegen in `docs/exemples/de/`.

## Ein Programm starten

Jedes Programm hat rechts vier kleine Symbole:

| Symbol | Aktion |
|---|---|
| **▶** | **Laden und starten**: lädt das Programm, falls nötig, und startet es dann: RESET, danach sein Befehl, auf der Tastatur getippt (`300R`, oder `E2B3R` und `RUN` bei einem BASIC-Programm). Braucht das Programm Integer BASIC und ist es nicht in `$E000`, lädt der Emulator es zuerst. Ein BASIC-Listing wird ins Integer BASIC getippt, ein 6502-Quelltext wird assembliert. |
| **⬇** | **Laden**: legt das Programm sofort in den Speicher, ohne es zu starten. |
| **📼** | **Über die Kassette**: wie an der echten Maschine. Der Emulator drückt RESET, tippt `C100R` und dann den Lesebefehl der ACI (zum Beispiel `0300.031ER`), und die Kassette läuft in Originalgeschwindigkeit, mit Ton und Fortschrittsbalken. Dateien `.txt` und `.bin` werden vorher in eine Kassette umgewandelt. Ausgegraut bei einem Quelltext (`.bas`, `.s`), der keine Aufnahme hat. |
| **⚙** | **Parameter**: Bereiche, Ladeadresse einer `.bin`, Startadresse und -befehl, Typ, ob BASIC nötig ist, Notizen. |

{{!Doppelklick}}Das Laden geht **sofort**, es sei denn, Sie wählen mit 📼 die Kassette. Ein **Doppelklick** auf ein Programm lädt es und startet es, wenn es einen Startbefehl hat (wie ▶). Die Kassette wird nur benutzt, wenn Sie sie verlangen; das Deck der Werkzeugleiste zeigt dann die Kassette und ihre Position.

Ist der Editor offen, öffnen ▶, ⬇ oder ein Doppelklick auf ein BASIC-Listing oder einen 6502-Quelltext ihn außerdem **in einem Tab des Editors**, markiert mit 📚; ist er schon offen, wird sein Tab einfach angezeigt.

Ein **Rechtsklick** auf ein Programm bietet den Rest: laden, laden und starten, als Kassette einlegen ohne etwas zu tippen, Parameter, umbenennen, löschen (mit Bestätigung), den Ordner öffnen.

## Die Einstellungen eines Programms

![Die Einstellungen ⚙ von Hamurabi: zwei Bereiche (Zero Page des BASIC, dann das Programm), gestartet mit E2B3R, dann RUN.](img/de/brut/bibliotheque-params.png "large")

| Feld | Rolle |
|---|---|
| **Bereiche** | Die Speicherbereiche, geschrieben wie für die ACI: `0300.031E`, oder `004A.00FF 0800.0FFF` bei einem BASIC-Programm. Jede Aufnahme der Kassette geht der Reihe nach in ihren Bereich. |
| **Ladeadresse** | Für eine `.bin`, die keine Adresse enthält. |
| **Start** | Die Adresse und der Befehl, den ▶ tippt. `\n` steht für RETURN zwischen zwei Zeilen: `E2B3R\nRUN`. |
| **Typ** | **Maschinensprache**, **G2-Basic-Programm** (Standard für ein BASIC-Listing, da der Emulator es mitbringt) oder **Integer-BASIC-Programm** (für alle, die Apples Original vorziehen: ▶ lädt dann das in den Einstellungen gewählte originale Integer BASIC). Der für ein Programm gewählte Typ hat Vorrang vor der allgemeinen BASIC-Einstellung. |
| **Braucht ein BASIC** | ▶ lädt zuerst das BASIC des Programmtyps. |
| **Notizen** | Freier Text, in der Liste angezeigt. |

Die Werte werden aus der Datei vorbelegt und beim Speichern geprüft; **⟲ Werte aus der Datei** liest sie erneut.

!!! attention "Immer vier Ziffern: 004A.00FF, nie 4A.FF"
    Die Einstellungen lehnen `4A.FF` ab. Das ist keine Laune: Das ROM der ACI löscht seinen Hex-Akkumulator am Punkt nicht, die Ziffern schieben sich also weiter hinein, und es würde `$004A-$4AFF` lesen (siehe Abschnitt 5.6). Deshalb schreibt der Emulator Adressen immer vierstellig.

## Erkannte Kassetten

Wird eine Datei **erkannt**, füllen sich ihre Einstellungen von selbst, und die Liste zeigt **✔ erkannt**. Der Emulator decodiert die Aufnahmen und vergleicht ihre Größen und SHA-256-Prüfsummen mit einer eingebauten Tabelle: den 24 Kassetten der Sammlung „Apple1Cassettes“ auf archive.org. Der Dateiname spielt dann überhaupt keine Rolle. Andernfalls erkennt er den **Namen** (ohne Rücksicht auf Groß- und Kleinschreibung, Leerzeichen und Bindestriche) bei gleichen Aufnahmegrößen; die Hilfeblase weist dann darauf hin, dass der Inhalt abweicht.

| Kassette | Bereiche | Start |
|---|---|---|
| `BASIC.wav` (Integer BASIC) | `E000.EFFF` | `E000R` |
| `Hamurabi.wav`, `StarTrek.wav`, `Wumpus.wav`, `Slots.wav`, `Craps.wav`, `Bowling.wav`, `Buzzword.wav`, `WordCross.wav`, `AppleiEnhancedCheckers.wav`, `LunarlanderasciiGraphics.wav` | `004A.00FF 0300.0FFF` | `E2B3R ⏎ RUN` |
| `AceyDucey.wav`, `Blackjack.wav`, `HiLo.wav` | `004A.00FF 0800.0FFF` | `E2B3R ⏎ RUN` |
| `Football.wav` | `004A.00FF 0400.2FFF` | `E2B3R ⏎ RUN` (12 KB) |
| `DealOrNoDeal.wav`, `StarTrek2003.wav` | `004A.00FF 0300.3FFF` | `E2B3R ⏎ RUN` (16 KB) |
| `APPLE30TH.wav` | `0280.0FFF` | `280R` |
| `ASMmchess.wav` | `0300.0BFF` | `300R` |
| `Microchess.wav` | `1000.18FF` | `1000R` |
| `LunarLander.wav` | `0300.09FF` | `300R` |
| `Mastermind.wav` | `0300.03FF` | `300R` |
| `Pasart.wav` | `0300.05FF` | `300R` |
| `Life.wav` | `2000.21FF` | `2000R` (12 KB) |

Ihre eigenen Einstellungen werden nie überschrieben: Ein Programm, das Sie selbst eingerichtet haben, behält seine Einstellungen, nur die Markierung „erkannt“ erscheint. Programme, die über die grundlegenden 8 KB hinausgehen, erhalten beim Start automatisch den nötigen Speicher, wie mit einer Erweiterungskarte.

**Unbekannte Kassette.** Der Emulator errät, was er kann: Eine Aufnahme von 4096 Bytes, die mit `JMP` beginnt, ist Integer BASIC; zwei Aufnahmen, von denen die erste 182 Bytes lang ist (`$4A-$FF`), sind ein BASIC-Programm, das nach seinen Zeigern LOMEM und HIMEM platziert wird. Die Liste zeigt dann „(vermutet)“; prüfen Sie mit ⚙. Ein Programm in Maschinensprache ohne Adressen wird nicht erraten: Geben Sie seinen Bereich an.

![Eine echte Kassette digitalisieren: von der Kassette in eine Datei, dann in die Bibliothek.](diagrams/de/workflow-capture.svg "diagram large")

## GitHub-Projekte

**+ GitHub-Projekt** nimmt eine Adresse an (`https://github.com/jefftranter/6502/tree/master/asm/jmon`) oder einfach `Besitzer/Repository`. Die nützlichen Dateien werden nach `~/.cache/apple1-emu/github/<Besitzer>/<Repository>/<Commit>/` kopiert, und das Projekt wird ein Knoten des Baums, mit seinem Commit, Datum und seiner Nachricht. **⟳ Aktualisieren** holt den neuesten Commit; 🌐 öffnet die Seite des Repositorys. Der ganze Netzwerkverkehr läuft im Hintergrund.

{{Neuesten Build testen}} versucht der Reihe nach:

<ol class="steps">
<li>die Dateien der neuesten <strong>Release</strong> des Repositorys (<code>.wav</code>/<code>.aiff</code>, dann <code>.bin</code>, dann <code>.hex</code>/<code>.txt</code>);</li>
<li>das neueste <strong>Artefakt von GitHub Actions</strong> (ein Token ist nötig);</li>
<li><strong>das Bauen aus den Quelltexten</strong>: Ein BASIC-Listing wird ins Integer BASIC getippt und mit <code>RUN</code> gestartet; ein 6502-Quelltext wird mit <strong>ca65/ld65</strong> (Paket <code>cc65</code>) assembliert, standardmäßig mit <code>-D APPLE1</code>; ein Woz-Dump wird geladen.</li>
</ol>

![Ein GitHub-Projekt, von der Adresse des Repositorys bis zum laufenden Programm.](diagrams/de/workflow-github.svg "diagram large")

Eine Datei `apple1.toml` (oder `apple1.cfg`) im Wurzelverzeichnis des Repositorys sagt, was gebaut werden soll:

```toml title="apple1.toml"
file    = "src/game.s"    # zu startendes Programm
load    = $0300           # Ladeadresse (sonst das .org des Quelltexts)
run     = $0300           # Startadresse (sonst die Ladeadresse)
ld65    = "apple1.cfg"    # Konfiguration für ld65 (sonst -t none)
defines = "APPLE1"        # Symbole -D für ca65
```

!!! astuce "Das GitHub-Token ist freiwillig"
    Öffentliche Repositorys brauchen keines. Ein Token (Variable `GITHUB_TOKEN` oder **⚙ Einstellungen**) hebt die Anfragegrenze an und gibt Zugriff auf Ihre privaten Repositorys und auf Artefakte. Es wird nie angezeigt oder in Meldungen geschrieben, und `gui.json` ist dann nur noch für Ihr Konto lesbar.

## Der Ordner „Art ASCII“

Die Bibliothek enthält auch einen Ordner **Art ASCII**: Zeichnungen im Bildschirmformat (Apfel, Apple-1-Platine, 6502-Chip, Kassette, Rakete…) und Porträts von Steve Wozniak und Steve Jobs, umgewandelt aus frei lizenzierten Fotos. Eine ASCII-Art-Datei `.txt` zeigt beim Auswählen eine Vorschau, mit **→ BASIC**, **→ 6502** und **🎨 Im Werkzeug öffnen**; **▶** startet sofort ihre 6502-Fassung. Kapitel 9 beschreibt das Werkzeug.
