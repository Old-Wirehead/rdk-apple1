---
version: 1.0
date: Novembre 2026
---

# L’émulateur <span class="part-sub">La fenêtre, l’écran, la bibliothèque, la cassette ACI, les cartes graphiques GEN2 et P-LAB, le débogueur et la ligne de commande</span> {: .part data-part="Partie I" }

# À propos de ce manuel

<p class="lead"><strong>Retro Dev Kit Apple 1</strong> — «&#8239;Le péché originel&#8239;» — reproduit l’Apple-1 de 1976 au cycle près : le 6502, le terminal vidéo, le clavier, la carte cassette ACI et, en option, la carte couleur GEN2 d’Uncle Bernie et la carte graphique P-LAB à TMS9918A. Il est livré avec son propre BASIC libre, le <strong>RDK G2 Basic</strong>. Autour de la machine, il ajoute un atelier complet : bibliothèque de programmes, éditeur, assembleur, débogueur et export vers un vrai Apple-1.</p>

Ce manuel décrit tout l’émulateur, version 1.0 (la première version publique ; les numéros 7.x désignaient les versions de test). Il complète le *Guide de démarrage rapide*, qui suffit pour installer le programme et lancer une première cassette. Ici, chaque panneau, chaque bouton et chaque commande sont expliqués, avec des exemples que vous pouvez taper tels quels.

## Comment ce manuel est organisé

<div class="cards">
<div class="card"><h4>Partie I — L’émulateur</h4>La fenêtre et ses panneaux, l’écran vintage, la bibliothèque, la cassette et l’ACI, les cartes graphiques GEN2 et P-LAB, l’éditeur, le débogueur, l’art ASCII, les préférences, la ligne de commande et le dépannage.</div>
<div class="card"><h4>Partie II — L’Integer BASIC</h4>Le BASIC de Steve Wozniak et son clone libre RDK G2 Basic : toutes les instructions, les fonctions et les opérateurs, la mémoire, les messages d’erreur et les bugs connus, avec leurs contournements.</div>
<div class="card"><h4>Partie III — L’assembleur 6502</h4>Le 6502 depuis le début, l’assembleur intégré et ses directives, les routines du Woz Monitor, des exemples complets et les pièges à éviter.</div>
<div class="card"><h4>Annexes</h4>Carte mémoire, commandes du Woz Monitor, format de l’ACI, jeu de caractères du 2513, raccourcis clavier, puis l’index des commandes.</div>
</div>

## Conventions

Les touches sont écrites ainsi : <kbd>Ctrl</kbd>+<kbd>R</kbd>, <kbd>RETURN</kbd>. Les boutons de l’interface sont en gras : **▶ Lancer**. Les adresses sont en hexadécimal, précédées de `$` comme dans les documents d’époque : `$FFEF`. Le Woz Monitor, lui, les écrit sans `$` : `FFEF`.

Les sessions sur l’écran de l’Apple-1 ont un fond sombre. Ce que **vous** tapez est en blanc, ce que la machine répond est en vert :

```mon
» 300R

0300: A2
BONJOUR, APPLE-1 !
```

Quatre sortes d’encadrés jalonnent le texte :

!!! astuce "Astuce"
    Un raccourci, une meilleure façon de faire.

!!! attention "Attention"
    Un piège qui fait perdre du temps ou des données.

!!! apple1 "Sur la vraie machine"
    Ce qui se passe sur un vrai Apple-1, et ce qui change par rapport à l’émulateur.

!!! bug "Bug connu"
    Un défaut de la ROM d’origine, avec son contournement. Chaque bug porte un statut : <span class="verified">vérifié</span> (reproduit dans l’émulateur avec le vrai Integer BASIC) ou <span class="unverified">non vérifié</span> (cité d’après une source, sans essai).

Tous les exemples de ce manuel ont été essayés dans l’émulateur. Les programmes 6502 de la partie III sont dans le dossier `docs/exemples/asm/` de la distribution. Ils donnent exactement les mêmes octets avec l’assembleur intégré et avec ca65.

## Ce qu’il vous faut

