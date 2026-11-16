# Kommentierte Beispiele

<p class="lead">Acht Programme, vom einfachsten bis zum vollständigsten. Sie liegen im Ordner <code>docs/exemples/de/asm/</code> der Distribution; jedes wurde mit dem eingebauten Assembler assembliert und im Emulator ausgeführt. Die hier gezeigten Ausgaben sind die des Emulators.</p>

Zum Ausprobieren: Öffnen Sie die Datei im Editor und klicken Sie auf **▶ Starten**, oder auf der Befehlszeile `apple1-emu asm hello.s -o hello.wav`, dann die Kassette abspielen (Kapitel 5). Lassen Sie `apple1.inc` (und `gen2.inc`) im selben Ordner wie die Quelltexte.

| Datei | Bytes | Adresse | Was sie zeigt |
|---|---|---|---|
| `hello.s` | 33 | `$0300` | eine Zeichenkette, `ECHO`, lokale Marken, Rückkehr zum Monitor |
| `clavier.s` | 32 | `$0300` | die Tastatur lesen, Zeichen vergleichen |
| `hexa.s` | 64 | `$0300` | Zeiger in der Zero Page, `(zp),Y`, 16-Bit-Addition, `PRBYTE` |
| `decimal.s` | 117 | `$0300` | 16-Bit-Zahlen, Subtraktionen, Worttabellen |
| `jeu.s` | 242 | `$0300` | ein vollständiges Spiel: Zufall, Unterprogramme, Meldungen |
| `macros.s` | 136 | `$0300` | Makros, `.local`, `.repeat`, `.if`, berechnete Tabellen |
| `gen2-lores.s` | 110 | `$0C00` | die GEN2-Karte in LORES |
| `gen2-hires.s` | 94 | `$0C00` | die GEN2-Karte in HIRES |

Die Originalfassungen dieser Programme wurden auch mit ca65 assembliert (dieselben Bytes, Kapitel 21); die deutschen Fassungen unterscheiden sich nur in Kommentaren und Texten.

## `hello.s`: eine Meldung ausgeben

@@include exemples/de/asm/hello.s

X läuft die Zeichenkette bis zum Byte 0 entlang, das `.asciiz` gesetzt hat. Das `bne @next` nach `inx` ist bei einer Zeichenkette unter 256 Zeichen nie falsch: Es ist ein kurzer, sicherer Sprung. Das Programm endet mit `jmp GETLINE`, nie mit `RTS` (Kapitel 22).

```mon
\
» 300R
0300: A2
HALLO, APPLE-1!
```

Der Monitor gibt zuerst `0300: A2` aus, das Byte an der Startadresse, dann übernimmt das Programm.

## `clavier.s`: Tasten als Echo

@@include exemples/de/asm/clavier.s

`cmp #'0' | $80` vergleicht mit dem Code der Ziffer 0, mit gesetztem Bit 7, wie es die Tastatur liefert. `bcc` („kleiner“) und `bcs` („größer oder gleich“) rahmen die Ziffern ein. Mit den Tasten `A`, `B`, `1`, `2`, dann ESC zeigt der Bildschirm:

```text
0300: ADAB1122
\
```

Das `AD` kommt vom Monitor (`lda KBDCR` beginnt mit dem Opcode `$AD`); `AB`, dann `11` und `22` kommen vom Programm; ESC gibt die Kontrolle an den Monitor zurück, der `\` ausgibt.

## `hexa.s`: ein Dump wie im Monitor

@@include exemples/de/asm/hexa.s

`PTR` und `PTR+1` bilden einen **16-Bit-Zeiger** in der Zero Page, niederwertiges Byte zuerst; `lda (PTR),y` liest das Byte an der Adresse PTR + Y. Die Addition `PTR += 8` zeigt den Übertrag bei der Arbeit: `clc`, `adc #8`, und wenn ein Übertrag entsteht (`bcc` nicht genommen), `inc PTR+1`. Das Ergebnis ist der Anfang des Woz Monitors:

```text
FF00: D8 58 A0 7F 8C 12 D0 A9
FF08: A7 8D 11 D0 8D 13 D0 C9
```

## `decimal.s`: eine Zahl dezimal ausgeben

@@include exemples/de/asm/decimal.s

