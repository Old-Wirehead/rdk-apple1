# Ihr erstes BASIC-Programm

Das Integer BASIC, Steve Wozniaks BASIC, passt in 4 KB, und das eigene **RDK G2 Basic** des Emulators, das dieselben Programme ausführt, ist eingebaut. Es kennt nur ganze Zahlen (−32767 bis 32767), ist aber schnell und angenehm. Der eingebaute Editor erspart Ihnen das Tippen auf dem Apple-1-Bildschirm, wo ein Tippfehler bedeutet, die ganze Zeile neu einzugeben.

## Das Programm schreiben

Klicken Sie in der Werkzeugleiste auf **📝 Editor**, dann auf das Symbol **Neu** (das erste) des Editors, wählen Sie „BASIC-Programm“ und tippen Sie dieses Programm ein (auch in `docs/exemples/de/basic/premier.bas`).

```basic title="premier.bas"
10 REM MEIN ERSTES PROGRAMM
20 PRINT "HALLO, APPLE-1!"
30 PRINT
40 FOR I=1 TO 8
50 PRINT I;" HOCH 2 = ";I*I
60 NEXT I
70 PRINT
80 DIM N$(20)
90 INPUT "DEIN VORNAME",N$
100 PRINT "FREUT MICH, ";N$;"!"
110 END
```

Schlüsselwörter, Zahlen, Zeichenketten und Kommentare erhalten jeweils eine eigene Farbe (die Farben lassen sich unter **⚙ Einstellungen → Editor** ändern); unter dem Text zeigt der Editor die Größe des Programms und den für Variablen verbleibenden Speicher.

!!! astuce "Rechtsklick in den Text"
    Das Menü des Editors enthält Ausschneiden, Kopieren und Einfügen, **Suchen** (Strg+F) und **Ersetzen** (Strg+H), Gehe zu Zeile (Strg+G), REM / REM entfernen für einen Block, **Neu nummerieren…** (`GOTO`, `GOSUB` und `THEN` folgen), einen Haltepunkt auf der Zeile (F9) und **Senden und starten** (F5).

## Das Programm starten

Klicken Sie auf **▶ Starten**. Das Programm kommt direkt in den Speicher (nach BASIC, falls nötig), dann tippt der Editor für Sie `E2B3R` und `RUN`:

![Links der Editor, rechts das laufende Programm.](img/en/brut/editeur-premier.png "large")

Das Programm fragt nach Ihrem Vornamen. Klicken Sie auf den Apple-1-Bildschirm, tippen Sie ihn ein, dann Eingabe:

```mon
DEIN VORNAME?WOZ
FREUT MICH, WOZ!
```

!!! attention "Drei Eigenheiten des Integer BASIC"
    - **Zeichenketten müssen dimensioniert werden**: Ohne `DIM N$(20)` bricht das `INPUT` einer Zeichenkette mit `*** STR OVFL ERR` ab.
    - **Variablennamen** bestehen aus einem Buchstaben, auf den eine Ziffer folgen darf: `A`, `B2`, `N$`. `SUMME` ist ein Syntaxfehler.
    - **Das abschließende `END` ist Pflicht**: Ohne es endet das Programm mit `*** END ERR`.

## Wenn ein Fehler auftritt

Der Editor prüft jede Zeile beim Tippen, nach den Regeln des echten BASIC. Eine fehlerhafte Zeile wird rot, und die Meldung erklärt das Problem, mit Zeilennummer und Spalte:

![Zwei gemeldete Fehler: eine nicht abgeschlossene Zeichenkette und ein zu langer Variablenname.](img/en/editeur-erreur.png "medium")

Ein Programm mit Fehlern startet nicht: Beheben Sie zuerst die roten Zeilen. Laufzeitfehler erscheinen auf dem Apple-1-Bildschirm: `*** BAD BRANCH ERR` bei einem `GOTO` zu einer fehlenden Zeile, `*** >32767 ERR` bei einem Überlauf oder einer Division durch null. Der **Debugger** (vollständiges Handbuch) geht dann Zeile für Zeile vor und beobachtet dabei die Variablen.

![Der Arbeitszyklus: schreiben, starten, korrigieren, dann auf die echte Maschine exportieren.](diagrams/en/workflow-basic.svg "diagram")

!!! astuce "Ein Programm anhalten"
    Während eines `RUN` hält ein Tastendruck das Programm mit `STOPPED AT 50` (die aktuelle Zeile) an. Hängt es fest, kehrt **Reset** (Strg+R) zum Monitor zurück, und `E2B3R` kehrt ins BASIC zurück, **ohne** das Programm **zu löschen**.