L’émulateur fonctionne sous Linux sans rien installer. Le Woz Monitor et la ROM de l’ACI sont intégrés, ainsi que le **RDK G2 Basic**, un BASIC libre compatible avec l’Integer BASIC (chapitre 14) : le bouton **BASIC** marche tout de suite, sur une installation neuve, sans fichier à chercher. L’**Integer BASIC** d’Apple (1976) n’est **pas** fourni, parce qu’il reste protégé par le droit d’auteur. Si vous voulez l’original, utilisez votre propre copie (par exemple la cassette `BASIC.wav` de la collection « Apple1Cassettes » d’archive.org) et choisissez-la dans **⚙ Préférences → Avancé → BASIC**.

!!! attention "Les ROM et les cassettes restent chez vous"
    L’émulateur ne livre ni l’Integer BASIC d’Apple ni les cassettes du commerce. Sa table de reconnaissance ne contient que des tailles, des empreintes et des paramètres. Gardez vos copies dans votre bibliothèque personnelle.

!!! note "Licence"
    Retro Dev Kit Apple 1 est un logiciel libre sous **GNU GPL 3.0** (sans garantie). Certaines parties gardent leur propre licence MIT : le RDK G2 Basic, ainsi que les tests et les glyphes GEN2 d’Uncle Bernie. Voir `LICENSE` et `NOTICE` dans l’archive, et l’annexe « Licence et crédits ».

## Les grands chemins

Quatre manières d’utiliser l’émulateur reviennent sans cesse. Elles sont détaillées plus loin, chacune avec son schéma :

| Je veux… | Le chemin | Voir |
|---|---|---|
| jouer à un programme d’époque | bibliothèque → **▶** | chapitre 4 |
| écrire un programme BASIC | **📝 Éditeur** → **▶ Lancer** → **📼 Exporter WAV** | partie II |
| écrire un programme 6502 | **📝 Éditeur** (`.s`) → **📃 Listing** → **🐛 Débogueur** | partie III |
| envoyer un programme sur un vrai Apple-1 | **📼 Exporter WAV** (`.aiff`) → smartphone → ACI | chapitre 5 |

# La fenêtre et la barre d’outils

## Vue d’ensemble

![La disposition par défaut : tous les panneaux ouverts et ancrés.](img/defaut-annote.png "large")

<ol class="legend">
<li><strong>Charger, Sauver, État</strong> : vidages hexadécimaux <code>.txt</code>, binaires <code>.bin</code> et <code>.prg</code> ; Sauver écrit aussi des cassettes <code>.wav</code>/<code>.aiff</code> ; État enregistre et restaure la machine.</li>
<li><strong>RESET, Effacer l’écran, Vitesse (jauge), Coller</strong> : les boutons de la machine (Ctrl+R, Ctrl+L, F3, Ctrl+V).</li>
<li><strong>BASIC</strong> : charge le RDK G2 Basic intégré en <code>$E000</code> (ou votre Integer BASIC, si vous l’avez choisi).</li>
<li><strong>Fenêtres</strong> : le menu <strong>⊞ Disposition ▾</strong>, puis un bouton par panneau (Écran monochrome, Écran couleur, Éditeur, Débogueur, Désassemblage, Dump, Bibliothèque, ASCII).</li>
<li><strong>Cartes graphiques</strong> : <strong>GEN2</strong> et <strong>P-LAB</strong> branchent ou débranchent les cartes.</li>
<li><strong>Cassette</strong> : <strong>📼 Insérer</strong> une cassette, puis lecture, stop, rembobiner, enregistrer, éjecter ; oscilloscope et son.</li>
<li><strong>EPROM</strong>, <strong>⚙ Préférences</strong>, <strong>?</strong> (bulles d’aide détaillées, aussi F1) et <strong>ℹ</strong> (à propos, raccourcis).</li>
<li><strong>Rangée d’affichage de l’écran</strong> : zoom (1x à 8x ou Ajuster), phosphore, affichage <strong>Normal</strong> ou <strong>Rapide</strong>, écran vintage, plein écran, <strong>Détacher</strong>.</li>
<li><strong>Éditeur</strong> : programmes BASIC et sources 6502 en onglets.</li>
<li><strong>Écran Apple-1</strong> : 40 × 24 caractères ; cliquez dessus pour taper, clic droit pour Copier, Coller, Effacer l’écran et Reset.</li>
<li><strong>Débogueur</strong> : pause, pas à pas, registres, points d’arrêt, variables.</li>
<li><strong>Désassemblage</strong> : le code autour du compteur ordinal (PC).</li>
<li><strong>Dump</strong> : la mémoire en hexadécimal et en ASCII, modifiable.</li>
<li><strong>Bibliothèque</strong> : vos programmes, chacun avec ses petites icônes ▶ ⬇ 📼 ⚙.</li>
<li><strong>Barre d’état</strong> : vitesse, débit d’affichage, cassette, destination du clavier ; dessous, le dernier message.</li>
</ol>

