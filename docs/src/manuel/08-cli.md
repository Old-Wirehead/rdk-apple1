# La ligne de commande

<p class="lead">Deux programmes : apple1-emu-gui, l’interface graphique, et apple1-emu, l’émulateur en terminal et la boîte à outils de conversion. Tout ce que font les boutons d’export se fait aussi en ligne de commande, pour vos scripts.</p>

## L’émulateur en terminal

`apple1-emu` sans commande lance l’Apple-1 dans le terminal (au moins 42 colonnes × 28 lignes) : 8 Ko de RAM plus `$E000`, ACI, vitesse réelle.

```sh
apple1-emu                                  # 8 Ko + E000, ACI, vitesse réelle
apple1-emu --fast                           # démarrer en mode rapide
apple1-emu --load samples/hello.txt --run 300
apple1-emu --tape samples/hello.wav         # cassette dans le lecteur virtuel
apple1-emu --basic ~/apple1/integer_basic.bin
apple1-emu --ram 32 --no-e000               # 32 Ko, pas de RAM en E000
```

| Option | Effet |
|---|---|
| `--ram KO` | RAM à partir de `$0000`, de 4 à 48 Ko (8 par défaut) |
| `--no-e000` | pas de RAM en `$E000-$EFFF` (le cavalier « W → E » de la carte) |
| `--no-aci` | pas de carte cassette |
| `--basic FICHIER` | votre Integer BASIC (binaire de 4 Ko ou vidage hex), chargé en `$E000` |
| `--load FICHIER[@ADR]` | un programme en mémoire (`.txt`, `.bin@ADR`, `.wav`, `.aif`) ; répétable |
| `--tape FICHIER` | une cassette dans le lecteur |
| `--type FICHIER` | tape un fichier texte au démarrage |
| `--run ADR` | tape `ADRR` après le chargement |
| `--fast`, `--no-throttle` | mode rapide ; affichage sans la limite de 60 caractères/s |
| `--no-refresh` | sans le modèle de rafraîchissement DRAM (1,0227 MHz au lieu de 0,960) |
| `--carte-couleur`, `--couleur-50hz` | la carte GEN2 (registres et mémoire ; l’image couleur n’existe que dans l’interface graphique) |
| `--cursor-hz HZ`, `--cursor-duty F` | clignotement du curseur (2,18 Hz, 2/3 visible par défaut) |
| `--monitor-rom`, `--monitor-page` | une autre image du Woz Monitor (256 ou 512 octets, page 0 ou 1) |
| `--aci-rom`, `--aci-page` | une autre image de la PROM de l’ACI |

**Touches du terminal.** <kbd>Ctrl</kbd>+<kbd>R</kbd> RESET, <kbd>Ctrl</kbd>+<kbd>L</kbd> CLEAR SCREEN, <kbd>F1</kbd> aide, <kbd>F2</kbd> ligne de commande, <kbd>F3</kbd> vitesse, <kbd>F4</kbd> limite d’affichage, <kbd>F5</kbd>/<kbd>F6</kbd> cassette lecture/stop, <kbd>F10</kbd> ou <kbd>Ctrl</kbd>+<kbd>Q</kbd> quitter. Un texte collé dans le terminal est tapé au clavier de l’Apple-1. Le retour arrière tape `_`, l’effacement du Woz Monitor.

**Commandes de F2.**

| Commande | Effet |
|---|---|
| {{charger}} `FICHIER[@ADR]` | charge en mémoire un `.txt`, un `.bin` (`@ADR` obligatoire), un `.wav` ou un `.aiff` |
| {{sauver}} `FICHIER DEBUT.FIN …` | sauve la mémoire ; l’extension choisit le format. Les plages peuvent être recopiées depuis la ligne ACI : `sauver st.aiff 004A.00FFR 0300.0FFFR` |
| {{cassette}} `FICHIER` | insère une cassette, qui démarre à la première lecture de l’ACI |
| `lecture`, `stop`, `rembobiner`, `ejecter` | le lecteur |
| {{enregistrer}} `FICHIER` | arme l’enregistreur ; tapez ensuite `C100R` puis `0300.03FFW` |
| `basic FICHIER` | charge votre Integer BASIC en `$E000` |
| `taper FICHIER` | tape un fichier texte au clavier |
| `reset`, `effacer`, `rapide`, `reel`, `debit`, `aide`, `quitter` | — |

