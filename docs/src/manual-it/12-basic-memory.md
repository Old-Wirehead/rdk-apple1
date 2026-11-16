# Variabili, stringhe, vettori e memoria

## Numeri

L'Integer BASIC conosce solo **interi a 16 bit con segno**, da −32767 a 32767. Un calcolo che esce da questo intervallo si ferma con `*** >32767 ERR` (`200*200`, `32767+1`, e anche `10/0` e `RND(0)`). Il valore −32768 non è utilizzabile (capitolo 18, bug 5). Non c'è virgola decimale né `INT`: `7/2` fa 3.

## Nomi delle variabili

È il limite che sorprende di più: un nome di variabile è **una lettera, seguita facoltativamente da una cifra**.

| Nome | Risultato (provato) |
|---|---|
| `A`, `Z`, `B1`, `Z9` | accettato |
| `AB`, `CNT`, `SCORE`, `COUNTER` | `SYNTAX ERR` |
| `A12` | `SYNTAX ERR` |
| `A$`, `N$` | stringa: **una sola lettera** |
| `B9$`, `Z9$` | `SYNTAX ERR` |

Fanno 26 × 11 = 286 variabili intere possibili, e 26 stringhe. Prendete l'abitudine di annotare il ruolo di ogni variabile in un `REM`.

## Stringhe

Una stringa va **dimensionata** prima dell'uso, con la sua lunghezza massima (da 1 a 255): `DIM N$(20)`. `DIM B$(256)` dà `>255 ERR`.

| Operazione | Si scrive | Esempio (N$ = "RADAR") |
|---|---|---|
| lunghezza | `LEN(N$)` | 5 |
| sottostringa | `N$(i,j)`: caratteri da i a j | `N$(1,3)` = "RAD" |
| coda | `N$(i)`: dal carattere i alla fine | `N$(4)` = "AR" |
| concatenare | `A$(LEN(A$)+1)=B$` | "APPLE" + "-1" → "APPLE-1" |
| confrontare | solo `=` e `#` | `IF N$="WOZ" THEN …` |

