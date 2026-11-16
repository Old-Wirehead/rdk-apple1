# Die Kassette und die ACI-Karte

<p class="lead">Der Apple-1 hatte weder Diskette noch BASIC im ROM: Alles lief über eine Audiokassette und die ACI-Karte (Apple Cassette Interface). Der Emulator führt das echte ACI-ROM aus, daher laden seine WAV- und AIFF-Dateien auf einem echten Apple-1, und Aufnahmen von einem echten Apple-1 laden im Emulator.</p>

## Dateiformate

| Endung | Inhalt | Lesen | Schreiben |
|---|---|---|---|
| `.wav` | ACI-Kassette, PCM mit 8 bis 32 Bit oder Gleitkomma, mono oder stereo, beliebige Abtastrate | ja | standardmäßig 44,1 kHz mono, 16 Bit |
| `.aif`, `.aiff` | ACI-Kassette (AIFF und AIFF-C) | ja | Smartphone-Profil: 48 kHz, 2 identische Kanäle, 16 Bit |
| `.txt`, `.hex`, `.apl` | Dump des Woz Monitors: Zeilen `0300: A9 C1 20 …` | ja | ja |
| `.bin` | rohe Bytes, ohne Adresse (`DATEI.bin@300`) | ja | ja |
| `.bas` | Integer-BASIC-Listing (Text) | im Editor | im Editor |
| `.s`, `.asm`, `.a65` | 6502-Quelltext (ca65 oder xa) | im Editor | im Editor |

Vom Emulator exportierte Kassetten tragen ihre Adressen in einem Kommentarblock der Datei (`APPLE1 ACI 0300.031E`), den Audioabspieler übergehen. Eine Kassette anderer Herkunft hat keine Adressen: Die Bibliothek erkennt oder errät sie (Kapitel 4), und die Befehlszeile nimmt `--addr` oder `--range` an.

Der Decoder verträgt Rauschen, schwache Pegel, vertauschte Polarität, einen Geschwindigkeitsfehler des Bandes von ±10 % und mehrere Aufnahmen in einer Datei. Auch WAV-Dateien des Werkzeugs **c2t** von Egan Ford werden gelesen. Ein **Netzbrummen** (50 oder 60 Hz, selbst so stark wie das Kassettensignal) wird vor dem Erkennen der Flanken herausgefiltert, so dass auch eine lange Aufnahme mit schlecht abgeschirmtem Kabel richtig gelesen wird.

## Das virtuelle Deck

Der Streifen **Kassette:** der Werkzeugleiste vereint alle Bedienelemente: **📼 Einlegen**, **▶** Wiedergabe, **⏹** Stopp, **⏪** Zurückspulen, **⏺** Aufnahme, **⏏** Auswerfen, dann Name und Position der Kassette, das Oszilloskop **📈** und der Ton **🔊**. Eine Kassette wird mit **📼 Einlegen** eingelegt (eine `.wav` oder `.aiff`) oder per Rechtsklick auf ein Programm der Bibliothek („Als Kassette einlegen“): Dann tippen Sie `C100R` und den Bereich selbst. Das Symbol **📼** eines Programms der Bibliothek geht einen Schritt weiter: Es legt die Kassette ein und tippt die Befehle für Sie.

![Der Streifen Kassette: beim Lesen von BASIC.wav: 7,6 von 35,8 Sekunden.](img/de/barre-cassette.png "large")

Die Kassette folgt der **emulierten** Zeit: Bei schneller Geschwindigkeit (<kbd>F3</kbd>) dauert das Laden weit weniger als der 10 Sekunden lange Vorspann, und es funktioniert trotzdem. Ohne Kassette geht das Laden aus der Bibliothek sofort (Kapitel 4).

Integer BASIC „auf die alte Art“ zu laden sieht so aus:

```mon
» C100R
C100: A9*
» E000.EFFFR
» E000R

E000: 4C
>
```