Tous les panneaux (écran, éditeur, débogueur, bibliothèque…) sont des onglets que vous pouvez déplacer, regrouper ou détacher dans leur propre fenêtre : c’est l’objet du chapitre 10. Au premier lancement, la fenêtre s’ouvre agrandie et une fenêtre **Votre espace de travail** propose cinq dispositions (chapitre 10) ; la taille et la position de la fenêtre et l’arrangement de ses panneaux sont retenus, et retrouvés tels quels au lancement suivant.

## La barre d’outils, bouton par bouton

La barre d’outils tient sur une seule rangée : la machine et ses outils à gauche, le lecteur de cassette au milieu, les panneaux (groupe Fenêtres), les cartes et les réglages à droite. Quand la fenêtre est trop étroite, les boutons ne gardent que leur icône (le texte reste dans l’infobulle), ou la barre passe sur plusieurs lignes si vous décochez **Barre compacte** dans les préférences.

![La barre d’outils jusqu’au groupe Cartes graphiques : fichiers, machine, fenêtres et cartes. Suit la zone Cassette (chapitre 5), puis les réglages.](img/barre-1.png "large")

| Bouton | Ce qu’il fait |
|---|---|
| {{Charger}} | Ouvre un programme et le met en mémoire : un `.txt` (vidage du Woz Monitor) ou un `.hex` ; pour un `.bin`, l’adresse est demandée. Les cassettes (`.wav`, `.aiff`) ne passent pas par ici : elles s’insèrent avec **📼 Insérer**, dans la zone Cassette. |
| {{Sauver}} | Demande une ou plusieurs plages (`0300.03FF`, `004A.00FF 0300.0FFF`…), puis un fichier. L’extension choisit le format : `.wav`, `.aiff`, `.txt` ou `.bin`. |
| **RESET** (bouton rouge) | Le bouton RESET de l’Apple-1 (<kbd>Ctrl</kbd>+<kbd>R</kbd>). Le 6502 et le PIA repartent, la mémoire et l’écran restent. |
| **🆑 Effacer l’écran** | Le bouton CLEAR SCREEN (<kbd>Ctrl</kbd>+<kbd>L</kbd>). |
| **Jauge (Vitesse réelle / Vitesse rapide)** | Bascule entre 1 MHz d’époque et la vitesse maximale (<kbd>F3</kbd>). Un cadran de compteur : allumé en bleu quand la vitesse rapide est active. |
| **📋 Coller** | Tape le contenu du presse-papiers au clavier de l’Apple-1, à la cadence d’un clavier (<kbd>Ctrl</kbd>+<kbd>V</kbd>). Idéal pour un vidage hexadécimal. |
| {{BASIC|**BASIC**}} | Charge le **RDK G2 Basic** intégré en `$E000` (7 933 octets, puis `E000R` le démarre). Si vous avez choisi l’Integer BASIC d’origine dans **⚙ Préférences → Avancé → BASIC**, c’est lui qui est chargé : une cassette, un vidage `.txt` ou un `.bin` de 4096 octets, cherché dans la bibliothèque (`BASIC.wav`, `integer_basic.bin`, tout nom qui contient « basic »), sinon demandé. Clic droit : choisir un autre fichier. |
| **Fenêtres** (groupe : **⊞ Disposition ▾** et un bouton par panneau) | Montre ou cache chaque panneau : **Écran monochrome** (le terminal de l’Apple-1), **Écran couleur** (l’écran de la carte graphique branchée, GEN2 ou P-LAB : le même bouton sert pour les deux ; sans carte branchée, il ouvre un choix pour en brancher une, et **P-LAB** affiche alors son écran), **📝 Éditeur** (chapitre 7), **🐛 Débogueur**, **Désassemblage**, **Dump** (chapitre 8), **📚 Bibliothèque** et **🎨 ASCII** (chapitre 9). Un bouton allumé signifie que le panneau est visible ; un panneau déjà affiché peut aussi se fermer avec la croix de son onglet. |
| **Cartes graphiques : GEN2 et P-LAB** | Un groupe nommé. **GEN2** branche ou débranche la carte couleur d’Uncle Bernie (chapitre 6) ; **P-LAB** fait de même pour la carte P-LAB à TMS9918A (section 6.6). Un bouton est allumé tant que sa carte est branchée ; l’écran d’une carte apparaît dans son propre panneau, montré par le bouton **Écran couleur**. |
| **📸 État ▾** | Un seul bouton, avec un menu : **Enregistrer l’état…** (<kbd>Ctrl</kbd>+<kbd>Maj</kbd>+<kbd>S</kbd>), **Charger un état…** (<kbd>Ctrl</kbd>+<kbd>Maj</kbd>+<kbd>O</kbd>), puis les derniers états, chacun avec son titre et sa date (section 2.5). |
| **EPROM** (puce à fenêtre UV, à droite de la barre, avant **⊞ Disposition**) | Ouvre le programmateur d’EPROM : lire, écrire ou vérifier une puce avec un programmateur `minipro` (section 2.6). |
| **Cassette : 📼 Insérer ▶ ⏹ ⏪ ⏺ ⏏** | Le lecteur, réuni dans une zone. **📼 Insérer** choisit un `.wav` ou un `.aiff` et le met dans le lecteur comme cassette virtuelle ; puis lecture, stop, rembobiner, enregistrer, éjecter. La zone montre le nom, la position et la durée. |
| **📈** et **🔊** | L’oscilloscope de la cassette, et le son du signal (chapitre 5). |
| **⊞ Disposition ▾** (premier bouton du groupe Fenêtres) | Un menu : **Par défaut** (tous les panneaux, comme au premier lancement), **Jeu** (écran Apple-1 et bibliothèque), **BASIC** (éditeur et écran), **Assembleur** (éditeur, écran et débogueur), **Débogage** (éditeur, débogueur, désassemblage et dump) ; puis **Enregistrer la disposition actuelle…**, qui demande un nom, et vos dispositions enregistrées par ordre alphabétique (clic droit : Renommer, Supprimer). Une coche marque le dernier choix ; tout est retenu dans `gui.json`. |
| **⚙** | Les préférences (chapitre 11). |
| **?** | Le mode aide (<kbd>F1</kbd>) : des bulles détaillées sur chaque commande, dans les 7 langues de l’interface. |
| **ℹ** | « À propos », avec la liste des raccourcis clavier. |