## Les conversions

Le format de sortie est choisi par l’**extension** : `.aif`/`.aiff` donne un AIFF (profil smartphone), `.wav` un WAV, `.txt`/`.hex` un vidage Woz, `.bin` un binaire.

{{export}} (alias de `export-wav`) produit une cassette. Le signal est créé par la vraie ROM de l’ACI exécutée dans le 6502 émulé : les temps de bits sont ceux d’une Apple-1 qui ferait `W`.

```sh
apple1-emu export samples/hello.txt -o hello.aiff
apple1-emu export prog.bin@300 -o prog.wav --rate 48000 --bits 16
apple1-emu export startrek.txt --range "004A.00FFR 0300.0FFFR" -o startrek.aiff
```

| Option | WAV (défaut) | AIFF (défaut) |
|---|---|---|
| `--rate` | 44 100 Hz | 48 000 Hz |
| `--bits 8\|16` | 16 | 16 |
| `--channels 1\|2` | 1 | 2, identiques |
| `--amplitude` (0,05 à 1) | 0,8 | 0,89 (−1 dBFS) |
| `--silence S` | 0,5 s | 1,0 s |
| `--range` | une plage, ou toute une ligne ACI ; répétable | |
| `--no-refresh`, `--no-embed` | bits 6 % plus courts ; pas d’adresses dans le fichier | |

{{import}} (alias de `import-wav`) décode une cassette : `-o prog.txt` ou `-o prog.bin`, `--addr ADR` pour une cassette sans adresses, `--range` pour placer plusieurs enregistrements.

{{info}} décrit un fichier (enregistrements, adresses, durée) ; avec `--aci-input stock|mimeo|gen2`, `--volts` et `--series`, il simule l’entrée analogique de l’ACI (section 5.8).

{{convert}} `ENTRÉE SORTIE` convertit entre `.txt`, `.bin`, `.wav` et `.aiff`.

## BASIC et assembleur

```sh
apple1-emu basic jeu.bas -o jeu.wav            # listing tokenisé -> cassette
apple1-emu basic jeu.bas -o jeu.txt --ram 16   # vidage Woz, pour 16 Ko
apple1-emu basic jeu.bas                       # vérifier seulement
apple1-emu asm jeu.s -o jeu.wav                # source 6502 -> cassette
apple1-emu asm jeu.s -o jeu.bin -l jeu.lst     # binaire + listing
apple1-emu asm jeu.s -I ../lib                 # vérifier, includes en plus
```

{{basic (ligne de commande)|`basic`}} tokenise un listing comme l’interpréteur et écrit le bloc `$004A-$00FF` puis le programme ; `--ram 8|12|16|32` choisit la machine cible (par défaut, la plus petite qui suffit). {{asm (ligne de commande)|`asm`}} assemble une source ca65 (ou xa) ; sans `-o`, il vérifie seulement. Les deux affichent la ligne à taper sur le vrai Apple-1.

```text title="Sortie de apple1-emu asm hello.s -o hello.txt"
36 octets, $0300-$0323 ; 2 fichier(s), 13 symboles
Cassette (après C100R) : 0300.0323R   puis 300R
écrit : hello.txt
```

## Art ASCII

{{ascii}} `IMAGE` : `--cols`, `--rows` (23 pour garder l’invite), `--zoom`, `--cx`, `--cy`, `--brightness`, `--contrast`, `--gamma`, `--invert`, `--dither`, `--no-auto-level`, `--ramp simple|full|slideshow|LISTE`, `--format asm|basic`, `--title`, `-o` (`.txt`, `.bas`, `.s`, `.wav`, `.aif`).

## PROM

