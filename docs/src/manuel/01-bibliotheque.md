# La bibliothèque

<p class="lead">La bibliothèque est le point de départ de presque tout : elle range vos cassettes, vos vidages et vos sources, reconnaît les cassettes d’époque, et lance un programme d’un clic, BASIC compris.</p>

![Le panneau de la bibliothèque : le dossier principal, le dossier « Art ASCII », des cassettes reconnues et un projet GitHub. BASIC.wav est en cours de lecture.](img/bibliotheque.png "small")

## Les dossiers

Le dossier principal est `~/apple1/programmes`. Il est créé au premier lancement, avec les exemples rangés dans cinq dossiers : **Art ASCII** (onze exemples d’art ASCII 40 × 24, chapitre 9), **BASIC** (programmes `.bas` et `.hex.txt` BASIC), **6502** (sources `.s`/`.asm` avec leurs `.bin`/`.hex`), **GEN2 COLOR** (les tests de la carte GEN2 de Bernie, `gen2-*.apl`, et les jeux couleur pour G2 Basic) et **WAV** (toutes les cassettes). Dans le panneau, ces cinq dossiers sont listés directement au premier niveau, dans l’ordre **BASIC**, **6502**, **GEN2 COLOR**, **WAV**, **Art ASCII** : le dossier racine (le nom du répertoire sur le disque) n’apparaît pas au-dessus d’eux, et les fichiers laissés en vrac dans ce dossier ne sont pas montrés. Rangez-les dans l’un des cinq dossiers. Un dossier que vous ajoutez, ou un projet GitHub, garde son propre titre. Une bibliothèque qui existe déjà n’est jamais réorganisée, sauf que ses dossiers **Assembleur** et **Uncle Bernie** (les noms des versions 7.8 à 7.9.1) sont renommés **6502** et **GEN2 COLOR**, leur contenu étant conservé. Pour en choisir un autre : **⚙ Préférences → Dossier de la bibliothèque → Parcourir…** (**Par défaut** revient à `~/apple1/programmes`).

**+ Dossier**, en haut du panneau, ajoute d’autres dossiers ; le dossier principal reste toujours là. La bibliothèque est un arbre : dossiers et sous-dossiers se replient, et l’état est mémorisé. Chaque dossier garde ses propres réglages dans un fichier `programs.json`. Clic droit sur un dossier ajouté : l’ouvrir dans le gestionnaire de fichiers, ou le retirer de la liste (rien n’est effacé sur le disque).

**Panneau étroit.** La bibliothèque peut être aussi étroite que vous voulez (jusqu’à une largeur minimale, en dessous de laquelle le séparateur s’arrête). Chaque entrée s’adapte alors : le nom prend toute la largeur, coupé par « … » s’il est trop long (la bulle d’aide montre le nom complet), les petites icônes ▶ ⬇ 📼 ⚙ passent à la ligne suivante, et la description est limitée à une ou deux lignes, également coupées par « … », le texte complet étant dans la bulle d’aide. Dans l’en-tête, **+ Dossier** et **+ Projet GitHub** deviennent des icônes avec bulle d’aide.

Les fichiers montrés sont : `.wav .aiff .bin .hex .txt .apl .bas .s .asm .prg`. La liste se met à jour toute seule, toutes les deux secondes : copiez un fichier dans le dossier, il apparaît.

Pour chaque programme, la liste indique le nom, le type, les plages d’adresses, la durée de la cassette, la commande de lancement et vos notes.

### Les jeux de Claude IA

Les dossiers **BASIC** et **6502** reçoivent aussi cinq petits jeux écrits par Claude IA, chacun en version française et anglaise : *Le Pendu* (`pendu.bas`, `hangman.bas`), *Les Allumettes* (`allumettes.bas`, `matches.bas`) et *La Bataille navale* (`bataille.bas`, `battleship.bas`) en BASIC ; *Tunnel* (`tunnel.s`, `tunnel-en.s`) et *Morpion* (`morpion.s`, `tictactoe.s`) en assembleur 6502. Un double-clic sur un jeu BASIC le lance avec le RDK G2 Basic (l’émulateur démarre le BASIC, charge le listing et tape `RUN`) ; les sources en assembleur sont assemblées par l’assembleur intégré. Elles incluent `apple1.inc`, installé avec elles. Le dossier **GEN2 COLOR** contient aussi *Puissance 4 en couleur* (`puissance4-couleur.bas`), un jeu pour le G2 Basic et la carte GEN2, signé Claude IA : Q et S déplacent le pion au-dessus de la grille, Espace le lâche. Les jeux font partie du projet et sont sous sa licence (annexe « Licence et crédits »).

## Lancer un programme

Chaque programme a quatre petites icônes à sa droite :

