# Annexes <span class="part-sub">Carte mémoire, commandes du Woz Monitor, temps de l’ACI, jeu de caractères et raccourcis clavier</span> {: .part data-part="Annexes" }

# Carte mémoire {: .appendix }

![La mémoire de l’Apple-1 dans l’émulateur, des vecteurs du moniteur à la page zéro.](diagrams/memoire.svg "diagram large")

| Adresses | Contenu | Détail |
|---|---|---|
| `$0000-$00FF` | page zéro | Woz Monitor `$24-$2B` ; Integer BASIC `$4A-$FF` (pointeurs LOMEM `$4A`, HIMEM `$4C`, PP `$CA`, PV `$CC`) |
| `$0100-$01FF` | pile du 6502 | |
| `$0200-$027F` | tampon de ligne du moniteur | 128 caractères |
| `$0280-$0FFF` | RAM de base (4 Ko) | programmes en langage machine ; BASIC entre LOMEM (`$0800`) et HIMEM (`$1000`) |
| `$1000-$BFFF` | RAM d’extension | 8 Ko par défaut dans l’émulateur (jusqu’à `$1FFF`), jusqu’à 48 Ko avec `--ram` |
| `$C000-$C0FF` | ACI : bascule de sortie, entrée bande | tout accès fait basculer la sortie |
| `$C100-$C1FF` | ACI : PROM (programme de l’ACI) | lancé par `C100R` |
| `$4000-$7FFF` | CodeTank de la P-LAB | EPROM de 16 Ko, en lecture seule, quand une ROM est branchée (par-dessus la RAM) |
| `$C250-$C257` | carte GEN2 : commutateurs | lecture ; pages texte/LORES `$0400`, `$0800`, HIRES `$2000`, `$4000` |
| `$CC00-$CC01` | carte P-LAB : TMS9918A | `$CC00` données, `$CC01` contrôle et état ; répété sur `$CC00-$CFFF` |
| `$D010-$D013` | PIA : `KBD`, `KBDCR`, `DSP`, `DSPCR` | chapitre 22 |
| `$E000-$EFFF` | BASIC | RDK G2 Basic (7 933 octets, il a besoin de RAM jusqu’à `$FEFF`), ou Integer BASIC d’Apple (4 Ko de RAM, chargés par cassette ou image) |
| `$FF00-$FFFF` | Woz Monitor | vecteurs : NMI `$0F00`, RESET `$FF00`, IRQ/BRK `$0000` |

# Les commandes du Woz Monitor {: .appendix }

Le moniteur attend une ligne terminée par <kbd>RETURN</kbd>. Les adresses et les octets s’écrivent en hexadécimal, sans `$` ; une ligne peut contenir plusieurs commandes séparées par des espaces.

