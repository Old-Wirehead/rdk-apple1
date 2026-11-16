# Bekannte Fehler und ihre Umgehungen

<p class="lead">Das Integer BASIC des Apple-1 wurde 1976 ohne Assembler geschrieben und nie berichtigt. Hier sind seine bekannten Mängel, jeder im Emulator mit dem echten BASIC nachvollzogen, und wie man sie vermeidet.</p>

<span class="verified">geprüft</span>: im Emulator mit dem echten Integer BASIC nachvollzogen, mit einem Protokoll als Beleg. <span class="unverified">nicht geprüft</span>: von einer Quelle gemeldet, nicht nachvollzogen. Fehler n entspricht dem Abschnitt 18.n. Da der Emulator den Originalcode Befehl für Befehl ausführt, gibt es einen hier nachvollzogenen Fehler auch auf einem echten Apple-1.

## Eine FOR-Schleife mit GOTO verlassen <span class="verified">geprüft</span>

Eine mit `GOTO` verlassene Schleife bleibt „offen“. Kommt das Programm erneut durch das `FOR`, wird ein neuer Eintrag gestapelt; beim neunten Durchgang: `>8 FORS ERR`.

```basic
10 N=0
20 FOR I=1 TO 10
30 IF I=2 THEN GOTO 50
40 NEXT I
50 N=N+1:IF N<20 THEN 20
60 PRINT "FERTIG ";N
70 END
```

```mon
>» RUN
*** >8 FORS ERR
STOPPED AT 20
```

!!! astuce "Umgehung"
    Verlassen Sie ein `FOR` nie mit `GOTO`: Geben Sie der Variablen ihren Endwert und gehen Sie durch `NEXT`. Hier: `30 IF I=2 THEN I=10:GOTO 40`. Das Programm gibt dann `FERTIG 20` aus. Die Portierungen aus *101 BASIC Computer Games* halten sich an diese Regel.

## Der Operator ^ hängt die Maschine auf <span class="verified">geprüft</span>

`^` (Potenz) steht in der Tabelle des Interpreters, aber `PRINT 2^3` kehrt nie zurück: Nur RESET hilft heraus.

!!! astuce "Umgehung"
    Multiplizieren Sie: `2*2*2`, oder eine Schleife `P=1:FOR I=1 TO N:P=P*X:NEXT I`.

## Die Meldung „PPED AT“ <span class="verified">geprüft</span>

Läuft der Operandenstapel des Interpreters über (ein im Direktmodus zu tief verschachtelter Ausdruck), sollte BASIC eine Fehlermeldung ausgeben, zeigt aber **3 Bytes zu weit** in die Meldungstabelle (die Disassemblierung von Eric Smith vermerkt es: `ldy #erri_stopped_at+3 ; The "+3" is a bug!`). Was Sie lesen, ist das Ende von `STOPPED AT`:

```mon
>» PRINT 1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1)))))))))))))
14
*** PPED AT  ERR
STOPPED AT 65535
```

!!! astuce "Umgehung"
    Zerlegen Sie lange Ausdrücke mit Zwischenvariablen. Über 12 Klammerebenen hinaus wird eine Programmzeile ohnehin abgelehnt (`TOO LONG ERR`).

## IF … THEN steuert nur die erste Anweisung <span class="verified">geprüft</span>

In `IF A=2 THEN PRINT "X":PRINT "Y"` hängt nur `PRINT "X"` von der Bedingung ab: `PRINT "Y"` wird **immer** ausgeführt (mit A=1 gibt das Programm `Y` aus). Das ist die häufigste Falle beim Portieren eines Programms, das für ein anderes BASIC geschrieben wurde.

!!! astuce "Umgehung"
    Kehren Sie die Bedingung um und springen Sie über den Block:

    ```basic
    80 IF A#2 THEN 90
    85 PRINT "X":PRINT "Y"
    90 REM WEITER
    ```

## −32768 <span class="verified">geprüft</span>

`X=-32767-1` gibt ein `-` und dann `*** >32767 ERR` aus. Der kleinste verwendbare Wert ist −32767.

