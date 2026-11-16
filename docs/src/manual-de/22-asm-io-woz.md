# Die Tastatur, die Anzeige und der Woz Monitor

<p class="lead">Der Apple-1 hat nur vier Ein-/Ausgaberegister, in einem PIA 6820 (oder 6821), und einen Monitor von 256 Bytes. Das ist nicht viel, aber es reicht: Dieses Kapitel zeigt, wie man eine Taste liest, ein Zeichen schreibt und die Routinen von Steve Wozniak benutzt.</p>

## Die vier Register des PIA

| Adresse | Name | Lesen | Schreiben |
|---|---|---|---|
| `$D010` | `KBD` | Tastencode, Bit 7 gesetzt; das Lesen **quittiert** die Taste | — |
| `$D011` | `KBDCR` | Bit 7 = 1: eine Taste wartet | Konfiguration (erledigt der Monitor) |
| `$D012` | `DSP` | Bit 7 = 1: die Anzeige ist beschäftigt | anzuzeigendes Zeichen (7 Bit) |
| `$D013` | `DSPCR` | — | Konfiguration (erledigt der Monitor) |

{{!KBD}}{{!KBDCR}}{{!DSP}}

Diese Namen sind in `apple1.inc` definiert, das alle Beispiele einbinden:

@@include exemples/de/asm/apple1.inc

## Eine Taste lesen

Warten, bis Bit 7 von `KBDCR` 1 wird, dann `KBD` lesen:

```asm
lesen:  lda KBDCR       ; eine Taste?
        bpl lesen       ; Bit 7 = 0: noch nicht
        lda KBD         ; ASCII-Code + $80
```

Die Tastatur liefert nur Großbuchstaben (der Emulator wandelt Kleinbuchstaben um), mit gesetztem Bit 7: `A` ergibt `$C1`, RETURN `$8D`, ESC `$9B`. Um zu wissen, ob eine Taste gedrückt wurde, **ohne zu warten**, prüfen Sie nur `KBDCR`:

```asm
        lda KBDCR
        bpl keine       ; keine Taste: weiterarbeiten
        lda KBD         ; eine Taste: lesen (und quittieren)
```

`jeu.s` (Kapitel 23) nutzt das, um eine Zufallszahl zu ziehen: Es zählt in einer Schleife, bis der Spieler eine Taste drückt.

## Ein Zeichen schreiben

Das Terminal des Apple-1 nimmt ein Zeichen nach dem anderen an, und nur, wenn es bereit ist: Bit 7 von `DSP` bleibt auf 1, bis das vorige Zeichen angezeigt ist. Die Routine `ECHO` des Monitors übernimmt das Warten für Sie. Hier ihre vier Befehle, im Monitor des Emulators gelesen:

```mon
\
» FFEF.FFFFR

FFEF: 2C
FFF0: 12 D0 30 FB 8D 12 D0 60
FFF8: 00 00 00 0F 00 FF 00 00
```

Das ist `BIT DSP` / `BMI ECHO` / `STA DSP` / `RTS`. `ECHO` schont A, X und Y. Da das Original-Terminal nur **ein Zeichen pro Bild** zeigt, etwa 60 pro Sekunde, ist ein Programm, das viel ausgibt, durch die Anzeige begrenzt, nicht durch den Prozessor; der Emulator hält diese Grenze standardmäßig ein (<kbd>F4</kbd> hebt sie auf, Kapitel 2).

Was die Anzeige mit jedem Code tut:

- **RETURN** (`$8D` oder `$0D`) beginnt eine neue Zeile; am unteren Bildschirmrand rollt alles eine Zeile nach oben.
- Die **übrigen Steuerzeichen** (`$00-$1F`, mit oder ohne Bit 7) werden ignoriert: kein Löschen, kein Zurück, keine Klingel.
- **Kleinbuchstaben** `$60-$7F` werden wie die Großbuchstaben `$40-$5F` angezeigt: Das Terminal behält nur 6 Bit.
- Nach der 40. Spalte geht der Cursor von selbst in die nächste Zeile.
- Es gibt keinen Code, um den Bildschirm zu löschen: Das ist eine Taste an der Maschine (**Bildschirm löschen**, <kbd>Strg</kbd>+<kbd>L</kbd>).

## Die Routinen des Woz Monitors

| Adresse | Name | Eingabe | Wirkung | Register |
|---|---|---|---|---|
| `$FFEF` | `ECHO` | A = Zeichen | zeigt es an | A, X, Y geschont |
| `$FFDC` | `PRBYTE` | A = Byte | zeigt es als 2 Hex-Ziffern | A zerstört; X, Y geschont |
| `$FFE5` | `PRHEX` | A (niederwertiges Nibble) | zeigt eine Hex-Ziffer | A zerstört; X, Y geschont |
| `$FF1F` | `GETLINE` | — | zurück zum Monitor: neue Zeile, dann Warten auf einen Befehl | — |
| `$FF1A` | `ESCAPE` | — | wie `GETLINE`, gibt aber zuerst `\` aus | — |
| `$FF00` | `RESET` | — | initialisiert den PIA und den Monitor | — |

{{!ECHO}}{{!PRBYTE}}{{!PRHEX}}{{!GETLINE}}

Der Monitor benutzt die Zero Page `$24-$2B` und den Zeilenpuffer `$0200-$027F`; alles andere gehört Ihnen (außer das Integer BASIC soll überleben, Kapitel 16). Seine Befehle (`300R`, `0300.031F`, `300: A9 8D` …) sind in Anhang B zusammengefasst.

## Ein Programm starten und beenden

Ein Programm wird mit dem Befehl `R` des Monitors gestartet (`300R`), der ein `JMP` zur Adresse macht, kein `JSR`. Es liegt also **keine Rücksprungadresse** auf dem Stapel: Beenden Sie mit `jmp GETLINE` (`$FF1F`), nicht mit `RTS`.

!!! attention "RTS und BRK geben die Kontrolle nicht zurück"
    Ein `RTS` am Ende eines mit `R` gestarteten Programms holt, was gerade auf dem Stapel liegt. Ein `BRK` springt über den Interruptvektor des ROMs, und der ist `$0000` (die letzten drei Wörter des Monitors, oben: NMI `$0F00`, RESET `$FF00`, IRQ/BRK `$0000`): Der Prozessor führt dann die Zero Page aus. In beiden Fällen drücken Sie RESET (<kbd>Strg</kbd>+<kbd>R</kbd>).

Ein Programm, das aus BASIC mit `CALL 768` aufgerufen wird, wird dagegen über ein `JSR` aufgerufen: Es muss mit `RTS` enden, um zu BASIC zurückzukehren. Kapitel 16 zeigt, wie man Maschinencode unter dem BASIC-Programm unterbringt.

```mon
>» POKE 768,96
>» CALL 768
>
```

(`96` = `$60`, der Opcode von `RTS`: das kleinstmögliche Unterprogramm.)

## Der PIA, die Kassette und die Farbkarte

Die ACI-Karte belegt `$C000-$C0FF` (jeder Zugriff schaltet den Ausgang um) und ihr Programm `$C100-$C1FF`; Kapitel 5 erklärt, wie sie funktioniert, und Anhang C ihre Zeiten. Die GEN2-Karte wird durch bloßes **Lesen** von `$C250-$C257` gesteuert (`bit SS_GR`); ihre Adressen stehen in `gen2.inc`:

@@include exemples/de/asm/gen2.inc
