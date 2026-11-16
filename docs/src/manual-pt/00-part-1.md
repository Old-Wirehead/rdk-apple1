---
version: 1.0
date: novembro de 2026
---

# O emulador <span class="part-sub">A janela, o ecrã, a biblioteca, a interface de cassete ACI, a placa a cores GEN2, o depurador e a linha de comandos</span> {: .part data-part="Parte I" }

# Sobre este manual

<p class="lead"><strong>Retro Dev Kit Apple 1</strong> — «O pecado original» — reproduz ciclo a ciclo o Apple-1 de 1976: o 6502, o terminal de vídeo, o teclado, a interface de cassete ACI e, como opções, a placa a cores GEN2 de Uncle Bernie e a placa gráfica P-LAB com TMS9918A. Traz o seu próprio BASIC livre, <strong>RDK G2 Basic</strong>. À volta da máquina acrescenta uma oficina completa: uma biblioteca de programas, um editor, um assembler, um depurador e a exportação para um Apple-1 verdadeiro.</p>

Este manual descreve todo o emulador, versão 1.0 (a primeira versão pública; os números 7.x eram as versões de teste). Completa o *Guia de início rápido*, que basta para instalar o programa e reproduzir uma primeira cassete. Aqui explicam-se cada painel, cada botão e cada comando, com exemplos que pode escrever tal como estão.

## Como está organizado este manual

<div class="cards">
<div class="card"><h4>Parte I — O emulador</h4>A janela e os seus painéis, o ecrã vintage, a biblioteca, a cassete e a ACI, as placas gráficas GEN2 e P-LAB, o editor, o depurador, a arte ASCII, as preferências, a linha de comandos e a resolução de problemas.</div>
<div class="card"><h4>Parte II — Integer BASIC</h4>O BASIC de Steve Wozniak e o seu clone livre RDK G2 Basic: cada instrução, função e operador, a memória, as mensagens de erro e os erros conhecidos, com as respetivas soluções.</div>
<div class="card"><h4>Parte III — O assembler 6502</h4>O 6502 desde o início, o assembler integrado e as suas diretivas, as rotinas do Woz Monitor, exemplos completos e as armadilhas a evitar.</div>
<div class="card"><h4>Anexos</h4>Mapa da memória, comandos do Woz Monitor, o formato ACI, o conjunto de caracteres do 2513, os atalhos de teclado e o índice de comandos.</div>
</div>

## Convenções

As teclas escrevem-se assim: <kbd>Ctrl</kbd>+<kbd>R</kbd>, <kbd>RETURN</kbd>. Os botões da interface aparecem a negrito: **▶ Executar**. Os endereços estão em hexadecimal, precedidos de `$` como nos documentos da época: `$FFEF`. O próprio Woz Monitor escreve-os sem `$`: `FFEF`.

As sessões no ecrã do Apple-1 têm fundo escuro. O que **escreve** aparece a branco, a resposta da máquina a verde:

```mon
» 300R

0300: A2
OLA, APPLE-1!
```

Ao longo do caminho surgem quatro tipos de caixas:

!!! astuce "Dica"
    Um atalho, ou uma forma melhor de fazer algo.

!!! attention "Atenção"
    Uma armadilha que custa tempo ou dados.

!!! apple1 "Na máquina verdadeira"
    O que acontece num Apple-1 verdadeiro e o que difere do emulador.

!!! bug "Erro conhecido"
    Um defeito da ROM original, com a sua solução. Cada erro tem um estado: <span class="verified">verificado</span> (reproduzido no emulador com o Integer BASIC verdadeiro) ou <span class="unverified">não verificado</span> (citado de uma fonte, não testado).

Todos os exemplos deste manual foram testados no emulador. Os programas 6502 da Parte III estão na pasta `docs/exemples/pt/asm/` da distribuição. Produzem exatamente os mesmos bytes com o assembler integrado e com o ca65.

## O que é preciso

O emulador funciona em Linux sem instalar nada. O Woz Monitor e a ROM da ACI vêm integrados, tal como o **RDK G2 Basic**, um BASIC livre compatível com o Integer BASIC (capítulo 14): o botão **BASIC** funciona de imediato, numa instalação nova, sem procurar nenhum ficheiro. O **Integer BASIC** da Apple de 1976 **não** está incluído, porque continua protegido por direitos de autor. Se quiser o original, use a sua própria cópia (por exemplo a cassete `BASIC.wav` da coleção «Apple1Cassettes» do archive.org) e escolha-a em **⚙ Preferências → Avançado → BASIC**.

