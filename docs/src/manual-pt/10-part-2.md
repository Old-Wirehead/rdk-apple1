# Integer BASIC <span class="part-sub">O BASIC de Steve Wozniak e o seu clone livre RDK G2 Basic: todas as instruções, a memória, as mensagens de erro, os erros conhecidos e como contorná-los, e a adaptação a partir do Applesoft</span> {: .part data-part="Parte II" }

# Primeiros passos em BASIC

<p class="lead">O Integer BASIC do Apple-1 cabe em 4 KB. Só conhece inteiros e nomes de variável de uma letra, e não tem nem DATA nem funções de cadeias: é pequeno, rápido e tem as suas armadilhas.</p>

!!! apple1 "De onde vem este BASIC"
    Steve Wozniak escreveu-o em 1976, à mão, em assembly, sem um computador para o montar. Era vendido em cassete (75 $) e carrega em `$E000-$EFFF`. O emulador não fornece o original da Apple: use o seu próprio `BASIC.wav` ou imagem de 4 KB (capítulo 4) se o quiser. Todos os testes desta parte foram feitos no emulador, com esse original.

## Dois BASIC: o RDK G2 Basic e o original

Desde a versão 7.9 o emulador tem o seu próprio BASIC, o **RDK G2 Basic**, integrado no programa. É um clone em sala limpa do Integer BASIC, escrito de raiz em assembly 6502 (não contém código do interpretador da Apple) e distribuído sob a licença MIT. É dedicado ao Uncle Bernie.

- **Compatível**: mesma sintaxe, mesmos tokens, mesmo formato de programa na memória, mesmo formato de cassete. Os programas, listagens e cassetes escritos para o original correm sem alterações. Foi verificado contra o original com 22 sessões de teste de saída idêntica.
- **Corrigido**: os erros conhecidos do original, descritos no capítulo 18, estão corrigidos.
- **Maior**: 7933 bytes em vez de 4 KB. Precisa de RAM de `$E000` a `$FEFF`; o emulador fornece-a, mas não caberia na janela de EPROM de 4 KB do original.
- **Conhece a GEN2**: ao arrancar procura a placa GEN2 e, se a encontrar, inicia o controlador da placa (e põe então o LOMEM em `$2000`, acima das páginas gráficas). As instruções gráficas propriamente ditas estão previstas para a versão 2 do G2 Basic.
- **Saudação**: `RDK G2 BASIC 1.0`, depois `DEDICATED TO UNCLE BERNIE`.
- **Alargado** (versão 7.9.2): `LOMEM=` e `HIMEM=` funcionam como no original, e uma nova função **`KEY(n)`** lê o teclado sem esperar, para que um jogo possa ver as teclas sem que o `INPUT` o bloqueie e sem que uma tecla pare o programa (capítulo 15). `LIST a,b` e `DEL a,b` usam os intervalos do original, e uma linha listada mais comprida do que o ecrã muda de linha como no original.

O botão **BASIC** carrega o G2 Basic. Para usar em vez dele o original da Apple, abra **⚙ Preferências → Avançado → BASIC**, escolha **Integer BASIC original (o seu ficheiro, carregado em $E000)** e o ficheiro; a escolha fica memorizada e o botão passa a carregá-lo. Os dois comportam-se da mesma forma em tudo o que esta parte descreve; as diferenças são indicadas onde importam (HIMEM ao arrancar, os erros do capítulo 18).

!!! note "Fonte e testes"
    O G2 Basic tem o seu próprio repositório, `Old-Wirehead/rdk-gen2basic-apple1`, com o seu manual, a sua fonte e os seus testes de regressão.

## Arrancar o BASIC

