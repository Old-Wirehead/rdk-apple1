# Referenz der Anweisungen und Funktionen

<p class="lead">Alle Anweisungen, die das Integer BASIC des Apple-1 kennt, aus der Schlüsselworttabelle des Interpreters, mit dem, was sie wirklich tun. Jedes Beispiel wurde ausprobiert.</p>

## Anweisungen

`PRINT` Liste `[;]`
{: .syntax }

{{PRINT}} zeigt Zahlen und Zeichenketten. `;` hängt Elemente ohne Leerzeichen an (`PRINT A;B` gibt für A=1, B=2 `12` aus); `,` springt zur nächsten Spalte, in Schritten von 8 (`PRINT 1,2,3` gibt `1       2       3` aus). Ein `;` am Ende unterdrückt den Zeilenumbruch. `PRINT` allein geht zur nächsten Zeile.

`INPUT` `["Aufforderung",]` Variable `[,Variable…]`
{: .syntax }

{{INPUT}} liest eine Zahl oder eine Zeichenkette. BASIC gibt nach der Aufforderung immer `?` aus: `INPUT "NAME",N$` gibt `NAME?` aus. Eine Zeichenkette, die für ihr `DIM` zu lang ist, ergibt `STR OVFL ERR`.

`LET` Variable `=` Ausdruck
{: .syntax }

{{LET}} ist freiwillig: `A=5` genügt.

`IF` Ausdruck `THEN` Anweisung | Zahl
{: .syntax }

{{IF}} führt die Anweisung aus, wenn der Ausdruck nicht null ist (`IF A THEN …` ist erlaubt). `THEN 100` springt zur Zeile 100. **Vorsicht**: Nur die erste Anweisung hängt von der Bedingung ab (Kapitel 18, Fehler 4).

`GOTO` Ausdruck — `GOSUB` Ausdruck — `RETURN`
{: .syntax }

{{GOTO}} und {{GOSUB}} nehmen einen **Ausdruck** an: `GOTO K*100`, `GOSUB 300+K` (das „berechnete GOTO“ ersetzt das fehlende `ON … GOTO`). Eine nicht vorhandene Zeile ergibt `BAD BRANCH ERR`. Höchstens acht verschachtelte {{GOSUB}} (`>8 GOSUBS ERR` beim neunten); `RETURN` ohne `GOSUB` ergibt `BAD RETURN ERR`.

`FOR` Var `=` Anfang `TO` Ende `[STEP Schritt]` … `NEXT` Var
{: .syntax }

{{FOR}}/{{NEXT}}: Der Rumpf wird **mindestens einmal** ausgeführt, auch wenn der Anfang hinter dem Ende liegt (`FOR I=5 TO 1` läuft einmal durch). `STEP` darf negativ sein. Höchstens acht verschachtelte Schleifen (`>8 FORS ERR`). `NEXT` muss die Variable nennen; ein `NEXT` ohne `FOR` ergibt `BAD NEXT ERR`.

`DIM` Var`(n)` `[,Var$(n)…]`
{: .syntax }

{{DIM}} legt ein Feld aus `n` ganzen Zahlen an, nummeriert **von 1 bis n** (`A(0)` ergibt `RANGE ERR`), oder reserviert `n` Zeichen (höchstens 255) für eine Zeichenkette. Jede Zeichenkette muss dimensioniert werden, bevor ihr etwas zugewiesen wird.

`REM` Text
{: .syntax }

{{REM}}: ein Kommentar bis zum Zeilenende (ein `:` nach REM gehört zum Kommentar).

`END`
{: .syntax }

{{END}} beendet das Programm. Ein Programm, das ohne `END` endet, gibt `*** END ERR` aus: Das ist harmlos, aber beenden Sie Ihre Programme mit `END`.

`TAB` n
{: .syntax }

{{TAB}} ist eine **Anweisung** (keine PRINT-Funktion wie in Applesoft): `TAB 10:PRINT "*"` setzt den Stern in Spalte 10.

`POKE` Adresse`,`Wert
{: .syntax }

{{POKE}} schreibt ein Byte (0 bis 255, sonst `>255 ERR`). Adressen über 32767 werden als negative Zahlen geschrieben: Adresse − 65536.

`CALL` Adresse
{: .syntax }

{{CALL}} ruft ein Maschinenprogramm auf, das mit `RTS` zurückkehrt. Beispiel: `POKE 768,96 : CALL 768` (96 = `RTS`). `CALL -256` kehrt sauber zum Woz Monitor zurück (`$FF00`).

## Funktionen

