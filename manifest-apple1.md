# Manifeste — RDK Apple 1 (spécificités)

> Rempli à partir du modèle de RDK-Interface-Commune 0.1.4 (`modeles/manifest-modele.md`), mis à jour pour la 0.1.8 (épinglée : `fbd1dcc`, fusion de la PR n° 11 ; 0.1.7 : PR n° 10). Ne décrit que ce qui diffère des défauts de l'interface commune. Code : `crates/apple1-rdk`.
>
> Rédigé le : 17/11/2026 · Auteur : Rusty Sr IA, complété par Claude (langues, son, oscilloscope, réglages par programme, GitHub, état, zoom) · Statut : brouillon, à valider par Wirehead
> Mentions : **fixé** (décidé par Wirehead), **proposé, à valider**.

## 0. Règle générale

- Machine de référence : rdk-apple1 **v1.0** « Original Sin » (étiquette `v1.0`, intouchée). La machine émulée est son noyau `apple1-core`, réutilisé tel quel.
- Dépôts de référence en lecture seule : `rdk-apple1-archive` (privé), RDK-Interface-Commune.
- Interdits : aucune ROM hors Woz Monitor, ACI et 2513 ; jamais le BASIC d'origine ; pas de documents de Bernie ni d'images tierces ; manuels de la v1.0 non modifiés (**fixé**, 17/11/2026) ; Linux seulement en 2.0 (**fixé**).

## 1. La machine en bref

| Élément | Valeur |
|---|---|
| Processeur | 6502 NMOS à 1,023 MHz → liste commune de registres |
| Mémoire | RAM 8 Ko en `$0000-$1FFF` + 4 Ko en `$E000-$EFFF` ; ACI `$C000-$C1FF` ; PIA `$D010-$D013` ; Woz Monitor `$FF00-$FFFF` |
| Vidéo | terminal 40 × 24 caractères, majuscules, ROM 2513, curseur `@` clignotant, ~60 car./s |
| Clavier | ASCII 7 bits ; clavier dessiné **QWERTY générique sans pavé numérique** (**fixé**) |
| Langages | Integer BASIC : RDK G2 Basic intégré (Émulation, défaut de la v1.0) ou BASIC d'origine de l'utilisateur (ROM réelle) ; assembleur 6502 (syntaxe ca65 de la v1.0) |
| Périphériques | cassette ACI ; programmateur d'EPROM (minipro) ; cartes GEN2 (couleur) et P-LAB (TMS9918A) de la v1.0 |
| Configurations | « Apple 1 moniteur vert » (défaut), « Apple 1 moniteur vert + couleur GEN2 », « Apple 1 + P-LAB » |

## 2. Pilote `clavier-editeur`

Défaut commun (tous les caractères passent). Le BASIC et l'assembleur sont mis en majuscules au moment de l'envoi à la machine.

## 3. Pilote `clavier-machine`

| Touche hôte | Touche de la machine | Remarque |
|---|---|---|
| lettre | majuscule (`$41-$5A`) | l'Apple 1 n'a pas de minuscules |
| chiffres, ponctuation | ASCII | |
| Entrée | CR `$0D` | |
| Retour arrière, Suppr | `_` | effacement du Woz Monitor |
| Échap | `$1B` | annule la ligne |
| Ctrl + lettre | code de contrôle (`$01-$1A`) | |
| flèches, F1…, Tab | rien | absentes de l'Apple 1 |

### 3 bis. Fenêtre Clavier

