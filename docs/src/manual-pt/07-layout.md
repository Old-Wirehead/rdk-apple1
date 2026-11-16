# Disposição dos painéis

<p class="lead">Cada painel é um separador que se coloca onde se quiser: lado a lado, em separadores, ou numa janela própria num segundo ecrã.</p>

## Mover um painel

O editor, o ecrã do Apple-1, o monitor a cores GEN2, o depurador, a desmontagem, o dump, a ferramenta ASCII e a biblioteca são todos **separadores acopláveis**. Arraste um separador pelo título: aparecem guias, e larga-o **à esquerda de**, **à direita de**, **acima** ou **abaixo** de outro painel, ou **no mesmo grupo de separadores**. Arraste as divisórias para redimensionar. A disposição é memorizada em `gui.json`.

## A disposição inicial

O editor fica à esquerda; ao centro, o ecrã do Apple-1, com o monitor a cores por baixo quando a placa GEN2 está ligada; à direita, o depurador com a desmontagem por baixo, depois o dump e a biblioteca na extremidade direita. No primeiríssimo arranque, a escolha depende da largura do **monitor** (a janela abre-se pequena e é maximizada um instante depois): abaixo de 1400 pontos (um portátil de 1366 × 768, um ecrã de 1024 × 768) começa com **editor e ecrã**, o par mais útil; a partir de 1400 pontos, com todos os painéis. Escolha outra disposição no menu quando quiser. Ela segue a largura da janela: cada painel mantém a sua parte, de um portátil a um monitor largo. O menu **⊞ Disposição ▾** da barra de ferramentas repõe-na a qualquer momento (**Padrão**, todos os painéis abertos) e oferece mais quatro disposições prontas: **Jogo** (ecrã do Apple-1 e biblioteca), **BASIC** (editor e ecrã), **Assembly** (editor, ecrã e depurador) e **Depuração** (editor, depurador, desmontagem e dump). **Guardar a disposição atual…** pede um nome e guarda a disposição atual, com as suas janelas destacadas e o tamanho da janela principal (ao carregá-la, essas janelas voltam a abrir-se onde estavam); as suas disposições aparecem por baixo de um separador, por ordem alfabética, e um clique com o botão direito permite **Mudar o nome** ou **Eliminar**. Ficam guardadas em `gui.json`.

## O assistente de arranque

No primeiro arranque, um assistente de três ecrãs substitui a antiga janela «Seu espaço de trabalho». É desenhado a âmbar sobre preto, com o efeito de tubo (linhas de varrimento, halo, vinheta, curvatura 0,07) na moldura; o texto pequeno e as miniaturas continuam nítidos. Um pequeno **×** em cima à direita fecha-o. **‹ Anterior** e **Seguinte ›** passam de um ecrã para outro; o último tem **Concluir**.

1. **Boas-vindas**: para que serve o kit.
2. **ROM e BASIC**: o Woz Monitor, a ROM da cassete e o G2 Basic vêm integrados. Não é preciso mais nada. Os puristas instalam o seu próprio Integer BASIC original a partir de uma fonte externa; o botão abre o guia («Instalar uma ROM»), que só tem ligações, nunca os ficheiros.
3. **Seu espaço de trabalho**: as cinco disposições (**Padrão**, **Jogo**, **BASIC**, **Assembly**, **Depuração**). Cada miniatura é clicável (não há botão «Escolher»). Os retângulos mantêm os números 1 a 6, e o ecrã a cores (GEN2 / P-LAB) é o **7**, na sua própria cor, nas miniaturas que o mostram e na legenda.

**Mostrar esta janela ao iniciar** está nos três ecrãs, assinalada por omissão. A mesma caixa está em **⚙ Preferências → Interface**. **⊞ Disposição ▾ → Escolher um espaço de trabalho…** volta a abri-lo. É uma janela normal: não fica por cima das outras. Não se abre quando a linha de comandos define um tamanho (`--taille`), nem para as capturas.

## Tamanho do texto de cada janela

O editor, a listagem do assembler, o depurador, a desmontagem, o dump e a biblioteca têm cada um um controlo compacto **A 13 ▲▼** na sua barra: os triângulos mudam o tamanho, que é memorizado para essa janela. Ctrl+roda e Ctrl+mais / Ctrl+menos atuam sobre a janela que está sob o ponteiro do rato (sobre o ecrã do Apple-1, Ctrl+mais / Ctrl+menos mudam o zoom). **Repor as preferências** devolve todos estes tamanhos a 13.

