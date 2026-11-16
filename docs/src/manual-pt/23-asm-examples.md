# Exemplos comentados

<p class="lead">Oito programas, do mais simples ao mais completo. Estão na pasta <code>docs/exemples/pt/asm/</code> da distribuição; cada um foi montado pelo assembler integrado e executado no emulador. As saídas mostradas aqui são as do emulador.</p>

Para os experimentar: abra o ficheiro no editor e clique em **▶ Executar**, ou, na linha de comandos, `apple1-emu asm hello.s -o hello.wav` e depois reproduza a cassete (capítulo 5). Mantenha `apple1.inc` (e `gen2.inc`) na mesma pasta que as fontes.

| Ficheiro | Bytes | Endereço | O que mostra |
|---|---|---|---|
| `hello.s` | 31 | `$0300` | uma cadeia, `ECHO`, etiquetas locais, regresso ao monitor |
| `clavier.s` | 32 | `$0300` | ler o teclado, comparar caracteres |
| `hexa.s` | 64 | `$0300` | ponteiro na página zero, `(zp),Y`, soma de 16 bits, `PRBYTE` |
| `decimal.s` | 117 | `$0300` | números de 16 bits, subtrações, tabelas de palavras |
| `jeu.s` | 249 | `$0300` | um jogo completo: acaso, sub-rotinas, mensagens |
| `macros.s` | 135 | `$0300` | macros, `.local`, `.repeat`, `.if`, tabelas calculadas |
| `gen2-lores.s` | 110 | `$0C00` | a placa GEN2 em LORES |
| `gen2-hires.s` | 94 | `$0C00` | a placa GEN2 em HIRES |

As versões originais destes programas também foram montadas com o ca65 (mesmos bytes, capítulo 21); as versões portuguesas só diferem nos comentários e nos textos.

## `hello.s`: mostrar uma mensagem

@@include exemples/pt/asm/hello.s

X percorre a cadeia até ao byte 0 colocado por `.asciiz`. O `bne @next` depois de `inx` nunca é falso para uma cadeia com menos de 256 caracteres: é um salto curto e seguro. O programa termina com `jmp GETLINE`, nunca com `RTS` (capítulo 22).

```mon
\
» 300R
0300: A2
OLA, APPLE-1!
```

O monitor mostra primeiro `0300: A2`, o byte do endereço de arranque, e depois o programa toma conta.

## `clavier.s`: repetir as teclas

@@include exemples/pt/asm/clavier.s

`cmp #'0' | $80` compara com o código do algarismo 0, com o bit 7 a um como o teclado o dá. `bcc` («menor») e `bcs` («maior ou igual») enquadram os algarismos. Com as teclas `A`, `B`, `1`, `2` e depois ESC, o ecrã mostra:

```text
0300: ADAB1122
\
```

O `AD` vem do monitor (`lda KBDCR` começa pelo opcode `$AD`); `AB`, depois `11` e `22` vêm do programa; ESC devolve o controlo ao monitor, que mostra `\`.

## `hexa.s`: um dump ao estilo do monitor

@@include exemples/pt/asm/hexa.s

`PTR` e `PTR+1` formam um **ponteiro de 16 bits** na página zero, byte baixo primeiro; `lda (PTR),y` lê o byte no endereço PTR + Y. A soma `PTR += 8` mostra o transporte a trabalhar: `clc`, `adc #8`, e se sair um transporte (`bcc` não tomado), `inc PTR+1`. O resultado é o início do Woz Monitor:

```text
FF00: D8 58 A0 7F 8C 12 D0 A9
FF08: A7 8D 11 D0 8D 13 D0 C9
```

## `decimal.s`: mostrar um número em decimal

@@include exemples/pt/asm/decimal.s

O 6502 não sabe dividir: `prdec` subtrai 10000 tantas vezes quantas puder (a contagem dá o algarismo das dezenas de milhar), depois 1000, 100 e 10. A subtração de 16 bits faz-se em dois tempos, `sbc` no byte baixo e depois no alto, ligados pelo transporte. `FLAG` suprime os zeros à esquerda. Saída:

```text
0 7 42 1976 6502 65535
```

!!! astuce "Preservar X numa sub-rotina"
    `prdec` usa X para si, enquanto o ciclo principal precisa dele: guarda-o com `txa` / `pha` no início e restaura-o com `pla` / `tax` no fim. O 6502 NMOS não tem `PHX`.

## `jeu.s`: adivinhar o algarismo

@@include exemples/pt/asm/jeu.s

O acaso vem do jogador: X conta sem parar (`@wait`) até à primeira tecla, e o resto da divisão por 10 dá o algarismo secreto. `prmsg` recebe o endereço de uma mensagem em **Y:X** (byte alto em Y), guarda-o no ponteiro `MSG` e mostra-a com `(MSG),y`. Uma partida, escrevendo `x` e depois os algarismos por ordem:

```text
ESCOLHI UM DIGITO. CARREGUE NUMA TECLA.
O TEU ALGARISMO? 0 MAIS ALTO.
O TEU ALGARISMO? 1 MAIS ALTO.
…
O TEU ALGARISMO? 5 MAIS ALTO.
O TEU ALGARISMO? 6 GANHO EM 7 TENTATIVAS
!
```

A última mensagem tem 41 caracteres: depois da 40.ª coluna, o cursor passa sozinho para a linha seguinte, e o `!` fica sozinho (capítulo 22).

## `macros.s`: deixar o assembler trabalhar

@@include exemples/pt/asm/macros.s

`putc` e `print` geram código a cada chamada; `.local` impede que as duas chamadas de `print` definam duas vezes `loop`. `.repeat 3` desenrola três vezes a linha de asteriscos (cada uma com a sua etiqueta anónima), `.if` escolhe a mensagem conforme `WIDTH`, e a tabela `squares` é calculada no momento da montagem. Saída:

```text
MACROS E .REPEAT
********************
********************
********************
LINHAS CURTAS
```

Mude `WIDTH = 20` para `40`: o editor volta a montar de imediato, as linhas ficam mais compridas e a mensagem passa a `LINHAS LONGAS`.

## `gen2-lores.s`: cor em baixa resolução

Estes dois programas precisam da placa GEN2 (**⚙ Preferências ▸ Placa de cor Uncle Bernie GEN2**, capítulo 6) e começam em `$0C00`, acima da página LORES.

@@include exemples/pt/asm/gen2-lores.s

As duas tabelas `rows_lo` e `rows_hi` são calculadas por `.repeat` com a fórmula dos endereços de linha do capítulo 6. Cada byte leva dois blocos, daí o nibble copiado para a metade de cima e a de baixo. As instruções `bit SS_…` são **leituras**: é assim que se mudam os modos da placa.

![gen2-lores.s: as dezasseis cores LORES, duas colunas cada, no monitor a cores.](img/pt/brut/gen2-lores.png "large")

## `gen2-hires.s`: alta resolução

@@include exemples/pt/asm/gen2-hires.s

Em HIRES, a página `$2000-$3FFF` está dividida em blocos de 128 bytes cujos primeiros 40 bytes vão para o terço de cima do ecrã, os 40 seguintes para o meio e os 40 depois desses para baixo. O programa preenche cada terço com um motivo diferente, alternando dois bytes conforme a paridade da coluna: o resultado são três faixas de cor.

![gen2-hires.s: três faixas de cor em HIRES: violeta, verde e azul.](img/pt/brut/gen2-hires.png "large")
