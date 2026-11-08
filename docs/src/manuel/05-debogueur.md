# Le débogueur, le désassemblage et le dump

<p class="lead">Le débogueur arrête l’Apple-1 et l’exécute pas à pas, instruction par instruction en 6502 ou ligne par ligne en BASIC, en montrant les registres, les variables et la mémoire.</p>

## Le panneau du débogueur

**🐛 Débogueur** ouvre le panneau, ancré à droite ou détachable comme l’éditeur. Ses commandes :

| Commande | Effet |
|---|---|
| **⏸ Pause** / **▶ Continuer** | arrête ou relance l’Apple-1 |
| **⬇ Pas** | exécute une instruction |
| **↪ Par-dessus** | exécute un `JSR` d’un seul coup, et s’arrête au retour |
| **⤴ Sortir** | continue jusqu’au `RTS` du sous-programme en cours |
| **⏭ Ligne BASIC** | avance d’une ligne du programme BASIC |
| **⏩ Jusqu’au curseur** | (dans l’éditeur) continue jusqu’à la ligne du curseur |

{{!Pas (débogueur)}}{{!Par-dessus}}{{!Sortir}}

Le cadre **Processeur** montre A, X, Y, S, PC et les drapeaux N V B D I Z C ; tous sont modifiables, en cliquant sur la valeur. Le nombre de cycles et d’instructions exécutés est affiché dessous.

## Les points d’arrêt

Un point d’arrêt se pose de quatre façons :

- **⏺ Point d’arrêt** dans l’éditeur (ou <kbd>F9</kbd>, ou le menu du clic droit), sur la ligne du curseur : dans une source `.s` (grâce à la table des adresses de l’assembleur) ou dans un programme BASIC (numéro de ligne) ;
- un clic dans la marge du **📃 Listing** ;
- un clic sur une ligne du désassemblage ;
- la saisie d’une adresse ou d’une étiquette : `$0368`, `ASK`, `PUTS+3`, ou `L190` pour la ligne BASIC 190. Une étiquette qui n’existe pas encore n’est pas refusée : le point d’arrêt est gardé et posé dès que l’étiquette existe (quand vous assemblez la source de l’éditeur, ou la lancez), avec un message qui le dit.

Les points d’arrêt, les variables suivies et la mémoire surveillée sont retenus d’une session à l’autre, mais charger un autre programme efface les points d’arrêt et les étiquettes du précédent. Quand le BASIC intégré est chargé, la section BASIC du débogueur s’intitule **RDK G2 Basic** ; sans BASIC en `$E000`, elle le dit sans insister. Les couleurs de la ligne courante, des adresses du dump et de la colonne ASCII se changent dans **⚙ Préférences → Interface** (« Couleurs de l’interface »). Les panneaux étroits suppriment la colonne des octets du désassemblage et font passer à la ligne la rangée de poke au lieu de la couper.

## Déboguer un programme BASIC

![boucle.bas arrêté sur un point d’arrêt en ligne 30 : la ligne courante en vert dans l’éditeur, les variables I et S suivies dans le débogueur.](img/brut/debogueur-basic.png "large")

La ligne courante est surlignée en vert dans l’éditeur, les points d’arrêt en violet. Le cadre BASIC (**RDK G2 Basic** ou **Integer BASIC**, selon celui qui est chargé) indique la ligne courante, les arrêts sur numéros de ligne et les **variables suivies** : entières (`A`, `B1`), éléments de tableau (`D(2)`), tableau entier (`D`), chaînes (`C$`), plus **Toutes les variables**.

Le débogage marche avec le **RDK G2 Basic**, le BASIC intégré à l’émulateur, comme avec l’original : le vrai Integer BASIC n’est plus nécessaire. Le débogueur remarque le début de chaque ligne (ligne courante, points d’arrêt sur ligne, **⏭ Ligne BASIC**) et lit les variables dans les tables de l’interpréteur. Pour l’original il lit la ligne dans les pointeurs de page zéro `$DC`/`$DD` ; le G2 Basic la garde ailleurs, et le débogueur connaît les deux organisations.

!!! attention "Une touche arrête le programme"
    Comme sur la vraie machine, une touche frappée pendant que le programme BASIC tourne l’arrête (`STOPPED AT`). Pendant une séance de débogage, gardez le clavier dans le panneau du débogueur, pas sur l’écran de l’Apple-1. Un programme G2 Basic peut lire le clavier sans être arrêté avec `KEY(0)` (chapitre 15).

## Déboguer un programme 6502

![decimal.s arrêté sur l’étiquette prdec : registres, désassemblage avec les étiquettes du programme, mémoire surveillée $06.$08.](img/brut/debogueur-asm.png "large")

Le **désassemblage** montre 24 lignes autour de PC, avec les étiquettes du programme assemblé dans leur propre colonne. Il ne se recentre que quand PC sort de la zone visible, pour que l’affichage ne saute pas à chaque pas. **Aller** fixe la liste sur une adresse ou une étiquette ; **Suivre PC** la fait de nouveau suivre l’exécution.

Quand vous sélectionnez une instruction dans la source 6502 de l’éditeur, ses octets sont aussi **surlignés** dans le désassemblage et dans le dump (chapitre 7), et les deux panneaux défilent jusqu’à eux s’ils sont hors de vue ; le désassemblage cesse alors de suivre PC jusqu’à ce que vous appuyiez sur **Suivre PC**.

La **mémoire surveillée** affiche un octet, un mot ou de l’ASCII (`$30`, `$30 w`, `0200.020F`, une étiquette), modifiable, en jaune quand la valeur change.

## Le dump

Le **Dump** affiche la mémoire en hexadécimal et en ASCII, 16 octets par ligne (8 si le panneau est étroit), le bit 7 enlevé pour l’ASCII. **Aller** saute à une adresse, ou aux raccourcis ZP, `$0200`, `$0300`, PC, `$E000`. Les octets changés sont en jaune, l’octet de PC sur fond vert. **Écrire** pose des octets (`0300 = A9 00`) ; un clic sur un octet remplit la ligne d’écriture.

Le désassemblage et le dump sont des panneaux à part entière : détachés par défaut, ils peuvent être ancrés près du débogueur (chapitre 10).

## En ligne de commande

Pour les démonstrations et les essais automatiques : `--debogueur` ouvre le panneau, `--dbg-arret ASK,L190` pose des points d’arrêt, `--dbg-surveiller 'A,C$,$30 w'` suit des variables et de la mémoire, et `--dbg-commande 5:pas` envoie une commande après 5 secondes (`pause`, `continuer`, `pas`, `dessus`, `sortir`, `basic`).
