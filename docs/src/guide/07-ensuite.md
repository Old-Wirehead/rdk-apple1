# Et ensuite ? {: .newpage }

Vous savez maintenant charger une cassette, écrire et lancer un programme BASIC, et le transférer sur une vraie machine. Le **Manuel complet** (`docs/Manuel-complet.pdf`) va beaucoup plus loin :

- **Guide de l’Integer BASIC** : toutes les instructions avec des exemples, les limites, les messages d’erreur, les bogues connus et leurs contournements, l’adaptation de programmes Applesoft.
- **Guide de l’assembleur 6502** : l’assembleur intégré (syntaxe ca65, macros, listing), les routines du Woz Monitor, des exemples testés dans l’émulateur.
- **Des jeux à jouer et à lire** : la bibliothèque contient cinq petits jeux de Claude IA (Le Pendu, Les Allumettes, La Bataille navale en BASIC ; Tunnel et Morpion en assembleur), chacun en français et en anglais.
- **Les outils** : débogueur, bibliothèque et projets GitHub, carte couleur GEN2 et carte graphique P-LAB, photo → ASCII, écran vintage, ligne de commande.

## Raccourcis essentiels

| Touche | Effet |
|---|---|
| Ctrl+R | RESET (retour au moniteur) |
| Ctrl+L | CLEAR SCREEN |
| F3 | vitesse réelle ou maximale |
| Ctrl+V | tape le presse-papiers sur le clavier de l’Apple-1 (aussi : clic droit sur l’écran → Coller) |
| Ctrl+F, Ctrl+H, Ctrl+G | dans l’éditeur : rechercher, remplacer, aller à une ligne |
| F5, F9 | dans l’éditeur : lancer l’onglet, point d’arrêt sur la ligne |
| Ctrl+plus, Ctrl+moins, Ctrl+0 | zoom de l’écran, et zoom ajusté à la fenêtre (dans l’éditeur : taille du texte) |
| F11 | plein écran |
| F1 | bulles d’aide détaillées (comme le bouton ?) |
| Retour arrière | `_`, l’effacement du moniteur |

## Les commandes à retenir

| Commande | Effet |
|---|---|
| `E000R` | démarre l’Integer BASIC (à froid : efface le programme) |
| `E2B3R` | revient au BASIC sans effacer le programme |
| `C100R` | démarre le programme de l’ACI (invite `*`) |
| `debut.finR` | lit la cassette dans cette plage, par exemple `0300.031ER` |
| `300R` | lance un programme en langage machine en `$0300` |
