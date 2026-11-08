# Installer sur Linux

## Décompresser et lancer

<ol class="steps">
<li><strong>Décompressez l’archive.</strong> Clic droit sur <code>apple1-emu.zip</code>, puis « Extraire ici ». Un dossier <code>apple1-emu</code> apparaît.</li>
<li><strong>Double-cliquez sur <code>run.sh</code>.</strong> Si le gestionnaire de fichiers demande quoi faire de ce fichier (par exemple sous Linux Mint), choisissez <strong>« Lancer »</strong> (et non « Afficher »). La fenêtre de l’Apple-1 s’ouvre, sans terminal.</li>
<li><strong>Facultatif : ajoutez l’émulateur au menu.</strong> Ouvrez un terminal dans le dossier (clic droit → « Ouvrir dans un terminal ») et tapez la commande ci-dessous. L’entrée <strong>Retro Dev Kit Apple 1</strong> apparaît dans la rubrique <em>Jeux</em> du menu, avec les commandes <code>apple1-emu-gui</code> (fenêtre) et <code>apple1-emu</code> (terminal).</li>
</ol>

```sh
./install.sh             # menu des applications + commandes dans ~/.local/bin
./install.sh --bureau    # idem, plus une icône sur le bureau
./install.sh --desinstaller
```

La désinstallation retire les programmes et l’entrée du menu, mais garde vos programmes (`~/apple1/programmes`) et vos réglages (`~/.config/apple1-emu`). Aucune commande ne demande `sudo` : tout est installé dans votre dossier personnel. Au premier lancement d’une nouvelle version, l’émulateur propose de repartir des réglages d’origine : la bibliothèque est conservée. Le bouton **Réinitialiser les préférences** fait la même chose. Un fichier vide nommé `portable` à côté du programme (ou `APPLE1_PORTABLE=1`) range préférences, bibliothèque par défaut et cache dans le dossier du kit.

!!! astuce "Mise à jour depuis une ancienne version"
    Les scripts `lancer.sh` et `installer.sh` des versions précédentes s’appellent maintenant `run.sh` et `install.sh`. Relancez `./install.sh` : il remplace l’ancienne installation, l’entrée du menu et l’icône du bureau qui pointaient vers `lancer.sh`.

!!! astuce "La version terminal"
    `./run.sh --terminal` ouvre l’émulateur dans un terminal (au moins 42 colonnes × 28 lignes), sans interface graphique. C’est pratique par une connexion SSH. Le Manuel complet décrit ses touches et ses commandes.

## Si la fenêtre ne s’ouvre pas

`run.sh` détecte un échec immédiat (pilote graphique, machine virtuelle sans accélération) et réessaie tout seul avec le rendu logiciel de Mesa (`LIBGL_ALWAYS_SOFTWARE=1`). Si cela échoue encore, un message s’affiche et le journal se trouve dans `~/.cache/apple1-emu/gui.log`.

| Symptôme | Que faire |
|---|---|
| Le gestionnaire de fichiers ouvre `run.sh` dans l’éditeur de texte | Clic droit → Propriétés → Permissions : cochez « Autoriser l’exécution », puis double-cliquez à nouveau et choisissez « Lancer ». |
| « Permission non accordée » dans le terminal | `chmod +x run.sh install.sh bin/*/apple1-*` |
| Fenêtre noire ou très lente | Normal en machine virtuelle : le rendu logiciel reste fluide pour l’écran de l’Apple-1. Désactivez au besoin l’« Écran vintage ». |
| Aucun son de cassette | Le son passe par ALSA ; sans carte son, il est simplement coupé. Le chargement fonctionne quand même. |

## Où sont rangés vos fichiers

| Emplacement | Contenu |
|---|---|
| `~/apple1/programmes` | La **bibliothèque** : vos cassettes, programmes et exemples (créée au premier lancement). |
| `~/.config/apple1-emu/gui.json` | Les réglages : zoom, couleurs, disposition des panneaux, onglets de l’éditeur. |
| `~/.cache/apple1-emu/` | Cassettes converties, projets GitHub téléchargés, journal `gui.log`. |
