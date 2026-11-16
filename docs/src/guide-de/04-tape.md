# Eine Kassette laden und starten

Beim Apple-1 läuft alles über die Kassette: Selbst das Integer BASIC wurde von einer Audiokassette geladen. Der Emulator liest Dateien `.wav` und `.aiff` so, wie es die originale ACI-Karte tut, Byte für Byte, und erkennt die 24 Kassetten der Sammlung „Apple1Cassettes“ auf archive.org.

## Die Bibliothek vorbereiten

Kopieren Sie Ihre Kassetten in den Ordner der Bibliothek, `~/apple1/programmes`: zum Beispiel `Hamurabi.wav`. Die Liste rechts aktualisiert sich von selbst. Die fünf Ordner BASIC, 6502, GEN2 COLOR, WAV und Art ASCII stehen dort direkt, ohne übergeordneten Ordner; Dateien, die lose im Ordner der Bibliothek liegen, werden nicht angezeigt. Jede erkannte Kassette ist mit **✔ erkannt** markiert, ihre Adressbereiche und ihr Startbefehl sind bereits eingetragen.

!!! note "Sie brauchen BASIC.wav nicht"
    Der Emulator hat sein eigenes BASIC, RDK G2 Basic, daher laufen BASIC-Programme ohne jede Kassette. Apples Integer BASIC von 1976 ist noch urheberrechtlich geschützt und nicht enthalten: Wenn Sie das Original vorziehen, verwenden Sie Ihre eigene Kopie (etwa `BASIC.wav`) und wählen Sie sie unter **⚙ Einstellungen → Erweitert → BASIC**.

## Der schnellste Weg: die Schaltfläche ▶

Jedes Programm der Liste hat rechts vier kleine Symbole: **▶** lädt und startet, **⬇** lädt in den Speicher, ohne zu starten, **📼** lädt über die Kassette, **⚙** öffnet seine Einstellungen. Ein Doppelklick wirkt wie ▶.

Klicken Sie auf **▶** neben `Hamurabi.wav`. Der Emulator lädt BASIC, falls es noch nicht im Speicher ist, dann das Programm, und tippt für Sie `E2B3R` (zurück ins BASIC, ohne das Programm zu löschen) und `RUN`. Das Laden geht sofort.

![Hamurabi, gestartet mit der Schaltfläche ▶ der Bibliothek.](img/en/hamurabi-ecran.png "small")

Bei einem Programm in Maschinensprache macht ▶ ein RESET und tippt dann seinen Startbefehl, zum Beispiel `300R`. Ein BASIC-Listing oder einen 6502-Quelltext öffnet ▶ außerdem in einem Tab des Editors, wenn der Editor offen ist.

## Wie 1976: die virtuelle Kassette

Das Laden geht sofort, es sei denn, Sie verlangen die Kassette. Klicken Sie auf das Symbol **📼** eines Programms, um ein echtes Laden nachzustellen: Der Emulator tippt `C100R`, um das Programm der ACI zu starten, dann den Adressbereich, und die Kassette läuft in Originalgeschwindigkeit. Unter dem Namen des Programms erscheint ein Fortschrittsbalken, und der Streifen **Kassette:** der Werkzeugleiste zeigt die Kassette und ihre Position.

![BASIC.wav wird über die virtuelle ACI geladen, das Kassetten-Oszilloskop ist offen.](img/en/brut/cassette-basic.png "large")

Die Schaltflächen 📈 und 🔊 am Ende des Streifens **Kassette:** öffnen das **Oszilloskop** und lassen Sie das Signal **hören**, wie den Lautsprecher eines Kassettenrekorders. Das Laden von BASIC dauert in Originalgeschwindigkeit etwa 36 Sekunden.

Wenn die Kassette anhält, zeigt der Monitor wieder `\`. Tippen Sie `E000R`, um BASIC zu starten. Seine Eingabeaufforderung ist ein `>`:

```mon
» E000R

E000: 4C
» >PRINT 6*7
42
```

!!! astuce "Von Hand laden"
    **📼 Einlegen** im Streifen **Kassette:** der Werkzeugleiste legt eine beliebige Datei `.wav` oder `.aiff` in den virtuellen Rekorder. Dann tippen Sie selbst `C100R` und danach den Bereich, zum Beispiel `E000.EFFFR` für BASIC: Die Kassette startet beim ersten Lesen, wie an der echten Maschine. Die Schaltfläche **Laden** ist für Dateien `.txt`, `.bin` und `.hex` gedacht, die sofort geladen werden.

| Programm | Nach dem Laden |
|---|---|
| Integer BASIC | `E000R` (Kaltstart, löscht das Programm im Speicher) |
| BASIC-Programm | `E2B3R` (Warmstart, behält das Programm), dann `RUN` |
| Maschinensprache | seine Adresse, gefolgt von `R`, oft `300R` |
