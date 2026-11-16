# Tipps, Fallen und Grenzen

<p class="lead">Was man lernt, wenn man Programme für den Apple-1 schreibt, erst im Emulator und dann an der echten Maschine.</p>

## Wohin mit einem Programm

| Adresse | Für |
|---|---|
| `$0280` | so tief wie möglich: gleich nach dem Puffer des Monitors |
| `$0300` | die übliche Wahl (und die Standardadresse des Assemblers) |
| `$0C00` | mit der GEN2-Karte in LORES oder Text (Seiten `$0400-$0BFF` für die Anzeige frei gelassen) |
| unter HIMEM | mit dem Integer BASIC: den Platz reservieren (Kapitel 16) |

Für die Zero Page verwenden die Beispiele `$06-$09`, die der Monitor frei lässt. Muss das Programm neben BASIC bestehen, wählen Sie Adressen, die BASIC nicht benutzt, oder sichern und wiederherstellen Sie sie.

## Mit der Reichweite der Verzweigungen leben

Eine Verzweigung reicht nur 127 Bytes vorwärts oder 128 rückwärts. Antwortet der Assembler `Sprung zu weit`, kehren Sie die Bedingung um und springen über ein `JMP`:

```asm
        beq :+          ; statt: bne weit
        jmp weit
:
```

## Zeichenketten und Bit 7

Die Tastatur liefert Codes mit gesetztem Bit 7, und der Monitor vergleicht mit diesen Codes. Die Anzeige dagegen ignoriert Bit 7: `.asciiz "HALLO"` wird einwandfrei angezeigt. Um eine Taste zu **vergleichen**, setzen Sie Bit 7 in die Konstante: `cmp #'Q' | $80`. Kleinbuchstaben in einer Zeichenkette werden als Großbuchstaben angezeigt.

## Ein wenig warten

Bei 1,023 MHz (0,96 MHz effektiv) kostet eine Schleife `dex` / `bne` 5 Takte pro Durchlauf:

```asm
pause:
        ldy #78         ; 78 x 256 Durchläufe zu 5 Takten: etwa 0,1 s
:       ldx #0
:       dex
        bne :-
        dey
        bne :--
        rts
```

Der Emulator in Originalgeschwindigkeit hält diese Zeiten ein; im schnellen Modus (<kbd>F3</kbd>) verschwinden sie. Die Zeitschleifen der ACI werden ebenso in Takten gezählt (Anhang C).

## Wirksam debuggen

- Setzen Sie Haltepunkte auf eine **Marke** (`start`, `prdec`, `start@done`) statt auf eine Adresse: Das bleibt richtig, wenn sich der Code verschiebt.
- Überspringen Sie `jsr ECHO` mit **↪ Überspringen**: Sonst verfolgen Sie die Warteschleife der Anzeige Befehl für Befehl.
- Reagiert der Apple-1 nach Ihrem Programm nicht mehr, sehen Sie auf PC: In der Zero Page ist es fast immer ein verirrtes `BRK` oder `RTS` (Kapitel 22).
- Der **Speicherauszug** zeigt die Zero Page: Ihre Zeiger sind dort sichtbar, niederwertiges Byte zuerst.

![Der Debugger, angehalten in einem 6502-Quelltext: Register, Flags, Disassemblierung mit Marken und Speicher.](img/de/brut/debogueur-asm.png "large")

## Zum echten Apple-1

<ol class="steps">
<li>Im Emulator assemblieren und ausprobieren, <strong>in Originalgeschwindigkeit</strong> und mit auf 60 Zeichen pro Sekunde begrenzter Anzeige.</li>
<li>Die Kassette exportieren (<strong>📼 WAV exportieren</strong> oder <code>-o prog.aif</code>): AIFF ist das sicherste Format, wenn Sie vom Smartphone abspielen.</li>
<li>Am Apple-1: <code>C100R</code>, dann den angezeigten Bereich (<code>0300.0320R</code>), die Wiedergabe starten, dann <code>300R</code>.</li>
<li>Ohne ACI: Der Dump <code>.hex</code> lässt sich direkt in den Woz Monitor tippen (oder in ein serielles Terminal einfügen); seine letzte Zeile startet das Programm.</li>
</ol>

## Grenzen des eingebauten Assemblers

- Ein einziger Ausgabeblock in der Reihenfolge des Quelltexts: Segmente wirken nicht, und es gibt keinen Linker. Für Code, der auf mehrere Bereiche verteilt ist, nehmen Sie ca65 und ld65.
- Kein Modulo-Operator, kein `.proc`, `.scope`, `.struct` oder `.enum`: Das sind Direktiven von ca65, die der eingebaute Assembler nicht kennt.
- Befehle des 65C02 werden assembliert, aber der emulierte Prozessor ist ein NMOS-6502: Im Emulator funktionieren sie nicht (Kapitel 20).
- Im **Listing** verliert eine von einem Makro erzeugte Zeile manchmal das Leerzeichen zwischen Befehl und Operand (`ldastring, x` für `lda string,x`). Das ist nur ein Anzeigemangel des Listings: Die Bytes sind richtig.
