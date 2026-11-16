# Fotografia → ASCII

<p class="lead">A ferramenta Fotografia → ASCII transforma uma fotografia numa imagem do Apple-1 de 40 × 24 caracteres e depois num programa BASIC ou 6502, pronto a executar ou a exportar para cassete.</p>

![A ferramenta Fotografia → ASCII: a fotografia com o seu quadro de recorte, as definições e a pré-visualização desenhada com o tipo de letra do 2513.](img/pt/brut/ascii-outil.png "large")

## Abrir uma imagem

**🎨 ASCII** abre a ferramenta, um painel acoplável como os outros. **Arraste e largue** uma fotografia `.jpg` ou `.png` sobre a janela, ou use **📂 Abrir…** (uma fotografia, ou uma arte ASCII já feita em `.txt`).

O quadro verde sobre a fotografia mostra a parte que é convertida: arraste-o para o mover; a roda do rato faz zoom.

## Definições

| Definição | Função |
|---|---|
| **Colunas**, **Linhas** | 40 × 24 no máximo. **23 linhas** deixa a última linha para o prompt, para que a imagem não deslize. |
| **Zoom**, **Centro** | a parte da fotografia que se mantém |
| **Brilho**, **Contraste**, **Gama** | as definições habituais (gama > 1 aclara) |
| **Inverter** | escuro = muita tinta |
| **Níveis auto** | estica os níveis de cinzento (de 2 % a 98 %) antes das outras definições |
| **Pontilhado** | Floyd-Steinberg: mais tons, mais ruído |
| **Caracteres** | a escala **completa** (26 níveis), **Simples** (10), a da **apresentação de Schmenk**, ou **Personalizada** |

Só são possíveis os caracteres do 2513: maiúsculas, algarismos e símbolos, sem minúsculas, sem `"` e sem `_`. São ordenados pelo número de pontos acesos no tipo de letra do Apple-1: é esse número, e não a forma, que dá o nível de cinzento.

!!! astuce "Para um retrato"
    40 × 24 caracteres é pouco: recorte sobre o rosto (zoom de 1,3 a 1,6), suba um pouco o contraste e experimente uma gama inferior a 1 para uma fotografia clara. Um fundo escuro e liso dá os melhores resultados.

## O programa produzido

- **BASIC**: uma linha `PRINT "…"` por linha do desenho. Uma linha completa de 40 caracteres termina em `;`, porque o ecrã muda de linha sozinho. O programa demora a ser escrito (cerca de 2 s por linha).
- **6502**: uma fonte ca65 em `$0300`, arrancada com `300R`. Depois de limpar o ecrã, um ciclo envia ao ECHO uma tabela comprimida (sequências de caracteres idênticos codificadas como «número + carácter»). A visualização é instantânea.

Com 24 linhas, o programa espera uma tecla antes de devolver o controlo; com 23, o prompt chega à última linha e a imagem fica inteira.

Os botões: **✎ Inserir no editor** (novo separador BASIC ou 6502), **▶ Executar**, **📼 Exportar WAV…**, **💾 Salvar na biblioteca** (um `.txt` na pasta «Art ASCII»), **📋 Copiar o texto**, **Ver o código**.

![Da fotografia ao Apple-1 verdadeiro: conversão, programa, cassete.](diagrams/pt/workflow-ascii.svg "diagram large")

## Na linha de comandos

```sh
apple1-emu ascii foto.jpg                        # só o desenho, no ecrã
apple1-emu ascii woz.jpg --zoom 1.3 --cx 0.45 --cy 0.32 \
          --contrast 1.3 --gamma 0.45 --rows 23 -o woz.txt
apple1-emu ascii woz.jpg -o woz.bas              # programa BASIC
apple1-emu ascii woz.jpg -o woz.s                # fonte 6502
apple1-emu ascii woz.jpg -o woz.wav              # cassete 6502, arrancada com 300R
apple1-emu ascii woz.jpg -o woz.wav --format basic
```

O capítulo 12 dá todas as opções.

!!! apple1 "A apresentação de Dave Schmenk"
    Em 2014, Dave Schmenk escreveu uma apresentação de retratos para o Apple-1. A biblioteca acrescenta-a como projeto GitHub (`dschmenk/Apple1-Slideshow`): é descarregada, não é fornecida, porque o repositório não tem licença. **▶ Testar a última compilação** monta-a (sintaxe do xa, traduzida automaticamente; os mesmos 3456 bytes que o xa 2.4.1) e executa-a com `280R`.
