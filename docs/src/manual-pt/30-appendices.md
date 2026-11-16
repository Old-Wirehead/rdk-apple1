# Apêndices <span class="part-sub">Mapa da memória, comandos do Woz Monitor, tempos da ACI, conjunto de caracteres, atalhos de teclado, armadilhas, licença e créditos</span> {: .part data-part="Apêndices" }

# Mapa da memória {: .appendix }

![A memória do Apple-1 no emulador, dos vetores do monitor à página zero.](diagrams/pt/memoire.svg "diagram large")

| Endereços | Conteúdo | Pormenores |
|---|---|---|
| `$0000-$00FF` | página zero | Woz Monitor `$24-$2B`; Integer BASIC `$4A-$FF` (ponteiros LOMEM `$4A`, HIMEM `$4C`, PP `$CA`, PV `$CC`) |
| `$0100-$01FF` | pilha do 6502 | |
| `$0200-$027F` | buffer de linha do monitor | 128 caracteres |
| `$0280-$0FFF` | RAM de base (4 KB) | programas em código máquina; o BASIC entre o LOMEM (`$0800`) e o HIMEM (`$1000`) |
| `$1000-$BFFF` | RAM de expansão | 8 KB por omissão no emulador (até `$1FFF`), até 48 KB com `--ram` |
| `$C000-$C0FF` | ACI: inversão da saída, entrada de cassete | qualquer acesso inverte a saída |
| `$C100-$C1FF` | ACI: PROM (o programa da ACI) | arrancado com `C100R` |
| `$4000-$7FFF` | P-LAB CodeTank | EPROM de 16 KB, só de leitura, quando há uma ROM inserida (por cima da RAM) |
| `$C250-$C257` | placa GEN2: comutadores | leitura; páginas texto/LORES `$0400`, `$0800`, HIRES `$2000`, `$4000` |
| `$CC00-$CC01` | placa P-LAB: TMS9918A | `$CC00` dados, `$CC01` controlo e estado; repetido em `$CC00-$CFFF` |
| `$D010-$D013` | PIA: `KBD`, `KBDCR`, `DSP`, `DSPCR` | capítulo 22 |
| `$E000-$EFFF` | BASIC | RDK G2 Basic (7933 bytes, precisa de RAM até `$FEFF`), ou o Integer BASIC da Apple (4 KB de RAM, carregado de cassete ou de uma imagem) |
| `$FF00-$FFFF` | Woz Monitor | vetores: NMI `$0F00`, RESET `$FF00`, IRQ/BRK `$0000` |

# Comandos do Woz Monitor {: .appendix }

O monitor espera por uma linha terminada com <kbd>RETURN</kbd>. Os endereços e os bytes escrevem-se em hexadecimal, sem `$`; uma linha pode conter vários comandos separados por espaços.