**Fenêtres de fichiers.** Ouvrir, Enregistrer sous et Exporter utilisent la fenêtre de fichiers propre à l’émulateur. Un bouton bleu **📁+ Nouveau dossier**, en haut à gauche, crée un dossier dans le dossier affiché, avec une bulle d’aide pour l’expliquer ; tapez le nom, appuyez sur <kbd>Entrée</kbd>, et le nouveau dossier est prêt à recevoir votre fichier.

![La fenêtre Charger, avec le bouton 📁+ Nouveau dossier en haut à gauche.](img/fenetre-fichiers.png "medium")

Les réglages d’affichage ne sont pas dans la barre d’outils : chaque écran a sa propre **rangée d’affichage**, en haut de son panneau (section 3.2).

![La rangée d’affichage de l’écran de l’Apple-1 : zoom, phosphore, vitesse d’affichage, écran vintage et plein écran ; Détacher est au bout de la rangée.](img/barre-2.png "large")

!!! astuce "Le mode aide"
    Quand vous découvrez un panneau, allumez **?** : chaque bouton explique ce qu’il fait, au survol. Éteint, il ne reste que les petites infobulles habituelles (désactivables dans **⚙ → Interface**). Le réglage est retenu.

## La barre d’état

![La barre d’état pendant la lecture de BASIC.wav : vitesse, limite d’affichage, cassette, clavier, et le dernier message dessous.](img/barre-etat.png "large")

