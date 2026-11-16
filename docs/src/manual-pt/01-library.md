# A biblioteca

<p class="lead">A biblioteca é o ponto de partida de quase tudo: guarda as suas cassetes, dumps e fontes, reconhece as cassetes da época e executa um programa com um clique, BASIC incluído.</p>

![O painel da biblioteca: a pasta principal, a pasta «Art ASCII», cassetes reconhecidas e um projeto GitHub. BASIC.wav está a ser reproduzido.](img/pt/bibliotheque.png "small")

## Pastas

A pasta principal é `~/apple1/programmes`. É criada no primeiro arranque, com os exemplos repartidos por cinco pastas: **Art ASCII** (onze exemplos de arte ASCII em 40 × 24, capítulo 9), **BASIC** (programas `.bas` e dumps BASIC `.hex.txt`), **6502** (fontes `.s`/`.asm` com os seus `.bin`/`.hex`), **GEN2 COLOR** (os testes da placa GEN2 de Bernie, `gen2-*.apl`, e os jogos a cores para G2 Basic) e **WAV** (todas as cassetes). No painel, essas cinco pastas aparecem diretamente no nível superior, pela ordem **BASIC**, **6502**, **GEN2 COLOR**, **WAV**, **Art ASCII**: a pasta raiz (o nome do diretório no disco) não é mostrada por cima, e os ficheiros soltos nessa pasta não são listados. Coloque-os numa das cinco pastas. Uma pasta que acrescente, ou um projeto GitHub, mantém o seu próprio título. Uma biblioteca já existente nunca é reorganizada, exceto que as suas pastas **Assembleur** e **Uncle Bernie** (os nomes das versões 7.8 a 7.9.1) passam a chamar-se **6502** e **GEN2 COLOR**, com o seu conteúdo. Para escolher outra: **⚙ Preferências → Pasta da biblioteca → 📂 Procurar…** (**⟲ Predefinida** volta a `~/apple1/programmes`).

**+ Pasta**, no topo do painel, acrescenta outras pastas; a pasta principal fica sempre. A biblioteca é uma árvore: pastas e subpastas recolhem-se, e o seu estado é memorizado. Cada pasta guarda as suas próprias definições num ficheiro `programs.json`. Clique com o botão direito numa pasta acrescentada para a abrir no gestor de ficheiros ou a retirar da lista (nada é apagado do disco).

**Painel estreito.** A biblioteca pode ser tão estreita quanto quiser (até uma largura mínima, abaixo da qual o separador para). Cada entrada adapta-se então: o nome ocupa toda a largura, cortado com «…» se for demasiado longo (o balão mostra o nome completo), os pequenos ícones ▶ ⬇ 📼 ⚙ passam para a linha seguinte e a descrição fica limitada a uma ou duas linhas, também cortada com «…», com o texto completo no balão. No cabeçalho, **+ Pasta** e **+ Projeto GitHub** tornam-se ícones com balões.

Os ficheiros mostrados são: `.wav .aiff .bin .hex .txt .apl .bas .s .asm .prg`. A lista atualiza-se sozinha a cada dois segundos: copie um ficheiro para a pasta e ele aparece.

Para cada programa, a lista mostra o nome, o tipo, os intervalos de endereços, a duração da cassete, o comando de execução e as suas notas.

### Os jogos de Claude IA

As pastas **BASIC** e **6502** recebem também cinco pequenos jogos escritos por Claude IA, cada um numa versão francesa e numa inglesa: *Forca* (`pendu.bas`, `hangman.bas`), *Fósforos* (`allumettes.bas`, `matches.bas`) e *Batalha naval* (`bataille.bas`, `battleship.bas`) em BASIC; *Túnel* (`tunnel.s`, `tunnel-en.s`) e *Jogo do galo* (`morpion.s`, `tictactoe.s`) em assembler 6502. Um duplo clique num jogo BASIC executa-o com o RDK G2 Basic (o emulador arranca o BASIC, carrega a listagem e escreve `RUN`); as fontes em assembler são montadas pelo assembler integrado. As fontes em assembler incluem `apple1.inc`, que é instalado com elas. A pasta **GEN2 COLOR** contém ainda *Quatro em linha a cores* (`puissance4-couleur.bas`), um jogo para G2 Basic e a placa GEN2, de Claude IA. Q e S movem a peça por cima da grelha, Espaço deixa-a cair. Os jogos fazem parte do projeto e estão sob a sua licença (anexo «Licença e créditos»). As versões portuguesas destes jogos estão em `docs/exemples/pt/`.

## Executar um programa

Cada programa tem quatro pequenos ícones à direita:

| Ícone | Ação |
|---|---|
| **▶** | **Carregar e executar**: carrega o programa se for preciso e depois executa-o: RESET e a seguir o seu comando escrito no teclado (`300R`, ou `E2B3R` e `RUN` para um programa BASIC). Se o programa precisar do Integer BASIC e este não estiver em `$E000`, o emulador carrega-o primeiro. Uma listagem BASIC é escrita no Integer BASIC; uma fonte 6502 é montada. |
| **⬇** | **Carregar**: põe o programa na memória de imediato, sem o arrancar. |
| **📼** | **Pela cassete**: como na máquina verdadeira. O emulador carrega em RESET, escreve `C100R` e depois o comando de leitura da ACI (por exemplo `0300.031ER`), e a cassete é reproduzida à velocidade real, com o seu som e uma barra de progresso. Os ficheiros `.txt` e `.bin` são primeiro convertidos em cassete. Desativado para uma fonte (`.bas`, `.s`), que não tem gravação. |
| **⚙** | **Parâmetros**: intervalos, endereço de carregamento de um `.bin`, endereço e comando de execução, tipo, se é preciso BASIC, notas. |

{{!duplo clique}}O carregamento é **instantâneo**, a menos que escolha a cassete com 📼. Um **duplo clique** num programa carrega-o e executa-o se tiver comando de execução (como ▶). A cassete só é usada quando a pede; o gravador da barra de ferramentas mostra então a cassete e a sua posição.

Quando o editor está aberto, ▶, ⬇ ou um duplo clique numa listagem BASIC ou numa fonte 6502 também a **abrem num separador do editor**, marcado com 📚; se já estiver aberta, o seu separador é simplesmente mostrado.

Um **clique com o botão direito** num programa oferece o resto: carregar, carregar e executar, inserir como cassete sem escrever nada, parâmetros, mudar o nome, eliminar (com confirmação), abrir a pasta.

## As definições de um programa

![As definições ⚙ de Hamurabi: dois intervalos (página zero do BASIC e depois o programa), arrancado com E2B3R e depois RUN.](img/pt/brut/bibliotheque-params.png "large")

| Campo | Função |
|---|---|
| **Intervalos** | As zonas de memória, escritas como para a ACI: `0300.031E`, ou `004A.00FF 0800.0FFF` para um programa BASIC. Cada gravação da cassete vai para o seu intervalo, por ordem. |
| **Endereço de carregamento** | Para um `.bin`, que não contém endereço. |
| **Executar** | O endereço e o comando escritos por ▶. `\n` representa RETURN entre duas linhas: `E2B3R\nRUN`. |
| **Tipo** | **Código máquina**, **Programa G2 Basic** (o valor por omissão para uma listagem BASIC, já que o emulador o fornece) ou **Programa Integer BASIC** (para quem prefere o original da Apple: ▶ carrega então o Integer BASIC original escolhido nas preferências). O tipo escolhido para um programa prevalece sobre a definição geral do BASIC nas preferências. |
| **Precisa de um BASIC** | ▶ carregará primeiro o BASIC do tipo do programa. |
| **Notas** | Texto livre, mostrado na lista. |

Os valores são preenchidos a partir do ficheiro e verificados ao guardar; **⟲ Valores do ficheiro** volta a lê-los.

!!! attention "Sempre quatro algarismos: 004A.00FF, nunca 4A.FF"
    As definições recusam `4A.FF`. Não é um capricho: a ROM da ACI não limpa o seu acumulador hexadecimal no ponto, pelo que os algarismos continuam a entrar e leria `$004A-$4AFF` (ver secção 5.6). Por isso o emulador escreve sempre os endereços com quatro algarismos.

## Cassetes reconhecidas

Quando um ficheiro é **reconhecido**, as suas definições preenchem-se sozinhas e a lista mostra **✔ reconhecido**. O emulador descodifica as gravações e compara os seus tamanhos e impressões digitais SHA-256 com uma tabela integrada: as 24 cassetes da coleção «Apple1Cassettes» do archive.org. O nome do ficheiro deixa então de ter qualquer importância. Na falta disso, reconhece o **nome** (sem ligar a maiúsculas, espaços ou hífenes) com os mesmos tamanhos de gravação; o balão de ajuda assinala então que o conteúdo é diferente.

