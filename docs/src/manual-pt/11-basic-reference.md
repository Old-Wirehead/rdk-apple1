# Referência das instruções e funções

<p class="lead">Todas as instruções reconhecidas pelo Integer BASIC do Apple-1, tiradas da tabela de palavras-chave do interpretador, com o que fazem de facto. Todos os exemplos foram experimentados.</p>

## Instruções

`PRINT` lista `[;]`
{: .syntax }

{{PRINT}} mostra números e cadeias. `;` junta os elementos sem espaço (`PRINT A;B` mostra `12` para A=1, B=2); `,` passa à coluna seguinte, de 8 em 8 (`PRINT 1,2,3` mostra `1       2       3`). Um `;` final suprime a mudança de linha. `PRINT` sozinho passa à linha seguinte.

`INPUT` `["pedido",]` variável `[,variável…]`
{: .syntax }

{{INPUT}} lê um número ou uma cadeia. O BASIC mostra sempre `?` depois do pedido: `INPUT "NOME",N$` mostra `NOME?`. Uma cadeia demasiado longa para o seu `DIM` dá `STR OVFL ERR`.

`LET` variável `=` expressão
{: .syntax }

{{LET}} é facultativo: `A=5` basta.

`IF` expressão `THEN` instrução | número
{: .syntax }

{{IF}} executa a instrução quando a expressão não é zero (`IF A THEN …` é permitido). `THEN 100` salta para a linha 100. **Atenção**: só a primeira instrução depende da condição (capítulo 18, erro 4).

`GOTO` expressão — `GOSUB` expressão — `RETURN`
{: .syntax }

{{GOTO}} e {{GOSUB}} aceitam uma **expressão**: `GOTO K*100`, `GOSUB 300+K` (o «GOTO calculado» substitui o `ON … GOTO` que falta). Uma linha inexistente dá `BAD BRANCH ERR`. No máximo oito {{GOSUB}} encaixados (`>8 GOSUBS ERR` no nono); `RETURN` sem `GOSUB` dá `BAD RETURN ERR`.

`FOR` var `=` início `TO` fim `[STEP passo]` … `NEXT` var
{: .syntax }

{{FOR}}/{{NEXT}}: o corpo é executado **pelo menos uma vez**, mesmo que o início esteja para lá do fim (`FOR I=5 TO 1` passa uma vez). `STEP` pode ser negativo. No máximo oito ciclos encaixados (`>8 FORS ERR`). `NEXT` tem de nomear a variável; um `NEXT` sem `FOR` dá `BAD NEXT ERR`.

`DIM` var`(n)` `[,var$(n)…]`
{: .syntax }

{{DIM}} cria uma tabela de `n` inteiros, numerados **de 1 a n** (`A(0)` dá `RANGE ERR`), ou reserva `n` caracteres (no máximo 255) para uma cadeia. Todas as cadeias têm de ser dimensionadas antes de lhes atribuir um valor.

`REM` texto
{: .syntax }

{{REM}}: um comentário, até ao fim da linha (um `:` depois de REM faz parte do comentário).

`END`
{: .syntax }

{{END}} termina o programa. Um programa que acaba sem `END` mostra `*** END ERR`: não faz mal, mas termine os seus programas com `END`.

`TAB` n
{: .syntax }

{{TAB}} é uma **instrução** (não uma função do PRINT como no Applesoft): `TAB 10:PRINT "*"` põe o asterisco na coluna 10.

`POKE` endereço`,`valor
{: .syntax }

{{POKE}} escreve um byte (0 a 255, senão `>255 ERR`). Os endereços acima de 32767 escrevem-se como números negativos: endereço − 65536.

`CALL` endereço
{: .syntax }

{{CALL}} chama um programa em código máquina, que volta com `RTS`. Exemplo: `POKE 768,96 : CALL 768` (96 = `RTS`). `CALL -256` volta de forma limpa ao Woz Monitor (`$FF00`).

## Funções

