# La disposition des panneaux

<p class="lead">Chaque panneau est un onglet que vous placez où vous voulez : côte à côte, en onglets, ou dans sa propre fenêtre sur un deuxième écran.</p>

## Déplacer un panneau

L’éditeur, l’écran de l’Apple-1, le moniteur couleur GEN2, le débogueur, le désassemblage, le dump, l’outil ASCII et la bibliothèque sont tous des **onglets ancrables**. Faites glisser un onglet par son titre : des repères apparaissent, et vous le déposez **à gauche**, **à droite**, **au-dessus**, **en dessous** d’un autre panneau, ou **dans le même groupe d’onglets**. Les séparations se tirent pour redimensionner. La disposition est retenue dans `gui.json`.

## La disposition de départ

L’éditeur est à gauche ; au centre, l’écran de l’Apple-1, avec le moniteur couleur dessous quand la carte GEN2 est branchée ; à droite, le débogueur avec le désassemblage dessous, puis le dump, et la bibliothèque tout à droite. Au tout premier lancement le choix se fait d’après la largeur du **moniteur** (la fenêtre s’ouvre petite et est agrandie un instant plus tard) : en dessous de 1400 points (un portable 1366 × 768, un écran 1024 × 768) elle démarre en **éditeur et écran**, la paire la plus utile ; à partir de 1400 points, avec tous les panneaux. Choisissez une autre disposition dans le menu à tout moment. Elle suit la largeur de la fenêtre : chaque panneau garde sa part, d’un portable à un grand moniteur. Le menu **⊞ Disposition ▾** de la barre d’outils la remet à tout moment (**Par défaut**, tous panneaux ouverts) et propose quatre autres dispositions toutes prêtes : **Jeu** (écran Apple-1 et bibliothèque), **BASIC** (éditeur et écran), **Assembleur** (éditeur, écran et débogueur) et **Débogage** (éditeur, débogueur, désassemblage et dump). **Enregistrer la disposition actuelle…** demande un nom et garde la disposition en cours, avec ses fenêtres détachées et la taille de la fenêtre principale (la charger rouvre ces fenêtres là où elles étaient) ; vos dispositions sont listées sous un séparateur, par ordre alphabétique, et un clic droit permet de les **Renommer** ou de les **Supprimer**. Elles sont retenues dans `gui.json`.

## L’assistant de démarrage

Au premier lancement, un assistant en trois écrans remplace l’ancienne fenêtre « Votre espace de travail ». Il est dessiné en ambre sur fond noir, avec l’effet de tube (balayage, halo, vignettage, courbure 0,07) sur le cadre ; le petit texte et les vignettes restent nets. Une petite croix **×** en haut à droite le ferme. **‹ Précédent** et **Suivant ›** passent d’un écran à l’autre ; le dernier écran a **Terminer**.

1. **Bienvenue** : ce que le kit permet de faire.
2. **ROM et BASIC** : le Woz Monitor, la ROM cassette et G2 Basic sont intégrés. Rien d’autre n’est obligatoire. Les puristes installent leur Integer BASIC original depuis une source externe ; le bouton ouvre le guide (chapitre « Installer une ROM »), qui ne contient que des liens, jamais les fichiers.
3. **Votre espace de travail** : les cinq dispositions (**Par défaut**, **Jeu**, **BASIC**, **Assembleur**, **Débogage**). Chaque vignette est cliquable (pas de bouton « Choisir »). Les rectangles gardent les numéros 1 à 6, et l’écran couleur (GEN2 / P-LAB) est le **7**, dans sa propre couleur, dans les vignettes qui l’affichent et dans la légende.

La case **Afficher cet assistant au démarrage** est sur les trois écrans, cochée par défaut. La même case est dans **⚙ Préférences → Interface**. **⊞ Disposition ▾ → Assistant de démarrage…** le rouvre. C’est une fenêtre ordinaire : elle ne reste pas au-dessus des autres. Elle ne s’ouvre pas quand la ligne de commande impose une taille (`--taille`), ni pour les captures.

## La taille du texte de chaque fenêtre

L’éditeur, la liste produite par l’assembleur, le débogueur, le désassemblage, le dump et la bibliothèque ont chacun un contrôle compact **A 13 ▲▼** dans leur barre : les triangles changent la taille, qui est retenue pour cette fenêtre. Ctrl+molette et Ctrl+plus / Ctrl+moins agissent sur la fenêtre qui est sous la souris (sur l’écran de l’Apple-1, Ctrl+plus / Ctrl+moins changent le zoom). **Réinitialiser les préférences** remet toutes ces tailles à 13.