- **Vitesse mesurée** : environ 0,960 MHz en vitesse réelle. C’est la fréquence effective de l’Apple-1, qui suspend le 6502 quatre cycles sur soixante-cinq pour rafraîchir sa mémoire dynamique.
- **Affichage ~60 car./s** : le terminal de l’Apple-1 n’affiche qu’un caractère par image, soit environ 60 par seconde. Cliquez pour basculer entre cette limite et l’affichage instantané, comme avec les boutons **Normal** / **Rapide** des rangées d’affichage.
- **Cassette** : nom, position et durée, ou « pas de cassette ».
- **Clavier** : à qui vont les touches (l’Apple-1, ou un champ de texte de l’interface).

## Le clavier de l’Apple-1

Cliquez sur l’écran : les touches vont à l’Apple-1. Le clavier d’origine n’a que des majuscules, et l’émulateur convertit les minuscules.

| Touche du PC | Pour l’Apple-1 |
|---|---|
| lettres, chiffres, symboles | le caractère, en majuscule |
| <kbd>Entrée</kbd> | RETURN (code `$8D`) |
| <kbd>Retour arrière</kbd> | `_` : le Woz Monitor et le BASIC effacent le caractère précédent (il reste affiché, suivi de `_`) |
| <kbd>Échap</kbd> | ESC : annule la ligne en cours dans le Woz Monitor |
| <kbd>Ctrl</kbd>+lettre | le caractère de contrôle (`Ctrl+D` = `$84`…) |
| <kbd>Ctrl</kbd>+<kbd>R</kbd> / <kbd>Ctrl</kbd>+<kbd>L</kbd> | les boutons RESET et CLEAR SCREEN |

Vous pouvez **sélectionner du texte à la souris** sur l’un ou l’autre écran : appuyez et glissez sur les caractères, la sélection est surlignée. Un **clic droit** sur l’un des écrans (celui de l’Apple-1 ou le moniteur GEN2) ouvre un petit menu : **Copier** met le texte sélectionné dans le presse-papiers (ou tout le texte de l’écran quand rien n’est sélectionné), **Coller (taper le texte)** tape le presse-papiers au clavier de l’Apple-1, comme si vous le saisissiez, puis **Effacer l’écran** et **Reset**.

![Le menu du clic droit sur l’écran de l’Apple-1, pendant une partie de Hamurabi.](img/ecran-menu.png "medium")

!!! apple1 "L’écran de l’Apple-1 n’efface rien"
    Le terminal ne sait qu’écrire un caractère et passer à la ligne. Il n’y a ni curseur mobile, ni effacement d’un caractère : la touche d’effacement affiche `_`, et le caractère reste à l’écran. C’est normal.

## L’état de la machine

Le bouton **📸 État** de la barre d’outils prend un instantané complet de la machine et le remet plus tard, exactement au même point : vous pouvez arrêter un jeu ou un programme en cours, fermer l’émulateur, et le reprendre le lendemain. Le bouton ouvre un menu :

![Le menu État : enregistrer, charger, puis les derniers états avec leur date.](img/menu-etat.png "medium")

- **Enregistrer l’état…** (<kbd>Ctrl</kbd>+<kbd>Maj</kbd>+<kbd>S</kbd>) demande un nom de fichier et écrit un fichier `.a1state`. Le dossier proposé est `etats`, à côté du dossier de la bibliothèque.
- **Charger un état…** (<kbd>Ctrl</kbd>+<kbd>Maj</kbd>+<kbd>O</kbd>) remplace la machine par l’état choisi. La barre d’état montre le fichier et la valeur de PC.
- **Derniers états** liste les huit derniers, avec leur titre et la date de l’enregistrement ; un clic en charge un aussitôt. Si le fichier a été déplacé ou supprimé, un message le dit.

