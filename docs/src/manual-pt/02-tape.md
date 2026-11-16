# A cassete e a placa ACI

<p class="lead">O Apple-1 não tinha disquete nem BASIC em ROM: tudo passava por uma cassete áudio e pela placa ACI (Apple Cassette Interface). O emulador executa a ROM verdadeira da ACI, pelo que os seus ficheiros WAV e AIFF carregam num Apple-1 verdadeiro, e as gravações feitas num Apple-1 verdadeiro carregam no emulador.</p>

## Formatos de ficheiro

| Extensão | Conteúdo | Leitura | Escrita |
|---|---|---|---|
| `.wav` | cassete ACI, PCM de 8 a 32 bits ou vírgula flutuante, mono ou estéreo, qualquer frequência | sim | 44,1 kHz mono de 16 bits por omissão |
| `.aif`, `.aiff` | cassete ACI (AIFF e AIFF-C) | sim | perfil smartphone: 48 kHz, 2 canais idênticos, 16 bits |
| `.txt`, `.hex`, `.apl` | dump do Woz Monitor: linhas `0300: A9 C1 20 …` | sim | sim |
| `.bin` | bytes em bruto, sem endereço (`FICHEIRO.bin@300`) | sim | sim |
| `.bas` | listagem Integer BASIC (texto) | no editor | no editor |
| `.s`, `.asm`, `.a65` | fonte 6502 (ca65 ou xa) | no editor | no editor |

As cassetes exportadas pelo emulador levam os seus endereços num bloco de comentário do ficheiro (`APPLE1 ACI 0300.031E`), que os leitores de áudio ignoram. Uma cassete de outra origem não tem endereços: a biblioteca reconhece-os ou adivinha-os (capítulo 4), e a linha de comandos aceita `--addr` ou `--range`.

O descodificador tolera ruído, níveis baixos, polaridade invertida, um erro de velocidade da fita de ±10 % e várias gravações num só ficheiro. Também lê os WAV produzidos pela ferramenta **c2t** de Egan Ford. Um **zumbido da rede elétrica** (50 ou 60 Hz, mesmo tão forte como o próprio sinal da cassete) é filtrado antes de detetar as transições, pelo que uma gravação longa feita com um cabo mal blindado continua a ler-se bem.

## O gravador virtual

A faixa **Cassete:** da barra de ferramentas reúne todos os controlos: **📼 Inserir**, **▶** reproduzir, **⏹** parar, **⏪** rebobinar, **⏺** gravar, **⏏** ejetar, e depois o nome e a posição da cassete, o osciloscópio **📈** e o som **🔊**. Uma cassete insere-se com **📼 Inserir** (um `.wav` ou `.aiff`), ou com um clique no botão direito num programa da biblioteca («Inserir como cassete»): depois escreve você mesmo `C100R` e o intervalo. O ícone **📼** de um programa da biblioteca vai um passo mais longe: insere a cassete e escreve os comandos por si.

![A faixa Cassete: durante a leitura de BASIC.wav: 7,6 segundos de 35,8.](img/pt/barre-cassette.png "large")

A cassete segue o tempo **emulado**: à velocidade rápida (<kbd>F3</kbd>), o carregamento demora muito menos do que o cabeçalho de 10 segundos, e funciona na mesma. Sem cassete, os carregamentos da biblioteca são instantâneos (capítulo 4).

Carregar o Integer BASIC «à antiga» faz-se assim:

```mon
» C100R
C100: A9*
» E000.EFFFR
» E000R

E000: 4C
>
```

