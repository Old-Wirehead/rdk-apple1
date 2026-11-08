# Variables, chaînes, tableaux et mémoire

## Les nombres

L’Integer BASIC ne connaît que des **entiers de 16 bits signés**, de −32767 à 32767. Un calcul qui sort de cet intervalle s’arrête sur `*** >32767 ERR` (`200*200`, `32767+1`, et aussi `10/0` et `RND(0)`). La valeur −32768 n’est pas utilisable (chapitre 18, bug 5). Il n’y a ni virgule, ni `INT` : `7/2` vaut 3.

## Les noms de variables

C’est la limite qui surprend le plus : un nom de variable est **une lettre, éventuellement suivie d’un chiffre**.

| Nom | Résultat (essayé) |
|---|---|
| `A`, `Z`, `B1`, `Z9` | accepté |
| `AB`, `CPT`, `SCORE`, `COMPTEUR` | `SYNTAX ERR` |
| `A12` | `SYNTAX ERR` |
| `A$`, `N$` | chaîne : **une lettre** seulement |
| `B9$`, `Z9$` | `SYNTAX ERR` |

Il y a donc 26 × 11 = 286 variables entières possibles, et 26 chaînes. Prenez l’habitude de noter dans un `REM` le rôle de chaque variable.

## Les chaînes

Une chaîne doit être **dimensionnée** avant usage, avec sa longueur maximale (1 à 255) : `DIM N$(20)`. `DIM B$(256)` donne `>255 ERR`.

| Opération | Écriture | Exemple (N$ = "KAYAK") |
|---|---|---|
| longueur | `LEN(N$)` | 5 |
| sous-chaîne | `N$(i,j)` : caractères i à j | `N$(1,3)` = "KAY" |
| fin | `N$(i)` : du caractère i à la fin | `N$(4)` = "AK" |
| concaténer | `A$(LEN(A$)+1)=B$` | "APPLE" + "-1" → "APPLE-1" |
| comparer | `=` et `#` seulement | `IF N$="WOZ" THEN …` |

Affecter une chaîne plus longue que son `DIM` donne `*** STR OVFL ERR`, de même qu’une réponse trop longue à `INPUT`. Il n’y a ni `MID$`, ni `LEFT$`, ni `CHR$`, ni `STR$`, ni `VAL`, ni `ASC` (ce dernier est dans la table mais ne marche pas).

!!! astuce "Remplacer CHR$ et VAL"
    Pour afficher un caractère par son code, gardez une chaîne de référence et découpez-la : `DIM C$(26):C$="ABCDEFGHIJKLMNOPQRSTUVWXYZ"` puis `PRINT C$(K,K)`. Pour lire un nombre, utilisez `INPUT N` plutôt que `INPUT N$`.

## Les tableaux

`DIM T(10)` crée un tableau de 10 entiers, **numérotés de 1 à 10**. `T(0)` et `T(11)` donnent `*** RANGE ERR`. Il n’y a qu’une dimension : pour une grille de 8 × 8, utilisez `G((L-1)*8+C)`. Dans l’émulateur, les éléments valent 0 après le `DIM` ; sur une vraie machine, la mémoire n’est pas forcément effacée : initialisez vos tableaux. Un tableau trop grand pour la mémoire donne `*** MEM FULL ERR` (`DIM A(3000)` en 8 Ko).

## La mémoire du BASIC

![La mémoire de l’Apple-1 : BASIC en $E000, programme sous HIMEM, variables au-dessus de LOMEM.](diagrams/memoire.svg "diagram large")

Le BASIC range le **programme en haut** de la mémoire, en descendant depuis HIMEM, et les **variables en bas**, en montant depuis LOMEM. L’espace libre est entre les deux.

