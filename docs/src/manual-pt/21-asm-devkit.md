# O assembler do devkit

<p class="lead">O emulador contém um assembler 6502 completo de duas passagens, compatível com o ca65. É usado no editor, que monta a cada tecla, na linha de comandos e para os projetos GitHub. Não há nada para instalar.</p>

## O ciclo de trabalho

![O ciclo de trabalho em assembly: escrever, montar, executar, depurar e depois exportar para o Apple-1 verdadeiro.](diagrams/pt/workflow-asm.svg "diagram large")

<ol class="steps">
<li><strong>Escrever</strong>: <strong>📝 Editor</strong>, depois <strong>🗋 Novo</strong> ou <strong>📂 Abrir</strong> um ficheiro <code>.s</code>, <code>.asm</code> ou <code>.a65</code>. A coloração da sintaxe distingue instruções, diretivas, etiquetas, números e comentários.</li>
<li><strong>Montar</strong>: nada a fazer, a montagem acompanha cada alteração. Em baixo no painel aparecem o tamanho, os endereços e a linha a escrever na máquina verdadeira; os erros são mostrados com a sua linha e coluna.</li>
<li><strong>Executar</strong>: <strong>▶ Executar</strong> põe os bytes na memória no endereço de <code>.org</code> e escreve o comando do Woz Monitor (<code>300R</code>).</li>
<li><strong>Depurar</strong>: <strong>⏺ Ponto de paragem</strong> na linha do cursor, depois passo a passo no depurador (capítulo 8), com as suas etiquetas na desmontagem.</li>
<li><strong>Exportar</strong>: <strong>📼 Exportar WAV</strong> escreve a cassete; no Apple-1 verdadeiro, escreva <code>C100R</code>, o intervalo indicado (<code>0300.031ER</code>) e depois <code>300R</code>.</li>
</ol>

![Uma fonte no editor: à esquerda o texto colorido, em baixo o resultado da montagem; à direita o Apple-1 executou o programa.](img/pt/brut/editeur-asm.png "large")

## A listagem e os símbolos

**📃 Listagem** mostra, para cada linha, o endereço, os bytes produzidos e a fonte; as linhas produzidas por uma macro são marcadas com `+`, os ficheiros incluídos são expandidos, e segue-se a tabela de símbolos. Um clique numa linha leva de volta à fonte; um clique na margem põe um ponto de paragem. Eis a listagem de `hello.s` (capítulo 23), produzida pela linha de comandos:

```text title="hello.lst"
0300  A2 00       start:  ldx #0
0302  BD 10 03    @next:  lda message,x   ; carácter seguinte
0305  F0 06               beq @done       ; 0 = fim da cadeia
0307  20 EF FF            jsr ECHO
030A  E8                  inx
030B  D0 F5               bne @next
030D  4C 1F FF    @done:  jmp GETLINE

                  message:
0310  8D                  .byte CR
0311  4F 4C 41 2C         .asciiz "OLA, APPLE-1!"
0315  20 41 50 50
…
; ---- Symboles (13)
; ECHO        $FFEF
; GETLINE     $FF1F
; message     $0310
; start       $0300
; start@done  $030D
; start@next  $0302
```

As etiquetas locais `@next` e `@done` pertencem a `start`: a tabela chama-lhes `start@next` e `start@done`, e o depurador aceita estes nomes como endereços de pontos de paragem. (A listagem escrita pela linha de comandos mantém o seu cabeçalho francês, *Symboles*.)

## A linha de comandos

{{asm (linha de comandos)|`apple1-emu asm`}} faz o mesmo sem janela:

```sh
apple1-emu asm hello.s                    # só verificar
apple1-emu asm hello.s -l hello.lst       # com a listagem
apple1-emu asm hello.s -o hello.wav       # cassete (.wav ou .aif)
apple1-emu asm hello.s -o hello.bin       # binário em bruto
apple1-emu asm hello.s -o hello.hex       # dump Woz, para colar no monitor
apple1-emu asm jeu.s -I ../common         # outra pasta para .include
```

Cada comando resume a montagem e dá a linha para o Apple-1 verdadeiro. O resumo é mostrado em francês («31 bytes … 2 ficheiro(s), 13 símbolos / Cassete (depois de C100R): 0300.031ER e depois 300R / escrito: hello.wav»):

```text
31 octets, $0300-$031E ; 2 fichier(s), 13 symboles
Cassette (après C100R) : 0300.031ER   puis 300R
écrit : hello.wav
```

Sem `.org`, o programa é colocado em `$0300`. O dump `.hex` tem a forma que o Woz Monitor aceita a partir do teclado, e termina com o comando de execução:

```text title="t1.hex (excerto)"
0280: 34 12 34 12 10 05 FF 34
0288: 12 FF FF A8 02 EA EA EA
…
0280R
```