!!! attention "As ROM e as cassetes ficam consigo"
    O emulador não inclui nem o Integer BASIC da Apple nem cassetes comerciais. A sua tabela de reconhecimento só contém tamanhos, impressões digitais e parâmetros. Guarde as suas cópias na sua biblioteca pessoal.

!!! note "Licença"
    Retro Dev Kit Apple 1 é software livre sob a **GNU GPL 3.0** (sem garantia). Algumas partes mantêm a sua própria licença MIT: o RDK G2 Basic e os testes e glifos GEN2 de Uncle Bernie. Veja `LICENSE` e `NOTICE` no arquivo, e o anexo «Licença e créditos».

## Os percursos principais

Quatro formas de usar o emulador surgem vezes sem conta. Cada uma é descrita mais adiante com o seu próprio esquema:

| Quero… | O percurso | Ver |
|---|---|---|
| jogar um programa da época | biblioteca → **▶** | capítulo 4 |
| escrever um programa BASIC | **📝 Editor** → **▶ Executar** → **📼 Exportar WAV** | Parte II |
| escrever um programa 6502 | **📝 Editor** (`.s`) → **📃 Listagem** → **🐛 Depurador** | Parte III |
| enviar um programa para um Apple-1 verdadeiro | **📼 Exportar WAV** (`.aiff`) → smartphone → ACI | capítulo 5 |

# A janela e a barra de ferramentas

## Vista geral

![A disposição padrão: todos os painéis abertos e acoplados.](img/pt/defaut-annote.png "large")

<ol class="legend">
<li><strong>Carregar, Guardar, Estado</strong>: dumps hexadecimais <code>.txt</code>, binários <code>.bin</code> e <code>.prg</code>; Guardar também escreve cassetes <code>.wav</code>/<code>.aiff</code>; Estado guarda e restaura a máquina.</li>
<li><strong>RESET, Limpar ecrã, Velocidade (indicador), Colar</strong>: os botões da máquina (Ctrl+R, Ctrl+L, F3, Ctrl+V).</li>
<li><strong>BASIC</strong>: carrega o RDK G2 Basic integrado em <code>$E000</code> (ou o seu Integer BASIC, se o escolheu).</li>
<li><strong>Janelas</strong>: o menu <strong>⊞ Disposição ▾</strong> e depois um botão por painel (Tela monocromática, Tela colorida, Editor, Depurador, Desmontagem, Dump, Biblioteca, ASCII).</li>
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

Cada painel (ecrã, editor, depurador, biblioteca…) é um separador que pode mover, agrupar ou destacar numa janela própria: é o tema do capítulo 10. No primeiro arranque a janela abre-se maximizada e uma janela **Seu espaço de trabalho** propõe cinco disposições (capítulo 10); o tamanho e a posição da janela e a disposição dos seus painéis são memorizados e reaparecem tal como estavam no arranque seguinte.

## A barra de ferramentas, botão a botão

A barra de ferramentas é uma só linha: a máquina e as suas ferramentas à esquerda, o gravador ao centro, os painéis no grupo Janelas, as placas e, à direita, as definições. Quando a janela é demasiado estreita, os botões ficam só com o ícone (o texto fica no balão de ajuda), ou a barra passa a várias linhas se desmarcar **Barra compacta** nas preferências.

![A barra de ferramentas até ao grupo Placas gráficas: ficheiros, máquina, janelas e placas. Segue-se a faixa Cassete (capítulo 5) e depois as definições.](img/pt/barre-1.png "large")

