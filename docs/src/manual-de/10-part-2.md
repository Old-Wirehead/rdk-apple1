# Integer BASIC <span class="part-sub">Steve Wozniaks BASIC und sein freier Klon RDK G2 Basic: alle Anweisungen, der Speicher, die Fehlermeldungen, die bekannten Fehler und ihre Umgehungen, und die Portierung aus Applesoft</span> {: .part data-part="Teil II" }

# Erste Schritte in BASIC

<p class="lead">Das Integer BASIC des Apple-1 passt in 4 KB. Es kennt nur ganze Zahlen und Variablennamen aus einem Buchstaben und hat weder DATA noch Zeichenkettenfunktionen: Es ist klein, schnell und hat seine Fallen.</p>

!!! apple1 "Woher dieses BASIC kommt"
    Steve Wozniak schrieb es 1976 von Hand in Assembler, ohne einen Computer, um es zu assemblieren. Es wurde auf Kassette verkauft (75 $) und lädt nach `$E000-$EFFF`. Der Emulator liefert Apples Original nicht mit: Verwenden Sie Ihr eigenes `BASIC.wav` oder 4-KB-Abbild (Kapitel 4), wenn Sie es möchten. Jeder Test in diesem Teil wurde im Emulator mit diesem Original gemacht.

## Zwei BASICs: RDK G2 Basic und das Original

Seit Version 7.9 hat der Emulator sein eigenes BASIC, **RDK G2 Basic**, ins Programm eingebaut. Es ist ein Clean-Room-Klon des Integer BASIC, von Grund auf neu in 6502-Assembler geschrieben (es enthält keinen Code von Apples Interpreter) und unter der MIT-Lizenz verbreitet. Es ist Uncle Bernie gewidmet.

- **Kompatibel**: gleiche Syntax, gleiche Tokens, gleiches Programmformat im Speicher, gleiches Kassettenformat. Programme, Listings und Kassetten für das Original laufen unverändert. Es wurde mit 22 Testsitzungen mit identischer Ausgabe gegen das Original geprüft.
- **Berichtigt**: Die bekannten Fehler des Originals, beschrieben in Kapitel 18, sind behoben.
- **Größer**: 7933 Bytes statt 4 KB. Es braucht RAM von `$E000` bis `$FEFF`; der Emulator stellt ihn bereit, aber es würde nicht in das 4-KB-EPROM-Fenster des Originals passen.
- **Kennt die GEN2**: Beim Start sucht es die GEN2-Karte und startet, wenn es sie findet, den Kartentreiber (und setzt dann LOMEM auf `$2000`, über die Grafikseiten). Die Grafikanweisungen selbst sind für Version 2 von G2 Basic geplant.
- **Begrüßung**: `RDK G2 BASIC 1.0`, dann `DEDICATED TO UNCLE BERNIE`.
- **Erweitert** (Version 7.9.2): `LOMEM=` und `HIMEM=` funktionieren wie im Original, und eine neue Funktion **`KEY(n)`** liest die Tastatur, ohne zu warten, damit ein Spiel die Tasten abfragen kann, ohne dass `INPUT` es blockiert und ohne dass ein Tastendruck das Programm anhält (Kapitel 15). `LIST a,b` und `DEL a,b` verwenden die Bereiche des Originals, und eine gelistete Zeile, die länger als der Bildschirm ist, wird wie beim Original umbrochen.

Die Schaltfläche **BASIC** lädt G2 Basic. Um stattdessen Apples Original zu verwenden, öffnen Sie **⚙ Einstellungen → Erweitert → BASIC**, wählen **Original-Integer-BASIC (Ihre eigene Datei, nach $E000 geladen)** und die Datei; die Wahl wird gespeichert, und die Schaltfläche lädt sie dann. Für alles, was in diesem Teil beschrieben ist, verhalten sich beide gleich; die Unterschiede sind dort vermerkt, wo sie zählen (HIMEM beim Start, die Fehler aus Kapitel 18).

!!! note "Quelltext und Tests"
    G2 Basic hat ein eigenes Repository, `Old-Wirehead/rdk-gen2basic-apple1`, mit seinem Handbuch, seinem Quelltext und seinen Regressionstests.

## BASIC starten

