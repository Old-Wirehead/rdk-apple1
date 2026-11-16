# Erros conhecidos e como contorná-los

<p class="lead">O Integer BASIC do Apple-1 foi escrito sem assembler, em 1976, e nunca foi corrigido. Eis os seus defeitos conhecidos, cada um reproduzido no emulador com o BASIC verdadeiro, e como evitá-los.</p>

<span class="verified">verificado</span>: reproduzido no emulador com o Integer BASIC verdadeiro, com uma transcrição que o comprova. <span class="unverified">não verificado</span>: assinalado por uma fonte, não reproduzido. O erro n corresponde à secção 18.n. Como o emulador executa o código original instrução a instrução, um erro reproduzido aqui existe também num Apple-1 verdadeiro.

## Sair de um ciclo FOR com GOTO <span class="verified">verificado</span>

Um ciclo de que se sai com `GOTO` fica «aberto». Se o programa voltar a passar pelo `FOR`, é empilhada uma nova entrada; à nona passagem: `>8 FORS ERR`.

```basic
10 N=0
20 FOR I=1 TO 10
30 IF I=2 THEN GOTO 50
40 NEXT I
50 N=N+1:IF N<20 THEN 20
60 PRINT "FEITO ";N
70 END
```

```mon
>» RUN
*** >8 FORS ERR
STOPPED AT 20
```

!!! astuce "Contorno"
    Nunca saia de um `FOR` com `GOTO`: dê à variável o seu valor final e passe pelo `NEXT`. Aqui: `30 IF I=2 THEN I=10:GOTO 40`. O programa mostra então `FEITO 20`. As adaptações de *101 BASIC Computer Games* seguem esta regra.

## O operador ^ bloqueia a máquina <span class="verified">verificado</span>

`^` (potência) está na tabela do interpretador, mas `PRINT 2^3` nunca volta: só o RESET o tira de lá.

!!! astuce "Contorno"
    Multiplique: `2*2*2`, ou um ciclo `P=1:FOR I=1 TO N:P=P*X:NEXT I`.

## A mensagem «PPED AT» <span class="verified">verificado</span>

Quando a pilha de operandos do interpretador transborda (uma expressão demasiado encaixada em modo direto), o BASIC devia mostrar uma mensagem de erro, mas aponta **3 bytes longe demais** na tabela de mensagens (a desmontagem de Eric Smith assinala-o: `ldy #erri_stopped_at+3 ; The "+3" is a bug!`). O que se lê é o fim de `STOPPED AT`:

```mon
>» PRINT 1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1)))))))))))))
14
*** PPED AT  ERR
STOPPED AT 65535
```

!!! astuce "Contorno"
    Divida as expressões longas com variáveis intermédias. Acima de 12 níveis de parênteses, uma linha de programa é recusada de qualquer modo (`TOO LONG ERR`).

## IF … THEN só comanda a primeira instrução <span class="verified">verificado</span>

Em `IF A=2 THEN PRINT "X":PRINT "Y"`, só `PRINT "X"` depende da condição: `PRINT "Y"` é **sempre** executado (com A=1, o programa mostra `Y`). É a armadilha mais frequente ao adaptar um programa escrito para outro BASIC.

!!! astuce "Contorno"
    Inverta a condição e salte por cima do bloco:

    ```basic
    80 IF A#2 THEN 90
    85 PRINT "X":PRINT "Y"
    90 REM CONTINUA
    ```

## −32768 <span class="verified">verificado</span>

`X=-32767-1` mostra um `-` e depois `*** >32767 ERR`. O menor valor utilizável é −32767.

!!! astuce "Contorno"
    Fique dentro de −32767 … 32767. Para ler uma palavra de 16 bits com `PEEK`, divida-a: `PEEK(A)+PEEK(A+1)*256` transborda acima de 32767; teste primeiro o byte alto (`IF PEEK(A+1)>127 THEN …`).

## ASC( não funciona <span class="verified">verificado</span>

`ASC("A")` está na tabela de palavras-chave, mas qualquer linha que o use é recusada: `SYNTAX ERR`. O mesmo acontece com `SCRN(`.

!!! astuce "Contorno (experimentado)"
    Procure o carácter numa cadeia de referência. A linha 60 fecha o ciclo (`K=26:NEXT K`) para evitar o erro 1:

```basic title="A posição de uma letra, sem ASC"
10 DIM C$(26),L$(1)
20 C$="ABCDEFGHIJKLMNOPQRSTUVWXYZ"
30 INPUT "LETRA",L$
40 FOR K=1 TO 26:IF C$(K,K)=L$ THEN 60:NEXT K
50 PRINT "NAO E UMA LETRA":END
60 PRINT "POSICAO ";K:K=26:NEXT K
70 GOTO 30
```

```mon
LETRA?» W
POSICAO 23
LETRA?» E
POSICAO 5
LETRA?» 5
NAO E UMA LETRA
```

## Palavras-chave gráficas que não fazem nada <span class="verified">verificado</span>

`COLOR=`, `PLOT` e `HLIN` são aceites e não fazem nada; `VLIN` e `VTAB` dão `SYNTAX ERR`. São restos do BASIC destinado ao Apple II.

