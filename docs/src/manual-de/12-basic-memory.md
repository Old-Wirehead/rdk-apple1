# Variablen, Zeichenketten, Felder und Speicher

## Zahlen

Das Integer BASIC kennt nur **vorzeichenbehaftete 16-Bit-Ganzzahlen**, von −32767 bis 32767. Eine Rechnung, die diesen Bereich verlässt, hält mit `*** >32767 ERR` an (`200*200`, `32767+1`, ebenso `10/0` und `RND(0)`). Der Wert −32768 ist nicht verwendbar (Kapitel 18, Fehler 5). Es gibt kein Dezimalkomma und kein `INT`: `7/2` ist 3.

## Variablennamen

Das ist die Grenze, die am meisten überrascht: Ein Variablenname ist **ein Buchstabe, wahlweise gefolgt von einer Ziffer**.

| Name | Ergebnis (ausprobiert) |
|---|---|
| `A`, `Z`, `B1`, `Z9` | angenommen |
| `AB`, `CNT`, `SCORE`, `COUNTER` | `SYNTAX ERR` |
| `A12` | `SYNTAX ERR` |
| `A$`, `N$` | Zeichenkette: nur **ein Buchstabe** |
| `B9$`, `Z9$` | `SYNTAX ERR` |

Das ergibt 26 × 11 = 286 mögliche Ganzzahlvariablen und 26 Zeichenketten. Gewöhnen Sie sich an, die Rolle jeder Variablen in einem `REM` zu notieren.

## Zeichenketten

Eine Zeichenkette muss vor der Verwendung **dimensioniert** werden, mit ihrer Höchstlänge (1 bis 255): `DIM N$(20)`. `DIM B$(256)` ergibt `>255 ERR`.

| Vorgang | Schreibweise | Beispiel (N$ = "OTTO") |
|---|---|---|
| Länge | `LEN(N$)` | 4 |
| Teilkette | `N$(i,j)`: Zeichen i bis j | `N$(1,3)` = "OTT" |
| Ende | `N$(i)`: von Zeichen i bis zum Ende | `N$(3)` = "TO" |
| Verketten | `A$(LEN(A$)+1)=B$` | "APPLE" + "-1" → "APPLE-1" |
| Vergleichen | nur `=` und `#` | `IF N$="WOZ" THEN …` |

Eine Zeichenkette zuzuweisen, die länger als ihr `DIM` ist, ergibt `*** STR OVFL ERR`, ebenso eine zu lange Antwort auf `INPUT`. Es gibt kein `MID$`, `LEFT$`, `CHR$`, `STR$`, `VAL` oder `ASC` (Letzteres steht in der Tabelle, funktioniert aber nicht).

!!! astuce "CHR$ und VAL ersetzen"
    Um ein Zeichen aus seinem Code auszugeben, halten Sie eine Bezugskette vor und schneiden sie zu: `DIM C$(26):C$="ABCDEFGHIJKLMNOPQRSTUVWXYZ"`, dann `PRINT C$(K,K)`. Um eine Zahl zu lesen, nehmen Sie `INPUT N` statt `INPUT N$`.

## Felder

`DIM T(10)` legt ein Feld aus 10 ganzen Zahlen an, **nummeriert von 1 bis 10**. `T(0)` und `T(11)` ergeben `*** RANGE ERR`. Es gibt nur eine Dimension: Für ein Gitter von 8 × 8 nehmen Sie `G((L-1)*8+C)`. Im Emulator sind die Elemente nach `DIM` 0; an einer echten Maschine wird der Speicher nicht unbedingt gelöscht: Initialisieren Sie Ihre Felder. Ein Feld, das zu groß für den Speicher ist, ergibt `*** MEM FULL ERR` (`DIM A(3000)` in 8 KB).

## Der Speicher von BASIC

![Speicher des Apple-1: BASIC bei $E000, das Programm unter HIMEM, die Variablen über LOMEM.](diagrams/de/memoire.svg "diagram large")

BASIC legt das **Programm oben** in den Speicher, von HIMEM abwärts wachsend, und die **Variablen unten**, von LOMEM aufwärts wachsend. Der freie Platz liegt dazwischen.

| Zeiger | Adresse | Wert bei 8 KB | Rolle |
|---|---|---|---|
| LOMEM | `$4A-$4B` (74-75) | `$0800` (2048) | Anfang der Variablen |
| HIMEM | `$4C-$4D` (76-77) | `$1000` (4096) | Ende des BASIC-Speichers |
| PP | `$CA-$CB` (202-203) | sinkt mit dem Programm | Anfang des Programms |
| PV | `$CC-$CD` (204-205) | steigt mit den Variablen | Ende der Variablen |

