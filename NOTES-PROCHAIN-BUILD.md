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
   À préciser : nom de l'archive Windows, titre de la fenêtre, numéro de
   version (Cargo.toml), ou tout cela.