## Ce qui est retenu d’un lancement à l’autre

La disposition est enregistrée toute seule quand vous quittez et retrouvée telle quelle : position et taille de chaque panneau, fenêtres détachées, zoom et disposition choisie. La fenêtre est rétablie à sa taille et à sa place, puis les panneaux reprennent les mêmes proportions, quel que soit l’écran : la restauration a été vérifiée à 2880 × 1620, 2560 × 1440, 1920 × 1080 et 1366 × 768, avec moins de 1 % de l’image différente après un relancement. Une disposition enregistrée sous un nom (**Enregistrer la disposition actuelle…**) garde en plus les fenêtres détachées. L’interface ne se zoome jamais toute seule : seul le réglage « Taille des icônes et du texte » change la taille des boutons.

## Quand l’écran est plus grand que son volet

Avec un **zoom fixe** (2x, 3x…, ou <kbd>Ctrl</kbd>+<kbd>+</kbd>), l’écran de l’Apple-1 garde la taille demandée, même si le volet est plus petit : il **défile** alors (molette ou barres) au lieu d’être réduit, et la barre d’état le dit, avec le conseil d’agrandir le volet. Seul **Ajuster** adapte le zoom au volet. Dans une fenêtre étroite (1024 × 768, par exemple), la barre d’outils défile horizontalement au lieu d’être coupée : le bouton **»** amène le groupe Cassette, un second clic revient au début. Le bouton **Lancer** de l’éditeur reste toujours visible.

## Détacher un panneau

Chaque panneau a un bouton **Détacher** en haut à droite (aussi par clic droit sur l’onglet) qui l’ouvre dans **sa propre fenêtre**, déplaçable sur un deuxième écran ; sa taille et sa place sont retenues. **⊟ Ancrer** le remet à sa place de départ.

- L’écran de l’Apple-1 et le moniteur couleur se détachent séparément. Le clavier va à l’Apple-1 depuis l’une ou l’autre de leurs fenêtres, et depuis la fenêtre principale. Fermer la fenêtre d’un écran le ré-ancre.
- Fermer la fenêtre d’un outil (éditeur, débogueur…) le ferme ; son bouton de la barre d’outils le rouvre au même endroit.
- Les boîtes de dialogue (Ouvrir, Enregistrer sous, Exporter) s’ouvrent toujours dans la fenêtre principale. Dans les fenêtres de fichiers, <kbd>Échap</kbd> ferme la fenêtre (même depuis le champ du nom), <kbd>Ctrl</kbd>+<kbd>L</kbd> modifie le chemin (son texte est sélectionné : taper le remplace) ; Enregistrer sous et Exporter d’un onglet démarrent dans le dossier de son fichier.
- Les dispositions enregistrées gardent les fenêtres détachées ; la fenêtre principale garde la même taille d’un lancement à l’autre, quel que soit le zoom.
- L’oscilloscope de la cassette reste une petite fenêtre flottante dans la fenêtre principale.

<div class="cards">
<div class="card"><h4>Pour jouer</h4>Fermez l’éditeur et le débogueur, gardez la bibliothèque. <strong>Ajuster</strong> agrandit l’écran ; <kbd>F11</kbd> passe en plein écran, avec l’écran vintage.</div>
<div class="card"><h4>Pour programmer</h4>Éditeur à gauche, écran au centre, débogueur et dump à droite. Détachez l’éditeur sur un deuxième moniteur pour lui donner toute la hauteur.</div>
<div class="card"><h4>Pour la GEN2</h4>Branchez la carte : le moniteur couleur se place sous l’écran de l’Apple-1. Détachez-le pour l’avoir en grand, comme un second moniteur.</div>
<div class="card"><h4>Pour présenter</h4><strong>Détachez</strong> l’écran de l’Apple-1 sur le vidéoprojecteur, en plein écran, la fenêtre principale sur votre portable.</div>
</div>

# Les préférences

<p class="lead">⚙ Préférences regroupe tous les réglages durables. Tout est retenu dans ~/.config/apple1-emu/gui.json (ou dans le dossier du kit, en mode portable).</p>

![L’onglet Interface des préférences : langue, thème, taille des icônes et du texte, barre compacte, bulles d’aide, fenêtre de démarrage et couleurs de l’interface.](img/preferences.png "large")

Les réglages sont répartis en six onglets :

