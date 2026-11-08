# Le clavier, l’écran et le Woz Monitor

<p class="lead">L’Apple-1 n’a que quatre registres d’entrée-sortie, dans un PIA 6820 (ou 6821), et 256 octets de moniteur. C’est peu, mais cela suffit : ce chapitre montre comment lire une touche, écrire un caractère et utiliser les routines de Steve Wozniak.</p>

## Les quatre registres du PIA

| Adresse | Nom | Lecture | Écriture |
|---|---|---|---|
| `$D010` | `KBD` | code de la touche, bit 7 à 1 ; la lecture **acquitte** la touche | — |
| `$D011` | `KBDCR` | bit 7 = 1 : une touche attend | configuration (faite par le moniteur) |
| `$D012` | `DSP` | bit 7 = 1 : l’écran est occupé | caractère à afficher (7 bits) |
| `$D013` | `DSPCR` | — | configuration (faite par le moniteur) |

{{!KBD}}{{!KBDCR}}{{!DSP}}

Ces noms sont définis dans `apple1.inc`, inclus par tous les exemples :

@@include exemples/asm/apple1.inc

## Lire une touche

On attend que le bit 7 de `KBDCR` passe à 1, puis on lit `KBD` :

```asm
lire:   lda KBDCR       ; une touche ?
        bpl lire        ; bit 7 = 0 : pas encore
        lda KBD         ; code ASCII + $80
```

Le clavier ne produit que des majuscules (l’émulateur convertit les minuscules), avec le bit 7 à 1 : `A` donne `$C1`, RETURN `$8D`, ESC `$9B`. Pour savoir si une touche est appuyée **sans attendre**, testez seulement `KBDCR` :

```asm
        lda KBDCR
        bpl rien        ; pas de touche : on continue le travail
        lda KBD         ; une touche : on la lit (et on l'acquitte)
```

`jeu.s` (chapitre 23) s’en sert pour tirer un nombre au hasard : il compte en boucle jusqu’à ce que le joueur appuie sur une touche.

## Écrire un caractère

Le terminal de l’Apple-1 prend un caractère à la fois, et seulement quand il est prêt : le bit 7 de `DSP` reste à 1 tant que le caractère précédent n’est pas affiché. La routine `ECHO` du moniteur fait l’attente pour vous. Voici ses quatre instructions, lues dans le moniteur de l’émulateur :

```mon
\
» FFEF.FFFFR

FFEF: 2C
FFF0: 12 D0 30 FB 8D 12 D0 60
FFF8: 00 00 00 0F 00 FF 00 00
```

Soit `BIT DSP` / `BMI ECHO` / `STA DSP` / `RTS`. `ECHO` conserve A, X et Y. Comme le terminal d’origine n’affiche qu’**un caractère par image**, environ 60 par seconde, un programme qui écrit beaucoup est limité par l’écran, pas par le processeur ; l’émulateur respecte cette limite par défaut (<kbd>F4</kbd> la lève, chapitre 2).

Ce que fait l’écran de chaque code :

- **RETURN** (`$8D`, ou `$0D`) passe à la ligne ; en bas de l’écran, tout remonte d’une ligne.
- Les **autres caractères de contrôle** (`$00-$1F`, avec ou sans bit 7) sont ignorés : pas d’effacement, pas de retour arrière, pas de sonnerie.
- Les **minuscules** `$60-$7F` s’affichent comme les majuscules `$40-$5F` : le terminal ne garde que 6 bits.
- Après la 40e colonne, le curseur passe seul à la ligne suivante.
- Il n’y a pas de code pour effacer l’écran : c’est un bouton de la machine (**Effacer écran**, <kbd>Ctrl</kbd>+<kbd>L</kbd>).

## Les routines du Woz Monitor

| Adresse | Nom | Entrée | Effet | Registres |
|---|---|---|---|---|
| `$FFEF` | `ECHO` | A = caractère | l’affiche | A, X, Y conservés |
| `$FFDC` | `PRBYTE` | A = octet | l’affiche en 2 chiffres hexadécimaux | A détruit ; X, Y conservés |
| `$FFE5` | `PRHEX` | A (quartet bas) | affiche un chiffre hexadécimal | A détruit ; X, Y conservés |
| `$FF1F` | `GETLINE` | — | retour au moniteur : à la ligne, puis attend une commande | — |
| `$FF1A` | `ESCAPE` | — | comme `GETLINE`, en affichant d’abord `\` | — |
| `$FF00` | `RESET` | — | initialise le PIA et le moniteur | — |

{{!ECHO}}{{!PRBYTE}}{{!PRHEX}}{{!GETLINE}}

Le moniteur utilise la page zéro `$24-$2B` et le tampon de ligne `$0200-$027F` ; tout le reste est à vous (sauf si l’Integer BASIC doit survivre, chapitre 16). Ses commandes (`300R`, `0300.031F`, `300: A9 8D`…) sont résumées à l’annexe B.

## Lancer et terminer un programme

Un programme est lancé par la commande `R` du moniteur (`300R`), qui fait un `JMP` à l’adresse, pas un `JSR`. Il n’y a donc **pas d’adresse de retour** sur la pile : terminez par `jmp GETLINE` (`$FF1F`), pas par `RTS`.

!!! attention "RTS et BRK ne rendent pas la main"
    Un `RTS` à la fin d’un programme lancé par `R` dépile n’importe quoi. Un `BRK` saute au vecteur d’interruption de la ROM, qui vaut `$0000` (les trois derniers mots du moniteur, ci-dessus : NMI `$0F00`, RESET `$FF00`, IRQ/BRK `$0000`) : le processeur exécute alors la page zéro. Dans les deux cas, faites RESET (<kbd>Ctrl</kbd>+<kbd>R</kbd>).

Un programme appelé depuis le BASIC par `CALL 768` est au contraire appelé par un `JSR` : il doit finir par `RTS` pour revenir au BASIC. Le chapitre 16 montre comment loger du code machine sous le programme BASIC.

```mon
>» POKE 768,96
>» CALL 768
>
```

(`96` = `$60`, l’opcode de `RTS` : le plus petit sous-programme possible.)

## Le PIA, la cassette et la carte couleur

La carte ACI occupe `$C000-$C0FF` (chaque accès fait basculer la sortie) et son programme `$C100-$C1FF` ; le chapitre 5 explique son fonctionnement et l’annexe C ses temps. La carte GEN2 se commande par simple **lecture** de `$C250-$C257` (`bit SS_GR`) ; ses adresses sont dans `gen2.inc` :

@@include exemples/asm/gen2.inc
