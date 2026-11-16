# O teclado, o ecrã e o Woz Monitor

<p class="lead">O Apple-1 só tem quatro registos de entrada/saída, num PIA 6820 (ou 6821), e um monitor de 256 bytes. Não é muito, mas chega: este capítulo mostra como ler uma tecla, escrever um carácter e usar as rotinas de Steve Wozniak.</p>

## Os quatro registos do PIA

| Endereço | Nome | Leitura | Escrita |
|---|---|---|---|
| `$D010` | `KBD` | código da tecla, bit 7 a um; a leitura **confirma** a tecla | — |
| `$D011` | `KBDCR` | bit 7 = 1: há uma tecla à espera | configuração (feita pelo monitor) |
| `$D012` | `DSP` | bit 7 = 1: o ecrã está ocupado | carácter a mostrar (7 bits) |
| `$D013` | `DSPCR` | — | configuração (feita pelo monitor) |

{{!KBD}}{{!KBDCR}}{{!DSP}}

Estes nomes estão definidos em `apple1.inc`, incluído por todos os exemplos:

@@include exemples/pt/asm/apple1.inc

## Ler uma tecla

Esperar que o bit 7 de `KBDCR` passe a 1 e depois ler `KBD`:

```asm
ler:    lda KBDCR       ; uma tecla?
        bpl ler         ; bit 7 = 0: ainda não
        lda KBD         ; código ASCII + $80
```

O teclado só produz maiúsculas (o emulador converte as minúsculas), com o bit 7 a um: `A` dá `$C1`, RETURN `$8D`, ESC `$9B`. Para saber se uma tecla foi carregada **sem esperar**, teste só `KBDCR`:

```asm
        lda KBDCR
        bpl nada        ; nenhuma tecla: continuar a trabalhar
        lda KBD         ; uma tecla: lê-la (e confirmá-la)
```

`jeu.s` (capítulo 23) usa isto para tirar um número ao acaso: conta num ciclo até o jogador carregar numa tecla.

## Escrever um carácter

O terminal do Apple-1 aceita um carácter de cada vez, e só quando está pronto: o bit 7 de `DSP` fica a 1 até o carácter anterior ter sido mostrado. A rotina `ECHO` do monitor espera por si. Eis as suas quatro instruções, lidas no monitor do emulador:

```mon
\
» FFEF.FFFFR

FFEF: 2C
FFF0: 12 D0 30 FB 8D 12 D0 60
FFF8: 00 00 00 0F 00 FF 00 00
```

Ou seja, `BIT DSP` / `BMI ECHO` / `STA DSP` / `RTS`. `ECHO` preserva A, X e Y. Como o terminal original só mostra **um carácter por imagem**, cerca de 60 por segundo, um programa que mostra muito é limitado pelo ecrã, não pelo processador; o emulador respeita este limite por omissão (<kbd>F4</kbd> retira-o, capítulo 2).

O que o ecrã faz com cada código:

- **RETURN** (`$8D`, ou `$0D`) começa uma linha nova; no fundo do ecrã, tudo sobe uma linha.
- Os **outros caracteres de controlo** (`$00-$1F`, com ou sem o bit 7) são ignorados: nem apagar, nem recuar, nem campainha.
- As **minúsculas** `$60-$7F` são mostradas como as maiúsculas `$40-$5F`: o terminal só guarda 6 bits.
- Depois da 40.ª coluna, o cursor passa sozinho para a linha seguinte.
- Não há código para limpar o ecrã: é um botão da máquina (**Limpar ecrã**, <kbd>Ctrl</kbd>+<kbd>L</kbd>).

## As rotinas do Woz Monitor

| Endereço | Nome | Entrada | Efeito | Registos |
|---|---|---|---|---|
| `$FFEF` | `ECHO` | A = carácter | mostra-o | A, X, Y preservados |
| `$FFDC` | `PRBYTE` | A = byte | mostra-o em 2 algarismos hexadecimais | A destruído; X, Y preservados |
| `$FFE5` | `PRHEX` | A (nibble baixo) | mostra um algarismo hexadecimal | A destruído; X, Y preservados |
| `$FF1F` | `GETLINE` | — | regresso ao monitor: linha nova, depois espera por um comando | — |
| `$FF1A` | `ESCAPE` | — | como `GETLINE`, mostrando antes `\` | — |
| `$FF00` | `RESET` | — | inicializa o PIA e o monitor | — |

{{!ECHO}}{{!PRBYTE}}{{!PRHEX}}{{!GETLINE}}

O monitor usa a página zero `$24-$2B` e o buffer de linha `$0200-$027F`; tudo o resto é seu (salvo se o Integer BASIC tiver de sobreviver, capítulo 16). Os seus comandos (`300R`, `0300.031F`, `300: A9 8D`…) estão resumidos no apêndice B.

## Arrancar e terminar um programa

Um programa é arrancado pelo comando `R` do monitor (`300R`), que faz um `JMP` para o endereço, não um `JSR`. Não há, portanto, **nenhum endereço de retorno** na pilha: termine com `jmp GETLINE` (`$FF1F`), não com `RTS`.

!!! attention "RTS e BRK não devolvem o controlo"
    Um `RTS` no fim de um programa arrancado com `R` tira o que estiver na pilha. Um `BRK` salta pelo vetor de interrupção da ROM, que é `$0000` (as três últimas palavras do monitor, acima: NMI `$0F00`, RESET `$FF00`, IRQ/BRK `$0000`): o processador executa então a página zero. Em ambos os casos, carregue em RESET (<kbd>Ctrl</kbd>+<kbd>R</kbd>).

Já um programa chamado a partir do BASIC com `CALL 768` é chamado por um `JSR`: tem de terminar com `RTS` para voltar ao BASIC. O capítulo 16 mostra como arrumar código máquina abaixo do programa BASIC.

```mon
>» POKE 768,96
>» CALL 768
>
```

(`96` = `$60`, o opcode de `RTS`: a sub-rotina mais pequena possível.)

## O PIA, a cassete e a placa de cores

A placa ACI ocupa `$C000-$C0FF` (cada acesso inverte a saída) e o seu programa `$C100-$C1FF`; o capítulo 5 explica o seu funcionamento e o apêndice C os seus tempos. A placa GEN2 é comandada simplesmente **lendo** `$C250-$C257` (`bit SS_GR`); os seus endereços estão em `gen2.inc`:

@@include exemples/pt/asm/gen2.inc
