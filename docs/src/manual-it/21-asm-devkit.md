# L'assembler del devkit

<p class="lead">L'emulatore contiene un assembler 6502 completo a due passate, compatibile con ca65. Serve nell'editor, che assembla a ogni tasto premuto, sulla riga di comando e per i progetti GitHub. Non c'è nulla da installare.</p>

## Il ciclo di lavoro

![Il ciclo di lavoro in assembly: scrivere, assemblare, eseguire, fare il debug, poi esportare verso il vero Apple-1.](diagrams/it/workflow-asm.svg "diagram large")

<ol class="steps">
<li><strong>Scrivere</strong>: <strong>📝 Editor</strong>, poi <strong>🗋 Nuovo</strong> o <strong>📂 Apri</strong> un file <code>.s</code>, <code>.asm</code> o <code>.a65</code>. La colorazione della sintassi distingue istruzioni, direttive, etichette, numeri e commenti.</li>
<li><strong>Assemblare</strong>: nulla da fare, l'assemblaggio segue ogni modifica. In fondo al pannello compaiono la dimensione, gli indirizzi e la riga da digitare sulla macchina vera; gli errori sono mostrati con la loro riga e colonna.</li>
<li><strong>Eseguire</strong>: <strong>▶ Esegui</strong> mette i byte in memoria all'indirizzo di <code>.org</code> e digita il comando del Woz Monitor (<code>300R</code>).</li>
<li><strong>Fare il debug</strong>: <strong>⏺ Punto di arresto</strong> sulla riga del cursore, poi passo passo nel debugger (capitolo 8), con le vostre etichette nel disassemblato.</li>
<li><strong>Esportare</strong>: <strong>📼 Esporta WAV</strong> scrive la cassetta; sul vero Apple-1, digitate <code>C100R</code>, l'intervallo indicato (<code>0300.031FR</code>), poi <code>300R</code>.</li>
</ol>

