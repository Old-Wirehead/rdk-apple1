# Num Apple-1 verdadeiro: exportar para WAV

Tudo o que escreve no emulador pode ir para um Apple-1 verdadeiro, uma réplica ou um kit equipado com a placa de cassete ACI. O ficheiro de áudio substitui a fita; um smartphone ou a saída de auscultadores de um PC serve de gravador.

## Criar a cassete

No editor, clique em **📼 Exportar WAV** e escolha um nome de ficheiro. A extensão decide o formato: `.wav`, ou `.aiff` para um telefone (o formato recomendado, ver abaixo). A parte de baixo do editor mostra então exatamente o que escrever na máquina verdadeira:

![Depois da exportação, a parte de baixo do editor mostra o comando de carregamento.](img/pt/editeur-export-bas.png "large")

Para `premier.bas`, o comando é `C100R` e depois `004A.00FFR 0F54.0FFFR`. O programa BASIC ocupa duas gravações seguidas na cassete: primeiro a página zero do BASIC (`$004A` a `$00FF`, que contém os ponteiros do programa) e depois o próprio programa, guardado no topo da memória. O botão **Copiar os intervalos** põe o comando na área de transferência.

!!! attention "Sempre quatro algarismos"
    Escreva `004A.00FF`, nunca `4A.FF`. A ROM da ACI não apaga os algarismos já lidos quando encontra o ponto: `4A.FF` seria entendido como `$004A` a `$4AFF`, e a máquina ficaria à espera de 19 KB que nunca chegam. O emulador mostra sempre a forma correta, e reproduz este comportamento porque executa a ROM verdadeira da ACI.

## Carregar na máquina verdadeira

O BASIC já tem de estar na memória, carregado da sua própria cassete (`C100R` e depois `E000.EFFFR`).

<ol class="steps">
<li><strong>Prepare o leitor.</strong> Copie o ficheiro para o telefone, ligue a saída de auscultadores à entrada «FROM TAPE» da ACI e ponha o volume no máximo.</li>
<li><strong>Volte ao monitor</strong> com RESET e escreva <code>C100R</code>. A ACI responde <code>C100: A9*</code> e começa uma linha nova; o asterisco é o seu prompt.</li>
<li><strong>Escreva os intervalos</strong> <code>004A.00FFR 0F54.0FFFR</code>, mas não carregue ainda em RETURN.</li>
<li><strong>Inicie a reprodução</strong> e carregue em <strong>RETURN durante o cabeçalho</strong>, o tom constante de cerca de 10 segundos com que começa cada gravação.</li>
<li><strong>Espere que o <code>\</code> volte</strong> e escreva <code>E2B3R</code> (arranque a quente do BASIC, que conserva o programa) e <code>RUN</code>.</li>
</ol>

```mon
» C100R

C100: A9*
» 004A.00FFR 0F54.0FFFR
\
» E2B3R

E2B3: 20
» >RUN
OLA, APPLE-1!
```

!!! apple1 "Ler de forma fiável a partir de um telefone"
    - **Volume no máximo.** Aceite o aviso do Android sobre o «volume elevado»; caso contrário, o volume fica limitado. Muitos telefones baixam o volume sempre que a ficha é ligada de novo: verifique-o de cada vez.
    - **Sem efeitos sonoros**: desative o equalizador, Dolby, Mi Sound e qualquer «melhoria de áudio».
    - **Modo de avião** ou «Não incomodar», para que nenhuma notificação se misture com o sinal. Sem Bluetooth.
    - **Um leitor simples** que reproduza AIFF sem o processar; o VLC funciona bem.
    - **Nunca MP3 nem AAC**: a compressão deforma as transições do sinal. Fique pelo WAV ou AIFF.
    - **Não confie no LED da ACI**: só acende por volta de 1,2 V, muito acima do que a placa precisa.

## Porquê AIFF a 48 kHz

Os ficheiros do emulador são pensados para a entrada da ACI. O AIFF de 16 bits sem compressão escapa a todos os codecs; 48 kHz é a frequência nativa do Android, por isso o telefone não reamostra. Os dois canais estéreo são estritamente idênticos, de modo que esquerdo e direito podem ser unidos em FROM TAPE sem que as duas saídas se oponham. O sinal é uma onda quadrada com transições abruptas, o que o comparador da ACI deteta melhor.

!!! astuce "Verificar um ficheiro antes de o reproduzir"
    Na linha de comandos, `apple1-emu info premier.aiff --aci-input stock` simula a entrada da placa original com vários níveis de tensão e diz-lhe a partir de que nível o ficheiro é lido corretamente (cerca de 0,25 V de pico para os nossos ficheiros).
