# Fehlermeldungen und Grenzen

## Meldungen

Ein Fehler gibt `*** MELDUNG ERR` aus, gefolgt von `STOPPED AT n`, wenn ein Programm läuft. Eine bei der Eingabe abgelehnte Zeile wird nicht gespeichert: Tippen Sie sie erneut.

| Meldung | Ursache | Ausprobiertes Beispiel |
|---|---|---|
| {{SYNTAX ERR}} | unbekannte Anweisung, ungültiger Variablenname, `:` im Direktmodus | `CNT=7`, `GET A$`, `X=5:PRINT X` im Direktmodus |
| {{>32767 ERR}} | Überlauf, Division durch null, Zeilennummer > 32767 | `200*200`, `10/0`, `RND(0)`, `32768 PRINT` |
| {{TOO LONG ERR}} | Zeile zu verwickelt (Ausdruck zu tief verschachtelt) oder zu lang | 13 verschachtelte Klammern |
| {{MEM FULL ERR}} | kein Platz mehr zwischen Variablen und Programm | `DIM A(3000)` in 8 KB |
| {{TOO MANY PARENS ERR}} | Klammern (vom Interpreter vorgesehen; von Fehler 3 verdeckt) | — |
| {{STRING ERR}} | Zeichenkette falsch verwendet | — |
| {{END ERR}} | das Programm endet ohne `END` | `10 PRINT "X"`, dann `RUN` |
| {{BAD BRANCH ERR}} | `GOTO`/`GOSUB` zu einer nicht vorhandenen Zeile | `GOTO 500` |
| {{>8 GOSUBS ERR}} | mehr als 8 verschachtelte `GOSUB` | Rekursion: 9 Aufrufe |
| {{BAD RETURN ERR}} | `RETURN` ohne `GOSUB` | `RUN 30` auf ein `RETURN` |
| {{>8 FORS ERR}} | mehr als 8 verschachtelte `FOR`, oder Fehler 1 | 9 Schleifen in einer Zeile |
| {{BAD NEXT ERR}} | `NEXT` ohne passendes `FOR` | `20 NEXT I` allein |
| {{STOPPED AT}} | während des Laufs wurde eine Taste gedrückt | — |
| {{>255 ERR}} | Wert > 255: `POKE`, `DIM` einer Zeichenkette | `POKE 768,300`, `DIM B$(256)` |
| {{RANGE ERR}} | Feldindex außerhalb der Grenzen | `A(0)`, `A(3)` nach `DIM A(2)` |
| {{DIM ERR}} | Feld oder Zeichenkette erneut dimensioniert | — |
| {{STR OVFL ERR}} | Zeichenkette länger als ihr `DIM` | `A$="ABCDE"` mit `DIM A$(3)` |
| {{RETYPE LINE}} | ungültige Antwort auf `INPUT` (Buchstaben für eine Zahl): neu tippen | — |

Die Zeilen mit „—“ stammen aus der Meldungstabelle des Interpreters; sie wurden in unseren Tests nicht ausgelöst.

## Grenzen

| Grenze | Wert |
|---|---|
| ganze Zahlen | −32767 bis 32767 |
| Zeilennummern | 0 bis 32767 |
| Variablennamen | Buchstabe + wahlweise Ziffer |
| Namen von Zeichenketten | ein Buchstabe + `$` |
| Länge einer Zeichenkette | 255 |
| Dimensionen eines Feldes | 1, Indizes von 1 bis n |
| verschachtelte `FOR` | 8 |
| verschachtelte `GOSUB` | 8 |
| verschachtelte Klammern | 12 |
| Länge einer getippten Zeile | 120 Zeichen gehen durch, 141 werden abgelehnt |


## Speicher sparen

Bei 8 KB hat BASIC etwa 2 KB für Programm und Variablen. Einige Wege, etwas zu gewinnen:

<ol class="steps">
<li><strong>LOMEM senken</strong>, wenn Sie keinen Maschinencode haben: <code>LOMEM=768</code> (<code>$0300</code>) gibt den Variablen 1280 Bytes mehr. Ausprobiert: <code>DIM T(1200)</code> scheitert mit dem Standard-LOMEM an <code>MEM FULL</code> und gelingt nach <code>LOMEM=768</code>. Die während der Entwicklung gemachten Portierungen aus *101 BASIC Computer Games* tun genau das.</li>
<li><strong>Knapp schreiben</strong>: BASIC behält Leerzeichen und <code>REM</code>. <code>LIST</code> rückt selbst wieder ein, und eine lange Zeile mit mehreren Anweisungen kostet weniger als mehrere Zeilen.</li>
<li><strong>Felder und Zeichenketten wiederverwenden</strong>, statt neue zu dimensionieren.</li>
<li><strong>RAM hinzufügen</strong>: die Einstellungen ⚙ eines Programms, <code>--ram 16</code> oder eine echte Erweiterung. Ein Programm über 8 KB verlangt eine Maschine mit 12, 16 oder 32 KB: <code>apple1-emu basic --ram 16</code> legt es oben in diesen Speicher.</li>
</ol>