| Botão | O que faz |
|---|---|
| {{Carregar}} | Abre um programa e põe-no na memória: um `.txt` (dump do Woz Monitor) ou um `.hex`; para um `.bin`, é pedido o endereço. As cassetes (`.wav`, `.aiff`) não passam por aqui: inserem-se com **📼 Inserir**, na faixa Cassete. |
| {{Guardar}} | Pede um ou mais intervalos (`0300.03FF`, `004A.00FF 0300.0FFF`…) e depois um ficheiro. A extensão escolhe o formato: `.wav`, `.aiff`, `.txt` ou `.bin`. |
| **RESET** (botão vermelho) | O botão RESET do Apple-1 (<kbd>Ctrl</kbd>+<kbd>R</kbd>). O 6502 e o PIA reiniciam; a memória e o ecrã são conservados. |
| **🆑 Limpar ecrã** | O botão CLEAR SCREEN (<kbd>Ctrl</kbd>+<kbd>L</kbd>). |
| **Indicador (Velocidade real / Velocidade rápida)** | Alterna entre o 1 MHz original e a velocidade máxima (<kbd>F3</kbd>). Um velocímetro: aceso a azul enquanto a velocidade rápida está ativa. |
| **📋 Colar** | Escreve a área de transferência no teclado do Apple-1, ao ritmo do teclado (<kbd>Ctrl</kbd>+<kbd>V</kbd>). Ideal para um dump hexadecimal. |
| {{BASIC|**BASIC**}} | Carrega o **RDK G2 Basic** integrado em `$E000` (7933 bytes; depois `E000R` arranca-o). Se escolheu o Integer BASIC original em **⚙ Preferências → Avançado → BASIC**, carrega esse: uma cassete, um dump `.txt` ou um `.bin` de 4096 bytes, procurado na biblioteca (`BASIC.wav`, `integer_basic.bin`, qualquer nome que contenha «basic»), ou então pedido. Botão direito: escolher outro ficheiro. |
| **Janelas** (grupo: **⊞ Disposição ▾** e um botão por painel) | Mostra ou esconde cada painel: **Tela monocromática** (o terminal do Apple-1), **Tela colorida** (o ecrã da placa gráfica ligada, GEN2 ou P-LAB: o mesmo botão serve as duas; sem placa ligada, propõe ligar uma, e **P-LAB** mostra então o seu ecrã), **📝 Editor** (capítulo 7), **🐛 Depurador**, **Desmontagem**, **Dump** (capítulo 8), **📚 Biblioteca** e **🎨 ASCII** (capítulo 9). Um botão aceso indica que o painel está visível; um painel já mostrado também se fecha com a cruz do seu separador. |
| **Placas gráficas: GEN2 e P-LAB** | Um grupo com nome. **GEN2** liga ou desliga a placa a cores de Uncle Bernie (capítulo 6); **P-LAB** faz o mesmo com a placa P-LAB de TMS9918A (secção 6.6). Um botão fica aceso enquanto a sua placa está ligada; o ecrã de uma placa aparece num painel próprio, mostrado pelo botão **Tela colorida**. |
| **📸 Estado ▾** | Um só botão com um menu: **Guardar estado…** (<kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>S</kbd>), **Carregar um estado…** (<kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>O</kbd>) e depois os últimos estados, cada um com o seu título e a sua data (secção 2.5). |
| **EPROM** (chip com janela UV, à direita da barra, antes de **⊞ Disposição**) | Abre o programador de EPROM: ler, escrever ou verificar um chip com um programador `minipro` (secção 2.6). |
| **Cassete: 📼 Inserir ▶ ⏹ ⏪ ⏺ ⏏** | O gravador, reunido numa faixa. **📼 Inserir** escolhe um `.wav` ou `.aiff` e põe-no no gravador como cassete virtual; depois reproduzir, parar, rebobinar, gravar, ejetar. A faixa mostra o nome, a posição e a duração. |
| **📈** e **🔊** | O osciloscópio da cassete e o som do sinal (capítulo 5). |
| **⊞ Disposição ▾** (primeiro botão do grupo Janelas) | Um menu: **Padrão** (todos os painéis, como no primeiro arranque), **Jogo** (ecrã do Apple-1 e biblioteca), **BASIC** (editor e ecrã), **Assembly** (editor, ecrã e depurador), **Depuração** (editor, depurador, desmontagem e dump); depois **Guardar a disposição atual…**, que pede um nome, e as suas disposições guardadas por ordem alfabética (botão direito: Mudar o nome, Eliminar). Um visto marca a última escolha; tudo fica guardado em `gui.json`. |
| **⚙** | As preferências (capítulo 11). |
| **?** | Modo de ajuda (<kbd>F1</kbd>): balões detalhados em cada controlo, nas 7 línguas da interface. |
| **ℹ** | «Acerca de», com a lista de atalhos de teclado. |

