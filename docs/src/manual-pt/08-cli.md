# A linha de comandos

<p class="lead">Dois programas: apple1-emu-gui, a interface gráfica, e apple1-emu, o emulador em terminal e caixa de ferramentas de conversão. Tudo o que os botões de exportação fazem pode também ser feito na linha de comandos, para os seus scripts.</p>

!!! attention "As ferramentas de linha de comandos falam francês"
    A interface gráfica existe em português (`--langue pt`), mas `apple1-emu` mostra as suas mensagens e a sua ajuda só em francês, e as opções da interface, os comandos de F2 e as palavras-chave do depurador são palavras francesas (`--carte-couleur`, `charger`, `pas`…). Este capítulo mostra a saída verdadeira e tradu-la onde for preciso.

## O emulador em terminal

`apple1-emu` sem comando executa o Apple-1 no terminal (pelo menos 42 colunas × 28 linhas): 8 KB de RAM mais `$E000`, ACI, velocidade real.

```sh
apple1-emu                                  # 8 KB + E000, ACI, velocidade real
apple1-emu --fast                           # arrancar em modo rápido
apple1-emu --load samples/hello.txt --run 300
apple1-emu --tape samples/hello.wav         # cassete no gravador virtual
apple1-emu --basic ~/apple1/integer_basic.bin
apple1-emu --ram 32 --no-e000               # 32 KB, sem RAM em E000
```

| Opção | Efeito |
|---|---|
| `--ram KB` | RAM a partir de `$0000`, de 4 a 48 KB (8 por omissão) |
| `--no-e000` | sem RAM em `$E000-$EFFF` (o jumper «W → E» da placa) |
| `--no-aci` | sem placa de cassete |
| `--basic FICHEIRO` | o seu Integer BASIC (binário de 4 KB ou dump hexadecimal), carregado em `$E000` |
| `--load FICHEIRO[@END]` | um programa para a memória (`.txt`, `.bin@END`, `.wav`, `.aif`); repetível |
| `--tape FICHEIRO` | uma cassete no gravador |
| `--type FICHEIRO` | escreve um ficheiro de texto ao arrancar |
| `--run END` | escreve `ENDR` depois de carregar |
| `--fast`, `--no-throttle` | modo rápido; visualização sem o limite de 60 caracteres/s |
| `--no-refresh` | sem o modelo de refrescamento da DRAM (1,0227 MHz em vez de 0,960) |
| `--carte-couleur`, `--couleur-50hz` | a placa GEN2 (registos e memória; a imagem a cores só existe na interface gráfica) |
| `--cursor-hz HZ`, `--cursor-duty F` | intermitência do cursor (2,18 Hz, visível 2/3 do tempo por omissão) |
| `--monitor-rom`, `--monitor-page` | outra imagem do Woz Monitor (256 ou 512 bytes, página 0 ou 1) |
| `--aci-rom`, `--aci-page` | outra imagem da PROM da ACI |

**Teclas do terminal.** <kbd>Ctrl</kbd>+<kbd>R</kbd> RESET, <kbd>Ctrl</kbd>+<kbd>L</kbd> CLEAR SCREEN, <kbd>F1</kbd> ajuda, <kbd>F2</kbd> linha de comandos, <kbd>F3</kbd> velocidade, <kbd>F4</kbd> limite de visualização, <kbd>F5</kbd>/<kbd>F6</kbd> reproduzir/parar a cassete, <kbd>F10</kbd> ou <kbd>Ctrl</kbd>+<kbd>Q</kbd> sair. O texto colado no terminal é escrito no teclado do Apple-1. O retrocesso escreve `_`, o apagamento do Woz Monitor.

**Comandos de F2** (as palavras dos comandos são francesas).