| Funktion | Ergebnis | Geprüftes Beispiel |
|---|---|---|
| {{PEEK}}`(adr)` | das Byte an der Adresse | `PEEK(76)+PEEK(77)*256` ergibt HIMEM (4096) |
| {{RND}}`(n)` | zufällige ganze Zahl von 0 bis n−1; `RND(0)`: `>32767 ERR`; `RND(-5)` ergibt eine negative Zahl | `RND(6)+1`: ein Würfel |
| {{SGN}}`(x)` | −1, 0 oder 1 (auch für eine Zahl ab 256) | `SGN(-5);SGN(0)` gibt `-10` aus |
| {{ABS}}`(x)` | Betrag | `ABS(-9)` = 9 |
| `KEY(n)` (nur G2 Basic) | Code der gedrückten Taste, ohne zu warten (siehe unten) | `K=KEY(0)` |
| {{LEN}}`(a$)` | Länge der Zeichenkette | `LEN("ABC")` = 3 |
| {{USR}}`(x)` | ruft `$0000` auf: **stürzt ab** ohne vorbereitetes Programm | — |
| {{ASC}}`(`, {{SCRN}}`(` | in der Tabelle vorhanden, aber `SYNTAX ERR` | — |

## Operatoren

| Vorrang | Operatoren |
|---|---|
| 1 | einstelliges `-` und `+`, `NOT` |
| 2 | `*` `/` `MOD` |
| 3 | `+` `-` |
| 4 | `=` `#` (ungleich, auch `<>`) `<` `>` `<=` `>=` |
| 5 | `AND` |
| 6 | `OR` |

{{MOD}}, {{AND}}, {{OR}} und {{NOT}}:

- Die Division **schneidet zur Null hin ab**: `7/2` = 3, `-7/2` = −3. `MOD` nimmt das Vorzeichen des Dividenden: `-7 MOD 3` = −1.
- Vergleiche ergeben 1 (wahr) oder 0 (falsch): `PRINT 5>3;5=3` gibt `10` aus. `NOT 0` = 1, `NOT 5` = 0.
- `AND` und `OR` sind logisch, nicht bitweise: `3 AND 0` = 0, `3 OR 0` = 1.
- `3*-2` wird angenommen und ergibt −6.
- `^` steht in der Tabelle, **hängt aber die Maschine auf** (Kapitel 18, Fehler 2).
- Zeichenketten lassen sich nur mit `=` und `#` vergleichen; `"ABC"<"ABD"` wird abgelehnt (`SYNTAX ERR`).

## Erweiterungen von RDK G2 Basic

Diese funktionieren nur mit **RDK G2 Basic** (das originale Integer BASIC antwortet `SYNTAX ERR`).

`KEY(n)`
{: .syntax }

`KEY(0)` liefert den **Code der Taste**, die gedrückt wurde (den 7-Bit-ASCII-Code: 65 für `A`, 13 für Return, 32 für die Leertaste) und verbraucht sie, oder `0`, wenn keine Taste gedrückt wurde. Es wartet nie, also kann ein Programm laufen, während es die Tastatur beobachtet. `KEY(1)` (jedes andere Argument ungleich null) **wartet** auf eine Taste.

```basic title="Auf eine Taste warten ohne INPUT"
10 K=KEY(0)
20 IF K=0 THEN 10
30 PRINT K
```

Im Original **hält** eine Taste, die gedrückt wird, während ein Programm läuft, es **an** (`STOPPED AT n`), und `INPUT` blockiert bis Return. Mit G2 Basic hält ein Tastendruck ein Programm nicht mehr an, sobald es `KEY` aufgerufen hat: Nur `KEY` sieht die Tastatur, und ein von `KEY` gelesenes <kbd>Strg</kbd>+<kbd>C</kbd> (Code 3) hält das Programm beim nächsten Aufruf an. Rufen Sie `KEY` also in der Hauptschleife des Programms auf.

`LOMEM=` n — `HIMEM=` n
{: .syntax }

Direktbefehle, die einen Ausdruck annehmen (`LOMEM=2048+512`); in einem Programm ergeben sie `SYNTAX ERR`, wie im Original. Sie werden mit dem Speicher in Kapitel 16 beschrieben.

`LIST` `[`a`[,`b`]]` — `DEL` a`[,`b`]`
{: .syntax }

`LIST` listet alles, `LIST n` eine Zeile, `LIST a,b` die Zeilen von `a` bis `b`. `DEL n` löscht eine Zeile, `DEL a,b` die Zeilen von `a` bis `b` (ist `b` kleiner als `a`, nur die Zeile `a`). Eine gelistete Zeile, die länger als der Bildschirm ist, wird wie im Original umbrochen (Folgezeilen sind um sechs Leerzeichen eingerückt).

Teilzeichenketten folgen dem Original: `A$(I)` mit `I` ab 256 oder negativ ergibt `>255 ERR`, und `A$(I,J)` mit `J` kleiner als `I-1` ergibt `STRING ERR`.

## Schlüsselwörter, die nichts tun

Die Tabelle enthält `COLOR=`, `PLOT`, `HLIN`, `VLIN` und `VTAB`, geerbt vom BASIC-Projekt für den Apple II. Auf dem Apple-1 werden `COLOR=` und `PLOT` angenommen und tun nichts, `VTAB` ergibt `SYNTAX ERR`. Lassen Sie sie aus Ihren Programmen heraus.
