# Integer BASIC <span class="part-sub">Il BASIC di Steve Wozniak e il suo clone libero RDK G2 Basic: tutte le istruzioni, la memoria, i messaggi di errore, i bug noti e come aggirarli, e il porting da Applesoft</span> {: .part data-part="Parte II" }

# Primi passi in BASIC

<p class="lead">L'Integer BASIC dell'Apple-1 sta in 4 KB. Conosce solo gli interi e i nomi di variabile di una lettera, e non ha né DATA né funzioni sulle stringhe: è piccolo, veloce e ha le sue trappole.</p>

!!! apple1 "Da dove viene questo BASIC"
    Steve Wozniak lo scrisse nel 1976, a mano, in assembly, senza un computer per assemblarlo. Era venduto su cassetta (75 $) e si carica in `$E000-$EFFF`. L'emulatore non fornisce l'originale di Apple: usate il vostro `BASIC.wav` o un'immagine da 4 KB (capitolo 4) se lo volete. Tutti i test di questa parte sono stati fatti nell'emulatore, con questo originale.

## Due BASIC: RDK G2 Basic e l'originale

Dalla versione 7.9 l'emulatore ha un BASIC proprio, **RDK G2 Basic**, integrato nel programma. È un clone in clean room dell'Integer BASIC, scritto da zero in assembly 6502 (non contiene codice dell'interprete di Apple) e distribuito con licenza MIT. È dedicato a Uncle Bernie.

- **Compatibile**: stessa sintassi, stessi token, stesso formato del programma in memoria, stesso formato di cassetta. Programmi, listati e cassette scritti per l'originale girano senza modifiche. È stato verificato contro l'originale con 22 sessioni di test dall'output identico.
- **Corretto**: i bug noti dell'originale, descritti nel capitolo 18, sono corretti.
- **Più grande**: 7933 byte invece di 4 KB. Richiede RAM da `$E000` a `$FEFF`; l'emulatore la fornisce, ma non entrerebbe nella finestra EPROM da 4 KB dell'originale.
- **Conosce la GEN2**: all'avvio cerca la scheda GEN2 e, se la trova, avvia il driver della scheda (e mette allora LOMEM a `$2000`, sopra le pagine grafiche). Le istruzioni grafiche vere e proprie sono previste per la versione 2 di G2 Basic.
- **Saluto**: `RDK G2 BASIC 1.0`, poi `DEDICATED TO UNCLE BERNIE`.
- **Esteso** (versione 7.9.2): `LOMEM=` e `HIMEM=` funzionano come nell'originale, e una nuova funzione **`KEY(n)`** legge la tastiera senza attendere, così che un gioco possa guardare i tasti senza che `INPUT` lo blocchi e senza che la pressione di un tasto fermi il programma (capitolo 15). `LIST a,b` e `DEL a,b` usano gli intervalli dell'originale, e una riga listata più lunga dello schermo va a capo come nell'originale.

Il pulsante **BASIC** carica G2 Basic. Per usare invece l'originale di Apple, aprite **⚙ Preferenze → Avanzate → BASIC**, scegliete **Integer BASIC originale (il suo file, caricato in $E000)** e il file; la scelta viene ricordata e il pulsante da allora lo carica. I due si comportano allo stesso modo per tutto ciò che è descritto in questa parte; le differenze sono indicate dove contano (HIMEM all'avvio, i bug del capitolo 18).

!!! note "Sorgente e test"
    G2 Basic ha un repository proprio, `Old-Wirehead/rdk-gen2basic-apple1`, con il suo manuale, il suo sorgente e i suoi test di regressione.

## Avviare il BASIC

Fate clic su **BASIC** nella barra degli strumenti: l'emulatore carica il BASIC scelto (G2 Basic per impostazione predefinita) e voi lo avviate con `E000R`. Il prompt del BASIC è `>`, quello del Woz Monitor `\`.

```mon
\
» E000R
E000: 4C
>
```

- `E000R`: **avvio a freddo**, cancella il programma in memoria.
- `E2B3R`: **avvio a caldo**, conserva il programma. È l'indirizzo da usare dopo aver caricato un programma da cassetta, e per tornare al BASIC dopo un passaggio dal monitor (RESET).

{{E000R}}{{E2B3R}}

## Modo diretto e modo programma