| Pointeur | Adresse | Valeur en 8 Ko | Rôle |
|---|---|---|---|
| LOMEM | `$4A-$4B` (74-75) | `$0800` (2048) | début des variables |
| HIMEM | `$4C-$4D` (76-77) | `$1000` (4096) | fin de la mémoire du BASIC |
| PP | `$CA-$CB` (202-203) | baisse avec le programme | début du programme |
| PV | `$CC-$CD` (204-205) | monte avec les variables | fin des variables |

{{!LOMEM}}{{!HIMEM}}{{!PP (pointeur de programme)}}

`PRINT PEEK(202)+PEEK(203)*256` donne le début du programme et `PRINT PEEK(204)+PEEK(205)*256` la fin des variables : la place libre est la différence des deux. Une variable entière comme `A` occupe 6 octets.

!!! apple1 "Pourquoi une cassette BASIC commence en $004A"
    Ces pointeurs sont en page zéro, de `$4A` à `$FF`. Une cassette BASIC enregistre donc **deux plages** : `004A.00FF` (les pointeurs), puis le programme de PP à HIMEM, par exemple `0F26.0FFF`. La ligne de commande `apple1-emu basic` les calcule pour vous :

    ```text
    10 lignes, 218 octets ; LOMEM $0800 HIMEM $1000 PP $0F26 ; 1830 octets libres pour les variables ; RAM 8 Ko
    Plages ACI (après C100R) : 004A.00FFR 0F26.0FFFR   puis E2B3R et RUN
    ```

### Changer LOMEM et HIMEM

La syntaxe est `LOMEM=n` et `HIMEM=n` (avec `=`, **pas** `:` comme sur l’Apple II, qui donne `SYNTAX ERR`).

- `LOMEM=2304` déplace le début des variables et **efface les variables** (le programme reste). Utile pour réserver `$0800-$08FF` à un programme en langage machine.
- `HIMEM=` **plus haut** (avec plus de RAM, par exemple `HIMEM=8192` en 16 Ko) fonctionne et garde le programme.
- `HIMEM=` **plus bas** est **ignoré sans message** par l’original, même après `SCR` (chapitre 18, bug 11). **Le RDK G2 Basic l’accepte** : le programme en mémoire descend avec la limite, ou `MEM FULL ERR` s’il ne tenait plus au-dessus des variables (`RANGE ERR` pour une valeur négative ; `LOMEM=` donne `RANGE ERR` s’il n’est pas sous le début du programme, et `>32767 ERR` au-dessus de 32767). Avec l’original, il existe un contournement vérifié : écrire le pointeur directement, puis `SCR` :

```basic title="Abaisser HIMEM à 3072 ($0C00)"
POKE 76,0 : POKE 77,12 : SCR
```

Au démarrage du G2 Basic, HIMEM est le haut de la RAM trouvée et LOMEM vaut `$0800` (`$2000` avec un pilote GEN2). (`76`/`77` = `$4C`/`$4D`, 12 × 256 = 3072.) Tapez ces trois instructions sur une ligne numérotée ou séparément en mode direct, **avant** d’entrer le programme : `SCR` l’efface.

## Mélanger BASIC et langage machine

| Zone (8 Ko) | Utilisable pour du 6502 ? |
|---|---|
| `$0000-$00FF` | page zéro : le Woz Monitor utilise `$24-$2B`, le BASIC au moins `$4A-$FF` ; à éviter, ou à sauver et restaurer |
| `$0100-$01FF` | pile du 6502 : non |
| `$0200-$027F` | tampon de ligne du Woz Monitor ; écrasé à chaque ligne tapée |
| `$0280-$07FF` | libre tant que le BASIC n’y touche pas : le meilleur endroit |
| `$0800-…` | variables du BASIC (sauf si LOMEM est relevé) |
| `… -$0FFF` | programme BASIC |

Le programme machine se charge par cassette ou par POKE, et s’appelle par `CALL 768` (pour `$0300`). Il doit finir par `RTS` et préserver la page zéro du BASIC. Exemple minimal :

```mon
>» POKE 768,96
>» CALL 768
>
```

(96 = `$60` = `RTS` : l’appel revient tout de suite.)
