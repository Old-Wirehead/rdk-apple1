# E agora? {: .newpage }

Já sabe carregar uma cassete, escrever e executar um programa BASIC e transferi-lo para uma máquina verdadeira. O **Manual completo** (em inglês: `docs/Complete-Manual.pdf`) vai muito mais longe:

- **Guia do Integer BASIC**: cada instrução com exemplos, os limites, as mensagens de erro, os erros conhecidos e como os contornar, a adaptação de programas Applesoft.
- **Guia do assembler 6502**: o assembler integrado (sintaxe do ca65, macros, listagem), as rotinas do Woz Monitor, exemplos testados no emulador.
- **Jogos para jogar e ler**: a biblioteca traz cinco pequenos jogos de Claude IA (Forca, Fósforos, Batalha naval em BASIC; Túnel e Jogo do galo em assembler), cada um em francês e inglês.
- **As ferramentas**: depurador, biblioteca e projetos GitHub, placa a cores GEN2 e placa gráfica P-LAB, foto → ASCII, ecrã vintage, linha de comandos.

## Atalhos essenciais

| Tecla | Efeito |
|---|---|
| Ctrl+R | RESET (regresso ao monitor) |
| Ctrl+L | CLEAR SCREEN |
| F3 | velocidade real ou velocidade máxima |
| Ctrl+V | escreve a área de transferência no teclado do Apple-1 (também: botão direito no ecrã → Colar) |
| Ctrl+mais, Ctrl+menos, Ctrl+0 | zoom do ecrã e zoom ajustado à janela (no editor: tamanho do texto) |
| F11 | ecrã inteiro |
| F1 | balões de ajuda detalhados (como o botão ?) |
| Retrocesso | `_`, o apagamento do monitor |
| Ctrl+F, Ctrl+H, Ctrl+G | no editor: procurar, substituir, ir para uma linha |
| F5, F9 | no editor: executar o separador, ponto de paragem na linha |

## Comandos a lembrar

| Comando | Efeito |
|---|---|
| `E000R` | arranca o Integer BASIC (a frio: apaga o programa) |
| `E2B3R` | regressa ao BASIC sem apagar o programa |
| `C100R` | arranca o programa da ACI (prompt `*`) |
| `início.fimR` | lê a cassete para esse intervalo, por exemplo `0300.031ER` |
| `300R` | executa um programa em linguagem máquina em `$0300` |
