# Auf einem echten Apple-1: Export als WAV

Alles, was Sie im Emulator schreiben, kann auf einen echten Apple-1, einen Nachbau oder einen Bausatz mit der ACI-Kassettenkarte gelangen. Die Audiodatei ersetzt die Kassette; ein Smartphone oder der Kopfhörerausgang eines PCs dient als Kassettenrekorder.

## Die Kassette erzeugen

Klicken Sie im Editor auf **📼 WAV exportieren** und wählen Sie einen Dateinamen. Die Endung bestimmt das Format: `.wav`, oder `.aiff` für ein Telefon (das empfohlene Format, siehe unten). Unten im Editor steht dann genau, was Sie an der echten Maschine tippen müssen:

![Nach dem Export zeigt der untere Rand des Editors den Ladebefehl.](img/de/editeur-export-bas.png "large")

Für `premier.bas` lautet der Befehl `C100R`, dann `004A.00FFR 0F5A.0FFFR`. Das BASIC-Programm belegt zwei aufeinanderfolgende Aufnahmen auf der Kassette: zuerst die Zero Page des BASIC (`$004A` bis `$00FF`, sie enthält die Zeiger des Programms), dann das Programm selbst, das am oberen Ende des Speichers liegt. Die Schaltfläche **Bereiche kopieren** legt den Befehl in die Zwischenablage.

!!! attention "Immer vier Ziffern"
    Tippen Sie `004A.00FF`, niemals `4A.FF`. Das ROM der ACI löscht die bereits gelesenen Ziffern nicht, wenn es auf den Punkt trifft: `4A.FF` würde als `$004A` bis `$4AFF` verstanden, und die Maschine würde auf 19 KB warten, die nie kommen. Der Emulator zeigt immer die richtige Form, und er gibt dieses Verhalten wieder, weil er das echte ROM der ACI ausführt.

## An der echten Maschine laden

BASIC muss bereits im Speicher sein, geladen von seiner eigenen Kassette (`C100R`, dann `E000.EFFFR`).

<ol class="steps">
<li><strong>Bereiten Sie den Abspieler vor.</strong> Kopieren Sie die Datei auf das Telefon, verbinden Sie den Kopfhörerausgang mit dem Eingang „FROM TAPE“ der ACI und stellen Sie die Lautstärke auf das Maximum.</li>
<li><strong>Kehren Sie zum Monitor zurück</strong> mit RESET, dann tippen Sie <code>C100R</code>. Die ACI antwortet <code>C100: A9*</code> und beginnt eine neue Zeile; der Stern ist ihre Eingabeaufforderung.</li>
<li><strong>Tippen Sie die Bereiche</strong> <code>004A.00FFR 0F5A.0FFFR</code>, drücken Sie aber noch nicht RETURN.</li>
<li><strong>Starten Sie die Wiedergabe</strong>, dann drücken Sie <strong>RETURN während des Vorspanns</strong>, des gleichmäßigen Tons von etwa 10 Sekunden, mit dem jede Aufnahme beginnt.</li>
<li><strong>Warten Sie, bis das <code>\</code> zurückkommt</strong>, dann tippen Sie <code>E2B3R</code> (Warmstart des BASIC, der das Programm behält) und <code>RUN</code>.</li>
</ol>

```mon
» C100R

C100: A9*
» 004A.00FFR 0F5A.0FFFR
\
» E2B3R

E2B3: 20
» >RUN
HALLO, APPLE-1!
```

!!! apple1 "Zuverlässig von einem Telefon lesen"
    - **Lautstärke auf Maximum.** Bestätigen Sie die Android-Warnung „hohe Lautstärke“, sonst bleibt die Lautstärke begrenzt. Viele Telefone senken die Lautstärke bei jedem erneuten Einstecken des Klinkensteckers: Prüfen Sie sie jedes Mal.
    - **Keine Klangeffekte**: Schalten Sie Equalizer, Dolby, Mi Sound und jede „Audio-Verbesserung“ aus.
    - **Flugmodus** oder „Nicht stören“, damit sich keine Benachrichtigung in das Signal mischt. Kein Bluetooth.
    - **Ein einfacher Abspieler**, der AIFF ohne Bearbeitung abspielt; VLC funktioniert gut.
    - **Niemals MP3 oder AAC**: Die Kompression verzerrt die Flanken des Signals. Bleiben Sie bei WAV oder AIFF.
    - **Verlassen Sie sich nicht auf die LED der ACI**: Sie leuchtet erst bei etwa 1,2 V, weit über dem, was die Karte braucht.

## Warum AIFF mit 48 kHz

Die Dateien des Emulators sind für den Eingang der ACI gemacht. Unkomprimiertes AIFF mit 16 Bit entgeht jedem Codec; 48 kHz ist die native Rate von Android, das Telefon muss also nicht umrechnen. Beide Stereokanäle sind streng identisch, so dass links und rechts an FROM TAPE zusammengeschaltet werden können, ohne dass die beiden Ausgänge gegeneinander arbeiten. Das Signal ist ein Rechteck mit steilen Flanken, das, was der Komparator der ACI am besten erkennt.

!!! astuce "Eine Datei vor dem Abspielen prüfen"
    Auf der Befehlszeile simuliert `apple1-emu info premier.aiff --aci-input stock` den Eingang der Originalkarte bei mehreren Spannungen und sagt Ihnen, ab welchem Pegel die Datei richtig gelesen wird (bei unseren Dateien etwa 0,25 V Spitze).
