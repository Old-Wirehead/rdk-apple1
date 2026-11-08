# L’éditeur

<p class="lead">L’éditeur intégré écrit des programmes Integer BASIC et des sources 6502, les vérifie à chaque frappe, les lance d’un clic dans l’Apple-1 émulé, et les exporte en cassette pour la vraie machine.</p>

## Les onglets et les fichiers

**📝 Éditeur** ouvre le panneau, ancré à gauche de l’écran. Sa barre d’outils est compacte, pour laisser le plus de lignes possible au texte. Elle commence par des icônes, expliquées par leurs infobulles :

| Icône | Action |
|---|---|
| **Nouveau** (première icône) | nouvel onglet : « Programme BASIC » ou « Source assembleur 6502 » |
| **📂** / **💾** / **💾…** | Ouvrir (`.bas`, `.txt`, `.s`, `.asm`, `.a65`, `.inc`) / Enregistrer (<kbd>Ctrl</kbd>+<kbd>S</kbd>) / Enregistrer sous… |
| **✂**, copier, **📋** | Couper, copier, coller |
| **🔍** | Rechercher et remplacer (<kbd>Ctrl</kbd>+<kbd>F</kbd>, <kbd>Ctrl</kbd>+<kbd>H</kbd>) |
| **⟲ ⟳** | Annuler (<kbd>Ctrl</kbd>+<kbd>Z</kbd>), rétablir (<kbd>Ctrl</kbd>+<kbd>Maj</kbd>+<kbd>Z</kbd>) |
| **A▲▼** | Taille du texte : un seul contrôle compact, la lettre A avec deux petits triangles empilés ; ▲ agrandit, ▼ réduit |

Viennent ensuite, sur la même rangée (ou la suivante si le panneau est étroit), les actions de l’onglet : **▶ Lancer**, **📼 Exporter WAV** et **Mémoire** pour le BASIC, ou **📃 Listing**, **⏺ Point d’arrêt** et **⏩ Jusqu’au curseur** pour une source 6502.

Chaque fichier est un onglet. Un point • signale un onglet modifié ; × le ferme (un second clic est demandé s’il n’est pas enregistré). Un onglet marqué **📚** a été ouvert depuis la bibliothèque : ouvrir un listing ou une source de la bibliothèque (▶, ⬇ ou double-clic) ouvre son onglet quand l’éditeur est visible, ou l’affiche s’il est déjà ouvert. Les onglets, texte non enregistré compris, sont retrouvés au lancement suivant. Un clic droit sur un onglet propose **Enregistrer sous…** et **Fermer**. Dans le texte, un **clic du milieu** colle la sélection primaire (Linux) : le curseur se place là où vous cliquez, puis le dernier texte surligné dans n’importe quelle application est inséré.

La taille du texte se règle avec le contrôle **A▲▼**, <kbd>Ctrl</kbd>+molette, ou <kbd>Ctrl</kbd>+<kbd>+</kbd> / <kbd>Ctrl</kbd>+<kbd>−</kbd> / <kbd>Ctrl</kbd>+<kbd>0</kbd> quand le texte a le clavier.

**Détacher** met l’éditeur dans sa propre fenêtre, par exemple sur un deuxième écran ; **⊟ Ancrer** le remet en place. Les touches tapées dans cette fenêtre vont à l’éditeur, jamais à l’Apple-1.

## Rechercher, remplacer et le menu du clic droit

**🔍** (ou <kbd>Ctrl</kbd>+<kbd>F</kbd>) ouvre une barre de recherche au-dessus du texte : <kbd>Entrée</kbd> va à l’occurrence suivante, <kbd>Maj</kbd>+<kbd>Entrée</kbd> à la précédente, **Aa** respecte la casse, **Mot** cherche des mots entiers, et le compteur indique « 3 sur 7 ». <kbd>Ctrl</kbd>+<kbd>H</kbd> ajoute le champ **Remplacer par**, avec **Remplacer** (l’occurrence courante) et **Tout remplacer**. <kbd>Échap</kbd> ferme la barre.

![La barre de recherche ouverte sur ECHO, et le menu du clic droit d’une source 6502.](img/editeur-menu.png "medium")

Le **clic droit** dans le texte ouvre un menu avec toutes les commandes et leur raccourci. Le menu agit sur la **sélection**, qui est conservée quand vous faites un clic droit dessus : **Couper** et **Copier** travaillent sur le texte sélectionné. Les commandes qui reposent sur un mot (aller à la définition, aide) utilisent le **mot sous le clic**, pas celui sous le curseur :

