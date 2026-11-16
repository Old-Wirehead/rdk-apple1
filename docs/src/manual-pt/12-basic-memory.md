# Variáveis, cadeias, tabelas e memória

## Números

O Integer BASIC só conhece **inteiros de 16 bits com sinal**, de −32767 a 32767. Um cálculo que sai deste intervalo para com `*** >32767 ERR` (`200*200`, `32767+1`, e também `10/0` e `RND(0)`). O valor −32768 não é utilizável (capítulo 18, erro 5). Não há vírgula decimal nem `INT`: `7/2` dá 3.

## Nomes de variável

É o limite que mais surpreende: um nome de variável é **uma letra, seguida opcionalmente de um algarismo**.

| Nome | Resultado (experimentado) |
|---|---|
| `A`, `Z`, `B1`, `Z9` | aceite |
| `AB`, `CNT`, `SCORE`, `COUNTER` | `SYNTAX ERR` |
| `A12` | `SYNTAX ERR` |
| `A$`, `N$` | cadeia: **uma só letra** |
| `B9$`, `Z9$` | `SYNTAX ERR` |

Isto dá 26 × 11 = 286 variáveis inteiras possíveis, e 26 cadeias. Habitue-se a anotar o papel de cada variável num `REM`.

## Cadeias

Uma cadeia tem de ser **dimensionada** antes de ser usada, com o seu comprimento máximo (1 a 255): `DIM N$(20)`. `DIM B$(256)` dá `>255 ERR`.

| Operação | Escreve-se | Exemplo (N$ = "RADAR") |
|---|---|---|
| comprimento | `LEN(N$)` | 5 |
| subcadeia | `N$(i,j)`: caracteres i a j | `N$(1,3)` = "RAD" |
| fim | `N$(i)`: do carácter i até ao fim | `N$(4)` = "AR" |
| concatenar | `A$(LEN(A$)+1)=B$` | "APPLE" + "-1" → "APPLE-1" |
| comparar | só `=` e `#` | `IF N$="WOZ" THEN …` |

Atribuir uma cadeia mais comprida do que o seu `DIM` dá `*** STR OVFL ERR`, tal como uma resposta demasiado longa ao `INPUT`. Não há `MID$`, `LEFT$`, `CHR$`, `STR$`, `VAL` nem `ASC` (este último está na tabela mas não funciona).

!!! astuce "Substituir CHR$ e VAL"
    Para mostrar um carácter a partir do seu código, tenha uma cadeia de referência e corte-a: `DIM C$(26):C$="ABCDEFGHIJKLMNOPQRSTUVWXYZ"` e depois `PRINT C$(K,K)`. Para ler um número, use `INPUT N` em vez de `INPUT N$`.

## Tabelas

`DIM T(10)` cria uma tabela de 10 inteiros, **numerados de 1 a 10**. `T(0)` e `T(11)` dão `*** RANGE ERR`. Há uma só dimensão: para uma grelha de 8 × 8, use `G((L-1)*8+C)`. No emulador, os elementos valem 0 depois do `DIM`; numa máquina verdadeira a memória não é necessariamente apagada: inicialize as suas tabelas. Uma tabela demasiado grande para a memória dá `*** MEM FULL ERR` (`DIM A(3000)` em 8 KB).

## A memória do BASIC

![Memória do Apple-1: o BASIC em $E000, o programa abaixo do HIMEM, as variáveis acima do LOMEM.](diagrams/pt/memoire.svg "diagram large")

O BASIC guarda o **programa no topo** da memória, a crescer para baixo a partir do HIMEM, e as **variáveis em baixo**, a crescer para cima a partir do LOMEM. O espaço livre fica entre os dois.

| Ponteiro | Endereço | Valor com 8 KB | Papel |
|---|---|---|---|
| LOMEM | `$4A-$4B` (74-75) | `$0800` (2048) | início das variáveis |
| HIMEM | `$4C-$4D` (76-77) | `$1000` (4096) | fim da memória do BASIC |
| PP | `$CA-$CB` (202-203) | desce com o programa | início do programa |
| PV | `$CC-$CD` (204-205) | sobe com as variáveis | fim das variáveis |

