# Primeiro arranque

No primeiro arranque a janela abre-se maximizada, e uma janela **Seu espaço de trabalho** propõe cinco disposições: **Padrão**, **Jogo**, **BASIC**, **Assembly** e **Depuração**. Cada miniatura numerada mostra a disposição, diz para que serve e indica o tamanho mínimo de ecrã de que precisa: clique na que pretende. Desmarque **Mostrar esta janela ao iniciar** se preferir não a voltar a ver; **⊞ Disposição ▾ → Escolher um espaço de trabalho…** volta a abri-la. Pode mudar de disposição a qualquer momento com o menu **⊞ Disposição ▾**. A disposição padrão mostra todas as ferramentas de relance. Cada zona é um painel acoplável que pode mover, agrupar em separadores ou destacar numa janela própria. O ecrã a cores (GEN2 / P-LAB) é o retângulo número 7, nas miniaturas que o mostram e na legenda; os números 1 a 6 não mudam. Esta janela não fica por cima das outras. Se a janela for estreita, a barra de ferramentas desloca-se: o botão **»** traz o grupo Cassete.

![A disposição padrão. O ecrã já mostra o pequeno programa da secção seguinte.](img/pt/defaut-annote.png "large")

<ol class="legend">
<li><strong>Carregar, Guardar, Estado</strong>: dumps hexadecimais <code>.txt</code>, binários <code>.bin</code> e <code>.prg</code>; Guardar também escreve cassetes <code>.wav</code>/<code>.aiff</code>; Estado guarda e restaura a máquina.</li>
<li><strong>RESET, Limpar ecrã, Velocidade (indicador), Colar</strong>: os botões da máquina (Ctrl+R, Ctrl+L, F3, Ctrl+V).</li>
<li><strong>BASIC</strong>: carrega o RDK G2 Basic integrado em <code>$E000</code>.</li>
<li><strong>Janelas</strong>: o menu <strong>⊞ Disposição ▾</strong> e depois um botão por painel (ecrãs, Editor, Depurador, Desmontagem, Dump, Biblioteca, ASCII).</li>
<li><strong>Placas gráficas</strong>: <strong>GEN2</strong> e <strong>P-LAB</strong> ligam ou desligam as placas.</li>
<li><strong>Cassete</strong>: <strong>📼 Inserir</strong> uma cassete e depois reproduzir, parar, rebobinar, gravar, ejetar; osciloscópio e som.</li>
<li><strong>EPROM</strong>, <strong>⚙ Preferências</strong>, <strong>?</strong> (balões de ajuda detalhados, também F1) e <strong>ℹ</strong> (acerca de, atalhos).</li>
<li><strong>Linha de visualização do ecrã</strong>: zoom (1x a 8x ou Ajustar), fósforo, velocidade de visualização <strong>Normal</strong> ou <strong>Rápido</strong>, ecrã vintage, ecrã inteiro, <strong>Destacar</strong>.</li>
<li><strong>Editor</strong>: programas BASIC e fontes 6502 em separadores.</li>
<li><strong>Ecrã do Apple-1</strong>: 40 × 24 caracteres; clique nele para escrever, botão direito para Copiar, Colar, Limpar ecrã e Reset.</li>
<li><strong>Depurador</strong>: pausa, passo a passo, registos, pontos de paragem, variáveis.</li>
<li><strong>Desmontagem</strong>: o código à volta do contador de programa (PC).</li>
<li><strong>Dump</strong>: a memória em hexadecimal e ASCII, editável.</li>
<li><strong>Biblioteca</strong>: os seus programas, cada um com os seus pequenos ícones ▶ ⬇ 📼 ⚙.</li>
<li><strong>Barra de estado</strong>: velocidade, ritmo de visualização, cassete, destino do teclado; por baixo, a última mensagem.</li>
</ol>

!!! astuce "Arranjar espaço"
    Para apenas jogar, feche o editor e o depurador com os seus botões no grupo Janelas (4): o ecrã aumenta. Na lista de zoom (8), **Ajustar** escolhe o maior zoom inteiro que cabe, **F11** passa a ecrã inteiro, **⊞ Disposição ▾ → Padrão** (4) repõe tudo; o mesmo menu oferece também uma disposição **Jogo** (ecrã do Apple-1 e biblioteca). Se os botões forem demasiado pequenos ou grandes para o seu ecrã, altere **⚙ Preferências → Interface → Tamanho dos ícones e do texto**.

!!! note "Acesso à rede no primeiro arranque"
    Da primeira vez que arranca, o emulador contacta o GitHub (`api.github.com`, `raw.githubusercontent.com`) para descarregar o projeto de demonstração (`dschmenk/Apple1-Slideshow`, indicado em «Projetos GitHub» nas **⚙ Preferências**), e mostra o progresso na barra de estado. Nada mais é enviado. Sem rede, simplesmente salta este passo; retire o projeto dessa lista para parar a transferência de vez.

## O Woz Monitor

A barra invertida `\` é o prompt do **Woz Monitor**, o programa de 256 bytes que arranca o Apple-1. Clique no ecrã e escreva: as minúsculas passam a maiúsculas, **Enter** é RETURN e **Retrocesso** dá o `_` que apaga o último carácter, como em 1976.

O monitor percebe quatro tipos de comandos:

| Escreve | Efeito |
|---|---|
| `300` | mostra o byte do endereço `$0300` |
| `300.30F` | mostra os bytes de `$0300` a `$030F` |
| `300: A9 C1` | escreve `A9` em `$0300` e depois `C1` em `$0301` |
| `300R` | executa o programa que começa em `$0300` (*Run*) |

## O seu primeiro programa em linguagem máquina

Escreva estas duas linhas, cada uma seguida de Enter:

```mon
» 300: A9 C1 20 EF FF 4C 1F FF

0300: 00
» 300R

0300: A9A
```

O `A` final foi escrito pelo seu programa. Os oito bytes são três instruções do 6502:

| Bytes | Instrução | Função |
|---|---|---|
| `A9 C1` | `LDA #$C1` | carrega o código da letra «A» (com o bit 7 a 1, como o Apple-1 espera) |
| `20 EF FF` | `JSR $FFEF` | chama ECHO, a rotina do monitor que mostra um carácter |
| `4C 1F FF` | `JMP $FF1F` | volta ao monitor, que espera o comando seguinte |

O monitor mostra primeiro o byte do endereço inicial (`0300: A9`) e depois o seu programa escreve o `A`. O assembler integrado (Manual completo) poupa-o a calcular estes bytes à mão.

!!! apple1 "No Apple-1 verdadeiro"
    Os comandos são idênticos: o emulador executa a ROM verdadeira do Woz Monitor. A máquina verdadeira mostra cerca de 60 caracteres por segundo. O emulador mantém esse ritmo quando a linha de visualização do ecrã está em **Normal**; **Rápido** mostra tudo de uma vez. A definição é comum ao ecrã do Apple-1 e ao ecrã GEN2.