Der 6502 kann nicht dividieren: `prdec` zieht so oft wie möglich 10000 ab (die Anzahl ergibt die Zehntausenderziffer), dann 1000, 100 und 10. Die 16-Bit-Subtraktion geschieht in zwei Schritten, `sbc` auf das niederwertige, dann auf das höherwertige Byte, verbunden durch den Übertrag. `FLAG` unterdrückt führende Nullen. Ausgabe:

```text
0 7 42 1976 6502 65535
```

!!! astuce "X in einem Unterprogramm schonen"
    `prdec` benutzt X für sich, während die Hauptschleife es braucht: Es sichert X am Anfang mit `txa` / `pha` und stellt es am Ende mit `pla` / `tax` wieder her. Der NMOS-6502 hat kein `PHX`.

## `jeu.s`: die Ziffer raten

@@include exemples/de/asm/jeu.s

Der Zufall kommt vom Spieler: X zählt ununterbrochen (`@wait`) bis zur ersten Taste, dann ergibt der Rest der Division durch 10 die geheime Ziffer. `prmsg` erhält die Adresse einer Meldung in **Y:X** (höherwertiges Byte in Y), legt sie im Zeiger `MSG` ab und gibt sie mit `(MSG),y` aus. Eine Partie, mit `x` und dann den Ziffern der Reihe nach getippt:

```text
ICH HABE EINE ZIFFER. TASTE DRUECKEN.
DEINE ZIFFER? 0 HOEHER.
DEINE ZIFFER? 1 HOEHER.
…
DEINE ZIFFER? 5 HOEHER.
DEINE ZIFFER? 6 GEWONNEN IN 7 VERSUCHEN!

```

Die letzte Zeile ist genau 40 Zeichen lang: Der Cursor springt von selbst in die nächste Zeile, und der RETURN der Meldung fügt eine Leerzeile hinzu (Kapitel 22).

## `macros.s`: den Assembler arbeiten lassen

@@include exemples/de/asm/macros.s

`putc` und `print` erzeugen bei jedem Aufruf Code; `.local` verhindert, dass die zwei Aufrufe von `print` zweimal `loop` definieren. `.repeat 3` entrollt die Sternzeile dreimal (jede mit ihrer eigenen anonymen Marke), `.if` wählt die Meldung nach `WIDTH`, und die Tabelle `squares` wird bei der Assemblierung berechnet. Ausgabe:

```text
MAKROS UND .REPEAT
********************
********************
********************
KURZE ZEILEN
```

Ändern Sie `WIDTH = 20` in `40`: Der Editor assembliert sofort neu, die Zeilen werden länger, und die Meldung wird zu `LANGE ZEILEN`.

## `gen2-lores.s`: Farbe in niedriger Auflösung

Diese beiden Programme brauchen die GEN2-Karte (**⚙ Einstellungen ▸ Uncle-Bernie-GEN2-Farbkarte**, Kapitel 6) und beginnen bei `$0C00`, über der LORES-Seite.

@@include exemples/de/asm/gen2-lores.s

Die beiden Tabellen `rows_lo` und `rows_hi` werden von `.repeat` mit der Formel der Zeilenadressen aus Kapitel 6 berechnet. Jedes Byte trägt zwei Blöcke, daher das in die obere und untere Hälfte kopierte Nibble. Die Befehle `bit SS_…` sind **Lesezugriffe**: So werden die Modi der Karte umgeschaltet.

![gen2-lores.s: die sechzehn LORES-Farben, je zwei Spalten, auf dem Farbmonitor.](img/de/brut/gen2-lores.png "large")

## `gen2-hires.s`: hohe Auflösung

@@include exemples/de/asm/gen2-hires.s

In HIRES ist die Seite `$2000-$3FFF` in Blöcke von 128 Bytes geteilt, deren erste 40 Bytes ins obere Bildschirmdrittel gehen, die nächsten 40 in die Mitte und die 40 danach nach unten. Das Programm füllt jedes Drittel mit einem anderen Muster und wechselt je nach Parität der Spalte zwischen zwei Bytes: Das Ergebnis sind drei Farbstreifen.

![gen2-hires.s: drei Farbstreifen in HIRES: violett, grün und blau.](img/de/brut/gen2-hires.png "large")