!!! astuce "Umgehung"
    Bleiben Sie innerhalb von −32767 … 32767. Um ein 16-Bit-Wort mit `PEEK` zu lesen, zerlegen Sie es: `PEEK(A)+PEEK(A+1)*256` läuft über 32767 über; prüfen Sie zuerst das höherwertige Byte (`IF PEEK(A+1)>127 THEN …`).

## ASC( funktioniert nicht <span class="verified">geprüft</span>

`ASC("A")` steht in der Schlüsselworttabelle, aber jede Zeile, die es verwendet, wird abgelehnt: `SYNTAX ERR`. Ebenso `SCRN(`.

!!! astuce "Umgehung (ausprobiert)"
    Suchen Sie das Zeichen in einer Bezugskette. Zeile 60 schließt die Schleife (`K=26:NEXT K`), um Fehler 1 zu vermeiden:

```basic title="Die Stelle eines Buchstabens, ohne ASC"
10 DIM C$(26),L$(1)
20 C$="ABCDEFGHIJKLMNOPQRSTUVWXYZ"
30 INPUT "BUCHSTABE",L$
40 FOR K=1 TO 26:IF C$(K,K)=L$ THEN 60:NEXT K
50 PRINT "KEIN BUCHSTABE":END
60 PRINT "STELLE ";K:K=26:NEXT K
70 GOTO 30
```

```mon
BUCHSTABE?» W
STELLE 23
BUCHSTABE?» E
STELLE 5
BUCHSTABE?» 5
KEIN BUCHSTABE
```

## Grafik-Schlüsselwörter, die nichts tun <span class="verified">geprüft</span>

`COLOR=`, `PLOT` und `HLIN` werden angenommen und tun nichts; `VLIN` und `VTAB` ergeben `SYNTAX ERR`. Es sind Überbleibsel des für den Apple II gedachten BASIC.

!!! astuce "Umgehung"
    Um Text zu platzieren, verwenden Sie `TAB n` und leere `PRINT`. Um farbig zu zeichnen, wird die GEN2-Karte mit `POKE` angesteuert (Kapitel 6).

## Der Fehler der 251 Zeichen beim Apple II <span class="verified">nicht vorhanden</span>

Eine spätere Version des Integer BASIC (Apple II) stürzt bei bestimmten Zeilen ab 251 Zeichen ab. Beim Apple-1 ist die getippte Zeile lange vorher begrenzt (etwa 128 Zeichen): Der Fehler kann nicht auftreten.

## STOPPED AT 65535 <span class="verified">geprüft</span>

Auf einen Fehler, den eine **bei der Eingabe abgelehnte** Zeile (zu tief verschachtelt) oder Fehler 3 im Direktmodus verursacht, folgt `STOPPED AT 65535`: BASIC glaubt, in einem Programm zu sein, und gibt eine unsinnige Zeilennummer aus. Das hat keine Folgen; das Programm im Speicher bleibt unberührt.

## USR bringt die Maschine zum Absturz <span class="verified">geprüft</span>

`X=USR(1)` springt nach `$0000`, wo normalerweise kein Code steht: Die Maschine hängt. RESET, dann `E2B3R` führen zu BASIC zurück, ohne das Programm zu verlieren.

!!! astuce "Umgehung"
    Rufen Sie Ihre Maschinenprogramme mit `CALL Adresse` auf; übergeben Sie Werte mit `POKE` und `PEEK`.

## HIMEM= kann nicht sinken <span class="verified">geprüft</span>

`HIMEM=3072` (und jeder Wert unter dem aktuellen) wird **stillschweigend ignoriert**, selbst nach `SCR`, weil der Interpreter den neuen Wert mit dem Programmzeiger vergleicht, bevor er ihn annimmt.

!!! astuce "Umgehung (geprüft)"
    `POKE 76,0 : POKE 77,12 : SCR` setzt HIMEM auf 3072; das nächste Programm wird darunter gespeichert. Siehe Kapitel 16.

