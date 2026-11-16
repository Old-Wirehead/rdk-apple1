# Carregar e executar uma cassete

No Apple-1 tudo passa pela cassete: o próprio Integer BASIC era carregado de uma fita de áudio. O emulador lê os ficheiros `.wav` e `.aiff` como faz a placa ACI original, byte a byte, e reconhece as 24 cassetes da coleção «Apple1Cassettes» do archive.org.

## Preparar a biblioteca

Copie as suas cassetes para a pasta da biblioteca, `~/apple1/programmes`: por exemplo `Hamurabi.wav`. A lista à direita atualiza-se sozinha. As cinco pastas BASIC, 6502, GEN2 COLOR, WAV e Art ASCII aparecem lá diretamente, sem pasta raiz por cima; os ficheiros soltos na pasta da biblioteca não são mostrados. Cada cassete reconhecida leva a marca **✔ reconhecido**, com os seus intervalos de endereços e o seu comando de execução já definidos.

!!! note "Não precisa de BASIC.wav"
    O emulador tem o seu próprio BASIC, RDK G2 Basic, por isso os programas BASIC funcionam sem nenhuma cassete. O Integer BASIC da Apple de 1976 continua protegido por direitos de autor e não está incluído: se preferir o original, use a sua própria cópia (como `BASIC.wav`) e escolha-a em **⚙ Preferências → Avançado → BASIC**.

## O caminho mais rápido: o botão ▶

Cada programa da lista tem quatro pequenos ícones à direita: **▶** carrega e executa, **⬇** carrega na memória sem executar, **📼** carrega pela cassete, **⚙** abre as suas definições. Um duplo clique faz o mesmo que ▶.

Clique em **▶** ao lado de `Hamurabi.wav`. O emulador carrega o BASIC se ainda não estiver na memória, depois o programa, e escreve por si `E2B3R` (regresso ao BASIC sem apagar o programa) e `RUN`. O carregamento é instantâneo.

![Hamurabi lançado com o botão ▶ da biblioteca.](img/pt/hamurabi-ecran.png "small")

Para um programa em linguagem máquina, ▶ faz um RESET e depois escreve o seu comando de execução, por exemplo `300R`. Uma listagem BASIC ou uma fonte 6502, ▶ abre-as também num separador do editor, se o editor estiver aberto.

## Como em 1976: a cassete virtual

O carregamento é instantâneo, a menos que peça a cassete. Clique no ícone **📼** de um programa para reproduzir um carregamento verdadeiro: o emulador escreve `C100R` para arrancar o programa da ACI, depois o intervalo de endereços, e a cassete é reproduzida à velocidade real. Aparece uma barra de progresso por baixo do nome do programa, e a faixa **Cassete:** da barra de ferramentas mostra a cassete e a sua posição.

![Carregamento de BASIC.wav pela ACI virtual, com o osciloscópio da cassete aberto.](img/pt/brut/cassette-basic.png "large")

Os botões 📈 e 🔊 no fim da faixa **Cassete:** abrem o **osciloscópio** e deixam-no **ouvir** o sinal, como o altifalante de um gravador. Carregar o BASIC demora cerca de 36 segundos à velocidade real.

Quando a cassete para, o monitor volta a mostrar `\`. Escreva `E000R` para arrancar o BASIC. O seu prompt é um `>`:

```mon
» E000R

E000: 4C
» >PRINT 6*7
42
```

!!! astuce "Carregar à mão"
    **📼 Inserir**, na faixa **Cassete:** da barra de ferramentas, põe qualquer ficheiro `.wav` ou `.aiff` no gravador virtual. Depois escreve você mesmo `C100R` e a seguir o intervalo, por exemplo `E000.EFFFR` para o BASIC: a cassete arranca na primeira leitura, como na máquina verdadeira. O botão **Carregar** serve para os ficheiros `.txt`, `.bin` e `.hex`, carregados de imediato.

| Programa | Depois do carregamento |
|---|---|
| Integer BASIC | `E000R` (arranque a frio, apaga o programa na memória) |
| Programa BASIC | `E2B3R` (arranque a quente, conserva o programa) e depois `RUN` |
| Linguagem máquina | o seu endereço seguido de `R`, muitas vezes `300R` |
