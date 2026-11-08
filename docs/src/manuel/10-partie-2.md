# L’Integer BASIC <span class="part-sub">Le BASIC de Steve Wozniak et son clone libre RDK G2 Basic : toutes les instructions, la mémoire, les messages d’erreur, les bugs connus et leurs contournements, et le portage depuis l’Applesoft</span> {: .part data-part="Partie II" }

# Premiers pas en BASIC

<p class="lead">L’Integer BASIC de l’Apple-1 tient en 4 Ko. Il ne connaît que les entiers, des noms de variables d’une lettre, et pas de DATA ni de fonctions de chaînes : il est petit, rapide, et a ses pièges.</p>

!!! apple1 "D’où vient ce BASIC"
    Steve Wozniak l’a écrit en 1976, à la main, en assembleur, sans ordinateur pour l’assembler. Il était vendu sur cassette (75 $) et se charge en `$E000-$EFFF`. L’émulateur ne fournit pas l’original d’Apple : utilisez votre propre `BASIC.wav` ou image de 4 Ko (chapitre 4) si vous le voulez. Tous les essais de cette partie ont été faits dans l’émulateur, avec cet original.

## Deux BASIC : le RDK G2 Basic et l’original

Depuis la version 7.9, l’émulateur a son propre BASIC, le **RDK G2 Basic**, intégré au programme. C’est un clone « salle blanche » de l’Integer BASIC, écrit de zéro en assembleur 6502 (il ne contient aucun code de l’interpréteur d’Apple) et distribué sous licence MIT. Il est dédié à Uncle Bernie.

- **Compatible** : même syntaxe, mêmes jetons, même format de programme en mémoire, même format de cassette. Les programmes, listings et cassettes écrits pour l’original tournent sans changement. Il a été comparé à l’original avec 22 séances d’essai dont les sorties sont identiques.
- **Corrigé** : les bugs connus de l’original, décrits au chapitre 18, sont corrigés.
- **Plus gros** : 7 933 octets au lieu de 4 Ko. Il a besoin de RAM de `$E000` à `$FEFF` ; l’émulateur la fournit, mais il ne tiendrait pas dans la fenêtre d’EPROM de 4 Ko de l’original.
- **Conscient de la GEN2** : au démarrage il cherche la carte GEN2 et, s’il la trouve, lance le pilote de la carte (puis place LOMEM en `$2000`, au-dessus des pages graphiques). Les instructions graphiques elles-mêmes sont prévues pour la version 2 du G2 Basic.
- **Bandeau** : `RDK G2 BASIC 1.0`, puis `DEDICATED TO UNCLE BERNIE`.
- **Étendu** (version 7.9.2) : `LOMEM=` et `HIMEM=` fonctionnent comme dans l’original, et une nouvelle fonction **`KEY(n)`** lit le clavier sans attendre, pour qu’un jeu puisse regarder les touches sans que `INPUT` le bloque et sans qu’une touche arrête le programme (chapitre 15). `LIST a,b` et `DEL a,b` utilisent les plages de l’original, et une ligne listée plus longue que l’écran passe à la ligne comme dans l’original.

Le bouton **BASIC** charge le G2 Basic. Pour utiliser l’original d’Apple à la place, ouvrez **⚙ Préférences → Avancé → BASIC**, choisissez **Integer BASIC d’origine (votre propre fichier, chargé en $E000)** et désignez le fichier ; le choix est retenu, et le bouton charge alors celui-là. Les deux se comportent de la même façon pour tout ce qui est décrit dans cette partie ; les différences sont signalées là où elles comptent (HIMEM au démarrage, les bugs du chapitre 18).

!!! note "Source et essais"
    Le G2 Basic a son propre dépôt, `Old-Wirehead/rdk-gen2basic-apple1`, avec son manuel, son source et ses tests de non-régression.

## Démarrer le BASIC