## Der Doppelpunkt im Direktmodus abgelehnt <span class="verified">geprüft</span>

`X=5:Y=X*2:PRINT Y` ohne Nummer getippt ergibt `SYNTAX ERR`; dieselbe Zeile mit Nummer funktioniert. Auch `FOR` und `GOSUB` werden im Direktmodus abgelehnt.

!!! astuce "Umgehung"
    Tippen Sie eine Anweisung pro Zeile, oder schreiben Sie eine vorläufige Zeile `0` und `RUN 0` … ohne zu vergessen, sie zu löschen (`DEL 0`).

## INPUT fügt immer „?“ hinzu <span class="verified">geprüft</span>

`INPUT "ALTER ? ",N` gibt `ALTER ? ?` aus: BASIC fügt sein eigenes Fragezeichen hinzu. Schreiben Sie die Aufforderung ohne `?`: `INPUT "ALTER",N` gibt `ALTER?` aus.

## Die FOR-Schleife läuft mindestens einmal <span class="verified">geprüft</span>

`FOR I=5 TO 1 … NEXT I` führt den Rumpf einmal aus (mit I=5), wo andere BASICs ihn überspringen.

!!! astuce "Umgehung"
    Erst prüfen: `IF N<1 THEN 100` vor `FOR I=1 TO N`.

## Ein Tastendruck hält das Programm an <span class="verified">geprüft</span>

Jede Taste, die gedrückt wird, während das Programm läuft, hält es an (`STOPPED AT`), außer während eines `INPUT`. Das ist kein Fehler, sondern eine Entwurfsentscheidung: Es gibt keine BREAK-Taste. Folge: Ein Echtzeitspiel kann nicht auf eine Taste warten, ohne anzuhalten. Die Tastatur mit `PEEK(-12271)` (`$D011`) und `PEEK(-12272)` (`$D010`) zu lesen, scheint möglich, aber der Interpreter prüft die Tastatur auch zwischen den Anweisungen und kann zuerst anhalten: Wir haben es nicht ausprobiert (<span class="unverified">nicht geprüft</span>). Für ein Echtzeitspiel wechseln Sie zur Maschinensprache (Teil III).

## Übersicht

| # | Fehler | Stand | Umgehung |
|---|---|---|---|
| 1 | FOR mit GOTO verlassen → `>8 FORS` | <span class="status-ok">geprüft</span> | Schleife mit NEXT beenden |
| 2 | `^` hängt | <span class="status-ok">geprüft</span> | multiplizieren |
| 3 | „PPED AT“ (+3) | <span class="status-ok">geprüft</span> | Zwischenvariablen |
| 4 | `IF…THEN a:b` führt b aus | <span class="status-ok">geprüft</span> | umgekehrte Bedingung + Sprung |
| 5 | −32768 | <span class="status-ok">geprüft</span> | ≥ −32767 bleiben |
| 6 | `ASC(`, `SCRN(` | <span class="status-ok">geprüft</span> | Bezugskette |
| 7 | COLOR=, PLOT, HLIN tun nichts; VLIN, VTAB abgelehnt | <span class="status-ok">geprüft</span> | TAB, POKE (GEN2) |
| 8 | 251 Zeichen (Apple II) | <span class="status-ok">nicht vorhanden</span> | — |
| 9 | STOPPED AT 65535 | <span class="status-ok">geprüft</span> | harmlos |
| 10 | USR stürzt ab | <span class="status-ok">geprüft</span> | CALL |
| 11 | HIMEM= sinkt nicht | <span class="status-ok">geprüft</span> | POKE 76/77 + SCR |
| 12 | `:` im Direktmodus abgelehnt | <span class="status-ok">geprüft</span> | eine Anweisung pro Zeile |
| 13 | INPUT fügt „?“ hinzu | <span class="status-ok">geprüft</span> | Aufforderung ohne „?“ |
| 14 | FOR läuft einmal | <span class="status-ok">geprüft</span> | erst prüfen |
| 15 | Tastatur mit PEEK während RUN lesen | <span class="status-src">nicht geprüft</span> | Maschinensprache |
