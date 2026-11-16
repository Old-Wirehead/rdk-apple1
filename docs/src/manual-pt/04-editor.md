# O editor

<p class="lead">O editor integrado escreve programas em Integer BASIC e fontes 6502, verifica-os a cada tecla, executa-os no Apple-1 emulado com um clique e exporta-os como cassetes para a máquina verdadeira.</p>

## Separadores e ficheiros

**📝 Editor** abre o painel, acoplado à esquerda do ecrã. A sua barra de ferramentas é compacta, para deixar o máximo de linhas ao texto. Começa com ícones, explicados pelos seus balões:

| Ícone | Ação |
|---|---|
| **Novo** (primeiro ícone) | Novo separador: «Programa BASIC» ou «Fonte assembler 6502» |
| **📂** / **💾** / **💾…** | Abrir (`.bas`, `.txt`, `.s`, `.asm`, `.a65`, `.inc`) / Guardar (<kbd>Ctrl</kbd>+<kbd>S</kbd>) / Guardar como… |
| **✂**, copiar, **📋** | Cortar, copiar, colar |
| **🔍** | Procurar e substituir (<kbd>Ctrl</kbd>+<kbd>F</kbd>, <kbd>Ctrl</kbd>+<kbd>H</kbd>) |
| **⟲ ⟳** | Anular (<kbd>Ctrl</kbd>+<kbd>Z</kbd>), refazer (<kbd>Ctrl</kbd>+<kbd>Shift</kbd>+<kbd>Z</kbd>) |
| **A▲▼** | Tamanho do texto: um controlo compacto, a letra A com dois pequenos triângulos sobrepostos; ▲ aumenta, ▼ reduz |

A seguir, na mesma linha (ou na seguinte se o painel for estreito), vêm as ações do separador: **▶ Executar**, **📼 Exportar WAV** e **Memória** para o BASIC, ou **📃 Listagem**, **⏺ Ponto de paragem** e **⏩ Até ao cursor** para uma fonte 6502.

Cada ficheiro é um separador. Um ponto • marca um separador modificado; × fecha-o (se não estiver guardado, é preciso um segundo clique). Um separador marcado com **📚** foi aberto a partir da biblioteca: abrir uma listagem ou uma fonte da biblioteca (▶, ⬇ ou duplo clique) abre o seu separador quando o editor está visível, ou mostra-o se já estiver aberto. Os separadores, incluindo o texto por guardar, regressam no arranque seguinte. Um clique com o botão direito num separador oferece **Guardar como…** e **Fechar**. No texto, um **clique do botão do meio** cola a seleção primária (Linux): o cursor vai para onde clica e depois é inserido o último texto realçado em qualquer aplicação.

O tamanho do texto ajusta-se com o controlo **A▲▼**, <kbd>Ctrl</kbd>+roda, ou <kbd>Ctrl</kbd>+<kbd>+</kbd> / <kbd>Ctrl</kbd>+<kbd>−</kbd> / <kbd>Ctrl</kbd>+<kbd>0</kbd> quando o texto tem o foco do teclado.

**Destacar** põe o editor numa janela própria, por exemplo num segundo ecrã; **⊟ Ancorar** volta a colocá-lo no lugar. As teclas escritas nessa janela vão para o editor, nunca para o Apple-1.

## Procurar, substituir e o menu do botão direito

**🔍** (ou <kbd>Ctrl</kbd>+<kbd>F</kbd>) abre uma barra de pesquisa por cima do texto: <kbd>Enter</kbd> vai para a ocorrência seguinte, <kbd>Shift</kbd>+<kbd>Enter</kbd> para a anterior, **Aa** distingue maiúsculas, **Palavra** procura palavras inteiras, e o contador mostra «3 de 7». <kbd>Ctrl</kbd>+<kbd>H</kbd> acrescenta o campo **Substituir por**, com **Substituir** (a ocorrência atual) e **Substituir tudo**. <kbd>Esc</kbd> fecha a barra.

![A barra de pesquisa aberta sobre ECHO, e o menu do botão direito de uma fonte 6502.](img/pt/editeur-menu.png "medium")

Um **clique com o botão direito** no texto abre um menu com todos os comandos e os seus atalhos. O menu atua sobre a **seleção**, que se mantém ao clicar com o botão direito sobre ela: **Cortar** e **Copiar** trabalham com o texto selecionado. Os comandos baseados numa palavra (ir para a definição, ajuda) usam a **palavra sob o clique**, não a que está sob o cursor.