| Commande | Effet | Exemple |
|---|---|---|
| `adr` | affiche l’octet à cette adresse | `FFEF` |
| `adr.adr` | affiche une plage, 8 octets par ligne | `FF00.FF1F` |
| `.adr` | continue l’affichage jusqu’à cette adresse | `.FF3F` |
| `adr: oct oct …` | range des octets à partir de l’adresse | `300: A9 C1 20 EF FF` |
| `: oct …` | continue le rangement | `: 4C 1F FF` |
| `adrR` | lance le programme à cette adresse (`JMP`) | `300R` |
| <kbd>Échap</kbd> | annule la ligne (affiche `\`) | |
| `_` (retour arrière) | annule le dernier caractère | |

{{!adrR}}{{!adr.adr}}

Une ligne de plus de 127 caractères est annulée comme par Échap. Après `R`, le moniteur affiche d’abord l’octet de l’adresse (`0300: A2`), puis le programme prend la main.

| Adresse | Commande | Rôle |
|---|---|---|
| `E000R` | démarrage à froid du BASIC | efface le programme |
| `E2B3R` | démarrage à chaud du BASIC | garde le programme |
| `C100R` | programme de l’ACI | puis `début.finR` (lire) ou `début.finW` (écrire) |
| `FF00R` | RESET du moniteur par logiciel | |

# Les temps de l’ACI {: .appendix }

Les durées sont produites par le programme de l’ACI, en comptant les cycles entre deux basculements de la sortie. Valeurs mesurées dans l’émulateur, qui exécute ce programme sur son 6502 (horloge 1,022727 MHz, avec l’arrêt de 4 cycles sur 65 pour le rafraîchissement de la mémoire) :

| Élément | Cycles | Durée | Fréquence |
|---|---|---|---|
| demi-période de l’en-tête | 592 | 618 µs | ≈ 809 Hz |
| 1re moitié du bit de départ | 180 | 188 µs | |
| demi-période d’un « 0 » | 238 | 248 µs | ≈ 2,0 kHz |
| demi-période d’un « 1 » | 473 | 494 µs | ≈ 1,0 kHz |
| en-tête complet | 16 390 demi-périodes | ≈ 10,1 s | |

Format d’un enregistrement :

<ol class="steps">
<li>l’<strong>en-tête</strong> : environ 10 secondes de demi-périodes de 618 µs ;</li>
<li>le <strong>bit de départ</strong> : une demi-période courte (188 µs), puis une demi-période de « 0 » ;</li>
<li>les <strong>données</strong> : chaque octet, bit de poids fort en premier, une période complète par bit ;</li>
<li>rien d’autre : ni longueur, ni adresse, ni somme de contrôle. C’est la commande de lecture qui donne la plage.</li>
</ol>

À la lecture, le programme de l’ACI mesure les demi-périodes par une boucle de 12 cycles. Il prend pour bit de départ une demi-période de moins de 31 tours (≈ 372 cycles, 388 µs), et sépare « 0 » et « 1 » à environ 0,74 ms par période complète. Le décodeur de fichiers de l’émulateur accepte des en-têtes de 0,40 à 0,90 ms par demi-période, ce qui couvre l’outil c2t (1 kHz) et des écarts de vitesse de bande.

# Le jeu de caractères du 2513 {: .appendix }

![Les 64 caractères du Signetics 2513, tels que l’émulateur les dessine : codes $20 à $5F (avec le bit 7, $A0 à $DF).](img/charset-2513.png "large")

Le générateur de caractères ne connaît que 64 signes : l’espace, la ponctuation, les chiffres et les majuscules. Le terminal ne garde que 6 bits du code envoyé :

| Code envoyé | Affichage |
|---|---|
| `$20-$5F` (ou `$A0-$DF`) | le caractère de la table |
| `$60-$7F` (minuscules, ou `$E0-$FF`) | la majuscule correspondante (`$40-$5F`) |
| `$0D` (ou `$8D`) | retour à la ligne |
| autres `$00-$1F` | rien : ignorés |

Chaque caractère est une matrice de 5 × 7 points dans une case de 7 × 8 ; l’écran fait 40 × 24 caractères, soit 280 × 192 points.

# Les raccourcis clavier {: .appendix }

| Touche | Effet |
|---|---|
| <kbd>Ctrl</kbd>+<kbd>R</kbd> | RESET de l’Apple-1 |
| <kbd>Ctrl</kbd>+<kbd>L</kbd> | CLEAR SCREEN |
| <kbd>Ctrl</kbd>+<kbd>V</kbd> | coller : tape le presse-papiers au clavier de l’Apple-1 |
| <kbd>F1</kbd> | mode aide (bulles détaillées) |
| <kbd>F3</kbd> | vitesse réelle ou rapide |
| <kbd>F11</kbd> | plein écran |
| <kbd>Ctrl</kbd>+<kbd>+</kbd> / <kbd>Ctrl</kbd>+<kbd>−</kbd> / <kbd>Ctrl</kbd>+<kbd>0</kbd> | zoom de l’écran (ou du texte de l’éditeur, s’il a le clavier) |
| <kbd>Ctrl</kbd>+<kbd>S</kbd> | enregistrer le fichier de l’éditeur |
| <kbd>Ctrl</kbd>+<kbd>F</kbd> / <kbd>Ctrl</kbd>+<kbd>H</kbd> / <kbd>Ctrl</kbd>+<kbd>G</kbd> | éditeur : rechercher, remplacer, aller à la ligne |
| <kbd>Ctrl</kbd>+<kbd>/</kbd> | éditeur : commenter / décommenter (6502), REM / dé-REM (BASIC) |
| <kbd>F5</kbd> / <kbd>F9</kbd> / <kbd>F12</kbd> | éditeur : lancer, point d’arrêt sur la ligne, aller à la définition (6502) |
| <kbd>Ctrl</kbd>+lettre | caractère de contrôle (`Ctrl+D` = `$84`) |
| clic droit sur un écran | Copier, Coller (taper le texte), Effacer l’écran, Reset |
| <kbd>Échap</kbd> | ESC (`$9B`) : annule la ligne dans le moniteur et le BASIC |
| retour arrière | tape `_`, l’effacement du moniteur et du BASIC |

Dans le **terminal** (`apple1-emu` sans fenêtre, chapitre 12) :

| Touche | Effet |
|---|---|
| <kbd>F2</kbd> | ligne de commande de l’émulateur |
| <kbd>F5</kbd> / <kbd>F6</kbd> | cassette : lecture, arrêt |
| <kbd>F10</kbd> ou <kbd>Ctrl</kbd>+<kbd>Q</kbd> | quitter |

# Pièges et comportements à connaître {: .appendix }

De petites choses qui surprennent la première fois. Aucune n’est un bogue ; chacune a sa raison.

- **BASIC et RUN.** **▶ Lancer** de l’éditeur démarre le BASIC, charge le listing et tape `RUN` pour vous. Un programme qui demande une saisie attend vos touches sur l’écran de l’Apple-1 : cliquez d’abord sur l’écran.
- **RND.** Le premier `RND` d’une exécution serait toujours le même sur une machine qui vient d’être remise à zéro ; **▶ Lancer** attend un moment aléatoire avant `RUN`, pour qu’un jeu ne commence pas toujours de la même façon.
- **Commandes de cassette.** La ligne « Cassette, sur le vrai Apple-1 » sous l’éditeur est la commande du Woz Monitor à taper sur une vraie machine (`C100R`, puis les commandes `R` des plages) avant de jouer la bande. C’est une indication, pas quelque chose à taper dans l’émulateur.
- **Le clavier ne va qu’à un endroit à la fois.** La ligne en bas de la fenêtre dit où : l’Apple-1, l’éditeur, un champ. Cliquer sur l’écran de l’Apple-1 envoie les touches à la machine ; cliquer dans l’éditeur les reprend. Après un clic sur une erreur de la liste de l’éditeur, les touches reviennent à l’éditeur.
- **Débogueur en pause.** Les touches tapées pendant que la machine est en pause ne sont pas perdues, mais seule la **dernière** est gardée (le verrou du vrai clavier ne tient qu’un caractère) ; elle est livrée quand vous continuez.
- **Les points d’arrêt suivent le programme.** Charger un autre programme (fichier, bibliothèque, assembleur) efface les points d’arrêt et les étiquettes du précédent. Les points d’arrêt que vous posez ne sont retenus d’une session à l’autre que tant que vous ne chargez rien d’autre.
- **Coller avec le bouton du milieu.** Dans l’éditeur, un clic du milieu place le curseur là et colle la **sélection primaire** X11/Wayland (le texte que vous venez de surligner dans n’importe quelle application). L’éditeur n’*écrit* pas encore sa propre sélection dans la sélection primaire : pour copier depuis l’éditeur, utilisez <kbd>Ctrl</kbd>+<kbd>C</kbd>.
- **Petites fenêtres.** Sur un moniteur de moins de 1400 points de large, la disposition du premier lancement est éditeur et écran ; le menu **⊞ Disposition ▾** permet d’en choisir une autre. Sur un écran de 1024 × 768, la barre d’outils défile horizontalement : le bouton **»** rend le groupe Cassette accessible, au lieu de le couper à droite. Un zoom fixe (<kbd>Ctrl</kbd>+<kbd>+</kbd>) plus grand que le volet de l’écran est gardé, et l’écran défile dans son volet ; la barre d’état le dit.
- **Enregistrer un onglet sans titre.** **Enregistrer sous** (icône, <kbd>Ctrl</kbd>+<kbd>S</kbd> ou menu du clic droit de l’onglet) ne laisse jamais deux onglets pour le même fichier : un onglet déjà ouvert sur ce fichier est fermé.
- **Premier lancement et réseau.** Le premier démarrage contacte GitHub pour récupérer le projet de démonstration (chapitre 4) ; retirez le projet de la liste « Projets GitHub » de **⚙ Préférences** pour arrêter cela une fois pour toutes.
- **Lire le clavier avec le G2 Basic.** Une touche frappée pendant qu’un programme tourne l’arrête, comme avec l’original, et `INPUT` attend. Un jeu qui doit surveiller les touches appelle `KEY(0)` dans sa boucle principale (chapitre 15) ; dès qu’il l’a fait, une touche n’arrête plus le programme.
- **Étiquettes et points d’arrêt.** Un point d’arrêt sur une étiquette qui n’existe pas encore (tapé avant d’assembler ou de lancer) est gardé et posé quand l’étiquette apparaît.
- **Dispositions et fenêtres.** Une disposition enregistrée retient les panneaux ancrés, les fenêtres détachées (place et taille) et la taille de la fenêtre principale. Elle ne retient pas quelle fenêtre a le clavier.

# Licence et crédits {: .appendix }

**Retro Dev Kit Apple 1** est un logiciel libre, publié sous la **Licence publique générale GNU, version 3** (GPL-3.0), sans aucune garantie. Le texte complet est dans le fichier `LICENSE` de l’archive et du dépôt.

Certaines parties viennent d’autres projets et gardent leur propre licence, la licence **MIT**, compatible avec la GPL ; elles sont listées dans le fichier `NOTICE` :

- le **RDK G2 Basic**, le BASIC intégré, écrit pour ce projet (dépôt `Old-Wirehead/rdk-gen2basic-apple1`, MIT) ;
- les **tests GEN2 d’Uncle Bernie** (`gen2-test1` à `gen2-test4`, les barres) et les **glyphes de la GEN2**, fournis par Uncle Bernie.

Les jeux de la bibliothèque écrits par **Claude IA** (Le Pendu, Les Allumettes, La Bataille navale, Tunnel et Morpion) sont distribués avec le projet.

Ce que le kit ne contient **pas** : l’Integer BASIC d’Apple, les cassettes du commerce et les démos de la carte P-LAB. Elles restent à leurs propriétaires ; vous apportez vos propres copies.

La carte P-LAB est une création de P-LAB ; le TMS9918A est une puce de Texas Instruments. Ce projet n’est affilié ni à Apple, ni à Texas Instruments, ni à P-LAB.
