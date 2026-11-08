# Votre premier programme BASIC

L’Integer BASIC, le BASIC de Steve Wozniak, tient dans 4 Ko, et le **RDK G2 Basic** de l’émulateur, qui fait tourner les mêmes programmes, est intégré. Il ne connaît que les nombres entiers de −32767 à 32767, mais il est rapide et agréable. L’éditeur intégré vous évite de taper le programme ligne par ligne sur l’écran de l’Apple-1, où une faute de frappe ne se corrige qu’en retapant la ligne entière.

## Écrire le programme

Cliquez sur **📝 Éditeur** dans la barre d’outils, puis sur l’icône **Nouveau** (la première) de l’éditeur et choisissez « Programme BASIC », et tapez ce programme. Il se trouve aussi dans `docs/exemples/basic/premier.bas` dans l’archive.

```basic title="premier.bas"
10 REM MON PREMIER PROGRAMME
20 PRINT "BONJOUR, APPLE-1 !"
30 PRINT
40 FOR I=1 TO 8
50 PRINT I;" AU CARRE = ";I*I
60 NEXT I
70 PRINT
80 DIM N$(20)
90 INPUT "VOTRE PRENOM",N$
100 PRINT "ENCHANTE, ";N$;" !"
110 END
```

Les mots-clés, les nombres, les chaînes et les commentaires prennent chacun leur couleur (réglable dans **⚙ Préférences → Éditeur**). Sous le texte, l’éditeur indique en permanence la taille du programme et la mémoire qui reste pour les variables.

!!! astuce "Clic droit dans le texte"
    Le menu de l’éditeur réunit couper, copier et coller, **Rechercher** (Ctrl+F) et **Remplacer** (Ctrl+H), aller à une ligne (Ctrl+G), REM / dé-REM d’un bloc, **Renuméroter…** (`GOTO`, `GOSUB` et `THEN` suivent), un point d’arrêt sur la ligne (F9) et **Envoyer et lancer** (F5).

## Le lancer

Cliquez sur **▶ Lancer**. Le programme est mis directement en mémoire (le BASIC est chargé d’abord s’il le faut), puis l’éditeur tape pour vous `E2B3R` et `RUN` :

![L’éditeur à gauche, le programme en cours d’exécution à droite.](img/brut/editeur-premier.png "large")

Le programme attend votre prénom. Cliquez sur l’écran de l’Apple-1, tapez-le, puis Entrée :

```mon
VOTRE PRENOM?WOZ
ENCHANTE, WOZ !
```

!!! attention "Trois particularités de l’Integer BASIC"
    - **Les chaînes se dimensionnent** : sans `DIM N$(20)`, l’`INPUT` d’une chaîne s’arrête sur `*** STR OVFL ERR`.
    - **Les noms de variables** font une lettre, éventuellement suivie d’un chiffre : `A`, `B2`, `N$`. `TOTAL` provoque une erreur de syntaxe.
    - **Le `END` final est obligatoire** : sans lui, le programme se termine par `*** END ERR`.

## Quand il y a une erreur

L’éditeur vérifie chaque ligne pendant que vous tapez, avec les règles du vrai BASIC. Une ligne fautive passe en rouge et le message explique le problème, avec le numéro de la ligne et la colonne :

![Deux erreurs signalées : une chaîne non fermée et un nom de variable trop long.](img/editeur-erreur.png "medium")

Un programme qui contient des erreurs ne se lance pas : corrigez d’abord les lignes rouges. Les erreurs qui ne se voient qu’à l’exécution s’affichent sur l’écran de l’Apple-1 : `*** BAD BRANCH ERR` pour un `GOTO` vers une ligne absente, `*** >32767 ERR` pour un dépassement ou une division par zéro. Le bouton **🐛 Débogueur** permet alors d’avancer ligne par ligne en surveillant les variables. Le Manuel complet le décrit en détail.

![Le cycle de travail : écrire, lancer, corriger, puis exporter vers la vraie machine.](diagrams/workflow-basic.svg "diagram")

!!! astuce "Arrêter un programme"
    Pendant un `RUN`, une touche pressée arrête le programme sur `STOPPED AT 50` (le numéro de la ligne en cours). Si le programme est bloqué dans une boucle, **Reset** (Ctrl+R) ramène au moniteur, et `E2B3R` revient au BASIC **sans effacer** le programme.