## Sintaxe

Uma linha contém, por esta ordem e tudo opcional: uma etiqueta, uma instrução ou uma diretiva, e um comentário depois de `;`. Mnemónicas e diretivas podem escrever-se em minúsculas ou maiúsculas; os símbolos distinguem maiúsculas e minúsculas.

```asm
PTR     = $06           ; uma constante (nenhum byte produzido)
start:  lda #0          ; uma etiqueta global, seguida de dois pontos
@loop:  inx             ; uma etiqueta local: visível até à global seguinte
        bne @loop
:       dex             ; uma etiqueta anónima
        bne :-          ; :- a anterior, :+ a seguinte, :-- a de antes
```

{{!etiquetas locais}}{{!etiquetas anónimas}}

**Números**: `255` em decimal, `$FF` em hexadecimal, `%1010` em binário, `'A'` para um carácter ($41). O teclado devolve os seus códigos com o **bit 7 a um**: para comparar uma tecla, escreva `'A' | $80`; o `apple1.inc` define igualmente `CR = $8D`. `*` representa o endereço atual.

**Operadores**, da prioridade mais baixa para a mais alta:

| Prioridade | Operadores | Significado |
|---|---|---|
| 1 | <code>&#124;&#124;</code> | ou lógico (resultado 0 ou 1) |
| 2 | `&&` | e lógico |
| 3 | `=` `<>` `<` `>` `<=` `>=` | comparações (0 ou 1) |
| 4 | `+` `-` <code>&#124;</code> | soma, subtração, ou bit a bit |
| 5 | `*` `/` `&` `^` `<<` `>>` | multiplicação, divisão, e, ou exclusivo, deslocamentos |
| 6 (unários) | `<` `>` `-` `~` `!` | byte baixo, byte alto, negação, complemento, não lógico |

Assim, `lda #<message` carrega o byte baixo do endereço e `lda #>message` o byte alto; `#>S+1` é `(>S)+1`. Não há operador módulo (`%` ou `.mod` dão um erro de sintaxe).

**Página zero ou absoluto.** Um endereço abaixo de `$100` conhecido no momento da montagem é codificado em página zero (`lda $0006` dá `A5 06`, como com o ca65). Para forçar a forma absoluta, escreva `a:` (`lda a:$06` dá `AD 06 00`); para forçar a página zero, `z:`.

!!! attention "Defina as constantes de página zero antes de as usar"
    Se `PTR = $06` estiver escrito **depois** de `lda PTR`, o assembler ainda não conhece o valor e codifica o endereço em 2 bytes. Assinala-o a amarelo: `aviso: endereço da página zero codificado em 2 bytes (símbolo definido mais abaixo; defina-o antes, ou escreva z: ou a:)`. O programa funciona, mas fica mais comprido, e os modos `(zp),Y` tornam-se impossíveis.

## Diretivas

| Diretiva | Efeito | Exemplo |
|---|---|---|
| `.org end` | endereço de montagem (e de carregamento) | `.org $0300` |
| `.byte` (`.byt`) | bytes, cadeias incluídas | `.byte CR, "OK", 0` |
| `.word` (`.addr`) | palavras de 16 bits, byte baixo primeiro | `.word 10000, end` |
| `.asciiz` | cadeia terminada por zero | `.asciiz "OLA"` |
| `.res n[, val]` | reserva n bytes (0, ou val) | `.res 3, $EA` |
| `.include "f"` | insere um ficheiro | `.include "apple1.inc"` |
| `.macro nome args` … `.endmacro` | define uma macro (`.mac`, `.endmac`) | veja `macros.s` |
| `.local a, b` | etiquetas próprias de cada chamada da macro | |
| `.repeat n[, var]` … `.endrep` | repete um bloco, var = 0 … n−1 | `.repeat 16, I` |
| `.if` `.elseif` `.else` `.endif` | montagem condicional | `.if WIDTH > 30` |
| `.ifdef` `.ifndef` | consoante um símbolo exista ou não | `.ifdef APPLE1` |
| `.ifblank` `.ifnblank` | argumento da macro vazio ou não | |
| `.defined(sím)` | 1 se o símbolo existir (numa expressão) | `.if .defined(d)` |
| `.define nome valor` | substituição textual (`.undefine`) | `.define CHARACTER '*'` |
| `.setcpu "65C02"` | aceita as instruções do 65C02 (`.pc02`, `.p02`) | |
| `.segment` `.code` `.data` `.rodata` `.bss` | aceites por compatibilidade | |
| `.feature string_escapes` | `\n`, `\"`… nas cadeias | |

{{!.org}}{{!.byte}}{{!.word}}{{!.asciiz}}{{!.res}}{{!.include}}{{!.macro}}{{!.repeat}}{{!.if}}{{!.setcpu}}