Assegnare una stringa più lunga del suo `DIM` dà `*** STR OVFL ERR`, come una risposta troppo lunga a `INPUT`. Non ci sono `MID$`, `LEFT$`, `CHR$`, `STR$`, `VAL` né `ASC` (quest'ultimo è nella tabella ma non funziona).

!!! astuce "Sostituire CHR$ e VAL"
    Per stampare un carattere a partire dal suo codice, tenete una stringa di riferimento e tagliatela: `DIM C$(26):C$="ABCDEFGHIJKLMNOPQRSTUVWXYZ"` poi `PRINT C$(K,K)`. Per leggere un numero, usate `INPUT N` invece di `INPUT N$`.

## Vettori

`DIM T(10)` crea un vettore di 10 interi, **numerati da 1 a 10**. `T(0)` e `T(11)` danno `*** RANGE ERR`. C'è una sola dimensione: per una griglia di 8 × 8, usate `G((L-1)*8+C)`. Nell'emulatore gli elementi valgono 0 dopo `DIM`; su una macchina vera la memoria non viene necessariamente azzerata: inizializzate i vostri vettori. Un vettore troppo grande per la memoria dà `*** MEM FULL ERR` (`DIM A(3000)` in 8 KB).

## La memoria del BASIC

![Memoria dell'Apple-1: il BASIC in $E000, il programma sotto HIMEM, le variabili sopra LOMEM.](diagrams/it/memoire.svg "diagram large")

Il BASIC mette il **programma in alto** nella memoria, crescendo verso il basso da HIMEM, e le **variabili in basso**, crescendo verso l'alto da LOMEM. Lo spazio libero sta in mezzo.

| Puntatore | Indirizzo | Valore con 8 KB | Ruolo |
|---|---|---|---|
| LOMEM | `$4A-$4B` (74-75) | `$0800` (2048) | inizio delle variabili |
| HIMEM | `$4C-$4D` (76-77) | `$1000` (4096) | fine della memoria del BASIC |
| PP | `$CA-$CB` (202-203) | scende con il programma | inizio del programma |
| PV | `$CC-$CD` (204-205) | sale con le variabili | fine delle variabili |

{{!LOMEM}}{{!HIMEM}}{{!PP (puntatore del programma)}}

`PRINT PEEK(202)+PEEK(203)*256` dà l'inizio del programma e `PRINT PEEK(204)+PEEK(205)*256` la fine delle variabili: lo spazio libero è la differenza tra i due. Una variabile intera come `A` occupa 6 byte.

!!! apple1 "Perché una cassetta BASIC comincia in $004A"
    Questi puntatori sono in pagina zero, da `$4A` a `$FF`. Una cassetta BASIC registra quindi **due intervalli**: `004A.00FF` (i puntatori), poi il programma da PP a HIMEM, per esempio `0F2C.0FFF`. Il comando `apple1-emu basic` li calcola per voi (il suo output è in francese: «10 righe, 212 byte… 1836 byte liberi per le variabili… intervalli ACI (dopo C100R)… poi E2B3R e RUN»):

    ```text
    10 lignes, 212 octets ; LOMEM $0800 HIMEM $1000 PP $0F2C ; 1836 octets libres pour les variables ; RAM 8 Ko
    Plages ACI (après C100R) : 004A.00FFR 0F2C.0FFFR   puis E2B3R et RUN
    ```

### Cambiare LOMEM e HIMEM

La sintassi è `LOMEM=n` e `HIMEM=n` (con `=`, **non** `:` come sull'Apple II, che dà `SYNTAX ERR`).

- `LOMEM=2304` sposta l'inizio delle variabili e **cancella le variabili** (il programma resta). Utile per riservare `$0800-$08FF` a un programma in linguaggio macchina.
- `HIMEM=` **più alto** (con più RAM, per esempio `HIMEM=8192` con 16 KB) funziona e conserva il programma.
- `HIMEM=` **più basso** viene **ignorato in silenzio** dall'originale, anche dopo `SCR` (capitolo 18, bug 11). **RDK G2 Basic lo accetta**: il programma in memoria viene spostato in basso insieme al limite, oppure si ottiene `MEM FULL ERR` se non starebbe più sopra le variabili (`RANGE ERR` per un valore negativo; `LOMEM=` dà `RANGE ERR` se non è sotto l'inizio del programma, e `>32767 ERR` sopra 32767). Con l'originale c'è un aggiramento verificato: scrivere direttamente il puntatore, poi `SCR`:

```basic title="Abbassare HIMEM a 3072 ($0C00)"
POKE 76,0 : POKE 77,12 : SCR
```

All'avvio di G2 Basic, HIMEM è il limite superiore della RAM trovata e LOMEM è `$0800` (`$2000` con un driver GEN2). (`76`/`77` = `$4C`/`$4D`, 12 × 256 = 3072.) Digitate queste tre istruzioni su una riga numerata o separatamente in modo diretto, **prima** di inserire il programma: `SCR` lo cancella.

## Mescolare BASIC e linguaggio macchina

| Zona (8 KB) | Utilizzabile per codice 6502? |
|---|---|
| `$0000-$00FF` | pagina zero: il Woz Monitor usa `$24-$2B`, il BASIC almeno `$4A-$FF`; evitatela, o salvatela e ripristinatela |
| `$0100-$01FF` | stack del 6502: no |
| `$0200-$027F` | buffer di riga del Woz Monitor; sovrascritto da ogni riga digitata |
| `$0280-$07FF` | libera finché il BASIC non la tocca: il posto migliore |
| `$0800-…` | variabili del BASIC (salvo LOMEM alzato) |
| `… -$0FFF` | programma BASIC |

Il programma in linguaggio macchina si carica da cassetta o con POKE, e si chiama con `CALL 768` (per `$0300`). Deve terminare con `RTS` e rispettare la pagina zero del BASIC. Esempio minimo:

```mon
>» POKE 768,96
>» CALL 768
>
```

(96 = `$60` = `RTS`: la chiamata ritorna subito.)
