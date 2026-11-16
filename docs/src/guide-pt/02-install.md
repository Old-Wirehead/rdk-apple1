# Instalação em Linux

## Descomprimir e executar

<ol class="steps">
<li><strong>Descomprima o arquivo.</strong> Clique com o botão direito em <code>apple1-emu.zip</code> e depois em «Extrair aqui». Aparece uma pasta <code>apple1-emu</code>.</li>
<li><strong>Faça duplo clique em <code>run.sh</code>.</strong> Se o gestor de ficheiros perguntar o que fazer com o ficheiro (por exemplo no Linux Mint), escolha <strong>«Executar»</strong> (não «Mostrar»). A janela do Apple-1 abre-se, sem terminal.</li>
<li><strong>Opcional: acrescentar o emulador ao menu.</strong> Abra um terminal na pasta (botão direito → «Abrir num terminal») e escreva o comando abaixo. Aparece uma entrada <strong>Retro Dev Kit Apple 1</strong> na secção <em>Jogos</em> do menu, juntamente com os comandos <code>apple1-emu-gui</code> (janela) e <code>apple1-emu</code> (terminal).</li>
</ol>

```sh
./install.sh             # menu de aplicações + comandos em ~/.local/bin
./install.sh --bureau    # o mesmo, mais um ícone no ambiente de trabalho
./install.sh --desinstaller
```

A desinstalação (`--desinstaller`) remove os programas e a entrada do menu, mas conserva os seus programas (`~/apple1/programmes`) e as suas definições (`~/.config/apple1-emu`). Nenhum comando precisa de `sudo`: tudo é instalado na sua pasta pessoal. O primeiro arranque de uma nova versão propõe recomeçar com as definições de origem; a biblioteca é conservada. O botão **Repor as preferências** faz o mesmo. Um ficheiro vazio chamado `portable` junto ao programa (ou `APPLE1_PORTABLE=1`) guarda as preferências, a biblioteca predefinida e a cache dentro da pasta do kit.

!!! astuce "Atualizar a partir de uma versão anterior"
    Os scripts `lancer.sh` e `installer.sh` das versões anteriores chamam-se agora `run.sh` e `install.sh`. Execute de novo `./install.sh`: substitui a instalação antiga, bem como a entrada do menu e o ícone do ambiente de trabalho que apontavam para `lancer.sh`.

!!! astuce "A versão de terminal"
    `./run.sh --terminal` abre o emulador num terminal (pelo menos 42 colunas × 28 linhas), sem interface gráfica. Prático através de uma ligação SSH. O Manual completo descreve as suas teclas e comandos.

## Se a janela não abrir

`run.sh` deteta uma falha imediata (controlador gráfico, máquina virtual sem aceleração) e tenta de novo sozinho com a renderização por software do Mesa (`LIBGL_ALWAYS_SOFTWARE=1`). Se também falhar, aparece uma mensagem e o registo fica em `~/.cache/apple1-emu/gui.log`.

| Sintoma | O que fazer |
|---|---|
| O gestor de ficheiros abre `run.sh` no editor de texto | Botão direito → Propriedades → Permissões: assinale «Permitir executar o ficheiro como programa», faça de novo duplo clique e escolha «Executar». |
| «Permissão negada» no terminal | `chmod +x run.sh install.sh bin/*/apple1-*` |
| Janela preta ou muito lenta | Normal numa máquina virtual: a renderização por software continua fluida para o ecrã do Apple-1. Desative o «Ecrã vintage» se for preciso. |
| Sem som da cassete | O som passa pelo ALSA; sem placa de som fica simplesmente mudo. O carregamento funciona na mesma. |

## Onde ficam os seus ficheiros

| Localização | Conteúdo |
|---|---|
| `~/apple1/programmes` | A **biblioteca**: as suas cassetes, programas e exemplos (criada no primeiro arranque). |
| `~/.config/apple1-emu/gui.json` | Definições: zoom, cores, disposição dos painéis, separadores do editor. |
| `~/.cache/apple1-emu/` | Cassetes convertidas, projetos GitHub descarregados, o registo `gui.log`. |