{{prom}} prépare des fichiers pour programmer de vraies PROM 256 × 4 (82S129, 3601, 6301), dans le format des vidages de quartets :

```sh
apple1-emu prom split woz -o proms/     # PROM-A2.bin (D7-D4) et PROM-A1.bin (D3-D0)
apple1-emu prom split aci -o proms/     # PROM-A3.bin (D7-D4) et PROM-A4.bin (D3-D0)
apple1-emu prom join A2.bin A1.bin      # réassembler une lecture et la comparer
```

`--swap-a1-a7` ne sert que pour une carte câblée d’après le schéma Apple de 1976, qui croise les lignes d’adresse 1 et 7 de la PROM A2.

Pour graver une EPROM 2716 à 27C512 ou une EEPROM 28C depuis l’interface graphique (programmateur TL866II+, T48 ou T56 avec `minipro`), voir le programmateur d’EPROM, section 2.6.

## L’interface graphique

`apple1-emu-gui [OPTIONS] [FICHIER…]` charge les fichiers donnés au démarrage. Options principales :

| Option | Effet |
|---|---|
| `--zoom N\|ajuster`, `--taille LxH` | zoom de l’écran, taille de la fenêtre |
| `--phosphore vert\|blanc\|ambre` | phosphore |
| `--screening`, `--sans-screening`, `--screening-options LISTE` | écran vintage de l’Apple-1 et ses réglages |
| `--gen2-screening` | écran vintage sur le moniteur couleur GEN2 |
| `--theme clair\|sombre\|systeme` | thème clair, sombre ou système |
| `--taille-icones auto\|petite\|normale\|grande\|tres-grande` | taille des icônes et du texte |
| `--barre-complete` | ne jamais réduire la barre d’outils aux icônes |
| `--bibliotheque DOSSIER`, `--basic FICHIER` | dossier de la bibliothèque, fichier du BASIC |
| `--carte-couleur`, `--sans-carte-couleur`, `--couleur-50hz` | carte GEN2 |
| `--langue CODE\|auto` | langue de l’interface |
| `--mode-rapide`, `--mode-cassette` | chargement de la bibliothèque : instantané (par défaut), ou par la cassette en vitesse réelle |
| `--cassette FICHIER` | insère une cassette |
| `--ecran-separe`, `--debogueur` | écran dans sa fenêtre, débogueur ouvert |
| `--editeur FICHIER`, `--ascii FICHIER` | ouvre un fichier dans l’éditeur ou l’outil ASCII |

Pour les démonstrations et les captures, des options rejouent des actions après quelques secondes : `--lancer-biblio S:NOM`, `--charger-biblio S:NOM`, `--parametres S:NOM`, `--preferences S` (et `--onglet-prefs interface|editeur|bibliotheque|cassette|gen2|avance`), `--editeur-lancer S`, `--editeur-recherche TEXTE` (barre de recherche ouverte), `--editeur-menu` et `--ecran-menu` (menus du clic droit ouverts), `--menu-etat` (le menu État ouvert), `--eprom` (le programmateur d’EPROM ouvert), `--fenetre-fichiers` (la fenêtre Charger ouverte), `--editeur-exporter S:FICHIER`, `--ascii-reglages R`, `--ascii-lancer S`, `--taper TEXTE`, `--taper-a S:TEXTE`, `--dbg-arret`, `--dbg-surveiller`, `--dbg-commande S:CMD`, `--capture IMAGE.png` (après `--delai S`) et `--quitter`.

# Dépannage et questions fréquentes

## La fenêtre ne s’ouvre pas

Le pilote OpenGL manque, ou vous êtes dans une machine virtuelle. `run.sh` relance automatiquement avec le rendu logiciel de Mesa ; à la main : `LIBGL_ALWAYS_SOFTWARE=1 ./apple1-emu-gui`. L’écran vintage reste fluide en rendu logiciel.

## Les touches n’arrivent pas à l’Apple-1

Cliquez sur l’écran de l’Apple-1. Quand un champ de texte ou une boîte de dialogue est actif, le clavier va à ce champ ; la barre d’état indique la destination du clavier.