| Icône | Action |
|---|---|
| **▶** | **Charger et lancer** : charge le programme s’il ne l’est pas, puis le lance : RESET, puis sa commande tapée au clavier (`300R`, ou `E2B3R` et `RUN` pour un programme BASIC). Si le programme a besoin de l’Integer BASIC et qu’il n’est pas en `$E000`, l’émulateur le charge d’abord. Un listing BASIC est tapé dans l’Integer BASIC, un source 6502 est assemblé. |
| **⬇** | **Charger** : met le programme en mémoire, instantanément, sans le lancer. |
| **📼** | **Par la cassette** : comme sur la vraie machine. L’émulateur fait RESET, tape `C100R` puis la commande de lecture de l’ACI (par exemple `0300.031ER`), et la cassette défile en vitesse réelle, avec son son et une barre de progression. Les `.txt` et `.bin` sont d’abord convertis en cassette. Grisée pour un source (`.bas`, `.s`), qui n’a pas d’enregistrement. |
| **⚙** | **Paramètres** : plages, adresse de chargement d’un `.bin`, adresse et commande de lancement, type, besoin du BASIC, notes. |

{{!double-clic}}Le chargement est **instantané**, sauf si vous choisissez la cassette avec 📼. Un **double-clic** sur un programme le charge, et le lance s’il a une commande de lancement (comme ▶). La cassette ne sert que si vous la demandez ; le lecteur de la barre d’outils montre alors la bande et sa position.

Quand l’éditeur est ouvert, ▶, ⬇ ou un double-clic sur un listing BASIC ou un source 6502 l’**ouvre aussi dans un onglet de l’éditeur**, marqué 📚 ; s’il est déjà ouvert, son onglet est simplement affiché.

Le **clic droit** sur un programme propose le reste : charger, charger et lancer, insérer comme cassette sans rien taper, paramètres, renommer, supprimer (avec confirmation), ouvrir le dossier.

## La fiche d’un programme

![La fiche ⚙ de Hamurabi : deux plages (page zéro du BASIC, puis le programme), lancement par E2B3R puis RUN.](img/brut/bibliotheque-params.png "large")

| Champ | Rôle |
|---|---|
| **Plages** | Les zones de mémoire, écrites comme pour l’ACI : `0300.031E`, ou `004A.00FF 0800.0FFF` pour un programme BASIC. Chaque enregistrement de la cassette va dans sa plage, dans l’ordre. |
| **Adresse de chargement** | Pour un `.bin`, qui ne contient pas d’adresse. |
| **Lancement** | L’adresse et la commande tapée par ▶. `\n` représente RETURN entre deux lignes : `E2B3R\nRUN`. |
| **Type** | **Langage machine**, **Programme G2 Basic** (le choix par défaut pour un listing BASIC, puisque l’émulateur le fournit) ou **Programme Integer BASIC** (pour ceux qui préfèrent l’original d’Apple : ▶ charge alors l’Integer BASIC d’origine choisi dans les préférences). Le type choisi pour un programme l’emporte sur le réglage général du BASIC des préférences. |
| **A besoin d’un BASIC** | ▶ chargera d’abord le BASIC du type du programme. |
| **Notes** | Texte libre, affiché dans la liste. |

Les valeurs sont préremplies d’après le fichier et vérifiées à l’enregistrement ; **Valeurs du fichier** les relit.

!!! attention "Toujours quatre chiffres : 004A.00FF, jamais 4A.FF"
    La fiche refuse `4A.FF`. Ce n’est pas un caprice : la ROM de l’ACI n’efface pas son accumulateur au point, et lirait `$004A-$4AFF` (voir la section 5.6). L’émulateur écrit donc toujours les adresses sur quatre chiffres.

## Les cassettes reconnues

Quand un fichier est **reconnu**, sa fiche est remplie toute seule et la liste affiche **✔ reconnu**. L’émulateur décode les enregistrements et compare leur taille et leur empreinte SHA-256 à une table intégrée : les 24 cassettes de la collection « Apple1Cassettes » d’archive.org. Le nom du fichier n’a alors aucune importance. À défaut, il reconnaît le **nom** (majuscules, espaces et tirets ignorés) avec les mêmes tailles d’enregistrements ; la bulle d’aide signale alors que le contenu diffère.

