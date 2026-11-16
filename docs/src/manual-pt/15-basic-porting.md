# Adaptar um programa de Applesoft ou Microsoft BASIC

<p class="lead">A maioria das listagens dos anos 1970 e 1980 está escrita em Microsoft BASIC (Applesoft, o BASIC de «101 BASIC Computer Games»). Podem ser adaptadas ao Apple-1, desde que se conheçam as equivalências. Cada linha indicada abaixo como «recusada» foi escrita no BASIC verdadeiro.</p>

## Equivalências

| Applesoft / Microsoft | Integer BASIC do Apple-1 | Nota |
|---|---|---|
| `ON X GOTO 100,200,300` | `GOTO 100*X` (se os números de linha o permitirem) ou uma série de `IF X=1 THEN 100` | recusada |
| `GET A$` | `INPUT A$` | recusada |
| `DATA` / `READ` / `RESTORE` | tabela preenchida por atribuições, ou uma cadeia cortada | recusada |
| `MID$(A$,I,N)` | `A$(I,I+N-1)` | recusada |
| `LEFT$(A$,N)` / `RIGHT$(A$,N)` | `A$(1,N)` / `A$(LEN(A$)-N+1)` | — |
| `A$+B$` | `A$(LEN(A$)+1)=B$` | — |
| `CHR$(N)`, `ASC(A$)` | cadeia de referência (capítulo 18, erro 6) | recusada |
| `STR$(N)`, `VAL(A$)` | `PRINT N` diretamente; `INPUT N` | recusada |
| `INT(X)` | inútil: tudo é inteiro | recusada |
| `RND(1)` (real de 0 a 1) | `RND(N)` (inteiro de 0 a N−1) | — |
| `X^2` | `X*X` | bloqueia (erro 2) |
| `HOME` | 24 × `PRINT`, ou CLEAR SCREEN (<kbd>Ctrl</kbd>+<kbd>L</kbd>) à mão | recusada |
| `PRINT TAB(10);"X"` | `TAB 10:PRINT "X"` | — |
| `STOP` | `END` | recusada |
| `DEF FN` | `GOSUB` | — |
| `A<>B` | `A#B` (`<>` também é aceite) | — |
| `IF A THEN X:Y` | condição invertida + salto (erro 4) | armadilha |
| `DIM A(10,10)` | `DIM A(121)` e `A(I*11+J+1)` | — |
| `A(0)` | os índices começam em 1 | `RANGE ERR` |
| `SCORE`, `NAME$` | `S`, `N$`: uma letra (+ algarismo) | `SYNTAX ERR` |
| números reais | inteiros ×10 ou ×100 (vírgula fixa) | — |

{{!adaptação}}

## O método

<ol class="steps">
<li><strong>Faça a lista das variáveis</strong> do programa original e atribua a cada uma uma letra (+ algarismo). Registe a tabela em <code>REM</code> no início.</li>
<li><strong>Substitua os DATA</strong> por atribuições numa sub-rotina de inicialização, ou por uma cadeia que se corta.</li>
<li><strong>Encontre cada <code>IF … THEN … : …</code></strong> e reescreva-o com um salto (erro 4).</li>
<li><strong>Encontre cada saída de um <code>FOR</code> por <code>GOTO</code></strong> e substitua-a por «valor final + <code>NEXT</code>» (erro 1).</li>
<li><strong>Desloque os índices das tabelas</strong> em 1 se o programa usar o índice 0.</li>
<li><strong>Verifique no editor</strong>: assinala, enquanto escreve, as palavras que o Integer BASIC não conhece. <strong>▶ Executar</strong> escreve o programa: qualquer linha recusada aparece no ecrã com <code>SYNTAX ERR</code>.</li>
<li><strong>Meça a memória</strong>: <code>apple1-emu basic prog.bas</code> dá o tamanho e a RAM necessária. Grande demais para 8 KB? <code>LOMEM=768</code> (capítulo 17), ou uma máquina de 16 KB.</li>
<li><strong>Exporte</strong> a cassete (<strong>📼 Exportar WAV</strong>) e anote os intervalos para a máquina verdadeira.</li>
</ol>

## Um exemplo de adaptação

Um fragmento típico em Microsoft BASIC:

```text title="Original (Microsoft BASIC)"
100 READ N$,S
110 IF S>HI THEN HI=S:BEST$=N$
120 IF N$<>"END" THEN 100
130 PRINT "MELHOR: ";BEST$;" (";HI;")"
140 DATA "ANA",12,"BRUNO",17,"CLARA",9,"END",0
```

No Apple-1: nem `READ`/`DATA`, nem nomes longos, e o `IF` da linha 110 atribuiria `BEST$` a cada passagem (erro 4).

```basic title="Adaptação em Integer BASIC (experimentada)"
10 DIM N$(10),B$(10),S(3)
20 S(1)=12:S(2)=17:S(3)=9
30 H=0
40 FOR I=1 TO 3
50 GOSUB 200
60 IF S(I)<=H THEN 80
70 H=S(I):B$=N$
80 NEXT I
90 PRINT "MELHOR: ";B$;" (";H;")"
100 END
200 IF I=1 THEN N$="ANA"
210 IF I=2 THEN N$="BRUNO"
220 IF I=3 THEN N$="CLARA"
230 RETURN
```

```mon
>» RUN
MELHOR: BRUNO (17)
```

## Lições das adaptações

Oito jogos de *101 BASIC Computer Games* (DEC, 1975) foram adaptados assim durante o desenvolvimento do emulador: BAGLES, MUGWMP, HEX, BULCOW, STARS, TRAP, CHOMP e WAR. Cada um foi jogado no emulador, exportado para cassete e recarregado através da ROM da ACI. Estas adaptações não são distribuídas (as listagens originais não são livres), mas fixaram as regras deste capítulo: variáveis de uma letra, tabelas a começar em 1, nada de `READ`/`DATA`, nenhum `GOTO` para fora de um `FOR`, uma só instrução comandada pelo `IF`, e `LOMEM=768` para os maiores.