Cliquez sur **BASIC** dans la barre d’outils : l’émulateur charge le BASIC que vous avez choisi (le G2 Basic par défaut) et vous le démarrez avec `E000R`. L’invite du BASIC est `>`, celle du Woz Monitor `\`.

```mon
\
» E000R
E000: 4C
>
```

- `E000R` : **démarrage à froid**, efface le programme en mémoire.
- `E2B3R` : **démarrage à chaud**, garde le programme. C’est l’adresse à utiliser après le chargement d’un programme par cassette, et pour revenir au BASIC après un passage par le moniteur (RESET).

{{E000R}}{{E2B3R}}

## Mode direct et mode programme

Une ligne qui commence par un **numéro** (0 à 32767) est rangée dans le programme ; une ligne sans numéro est exécutée tout de suite.

```mon
>» PRINT 2*3+4*5
26
>» 10 PRINT "BONJOUR"
>» 20 GOTO 10
>» RUN
BONJOUR
BONJOUR
…
```

Appuyez sur une touche pour arrêter le programme : le BASIC affiche `STOPPED AT 10`. Retaper une ligne avec le même numéro la remplace ; un numéro seul l’efface.

!!! attention "Le mode direct est limité"
    En mode direct, le BASIC refuse les instructions séparées par `:` (`SYNTAX ERR`), ainsi que `FOR` et `GOSUB`. Pour essayer une boucle, écrivez-la dans une ligne numérotée.

## Corriger une ligne

Le clavier de l’Apple-1 n’a pas de touche d’effacement : le **retour arrière** de l’émulateur tape `_`, qui annule le caractère précédent (la ligne affichée n’est pas corrigée, mais le BASIC reçoit la bonne). <kbd>Échap</kbd> annule toute la ligne : le BASIC affiche `\` et attend la ligne suivante. Pour une correction plus confortable, utilisez l’éditeur (chapitre 7) : il colore la syntaxe, signale les erreurs avant l’envoi, et tape le programme pour vous.

## Un premier programme

@@include exemples/basic/boucle.bas

```mon
>» RUN
SOMME = 55
>
```

Dans l’éditeur, **▶ Lancer** tape le programme et `RUN` ; dans le débogueur, vous pouvez le suivre ligne par ligne (figure du chapitre 8).

## Les commandes

| Commande | Effet |
|---|---|
| {{LIST}} `[n[,m]]` | liste le programme, la ligne `n`, ou les lignes `n` à `m` |
| {{RUN}} `[n]` | efface les variables et lance le programme, depuis la ligne `n` le cas échéant |
| {{DEL}} `n[,m]` | efface la ligne `n`, ou les lignes `n` à `m` |
| {{SCR}} | efface le programme et les variables |
| {{CLR}} | efface les variables, garde le programme |
| {{AUTO}} `n[,pas]` | numérotation automatique à partir de `n` (pas de 10 par défaut) ; <kbd>Ctrl</kbd>+<kbd>D</kbd> puis <kbd>RETURN</kbd> en sort, <kbd>Échap</kbd> n’en sort pas |
| {{MAN}} | revient à la numérotation manuelle |
| {{HIMEM}}`=n`, {{LOMEM}}`=n` | les bornes de la mémoire du BASIC (chapitre 16) |

`GOTO n` en mode direct reprend un programme arrêté **sans** effacer les variables, à la différence de `RUN n`.

!!! astuce "LIST et l’affichage lent"
    Le terminal de l’Apple-1 affiche environ 60 caractères par seconde : lister un long programme prend du temps. Désactivez la limite d’affichage (**Rapide** dans la rangée d’affichage, ou dans la barre d’état) pour lire plus vite, ou listez par tranches : `LIST 100,200`.

## Trois programmes à taper

### Devine un nombre

@@include exemples/basic/devine.bas

```mon
>» RUN
J'AI CHOISI UN NOMBRE DE 1 A 100.
TON ESSAI?» 50
PLUS GRAND !
TON ESSAI?» 75
PLUS PETIT !
TON ESSAI?» 62
PLUS PETIT !
TON ESSAI?» 56
PLUS GRAND !
TON ESSAI?» 57
BRAVO, EN 5 COUPS.
```

`RND(100)` donne un entier de 0 à 99 ; `#` veut dire « différent de ».

### Une table de multiplication

@@include exemples/basic/table.bas

### Jouer avec les chaînes

@@include exemples/basic/chaines.bas

```mon
>» RUN
TON NOM?» KAYAK
INITIALE : K
FIN : AK
A L'ENVERS : KAYAK
C'EST UN PALINDROME !
```

Les chaînes se découpent avec `N$(début,fin)` et s’allongent avec `R$(LEN(R$)+1)=…` : il n’y a ni `MID$`, ni `+` pour les chaînes (chapitre 16).

Les quatre programmes sont dans `docs/exemples/basic/` : ouvrez-les dans l’éditeur, ou faites-en une cassette avec `apple1-emu basic devine.bas -o devine.aiff`.