{{!LOMEM}}{{!HIMEM}}{{!PP (Programmzeiger)}}

`PRINT PEEK(202)+PEEK(203)*256` ergibt den Anfang des Programms und `PRINT PEEK(204)+PEEK(205)*256` das Ende der Variablen: Der freie Platz ist die Differenz der beiden. Eine Ganzzahlvariable wie `A` belegt 6 Bytes.

!!! apple1 "Warum eine BASIC-Kassette bei $004A beginnt"
    Diese Zeiger liegen in der Zero Page, von `$4A` bis `$FF`. Eine BASIC-Kassette zeichnet daher **zwei Bereiche** auf: `004A.00FF` (die Zeiger), dann das Programm von PP bis HIMEM, zum Beispiel `0F2C.0FFF`. Der Befehl `apple1-emu basic` berechnet sie für Sie (seine Ausgabe ist französisch: „10 Zeilen, 212 Bytes … 1836 Bytes frei für die Variablen … ACI-Bereiche (nach C100R) … dann E2B3R und RUN“):

    ```text
    10 lignes, 212 octets ; LOMEM $0800 HIMEM $1000 PP $0F2C ; 1836 octets libres pour les variables ; RAM 8 Ko
    Plages ACI (après C100R) : 004A.00FFR 0F2C.0FFFR   puis E2B3R et RUN
    ```

### LOMEM und HIMEM ändern

Die Schreibweise ist `LOMEM=n` und `HIMEM=n` (mit `=`, **nicht** `:` wie beim Apple II, was `SYNTAX ERR` ergibt).

- `LOMEM=2304` verschiebt den Anfang der Variablen und **löscht die Variablen** (das Programm bleibt). Nützlich, um `$0800-$08FF` für ein Maschinenprogramm zu reservieren.
- `HIMEM=` **höher** (mit mehr RAM, zum Beispiel `HIMEM=8192` bei 16 KB) funktioniert und behält das Programm.
- `HIMEM=` **niedriger** wird vom Original **stillschweigend ignoriert**, selbst nach `SCR` (Kapitel 18, Fehler 11). **RDK G2 Basic nimmt es an**: Das Programm im Speicher wird mit der Grenze nach unten verschoben, oder es kommt `MEM FULL ERR`, wenn es nicht mehr über die Variablen passt (`RANGE ERR` für einen negativen Wert; `LOMEM=` ergibt `RANGE ERR`, wenn es nicht unter dem Programmanfang liegt, und `>32767 ERR` über 32767). Mit dem Original gibt es eine geprüfte Umgehung: den Zeiger direkt schreiben, dann `SCR`:

```basic title="HIMEM auf 3072 ($0C00) senken"
POKE 76,0 : POKE 77,12 : SCR
```

Beim Start von G2 Basic ist HIMEM die Obergrenze des gefundenen RAMs und LOMEM `$0800` (`$2000` mit einem GEN2-Treiber). (`76`/`77` = `$4C`/`$4D`, 12 × 256 = 3072.) Tippen Sie diese drei Anweisungen in einer nummerierten Zeile oder einzeln im Direktmodus, **bevor** Sie das Programm eingeben: `SCR` löscht es.

## BASIC und Maschinensprache mischen

| Bereich (8 KB) | Für 6502-Code nutzbar? |
|---|---|
| `$0000-$00FF` | Zero Page: Der Woz Monitor benutzt `$24-$2B`, BASIC mindestens `$4A-$FF`; meiden, oder sichern und wiederherstellen |
| `$0100-$01FF` | 6502-Stapel: nein |
| `$0200-$027F` | Zeilenpuffer des Woz Monitors; von jeder getippten Zeile überschrieben |
| `$0280-$07FF` | frei, solange BASIC ihn nicht anrührt: der beste Platz |
| `$0800-…` | BASIC-Variablen (außer LOMEM wird angehoben) |
| `… -$0FFF` | BASIC-Programm |

Das Maschinenprogramm wird von Kassette oder per POKE geladen und mit `CALL 768` (für `$0300`) aufgerufen. Es muss mit `RTS` enden und die Zero Page von BASIC schonen. Kleinstes Beispiel:

```mon
>» POKE 768,96
>» CALL 768
>
```

(96 = `$60` = `RTS`: Der Aufruf kehrt sofort zurück.)