| Comando | Efeito |
|---|---|
| {{charger}} `FICHEIRO[@END]` | *carregar*: carrega na memória um `.txt`, um `.bin` (`@END` obrigatório), um `.wav` ou um `.aiff` |
| {{sauver}} `FICHEIRO INÍCIO.FIM …` | *guardar*: guarda a memória; a extensão escolhe o formato. Os intervalos podem ser copiados da linha da ACI: `sauver st.aiff 004A.00FFR 0300.0FFFR` |
| {{cassette}} `FICHEIRO` | *cassete*: insere uma cassete, que arranca na primeira leitura da ACI |
| `lecture`, `stop`, `rembobiner`, `ejecter` | o gravador: reproduzir, parar, rebobinar, ejetar |
| {{enregistrer}} `FICHEIRO` | *gravar*: arma o gravador; depois escreva `C100R` e `0300.03FFW` |
| `basic FICHEIRO` | carrega o seu Integer BASIC em `$E000` |
| `taper FICHEIRO` | *escrever*: escreve um ficheiro de texto no teclado |
| `reset`, `effacer`, `rapide`, `reel`, `debit`, `aide`, `quitter` | reset, apagar, rápido, velocidade real, ritmo de visualização, ajuda, sair |

## Conversões

O formato de saída é escolhido pela **extensão**: `.aif`/`.aiff` dá um AIFF (perfil smartphone), `.wav` um WAV, `.txt`/`.hex` um dump Woz, `.bin` um binário.

{{export}} (alias de `export-wav`) produz uma cassete. O sinal é criado pela ROM verdadeira da ACI a correr no 6502 emulado: os tempos dos bits são os de um Apple-1 a executar `W`.

```sh
apple1-emu export samples/hello.txt -o hello.aiff
apple1-emu export prog.bin@300 -o prog.wav --rate 48000 --bits 16
apple1-emu export startrek.txt --range "004A.00FFR 0300.0FFFR" -o startrek.aiff
```

| Opção | WAV (por omissão) | AIFF (por omissão) |
|---|---|---|
| `--rate` | 44 100 Hz | 48 000 Hz |
| `--bits 8\|16` | 16 | 16 |
| `--channels 1\|2` | 1 | 2, idênticos |
| `--amplitude` (de 0,05 a 1) | 0,8 | 0,89 (−1 dBFS) |
| `--silence S` | 0,5 s | 1,0 s |
| `--range` | um intervalo, ou uma linha ACI inteira; repetível | |
| `--no-refresh`, `--no-embed` | bits 6 % mais curtos; sem endereços no ficheiro | |

{{import}} (alias de `import-wav`) descodifica uma cassete: `-o prog.txt` ou `-o prog.bin`, `--addr END` para uma cassete sem endereços, `--range` para colocar várias gravações.

{{info}} descreve um ficheiro (gravações, endereços, duração); com `--aci-input stock|mimeo|gen2`, `--volts` e `--series`, simula a entrada analógica da ACI (secção 5.8).

{{convert}} `ENTRADA SAÍDA` converte entre `.txt`, `.bin`, `.wav` e `.aiff`.

## BASIC e assembler

```sh
apple1-emu basic jogo.bas -o jogo.wav          # listagem tokenizada -> cassete
apple1-emu basic jogo.bas -o jogo.txt --ram 16 # dump Woz, para 16 KB
apple1-emu basic jogo.bas                      # só verificar
apple1-emu asm jogo.s -o jogo.wav              # fonte 6502 -> cassete
apple1-emu asm jogo.s -o jogo.bin -l jogo.lst  # binário + listagem
apple1-emu asm jogo.s -I ../lib                # verificar, caminho de inclusão extra
```

{{basic (linha de comandos)|`basic`}} tokeniza uma listagem como o interpretador faz e escreve o bloco `$004A-$00FF` e depois o programa; `--ram 8|12|16|32` escolhe a máquina de destino (por omissão, a mais pequena que chegue). {{asm (linha de comandos)|`asm`}} monta uma fonte ca65 (ou xa); sem `-o`, só verifica. Ambos mostram a linha a escrever no Apple-1 verdadeiro.

```text title="Saída de apple1-emu asm hello.s -o hello.txt"
31 octets, $0300-$031E ; 2 fichier(s), 13 symboles
Cassette (après C100R) : 0300.031ER   puis 300R
écrit : hello.txt
```

Ou seja: «31 bytes, $0300-$031E; 2 ficheiro(s), 13 símbolos / Cassete (depois de C100R): 0300.031ER e depois 300R / escrito: hello.txt».

## Arte ASCII

