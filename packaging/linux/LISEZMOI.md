# Retro Dev Kit Apple 1 — Le péché originel

*Démarrage rapide.* Émulateur et atelier de développement Apple-1 (famille Retro Dev Kit).

## Lancer

1. Décompressez `apple1-emu.zip` (clic droit → « Extraire ici »).
2. Double-cliquez sur **`run.sh`**. Si le gestionnaire de fichiers demande quoi faire (par exemple sous Linux Mint), choisissez
   **« Lancer »** (pas « Afficher »). La fenêtre de Retro Dev Kit Apple 1 s'ouvre, sans
   terminal.
3. Facultatif : dans un terminal ouvert dans ce dossier, `./install.sh`
   ajoute **Retro Dev Kit Apple 1** au menu des applications (rubrique Jeux) et les commandes
   `apple1-emu-gui` et `apple1-emu`. `./install.sh --bureau` met aussi une
   icône sur le bureau ; `./install.sh --desinstaller` retire tout (vos
   programmes et réglages restent).
   Les anciens `lancer.sh` et `installer.sh` s'appellent maintenant `run.sh`
   et `install.sh` : relancer `./install.sh` remplace une ancienne
   installation (menu, icône du bureau).

La version terminal d'avant est toujours là : `./run.sh --terminal`
(ou clic droit sur l'entrée du menu → « Version terminal »).

Rien à installer : `apple1-emu-gui` utilise les bibliothèques graphiques déjà
présentes sur la plupart des distributions Linux récentes (glibc 2.17 ou plus :
Ubuntu 18.04, Debian 9 et plus récentes ; X11 ou Wayland, OpenGL/Mesa).

**PC 64 bits et 32 bits** : l'archive contient les deux versions, dans
`bin/x86_64/` (PC 64 bits) et `bin/i686/` (PC 32 bits, Pentium 4 ou plus
récent). `run.sh` et `install.sh` choisissent seuls la bonne d'après
`uname -m`. Si la fenêtre
ne s'ouvre pas, `run.sh` réessaie seul avec le rendu logiciel
(`LIBGL_ALWAYS_SOFTWARE=1`) et le journal est dans `~/.cache/apple1-emu/gui.log`.

![zoom 3x](captures/capture-zoom-3x.png)

## Documentation

- `docs/Guide-demarrage-rapide.pdf` : 17 pages pour démarrer.
- `docs/Manuel-complet.pdf` : 131 pages. Il décrit chaque panneau, le BASIC (avec les bugs connus et comment les contourner), l'assembleur 6502 du devkit, et contient les annexes et un index.
- En anglais : `docs/Quick-Start-Guide.pdf` (19 pages) et `docs/Complete-Manual.pdf` (132 pages) ; en russe : `docs/Kratkoe-rukovodstvo.pdf` (19 pages) et `docs/Polnoe-rukovodstvo.pdf` (137 pages).
- En russe : `docs/Kratkoe-rukovodstvo.pdf` (19 pages) et `docs/Polnoe-rukovodstvo.pdf` (126 pages).
- `docs/exemples/basic/` et `docs/exemples/asm/` (copiés aussi par `install.sh`) : les programmes des manuels (`docs/exemples/en/` pour la version anglaise).

## L'écran et le zoom

- Les caractères sont dessinés point par point avec la ROM du générateur de
  caractères **2513** de l'Apple-1 (40 × 24 caractères, 280 × 192 points),
  avec le curseur `@` qui clignote.
- **Zoom** : boutons `1x` à `8x`, ou **Ajuster** (le plus grand zoom qui tient
  dans la fenêtre, les points restent nets). Au clavier : **Ctrl+plus**,
  **Ctrl+moins**, **Ctrl+0** (ajuster). **F11** : plein écran.
- **Phosphore** vert, blanc ou ambre ; case **Lignes de balayage** pour
  l'effet « écran cathodique ».
- Case **Écran vintage (screening)** : imite un moniteur à tube d'époque.
  De fines lignes sombres séparent les lignes de balayage et restent fines à
  tous les zooms. Les points sont à peine adoucis, avec un halo discret.
  Dans les Préférences, vous trouvez des curseurs pour l'intensité, la
  rémanence (courte par défaut, ou aucune) et la courbure du tube bombé
  (option). Le scintillement est aussi en option.
  Voir `captures/capture-screening.png`.
- La fenêtre se redimensionne ; elle prend 80 % de l'écran au premier
  lancement. Zoom, couleur et taille sont retenus
  (`~/.config/apple1-emu/gui.json`).

## Les boutons

| Bouton | Effet |
|---|---|
| 📂 **Charger** | un `.wav` ou `.aiff` : **en mémoire** (immédiat) ou **comme cassette virtuelle** (comme sur la vraie machine) ; un `.txt` (vidage du moniteur) ; un `.bin` (l'adresse est demandée, ex. `300` ou `E000`) |
| 💾 **Sauver** | tapez les plages (`0300.03FF`, ou `004A.00FF 0300.0FFF` pour un programme BASIC), puis choisissez le fichier : `.wav` / `.aiff` = cassette pour le vrai Apple-1, `.txt` = vidage hexa, `.bin` = binaire |
| ⟲ **Reset** | bouton RESET (Ctrl+R) |
| 🗋 **Effacer écran** | bouton CLEAR SCREEN (Ctrl+L) |
| ▶ **Vitesse réelle** / ⏩ **rapide** | bascule (F3) |
| 📋 **Coller** | tape le contenu du presse-papiers sur le clavier de l'Apple-1 (aussi Ctrl+V) |
| **BASIC** | charge le **RDK G2 Basic** intégré (BASIC libre compatible avec l'Integer BASIC, 7 933 octets) en `$E000` ; puis tapez `E000R`. Pour utiliser l'Integer BASIC d'origine (fichier à vous, jamais fourni), choisissez-le dans ⚙ Préférences → Avancé → BASIC : une cassette `.wav`/`.aiff` (comme `BASIC.wav`), un `.txt` ou un `.bin` de 4 Ko |
| ⚙ **Préférences** | **langue**, **dossier de la bibliothèque**, mode du double-clic, affichage, fichier du BASIC, **carte couleur Uncle Bernie GEN2** (voir plus bas) ; tout est retenu |
| Cassette ▶ ⏹ ⏪ ⏺ ⏏ | lecture, stop, rembobiner, **enregistrer** (ce que l'Apple-1 écrit sur la cassette va dans un `.wav`/`.aiff`), éjecter ; la barre indique la position |

![cassette](captures/capture-cassette.png)

## La carte couleur Uncle Bernie GEN2

Dans **⚙ Préférences**, cochez « Carte couleur Uncle Bernie GEN2 ». Vous
pouvez aussi lancer `./run.sh --carte-couleur`. Un second écran, en
couleur, apparaît à côté de celui de l'Apple-1 : c'est le moniteur branché
sur la carte. Le terminal de l'Apple-1 continue de fonctionner sur le
premier écran.

- La carte reprend la vidéo de l'Apple II : texte 40 × 24, LORES 16
  couleurs, HIRES 280 × 192, mode MIXED, 2 pages.
- Les modes se choisissent en **lisant** `$C250` à `$C257`, par exemple
  `C250` puis `C257` dans le moniteur pour passer en HIRES.
- La carte apporte aussi la RAM de `$0000` à `$BFFF`.

Pour essayer, cliquez sur ▶ à côté d'un de ces exemples de la bibliothèque (il est chargé puis lancé) :

| Fichier | Lancement |
|---|---|
| `gen2-barres.txt` (16 barres de couleur) | `C00R` |
| `gen2-test3.apl` (tous les caractères) | `C00R` |
| `gen2-test2.apl` (LORES puis machine à écrire) | `280R` |
| `gen2-test4.apl` (fenêtre de texte qui défile) | `280R` |
| `gen2-test1.apl` (test mémoire, rayures HIRES) | `24BR` |

Si votre bibliothèque existait déjà, copiez-y les fichiers `gen2-*` du dossier
`exemples/`. Les quatre tests sont d'Uncle Bernie ; leur mode d'emploi est dans
`exemples/GEN2_TESTS_HOWTO`. Ce qui est documenté et ce qui est approché
(teintes exactes, dessin des minuscules) est expliqué dans `MANUEL.md`.

![carte couleur](captures/capture-gen2-test2-lores.png)

## La bibliothèque (à droite)

- Vos programmes sont dans **`~/apple1/programmes`** (créé au premier
  lancement, avec les exemples rangés en cinq dossiers : **BASIC**, **6502**,
  **GEN2 COLOR**, **WAV** et **Art ASCII**). Pour un autre dossier : **⚙ Préférences →
  Parcourir…** (« Par défaut » pour revenir). « Ouvrir le dossier » le montre
  dans Nemo. Déposez-y vos `.wav`, `.aiff`, `.txt`, `.apl`, `.bin` : la liste
  se met à jour seule.
- Pour chaque programme : nom, type, adresses, durée de la cassette, commande
  pour le lancer, et vos notes.
- **▶** à droite du nom : charge le programme (si besoin) puis le **lance**
  (RESET puis `300R`, ou `E2B3R` puis `RUN` pour un programme BASIC). Pour
  un programme BASIC, le BASIC du programme est chargé d'abord, tout seul : le
  RDK G2 Basic intégré, ou l'Integer BASIC d'origine si le programme est réglé
  « Programme Integer BASIC » (⚙) et que vous l'avez choisi dans les préférences.
- Cassettes sans adresses (archive.org « Apple1Cassettes ») : `BASIC.wav`
  est reconnu (`E000.EFFF`), et un programme BASIC en deux enregistrements
  (`Hamurabi.wav`, `StarTrek.wav`…) est placé d'après ses pointeurs
  (« devinées » dans la liste ; vérifiez avec ⚙). Dans ⚙, une plage par
  enregistrement, dans l'ordre : `004A.00FF 0300.0FFF`.
- **Cassettes reconnues** : les 24 cassettes d'archive.org
  (« Apple1Cassettes » : Hamurabi, Star Trek, Microchess, Lunar Lander,
  Football…) sont reconnues par leur **contenu** (même renommées), ou à
  défaut par leur nom. Leurs paramètres ⚙ (plages, BASIC ou langage machine,
  commande de lancement) sont **préréglés** et la liste affiche
  **✔ reconnu**. Il suffit de cliquer sur ▶. Les paramètres que vous avez
  réglés vous-même ne sont **jamais** remplacés. Les gros programmes
  (Football, Deal or No Deal, Star Trek 2003, Life) reçoivent la RAM
  nécessaire. Détail et tableau : MANUEL.md.
- **⚙** à droite du nom : les **paramètres** du programme, que vous pouvez
  corriger : plages (comme pour l'ACI, ex. `0300.031E` ou `004A.00FF 0800.0FFF`),
  adresse de chargement d'un `.bin`, adresse et commande de lancement, type
  (langage machine ou BASIC), besoin du BASIC, notes. Ils sont vérifiés, puis
  rangés dans `programs.json` dans le dossier.
- **Double-clic**, au choix (au-dessus de la liste) :
  **⚡ Rapide** = chargement instantané en mémoire ;
  **📼 Cassette** = comme sur la vraie machine : `C100R` et `0300.031ER` sont
  tapés pour vous, la cassette défile et une **barre de progression**
  s'affiche sous le nom du programme.
- **Clic droit** (ou boutons au-dessus de la liste) : charger, charger et
  lancer, insérer comme cassette, paramètres, renommer, supprimer (avec
  confirmation), ouvrir le dossier.

- **Plusieurs dossiers et projets GitHub** (v6) : « + Dossier » et
  « + Projet GitHub » (adresse ou `propriétaire/dépôt`) en haut de la
  bibliothèque, qui devient un arbre repliable. Sur un projet GitHub,
  **Tester le dernier build** prend la dernière Release, sinon l'artefact
  GitHub Actions, sinon compile les sources (BASIC tapé, ou ca65/ld65 si le
  paquet `cc65` est installé) ; **Mettre à jour** prend le dernier commit.
  Position gauche/droite dans ⚙ Préférences. Bouton **?** ou **F1** :
  bulles d'aide détaillées. Détails : MANUEL.md.

- **Son et oscilloscope de la cassette** (v6.1) : pendant une lecture ou un
  enregistrement, 🔊 fait entendre le signal (volume faible, réglable ; muet
  en vitesse maximale) et 📈 affiche une fenêtre oscilloscope (trace verte,
  10 ms autour de la tête de lecture).

- **Éditeur Integer BASIC** (v7.0) : bouton **📝 Éditeur**. Vos
  programmes en onglets (Nouveau, Ouvrir, Enregistrer), en couleurs, avec
  les erreurs expliquées (ligne, numéro BASIC, colonne). **▶ Lancer** met le
  programme directement en mémoire (BASIC chargé tout seul depuis
  `BASIC.wav`) et tape `E2B3R` puis `RUN` ; **📼 Exporter WAV** en fait une
  cassette pour le vrai Apple-1, et le bas de la fenêtre indique quoi taper
  (`C100R` puis par exemple `004A.00FFR 0C3B.0FFFR`, puis `E2B3R` et `RUN`).
  Taille du texte : A−/A+, Ctrl+molette, ou Ctrl+plus/moins dans le texte.

![éditeur BASIC](captures/capture-editeur.png)

- **Assembleur 6502** (v7.1) : le même éditeur ouvre vos sources `.s` /
  `.asm` (syntaxe ca65 : étiquettes, macros, `.include`…), en couleurs,
  avec les erreurs expliquées (ligne et colonne) et un 📃 Listing.
  **▶ Lancer** assemble, met le code en mémoire et tape `300R` ;
  **📼 Exporter WAV** en fait une cassette, et le bas de la fenêtre indique
  quoi taper (`C100R` puis par exemple `0300.0B58R`, puis `300R`). En ligne
  de commande : `apple1-emu asm jeu.s -o jeu.wav`.

![assembleur 6502](captures/capture-assembleur.png)

- **Éditeur à gauche ou sur un autre écran** (v7.2) : l'éditeur est ancré à
  gauche (largeur réglable en tirant son bord) ; **🗗 Détacher** l'ouvre dans
  sa propre fenêtre, à déplacer sur un deuxième écran, **⊟ Ancrer** le remet
  à gauche. Taille et position sont retenues.

![éditeur à gauche](captures/capture-editeur-gauche.png)

- **Débogueur BASIC et 6502** (v7.3) : **🐛 Débogueur** arrête l'Apple-1 et
  l'exécute pas à pas (instruction, par-dessus un `JSR`, sortie d'un
  sous-programme, jusqu'au curseur, ligne BASIC), avec points d'arrêt,
  registres et drapeaux modifiables, désassemblage avec étiquettes, variables
  BASIC suivies (`A`, `D(2)`, `C$`), mémoire surveillée et dump hexa/ASCII
  modifiable ; la ligne en cours est surlignée dans l'éditeur.

![débogueur](captures/capture-debogueur.png)

- **Assembleur perfectionné** (v7.4) : `.if/.elseif/.else/.endif`,
  `.ifdef/.ifndef`, `.repeat/.endrep`, `.define`, préfixes `a:`/`z:`,
  échappements `\n \t \xHH` (avec `.feature string_escapes`), `.setcpu
  "65C02"` (instructions du 65C02, pour une carte modifiée), macros
  développées et table des symboles dans le listing, avertissement quand une
  adresse de page zéro est codée sur 2 octets. Mêmes octets que ca65.

- **Photo → ASCII** (v7.7) : **🎨 ASCII** ouvre un panneau ancrable ;
  glissez-déposez une photo (jpg, png), recadrez (cadre vert, molette),
  réglez lignes/colonnes (40 × 24, ou 23 lignes), luminosité, contraste,
  gamma, inversion, tramage ; seuls les caractères du 2513 sont utilisés ;
  aperçu avec la police de l'Apple-1. **✎ Insérer** met le programme
  (BASIC en `PRINT`, ou 6502 en `$0300`) dans un nouvel onglet de
  l'éditeur, **▶ Lancer**, **📼 Exporter WAV…**, **💾 Enregistrer** dans
  le dossier « Art ASCII » de la bibliothèque : exemples originaux et
  portraits de **Wozniak** et **Jobs** (photos libres CC BY-SA, voir
  `LICENCES.md`). Le diaporama de Dave Schmenk est un projet GitHub de la
  bibliothèque (**▶ Tester le dernier build**). En ligne de commande :
  `apple1-emu ascii photo.jpg -o art.wav`.

![Photo → ASCII](captures/capture-ascii.png)

![Art ASCII dans la bibliothèque](captures/capture-ascii-bibliotheque.png)

- **Disposition ancrable** (v7.6) : chaque panneau (éditeur, écran
  Apple-1, écran couleur GEN2, débogueur, désassemblage, dump,
  bibliothèque) est un onglet qu'on fait glisser à côté, au-dessus, en
  dessous d'un autre ou dans le même groupe ; disposition retenue. Au
  départ : éditeur à gauche, écran au centre (couleur en dessous),
  débogueur et désassemblage dessous, dump, bibliothèque à droite ;
  **⊞ Disposition par défaut** la remet. **🗗 Détacher** met n'importe quel
  panneau, dont l'écran et l'écran couleur séparément, dans sa propre
  fenêtre (clavier inclus pour les écrans), **⊟ Ancrer** le remet.

![disposition de départ](captures/capture-disposition.png)

- **Fenêtres** (v7.5) : le désassemblage ne sautille plus pendant
  l'attente d'une touche (24 lignes fixes, recentrage seulement quand PC
  sort) ; **🗐 Désassemblage** et **🗐 Dump** ont chacun leur fenêtre,
  séparée ou ancrée, retenue ; points d'arrêt retenus ; **🗗 Écran séparé**
  met l'écran de l'Apple-1 dans une fenêtre déplaçable et redimensionnable
  (image proportionnelle, échelle entière au choix, clavier inclus) ;
  **🎨 GEN2** branche ou débranche la carte couleur en un clic.

![désassemblage et dump séparés](captures/capture-debogueur-fenetres.png)

![écran séparé](captures/capture-ecran-separe.png)

![chargement par la cassette](captures/capture-chargement-cassette.png)

## Langue de l'interface

Français, anglais, allemand, russe, espagnol, italien ou portugais. Par
défaut, l'émulateur prend la langue de **votre disposition de clavier**
(clavier français → français, `us` → anglais, `ru` → russe…), sinon celle du
système (`LANG`), sinon l'anglais. Pour en imposer une : **⚙ Préférences →
Langue** (ou `./run.sh --langue fr`).

## Le clavier

Cliquez sur l'écran de l'Apple-1, puis tapez : les minuscules deviennent des
majuscules, **Entrée** = RETURN, **Retour arrière** = `_` (effacement du
moniteur), **Échap** = ESC. Le bas de la fenêtre indique où va le clavier.

Essai : cliquez sur ▶ à côté de `hello.aiff` : `HELLO WORLD!` s'affiche. Ou
double-cliquez sur `hello.aiff`, cliquez sur l'écran, tapez `300R` puis
Entrée.

Avec la cassette virtuelle : Charger → `hello.wav` → « Insérer comme
cassette », puis tapez `C100R` (Entrée) et `0300.031ER` (Entrée) ; la cassette
démarre seule, et au retour du `\`, tapez `300R`.

## Pour aller plus loin

Le manuel complet (export vers un vrai Apple-1 avec un smartphone, format
ACI, PROM, tests…) est dans **`MANUEL.md`**.