## « Integer BASIC introuvable »

Le BASIC n’est pas fourni. Mettez votre `BASIC.wav` (ou un `.bin` de 4096 octets) dans la bibliothèque, ou désignez-le par le bouton **BASIC** ou **⚙ Préférences → Integer BASIC**. L’émulateur le cherche ensuite tout seul.

## Un programme BASIC s’arrête avec STOPPED AT

Une touche a été frappée pendant l’exécution : sur l’Apple-1, n’importe quelle touche interrompt un programme BASIC. Tapez `RUN` pour recommencer, ou `GOTO` suivi du numéro de ligne pour reprendre sans effacer les variables.

## Le programme ne se lance pas après le chargement

- Programme BASIC : il faut `E2B3R` (entrée à chaud) et non `E000R`, qui efface le programme. Le BASIC doit être en mémoire **avant** le programme.
- Langage machine : vérifiez l’adresse de lancement dans la fiche ⚙ ; ce n’est pas toujours `300R` (`280R`, `1000R`, `2000R`…).
- Programme de plus de 8 Ko : la RAM est ajoutée par ▶, mais pas par un chargement à la main. Lancez par ▶, ou utilisez `--ram 16`.

## La cassette ne se lit pas sur le vrai Apple-1

<ol class="steps">
<li>Volume au maximum, tous les effets audio coupés, aucune notification.</li>
<li>RETURN pendant l’en-tête, pas avant le début de la lecture ni après la fin de l’en-tête.</li>
<li>Les plages exactes, sur quatre chiffres : <code>004A.00FFR</code>, jamais <code>4A.FFR</code>.</li>
<li>Essayez un autre lecteur (VLC), puis <code>--rate 44100</code>, <code>--channels 1</code>, ou <code>--no-refresh</code>.</li>
<li>Vérifiez le niveau avec <code>apple1-emu info prog.aiff --aci-input stock</code> (section 5.8).</li>
<li>Si la lecture ne finit jamais : RESET, puis recommencez avec plus de volume.</li>
</ol>

## L’écran est plein de @ en inverse

C’est le moniteur couleur de la GEN2 : sa mémoire démarre à zéro, et le code `$00` est un `@` en inverse. Le programme n’a pas encore rempli la page texte. Sur la vraie carte, ce serait un contenu aléatoire.

## Les caractères sortent lentement

C’est normal : le terminal de l’Apple-1 affiche environ 60 caractères par seconde. Cliquez sur **Affichage ~60 car./s** dans la barre d’état (ou **Rapide** dans la rangée d’affichage de l’écran) pour un affichage instantané, et passez en vitesse rapide (<kbd>F3</kbd>) pour tout accélérer.

## Le son de la cassette est muet

Il est coupé en vitesse maximale, et quand ALSA ou la carte son manquent : l’oscilloscope l’indique (« muet en vitesse maximale », « pas de sortie son »). Le volume se règle dans l’oscilloscope ; 🔊 dans la barre d’outils coupe ou remet le son.

## Le projet GitHub ne se construit pas

« Tester le dernier build » compile avec ca65/ld65 (paquet `cc65`) s’il ne trouve ni Release ni artefact. Installez `cc65`, ou ajoutez un `apple1.toml` qui désigne le bon fichier. Certains sources demandent un ca65 récent (2.19 ou plus) ; l’erreur de ca65 s’affiche alors telle quelle.

## Ce que l’émulateur ne fait pas

- Les opcodes non documentés du 6502 sont traités comme des NOP de la bonne taille, et les opcodes JAM bloquent le processeur ; leurs effets exacts ne sont pas reproduits (le logiciel de l’Apple-1 ne les utilise pas).
- Le comparateur de l’ACI est idéal pendant l’émulation ; seule la commande `info` le simule en analogique.
- L’émulation a été comparée aux ROM, aux listings et aux outils indépendants (c2t, ca65, xa), mais les fichiers audio n’ont pas encore été joués sur un vrai Apple-1 par l’auteur de ce manuel.
