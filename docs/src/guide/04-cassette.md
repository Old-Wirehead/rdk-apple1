# Charger et lancer une cassette

Sur l’Apple-1, tout passe par la cassette : l’Integer BASIC lui-même se chargeait depuis une bande audio. L’émulateur lit les fichiers `.wav` et `.aiff` comme la carte ACI d’origine, octet par octet, et reconnaît les 24 cassettes de la collection « Apple1Cassettes » d’archive.org.

## Préparer la bibliothèque

Copiez vos cassettes dans le dossier de la bibliothèque, `~/apple1/programmes` : par exemple `Hamurabi.wav`. La liste de droite se met à jour toute seule. Les cinq dossiers BASIC, 6502, GEN2 COLOR, WAV et Art ASCII y sont listés directement, sans dossier racine au-dessus ; les fichiers laissés en vrac dans le dossier de la bibliothèque n’apparaissent pas. Chaque cassette reconnue porte la mention **✔ reconnu**, avec ses plages d’adresses et sa commande de lancement déjà réglées.

!!! note "Vous n’avez pas besoin de BASIC.wav"
    L’émulateur a son propre BASIC, le RDK G2 Basic : les programmes BASIC tournent donc sans aucune cassette. L’Integer BASIC d’Apple (1976) reste protégé par le droit d’auteur et n’est pas fourni : si vous préférez l’original, utilisez votre propre copie (par exemple `BASIC.wav`) et choisissez-la dans **⚙ Préférences → Avancé → BASIC**.

## Le plus rapide : le bouton ▶

Chaque programme de la liste a quatre petites icônes à sa droite : **▶** charge et lance, **⬇** charge en mémoire sans lancer, **📼** charge par la cassette, **⚙** ouvre ses paramètres. Un double-clic fait comme ▶.

Cliquez sur **▶** à côté de `Hamurabi.wav`. L’émulateur charge le BASIC s’il n’est pas déjà en mémoire, puis le programme, et tape pour vous `E2B3R` (retour au BASIC sans effacer le programme) et `RUN`. Le chargement est instantané.

![Hamurabi lancé par le bouton ▶ de la bibliothèque.](img/hamurabi-ecran.png "small")

Pour un programme en langage machine, ▶ fait un RESET puis tape sa commande de lancement, par exemple `300R`. Pour un listing BASIC ou un source 6502, ▶ l’ouvre aussi dans un onglet de l’éditeur, si l’éditeur est ouvert.

## Comme en 1976 : la cassette virtuelle

Le chargement est instantané, sauf si vous demandez la cassette. Cliquez sur l’icône **📼** d’un programme pour reproduire le vrai chargement : l’émulateur tape `C100R` pour démarrer le programme de l’ACI, puis la plage d’adresses, et la bande défile au rythme réel. Une barre de progression apparaît sous le nom du programme, et la zone **Cassette :** de la barre d’outils montre la bande et sa position.

![Chargement de BASIC.wav par l’ACI virtuelle, avec l’oscilloscope cassette ouvert.](img/brut/cassette-basic.png "large")

Les boutons 📈 et 🔊, au bout de la zone **Cassette :**, ouvrent l’**oscilloscope** et font **entendre** le signal, comme le haut-parleur d’un magnétophone. Le chargement du BASIC dure environ 36 secondes en vitesse réelle.

Quand la bande s’arrête, le moniteur affiche à nouveau `\`. Tapez `E000R` pour démarrer le BASIC. Son invite est un chevron `>` :

```mon
» E000R

E000: 4C
» >PRINT 6*7
42
```

!!! astuce "Lancer à la main"
    **📼 Insérer**, dans la zone **Cassette :** de la barre d’outils, met n’importe quel fichier `.wav` ou `.aiff` dans le lecteur virtuel. Il vous reste alors à taper `C100R`, puis la plage, par exemple `E000.EFFFR` pour le BASIC : la bande démarre à la première lecture, comme sur la vraie machine. Le bouton **Charger** sert aux fichiers `.txt`, `.bin` et `.hex`, chargés instantanément.

| Programme | Après le chargement |
|---|---|
| Integer BASIC | `E000R` (démarrage à froid, efface le programme en mémoire) |
| Programme BASIC | `E2B3R` (démarrage à chaud, garde le programme), puis `RUN` |
| Langage machine | son adresse suivie de `R`, souvent `300R` |