| Commande | Raccourci |
|---|---|
| Couper, Copier, Coller, Tout sélectionner | <kbd>Ctrl</kbd>+<kbd>X</kbd>, <kbd>C</kbd>, <kbd>V</kbd>, <kbd>A</kbd> |
| Rechercher, Remplacer | <kbd>Ctrl</kbd>+<kbd>F</kbd>, <kbd>Ctrl</kbd>+<kbd>H</kbd> |
| Aller à la ligne… | <kbd>Ctrl</kbd>+<kbd>G</kbd> |
| Commenter / décommenter (6502), REM / dé-REM (BASIC) | <kbd>Ctrl</kbd>+<kbd>/</kbd> |
| Indenter, Désindenter | |
| Aller à la définition d’une étiquette (6502) ou d’un numéro de ligne (BASIC) | <kbd>F12</kbd> |
| Aide sur *mot* (sur un mot-clé, une instruction ou une directive : ouvre sa fiche dans la référence) | |
| Voir dans le désassemblage (6502) | |
| Renuméroter… (BASIC : `GOTO`, `GOSUB` et `THEN` suivent) | |
| Point d’arrêt sur cette ligne | <kbd>F9</kbd> |
| Assembler et lancer (6502), Envoyer et lancer (BASIC) | <kbd>F5</kbd> |

**Couleurs.** Les couleurs du texte suivent le thème de l’interface. **⚙ Préférences → Éditeur** propose, pour le BASIC et pour l’assembleur 6502 séparément, les palettes *Automatique*, *Classique (sombre)*, *Clair*, *Ambre*, *Contraste élevé*, ou vos propres couleurs, une pastille par sorte de mot (chapitre 11).

## Un programme BASIC

![L’éditeur BASIC : une erreur signalée sur fond rouge, avec son numéro de ligne et sa colonne en bas du panneau.](img/editeur-erreur.png "large")

Le texte est coloré : numéros de ligne, mots-clés, nombres, chaînes, `REM`, opérateurs. Chaque ligne est **tokenisée** comme le ferait l’Integer BASIC, par un tokeniseur natif comparé octet pour octet au vrai. Une ligne que le BASIC refuserait passe sur fond rouge, et l’erreur indique la ligne du texte, le numéro de ligne BASIC et la colonne ; un clic y place le curseur. Erreurs détectées : syntaxe, nom de variable invalide, nombre supérieur à 32767, commande du mode direct (`LIST`, `RUN`…) dans le programme, chaîne non fermée, ligne de plus de 255 octets, mémoire pleine.

- **Mémoire** : automatique (8 Ko avec LOMEM `$0800`, ou `$0300` si le programme est grand, sinon 12, 16 ou 32 Ko), ou imposée.
- {{▶ Lancer}} : charge l’Integer BASIC s’il manque (comme le bouton BASIC), met le programme tokenisé en mémoire, ajoute de la RAM si besoin, puis tape `E2B3R` et `RUN`. C’est instantané : rien n’est tapé ligne à ligne.
- {{📼 Exporter WAV}} : une cassette (`.wav`, ou `.aiff` pour un smartphone) de deux enregistrements, avec sa fiche dans la bibliothèque. Le bas du panneau affiche ce qu’il faut taper sur le vrai Apple-1, par exemple `C100R` puis `004A.00FFR 0C3B.0FFFR`, puis `E2B3R` et `RUN` ; **📋 Copier** le met dans le presse-papiers.

![De l’éditeur au vrai Apple-1 : écrire, lancer, corriger, exporter.](diagrams/workflow-basic.svg "diagram large")

## Une source 6502

![Une source 6502 dans l’éditeur : les couleurs, le résultat de l’assemblage et la ligne à taper sur le vrai Apple-1.](img/brut/editeur-asm.png "large")

Une source est assemblée **à chaque modification** par l’assembleur intégré à deux passes, qui accepte la syntaxe de ca65 (et celle de xa, traduite automatiquement). Le bas du panneau affiche la taille et les adresses (`36 octets, $0300-$0323 — 2 fichier(s), 13 symboles`), puis la ligne pour la vraie machine (`C100R` puis `0300.0323R`, puis `300R`).

- **Erreurs** : fichier, ligne et colonne ; un clic y mène, et ouvre le fichier inclus s’il le faut. Les avertissements (page zéro codée en absolu) sont en jaune.
- **📃 Listing** : adresse, octets et source de chaque ligne, avec les fichiers inclus et les macros développées, puis la table des symboles ; un clic ramène à la ligne.
- **▶ Lancer** : met les octets en mémoire à l’origine (`.org`, `$0300` par défaut), ajoute de la RAM si besoin, puis tape par exemple `300R`.
- **📼 Exporter WAV** : la cassette, comme pour le BASIC.
- **⏺ Point d’arrêt** et **⏩ Jusqu’au curseur** : le lien avec le débogueur (chapitre 8).
- **Lien avec le débogueur** : quand vous sélectionnez une instruction dans la source (ou placez le curseur sur sa ligne), les octets qu’elle a produits sont surlignés dans le **désassemblage** du débogueur et dans le **dump hexa / ASCII**, si ces panneaux sont visibles ; s’ils sont ailleurs en mémoire, les panneaux défilent pour les montrer. La table des lignes source et des adresses de l’assembleur le rend possible.

![Une erreur d’assemblage : ECO au lieu de ECHO, symbole inconnu, signalé avec sa ligne et sa colonne.](img/brut/editeur-asm-erreur.png "large")

La partie III est entièrement consacrée à l’assembleur : syntaxe, directives, listing, exemples.
