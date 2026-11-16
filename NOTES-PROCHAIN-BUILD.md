# Notes pour le prochain build

Remarques de Wirehead à traiter au moment du prochain build, pas avant
(« une note n'est pas une tâche »). Chaque note est retirée de ce fichier
quand elle est traitée.

## 16/11/2026 (essais de la version Windows)

1. **Démarrer en plein écran.**
2. **Démarrer avec l'écran couleur et la carte GEN2 actifs.**

À préciser avec Wirehead au moment du build :
- Plein écran = vrai plein écran (comme F11) ? Aujourd'hui la fenêtre démarre
  déjà agrandie (`with_maximized` dans `crates/apple1-gui/src/main.rs`).
- Pour Linux et Windows, ou seulement pour Windows ?
- Seulement au premier lancement (réglages par défaut : `carte_couleur` vaut
  `false` dans `GuiConfig`), ou à chaque lancement même si l'utilisateur a
  changé le réglage ?

## 16/11/2026 (suite)

3. **Ajouter des exemples `.bas` dans le dossier BASIC de la bibliothèque.**
   Constat sur les captures de la version Windows : le dossier « BASIC »
   affiche « BASIC (0) », il est vide.