**Janelas de ficheiros.** Abrir, Guardar como e Exportar usam a janela de ficheiros própria do emulador. Um botão azul **📁+ Nova pasta**, em cima à esquerda, cria uma pasta na pasta mostrada, com um balão que o explica; escreva o nome, carregue em <kbd>Enter</kbd> e a pasta nova está pronta para receber o seu ficheiro.

![A janela Carregar, com o botão 📁+ Nova pasta em cima à esquerda.](img/pt/fenetre-fichiers.png "medium")

As definições de visualização não estão na barra de ferramentas: cada ecrã tem a sua própria **linha de visualização**, no topo do seu painel (secção 3.2).

![A linha de visualização do ecrã do Apple-1: zoom, fósforo, velocidade de visualização, ecrã vintage e ecrã inteiro; Destacar está na extremidade direita da linha.](img/pt/barre-2.png "large")

!!! astuce "Modo de ajuda"
    Quando explorar um painel, ative **?**: cada botão explica o que faz quando passa o rato por cima. Desativado, ficam só os balões curtos habituais (podem ser desativados em **⚙ → Interface**). A definição é memorizada.

## A barra de estado

![A barra de estado enquanto BASIC.wav é reproduzido: velocidade, limite de visualização, cassete, teclado e, por baixo, a última mensagem.](img/pt/barre-etat.png "large")

- **Velocidade medida**: cerca de 0,960 MHz à velocidade real. É o relógio efetivo do Apple-1, que para o 6502 durante quatro ciclos em cada sessenta e cinco para refrescar a sua memória dinâmica.
- **Ecrã ~60 car./s**: o terminal do Apple-1 só mostra um carácter por imagem, cerca de 60 por segundo. Um clique alterna entre este limite e a visualização instantânea, como os botões **Normal** / **Rápido** das linhas de visualização.
- **Cassete**: nome, posição e duração, ou «sem cassete».
- **Teclado**: para onde vão as teclas (para o Apple-1 ou para um campo de texto da interface).

## O teclado do Apple-1

Clique no ecrã: as teclas vão para o Apple-1. O teclado original só tem maiúsculas, e o emulador converte as minúsculas.

| Tecla do PC | Para o Apple-1 |
|---|---|
| letras, algarismos, símbolos | o carácter, em maiúscula |
| <kbd>Enter</kbd> | RETURN (código `$8D`) |
| <kbd>Retrocesso</kbd> | `_`: o Woz Monitor e o BASIC apagam o carácter anterior (fica no ecrã, seguido de `_`) |
| <kbd>Esc</kbd> | ESC: anula a linha em curso no Woz Monitor |
| <kbd>Ctrl</kbd>+letra | o carácter de controlo (`Ctrl+D` = `$84`…) |
| <kbd>Ctrl</kbd>+<kbd>R</kbd> / <kbd>Ctrl</kbd>+<kbd>L</kbd> | os botões RESET e CLEAR SCREEN |

Pode **selecionar texto com o rato** em ambos os ecrãs: carregue e arraste sobre os caracteres, e a seleção fica realçada. Um **clique com o botão direito** em qualquer dos ecrãs (o do Apple-1 ou o monitor GEN2) abre um pequeno menu: **Copiar** põe o texto selecionado na área de transferência (ou todo o texto do ecrã, se nada estiver selecionado), **Colar (escrever o texto)** escreve a área de transferência no teclado do Apple-1, como se fosse você a escrever, e depois **Limpar ecrã** e **Reset**.

![O menu do botão direito do ecrã do Apple-1, durante um jogo de Hamurabi.](img/pt/ecran-menu.png "medium")

!!! apple1 "O ecrã do Apple-1 não apaga nada"
    O terminal só sabe escrever um carácter e passar à linha seguinte. Não há cursor móvel nem forma de apagar um carácter: a tecla de apagar escreve `_` e o carácter fica no ecrã. É normal.

## Estado da máquina

O botão **📸 Estado** da barra de ferramentas tira um retrato completo da máquina e repõe-no mais tarde, exatamente no mesmo ponto: pode parar um jogo ou um programa em curso, fechar o emulador e continuar no dia seguinte. O botão abre um menu:

![O menu Estado: guardar, carregar e depois os últimos estados com a data.](img/pt/menu-etat.png "medium")