| Comando | Atalho |
|---|---|
| Cortar, Copiar, Colar, Selecionar tudo | <kbd>Ctrl</kbd>+<kbd>X</kbd>, <kbd>C</kbd>, <kbd>V</kbd>, <kbd>A</kbd> |
| Procurar, Substituir | <kbd>Ctrl</kbd>+<kbd>F</kbd>, <kbd>Ctrl</kbd>+<kbd>H</kbd> |
| Ir para a linha… | <kbd>Ctrl</kbd>+<kbd>G</kbd> |
| Comentar / descomentar (6502), REM / tirar REM (BASIC) | <kbd>Ctrl</kbd>+<kbd>/</kbd> |
| Indentar, Remover indentação | |
| Ir para a definição de uma etiqueta (6502) ou de um número de linha (BASIC) | <kbd>F12</kbd> |
| Ajuda sobre *palavra* (numa palavra-chave, numa instrução ou numa diretiva: abre a sua entrada na referência) | |
| Ver no desassemblado (6502) | |
| Renumerar… (BASIC: `GOTO`, `GOSUB` e `THEN` acompanham) | |
| Ponto de paragem nesta linha | <kbd>F9</kbd> |
| Assemblar e executar (6502), Enviar e executar (BASIC) | <kbd>F5</kbd> |

**Cores.** As cores do texto seguem o tema da interface. **⚙ Preferências → Editor** oferece, separadamente para o BASIC e para o assembler 6502, os esquemas *Automático*, *Clássico (escuro)*, *Claro*, *Âmbar*, *Alto contraste* ou as suas próprias cores, uma amostra por tipo de palavra (capítulo 11).

## Um programa BASIC

![O editor BASIC: erros sobre fundo vermelho, com o número de linha e a coluna no fundo do painel.](img/pt/editeur-erreur.png "large")

O texto é colorido: números de linha, palavras-chave, números, cadeias, `REM`, operadores. Cada linha é **tokenizada** como o Integer BASIC o faria, por um tokenizador nativo comparado byte a byte com o verdadeiro. Uma linha que o BASIC recusaria fica a vermelho, e o erro indica a linha do texto, o número de linha BASIC e a coluna; um clique leva lá o cursor. Erros detetados: sintaxe, nome de variável inválido, número maior do que 32767, comando do modo direto (`LIST`, `RUN`…) dentro do programa, cadeia não fechada, linha com mais de 255 bytes, memória cheia.

- **Memória**: automática (8 KB com LOMEM `$0800`, ou `$0300` se o programa for grande; senão 12, 16 ou 32 KB), ou forçada.
- {{▶ Executar}}: carrega o Integer BASIC se faltar (como o botão BASIC), põe na memória o programa tokenizado, acrescenta RAM se for preciso e depois escreve `E2B3R` e `RUN`. É instantâneo: nada é escrito linha a linha.
- {{📼 Exportar WAV}}: uma cassete (`.wav`, ou `.aiff` para um smartphone) de duas gravações, com as suas definições na biblioteca. No fundo do painel aparece o que escrever no Apple-1 verdadeiro, por exemplo `C100R`, depois `004A.00FFR 0C3B.0FFFR` e a seguir `E2B3R` e `RUN`; **📋 Copiar** põe isso na área de transferência.

![Do editor ao Apple-1 verdadeiro: escrever, executar, corrigir, exportar.](diagrams/pt/workflow-basic.svg "diagram large")

## Uma fonte 6502

![Uma fonte 6502 no editor: as cores, o resultado da montagem e a linha a escrever no Apple-1 verdadeiro.](img/pt/brut/editeur-asm.png "large")

Uma fonte é montada **a cada alteração** pelo assembler integrado de duas passagens, que aceita a sintaxe do ca65 (e a do xa, traduzida automaticamente). No fundo do painel aparecem o tamanho e os endereços (`33 bytes, $0300-$0320 — 2 ficheiro(s), 13 símbolos`) e depois a linha para a máquina verdadeira (`C100R`, depois `0300.0320R` e a seguir `300R`).

- **Erros**: ficheiro, linha e coluna; um clique leva lá, abrindo o ficheiro incluído se for preciso. Os avisos (página zero codificada como absoluta) aparecem a amarelo.
- **📃 Listagem**: endereço, bytes e fonte de cada linha, com os ficheiros incluídos e as macros expandidas, e depois a tabela de símbolos; um clique leva de volta à linha.
- **▶ Executar**: põe os bytes na memória na origem (`.org`, `$0300` por omissão), acrescenta RAM se for preciso e depois escreve por exemplo `300R`.
- **📼 Exportar WAV**: a cassete, como para o BASIC.
- **⏺ Ponto de paragem** e **⏩ Até ao cursor**: a ligação com o depurador (capítulo 8).
- **Ligação com o depurador**: quando seleciona uma instrução na fonte (ou põe o cursor na sua linha), os bytes que ela produziu são realçados na **desmontagem** do depurador e no **dump hexadecimal / ASCII**, se esses painéis estiverem visíveis; se estiverem noutra parte da memória, os painéis deslocam-se para os mostrar. A tabela de linhas da fonte e endereços do assembler torna isso possível.

![Um erro de montagem: ECO em vez de ECHO, um símbolo desconhecido, assinalado com a sua linha e coluna.](img/pt/brut/editeur-asm-erreur.png "large")

A Parte III é inteiramente dedicada ao assembler: sintaxe, diretivas, listagem, exemplos.