| Onglet | Réglages |
|---|---|
| **Interface** | langue : automatique (selon la disposition du clavier), ou imposée : français, anglais, allemand, russe, espagnol, italien, portugais ; la langue détectée et sa source sont indiquées. Thème : sombre, clair ou système. Taille des icônes et du texte : petite, normale (par défaut), grande, très grande. Barre compacte (icônes seules quand la fenêtre est étroite). Bulles d’aide au survol. **Afficher cette fenêtre au démarrage** (la fenêtre « Votre espace de travail ») et le bouton qui l’ouvre tout de suite. **Couleurs de l’interface** : les couleurs des listes de l’interface, choisies **séparément pour le thème sombre et pour le thème clair** (boutons actifs, ligne courante du débogueur, ligne surlignée, cellule sélectionnée, ligne d’un point d’arrêt, adresses et colonne ASCII du dump, valeurs changées, étiquettes) ; chaque couleur a une pastille avec aperçu immédiat, les valeurs par défaut sont lisibles (contraste d’au moins 4,5:1 sur le fond du thème), et des boutons **Défaut** remettent un thème ou les deux. |
| **Éditeur** | couleurs de la syntaxe, pour le BASIC et pour l’assembleur 6502 séparément (chapitre 7) |
| **Bibliothèque** | le dossier principal (**Parcourir…**, **⟲ Par défaut**, **Ouvrir le dossier**), les autres dossiers, les projets GitHub et le jeton GitHub facultatif (seulement pour les dépôts privés et les artefacts Actions ; jamais affiché, et la variable `GITHUB_TOKEN` passe en priorité) ; position de la bibliothèque, à gauche ou à droite (à droite par défaut) |
| **Cassette** | son de la cassette (muet en vitesse rapide) et son volume, oscilloscope pendant la lecture ou l’écriture |
| **GEN2** | carte couleur d’Uncle Bernie, balayage 60 ou 50 Hz, palette LORES (chapitre 6) |
| **Avancé** | affichage et vitesse : afficher la bibliothèque, affichage limité à ~60 caractères/s comme le vrai terminal, vitesse rapide ; **Écran vintage : réglages fins** (communs aux deux écrans, chapitre 3) ; le BASIC chargé en `$E000` : le **RDK G2 Basic** intégré (par défaut) ou l’**Integer BASIC d’origine** (votre propre fichier, **📂 Choisir…** / **Oublier**) |

Chaque curseur des préférences (intensité de l’écran vintage, rémanence et courbure, volume de la cassette) affiche sa **valeur numérique** à côté de lui. Au premier lancement la courbure de l’écran vintage est réglée sur **0,07** (un léger bombé du tube) ; une installation existante garde sa valeur. L’aperçu de l’onglet **Éditeur** suit la langue de l’interface.

Le zoom, le phosphore et la case de l’écran vintage ne sont pas dans les préférences : ils sont dans la rangée d’affichage de chaque écran (chapitre 3).

![L’onglet Éditeur : une palette de couleurs pour le BASIC et pour l’assembleur 6502, ou vos propres couleurs.](img/preferences-editeur.png "large")

**La langue automatique.** L’émulateur suit la disposition du clavier active : `fr` donne le français, `us` ou `gb` l’anglais, `de`, `ch` ou `at` l’allemand, `ru` le russe, etc. Il l’interroge auprès de GNOME, de `setxkbmap`, de Cinnamon (cas de Linux Mint), de `localectl`, puis de `/etc/default/keyboard`, et enfin des variables `LANG`. En ligne de commande : `--langue fr|en|de|ru|es|it|pt|auto`.

!!! astuce "Repartir de zéro"
    **Réinitialiser les préférences**, en bas de la fenêtre, remet les réglages d’origine après confirmation (langue, thème, disposition, écran vintage…). Vos fichiers de bibliothèque restent en place ; les onglets de l’éditeur qui ne sont pas enregistrés sont fermés. Au premier lancement d’une nouvelle version du kit, la même question est posée, parce que `gui.json` survit quand on efface le dossier du programme pour en installer un autre.

    Pour un kit portable, placez un fichier vide nommé `portable` à la racine du kit (à côté de `run.sh`), ou lancez-le avec `APPLE1_PORTABLE=1` : les préférences (`config/gui.json`), la bibliothèque par défaut (`programmes`) et le cache sont alors dans le dossier du kit.

    Fermer l’émulateur et renommer `~/.config/apple1-emu/gui.json` revient au même que le bouton, au lancement suivant. Les fiches des programmes restent dans les `programs.json` de chaque dossier.