Klicken Sie in der Werkzeugleiste auf **BASIC**: Der Emulator lädt das gewählte BASIC (standardmäßig G2 Basic), und Sie starten es mit `E000R`. Die Eingabeaufforderung von BASIC ist `>`, die des Woz Monitors `\`.

```mon
\
» E000R
E000: 4C
>
```

- `E000R`: **Kaltstart**, löscht das Programm im Speicher.
- `E2B3R`: **Warmstart**, behält das Programm. Diese Adresse brauchen Sie nach dem Laden eines Programms von Kassette und um nach einem Abstecher in den Monitor (RESET) zu BASIC zurückzukehren.

{{E000R}}{{E2B3R}}

## Direktmodus und Programmmodus

Eine Zeile, die mit einer **Zahl** (0 bis 32767) beginnt, wird im Programm gespeichert; eine Zeile ohne Zahl wird sofort ausgeführt.

```mon
>» PRINT 2*3+4*5
26
>» 10 PRINT "HALLO"
>» 20 GOTO 10
>» RUN
HALLO
HALLO
…
```

Drücken Sie eine Taste, um das Programm anzuhalten: BASIC gibt `STOPPED AT 10` aus. Eine Zeile mit derselben Nummer erneut zu tippen, ersetzt sie; eine Nummer allein löscht sie.

!!! attention "Der Direktmodus ist eingeschränkt"
    Im Direktmodus lehnt BASIC durch `:` getrennte Anweisungen ab (`SYNTAX ERR`), ebenso `FOR` und `GOSUB`. Um eine Schleife auszuprobieren, schreiben Sie sie in eine nummerierte Zeile.

## Eine Zeile berichtigen

Die Tastatur des Apple-1 hat keine Löschtaste: Die **Rücktaste** des Emulators tippt `_`, das das vorige Zeichen aufhebt (die Zeile auf dem Bildschirm wird nicht berichtigt, aber BASIC erhält die richtige). <kbd>Esc</kbd> verwirft die ganze Zeile: BASIC gibt `\` aus und wartet auf die nächste Zeile. Bequemer bearbeiten Sie im Editor (Kapitel 7): Er färbt die Syntax, meldet Fehler vor dem Senden und tippt das Programm für Sie.

## Ein erstes Programm

@@include exemples/de/basic/boucle.bas

```mon
>» RUN
SUMME = 55
>
```

Im Editor tippt **▶ Starten** das Programm und `RUN`; im Debugger können Sie es Zeile für Zeile verfolgen (Abbildung in Kapitel 8).

## Befehle

| Befehl | Wirkung |
|---|---|
| {{LIST}} `[n[,m]]` | listet das Programm, die Zeile `n` oder die Zeilen `n` bis `m` |
| {{RUN}} `[n]` | löscht die Variablen und startet das Programm, ab Zeile `n`, falls angegeben |
| {{DEL}} `n[,m]` | löscht die Zeile `n` oder die Zeilen `n` bis `m` |
| {{SCR}} | löscht das Programm und die Variablen |
| {{CLR}} | löscht die Variablen, behält das Programm |
| {{AUTO}} `n[,schritt]` | automatische Nummerierung ab `n` (standardmäßig Schritt 10); <kbd>Strg</kbd>+<kbd>D</kbd>, dann <kbd>RETURN</kbd> beendet sie, <kbd>Esc</kbd> nicht |
| {{MAN}} | zurück zur Nummerierung von Hand |
| {{HIMEM}}`=n`, {{LOMEM}}`=n` | die Grenzen des BASIC-Speichers (Kapitel 16) |

`GOTO n` im Direktmodus setzt ein angehaltenes Programm fort, **ohne** die Variablen zu löschen, anders als `RUN n`.

!!! astuce "LIST und die langsame Anzeige"
    Das Terminal des Apple-1 zeigt etwa 60 Zeichen pro Sekunde: Ein langes Programm zu listen, dauert. Schalten Sie die Anzeigegrenze aus (**Schnell** in der Anzeigezeile oder in der Statusleiste), um schneller zu lesen, oder listen Sie in Stücken: `LIST 100,200`.

## Drei Programme zum Abtippen

### Eine Zahl raten

@@include exemples/de/basic/devine.bas

```mon
>» RUN
ICH HABE EINE ZAHL VON 1 BIS 100.
DEIN TIPP?» 50
HOEHER!
DEIN TIPP?» 75
NIEDRIGER!
DEIN TIPP?» 62
NIEDRIGER!
DEIN TIPP?» 56
HOEHER!
DEIN TIPP?» 57
BRAVO, IN 5 VERSUCHEN.
```

`RND(100)` ergibt eine ganze Zahl von 0 bis 99; `#` bedeutet „ungleich“.

### Ein Einmaleins

@@include exemples/de/basic/table.bas

### Mit Zeichenketten spielen

@@include exemples/de/basic/chaines.bas

```mon
>» RUN
DEIN NAME?» OTTO
ANFANG: O
ENDE: TO
RUECKWAERTS: OTTO
EIN PALINDROM!
```

Zeichenketten werden mit `N$(anfang,ende)` zerlegt und mit `R$(LEN(R$)+1)=…` verlängert: Es gibt weder `MID$` noch `+` für Zeichenketten (Kapitel 16).

Die vier Programme liegen in `docs/exemples/de/basic/`: Öffnen Sie sie im Editor, oder machen Sie eine Kassette mit `apple1-emu basic devine.bas -o devine.aiff`.