{{ascii}} `IMAGEM`: `--cols`, `--rows` (23 para manter o prompt), `--zoom`, `--cx`, `--cy`, `--brightness`, `--contrast`, `--gamma`, `--invert`, `--dither`, `--no-auto-level`, `--ramp simple|full|slideshow|LISTA`, `--format asm|basic`, `--title`, `-o` (`.txt`, `.bas`, `.s`, `.wav`, `.aif`).

## PROM

{{prom}} prepara ficheiros para programar PROM verdadeiras de 256 × 4 (82S129, 3601, 6301), no formato de dumps de nibbles:

```sh
apple1-emu prom split woz -o proms/     # PROM-A2.bin (D7-D4) e PROM-A1.bin (D3-D0)
apple1-emu prom split aci -o proms/     # PROM-A3.bin (D7-D4) e PROM-A4.bin (D3-D0)
apple1-emu prom join A2.bin A1.bin      # juntar uma leitura e compará-la
```

`--swap-a1-a7` só serve para uma placa ligada segundo o esquema da Apple de 1976, que cruza as linhas de endereço 1 e 7 da PROM A2.

Para gravar uma EPROM de 2716 a 27C512 ou uma EEPROM 28C a partir da interface gráfica (um programador TL866II+, T48 ou T56 com `minipro`), veja o programador de EPROM, secção 2.6.

## A interface gráfica

`apple1-emu-gui [OPÇÕES] [FICHEIRO…]` carrega os ficheiros indicados ao arrancar. Opções principais (os nomes das opções são franceses):

| Opção | Efeito |
|---|---|
| `--zoom N\|ajuster`, `--taille LxA` | zoom do ecrã (`ajuster` = ajustar), tamanho da janela |
| `--phosphore vert\|blanc\|ambre` | fósforo (verde, branco, âmbar) |
| `--screening`, `--sans-screening`, `--screening-options LISTA` | ecrã vintage do Apple-1 e as suas definições |
| `--gen2-screening` | ecrã vintage no monitor a cores GEN2 |
| `--theme clair\|sombre\|systeme` | tema claro, escuro ou do sistema |
| `--taille-icones auto\|petite\|normale\|grande\|tres-grande` | tamanho dos ícones e do texto |
| `--barre-complete` | nunca reduzir a barra de ferramentas a ícones |
| `--bibliotheque PASTA`, `--basic FICHEIRO` | pasta da biblioteca, ficheiro BASIC |
| `--carte-couleur`, `--sans-carte-couleur`, `--couleur-50hz` | placa GEN2 |
| `--langue CÓDIGO\|auto` | língua da interface (`--langue pt` para português) |
| `--mode-rapide`, `--mode-cassette` | carregamento da biblioteca: instantâneo (por omissão) ou pela cassete à velocidade real |
| `--cassette FICHEIRO` | insere uma cassete |
| `--ecran-separe`, `--debogueur` | ecrã numa janela própria, depurador aberto |
| `--editeur FICHEIRO`, `--ascii FICHEIRO` | abre um ficheiro no editor ou na ferramenta ASCII |

Para demonstrações e capturas, algumas opções reproduzem ações ao fim de alguns segundos: `--lancer-biblio S:NOME`, `--charger-biblio S:NOME`, `--parametres S:NOME`, `--preferences S` (e `--onglet-prefs interface|editeur|bibliotheque|cassette|gen2|avance`), `--editeur-lancer S`, `--editeur-recherche TEXTO` (barra de pesquisa aberta), `--editeur-menu` e `--ecran-menu` (menus do botão direito abertos), `--menu-etat` (o menu Estado aberto), `--eprom` (o programador de EPROM aberto), `--fenetre-fichiers` (a janela Carregar aberta), `--editeur-exporter S:FICHEIRO`, `--ascii-reglages R`, `--ascii-lancer S`, `--taper TEXTO`, `--taper-a S:TEXTO`, `--dbg-arret`, `--dbg-surveiller`, `--dbg-commande S:CMD`, `--capture IMAGEM.png` (depois de `--delai S`) e `--quitter`.

# Resolução de problemas e perguntas frequentes

## A janela não abre

