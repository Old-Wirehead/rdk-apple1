---
version: 1.0
date: novembro de 2026
---

# Bem-vindo

<p class="lead"><strong>Retro Dev Kit Apple 1</strong> — «O pecado original» — devolve a vida ao Apple-1 de Steve Wozniak (1976) num PC com Linux: o processador 6502, o ecrã de 40 × 24 caracteres, o Woz Monitor, a interface de cassete ACI e um BASIC integrado compatível com o Integer BASIC (RDK G2 Basic). Sobretudo, cria cassetes de áudio que pode reproduzir num Apple-1 verdadeiro.</p>

Em cerca de meia hora, este guia leva-o do arquivo descarregado até um programa BASIC seu, carregado numa máquina verdadeira, com uma captura de ecrã em cada passo. O **Manual completo** descreve cada painel em pormenor, com guias completos do Integer BASIC e do assembler 6502.

## O que é preciso

<div class="cards">
<div class="card"><h4>Um PC com Linux</h4>Uma distribuição Linux recente, de 64 bits (x86_64) ou de 32 bits (i686). Nada a instalar: o emulador usa as bibliotecas gráficas já presentes (X11 ou Wayland, OpenGL/Mesa).</div>
<div class="card"><h4>O arquivo <code>apple1-emu.zip</code></h4>Os dois programas (janela e terminal), exemplos, este guia e o Manual completo em PDF.</div>
<div class="card"><h4>O BASIC está incluído</h4><strong>RDK G2 Basic</strong>, um BASIC livre compatível com o Integer BASIC, vem integrado. O original da Apple de 1976 não está incluído (continua protegido por direitos de autor): se o quiser, use a sua própria cópia, como a cassete <code>BASIC.wav</code> da coleção «Apple1Cassettes» do archive.org.</div>
<div class="card"><h4>Opcional: um Apple-1 verdadeiro</h4>Com uma placa de cassete ACI: original, réplica ou kit. Um smartphone ou a saída de auscultadores de um PC serve de gravador.</div>
</div>

## Como ler este guia

Os comandos a escrever no Apple-1 aparecem `ASSIM`. As sessões surgem sobre fundo preto, como o ecrã da época; o que escreve aparece a branco e a resposta da máquina a verde:

```mon
» 300R
0300: A9A
```

Ao longo do caminho surgem três tipos de caixas:

!!! astuce "Dica"
    Um atalho, ou uma forma mais rápida de obter o mesmo resultado.

!!! attention "Atenção"
    Uma armadilha frequente, ou uma ação que pode fazê-lo perder o seu trabalho.

!!! apple1 "No Apple-1 verdadeiro"
    O que muda ao passar do emulador para a máquina verdadeira.