Clique em **BASIC** na barra de ferramentas: o emulador carrega o BASIC escolhido (G2 Basic por omissão) e arranca-o com `E000R`. O prompt do BASIC é `>`, o do Woz Monitor é `\`.

```mon
\
» E000R
E000: 4C
>
```

- `E000R`: **arranque a frio**, apaga o programa na memória.
- `E2B3R`: **arranque a quente**, mantém o programa. É o endereço a usar depois de carregar um programa de cassete, e para voltar ao BASIC depois de uma passagem pelo monitor (RESET).

{{E000R}}{{E2B3R}}

## Modo direto e modo programa

Uma linha que começa por um **número** (0 a 32767) é guardada no programa; uma linha sem número é executada de imediato.

```mon
>» PRINT 2*3+4*5
26
>» 10 PRINT "OLA"
>» 20 GOTO 10
>» RUN
OLA
OLA
…
```

Carregue numa tecla para parar o programa: o BASIC mostra `STOPPED AT 10`. Voltar a escrever uma linha com o mesmo número substitui-a; um número sozinho apaga-a.

!!! attention "O modo direto é limitado"
    Em modo direto, o BASIC recusa instruções separadas por `:` (`SYNTAX ERR`), assim como `FOR` e `GOSUB`. Para experimentar um ciclo, escreva-o numa linha numerada.

## Corrigir uma linha

O teclado do Apple-1 não tem tecla de apagar: o **Retrocesso** do emulador escreve `_`, que anula o carácter anterior (a linha no ecrã não é corrigida, mas o BASIC recebe a correta). <kbd>Esc</kbd> anula a linha inteira: o BASIC mostra `\` e espera pela linha seguinte. Para editar com mais conforto, use o editor (capítulo 7): colore a sintaxe, assinala os erros antes de enviar e escreve o programa por si.

## Um primeiro programa

@@include exemples/pt/basic/boucle.bas

```mon
>» RUN
SOMA = 55
>
```

No editor, **▶ Executar** escreve o programa e `RUN`; no depurador pode segui-lo linha a linha (figura no capítulo 8).

## Comandos

| Comando | Efeito |
|---|---|
| {{LIST}} `[n[,m]]` | lista o programa, a linha `n`, ou as linhas `n` a `m` |
| {{RUN}} `[n]` | apaga as variáveis e executa o programa, a partir da linha `n` se indicada |
| {{DEL}} `n[,m]` | apaga a linha `n`, ou as linhas `n` a `m` |
| {{SCR}} | apaga o programa e as variáveis |
| {{CLR}} | apaga as variáveis, mantém o programa |
| {{AUTO}} `n[,passo]` | numeração automática a partir de `n` (passo 10 por omissão); <kbd>Ctrl</kbd>+<kbd>D</kbd> e depois <kbd>RETURN</kbd> termina-a, <kbd>Esc</kbd> não |
| {{MAN}} | volta à numeração manual |
| {{HIMEM}}`=n`, {{LOMEM}}`=n` | os limites da memória do BASIC (capítulo 16) |

`GOTO n` em modo direto retoma um programa parado **sem** apagar as variáveis, ao contrário de `RUN n`.

!!! astuce "LIST e a visualização lenta"
    O terminal do Apple-1 mostra cerca de 60 caracteres por segundo: listar um programa longo leva tempo. Desligue o limite de visualização (**Rápido** na linha de visualização, ou na barra de estado) para ler mais depressa, ou liste às fatias: `LIST 100,200`.

## Três programas para escrever

### Adivinhar um número

@@include exemples/pt/basic/devine.bas

```mon
>» RUN
ESCOLHI UM NUMERO DE 1 A 100.
O TEU PALPITE?» 50
MAIS ALTO!
O TEU PALPITE?» 75
MAIS BAIXO!
O TEU PALPITE?» 62
MAIS BAIXO!
O TEU PALPITE?» 56
MAIS ALTO!
O TEU PALPITE?» 57
BOA, EM 5 TENTATIVAS.
```

`RND(100)` dá um inteiro de 0 a 99; `#` quer dizer «diferente de».

### Uma tabuada

@@include exemples/pt/basic/table.bas

### Brincar com as cadeias

@@include exemples/pt/basic/chaines.bas

```mon
>» RUN
O TEU NOME?» RADAR
INICIAL: R
FIM: AR
AO CONTRARIO: RADAR
E UM PALINDROMO!
```

As cadeias cortam-se com `N$(início,fim)` e alongam-se com `R$(LEN(R$)+1)=…`: não há nem `MID$` nem `+` para as cadeias (capítulo 16).

Os quatro programas estão em `docs/exemples/pt/basic/`: abra-os no editor, ou faça uma cassete com `apple1-emu basic devine.bas -o devine.aiff`.