## O que é memorizado entre dois arranques

A disposição é guardada sozinha ao sair e recuperada tal como estava: a posição e o tamanho de cada painel, as janelas destacadas, o zoom e a disposição escolhida. A janela é reposta com o seu tamanho e no seu lugar, e depois os painéis voltam a ser colocados nas mesmas proporções, seja qual for o ecrã: a disposição foi verificada a 2880 × 1620, 2560 × 1440, 1920 × 1080 e 1366 × 768, com menos de 1 % da imagem diferente depois de um novo arranque. Uma disposição guardada com nome (**Guardar a disposição atual…**) guarda ainda as janelas destacadas. A interface nunca faz zoom sozinha: só a definição «Tamanho dos ícones e do texto» muda o tamanho dos botões.

## Quando o ecrã é maior do que o seu painel

Com um **zoom fixo** (2x, 3x… ou <kbd>Ctrl</kbd>+<kbd>+</kbd>), o ecrã do Apple-1 mantém o tamanho pedido, mesmo que o painel seja mais pequeno: **desliza** então (roda ou barras) em vez de ser reduzido, e a barra de estado indica-o, com uma sugestão para aumentar o painel. Só **Ajustar** adapta o zoom ao painel. Numa janela estreita (1024 × 768, por exemplo), a barra de ferramentas desliza para o lado em vez de ser cortada: o botão **»** traz o grupo Cassete, e um segundo clique volta ao início. O botão **Executar** do editor fica sempre visível.

## Destacar um painel

Cada painel tem um botão **Destacar** em cima à direita (também com o botão direito no separador) que o abre **numa janela própria**, que pode levar para um segundo ecrã; o seu tamanho e o seu lugar são memorizados. **⊟ Ancorar** volta a pô-lo no seu lugar inicial.

- O ecrã do Apple-1 e o monitor a cores destacam-se separadamente. O teclado vai para o Apple-1 a partir de qualquer das suas janelas e da janela principal. Fechar a janela de um ecrã volta a acoplá-lo.
- Fechar a janela de uma ferramenta (editor, depurador…) fecha a ferramenta; o seu botão da barra de ferramentas volta a abri-la no mesmo lugar.
- As caixas de diálogo (Abrir, Guardar como, Exportar) abrem-se sempre na janela principal. Nos diálogos de ficheiros, <kbd>Esc</kbd> fecha o diálogo (também a partir do campo do nome do ficheiro), <kbd>Ctrl</kbd>+<kbd>L</kbd> edita o caminho (o seu texto fica selecionado, pelo que escrever o substitui); Guardar e Exportar de um separador começam na pasta do seu ficheiro.
- As disposições guardadas mantêm as janelas destacadas; a janela principal mantém o mesmo tamanho de um arranque para o outro, seja qual for o zoom.
- O osciloscópio da cassete continua a ser uma pequena janela flutuante dentro da janela principal.

<div class="cards">
<div class="card"><h4>Para jogar</h4>Feche o editor e o depurador, mantenha a biblioteca. <strong>Ajustar</strong> aumenta o ecrã; <kbd>F11</kbd> passa a ecrã inteiro, com o ecrã vintage.</div>
<div class="card"><h4>Para programar</h4>Editor à esquerda, ecrã ao centro, depurador e dump à direita. Destaque o editor para um segundo monitor para lhe dar toda a altura.</div>
<div class="card"><h4>Para a GEN2</h4>Ligue a placa: o monitor a cores fica por baixo do ecrã do Apple-1. Destaque-o para o ter grande, como um segundo monitor.</div>
<div class="card"><h4>Para uma apresentação</h4><strong>Destaque</strong> o ecrã do Apple-1 para o projetor, em ecrã inteiro, e a janela principal no seu portátil.</div>
</div>

# Preferências

<p class="lead">⚙ Preferências reúne todas as definições permanentes. Tudo fica guardado em ~/.config/apple1-emu/gui.json (ou na pasta do kit, em modo portátil).</p>

![O separador Interface das preferências: língua, tema, tamanho dos ícones e do texto, barra compacta, balões, janela de arranque e cores da interface.](img/pt/preferences.png "large")

As definições estão repartidas por seis separadores:

| Separador | Definições |
|---|---|
| **Interface** | língua: automática (a partir da disposição do teclado) ou forçada: francês, inglês, alemão, russo, espanhol, italiano, português; são mostrados a língua detetada e a sua origem. Tema: escuro, claro ou sistema. Tamanho dos ícones e do texto: pequeno, normal (por omissão), grande, muito grande. Barra compacta (só ícones quando a janela é estreita). Balões ao passar o rato. **Mostrar esta janela ao iniciar** (a janela «Seu espaço de trabalho») e o botão que a abre de imediato. **Cores da interface**: as cores das listas da interface, escolhidas **separadamente para o tema escuro e para o claro** (botões ativos, linha atual do depurador, linha realçada, célula selecionada, linha de ponto de paragem, endereços e coluna ASCII do dump, valores alterados, etiquetas); cada cor tem uma amostra com pré-visualização em direto, os valores por omissão são legíveis (contraste de pelo menos 4,5:1 sobre o fundo do tema), e os botões **Repor** restauram um tema ou os dois. |
| **Editor** | cores da sintaxe, separadamente para o BASIC e para o assembler 6502 (capítulo 7) |
| **Biblioteca** | a pasta principal (**Procurar…**, **⟲ Predefinida**, **Abrir a pasta**), as outras pastas, os projetos GitHub e o token GitHub opcional (só para repositórios privados e artefactos do Actions; nunca é mostrado, e a variável `GITHUB_TOKEN` tem prioridade); posição da biblioteca, à esquerda ou à direita (à direita por omissão) |
| **Cassete** | som da cassete (mudo à velocidade rápida) e o seu volume, osciloscópio durante a reprodução ou a gravação |
| **GEN2** | a placa a cores de Uncle Bernie, varrimento a 60 ou 50 Hz, paleta LORES (capítulo 6) |
| **Avançado** | visualização e velocidade: mostrar a biblioteca, visualização limitada a ~60 caracteres/s como o terminal verdadeiro, velocidade rápida; **Ecrã vintage: ajustes finos** (comuns aos dois ecrãs, capítulo 3); o ficheiro Integer BASIC carregado em `$E000` (**📂 Escolher…** / **Esquecer**) |

Cada cursor das preferências (intensidade, remanência e curvatura do ecrã vintage, volume da cassete) mostra o seu **valor numérico** ao lado. No primeiro arranque, a curvatura do ecrã vintage vale **0,07** (um ligeiro arredondamento do tubo); uma instalação existente mantém o seu próprio valor. A pré-visualização do separador **Editor** segue a língua da interface.

O zoom, o fósforo e a caixa do ecrã vintage não estão nas preferências: estão na linha de visualização de cada ecrã (capítulo 3).

![O separador Editor: um esquema de cores para o BASIC e para o assembler 6502, ou as suas próprias cores.](img/pt/preferences-editeur.png "large")

**Língua automática.** O emulador segue a disposição ativa do teclado: `fr` dá francês, `us` ou `gb` inglês, `de`, `ch` ou `at` alemão, `ru` russo, e assim por diante. Pergunta ao GNOME, ao `setxkbmap`, ao Cinnamon (o caso do Linux Mint), ao `localectl`, depois a `/etc/default/keyboard` e, por fim, às variáveis `LANG`. Na linha de comandos: `--langue fr|en|de|ru|es|it|pt|auto`.

!!! astuce "Recomeçar do zero"
    **Repor as preferências**, no fundo da janela, restaura as definições de origem depois de uma confirmação (língua, tema, disposição, ecrã vintage…). Os ficheiros da sua biblioteca ficam onde estão; os separadores do editor por guardar são fechados. O primeiro arranque de uma nova versão do kit faz a mesma pergunta, porque `gui.json` sobrevive quando a pasta do programa é apagada para instalar outra.

    Para um kit portátil, ponha um ficheiro vazio chamado `portable` na raiz do kit (ao lado de `run.sh`), ou arranque-o com `APPLE1_PORTABLE=1`: as preferências (`config/gui.json`), a biblioteca por omissão (`programmes`) e a cache ficam então na pasta do kit.

    Fechar o emulador e mudar o nome de `~/.config/apple1-emu/gui.json` faz o mesmo que o botão, no arranque seguinte. As definições dos programas ficam no `programs.json` de cada pasta.