!!! astuce "Contorno"
    Para posicionar texto, use `TAB n` e `PRINT` vazios. Para desenhar a cores, a placa GEN2 comanda-se com `POKE` (capítulo 6).

## O erro dos 251 caracteres do Apple II <span class="verified">ausente</span>

Uma versão posterior do Integer BASIC (Apple II) bloqueia com certas linhas de 251 caracteres ou mais. No Apple-1 a linha escrita é limitada muito antes (cerca de 128 caracteres): o erro não pode acontecer.

## STOPPED AT 65535 <span class="verified">verificado</span>

Um erro causado por uma linha **recusada enquanto é introduzida** (demasiado encaixada) ou pelo erro 3 em modo direto é seguido de `STOPPED AT 65535`: o BASIC julga estar num programa e mostra um número de linha absurdo. Não tem consequências; o programa na memória não é tocado.

## USR bloqueia a máquina <span class="verified">verificado</span>

`X=USR(1)` salta para `$0000`, onde normalmente não há código: a máquina bloqueia. RESET e depois `E2B3R` voltam ao BASIC sem perder o programa.

!!! astuce "Contorno"
    Chame os seus programas em código máquina com `CALL endereço`; passe os valores com `POKE` e `PEEK`.

## HIMEM= não pode descer <span class="verified">verificado</span>

`HIMEM=3072` (e qualquer valor abaixo do atual) é **ignorado em silêncio**, mesmo depois de `SCR`, porque o interpretador compara o novo valor com o ponteiro do programa antes de o aceitar.

!!! astuce "Contorno (verificado)"
    `POKE 76,0 : POKE 77,12 : SCR` põe o HIMEM em 3072; o programa seguinte é guardado abaixo. Veja o capítulo 16.

## Os dois pontos recusados em modo direto <span class="verified">verificado</span>

`X=5:Y=X*2:PRINT Y` escrito sem número dá `SYNTAX ERR`; a mesma linha com número funciona. `FOR` e `GOSUB` também são recusados em modo direto.

!!! astuce "Contorno"
    Escreva uma instrução por linha, ou escreva uma linha provisória `0` e `RUN 0`… sem se esquecer de a apagar (`DEL 0`).

## INPUT acrescenta sempre «?» <span class="verified">verificado</span>

`INPUT "IDADE ? ",N` mostra `IDADE ? ?`: o BASIC acrescenta o seu próprio ponto de interrogação. Escreva o pedido sem `?`: `INPUT "IDADE",N` mostra `IDADE?`.

## O ciclo FOR corre pelo menos uma vez <span class="verified">verificado</span>

`FOR I=5 TO 1 … NEXT I` executa o corpo uma vez (com I=5), quando outros BASIC o saltam.

!!! astuce "Contorno"
    Teste antes: `IF N<1 THEN 100` antes de `FOR I=1 TO N`.

## Uma tecla para o programa <span class="verified">verificado</span>

Qualquer tecla carregada enquanto o programa corre para-o (`STOPPED AT`), exceto durante um `INPUT`. Não é um erro mas uma escolha de conceção: não há tecla BREAK. Consequência: um jogo em tempo real não pode esperar por uma tecla sem parar. Ler o teclado com `PEEK(-12271)` (`$D011`) e `PEEK(-12272)` (`$D010`) parece possível, mas o interpretador também testa o teclado entre instruções e pode parar primeiro: não o experimentámos (<span class="unverified">não verificado</span>). Para um jogo em tempo real, passe ao código máquina (Parte III).

## Resumo

| # | Erro | Estado | Contorno |
|---|---|---|---|
| 1 | sair de FOR com GOTO → `>8 FORS` | <span class="status-ok">verificado</span> | terminar o ciclo com NEXT |
| 2 | `^` bloqueia | <span class="status-ok">verificado</span> | multiplicar |
| 3 | «PPED AT» (+3) | <span class="status-ok">verificado</span> | variáveis intermédias |
| 4 | `IF…THEN a:b` executa b | <span class="status-ok">verificado</span> | condição invertida + salto |
| 5 | −32768 | <span class="status-ok">verificado</span> | ficar ≥ −32767 |
| 6 | `ASC(`, `SCRN(` | <span class="status-ok">verificado</span> | cadeia de referência |
| 7 | COLOR=, PLOT, HLIN não fazem nada; VLIN, VTAB recusados | <span class="status-ok">verificado</span> | TAB, POKE (GEN2) |
| 8 | 251 caracteres (Apple II) | <span class="status-ok">ausente</span> | — |
| 9 | STOPPED AT 65535 | <span class="status-ok">verificado</span> | inofensivo |
| 10 | USR bloqueia | <span class="status-ok">verificado</span> | CALL |
| 11 | HIMEM= não desce | <span class="status-ok">verificado</span> | POKE 76/77 + SCR |
| 12 | `:` recusado em modo direto | <span class="status-ok">verificado</span> | uma instrução por linha |
| 13 | INPUT acrescenta «?» | <span class="status-ok">verificado</span> | pedido sem «?» |
| 14 | FOR corre uma vez | <span class="status-ok">verificado</span> | testar antes |
| 15 | ler o teclado com PEEK durante o RUN | <span class="status-src">não verificado</span> | código máquina |