| Cassette | Plages | Lancement |
|---|---|---|
| `BASIC.wav` (Integer BASIC) | `E000.EFFF` | `E000R` |
| `Hamurabi.wav`, `StarTrek.wav`, `Wumpus.wav`, `Slots.wav`, `Craps.wav`, `Bowling.wav`, `Buzzword.wav`, `WordCross.wav`, `AppleiEnhancedCheckers.wav`, `LunarlanderasciiGraphics.wav` | `004A.00FF 0300.0FFF` | `E2B3R ⏎ RUN` |
| `AceyDucey.wav`, `Blackjack.wav`, `HiLo.wav` | `004A.00FF 0800.0FFF` | `E2B3R ⏎ RUN` |
| `Football.wav` | `004A.00FF 0400.2FFF` | `E2B3R ⏎ RUN` (12 Ko) |
| `DealOrNoDeal.wav`, `StarTrek2003.wav` | `004A.00FF 0300.3FFF` | `E2B3R ⏎ RUN` (16 Ko) |
| `APPLE30TH.wav` | `0280.0FFF` | `280R` |
| `ASMmchess.wav` | `0300.0BFF` | `300R` |
| `Microchess.wav` | `1000.18FF` | `1000R` |
| `LunarLander.wav` | `0300.09FF` | `300R` |
| `Mastermind.wav` | `0300.03FF` | `300R` |
| `Pasart.wav` | `0300.05FF` | `300R` |
| `Life.wav` | `2000.21FF` | `2000R` (12 Ko) |

Vos propres réglages ne sont jamais écrasés : un programme que vous avez réglé vous-même garde sa fiche, seul l’indicateur « reconnu » apparaît. Les programmes qui dépassent les 8 Ko de base reçoivent automatiquement la RAM nécessaire au lancement, comme avec une carte d’extension.

**Cassette inconnue.** L’émulateur devine ce qu’il peut : un enregistrement de 4096 octets qui commence par `JMP` est l’Integer BASIC ; deux enregistrements dont le premier fait 182 octets (`$4A-$FF`) sont un programme BASIC, placé d’après ses pointeurs LOMEM et HIMEM. La liste indique alors « (devinées) » ; vérifiez avec ⚙. Un programme en langage machine sans adresses n’est pas deviné : indiquez sa plage.

![Numériser une vraie cassette : de la bande au fichier, puis à la bibliothèque.](diagrams/workflow-capture.svg "diagram large")

## Les projets GitHub

**+ Projet GitHub** accepte une adresse (`https://github.com/jefftranter/6502/tree/master/asm/jmon`) ou simplement `propriétaire/dépôt`. Les fichiers utiles sont copiés dans `~/.cache/apple1-emu/github/<propriétaire>/<dépôt>/<commit>/`, et le projet devient un nœud de l’arbre, avec son commit, sa date et son message. **Mettre à jour** va chercher le dernier commit ; 🌐 ouvre la page du dépôt. Tout le réseau se fait en arrière-plan.

{{Tester le dernier build}} essaie, dans l’ordre :

<ol class="steps">
<li>les fichiers de la dernière <strong>Release</strong> du dépôt (<code>.wav</code>/<code>.aiff</code>, puis <code>.bin</code>, puis <code>.hex</code>/<code>.txt</code>) ;</li>
<li>le dernier <strong>artefact GitHub Actions</strong> (un jeton est nécessaire) ;</li>
<li>la <strong>compilation des sources</strong> : un listing BASIC est tapé dans l’Integer BASIC puis lancé par <code>RUN</code> ; un source 6502 est assemblé avec <strong>ca65/ld65</strong> (paquet <code>cc65</code>), avec <code>-D APPLE1</code> par défaut ; un vidage Woz est chargé.</li>
</ol>

![Un projet GitHub, de l’adresse du dépôt au programme lancé.](diagrams/workflow-github.svg "diagram large")

Un fichier `apple1.toml` (ou `apple1.cfg`) à la racine du dépôt précise ce qu’il faut construire :

```toml title="apple1.toml"
file    = "src/jeu.s"     # programme à lancer
load    = $0300           # adresse de chargement (sinon le .org du source)
run     = $0300           # adresse de lancement (sinon celle de chargement)
ld65    = "apple1.cfg"    # configuration ld65 (sinon -t none)
defines = "APPLE1"        # symboles -D pour ca65
```

!!! astuce "Le jeton GitHub est facultatif"
    Les dépôts publics n’en demandent pas. Un jeton (variable `GITHUB_TOKEN`, ou **⚙ Préférences**) relève la limite de requêtes et donne accès à vos dépôts privés et aux artefacts. Il n’est jamais affiché ni écrit dans les messages, et `gui.json` passe alors en lecture réservée à votre compte.

## Le dossier « Art ASCII »

La bibliothèque contient aussi un dossier **Art ASCII** : des dessins au format de l’écran (pomme, carte Apple-1, puce 6502, cassette, fusée…) et des portraits de Steve Wozniak et Steve Jobs convertis depuis des photos libres. Un `.txt` d’art ASCII montre un aperçu quand il est choisi, avec **→ BASIC**, **→ 6502** et **🎨 Ouvrir dans l’outil** ; **▶** lance tout de suite sa version 6502. Le chapitre 9 décrit l’outil.
