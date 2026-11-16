# Conselhos, armadilhas e limites

<p class="lead">O que se aprende a escrever programas para o Apple-1, primeiro no emulador e depois na máquina verdadeira.</p>

## Onde pôr um programa

| Endereço | Para |
|---|---|
| `$0280` | o mais abaixo possível: logo a seguir ao buffer do monitor |
| `$0300` | a escolha habitual (e o endereço por omissão do assembler) |
| `$0C00` | com a placa GEN2 em LORES ou texto (páginas `$0400-$0BFF` deixadas livres para o ecrã) |
| abaixo do HIMEM | com o Integer BASIC: reserve o espaço (capítulo 16) |

Para a página zero, os exemplos usam `$06-$09`, que o monitor deixa livres. Se o programa tiver de coexistir com o BASIC, escolha endereços que o BASIC não use, ou guarde-os e restaure-os.

## Viver com o alcance dos saltos condicionais

Um salto condicional só chega a 127 bytes para a frente ou 128 para trás. Quando o assembler responde `salto demasiado longe`, inverta a condição e salte por cima de um `JMP`:

```asm
        beq :+          ; em vez de: bne longe
        jmp longe
:
```

## As cadeias e o bit 7

O teclado devolve códigos com o bit 7 a um, e o monitor compara com esses códigos. Já o ecrã ignora o bit 7: `.asciiz "OLA"` mostra-se perfeitamente. Para **comparar** uma tecla, ponha o bit 7 na constante: `cmp #'Q' | $80`. As minúsculas de uma cadeia são mostradas em maiúsculas.

## Esperar um pouco

A 1,023 MHz (0,96 MHz efetivos), um ciclo `dex` / `bne` custa 5 ciclos de relógio por volta:

```asm
pausa:
        ldy #78         ; 78 x 256 voltas de 5 ciclos: cerca de 0,1 s
:       ldx #0
:       dex
        bne :-
        dey
        bne :--
        rts
```

O emulador à velocidade real respeita estas durações; à velocidade rápida (<kbd>F3</kbd>), desaparecem. Os ciclos de temporização da ACI contam-se da mesma maneira em ciclos (apêndice C).

## Depurar com eficácia

- Ponha os pontos de paragem numa **etiqueta** (`start`, `prdec`, `start@done`) em vez de num endereço: continua certo quando o código se desloca.
- Passe por cima de `jsr ECHO` com **↪ Por cima**: senão seguirá o ciclo de espera do ecrã instrução a instrução.
- Se o Apple-1 deixar de responder depois do seu programa, veja o PC: na página zero, é quase sempre um `BRK` ou um `RTS` perdido (capítulo 22).
- O **dump** mostra a página zero: os seus ponteiros veem-se lá, byte baixo primeiro.

![O depurador parado numa fonte 6502: registos, indicadores, desmontagem com etiquetas e memória.](img/pt/brut/debogueur-asm.png "large")

## Rumo ao Apple-1 verdadeiro

<ol class="steps">
<li>Monte e experimente no emulador, <strong>à velocidade real</strong> e com a visualização limitada a 60 caracteres por segundo.</li>
<li>Exporte a cassete (<strong>📼 Exportar WAV</strong>, ou <code>-o prog.aif</code>): o AIFF é o formato mais seguro se reproduzir a partir de um smartphone.</li>
<li>No Apple-1: <code>C100R</code>, depois o intervalo indicado (<code>0300.031ER</code>), inicie a reprodução e depois <code>300R</code>.</li>
<li>Sem ACI: o dump <code>.hex</code> pode ser escrito (ou colado num terminal série) diretamente no Woz Monitor; a sua última linha executa o programa.</li>
</ol>

## Limites do assembler integrado

- Um único bloco de saída, pela ordem da fonte: os segmentos não têm efeito e não há editor de ligações. Para código repartido por várias zonas, use o ca65 e o ld65.
- Nem operador módulo, nem `.proc`, `.scope`, `.struct` ou `.enum`: são diretivas do ca65 que o assembler integrado não conhece.
- As instruções do 65C02 são montadas, mas o processador emulado é um 6502 NMOS: não funcionam no emulador (capítulo 20).
- Na **listagem**, uma linha produzida por uma macro perde por vezes o espaço entre a instrução e o operando (`ldastring, x` por `lda string,x`). É só um defeito de visualização da listagem: os bytes estão certos.