- Titre : « Clavier QWERTY générique (sans pavé numérique) — Apple 1 ».
- Rangées : Échap · `` ` `` 1-0 - = ⌫ · Tab Q-P [ ] \ · Verr. Maj A-L ; ' Entrée · Maj Z-M , . / Maj · Ctrl Alt Espace Alt Gr Ctrl.
- Maj et Ctrl du clavier dessiné sont **rémanents** (s'appliquent à la touche suivante).
- Dessin fourni par `manifeste_clavier` (pas de dessin personnalisé) ; fenêtre ouverte au démarrage sous l'écran monochrome : **oui**.
- Double-clic qui charge (`double_clic_charge`) : `.woz`, `.dump`, `.hex`, `.wav`, `.aif(f)`, `.bin`, `.prg`, `.mon`, `.apl`, `.txt` (vidage Woz ou art ASCII), photos.
- Coller (Ctrl+V) : le texte est tapé dans l'Apple 1, comme en v1.0.

## 4. Moniteurs et pilotes vidéo

| Moniteur | Pilote | Détails |
|---|---|---|
| Monochrome | `video-numerique` | terminal 280 × 192 points (40 × 24 cases de 7 × 8), ROM 2513 de la v1.0 ; image calculée à 3 fois (840 × 576) |
| Couleur | `video-couleur` | carte GEN2 (280 × 192, LORES/HIRES/TEXTE/MIXED) ou P-LAB (256 × 192), agrandie 3 fois ; pas d'image sans carte |

- Phosphore (0.1.6, `phosphore_monochrome`) : vert (défaut), blanc ou ambre, couleurs de la v1.0 ; la partie commune teinte le moniteur monochrome.
- Écran vintage (effets CRT de la v1.0, **oui**) : lignes de balayage, rémanence (selon le phosphore), halo, points adoucis, tube bombé et coins assombris, scintillement, masque RVB (moniteur couleur). Bouton 📺 et fenêtre 🔆 Écran ; calculé par la machine.

## 5. Mémoire

| Zone | Adresses | Lecture | Écriture |
|---|---|---|---|
| RAM | `$0000-$1FFF`, `$E000-$EFFF` | oui | oui |
| ACI | `$C000-$C0FF` E/S, `$C100-$C1FF` ROM | ROM : oui ; E/S : `$FF` (sans effet de bord) | non (`LectureSeule`) |
| PIA | `$D010-$D013` (miroirs) | état sans effet de bord | non |
| Woz Monitor | `$FF00-$FFFF` | oui | non |

Désassemblage : départ `$FF00`, puis l'adresse du dernier programme assemblé ou chargé ; 6502 NMOS ; étiquettes du Woz Monitor, de l'ACI, du PIA et du programme.

## 6. Groupe Périphériques

| Identifiant | Icône | Libellé | Effet |
|---|---|---|---|
| `aci-enregistrer` | ⏺ | Sortie ACI | 1er clic : enregistre la sortie cassette ; 2e clic : écrit `Cassettes/sortie-aci-<horodatage>.wav` dans la bibliothèque |
| `chargement-cassette` | 📼 | Chargement cassette | allumé : Charger et ▶ passent par l'ACI en temps réel (RESET, `C100R`, lecture) ; éteint : ⚡ rapide |
| `sauver` | 💾 | Sauver | fenêtre : plages → `.woz`, `.wav`, `.aiff`, Intel `.hex`, `.dump`, `.bin`, `.prg` |
| `gen2` (+ `gen2-50hz`, `accolades`) | 🌈 | GEN2 | carte couleur d'Uncle Bernie ; suit la configuration |
| `plab` (+ `plab-nmi`, `codetank`, `source-couleur`) | 🎮 | P-LAB | TMS9918A, NMI, CodeTank (`P-LAB/` de la bibliothèque) ; suit la configuration |
| `ecran-vintage` | 📺 | Écran vintage | effets CRT |
| `reglages-ecran` | 🔆 | Écran | fenêtre : phosphore et réglages CRT |
| `photo-ascii` | 🎨 | Photo → ASCII | fenêtre de la v1.0 : photo ou `.txt` → art ASCII → BASIC / 6502, cassette, `.txt` |
| `exemples` | 📥 | Exemples | copie les exemples du paquet et l'art ASCII de la v1.0 dans la bibliothèque (sans rien remplacer) |
| `son-cassette` | 🔊 | Son | son de la cassette (lecture 📼 et écriture ⏺, vitesse réelle) par ALSA, comme la v1.0 ; volume dans l'oscilloscope |
| `oscilloscope` | 📈 | Oscilloscope | fenêtre de la v1.0 : le signal sous la tête, ouverte quand la cassette avance |
| `reglages-programme` | ⚙ | Réglages | réglages par programme de la v1.0 (`programs.json` du dossier du programme : plages, adresses, commande de lancement, BASIC) ; s'ouvre sur le dernier programme chargé, sélecteur pour un autre |
| `github` | 🐙 | GitHub | projets GitHub de la v1.0 : téléchargés dans `GitHub/` de la bibliothèque, ⟳ Mettre à jour, ▶ Tester le dernier build ; jeton : `GITHUB_TOKEN` ou saisi pour la session (jamais écrit) |

Programmateur de PROM : **oui**. Fenêtre de la v1.0 (minipro : lire, écrire, vérifier ; puce ; mémoire ou fichier), fichiers en **Intel HEX** dans `PROM/` ; sans fenêtre : image `$E000-$EFFF` en `PROM/prom-E000-EFFF.hex`. Les fenêtres de la machine sont dessinées par la machine (contexte egui reçu par `OptionsLancement::preparer`).

## 7. Bibliothèque

- Dossier (0.1.5) : `Documents/RDK librairies/Apple 1` ; l'ancien `Biblio/` du kit reste lu.
- Dossiers en plus : `Cassettes`, `PROM`, `GEN2 COLOR`, `P-LAB`, `Art ASCII`.
- Formats (0.1.5) : `.woz` (dump du Woz Monitor, lecture et écriture), `.txt`/`.mon`/`.apl` (anciens dumps, lecture seule), `.dump` et Intel `.hex` communs (un ancien `.hex` au format Woz reste lu), `.wav`/`.aif(f)` (ACI), `.bin`, `.prg`, `.s` (assembleur), photos (art ASCII).
- ROM (0.1.5) : Woz Monitor, ACI et 2513 intégrés ; seul `integer_basic.bin` (4096 octets, facultatif, variable `RDK_APPLE1_BASIC`) est cherché dans `ROM/`, identifié par sa signature. ROM réelle = ce BASIC ; Émulation = RDK G2 Basic.
- Charger : sources `.bas`/`.asm`/`.s` → ⬇ RAM ; programmes → mémoire (RAM étendue au besoin, sauf GEN2) ou cassette 📼.
- Play : RESET, chargement, puis `xxxxR` (adresse de lancement) ou, pour un programme BASIC (page zéro `$004A`), `E2B3R` puis `RUN`.
- Programmes reconnus de la v1.0 (empreinte SHA-256) : adresse de lancement et notes.

### 7 bis. Éditeur → machine

| Pilote | Bouton | Effet |
|---|---|---|
| `editeur-machine` | ▶ Lancer | BASIC : tokenisé, rangé avec sa page zéro (LOMEM/HIMEM pour 8 Ko), BASIC de l'utilisateur rangé en `$E000` si besoin, puis `E2B3R` et `RUN`. Assembleur : assemblé, rangé, puis `xxxxR` |
| | ⏸ / ⏹ | pause ; ⏹ = RESET (pas de BREAK sur l'Apple 1) |
| `editeur-memoire` | ⬇ RAM | BASIC ou code assemblé, sans lancer |

## 8. Syntaxe

| Pilote | Langage | Erreurs | Couleurs |
|---|---|---|---|
| `syntaxe-basic` | BASIC entier de l'Apple 1 | tokeniseur de la v1.0 (commandes immédiates refusées, nombres, guillemets…) | n° de ligne, mots clés, variables, nombres, chaînes, REM |
| `syntaxe-asm` | 6502, ca65 de la v1.0 | assembleur de la v1.0 | étiquettes, instructions, directives, nombres, commentaires |

## 9. Débogueur

- Registres : liste commune du 6502 (A, X, Y, S, PC, P).
- Variables en direct : table des variables du BASIC (LOMEM `$4A` → PV `$CC`), entiers, tableaux, chaînes.
- Commandes : Pas, Par-dessus, Sortir, Continuer, Pause, Exécuter jusqu'à (débogueur de la v1.0).

## 10. Cassette

ACI de Wozniak (1 bit, ~1 000/2 000 Hz). `cassette-machine` : entrée = échantillons à 44 100 Hz envoyés au comparateur ; sortie = signal de l'ACI rendu à 44 100 Hz. Lecture : `C100R` puis `début.finR` ; écriture : `C100R` puis `début.finW`.

## 11. Autres périphériques

Souris, joystick, imprimante, ports série et parallèle : non utilisés (pilotes vides).

## 12. Barre d'état

PC, RAM, BASIC (en mémoire / absent), Moniteur, Cassette (nom, position), Message (dernière action).

## 13. Assistant

| Écran | Titre | Contenu |
|---|---|---|
| 1 | Bienvenue dans le Retro Dev Kit Apple 1 | la machine, le Woz Monitor, l'éditeur |
| 2 | Avant de commencer | ROM fournies, BASIC de l'utilisateur, cassette, clavier |

Écran 3 « Votre configuration » (commun, contrat 1.2.0) : miniature de chaque configuration, **dessin du kit** en attendant une meilleure illustration (choix de Wirehead du 24/11/2026), calculé par `crates/apple1-rdk/src/miniature.rs` d'après les périphériques : carte Apple 1, moniteur vert, carte cassette ACI, carte couleur GEN2 ou graphique P-LAB, moniteur couleur. Aucune image ni marque.

## 16. À propos

« Retro Dev Kit Apple 1 », version 2.0.0, GPL-3.0 ; ROM Woz Monitor, ACI et 2513 ; BASIC non fourni.

## 17. État des pilotes

| Pilote | Utilisé | Personnalisé | Vide |
|---|---|---|---|
| `clavier-editeur` | oui | non (défaut) | |
| `clavier-machine` | oui | 3 | |
| `video-analogique` | | | oui |
| `video-numerique` | oui | 4 | |
| `video-couleur` | oui | 4 (GEN2, P-LAB, phosphore 0.1.6) | |
| `souris-machine`, `machine-imprimante`, `joystick-machine`, `port-parallele`, `port-serie` | | | oui |
| `dump-memoire`, `debogueur-memoire` | oui | 5, 9 | |
| `debogueur-processeur` | oui | 9 | |
| `editeur-debogueur` | oui | non (défaut) | |
| `editeur-machine`, `editeur-memoire` | oui | 7 bis | |
| `desassembleur-memoire` | oui | 5 | |
| `biblio-editeur` | oui | non (défaut) | |
| `biblio-memoire` | oui | 7 | |
| `biblio-cassette` | oui | non (défaut) | |
| `cassette-machine` | oui | 10 | |
| `syntaxe-basic`, `syntaxe-asm` | oui | 8 | |
| `rom-machine` (0.1.5) | oui | 7 | |
| `machine-etat` | oui | 12 | |

## 18. Points à préciser

1. Réglages par programme : l'interface commune ne dit pas quel fichier est choisi dans la bibliothèque ; la fenêtre ⚙ s'ouvre sur le **dernier programme chargé ou lancé** (**proposé, à valider**).
2. Projets GitHub rangés dans la bibliothèque (`GitHub/<propriétaire>/<dépôt>/<commit>/`) au lieu du cache de la v1.0 ; le jeton n'est plus enregistré (**proposé, à valider**).
3. Pages d'aide (`aide/`) et manuels : en français seulement.

## 18 bis. Langues, état, zoom (interface commune 0.1.7 et 0.1.8)

- **7 langues** (fr, en, de, ru, es, it, pt) : langue choisie dans ⚙ Préférences → Interface ou l'assistant (langue du système par défaut). Textes de l'Apple 1 : `crates/apple1-rdk/traductions/apple1.txt` ; textes repris de la v1.0 : sa table `i18n/strings.txt`, lue sans copie. Les noms de touches envoyés à la machine, de dossiers et de configurations restent en français.
- **État de la machine** (menu 📸 État commun) : fichiers `.a1state` de la v1.0 (même format), dans `États/` de la bibliothèque ; les boutons des cartes suivent l'état chargé.
- **Zoom « Ajuster »** des moniteurs : commun (0.1.7, corrigé en 0.1.8), facteur fractionnaire.

## 19. Restrictions

- Ne jamais modifier les dépôts de référence ni l'étiquette `v1.0`.
- Aucune ROM propriétaire, aucune image tierce, aucun secret dans le dépôt.

Rusty Sr IA
