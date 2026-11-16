# Notes pour le prochain build

Remarques de Wirehead à traiter au moment du prochain build, pas avant
(« une note n'est pas une tâche »). Chaque note est retirée de ce fichier
quand elle est traitée.

## 16/11/2026 (essais de la version Windows)

1. **Démarrer en plein écran.**
2. **Démarrer avec l'écran couleur et la carte GEN2 actifs.**

Décisions de Wirehead (16/11/2026) :
- Plein écran = **fenêtre agrandie**, comme aujourd'hui : rien à changer
  pour le point 1 (la fenêtre démarre déjà agrandie, `with_maximized` dans
  `crates/apple1-gui/src/main.rs`). À vérifier sous Windows.
- Point 2 (GEN2 et écran couleur actifs) : **version Windows seulement**,
  **au premier lancement** (réglage par défaut ; un choix de l'utilisateur est
  ensuite gardé).

## 16/11/2026 (suite)

3. **Ajouter des exemples `.bas` dans le dossier BASIC de la bibliothèque.**
   Constat sur les captures de la version Windows : le dossier « BASIC »
   affiche « BASIC (0) », il est vide.
4. **Ajouter la mention « Beta » au nom du prochain build.**
   Décision de Wirehead : dans le **numéro de version**, pour la **version
   Windows seulement**. Attention : la version de Cargo.toml est commune à
   Linux et Windows ; il faudra donc ajouter « Beta » seulement dans le
   numéro affiché par la version Windows (« À propos », nom de l'archive),
   sans changer Cargo.toml. À confirmer avec Wirehead au moment du build.
5. **Écrire le nom de l'adaptateur graphique wgpu dans `gui.log`** (version
   Windows), pour prouver quel moteur est utilisé (carte graphique, ou
   « Microsoft Basic Render Driver » sans pilote 3D). Proposé par la session
   Claude de la VM.
6. **Interface allemande : le bouton « Schnell » passe à la ligne** dans la
   rangée d'affichage de l'écran monochrome (disposition par défaut, fenêtre
   de 1920 points de large). Constaté sur les captures allemandes du guide
   (`docs/img/de/defaut-annote.png`), 16/11/2026.
7. **Fermer le dernier onglet de l'éditeur fait disparaître le panneau de
   l'éditeur.** On peut le rouvrir par le menu « Disposition », mais il faut
   corriger. Constaté par Wirehead sur la
   version Windows (VM Windows 10), 16/11/2026. À vérifier aussi sous Linux.
8. **Son Windows (PR n°6) : il fonctionne sur la VM Windows 10, mais il
   saccade un peu.** D'après Wirehead, cela vient du chargement de la
   cassette, qui avance par à-coups (barre de progression). À regarder :
   régularité de l'émulation sur la VM, ou une file audio plus longue
   (QUEUE dans `audio.rs`). Constaté le 16/11/2026.
9. **La fenêtre « Dump » change de taille quand on la ferme puis qu'on la
   rouvre avec le bouton « Dump ».** Elle devrait revenir à sa taille
   d'avant. Constaté par Wirehead sur la version Windows, 16/11/2026. À
   vérifier aussi sous Linux.
10. **Dossier « 6502 » de la bibliothèque : aucun source `.s`, mais deux
    `.txt`** (`hello.txt` et `multi.txt`, des vidages hexadécimaux Woz).
    Comme le dossier BASIC (note 3), il faut y mettre des exemples (`.s`).
    Les `.txt` n'y ont pas leur place tels quels : les retirer, ou les
    renommer avec une extension qui dit ce qu'ils sont (`.dump` ? à décider
    avec Wirehead ; attention, l'émulateur charge aujourd'hui les vidages par
    l'extension `.txt`). Constaté par Wirehead, 16/11/2026 ; vérifié sur une
    bibliothèque neuve sous Linux (même contenu).