Falta o controlador OpenGL, ou está numa máquina virtual. `run.sh` volta a tentar automaticamente com a renderização por software do Mesa; à mão: `LIBGL_ALWAYS_SOFTWARE=1 ./apple1-emu-gui`. O ecrã vintage continua fluido com a renderização por software.

## As teclas não chegam ao Apple-1

Clique no ecrã do Apple-1. Quando um campo de texto ou uma caixa de diálogo está ativo, o teclado vai para esse campo; a barra de estado mostra para onde vai o teclado.

## «Integer BASIC introuvable» (Integer BASIC não encontrado)

O BASIC não é fornecido. Ponha o seu `BASIC.wav` (ou um `.bin` de 4096 bytes) na biblioteca, ou indique-o com o botão **BASIC** ou em **⚙ Preferências → Integer BASIC**. O emulador encontra-o então sozinho.

## Um programa BASIC para com STOPPED AT

Foi carregada uma tecla enquanto ele corria: no Apple-1, qualquer tecla interrompe um programa BASIC. Escreva `RUN` para recomeçar, ou `GOTO` seguido do número da linha para continuar sem apagar as variáveis.

## O programa não arranca depois de carregado

- Programa BASIC: é preciso `E2B3R` (entrada a quente), não `E000R`, que apaga o programa. O BASIC tem de estar na memória **antes** do programa.
- Código máquina: verifique o endereço de execução nas definições ⚙; nem sempre é `300R` (`280R`, `1000R`, `2000R`…).
- Programa com mais de 8 KB: a RAM é acrescentada pelo ▶, mas não por um carregamento manual. Arranque-o com ▶, ou use `--ram 16`.

## A cassete não carrega no Apple-1 verdadeiro

<ol class="steps">
<li>Volume no máximo, todos os efeitos de áudio desativados, sem notificações.</li>
<li>RETURN durante o cabeçalho, nem antes de a reprodução começar nem depois de o cabeçalho acabar.</li>
<li>Os intervalos exatos, com quatro algarismos: <code>004A.00FFR</code>, nunca <code>4A.FFR</code>.</li>
<li>Experimente outro leitor (VLC), depois <code>--rate 44100</code>, <code>--channels 1</code> ou <code>--no-refresh</code>.</li>
<li>Verifique o nível com <code>apple1-emu info prog.aiff --aci-input stock</code> (secção 5.8).</li>
<li>Se o carregamento nunca terminar: RESET e tente de novo com mais volume.</li>
</ol>

## O ecrã está cheio de @ em vídeo inverso

É o monitor a cores GEN2: a sua memória começa a zero, e o código `$00` é um `@` em vídeo inverso. O programa ainda não preencheu a página de texto. Na placa verdadeira, o conteúdo seria aleatório.

## Os caracteres saem devagar

É normal: o terminal do Apple-1 mostra cerca de 60 caracteres por segundo. Clique em **Ecrã: ~60 car./s** na barra de estado (ou em **Rápido** na linha de visualização do ecrã) para uma visualização instantânea, e passe ao modo rápido (<kbd>F3</kbd>) para acelerar tudo.

## O som da cassete está mudo

Está desligado à velocidade rápida, e quando faltam o ALSA ou a placa de som: o osciloscópio di-lo («mudo à velocidade máxima», «sem saída de áudio»). O volume ajusta-se no osciloscópio; 🔊 na barra de ferramentas liga ou desliga o som.

## O projeto GitHub não compila

«Testar a última compilação» compila com ca65/ld65 (pacote `cc65`) se não encontrar nem uma Release nem um artefacto. Instale o `cc65`, ou acrescente um `apple1.toml` que indique o ficheiro certo. Algumas fontes precisam de um ca65 recente (2.19 ou posterior); o erro do ca65 é então mostrado tal como está.

## O que o emulador não faz

- Os opcodes não documentados do 6502 são tratados como NOP do comprimento certo, e os opcodes JAM param o processador; os seus efeitos exatos não são reproduzidos (o software do Apple-1 não os usa).
- O comparador da ACI é ideal durante a emulação; só o comando `info` o simula como circuito analógico.
- A emulação foi verificada com as ROM, as listagens e ferramentas independentes (c2t, ca65, xa), mas o autor deste manual ainda não reproduziu os ficheiros de áudio num Apple-1 verdadeiro.