!!! attention "Os segmentos não deslocam nada"
    Com o ca65 e o ld65, `.code`, `.data` e `.bss` enviam o código para zonas definidas pela configuração do editor de ligações. O assembler integrado produz **um único bloco**, pela ordem da fonte: as diretivas de segmento são aceites mas não têm efeito, e `.bss` produz zeros. Para um projeto que precise de um verdadeiro ficheiro `.cfg`, a biblioteca usa o ca65 e o ld65 (capítulo 4).

## Macros, .repeat e montagem condicional

Uma macro é um modelo de linhas, copiado a cada chamada com os seus argumentos. `.local` dá a cada chamada as suas próprias etiquetas, sem conflitos. `.repeat` desenrola um bloco no momento da montagem, o que também é prático para **calcular tabelas**:

```asm
quadrados:
.repeat 16, I
        .byte I * I     ; 0, 1, 4, 9 … 225
.endrep
```

O capítulo 23 apresenta `macros.s`, que usa tudo isto, e `gen2-lores.s`, que calcula os endereços das 24 linhas do ecrã com `.repeat`.

## Mensagens de erro

Cada erro indica o ficheiro, a linha, a coluna e o texto em causa; nenhum byte é escrito enquanto restar um erro. O editor mostra as mensagens na língua da interface (abaixo, em português); a linha de comandos mostra-as em francês.

| Mensagem | Causa habitual |
|---|---|
| `erro de sintaxe` | operador desconhecido, parêntese em falta |
| `instrução, diretiva ou macro desconhecida` | gralha; instrução do 65C02 sem `.setcpu`; etiqueta sem dois pontos |
| `modo de endereçamento impossível para esta instrução` | `sta #5`, `ldx ($06),y`… |
| `símbolo não definido` | `jsr ECO` em vez de `ECHO`; `.include` esquecido |
| `símbolo já definido` | duas etiquetas com o mesmo nome |
| `valor fora dos limites` | `lda #300`, endereço para lá de `$FFFF` |
| `salto demasiado longe (-128 a +127 bytes)` | substitua `bne longe` por `beq :+` / `jmp longe` / `:` |
| `ficheiro .include não encontrado` | nome, ou pasta: acrescente `-I` |
| `bloco mal fechado (.macro/.endmacro, .if/.endif)` | `.endif` ou `.endmacro` esquecido |
| `divisão por zero` | numa expressão |
| `cadeia ou carácter sem aspa de fecho` | `"OLA` |
| `valor desconhecido neste ponto (.org, .res, .if, .repeat)` | estas diretivas precisam de um valor já definido |

![Um erro no editor: ECO em vez de ECHO. Um clique na mensagem leva à linha.](img/pt/brut/editeur-asm-erreur.png "large")

## Compatibilidade com o ca65 e o xa

O assembler integrado segue a sintaxe do **ca65** (pacote cc65) em tudo o que um programa de um só bloco precisa. Os oito exemplos do capítulo 23 foram montados na sua versão original com as duas ferramentas e comparados byte a byte. As versões portuguesas só diferem nos comentários e nos textos; o seu tamanho com o assembler integrado está na última coluna (não foram de novo comparadas com o ca65):

| Fonte | Bytes (original) | Integrado = ca65 | Bytes (português) |
|---|---|---|---|
| `hello.s` | 33 | <span class="status-ok">idêntico</span> | 31 |
| `clavier.s` | 32 | <span class="status-ok">idêntico</span> | 32 |
| `hexa.s` | 64 | <span class="status-ok">idêntico</span> | 64 |
| `decimal.s` | 117 | <span class="status-ok">idêntico</span> | 117 |
| `jeu.s` | 222 | <span class="status-ok">idêntico</span> | 249 |
| `macros.s` | 135 | <span class="status-ok">idêntico</span> | 135 |
| `gen2-lores.s` | 110 | <span class="status-ok">idêntico</span> | 110 |
| `gen2-hires.s` | 94 | <span class="status-ok">idêntico</span> | 94 |

As fontes escritas para o **xa** (xa65) são reconhecidas pelas suas linhas `#include` ou `#define`, e traduzidas linha a linha antes da montagem (os números de linha dos erros continuam corretos): `#define`, `#include`, `*=`, `.BYT`, `.ASC`, comentários `/* */` e etiquetas na coluna 0 sem dois pontos. A apresentação de diapositivos de Dave Schmenk (capítulo 9) é montada assim, com os mesmos 3456 bytes do xa 2.4.1.

Para os **projetos GitHub**, a biblioteca prefere o ca65 e o ld65 se estiverem instalados (ou se o projeto fornecer uma configuração do ld65), com `-D APPLE1` por omissão; caso contrário, usa o assembler integrado. O ficheiro `apple1.toml` do repositório indica os endereços e os símbolos (capítulo 4).