| Função | Resultado | Exemplo verificado |
|---|---|---|
| {{PEEK}}`(end)` | o byte no endereço | `PEEK(76)+PEEK(77)*256` dá o HIMEM (4096) |
| {{RND}}`(n)` | inteiro aleatório de 0 a n−1; `RND(0)`: `>32767 ERR`; `RND(-5)` dá um número negativo | `RND(6)+1`: um dado |
| {{SGN}}`(x)` | −1, 0 ou 1 (também para um número de 256 ou mais) | `SGN(-5);SGN(0)` mostra `-10` |
| {{ABS}}`(x)` | valor absoluto | `ABS(-9)` = 9 |
| `KEY(n)` (só G2 Basic) | código da tecla carregada, sem esperar (ver abaixo) | `K=KEY(0)` |
| {{LEN}}`(a$)` | comprimento da cadeia | `LEN("ABC")` = 3 |
| {{USR}}`(x)` | chama `$0000`: **bloqueia** sem um programa preparado | — |
| {{ASC}}`(`, {{SCRN}}`(` | presentes na tabela, mas `SYNTAX ERR` | — |

## Operadores

| Prioridade | Operadores |
|---|---|
| 1 | `-` e `+` unários, `NOT` |
| 2 | `*` `/` `MOD` |
| 3 | `+` `-` |
| 4 | `=` `#` (diferente, também `<>`) `<` `>` `<=` `>=` |
| 5 | `AND` |
| 6 | `OR` |

{{MOD}}, {{AND}}, {{OR}} e {{NOT}}:

- A divisão **trunca em direção a zero**: `7/2` = 3, `-7/2` = −3. `MOD` toma o sinal do dividendo: `-7 MOD 3` = −1.
- As comparações dão 1 (verdadeiro) ou 0 (falso): `PRINT 5>3;5=3` mostra `10`. `NOT 0` = 1, `NOT 5` = 0.
- `AND` e `OR` são lógicos, não bit a bit: `3 AND 0` = 0, `3 OR 0` = 1.
- `3*-2` é aceite e vale −6.
- `^` existe na tabela, mas **bloqueia a máquina** (capítulo 18, erro 2).
- As cadeias só se podem comparar com `=` e `#`; `"ABC"<"ABD"` é recusado (`SYNTAX ERR`).

## Extensões do RDK G2 Basic

Só funcionam com o **RDK G2 Basic** (o Integer BASIC original responde `SYNTAX ERR`).

`KEY(n)`
{: .syntax }

`KEY(0)` devolve o **código da tecla** carregada (o código ASCII de 7 bits: 65 para `A`, 13 para Return, 32 para a barra de espaços) e consome-a, ou `0` se nenhuma tecla foi carregada. Nunca espera, por isso um programa pode continuar a correr enquanto vigia o teclado. `KEY(1)` (qualquer outro argumento diferente de zero) **espera** por uma tecla.

```basic title="Esperar por uma tecla sem INPUT"
10 K=KEY(0)
20 IF K=0 THEN 10
30 PRINT K
```

No original, uma tecla carregada enquanto um programa corre **para-o** (`STOPPED AT n`), e o `INPUT` bloqueia até ao Return. Com o G2 Basic, assim que um programa chama `KEY`, uma tecla deixa de o parar: só o `KEY` vê o teclado, e um <kbd>Ctrl</kbd>+<kbd>C</kbd> (código 3) lido pelo `KEY` para o programa na chamada seguinte. Chame, portanto, `KEY` no ciclo principal do programa.

`LOMEM=` n — `HIMEM=` n
{: .syntax }

Comandos imediatos que aceitam uma expressão (`LOMEM=2048+512`); dentro de um programa dão `SYNTAX ERR`, como no original. São descritos com a memória no capítulo 16.

`LIST` `[`a`[,`b`]]` — `DEL` a`[,`b`]`
{: .syntax }

`LIST` lista tudo, `LIST n` uma linha, `LIST a,b` as linhas de `a` a `b`. `DEL n` apaga uma linha, `DEL a,b` as linhas de `a` a `b` (se `b` for menor do que `a`, só a linha `a`). Uma linha listada mais comprida do que o ecrã muda de linha como no original (as linhas de continuação são recuadas seis espaços).

As subcadeias seguem o original: `A$(I)` com `I` de 256 ou mais, ou negativo, dá `>255 ERR`, e `A$(I,J)` com `J` menor do que `I-1` dá `STRING ERR`.

## Palavras-chave que não fazem nada

A tabela contém `COLOR=`, `PLOT`, `HLIN`, `VLIN` e `VTAB`, herdadas do projeto de BASIC para o Apple II. No Apple-1, `COLOR=` e `PLOT` são aceites e não fazem nada, `VTAB` dá `SYNTAX ERR`. Deixe-as fora dos seus programas.