| Cassete | Intervalos | Execução |
|---|---|---|
| `BASIC.wav` (Integer BASIC) | `E000.EFFF` | `E000R` |
| `Hamurabi.wav`, `StarTrek.wav`, `Wumpus.wav`, `Slots.wav`, `Craps.wav`, `Bowling.wav`, `Buzzword.wav`, `WordCross.wav`, `AppleiEnhancedCheckers.wav`, `LunarlanderasciiGraphics.wav` | `004A.00FF 0300.0FFF` | `E2B3R ⏎ RUN` |
| `AceyDucey.wav`, `Blackjack.wav`, `HiLo.wav` | `004A.00FF 0800.0FFF` | `E2B3R ⏎ RUN` |
| `Football.wav` | `004A.00FF 0400.2FFF` | `E2B3R ⏎ RUN` (12 KB) |
| `DealOrNoDeal.wav`, `StarTrek2003.wav` | `004A.00FF 0300.3FFF` | `E2B3R ⏎ RUN` (16 KB) |
| `APPLE30TH.wav` | `0280.0FFF` | `280R` |
| `ASMmchess.wav` | `0300.0BFF` | `300R` |
| `Microchess.wav` | `1000.18FF` | `1000R` |
| `LunarLander.wav` | `0300.09FF` | `300R` |
| `Mastermind.wav` | `0300.03FF` | `300R` |
| `Pasart.wav` | `0300.05FF` | `300R` |
| `Life.wav` | `2000.21FF` | `2000R` (12 KB) |

As suas próprias definições nunca são substituídas: um programa que configurou recebe só a marca «reconhecido», e as suas definições ficam. Os programas que ultrapassam os 8 KB de base recebem automaticamente a RAM de que precisam ao arrancar, como com uma placa de expansão.

**Cassete desconhecida.** O emulador adivinha o que pode: uma gravação de 4096 bytes que começa por `JMP` é o Integer BASIC; duas gravações das quais a primeira tem 182 bytes (`$4A-$FF`) são um programa BASIC, colocado segundo os seus ponteiros LOMEM e HIMEM. A lista mostra então «(supostos)»; verifique com ⚙. Um programa em código máquina sem endereços não é adivinhado: indique o seu intervalo.

![Digitalizar uma cassete verdadeira: da cassete para um ficheiro e depois para a biblioteca.](diagrams/pt/workflow-capture.svg "diagram large")

## Projetos GitHub

**+ Projeto GitHub** aceita um endereço (`https://github.com/jefftranter/6502/tree/master/asm/jmon`) ou simplesmente `proprietário/repositório`. Os ficheiros úteis são copiados para `~/.cache/apple1-emu/github/<proprietário>/<repositório>/<commit>/`, e o projeto torna-se um nó da árvore, com o seu commit, a data e a mensagem. **⟳ Atualizar** vai buscar o último commit; 🌐 abre a página do repositório. Todo o acesso à rede é feito em segundo plano.

{{Testar a última compilação}} tenta, por ordem:

<ol class="steps">
<li>os ficheiros da última <strong>Release</strong> do repositório (<code>.wav</code>/<code>.aiff</code>, depois <code>.bin</code>, depois <code>.hex</code>/<code>.txt</code>);</li>
<li>o último <strong>artefacto do GitHub Actions</strong> (é preciso um token);</li>
<li><strong>a compilação a partir das fontes</strong>: uma listagem BASIC é escrita no Integer BASIC e arrancada com <code>RUN</code>; uma fonte 6502 é montada com <strong>ca65/ld65</strong> (pacote <code>cc65</code>), com <code>-D APPLE1</code> por omissão; um dump Woz é carregado.</li>
</ol>

![Um projeto GitHub, do endereço do repositório ao programa em execução.](diagrams/pt/workflow-github.svg "diagram large")

Um ficheiro `apple1.toml` (ou `apple1.cfg`) na raiz do repositório diz o que construir:

```toml title="apple1.toml"
file    = "src/game.s"    # programa a executar
load    = $0300           # endereço de carregamento (senão, o .org da fonte)
run     = $0300           # endereço de execução (senão, o de carregamento)
ld65    = "apple1.cfg"    # configuração do ld65 (senão, -t none)
defines = "APPLE1"        # símbolos -D para o ca65
```

!!! astuce "O token GitHub é opcional"
    Os repositórios públicos não precisam dele. Um token (variável `GITHUB_TOKEN`, ou **⚙ Preferências**) aumenta o limite de pedidos e dá acesso aos seus repositórios privados e aos artefactos. Nunca é mostrado nem escrito nas mensagens, e `gui.json` passa então a ser legível só pela sua conta.

## A pasta «Art ASCII»

A biblioteca contém também uma pasta **Art ASCII**: desenhos no formato do ecrã (maçã, placa do Apple-1, chip 6502, cassete, foguetão…) e retratos de Steve Wozniak e Steve Jobs convertidos a partir de fotografias de licença livre. Um `.txt` de arte ASCII mostra uma pré-visualização quando é selecionado, com **→ BASIC**, **→ 6502** e **🎨 Abrir na ferramenta**; **▶** executa de imediato a sua versão 6502. O capítulo 9 descreve a ferramenta.