`C100R` startet das Programm der ACI, das mit seiner Eingabeaufforderung `*` antwortet. Die Zeile `E000.EFFFR` bittet es, die Kassette nach `$E000-$EFFF` zu **lesen**. Am Ende der Kassette kehrt die ACI zum Monitor zurück (`\`); `E000R` startet das BASIC, dessen Eingabeaufforderung `>` ist.

## Ton und Oszilloskop

![Das Oszilloskop beim Abspielen von BASIC.wav: das Kästchen „Kassettenton“, die Lautstärke und das Signal mit 1 ms pro Teilung.](img/de/oscilloscope.png "small")

Während der Wiedergabe oder Aufnahme lässt **🔊** Sie das Signal hören, wie den Lautsprecher eines Kassettenrekorders, standardmäßig leise. **📈** öffnet das **Oszilloskop**: eine grüne Spur auf dunklem Raster, 1 ms pro Teilung, mit 10 ms um den Tonkopf oder den letzten 10 Millisekunden, die der Apple-1 geschrieben hat. Man erkennt den Vorspann (≈ 800 Hz), die 0-Bits (≈ 2 kHz) und die 1-Bits (≈ 1 kHz). Bei schneller Geschwindigkeit ist der Ton stumm.

## Aufnehmen vom emulierten Apple-1

Um wie 1976 eine Kassette herzustellen, schalten Sie die Aufnahme scharf und bitten dann die ACI zu schreiben:

<ol class="steps">
<li>Klicken Sie auf <strong>⏺</strong> und wählen Sie die Datei (<code>.wav</code> oder <code>.aiff</code>).</li>
<li>Tippen Sie <code>C100R</code>, dann den Bereich, gefolgt von <code>W</code>, zum Beispiel <code>0300.03FFW</code>, dann <kbd>RETURN</kbd>.</li>
<li>Die ACI schreibt einen 10 Sekunden langen Vorspann, dann die Daten. Die Datei wird 1,5 s nach dem Ende geschrieben oder bei <strong>⏹</strong>.</li>
</ol>

Am einfachsten bleibt **Sichern** in der Werkzeugleiste oder **📼 WAV exportieren** im Editor: Das Signal ist taktgenau dasselbe und wird sofort erzeugt.

## Wie die ACI funktioniert

![Die zwei Richtungen der ACI: Beim Schreiben schaltet jeder Zugriff auf $C0xx den Ausgang um; beim Lesen steuert der Komparator die Adressleitung A0 des PROMs.](diagrams/de/aci.svg "diagram large")

Die Karte ist bemerkenswert einfach: ein PROM von 256 Bytes, sichtbar bei `$C000` und `$C100`, ein Flipflop und ein Komparator.

- **Schreiben.** Jeder Zugriff auf eine Adresse in `$C000-$C0FF` schaltet den Ausgang um (74LS74). Das ROM-Programm bestimmt die Zeit zwischen zwei Zugriffen: Diese Zeit codiert die Bits.
- **Lesen.** Der Pegel des Kassetteneingangs steuert nach dem Komparator LM311 die Adressleitung A0 des PROMs, wenn A7 = 1 ist. Derselbe Befehl liest daher je nach Pegel `$C080` oder `$C081`: Das ROM misst die Halbperioden mit einer Schleife von 12 Takten.
- **Codierung.** Eine „1“ dauert etwa doppelt so lange wie eine „0“: 1 kHz für eine 1, 2 kHz für eine 0. Die Bytes gehen mit dem höchstwertigen Bit zuerst hinaus, ohne Parität und ohne Prüfsumme.

Anhang C gibt die genauen Zeiten in Takten und Mikrosekunden an.

## Befehle der ACI

Nach `C100R` wartet die ACI auf eine Zeile im Format des Monitors, die mit `R` (Lesen) oder `W` (Schreiben) endet:

| Zeile | Wirkung |
|---|---|
| `0300.03FFR` | liest die Kassette nach `$0300-$03FF` |
| `0300.03FFW` | schreibt `$0300-$03FF` auf Kassette, mit 10 s Vorspann |
| `004A.00FFR 0300.0FFFR` | liest **zwei** aufeinanderfolgende Aufnahmen: die Zero Page des BASIC, dann das Programm |
| `004A.00FFW800.FFFW` | die zusammengeschriebene Form aus dem BASIC-Handbuch von 1976, ebenfalls angenommen |

{{!C100R}}{{!R (Lesen)}}{{!W (Schreiben)}}

!!! attention "Die Nullen von 004A.00FF sind Pflicht"
    Der Parser des ROMs löscht seinen Hex-Akkumulator am Punkt nicht („Doesn’t clear HEX1!!!“, sagt das Listing). Die Endziffern schieben sich hinter die des Anfangs: `4A.FF` bedeutet `$004A-$4AFF`, fast 19 KB! `0300.0FFF` funktioniert, weil die vier Ziffern die 3 von `0300` hinausschieben. Schreiben Sie beide Adressen immer vierstellig. Der Emulator gibt dieses Verhalten wieder, da er das echte ROM ausführt.

Ein BASIC-Programm wird als zwei Aufnahmen gespeichert: die Zero Page `$004A-$00FF` (sie enthält die Zeiger LOMEM und HIMEM und das Ende des Programms), dann das Programm selbst, das immer bei HIMEM endet. Zum Zurückladen muss das BASIC schon im Speicher sein; dann kehren Sie mit `E2B3R` „warm“ ins BASIC zurück, was das Programm behält.

```mon
» C100R
C100: A9*
» 004A.00FFR 0300.0FFFR
» E2B3R

E2B3: 20
» >RUN
```

## Auf einen echten Apple-1

Das ist es, was den Emulator rechtfertigt: ein Programm am PC vorbereiten und es dann von einem Smartphone oder dem Kopfhörerausgang eines Computers in die ACI einer echten Maschine einspielen.

<ol class="steps">
<li><strong>Bereiten Sie die Datei vor.</strong> <strong>📼 WAV exportieren</strong> im Editor oder <code>apple1-emu export prog.txt -o prog.aiff</code>. Wählen Sie <code>.aiff</code> für ein Smartphone. Notieren Sie die angezeigte Zeile, zum Beispiel <code>C100R</code>, dann <code>0300.031ER</code>.</li>
<li><strong>Richten Sie den Abspieler ein.</strong> Lautstärke auf Maximum (die Warnung „hohe Lautstärke“ bestätigen), alle Audioeffekte aus (Equalizer, Dolby, „Audio-Verbesserung“), Flugmodus, kein Bluetooth. Ein einfacher Abspieler wie VLC genügt.</li>
<li><strong>Am Apple-1</strong> tippen Sie <code>C100R</code>: Der Monitor antwortet <code>C100: A9*</code>.</li>
<li>Tippen Sie die Lesezeile, zum Beispiel <code>0300.031ER</code>, drücken Sie aber <strong>noch nicht RETURN</strong>.</li>
<li>Starten Sie die Wiedergabe, dann drücken Sie <kbd>RETURN</kbd> <strong>während des Vorspanns</strong> (er dauert etwa 10 s, nach einer Sekunde Stille).</li>
<li>Am Ende kehrt die ACI zum Monitor zurück (<code>\</code>). Starten Sie das Programm: <code>300R</code>, oder <code>E2B3R</code>, dann <code>RUN</code> bei einem BASIC-Programm.</li>
</ol>

**Warum dieses AIFF-Profil?** Unkomprimiert mit 16 Bit, kein verlustbehafteter Codec: MP3 oder AAC würden die Flanken abrunden. 48 kHz, die native Rate von Android, um jedes Umrechnen zu vermeiden. Zwei streng identische Kanäle, denn ein Kabel, das links und rechts an FROM TAPE zusammenschaltet, darf nie zwei verschiedene Signale erhalten. Ein Rechteck mit −1 dBFS: Der Eingang der originalen ACI ist ein Hochpass (0,01 µF), der kaum mehr als die Flanken sieht, und ein Rechteck mit steilen Flanken passt am besten. Eine Sekunde Stille am Anfang und am Ende, weil ein Telefon manchmal den Anfang eines Tons abschneidet.

!!! apple1 "Verlassen Sie sich nicht auf die LED"
    Das Handbuch von 1976 verlangt einen Pegel, bei dem die LED der ACI „gerade leuchtet“. Doch sie leuchtet erst bei etwa 1,2 V, weit mehr, als der Komparator braucht. Eine dunkle LED beweist also nicht, dass das Signal zu schwach ist. Endet das Laden nie, wartet das ROM noch auf Flanken: Drücken Sie RESET und drehen Sie die Lautstärke auf.

!!! attention "Niemals MP3 oder AAC"
    Ein verlustbehaftetes Format verschiebt die Flanken und macht die Kassette unlesbar. Verwenden Sie WAV, AIFF oder FLAC, wenn Ihr Abspieler es decodiert.

## Welcher Pegel ist nötig? Die Eingangsmodelle

Der Befehl `info --aci-input` simuliert den analogen Eingang der Karte (Koppelkondensator, Vorspannung, Hysterese des LM311) für eine Reihe von Spitzenspannungen und sagt Ihnen, ab welchem Pegel die Datei gelesen würde:

```sh
apple1-emu info prog.aiff --aci-input stock     # Karte von 1976, 0,01 µF
apple1-emu info prog.aiff --aci-input mimeo     # 0,1 µF (Rat des Mimeo-Leitfadens)
apple1-emu info prog.aiff --aci-input gen2      # ACI Gen2 von Uncle Bernie
apple1-emu info prog.aiff --aci-input stock --volts 0.5 --series 1200
```

| Modell | Was es darstellt |
|---|---|
| `stock` | die originale ACI: 0,01 µF in den auf 2,5 V vorgespannten Komparator, Rückkopplung 3k/47k |
| `mimeo` | die Änderung von Mike Willegal: 0,1 µF, zuverlässiger |
| `gen2` | die ACI Gen2 von Uncle Bernie: ausgesuchte 100 nF, 300k statt 47k (weniger Hysterese) |

`--volts` legt die Spitzenspannung an der Buchse für digitale Vollaussteuerung fest; `--series` fügt einen Serienwiderstand hinzu (zum Beispiel die 1,2 kΩ eines Smartphone-Kabels). Mit dem Modell `stock` werden die Dateien des Emulators ab **etwa 0,25 V Spitze** gelesen; ein Smartphone-Kopfhörerausgang liefert bei Maximum meist 0,5 bis 1 V. Dieses Modell ist vereinfacht: kein Rauschen, keine Drift des Arbeitspunkts.

## Eine echte Kassette oder eine Apple-1-Aufnahme digitalisieren

Um ein auf einem echten Apple-1 getipptes Programm zurückzuholen, nehmen Sie den Ausgang **TO TAPE** der ACI über einen Mikrofoneingang auf (ein Telefon oder eine PC-Soundkarte mit Audacity bei 44,1 oder 48 kHz), mit ausgeschalteter automatischer Verstärkung und Rauschunterdrückung. Am Apple-1: `C100R`, dann `0300.03FFW` und <kbd>RETURN</kbd>. Hören Sie einige Sekunden nach der Rückkehr des `\` auf, dann:

```sh
apple1-emu import aufnahme.aiff --addr 300 -o prog.txt
apple1-emu import startrek.aiff --range "004A.00FFR 0300.0FFFR" -o startrek.txt
```

!!! apple1 "TO TAPE ist ein Ausgang mit Mikrofonpegel"
    Er liefert nur etwa 50 mV Spitze-Spitze. Prüfen Sie die Wellenform in Audacity: keine Übersteuerung, kein Pegelsprung während des Vorspanns. Der Import des Emulators verträgt jede Verstärkung und einen Gleichspannungsversatz.
