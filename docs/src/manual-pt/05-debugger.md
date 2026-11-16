# O depurador, a desmontagem e o dump

<p class="lead">O depurador para o Apple-1 e fá-lo avançar passo a passo, uma instrução de cada vez em 6502 ou uma linha de cada vez em BASIC, mostrando os registos, as variáveis e a memória.</p>

## O painel do depurador

**🐛 Depurador** abre o painel, acoplado à direita ou destacável como o editor. Os seus comandos:

| Comando | Efeito |
|---|---|
| **⏸ Pausa** / **▶ Continuar** | para ou retoma o Apple-1 |
| **⬇ Passo** | executa uma instrução |
| **↪ Por cima** | executa um `JSR` de uma vez e para no regresso |
| **⤴ Sair** | executa até ao `RTS` da sub-rotina atual |
| **⏭ Linha BASIC** | avança uma linha do programa BASIC |
| **⏩ Até ao cursor** | (no editor) executa até à linha do cursor |

{{!Passo (depurador)}}{{!Por cima}}{{!Sair}}

O quadro **Processador** mostra A, X, Y, S, PC e os indicadores N V B D I Z C; todos se podem alterar clicando no valor. Por baixo aparece o número de ciclos e de instruções executados.

## Pontos de paragem

Um ponto de paragem pode ser colocado de quatro formas:

- **⏺ Ponto de paragem** no editor (ou <kbd>F9</kbd>, ou o menu do botão direito), na linha do cursor: numa fonte `.s` (graças à tabela de endereços do assembler) ou num programa BASIC (número de linha);
- um clique na margem da **📃 Listagem**;
- um clique numa linha da desmontagem;
- escrever um endereço ou uma etiqueta: `$0368`, `ASK`, `PUTS+3`, ou `L190` para a linha BASIC 190. Uma etiqueta que ainda não exista não é recusada: o ponto de paragem é guardado e colocado assim que a etiqueta existir (quando montar a fonte do editor ou a executar), com uma mensagem que o indica.

Os pontos de paragem, as variáveis vigiadas e as vigilâncias de memória são memorizados de uma sessão para a outra, mas carregar outro programa apaga os pontos de paragem e as etiquetas do anterior. Quando o BASIC integrado está carregado, a secção BASIC do depurador intitula-se **RDK G2 Basic**; sem BASIC em `$E000`, di-lo de forma neutra. As cores da linha atual, dos endereços do dump e da coluna ASCII alteram-se em **⚙ Preferências → Interface** («Cores da interface»). Nos painéis estreitos desaparece a coluna de bytes da desmontagem e a linha de escrita passa para a linha seguinte em vez de ser cortada.

## Depurar um programa BASIC

![boucle.bas parado num ponto de paragem na linha 30: a linha atual a verde no editor, as variáveis I e S vigiadas no depurador.](img/pt/brut/debogueur-basic.png "large")

A linha atual é realçada a verde no editor, os pontos de paragem a violeta. O quadro BASIC (**RDK G2 Basic** ou **Integer BASIC**, conforme o que estiver carregado) mostra a linha atual, os pontos de paragem em números de linha e as **variáveis vigiadas**: inteiras (`A`, `B1`), elementos de tabela (`D(2)`), tabelas inteiras (`D`), cadeias (`C$`), mais **Todas as variáveis**.

A depuração funciona com o **RDK G2 Basic**, o BASIC integrado no emulador, tal como com o original: o Integer BASIC verdadeiro já não é necessário. O depurador deteta o início de cada linha (a linha atual, os pontos de paragem de linha, **⏭ Linha BASIC**) e lê as variáveis nas tabelas do interpretador. No original lê a linha nos ponteiros da página zero `$DC`/`$DD`; o G2 Basic guarda-a noutro sítio, e o depurador conhece as duas disposições.

!!! attention "Uma tecla para o programa"
    Como na máquina verdadeira, uma tecla carregada enquanto o programa BASIC corre para-o (`STOPPED AT`). Durante uma sessão de depuração, mantenha o foco do teclado no painel do depurador, não no ecrã do Apple-1. Um programa G2 Basic pode ler o teclado sem ser parado com `KEY(0)` (capítulo 15).

## Depurar um programa 6502

![decimal.s parado na etiqueta prdec: registos, desmontagem com as etiquetas do programa, vigilância de memória $06.$08.](img/pt/brut/debogueur-asm.png "large")

A **desmontagem** mostra 24 linhas à volta de PC, com as etiquetas do programa montado numa coluna própria. Só volta a centrar-se quando PC sai da zona visível, para que a visualização não salte a cada passo. **Ir** fixa a lista num endereço ou numa etiqueta; **Seguir PC** fá-la seguir de novo a execução.

Quando seleciona uma instrução na fonte 6502 do editor, os seus bytes também são **realçados** na desmontagem e no dump (capítulo 7), e os dois painéis deslocam-se até eles se não estiverem visíveis; a desmontagem deixa então de seguir PC até carregar em **Seguir PC**.

A **vigilância de memória** mostra um byte, uma palavra ou ASCII (`$30`, `$30 w`, `0200.020F`, uma etiqueta), editável, a amarelo quando o valor muda.

## O dump

O **Dump** mostra a memória em hexadecimal e ASCII, 16 bytes por linha (8 se o painel for estreito), com o bit 7 retirado para o ASCII. **Ir** salta para um endereço ou para os atalhos ZP, `$0200`, `$0300`, PC, `$E000`. Os bytes alterados aparecem a amarelo, o byte de PC sobre fundo verde. **Escrever** deposita bytes (`0300 = A9 00`); um clique num byte preenche a linha de escrita.

A desmontagem e o dump são painéis de pleno direito: destacados por omissão, podem ser acoplados ao lado do depurador (capítulo 10).

## Na linha de comandos

Para demonstrações e testes automáticos: `--debogueur` abre o painel, `--dbg-arret ASK,L190` coloca pontos de paragem, `--dbg-surveiller 'A,C$,$30 w'` vigia variáveis e memória, e `--dbg-commande 5:pas` envia um comando ao fim de 5 segundos (`pause`, `continuer`, `pas`, `dessus`, `sortir`, `basic`). Estes nomes de opções e palavras-chave estão em francês.