- **Guardar estado…** (<kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>S</kbd>) pede um nome de ficheiro e escreve um ficheiro `.a1state`. A pasta proposta é `etats`, junto à pasta da biblioteca.
- **Carregar um estado…** (<kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>O</kbd>) substitui a máquina pelo estado escolhido. A barra de estado mostra o ficheiro e o valor de PC.
- **Últimos estados** lista os oito últimos, com o título e a data em que foram guardados; um clique carrega um de imediato. Se o ficheiro foi movido ou eliminado, uma mensagem indica-o.

Um estado contém a memória, os registos do 6502, o PIA, o ecrã, a placa GEN2 se estiver ligada e a cassete virtual com a sua posição. Não contém as suas preferências nem a biblioteca. Um ficheiro que não se consiga ler (danificado, ou que não seja um estado) é recusado com uma mensagem, e a máquina fica intacta.

!!! astuce "Uso típico"
    Guarde um estado mesmo antes de uma passagem difícil de um jogo, ou no fim de uma longa sessão de BASIC: um estado é também uma forma cómoda de partilhar um programa em execução com um amigo que use a mesma versão.

## O programador de EPROM

**EPROM** na barra de ferramentas abre uma janela para ler, escrever e verificar um chip EPROM ou EEPROM a partir do emulador, por exemplo para gravar a ROM do BASIC, um programa para a expansão GEN2 ou o conteúdo da sua própria memória.

![O programador de EPROM: programador, chip, origem dos dados e as três operações.](img/pt/programmateur-eprom.png "medium")

Só são suportados os programadores controlados pela ferramenta livre **`minipro`**: os XGecu **TL866II+**, **T48** e **T56**. Em Debian, Ubuntu e Linux Mint, instala-se com `sudo apt install minipro`; ligue o programador a uma porta USB e carregue em **Detetar**. Se faltar o `minipro`, a janela di-lo e explica o que fazer; do lado do emulador não é preciso mais nada.

| Definição | Função |
|---|---|
| **Programador** | o programador detetado, com o seu firmware (por exemplo «Found TL866II+ 04.2.132 (USB)») |
| **Chip** | os chips habituais do mundo Apple-1: 2716, 2732, 2764, 27C64, 27C128, 27C256, 27C512, 28C64, 28C256, SST27SF256, SST27SF512 e a ROM de máscara 23128 (só leitura) |
| **Dados** | a **Memória do Apple-1** (um intervalo de endereços, `$E000`–`$EFFF` por omissão) ou um **ficheiro** (`.bin`, `.hex`, `.aiff`…) |

Três botões atuam sobre o chip, e uma barra de progresso e um registo mostram o que se passa:

- **Ler o chip** lê o chip inteiro. Se os dados forem a memória ou um ficheiro existente, o conteúdo é **comparado** e as diferenças são listadas: um ficheiro existente nunca é substituído. Com um nome de ficheiro novo, o conteúdo é guardado nele.
- **Escrever o chip** programa o chip com os dados escolhidos. O chip tem de estar virgem ou ser apagável; no fim, o resultado é confirmado.
- **Verificar** compara o chip com os dados escolhidos, byte a byte, e indica cada diferença (`$E010: chip $4C, dados $A9`).

**Fechar** fecha a janela.

A janela nunca altera a memória emulada: escrever e verificar leem-na, e ler apenas a compara ou preenche um ficheiro.

# O ecrã: zoom, fósforo e efeito vintage

## O ecrã original

O Apple-1 mostra 40 colunas × 24 linhas de caracteres em maiúsculas. Cada carácter é uma matriz de 5 × 7 pontos numa célula de 7 × 8, lida da ROM do gerador de caracteres **Signetics 2513**: o emulador usa o conteúdo dessa ROM, ponto a ponto (Anexo D). A imagem tem portanto 280 × 192 pontos.

O cursor é um `@` intermitente, comandado por um NE555: 2,18 Hz, visível cerca de dois terços do tempo. Em algumas placas, a tolerância do condensador dá 1,7 Hz; a opção `--cursor-hz 1.7` reproduz isso.

## A linha de visualização

Cada ecrã tem a sua própria linha de definições no topo do seu painel: o ecrã do Apple-1 e o monitor a cores quando a placa GEN2 está ligada (capítulo 6).

