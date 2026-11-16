# Ein Programm aus Applesoft oder Microsoft BASIC portieren

<p class="lead">Die meisten Listings der 1970er- und 1980er-Jahre sind in Microsoft BASIC geschrieben (Applesoft, das BASIC von „101 BASIC Computer Games“). Sie lassen sich auf den Apple-1 portieren, sofern man die Entsprechungen kennt. Jede unten als „abgelehnt“ bezeichnete Zeile wurde im echten BASIC getippt.</p>

## Entsprechungen

| Applesoft / Microsoft | Integer BASIC des Apple-1 | Anmerkung |
|---|---|---|
| `ON X GOTO 100,200,300` | `GOTO 100*X` (wenn die Zeilennummern es erlauben) oder eine Folge `IF X=1 THEN 100` | abgelehnt |
| `GET A$` | `INPUT A$` | abgelehnt |
| `DATA` / `READ` / `RESTORE` | durch Zuweisungen gefülltes Feld oder eine zerlegte Zeichenkette | abgelehnt |
| `MID$(A$,I,N)` | `A$(I,I+N-1)` | abgelehnt |
| `LEFT$(A$,N)` / `RIGHT$(A$,N)` | `A$(1,N)` / `A$(LEN(A$)-N+1)` | — |
| `A$+B$` | `A$(LEN(A$)+1)=B$` | — |
| `CHR$(N)`, `ASC(A$)` | Bezugskette (Kapitel 18, Fehler 6) | abgelehnt |
| `STR$(N)`, `VAL(A$)` | direkt `PRINT N`; `INPUT N` | abgelehnt |
| `INT(X)` | unnötig: alles ist ganzzahlig | abgelehnt |
| `RND(1)` (reell von 0 bis 1) | `RND(N)` (ganzzahlig von 0 bis N−1) | — |
| `X^2` | `X*X` | hängt (Fehler 2) |
| `HOME` | 24 × `PRINT`, oder CLEAR SCREEN (<kbd>Strg</kbd>+<kbd>L</kbd>) von Hand | abgelehnt |
| `PRINT TAB(10);"X"` | `TAB 10:PRINT "X"` | — |
| `STOP` | `END` | abgelehnt |
| `DEF FN` | `GOSUB` | — |
| `A<>B` | `A#B` (`<>` wird auch angenommen) | — |
| `IF A THEN X:Y` | umgekehrte Bedingung + Sprung (Fehler 4) | Falle |
| `DIM A(10,10)` | `DIM A(121)` und `A(I*11+J+1)` | — |
| `A(0)` | Indizes beginnen bei 1 | `RANGE ERR` |
| `SCORE`, `NAME$` | `S`, `N$`: ein Buchstabe (+ Ziffer) | `SYNTAX ERR` |
| reelle Zahlen | ganze Zahlen ×10 oder ×100 (Festkomma) | — |

{{!Portierung}}

## Das Vorgehen

<ol class="steps">
<li><strong>Die Variablen auflisten</strong> des Originalprogramms und jeder einen Buchstaben (+ Ziffer) zuordnen. Die Tabelle oben in <code>REM</code> festhalten.</li>
<li><strong>Die DATA ersetzen</strong> durch Zuweisungen in einem Initialisierungs-Unterprogramm oder durch eine Zeichenkette, die man zerlegt.</li>
<li><strong>Jedes <code>IF … THEN … : …</code> finden</strong> und mit einem Sprung umschreiben (Fehler 4).</li>
<li><strong>Jedes Verlassen eines <code>FOR</code> mit <code>GOTO</code> finden</strong> und durch „Endwert + <code>NEXT</code>“ ersetzen (Fehler 1).</li>
<li><strong>Feldindizes um 1 verschieben</strong>, wenn das Programm den Index 0 benutzt.</li>
<li><strong>Im Editor prüfen</strong>: Er markiert beim Tippen Wörter, die das Integer BASIC nicht kennt. <strong>▶ Starten</strong> tippt das Programm ein: Jede abgelehnte Zeile erscheint auf dem Bildschirm mit <code>SYNTAX ERR</code>.</li>
<li><strong>Den Speicher messen</strong>: <code>apple1-emu basic prog.bas</code> gibt die Größe und den nötigen RAM an. Zu groß für 8 KB? <code>LOMEM=768</code> (Kapitel 17), oder eine Maschine mit 16 KB.</li>
<li><strong>Die Kassette exportieren</strong> (<strong>📼 WAV exportieren</strong>) und die Bereiche für die echte Maschine notieren.</li>
</ol>

## Ein Portierungsbeispiel

Ein typisches Stück in Microsoft BASIC:

```text title="Original (Microsoft BASIC)"
100 READ N$,S
110 IF S>HI THEN HI=S:BEST$=N$
120 IF N$<>"END" THEN 100
130 PRINT "BESTER: ";BEST$;" (";HI;")"
140 DATA "ANNA",12,"BERND",17,"CLARA",9,"END",0
```

Auf dem Apple-1: kein `READ`/`DATA`, keine langen Namen, und das `IF` in Zeile 110 würde `BEST$` bei jedem Durchgang zuweisen (Fehler 4).

```basic title="Portierung in Integer BASIC (ausprobiert)"
10 DIM N$(10),B$(10),S(3)
20 S(1)=12:S(2)=17:S(3)=9
30 H=0
40 FOR I=1 TO 3
50 GOSUB 200
60 IF S(I)<=H THEN 80
70 H=S(I):B$=N$
80 NEXT I
90 PRINT "BESTER: ";B$;" (";H;")"
100 END
200 IF I=1 THEN N$="ANNA"
210 IF I=2 THEN N$="BERND"
220 IF I=3 THEN N$="CLARA"
230 RETURN
```

```mon
>» RUN
BESTER: BERND (17)
```

## Lehren aus den Portierungen

Acht Spiele aus *101 BASIC Computer Games* (DEC, 1975) wurden während der Entwicklung des Emulators so portiert: BAGLES, MUGWMP, HEX, BULCOW, STARS, TRAP, CHOMP und WAR. Jedes wurde im Emulator gespielt, auf Kassette exportiert und über das ACI-ROM neu geladen. Diese Portierungen werden nicht verbreitet (die Original-Listings sind nicht frei), aber sie haben die Regeln dieses Kapitels festgelegt: Variablen aus einem Buchstaben, Felder ab 1, kein `READ`/`DATA`, kein `GOTO` aus einem `FOR` heraus, eine einzige von `IF` gesteuerte Anweisung und `LOMEM=768` für die größeren.
