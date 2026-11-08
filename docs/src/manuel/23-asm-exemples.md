# Exemples commentés

<p class="lead">Huit programmes, du plus simple au plus complet. Ils sont dans le dossier <code>docs/exemples/asm/</code> de la distribution ; chacun a été assemblé par l’assembleur intégré et par ca65 (mêmes octets), puis exécuté dans l’émulateur. Les sorties montrées ici sont celles de l’émulateur.</p>

Pour les essayer : ouvrez le fichier dans l’éditeur et cliquez sur **▶ Lancer**, ou bien, en ligne de commande, `apple1-emu asm hello.s -o hello.wav` puis lisez la cassette (chapitre 5). Gardez `apple1.inc` (et `gen2.inc`) dans le même dossier que les sources.

| Fichier | Octets | Adresse | Ce qu’il montre |
|---|---|---|---|
| `hello.s` | 36 | `$0300` | une chaîne, `ECHO`, étiquettes locales, retour au moniteur |
| `clavier.s` | 32 | `$0300` | lire le clavier, comparer des caractères |
| `hexa.s` | 64 | `$0300` | pointeur en page zéro, `(zp),Y`, addition 16 bits, `PRBYTE` |
| `decimal.s` | 117 | `$0300` | nombres de 16 bits, soustractions, tables de mots |
| `jeu.s` | 254 | `$0300` | un jeu complet : hasard, sous-programmes, messages |
| `macros.s` | 137 | `$0300` | macros, `.local`, `.repeat`, `.if`, tables calculées |
| `gen2-lores.s` | 110 | `$0C00` | la carte GEN2 en LORES |
| `gen2-hires.s` | 94 | `$0C00` | la carte GEN2 en HIRES |

## `hello.s` : afficher un message

@@include exemples/asm/hello.s

X parcourt la chaîne jusqu’à l’octet 0 posé par `.asciiz`. Le `bne @suite` après `inx` n’est jamais faux pour une chaîne de moins de 256 caractères : c’est un saut court et sûr. Le programme finit par `jmp GETLINE`, jamais par `RTS` (chapitre 22).

```mon
\
» 300R
0300: A2
BONJOUR, APPLE-1 !
```

Le moniteur affiche d’abord `0300: A2`, l’octet à l’adresse de lancement, puis le programme prend la main.

## `clavier.s` : l’écho des touches

@@include exemples/asm/clavier.s

`cmp #'0' | $80` compare au code du chiffre 0, avec le bit 7 comme le clavier. `bcc` (« plus petit ») et `bcs` (« plus grand ou égal ») encadrent les chiffres. Avec les touches `A`, `B`, `1`, `2` puis ESC, l’écran montre :

```text
0300: ADAB1122
\
```

Le `AD` vient du moniteur (`lda KBDCR` commence par l’opcode `$AD`) ; `AB` puis `11` et `22` viennent du programme ; ESC rend la main au moniteur, qui affiche `\`.

## `hexa.s` : un vidage façon moniteur

@@include exemples/asm/hexa.s

`PTR` et `PTR+1` forment un **pointeur 16 bits** en page zéro, octet bas d’abord ; `lda (PTR),y` lit l’octet à l’adresse PTR + Y. L’addition `PTR += 8` montre la retenue : `clc`, `adc #8`, et si la retenue sort (`bcc` non pris), `inc PTR+1`. Le résultat est le début du Woz Monitor :

```text
FF00: D8 58 A0 7F 8C 12 D0 A9
FF08: A7 8D 11 D0 8D 13 D0 C9
```

## `decimal.s` : afficher un nombre en décimal

@@include exemples/asm/decimal.s

Le 6502 ne sait pas diviser : `prdec` soustrait 10000 autant de fois que possible (le compte donne le chiffre des dizaines de milliers), puis 1000, 100 et 10. La soustraction 16 bits se fait en deux temps, `sbc` sur l’octet bas puis sur l’octet haut, la retenue faisant le lien. `FLAG` supprime les zéros de tête. Sortie :

```text
0 7 42 1976 6502 65535
```

!!! astuce "Conserver X dans un sous-programme"
    `prdec` utilise X pour lui-même, alors que la boucle principale en a besoin : elle le sauve avec `txa` / `pha` au début et le restaure avec `pla` / `tax` à la fin. Le 6502 NMOS n’a pas de `PHX`.

## `jeu.s` : devinez le chiffre

@@include exemples/asm/jeu.s

Le hasard vient du joueur : X compte sans arrêt (`@att`) jusqu’à la première touche, puis le reste de la division par 10 donne le chiffre secret. `prmsg` reçoit l’adresse d’un message dans **Y:X** (octet haut dans Y), la range dans le pointeur `MSG`, et l’affiche avec `(MSG),y`. Une partie, en tapant `x` puis les chiffres dans l’ordre :

```text
J'AI CHOISI UN CHIFFRE. APPUYEZ SUR UNE TOUCHE.
VOTRE CHIFFRE ? 0 C'EST PLUS.
VOTRE CHIFFRE ? 1 C'EST PLUS.
…
VOTRE CHIFFRE ? 7 C'EST PLUS.
VOTRE CHIFFRE ? 8 GAGNE EN 9 COUPS !
```

## `macros.s` : laisser travailler l’assembleur

@@include exemples/asm/macros.s

`putc` et `print` produisent du code à chaque appel ; `.local` évite que les deux appels de `print` définissent deux fois `boucle`. `.repeat 3` déroule trois fois la ligne d’étoiles (chacune avec sa propre étiquette anonyme), `.if` choisit le message selon `LARGEUR`, et la table `carres` est calculée à l’assemblage. Sortie :

```text
MACROS ET .REPEAT
********************
********************
********************
LIGNES COURTES
```

Changez `LARGEUR = 20` en `40` : l’éditeur réassemble aussitôt, les lignes s’allongent et le message devient `LIGNES LONGUES`.

## `gen2-lores.s` : la couleur en basse résolution

Ces deux programmes demandent la carte GEN2 (**⚙ Paramètres ▸ Carte couleur**, chapitre 6) et se lancent en `$0C00`, au-dessus de la page LORES.

@@include exemples/asm/gen2-lores.s

Les deux tables `lignes_lo` et `lignes_hi` sont calculées par `.repeat` avec la formule des adresses de rangée du chapitre 6. Chaque octet porte deux blocs, d’où le quartet copié en haut et en bas. Les `bit SS_…` sont des **lectures** : c’est ainsi que se commutent les modes de la carte.

![gen2-lores.s : les seize couleurs LORES, deux colonnes chacune, sur le moniteur couleur.](img/brut/gen2-lores.png "large")

## `gen2-hires.s` : la haute résolution

@@include exemples/asm/gen2-hires.s

En HIRES, la page `$2000-$3FFF` est découpée en blocs de 128 octets dont les 40 premiers vont dans le tiers haut de l’écran, les 40 suivants au milieu et les 40 d’après en bas. Le programme remplit chaque tiers avec un motif différent, en alternant deux octets selon la parité de la colonne : on obtient trois bandes de couleur.

![gen2-hires.s : trois bandes de couleur en HIRES : violet, vert et bleu.](img/brut/gen2-hires.png "large")