![Un sorgente nell'editor: a sinistra il testo colorato, in basso il risultato dell'assemblaggio; a destra l'Apple-1 ha eseguito il programma.](img/it/brut/editeur-asm.png "large")

## Il listato e i simboli

**📃 Listato** mostra, per ogni riga, l'indirizzo, i byte prodotti e il sorgente; le righe prodotte da una macro sono marcate `+`, i file inclusi vengono espansi, e segue la tabella dei simboli. Un clic su una riga riporta al sorgente; un clic nel margine mette un punto di arresto. Ecco il listato di `hello.s` (capitolo 23), prodotto dalla riga di comando:

```text title="hello.lst"
0300  A2 00       start:  ldx #0
0302  BD 10 03    @next:  lda message,x   ; carattere seguente
0305  F0 06               beq @done       ; 0 = fine della stringa
0307  20 EF FF            jsr ECHO
030A  E8                  inx
030B  D0 F5               bne @next
030D  4C 1F FF    @done:  jmp GETLINE

                  message:
0310  8D                  .byte CR
0311  43 49 41 4F         .asciiz "CIAO, APPLE-1!"
0315  2C 20 41 50
…
; ---- Symboles (13)
; ECHO        $FFEF
; GETLINE     $FF1F
; message     $0310
; start       $0300
; start@done  $030D
; start@next  $0302
```

Le etichette locali `@next` e `@done` appartengono a `start`: la tabella le chiama `start@next` e `start@done`, e il debugger accetta questi nomi come indirizzi dei punti di arresto. (Il listato scritto dalla riga di comando conserva la sua intestazione francese, *Symboles*.)

## La riga di comando

{{asm (riga di comando)|`apple1-emu asm`}} fa lo stesso senza finestra:

```sh
apple1-emu asm hello.s                    # solo controllo
apple1-emu asm hello.s -l hello.lst       # con il listato
apple1-emu asm hello.s -o hello.wav       # cassetta (.wav o .aif)
apple1-emu asm hello.s -o hello.bin       # binario grezzo
apple1-emu asm hello.s -o hello.hex       # dump Woz, da incollare nel monitor
apple1-emu asm jeu.s -I ../common         # un'altra cartella per .include
```

Ogni comando riassume l'assemblaggio e dà la riga per il vero Apple-1. Il riepilogo è stampato in francese («32 byte … 2 file, 13 simboli / Cassetta (dopo C100R): 0300.031FR poi 300R / scritto: hello.wav»):

```text
32 octets, $0300-$031F ; 2 fichier(s), 13 symboles
Cassette (après C100R) : 0300.031FR   puis 300R
écrit : hello.wav
```

Senza `.org`, il programma viene collocato in `$0300`. Il dump `.hex` ha la forma che il Woz Monitor accetta dalla tastiera, e termina con il comando di avvio:

```text title="t1.hex (estratto)"
0280: 34 12 34 12 10 05 FF 34
0288: 12 FF FF A8 02 EA EA EA
…
0280R
```

## Sintassi

Una riga contiene, in quest'ordine e tutto facoltativo: un'etichetta, un'istruzione o una direttiva, e un commento dopo `;`. Mnemonici e direttive si possono scrivere in minuscolo o maiuscolo; i simboli distinguono maiuscole e minuscole.

```asm
PTR     = $06           ; una costante (nessun byte prodotto)
start:  lda #0          ; un'etichetta globale, seguita da due punti
@loop:  inx             ; un'etichetta locale: visibile fino alla globale successiva
        bne @loop
:       dex             ; un'etichetta anonima
        bne :-          ; :- la precedente, :+ la successiva, :-- quella prima ancora
```

{{!etichette locali}}{{!etichette anonime}}

**Numeri**: `255` in decimale, `$FF` in esadecimale, `%1010` in binario, `'A'` per un carattere ($41). La tastiera restituisce i suoi codici con il **bit 7 a uno**: per confrontare un tasto, scrivete `'A' | $80`; `apple1.inc` definisce allo stesso modo `CR = $8D`. `*` indica l'indirizzo corrente.

**Operatori**, dalla precedenza più bassa alla più alta:

| Precedenza | Operatori | Significato |
|---|---|---|
| 1 | <code>&#124;&#124;</code> | o logico (risultato 0 o 1) |
| 2 | `&&` | e logico |
| 3 | `=` `<>` `<` `>` `<=` `>=` | confronti (0 o 1) |
| 4 | `+` `-` <code>&#124;</code> | addizione, sottrazione, o bit a bit |
| 5 | `*` `/` `&` `^` `<<` `>>` | moltiplicazione, divisione, e, o esclusivo, scorrimenti |
| 6 (unari) | `<` `>` `-` `~` `!` | byte basso, byte alto, negazione, complemento, non logico |

Così `lda #<message` carica il byte basso dell'indirizzo e `lda #>message` il byte alto; `#>S+1` è `(>S)+1`. Non esiste un operatore modulo (`%` o `.mod` danno un errore di sintassi).

**Pagina zero o assoluto.** Un indirizzo sotto `$100` noto al momento dell'assemblaggio viene codificato in pagina zero (`lda $0006` dà `A5 06`, come con ca65). Per forzare la forma assoluta, scrivete `a:` (`lda a:$06` dà `AD 06 00`); per forzare la pagina zero, `z:`.

!!! attention "Definite le costanti di pagina zero prima di usarle"
    Se `PTR = $06` è scritto **dopo** `lda PTR`, l'assembler non conosce ancora il valore e codifica l'indirizzo su 2 byte. Lo segnala in giallo: `avviso: indirizzo in pagina zero codificato su 2 byte (simbolo definito più in basso; definitelo prima, o scrivete z: o a:)`. Il programma funziona, ma è più lungo, e i modi `(zp),Y` diventano impossibili.

## Direttive

| Direttiva | Effetto | Esempio |
|---|---|---|
| `.org ind` | indirizzo di assemblaggio (e di caricamento) | `.org $0300` |
| `.byte` (`.byt`) | byte, stringhe comprese | `.byte CR, "OK", 0` |
| `.word` (`.addr`) | parole a 16 bit, byte basso per primo | `.word 10000, end` |
| `.asciiz` | stringa terminata da zero | `.asciiz "CIAO"` |
| `.res n[, val]` | riserva n byte (0, o val) | `.res 3, $EA` |
| `.include "f"` | inserisce un file | `.include "apple1.inc"` |
| `.macro nome arg` … `.endmacro` | definisce una macro (`.mac`, `.endmac`) | vedete `macros.s` |
| `.local a, b` | etichette proprie di ogni chiamata della macro | |
| `.repeat n[, var]` … `.endrep` | ripete un blocco, var = 0 … n−1 | `.repeat 16, I` |
| `.if` `.elseif` `.else` `.endif` | assemblaggio condizionale | `.if WIDTH > 30` |
| `.ifdef` `.ifndef` | a seconda che un simbolo esista | `.ifdef APPLE1` |
| `.ifblank` `.ifnblank` | argomento della macro vuoto o no | |
| `.defined(sim)` | 1 se il simbolo esiste (in un'espressione) | `.if .defined(d)` |
| `.define nome valore` | sostituzione testuale (`.undefine`) | `.define CHARACTER '*'` |
| `.setcpu "65C02"` | accetta le istruzioni del 65C02 (`.pc02`, `.p02`) | |
| `.segment` `.code` `.data` `.rodata` `.bss` | accettate per compatibilità | |
| `.feature string_escapes` | `\n`, `\"`… nelle stringhe | |

{{!.org}}{{!.byte}}{{!.word}}{{!.asciiz}}{{!.res}}{{!.include}}{{!.macro}}{{!.repeat}}{{!.if}}{{!.setcpu}}

!!! attention "I segmenti non spostano nulla"
    Con ca65 e ld65, `.code`, `.data` e `.bss` mandano il codice in zone definite dalla configurazione del linker. L'assembler integrato produce **un solo blocco**, nell'ordine del sorgente: le direttive di segmento sono accettate ma non hanno effetto, e `.bss` produce zeri. Per un progetto che richiede un vero file `.cfg`, la libreria usa ca65 e ld65 (capitolo 4).

## Macro, .repeat e assemblaggio condizionale

Una macro è un modello di righe, copiato a ogni chiamata con i suoi argomenti. `.local` dà a ogni chiamata le sue etichette, senza conflitti. `.repeat` srotola un blocco al momento dell'assemblaggio, il che è comodo anche per **calcolare tabelle**:

```asm
quadrati:
.repeat 16, I
        .byte I * I     ; 0, 1, 4, 9 … 225
.endrep
```

Il capitolo 23 presenta `macros.s`, che usa tutto questo, e `gen2-lores.s`, che calcola gli indirizzi delle 24 righe dello schermo con `.repeat`.

## Messaggi di errore

Ogni errore indica il file, la riga, la colonna e il testo incriminato; finché resta un errore non viene scritto alcun byte. L'editor mostra i messaggi nella lingua dell'interfaccia (qui sotto, in italiano); la riga di comando li stampa in francese.

| Messaggio | Causa abituale |
|---|---|
| `errore di sintassi` | operatore sconosciuto, parentesi mancante |
| `istruzione, direttiva o macro sconosciuta` | errore di battitura; istruzione del 65C02 senza `.setcpu`; etichetta senza due punti |
| `modo di indirizzamento impossibile per questa istruzione` | `sta #5`, `ldx ($06),y`… |
| `simbolo non definito` | `jsr ECO` invece di `ECHO`; `.include` dimenticato |
| `simbolo già definito` | due etichette con lo stesso nome |
| `valore fuori intervallo` | `lda #300`, indirizzo oltre `$FFFF` |
| `salto troppo lontano (da -128 a +127 byte)` | sostituite `bne lontano` con `beq :+` / `jmp lontano` / `:` |
| `file .include non trovato` | nome, o cartella: aggiungete `-I` |
| `blocco chiuso male (.macro/.endmacro, .if/.endif)` | `.endif` o `.endmacro` dimenticato |
| `divisione per zero` | in un'espressione |
| `stringa o carattere senza virgoletta di chiusura` | `"CIAO` |
| `valore sconosciuto in questo punto (.org, .res, .if, .repeat)` | queste direttive richiedono un valore già definito |

![Un errore nell'editor: ECO invece di ECHO. Un clic sul messaggio porta alla riga.](img/it/brut/editeur-asm-erreur.png "large")

## Compatibilità con ca65 e xa

L'assembler integrato segue la sintassi di **ca65** (pacchetto cc65) per tutto ciò che serve a un programma di un solo blocco. Gli otto esempi del capitolo 23 sono stati assemblati nella versione originale con i due strumenti e confrontati byte per byte. Le versioni italiane differiscono solo nei commenti e nei testi; la loro dimensione con l'assembler integrato è nell'ultima colonna (non sono state riconfrontate con ca65):

| Sorgente | Byte (originale) | Integrato = ca65 | Byte (italiano) |
|---|---|---|---|
| `hello.s` | 33 | <span class="status-ok">identico</span> | 32 |
| `clavier.s` | 32 | <span class="status-ok">identico</span> | 32 |
| `hexa.s` | 64 | <span class="status-ok">identico</span> | 64 |
| `decimal.s` | 117 | <span class="status-ok">identico</span> | 117 |
| `jeu.s` | 222 | <span class="status-ok">identico</span> | 240 |
| `macros.s` | 135 | <span class="status-ok">identico</span> | 132 |
| `gen2-lores.s` | 110 | <span class="status-ok">identico</span> | 110 |
| `gen2-hires.s` | 94 | <span class="status-ok">identico</span> | 94 |

I sorgenti scritti per **xa** (xa65) sono riconosciuti dalle loro righe `#include` o `#define`, e tradotti riga per riga prima dell'assemblaggio (i numeri di riga degli errori restano giusti): `#define`, `#include`, `*=`, `.BYT`, `.ASC`, commenti `/* */` ed etichette in colonna 0 senza due punti. La presentazione di Dave Schmenk (capitolo 9) viene assemblata così, negli stessi 3456 byte di xa 2.4.1.

Per i **progetti GitHub**, la libreria preferisce ca65 e ld65 se sono installati (o se il progetto fornisce una configurazione ld65), con `-D APPLE1` per impostazione predefinita; altrimenti usa l'assembler integrato. Il file `apple1.toml` del repository indica gli indirizzi e i simboli (capitolo 4).
