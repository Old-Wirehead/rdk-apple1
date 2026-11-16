# Mensagens de erro e limites

## Mensagens

Um erro mostra `*** MENSAGEM ERR`, seguido, quando um programa está a correr, de `STOPPED AT n`. Uma linha recusada à entrada não é guardada: escreva-a de novo.

| Mensagem | Causa | Exemplo experimentado |
|---|---|---|
| {{SYNTAX ERR}} | instrução desconhecida, nome de variável inválido, `:` em modo direto | `CNT=7`, `GET A$`, `X=5:PRINT X` em modo direto |
| {{>32767 ERR}} | transbordo, divisão por zero, número de linha > 32767 | `200*200`, `10/0`, `RND(0)`, `32768 PRINT` |
| {{TOO LONG ERR}} | linha demasiado complexa (expressão demasiado encaixada) ou demasiado longa | 13 parênteses encaixados |
| {{MEM FULL ERR}} | já não há espaço entre as variáveis e o programa | `DIM A(3000)` em 8 KB |
| {{TOO MANY PARENS ERR}} | parênteses (previsto pelo interpretador; mascarado pelo erro 3) | — |
| {{STRING ERR}} | cadeia mal usada | — |
| {{END ERR}} | o programa acaba sem `END` | `10 PRINT "X"` e depois `RUN` |
| {{BAD BRANCH ERR}} | `GOTO`/`GOSUB` para uma linha inexistente | `GOTO 500` |
| {{>8 GOSUBS ERR}} | mais de 8 `GOSUB` encaixados | recursão: 9 chamadas |
| {{BAD RETURN ERR}} | `RETURN` sem `GOSUB` | `RUN 30` sobre um `RETURN` |
| {{>8 FORS ERR}} | mais de 8 `FOR` encaixados, ou o erro 1 | 9 ciclos numa linha |
| {{BAD NEXT ERR}} | `NEXT` sem o `FOR` correspondente | `20 NEXT I` sozinho |
| {{STOPPED AT}} | foi carregada uma tecla durante a execução | — |
| {{>255 ERR}} | valor > 255: `POKE`, `DIM` de uma cadeia | `POKE 768,300`, `DIM B$(256)` |
| {{RANGE ERR}} | índice de tabela fora dos limites | `A(0)`, `A(3)` depois de `DIM A(2)` |
| {{DIM ERR}} | tabela ou cadeia dimensionada de novo | — |
| {{STR OVFL ERR}} | cadeia mais comprida do que o seu `DIM` | `A$="ABCDE"` com `DIM A$(3)` |
| {{RETYPE LINE}} | resposta inválida ao `INPUT` (letras para um número): escreva de novo | — |

As linhas com «—» vêm da tabela de mensagens do interpretador; não foram provocadas nos nossos testes.

## Limites

| Limite | Valor |
|---|---|
| inteiros | −32767 a 32767 |
| números de linha | 0 a 32767 |
| nomes de variável | letra + algarismo opcional |
| nomes de cadeia | uma letra + `$` |
| comprimento de uma cadeia | 255 |
| dimensões de uma tabela | 1, índices de 1 a n |
| `FOR` encaixados | 8 |
| `GOSUB` encaixados | 8 |
| parênteses encaixados | 12 |
| comprimento de uma linha escrita | 120 caracteres passam, 141 são recusados |


## Poupar memória

Com 8 KB, o BASIC tem cerca de 2 KB para o programa e as variáveis. Algumas maneiras de ganhar espaço:

<ol class="steps">
<li><strong>Baixar o LOMEM</strong> se não tiver código máquina: <code>LOMEM=768</code> (<code>$0300</code>) dá mais 1280 bytes às variáveis. Experimentado: <code>DIM T(1200)</code> falha com <code>MEM FULL</code> com o LOMEM por omissão, e funciona depois de <code>LOMEM=768</code>. As adaptações de *101 BASIC Computer Games* feitas durante o desenvolvimento fazem exatamente isso.</li>
<li><strong>Escrever de forma compacta</strong>: o BASIC guarda os espaços e os <code>REM</code>. O <code>LIST</code> volta a indentar sozinho, e uma linha longa com várias instruções custa menos do que várias linhas.</li>
<li><strong>Reutilizar tabelas e cadeias</strong> em vez de dimensionar novas.</li>
<li><strong>Acrescentar RAM</strong>: as definições ⚙ de um programa, <code>--ram 16</code>, ou uma expansão verdadeira. Um programa com mais de 8 KB exige uma máquina de 12, 16 ou 32 KB: <code>apple1-emu basic --ram 16</code> coloca-o no topo dessa memória.</li>
</ol>