Un état contient la mémoire, les registres du 6502, le PIA, l’écran, la carte GEN2 si elle est branchée, et la cassette virtuelle avec sa position. Il ne contient ni vos préférences ni la bibliothèque. Un fichier illisible (abîmé, ou qui n’est pas un état) est refusé avec un message, et la machine reste intacte.

!!! astuce "Un usage typique"
    Enregistrez un état juste avant un passage difficile d’un jeu, ou à la fin d’une longue séance de BASIC : un état est aussi un moyen pratique de partager un programme en cours d’exécution avec un ami qui utilise la même version.

## Le programmateur d’EPROM

**EPROM**, dans la barre d’outils, ouvre une fenêtre pour lire, écrire et vérifier une puce EPROM ou EEPROM depuis l’émulateur, par exemple pour graver la ROM du BASIC, un programme pour l’extension GEN2, ou le contenu de votre propre mémoire.

![Le programmateur d’EPROM : programmateur, puce, source des données et les trois opérations.](img/programmateur-eprom.png "medium")

Seuls les programmateurs pilotés par l’outil libre **`minipro`** sont pris en charge : les XGecu **TL866II+**, **T48** et **T56**. Sous Debian, Ubuntu et Linux Mint, installez-le avec `sudo apt install minipro`, branchez le programmateur sur un port USB, puis appuyez sur **Détecter**. Si `minipro` manque, la fenêtre le dit et explique quoi faire ; rien d’autre n’est nécessaire du côté de l’émulateur.

| Réglage | Rôle |
|---|---|
| **Programmateur** | le programmateur détecté, avec son firmware (par exemple « Found TL866II+ 04.2.132 (USB) ») |
| **Puce** | les puces courantes du monde Apple-1 : 2716, 2732, 2764, 27C64, 27C128, 27C256, 27C512, 28C64, 28C256, SST27SF256, SST27SF512, et la ROM masquée 23128 (lecture seule) |
| **Données** | la **mémoire de l’Apple-1** (une plage d’adresses, `$E000`–`$EFFF` par défaut), ou un **fichier** (`.bin`, `.hex`, `.aiff`…) |

Trois boutons agissent sur la puce, et une barre de progression et un journal montrent ce qui se passe :

- **Lire la puce** lit toute la puce. Avec la mémoire ou un fichier existant comme données, le contenu est **comparé** et les différences sont listées : un fichier existant n’est jamais écrasé. Avec un nouveau nom de fichier, le contenu y est enregistré.
- **Écrire la puce** programme la puce avec les données choisies. La puce doit être vierge ou effaçable, et une confirmation du résultat s’affiche à la fin.
- **Vérifier** compare la puce avec les données choisies, octet par octet, et signale chaque différence (`$E010: puce $4C, données $A9`).

**Fermer** ferme la fenêtre.

La fenêtre ne modifie jamais la mémoire émulée : écrire et vérifier la lisent, et lire ne fait que la comparer ou remplir un fichier.

# L’écran : zoom, phosphore et effet vintage

## L’écran d’origine

L’Apple-1 affiche 40 colonnes × 24 lignes de majuscules. Chaque caractère est une matrice de 5 × 7 points dans une case de 7 × 8, lue dans la ROM du générateur de caractères **Signetics 2513** : l’émulateur utilise le contenu de cette ROM, point par point (annexe D). L’image fait donc 280 × 192 points.

Le curseur est un `@` qui clignote, piloté par un NE555 : 2,18 Hz, visible environ les deux tiers du temps. Sur certaines cartes, la tolérance du condensateur donne plutôt 1,7 Hz ; l’option `--cursor-hz 1.7` le reproduit.

## La rangée d’affichage

Chaque écran a sa propre rangée de réglages, en haut de son panneau : l’écran de l’Apple-1, et le moniteur couleur quand la carte GEN2 est branchée (chapitre 6).