| Comando | Efeito | Exemplo |
|---|---|---|
| `end` | mostra o byte nesse endereço | `FFEF` |
| `end.end` | mostra um intervalo, 8 bytes por linha | `FF00.FF1F` |
| `.end` | continua a visualização até esse endereço | `.FF3F` |
| `end: byte byte …` | guarda bytes a partir do endereço | `300: A9 C1 20 EF FF` |
| `: byte …` | continua a guardar | `: 4C 1F FF` |
| `endR` | executa o programa nesse endereço (`JMP`) | `300R` |
| <kbd>Esc</kbd> | anula a linha (mostra `\`) | |
| `_` (retrocesso) | anula o último carácter | |

{{!endR}}{{!end.end}}

Uma linha com mais de 127 caracteres é anulada como com Esc. Depois de `R`, o monitor mostra primeiro o byte no endereço (`0300: A2`) e depois o programa toma conta.

| Endereço | Comando | Papel |
|---|---|---|
| `E000R` | arranque a frio do BASIC | apaga o programa |
| `E2B3R` | arranque a quente do BASIC | mantém o programa |
| `C100R` | o programa da ACI | depois `início.fimR` (ler) ou `início.fimW` (gravar) |
| `FF00R` | RESET por software do monitor | |

# Tempos da ACI {: .appendix }

As durações são produzidas pelo programa da ACI, contando ciclos entre duas inversões da saída. Valores medidos no emulador, que executa esse programa no seu 6502 (relógio de 1,022727 MHz, com a paragem de 4 ciclos a cada 65 ciclos para o refrescamento da memória):

| Elemento | Ciclos | Duração | Frequência |
|---|---|---|---|
| semiperíodo do cabeçalho | 592 | 618 µs | ≈ 809 Hz |
| primeira metade do bit de arranque | 180 | 188 µs | |
| semiperíodo de um «0» | 238 | 248 µs | ≈ 2,0 kHz |
| semiperíodo de um «1» | 473 | 494 µs | ≈ 1,0 kHz |
| cabeçalho completo | 16 390 semiperíodos | ≈ 10,1 s | |

Formato de uma gravação:

<ol class="steps">
<li>o <strong>cabeçalho</strong>: cerca de 10 segundos de semiperíodos de 618 µs;</li>
<li>o <strong>bit de arranque</strong>: um semiperíodo curto (188 µs), depois um semiperíodo de «0»;</li>
<li>os <strong>dados</strong>: cada byte, bit mais significativo primeiro, um período completo por bit;</li>
<li>mais nada: nem comprimento, nem endereço, nem soma de controlo. É o comando de leitura que dá o intervalo.</li>
</ol>

Na leitura, o programa da ACI mede os semiperíodos com um ciclo de 12 ciclos de relógio. Toma como bit de arranque qualquer semiperíodo com menos de 31 voltas (≈ 372 ciclos, 388 µs), e separa «0» de «1» por volta de 0,74 ms por período completo. O descodificador de ficheiros do emulador aceita cabeçalhos de 0,40 a 0,90 ms por semiperíodo, o que cobre a ferramenta c2t (1 kHz) e as variações de velocidade da fita.

# O conjunto de caracteres do 2513 {: .appendix }

![Os 64 caracteres do Signetics 2513, tal como o emulador os desenha: códigos $20 a $5F (com o bit 7, $A0 a $DF).](img/charset-2513.png "large")

O gerador de caracteres só conhece 64 símbolos: espaço, pontuação, algarismos e maiúsculas. O terminal só guarda 6 bits do código que recebe:

| Código enviado | Visualização |
|---|---|
| `$20-$5F` (ou `$A0-$DF`) | o carácter da tabela |
| `$60-$7F` (minúsculas, ou `$E0-$FF`) | a maiúscula correspondente (`$40-$5F`) |
| `$0D` (ou `$8D`) | linha nova |
| outros `$00-$1F` | nada: ignorados |

Cada carácter é uma matriz de 5 × 7 pontos numa célula de 7 × 8; o ecrã tem 40 × 24 caracteres, ou seja 280 × 192 pontos.

# Atalhos de teclado {: .appendix }

| Tecla | Efeito |
|---|---|
| <kbd>Ctrl</kbd>+<kbd>R</kbd> | RESET do Apple-1 |
| <kbd>Ctrl</kbd>+<kbd>L</kbd> | CLEAR SCREEN |
| <kbd>Ctrl</kbd>+<kbd>V</kbd> | colar: escreve a área de transferência no teclado do Apple-1 |
| <kbd>F1</kbd> | modo de ajuda (balões de ajuda pormenorizados) |
| <kbd>F3</kbd> | velocidade real ou rápida |
| <kbd>F11</kbd> | ecrã inteiro |
| <kbd>Ctrl</kbd>+<kbd>+</kbd> / <kbd>Ctrl</kbd>+<kbd>−</kbd> / <kbd>Ctrl</kbd>+<kbd>0</kbd> | zoom do ecrã (ou do texto do editor, quando o editor tem o foco do teclado) |
| <kbd>Ctrl</kbd>+<kbd>S</kbd> | guardar o ficheiro do editor |
| <kbd>Ctrl</kbd>+<kbd>F</kbd> / <kbd>Ctrl</kbd>+<kbd>H</kbd> / <kbd>Ctrl</kbd>+<kbd>G</kbd> | editor: procurar, substituir, ir para a linha |
| <kbd>Ctrl</kbd>+<kbd>/</kbd> | editor: comentar / descomentar (6502), pôr / tirar REM (BASIC) |
| <kbd>F5</kbd> / <kbd>F9</kbd> / <kbd>F12</kbd> | editor: executar, ponto de paragem na linha, ir para a definição (6502) |
| <kbd>Ctrl</kbd>+letra | carácter de controlo (`Ctrl+D` = `$84`) |
| clique direito num ecrã | Copiar, Colar (escrever o texto), Limpar ecrã, Reset |
| <kbd>Esc</kbd> | ESC (`$9B`): anula a linha no monitor e no BASIC |
| retrocesso | escreve `_`, o apagamento do monitor e do BASIC |

No **terminal** (`apple1-emu` sem janela, capítulo 12):

| Tecla | Efeito |
|---|---|
| <kbd>F2</kbd> | linha de comandos do emulador |
| <kbd>F5</kbd> / <kbd>F6</kbd> | cassete: reproduzir, parar |
| <kbd>F10</kbd> ou <kbd>Ctrl</kbd>+<kbd>Q</kbd> | sair |

# Armadilhas e comportamentos a conhecer {: .appendix }

Pequenas coisas que surpreendem da primeira vez. Nenhuma é um erro; cada uma tem a sua razão.

- **BASIC e RUN.** **▶ Executar** do editor arranca o BASIC, carrega a listagem e escreve `RUN` por si. Um programa que pede dados espera pelas suas teclas no ecrã do Apple-1: clique primeiro no ecrã.
- **RND.** O primeiro `RND` de uma execução seria sempre o mesmo numa máquina acabada de reiniciar; **▶ Executar** espera um momento aleatório antes do `RUN`, para que um jogo não comece sempre da mesma maneira.
- **Comandos da cassete.** A linha «Cassete, no Apple-1 real:» por baixo do editor é o comando do Woz Monitor a escrever numa máquina verdadeira (`C100R`, depois os comandos `R` dos intervalos) antes de reproduzir a cassete. É uma indicação, não algo a escrever no emulador.
- **O teclado vai para um só sítio de cada vez.** A linha no fundo da janela diz para onde: para o Apple-1, para o editor, para um campo. Um clique no ecrã do Apple-1 envia as teclas para a máquina; um clique no editor recupera-as. Depois de clicar num erro da lista do editor, as teclas voltam ao editor.
- **Depurador em pausa.** As teclas carregadas enquanto a máquina está em pausa não se perdem, mas só a **última** é guardada (o registo do teclado verdadeiro guarda um carácter); é entregue quando continuar.
- **Os pontos de paragem vão com o programa.** Carregar outro programa (ficheiro, biblioteca, assembler) apaga os pontos de paragem e as etiquetas do anterior. Os pontos de paragem que pôs ficam de uma sessão para a outra apenas enquanto não carregar outra coisa.
- **Colar com o botão do meio.** No editor, um clique do meio leva o cursor para lá e cola a **seleção primária** do X11/Wayland (o texto que acabou de realçar numa aplicação qualquer). O editor ainda não *escreve* a sua própria seleção na seleção primária: para copiar a partir do editor, use <kbd>Ctrl</kbd>+<kbd>C</kbd>.
- **Janelas pequenas.** Num monitor com menos de 1400 pontos de largura, a disposição do primeiro arranque é editor e ecrã; no menu **⊞ Disposição ▾** pode escolher outra. Num ecrã de 1024 × 768, a barra de ferramentas desliza para o lado: o botão **»** chega ao grupo Cassete, em vez de o cortar à direita. Um zoom fixo (<kbd>Ctrl</kbd>+<kbd>+</kbd>) maior do que o painel do ecrã é mantido, e o ecrã desliza dentro do seu painel; a barra de estado indica-o.
- **Guardar um separador sem nome.** **Guardar como…** (ícone, <kbd>Ctrl</kbd>+<kbd>S</kbd> ou o menu de contexto do separador) nunca deixa dois separadores para o mesmo ficheiro: um separador já aberto nesse ficheiro é fechado.
- **Primeiro arranque e a rede.** O primeiro arranque contacta o GitHub para ir buscar o projeto de demonstração (capítulo 4); retire o projeto da lista «Projetos GitHub» em **⚙ Preferências** para que isso deixe de acontecer.
- **Ler o teclado no G2 Basic.** Uma tecla carregada enquanto um programa corre para-o, como no original, e o `INPUT` espera. Um jogo que precise de vigiar as teclas chama `KEY(0)` no seu ciclo principal (capítulo 15); a partir daí, uma tecla deixa de parar o programa.
- **Etiquetas e pontos de paragem.** Um ponto de paragem numa etiqueta que ainda não existe (escrito antes de montar ou executar) é guardado e colocado quando a etiqueta aparece.
- **Disposições e janelas.** Uma disposição guardada lembra-se dos painéis acoplados, das janelas destacadas (posição e tamanho) e do tamanho da janela principal. Não se lembra de qual janela tem o teclado.

# Licença e créditos {: .appendix }

O **Retro Dev Kit Apple 1** é software livre, publicado sob a **GNU General Public License, versão 3** (GPL-3.0), sem qualquer garantia. O texto completo está no ficheiro `LICENSE` do arquivo e do repositório.

Algumas partes vêm de outros projetos e mantêm a sua própria licença, a licença **MIT**, compatível com a GPL; estão listadas no ficheiro `NOTICE`:

- o **RDK G2 Basic**, o BASIC integrado, escrito para este projeto (repositório `Old-Wirehead/rdk-gen2basic-apple1`, MIT);
- os **testes GEN2 de Uncle Bernie** (`gen2-test1` a `gen2-test4`, as barras) e os **glifos GEN2**, fornecidos por Uncle Bernie.

Os jogos da biblioteca escritos por **Claude IA** (Forca, Fósforos, Batalha naval, Túnel e Jogo do galo) são distribuídos com o projeto.

O que o kit **não** contém: o Integer BASIC da Apple, as cassetes comerciais e as demos da placa P-LAB. Continuam a pertencer aos seus donos; traga as suas próprias cópias.

A placa P-LAB é uma criação da P-LAB; o TMS9918A é um circuito da Texas Instruments. Este projeto não está associado à Apple, à Texas Instruments nem à P-LAB.
