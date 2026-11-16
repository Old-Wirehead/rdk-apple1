# O seu primeiro programa BASIC

O Integer BASIC, o BASIC de Steve Wozniak, cabe em 4 KB, e o **RDK G2 Basic** do próprio emulador, que executa os mesmos programas, vem integrado. Só conhece números inteiros (−32767 a 32767), mas é rápido e agradável. O editor integrado poupa-o a escrever no ecrã do Apple-1, onde um erro de digitação obriga a reescrever a linha inteira.

## Escrever o programa

Clique em **📝 Editor** na barra de ferramentas, depois no ícone **Novo** (o primeiro) do editor, escolha «Programa BASIC» e escreva este programa (também em `docs/exemples/pt/basic/premier.bas`).

```basic title="premier.bas"
10 REM O MEU PRIMEIRO PROGRAMA
20 PRINT "OLA, APPLE-1!"
30 PRINT
40 FOR I=1 TO 8
50 PRINT I;" AO QUADRADO = ";I*I
60 NEXT I
70 PRINT
80 DIM N$(20)
90 INPUT "O TEU NOME",N$
100 PRINT "MUITO PRAZER, ";N$;"!"
110 END
```

As palavras-chave, os números, as cadeias e os comentários têm cada um a sua cor (as cores alteram-se em **⚙ Preferências → Editor**); por baixo do texto, o editor mostra o tamanho do programa e a memória que resta para as variáveis. O Apple-1 não tem acentos: o programa usa só maiúsculas sem acentos.

!!! astuce "Botão direito no texto"
    O menu do editor contém cortar, copiar e colar, **Procurar** (Ctrl+F) e **Substituir** (Ctrl+H), ir para uma linha (Ctrl+G), REM / tirar REM num bloco, **Renumerar…** (`GOTO`, `GOSUB` e `THEN` acompanham), um ponto de paragem na linha (F9) e **Enviar e executar** (F5).

## Executá-lo

Clique em **▶ Executar**. O programa vai diretamente para a memória (depois do BASIC, se for preciso) e depois o editor escreve por si `E2B3R` e `RUN`:

![O editor à esquerda, o programa em execução à direita.](img/pt/brut/editeur-premier.png "large")

O programa pede o seu nome. Clique no ecrã do Apple-1, escreva-o e carregue em Enter:

```mon
O TEU NOME?WOZ
MUITO PRAZER, WOZ!
```

!!! attention "Três particularidades do Integer BASIC"
    - **As cadeias têm de ser dimensionadas**: sem `DIM N$(20)`, o `INPUT` de uma cadeia para com `*** STR OVFL ERR`.
    - **Os nomes de variáveis** têm uma letra, seguida opcionalmente de um algarismo: `A`, `B2`, `N$`. `TOTAL` é um erro de sintaxe.
    - **O `END` final é obrigatório**: sem ele, o programa termina com `*** END ERR`.

## Quando há um erro

O editor verifica cada linha enquanto escreve, com as regras do BASIC verdadeiro. Uma linha errada fica a vermelho e a mensagem explica o problema, com o número da linha e a coluna:

![Dois erros assinalados: uma cadeia não fechada e um nome de variável demasiado comprido.](img/pt/editeur-erreur.png "medium")

Um programa com erros não é executado: corrija primeiro as linhas vermelhas. Os erros de execução aparecem no ecrã do Apple-1: `*** BAD BRANCH ERR` para um `GOTO` para uma linha que não existe, `*** >32767 ERR` para um transbordo ou uma divisão por zero. O **Depurador** (Manual completo) avança então linha a linha, vigiando as variáveis.

![O ciclo de trabalho: escrever, executar, corrigir e depois exportar para a máquina verdadeira.](diagrams/pt/workflow-basic.svg "diagram")

!!! astuce "Parar um programa"
    Durante um `RUN`, carregar numa tecla para o programa com `STOPPED AT 50` (a linha em curso). Se ficar bloqueado, **Reset** (Ctrl+R) volta ao monitor e `E2B3R` regressa ao BASIC **sem apagar** o programa.
