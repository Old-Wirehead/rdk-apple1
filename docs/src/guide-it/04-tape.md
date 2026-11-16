# Caricare ed eseguire una cassetta

Sull'Apple-1 tutto passa dalla cassetta: lo stesso Integer BASIC si caricava da un nastro audio. L'emulatore legge i file `.wav` e `.aiff` come fa la scheda ACI originale, byte per byte, e riconosce le 24 cassette della raccolta «Apple1Cassettes» di archive.org.

## Preparare la libreria

Copiate le vostre cassette nella cartella della libreria, `~/apple1/programmes`: per esempio `Hamurabi.wav`. L'elenco a destra si aggiorna da solo. Le cinque cartelle BASIC, 6502, GEN2 COLOR, WAV e Art ASCII vi compaiono direttamente, senza cartella radice sopra di loro; i file lasciati sciolti nella cartella della libreria non vengono mostrati. Ogni cassetta riconosciuta porta il segno **✔ riconosciuto**, con i suoi intervalli di indirizzi e il suo comando di avvio già impostati.

!!! note "Non vi serve BASIC.wav"
    L'emulatore ha il suo BASIC, RDK G2 Basic, quindi i programmi BASIC funzionano senza alcuna cassetta. L'Integer BASIC di Apple del 1976 è ancora protetto dal diritto d'autore e non è incluso: se preferite l'originale, usate una vostra copia (come `BASIC.wav`) e sceglietela in **⚙ Preferenze → Avanzate → BASIC**.

## La via più rapida: il pulsante ▶

Ogni programma dell'elenco ha quattro piccole icone alla sua destra: **▶** carica ed esegue, **⬇** carica in memoria senza eseguire, **📼** carica dalla cassetta, **⚙** apre le sue impostazioni. Un doppio clic fa lo stesso di ▶.

Fate clic su **▶** accanto a `Hamurabi.wav`. L'emulatore carica il BASIC se non è già in memoria, poi il programma, e digita per voi `E2B3R` (ritorno al BASIC senza cancellare il programma) e `RUN`. Il caricamento è istantaneo.

![Hamurabi avviato con il pulsante ▶ della libreria.](img/it/hamurabi-ecran.png "small")

Per un programma in linguaggio macchina, ▶ fa un RESET e poi digita il suo comando di avvio, per esempio `300R`. Un listato BASIC o un sorgente 6502, ▶ li apre anche in una scheda dell'editor, se l'editor è aperto.

## Come nel 1976: la cassetta virtuale

Il caricamento è istantaneo, a meno che non chiediate la cassetta. Fate clic sull'icona **📼** di un programma per riprodurre un caricamento vero: l'emulatore digita `C100R` per avviare il programma dell'ACI, poi l'intervallo di indirizzi, e la cassetta scorre a velocità reale. Sotto il nome del programma compare una barra di avanzamento, e la striscia **Cassetta:** della barra degli strumenti mostra la cassetta e la sua posizione.

![Caricamento di BASIC.wav attraverso l'ACI virtuale, con l'oscilloscopio della cassetta aperto.](img/it/brut/cassette-basic.png "large")

I pulsanti 📈 e 🔊 alla fine della striscia **Cassetta:** aprono l'**oscilloscopio** e vi fanno **sentire** il segnale, come l'altoparlante di un registratore. Caricare il BASIC richiede circa 36 secondi a velocità reale.

Quando la cassetta si ferma, il monitor mostra di nuovo `\`. Digitate `E000R` per avviare il BASIC. Il suo prompt è un `>`:

```mon
» E000R

E000: 4C
» >PRINT 6*7
42
```

!!! astuce "Caricare a mano"
    **📼 Inserisci**, nella striscia **Cassetta:** della barra degli strumenti, mette un qualsiasi file `.wav` o `.aiff` nel registratore virtuale. Poi digitate voi stessi `C100R` e quindi l'intervallo, per esempio `E000.EFFFR` per il BASIC: la cassetta parte alla prima lettura, come sulla macchina vera. Il pulsante **Carica** serve per i file `.txt`, `.bin` e `.hex`, caricati all'istante.

| Programma | Dopo il caricamento |
|---|---|
| Integer BASIC | `E000R` (avvio a freddo, cancella il programma in memoria) |
| Programma BASIC | `E2B3R` (avvio a caldo, conserva il programma), poi `RUN` |
| Linguaggio macchina | il suo indirizzo seguito da `R`, spesso `300R` |