`C100R` arranca o programa da ACI, que responde com o seu prompt `*`. A linha `E000.EFFFR` pede-lhe para **ler** a cassete para `$E000-$EFFF`. No fim da cassete, a ACI volta ao monitor (`\`); `E000R` arranca o BASIC, cujo prompt é `>`.

## Som e osciloscópio

![O osciloscópio enquanto BASIC.wav toca: a caixa «Som da cassete», o volume e o sinal a 1 ms por divisão.](img/pt/oscilloscope.png "small")

Durante a reprodução ou a gravação, **🔊** deixa-o ouvir o sinal, como o altifalante de um gravador, a volume baixo por omissão. **📈** abre o **osciloscópio**: um traço verde sobre uma grelha escura, 1 ms por divisão, que mostra 10 ms à volta da cabeça de leitura ou os últimos 10 milissegundos escritos pelo Apple-1. Distinguem-se o cabeçalho (≈ 800 Hz), os bits 0 (≈ 2 kHz) e os bits 1 (≈ 1 kHz). À velocidade rápida o som fica mudo.

## Gravar a partir do Apple-1 emulado

Para produzir uma cassete como em 1976, arme o gravador e depois peça à ACI que escreva:

<ol class="steps">
<li>Clique em <strong>⏺</strong> e escolha o ficheiro (<code>.wav</code> ou <code>.aiff</code>).</li>
<li>Escreva <code>C100R</code>, depois o intervalo seguido de <code>W</code>, por exemplo <code>0300.03FFW</code>, e <kbd>RETURN</kbd>.</li>
<li>A ACI escreve um cabeçalho de 10 segundos e depois os dados. O ficheiro é escrito 1,5 s depois do fim, ou ao carregar em <strong>⏹</strong>.</li>
</ol>

O mais simples continua a ser **Guardar** na barra de ferramentas, ou **📼 Exportar WAV** no editor: o sinal é o mesmo, ciclo a ciclo, e é produzido de imediato.

## Como funciona a ACI

![Os dois sentidos da ACI: na escrita, qualquer acesso a $C0xx inverte a saída; na leitura, o comparador comanda a linha de endereço A0 da PROM.](diagrams/pt/aci.svg "diagram large")

A placa é notavelmente simples: uma PROM de 256 bytes visível em `$C000` e `$C100`, um flip-flop e um comparador.

- **Escrita.** Cada acesso a um endereço de `$C000-$C0FF` inverte a saída (74LS74). O programa da ROM define o tempo entre dois acessos: esse tempo codifica os bits.
- **Leitura.** O nível da entrada da fita, depois do comparador LM311, comanda a linha de endereço A0 da PROM quando A7 = 1. A mesma instrução lê portanto `$C080` ou `$C081` consoante o nível: a ROM mede os semiperíodos com um ciclo de 12 tempos de relógio.
- **Codificação.** Um «1» dura cerca do dobro de um «0»: 1 kHz para um 1, 2 kHz para um 0. Os bytes saem com o bit mais significativo primeiro, sem paridade nem soma de verificação.

O Anexo C dá os tempos exatos, em ciclos e em microssegundos.

## Os comandos da ACI

Depois de `C100R`, a ACI espera uma linha no formato do monitor, terminada em `R` (leitura) ou `W` (escrita):

| Linha | Efeito |
|---|---|
| `0300.03FFR` | lê a cassete para `$0300-$03FF` |
| `0300.03FFW` | escreve `$0300-$03FF` na cassete, com 10 s de cabeçalho |
| `004A.00FFR 0300.0FFFR` | lê **duas** gravações seguidas: a página zero do BASIC e depois o programa |
| `004A.00FFW800.FFFW` | a forma colada do manual BASIC de 1976, também aceite |

{{!C100R}}{{!R (leitura)}}{{!W (escrita)}}

!!! attention "Os zeros de 004A.00FF são obrigatórios"
    O analisador da ROM não limpa o seu acumulador hexadecimal no ponto («Doesn’t clear HEX1!!!», diz a listagem). Os algarismos do fim juntam-se atrás dos do início: `4A.FF` significa `$004A-$4AFF`, quase 19 KB! `0300.0FFF` funciona porque os quatro algarismos empurram para fora o 3 de `0300`. Escreva sempre os dois endereços com quatro algarismos. O emulador reproduz este comportamento, já que executa a ROM verdadeira.

Um programa BASIC é guardado em duas gravações: a página zero `$004A-$00FF` (que contém os ponteiros LOMEM e HIMEM e o fim do programa) e depois o próprio programa, que termina sempre em HIMEM. Para o voltar a carregar, o BASIC já tem de estar na memória; regressa-se então ao BASIC «a quente» com `E2B3R`, que conserva o programa.

```mon
» C100R
C100: A9*
» 004A.00FFR 0300.0FFFR
» E2B3R

E2B3: 20
» >RUN
```

## Para um Apple-1 verdadeiro

É isto que justifica o emulador: preparar um programa no PC e depois reproduzi-lo na ACI de uma máquina verdadeira, a partir de um smartphone ou da saída de auscultadores de um computador.

<ol class="steps">
<li><strong>Prepare o ficheiro.</strong> <strong>📼 Exportar WAV</strong> no editor, ou <code>apple1-emu export prog.txt -o prog.aiff</code>. Escolha <code>.aiff</code> para um smartphone. Tome nota da linha mostrada, por exemplo <code>C100R</code> e depois <code>0300.031ER</code>.</li>
<li><strong>Prepare o leitor.</strong> Volume no máximo (aceite o aviso de «volume elevado»), todos os efeitos de áudio desativados (equalizador, Dolby, «melhoria de áudio»), modo de avião, sem Bluetooth. Um leitor simples como o VLC serve.</li>
<li><strong>No Apple-1</strong>, escreva <code>C100R</code>: o monitor responde <code>C100: A9*</code>.</li>
<li>Escreva a linha de leitura, por exemplo <code>0300.031ER</code>, mas <strong>não carregue ainda em RETURN</strong>.</li>
<li>Inicie a reprodução e carregue em <kbd>RETURN</kbd> <strong>durante o cabeçalho</strong> (dura cerca de 10 s, depois de um segundo de silêncio).</li>
<li>No fim, a ACI volta ao monitor (<code>\</code>). Execute o programa: <code>300R</code>, ou <code>E2B3R</code> e depois <code>RUN</code> para um programa BASIC.</li>
</ol>

**Porquê este perfil AIFF?** Sem compressão, 16 bits, nenhum codec com perdas: MP3 ou AAC arredondariam as transições. 48 kHz, a frequência nativa do Android, para evitar qualquer reamostragem. Dois canais estritamente idênticos, porque um cabo que une esquerdo e direito em FROM TAPE nunca deve receber dois sinais diferentes. Uma onda quadrada a −1 dBFS: a entrada da ACI original é um filtro passa-alto (0,01 µF) que pouco mais vê do que as transições, e uma onda quadrada com transições abruptas é o que melhor lhe convém. Um segundo de silêncio no início e no fim, porque um telefone às vezes corta o começo de um som.

!!! apple1 "Não confie no LED"
    O manual de 1976 pede um nível em que o LED da ACI esteja «mesmo aceso». No entanto, ele só acende por volta de 1,2 V, muito mais do que o comparador precisa. Um LED apagado não prova portanto que o sinal seja demasiado fraco. Se o carregamento nunca terminar, a ROM continua à espera de transições: carregue em RESET e suba o volume.

!!! attention "Nunca MP3 nem AAC"
    Um formato com perdas desloca as transições e torna a cassete ilegível. Use WAV, AIFF, ou FLAC se o seu leitor o descodificar.

## Que nível é preciso? Os modelos de entrada

O comando `info --aci-input` simula a entrada analógica da placa (condensador de acoplamento, polarização, histerese do LM311) para uma série de tensões de pico, e diz-lhe a partir de que nível o ficheiro seria lido:

```sh
apple1-emu info prog.aiff --aci-input stock     # placa de 1976, 0,01 µF
apple1-emu info prog.aiff --aci-input mimeo     # 0,1 µF (conselho do guia Mimeo)
apple1-emu info prog.aiff --aci-input gen2      # ACI Gen2 de Uncle Bernie
apple1-emu info prog.aiff --aci-input stock --volts 0.5 --series 1200
```

| Modelo | O que representa |
|---|---|
| `stock` | a ACI original: 0,01 µF para o comparador polarizado a 2,5 V, realimentação 3k/47k |
| `mimeo` | a modificação de Mike Willegal: 0,1 µF, mais fiável |
| `gen2` | a ACI Gen2 de Uncle Bernie: 100 nF emparelhados, 300k em vez de 47k (menos histerese) |

`--volts` define a tensão de pico na ficha para a escala digital completa; `--series` acrescenta uma resistência em série (por exemplo os 1,2 kΩ de um cabo para smartphone). Com o modelo `stock`, os ficheiros do emulador são lidos a partir de **cerca de 0,25 V de pico**; a saída de auscultadores de um smartphone no máximo dá normalmente de 0,5 a 1 V. Este modelo é simplificado: sem ruído nem deriva do ponto de polarização.

## Digitalizar uma cassete verdadeira ou uma gravação do Apple-1

Para recuperar um programa escrito num Apple-1 verdadeiro, grave a saída **TO TAPE** da ACI através de uma entrada de microfone (um telefone, ou a placa de som de um PC com o Audacity a 44,1 ou 48 kHz), com o controlo automático de ganho e a redução de ruído desativados. No Apple-1: `C100R`, depois `0300.03FFW` e <kbd>RETURN</kbd>. Pare alguns segundos depois de o `\` voltar, e depois:

```sh
apple1-emu import gravacao.aiff --addr 300 -o prog.txt
apple1-emu import startrek.aiff --range "004A.00FFR 0300.0FFFR" -o startrek.txt
```

!!! apple1 "TO TAPE é uma saída de nível de microfone"
    Fornece apenas cerca de 50 mV pico a pico. Verifique a forma de onda no Audacity: sem saturação nem saltos de nível durante o cabeçalho. A importação do emulador tolera qualquer ganho e um desvio de corrente contínua.