Una riga che comincia con un **numero** (da 0 a 32767) viene memorizzata nel programma; una riga senza numero viene eseguita subito.

```mon
>» PRINT 2*3+4*5
26
>» 10 PRINT "CIAO"
>» 20 GOTO 10
>» RUN
CIAO
CIAO
…
```

Premete un tasto per fermare il programma: il BASIC stampa `STOPPED AT 10`. Ridigitare una riga con lo stesso numero la sostituisce; un numero da solo la cancella.

!!! attention "Il modo diretto è limitato"
    In modo diretto il BASIC rifiuta le istruzioni separate da `:` (`SYNTAX ERR`), come pure `FOR` e `GOSUB`. Per provare un ciclo, scrivetelo in una riga numerata.

## Correggere una riga

La tastiera dell'Apple-1 non ha un tasto di cancellazione: il **Backspace** dell'emulatore digita `_`, che annulla il carattere precedente (la riga sullo schermo non viene corretta, ma il BASIC riceve quella giusta). <kbd>Esc</kbd> annulla l'intera riga: il BASIC stampa `\` e attende la riga successiva. Per modificare con più comodità, usate l'editor (capitolo 7): colora la sintassi, segnala gli errori prima dell'invio e digita il programma per voi.

## Un primo programma

@@include exemples/it/basic/boucle.bas

```mon
>» RUN
SOMMA = 55
>
```

Nell'editor, **▶ Esegui** digita il programma e `RUN`; nel debugger potete seguirlo riga per riga (figura nel capitolo 8).

## Comandi

| Comando | Effetto |
|---|---|
| {{LIST}} `[n[,m]]` | lista il programma, la riga `n`, o le righe da `n` a `m` |
| {{RUN}} `[n]` | cancella le variabili ed esegue il programma, dalla riga `n` se indicata |
| {{DEL}} `n[,m]` | cancella la riga `n`, o le righe da `n` a `m` |
| {{SCR}} | cancella il programma e le variabili |
| {{CLR}} | cancella le variabili, conserva il programma |
| {{AUTO}} `n[,passo]` | numerazione automatica da `n` (passo 10 per impostazione predefinita); <kbd>Ctrl</kbd>+<kbd>D</kbd> poi <kbd>RETURN</kbd> la termina, <kbd>Esc</kbd> no |
| {{MAN}} | torna alla numerazione manuale |
| {{HIMEM}}`=n`, {{LOMEM}}`=n` | i limiti della memoria del BASIC (capitolo 16) |

`GOTO n` in modo diretto riprende un programma fermato **senza** cancellare le variabili, a differenza di `RUN n`.

!!! astuce "LIST e la visualizzazione lenta"
    Il terminale dell'Apple-1 mostra circa 60 caratteri al secondo: listare un programma lungo richiede tempo. Disattivate il limite di visualizzazione (**Veloce** nella riga di visualizzazione, o nella barra di stato) per leggere più in fretta, oppure listate a fette: `LIST 100,200`.

## Tre programmi da digitare

### Indovinare un numero

@@include exemples/it/basic/devine.bas

```mon
>» RUN
HO SCELTO UN NUMERO DA 1 A 100.
LA TUA PROPOSTA?» 50
PIU ALTO!
LA TUA PROPOSTA?» 75
PIU BASSO!
LA TUA PROPOSTA?» 62
PIU BASSO!
LA TUA PROPOSTA?» 56
PIU ALTO!
LA TUA PROPOSTA?» 57
BRAVO, IN 5 TENTATIVI.
```

`RND(100)` dà un intero da 0 a 99; `#` significa «diverso da».

### Una tavola pitagorica

@@include exemples/it/basic/table.bas

### Giocare con le stringhe

@@include exemples/it/basic/chaines.bas

```mon
>» RUN
IL TUO NOME?» RADAR
INIZIALE: R
FINE: AR
AL CONTRARIO: RADAR
E UN PALINDROMO!
```

Le stringhe si tagliano con `N$(inizio,fine)` e si allungano con `R$(LEN(R$)+1)=…`: non ci sono né `MID$` né `+` per le stringhe (capitolo 16).

I quattro programmi sono in `docs/exemples/it/basic/`: apriteli nell'editor, o fate una cassetta con `apple1-emu basic devine.bas -o devine.aiff`.
