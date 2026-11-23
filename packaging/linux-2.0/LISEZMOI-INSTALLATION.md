# Retro Dev Kit Apple 1 2.0 — installation (Linux x86_64)

1. Décompressez l'archive où vous voulez.
2. Lancez `./run.sh` (ou double-cliquez dessus).
3. Bibliothèque : `~/Documents/RDK librairies/Apple 1`, créée au premier
   lancement avec ses dossiers et les exemples du paquet, comme en v1.0
   (la barre d'état l'annonce). Le bouton 📥 **Exemples** y recopie les exemples du
   paquet (BASIC, assembleur, GEN2, P-LAB, art ASCII, cassette) sans rien
   remplacer ; le dossier `Biblio/` du paquet reste lu comme « ancienne
   bibliothèque ».
4. Facultatif : pour le BASIC d'origine, copiez **votre** copie du BASIC de
   l'Apple 1 (`integer_basic.bin`, 4096 octets) dans le dossier `ROM` de la
   bibliothèque (ou désignez-la par la variable `RDK_APPLE1_BASIC`), puis
   choisissez « ROM réelle ». Elle n'est pas fournie : elle n'est pas libre.
   Sans elle, le mode « Émulation » utilise RDK G2 Basic, intégré.

Configurations (liste de la barre d'icônes) : « Apple 1 moniteur vert »
(défaut), « Apple 1 moniteur vert + couleur GEN2 », « Apple 1 + P-LAB ».

Le kit range ses préférences (`preferences.json`, `apple1-rdk.cfg`) dans ce
dossier.

ROM fournies (comme en v1.0) : Woz Monitor, ROM de l'ACI, générateur de
caractères 2513. Licence : GPL-3.0 (fichier `LICENSE`).

La version 1.0 reste disponible sur
<https://github.com/Old-Wirehead/rdk-apple1/releases/tag/v1.0>.