| Réglage | Rôle |
|---|---|
| **Zoom** | une liste : `1x` à `8x` agrandissent l’image d’un facteur entier, les points restent carrés et nets ; **Ajuster** choisit le plus grand facteur qui tient dans le panneau (le facteur retenu est indiqué entre parenthèses). Au clavier : <kbd>Ctrl</kbd>+<kbd>+</kbd>, <kbd>Ctrl</kbd>+<kbd>−</kbd>, <kbd>Ctrl</kbd>+<kbd>0</kbd> (ajuster). |
| **Phosphore** | écran de l’Apple-1 seulement : le vert des moniteurs de l’époque, le blanc d’un téléviseur, ou l’ambre. |
| **Affichage : Normal / Rapide** | **Normal** affiche les caractères au rythme du vrai terminal, environ 60 par seconde ; **Rapide** affiche tout d’un coup. Ce réglage est commun : le changer sur un écran le change sur l’autre. |
| **Écran vintage** | l’effet de tube des années 1970 (section suivante) ; chaque écran a sa case. |
| **⛶** | plein écran (<kbd>F11</kbd>). |
| **Détacher** | met l’écran dans sa propre fenêtre (chapitre 10). |

Chaque écran est **centré** dans son panneau, horizontalement et verticalement, avec sa légende centrée sous l’image. Si le zoom choisi est plus grand que le panneau, l’écran s’affiche au plus grand zoom qui tient au lieu d’être coupé. Quand la fenêtre est plus étroite que la rangée d’affichage, la rangée passe sur une deuxième ligne, et **Détacher** reste à l’extrémité droite de la dernière ligne.

![Le phosphore ambre, avec l’écran vintage et une légère courbure.](img/brut/ambre.png "medium")

## L’écran vintage

La case **Écran vintage** de la rangée d’affichage imite un moniteur à tube des années 1970. L’effet principal est la fine ligne sombre entre deux lignes de balayage : chaque rangée de points devient une bande horizontale, comme sur la photo d’un vrai Apple-1. Cette ligne garde la même épaisseur en pixels de votre écran à tous les zooms : elle reste fine et nette, même en plein écran.

![L’écran vintage, avec un tube légèrement bombé (0,15) et les coins assombris, pendant une partie de Hamurabi.](img/brut/vintage.png "medium")

Dans **⚙ Préférences → Avancé → Écran vintage : réglages fins**, vous réglez l’**intensité** et choisissez les effets ; ces réglages sont communs aux deux écrans. Chaque curseur affiche sa **valeur numérique** à côté de lui (courbure, intensité…) :

| Effet | Par défaut | Rôle |
|---|---|---|
| séparation des lignes de balayage | activé | la ligne sombre entre deux rangées de points |
| points adoucis horizontalement | activé | un léger flou, horizontal seulement, comme le faisceau |
| rémanence du phosphore | courte | les points s’éteignent en moins d’un dixième de seconde (un peu plus lentement en ambre) ; tout à gauche : aucune |
| halo lumineux | activé, discret | une lueur autour des points, qui restent nets |
| tube bombé et coins assombris | désactivé | avec un curseur **Courbure** |
| scintillement | désactivé | une très légère variation de luminosité |
| masque RVB du phosphore | activé | écran couleur de la carte GEN2 seulement : de fines bandes verticales rouges, vertes et bleues séparées par de minces traits sombres, comme sur un tube de téléviseur couleur. Il est visible à partir du zoom 3, et sa force suit l’**intensité** |

L’effet peut aussi s’appliquer au moniteur couleur de la carte GEN2, par la case de sa propre rangée d’affichage. Il est calculé par le processeur, à la résolution de l’Apple-1, et seulement quand l’image change : il reste fluide même sans accélération graphique.

En ligne de commande : `--screening`, `--sans-screening`, `--gen2-screening` (moniteur GEN2), ou par exemple `--screening-options intensite=0.7,remanence=0.5,courbure=0.4,sans-halo` (valeurs de 0 à 1).

!!! astuce "Écran détaché et deuxième moniteur"
    **Détacher**, au bout de la rangée d’affichage, met l’écran de l’Apple-1 dans sa propre fenêtre. Placez-la sur un deuxième moniteur, en plein écran, avec l’effet vintage : le clavier va à l’Apple-1 dès que cette fenêtre a le focus. Avec **Ajuster**, l’image suit la taille de la fenêtre, par pas entiers (**Échelle entière**, dans la barre de titre de la fenêtre, le plus net) ou en la remplissant.