{{!LOMEM}}{{!HIMEM}}{{!PP (ponteiro do programa)}}

`PRINT PEEK(202)+PEEK(203)*256` dá o início do programa e `PRINT PEEK(204)+PEEK(205)*256` o fim das variáveis: o espaço livre é a diferença entre os dois. Uma variável inteira como `A` ocupa 6 bytes.

!!! apple1 "Porque é que uma cassete BASIC começa em $004A"
    Estes ponteiros estão na página zero, de `$4A` a `$FF`. Uma cassete BASIC grava, portanto, **dois intervalos**: `004A.00FF` (os ponteiros), depois o programa de PP ao HIMEM, por exemplo `0F2C.0FFF`. O comando `apple1-emu basic` calcula-os por si (a sua saída é em francês: «10 linhas, 212 bytes… 1836 bytes livres para as variáveis… intervalos ACI (depois de C100R)… depois E2B3R e RUN»):

    ```text
    10 lignes, 212 octets ; LOMEM $0800 HIMEM $1000 PP $0F2C ; 1836 octets libres pour les variables ; RAM 8 Ko
    Plages ACI (après C100R) : 004A.00FFR 0F2C.0FFFR   puis E2B3R et RUN
    ```

### Mudar o LOMEM e o HIMEM

A sintaxe é `LOMEM=n` e `HIMEM=n` (com `=`, **não** `:` como no Apple II, que dá `SYNTAX ERR`).

- `LOMEM=2304` desloca o início das variáveis e **apaga as variáveis** (o programa fica). Útil para reservar `$0800-$08FF` para um programa em código máquina.
- `HIMEM=` **mais alto** (com mais RAM, por exemplo `HIMEM=8192` com 16 KB) funciona e mantém o programa.
- `HIMEM=` **mais baixo** é **ignorado em silêncio** pelo original, mesmo depois de `SCR` (capítulo 18, erro 11). **O RDK G2 Basic aceita-o**: o programa na memória é deslocado para baixo com o limite, ou dá `MEM FULL ERR` se já não couber acima das variáveis (`RANGE ERR` para um valor negativo; `LOMEM=` dá `RANGE ERR` se não ficar abaixo do início do programa, e `>32767 ERR` acima de 32767). Com o original há um contorno verificado: escrever diretamente o ponteiro, depois `SCR`:

```basic title="Baixar o HIMEM para 3072 ($0C00)"
POKE 76,0 : POKE 77,12 : SCR
```

No arranque do G2 Basic, o HIMEM é o topo da RAM encontrada e o LOMEM é `$0800` (`$2000` com um controlador GEN2). (`76`/`77` = `$4C`/`$4D`, 12 × 256 = 3072.) Escreva estas três instruções numa linha numerada ou separadamente em modo direto, **antes** de introduzir o programa: o `SCR` apaga-o.

## Misturar BASIC e código máquina

| Zona (8 KB) | Utilizável para código 6502? |
|---|---|
| `$0000-$00FF` | página zero: o Woz Monitor usa `$24-$2B`, o BASIC pelo menos `$4A-$FF`; evite-a, ou guarde-a e restaure-a |
| `$0100-$01FF` | pilha do 6502: não |
| `$0200-$027F` | buffer de linha do Woz Monitor; reescrito por cada linha escrita |
| `$0280-$07FF` | livre enquanto o BASIC não lhe tocar: o melhor sítio |
| `$0800-…` | variáveis do BASIC (salvo se o LOMEM for subido) |
| `… -$0FFF` | programa BASIC |

O programa em código máquina é carregado de cassete ou por POKE, e chamado com `CALL 768` (para `$0300`). Tem de terminar com `RTS` e respeitar a página zero do BASIC. Exemplo mínimo:

```mon
>» POKE 768,96
>» CALL 768
>
```

(96 = `$60` = `RTS`: a chamada volta de imediato.)