| Definição | Função |
|---|---|
| **Zoom** | uma lista: `1x` a `8x` ampliam a imagem por um fator inteiro, para que os pontos fiquem quadrados e nítidos; **Ajustar** escolhe o maior fator que cabe no painel (o fator escolhido aparece entre parênteses). Pelo teclado: <kbd>Ctrl</kbd>+<kbd>+</kbd>, <kbd>Ctrl</kbd>+<kbd>−</kbd>, <kbd>Ctrl</kbd>+<kbd>0</kbd> (ajustar). |
| **Fósforo** | só o ecrã do Apple-1: o verde dos monitores da época, o branco de um televisor ou o âmbar. |
| **Visualização: Normal / Rápido** | **Normal** mostra os caracteres ao ritmo do terminal verdadeiro, cerca de 60 por segundo; **Rápido** mostra tudo de uma vez. Esta definição é partilhada: mudá-la num ecrã muda-a no outro. |
| **Ecrã vintage** | o efeito de tubo dos anos 70 (secção seguinte); cada ecrã tem a sua própria caixa. |
| **⛶** | ecrã inteiro (<kbd>F11</kbd>). |
| **Destacar** | põe o ecrã numa janela própria (capítulo 10). |

Cada ecrã fica **centrado** no seu painel, na horizontal e na vertical, com a legenda centrada por baixo da imagem. Se o zoom escolhido for maior do que o painel, o ecrã é mostrado com o maior zoom que cabe, em vez de ser cortado. Quando a janela é mais estreita do que a linha de visualização, a linha passa a uma segunda linha, e **Destacar** fica na extremidade direita da última.

![Fósforo âmbar, com o ecrã vintage e uma ligeira curvatura.](img/pt/brut/ambre.png "medium")

## O ecrã vintage

A caixa **Ecrã vintage** da linha de visualização imita um monitor de tubo dos anos 70. O efeito principal é a fina linha escura entre duas linhas de varrimento: cada fila de pontos torna-se uma faixa horizontal, como na fotografia de um Apple-1 verdadeiro. Esta linha mantém a mesma espessura em píxeis do seu ecrã a qualquer zoom: continua fina e nítida, mesmo em ecrã inteiro.

![O ecrã vintage, com uma ligeira curvatura do tubo (0,15) e cantos escuros, durante um jogo de Hamurabi.](img/pt/brut/vintage.png "medium")

Em **⚙ Preferências → Avançado → Ecrã vintage: ajustes finos**, define a **Intensidade** e escolhe os efeitos; estas definições são comuns aos dois ecrãs. Cada cursor mostra o seu **valor numérico** ao lado (curvatura, intensidade…):

| Efeito | Por omissão | Função |
|---|---|---|
| separação das linhas de varrimento | ativado | a linha escura entre duas filas de pontos |
| pontos suavizados na horizontal | ativado | um ligeiro desfoque, só horizontal, como o feixe |
| remanência do fósforo | curta | os pontos apagam-se em menos de um décimo de segundo (um pouco mais devagar em âmbar); totalmente à esquerda: nenhuma |
| halo | ativado, discreto | um brilho à volta dos pontos, que continuam nítidos |
| tubo abaulado e cantos escuros | desativado | com um cursor **Curvatura** |
| cintilação | desativada | uma variação muito ligeira do brilho |
| máscara de fósforo RGB | ativada | só o ecrã a cores da placa GEN2: finas riscas verticais vermelhas, verdes e azuis separadas por linhas escuras, como no tubo de um televisor a cores. É visível a partir do zoom 3, e a sua força segue a **Intensidade** |

O efeito também pode ser aplicado ao monitor a cores da placa GEN2, com a caixa da sua própria linha de visualização. É calculado pelo processador, à resolução do Apple-1 e só quando a imagem muda: continua fluido mesmo sem aceleração gráfica.

Na linha de comandos: `--screening`, `--sans-screening`, `--gen2-screening` (monitor GEN2), ou por exemplo `--screening-options intensite=0.7,remanence=0.5,courbure=0.4,sans-halo` (valores de 0 a 1; os nomes das opções estão em francês).

!!! astuce "Ecrã destacado e segundo monitor"
    **Destacar**, no fim da linha de visualização, põe o ecrã do Apple-1 numa janela própria. Coloque-a num segundo monitor, em ecrã inteiro, com o efeito vintage: o teclado vai para o Apple-1 assim que essa janela tem o foco. Com **Ajustar**, a imagem segue o tamanho da janela, por passos inteiros (**Escala inteira**, na barra de título da janela, a mais nítida) ou preenchendo-a.
