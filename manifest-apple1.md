# Manifeste — RDK Apple 1 (spécificités)

> Rempli à partir du modèle de RDK-Interface-Commune 0.1.4 (`modeles/manifest-modele.md`). Ne décrit que ce qui diffère des défauts de l'interface commune. Code : `crates/apple1-rdk`.
>
> Rédigé le : 17/11/2026 · Auteur : Rusty Sr IA · Statut : brouillon, à valider par Wirehead
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
| Langages | BASIC entier de l'Apple 1 (copie de l'utilisateur), assembleur 6502 (syntaxe ca65 de la v1.0) |
| Périphériques | cassette ACI ; programmateur de PROM |

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
- Double-clic qui charge (`double_clic_charge`) : `.wav`, `.aif(f)`, `.bin`, `.prg`, `.mon`, `.hex`, `.apl`.

## 4. Moniteurs et pilotes vidéo

| Moniteur | Pilote | Détails |
|---|---|---|
| Monochrome | `video-numerique` | 280 × 192 points (40 × 24 cases de 7 × 8), ROM 2513 de la v1.0 |
| Couleur | — | **absent** (`manifeste_moniteurs` = monochrome seul ; **fixé** : l'Apple 1 est monochrome) |

Effets CRT : non en 2.0. Option `video-couleur` : non (carte GEN2 de la v1.0 non reprise en 2.0).

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
| `aci-enregistrer` | ⏺ | Sortie ACI | 1er clic : enregistre la sortie cassette ; 2e clic : écrit `Biblio/Cassettes/sortie-aci-<horodatage>.wav` |

Programmateur de PROM : **oui**. 2.0 : écrit l'image `$E000-$EFFF` dans `Biblio/PROM/prom-E000-EFFF.bin` et donne la commande `minipro -p 2732@DIP24 -w …` (la v1.0 pilotait minipro elle-même : à reprendre plus tard).

## 7. Bibliothèque

- Dossiers en plus : `Cassettes`, `PROM`.
- Charger : sources `.bas`/`.asm` → ⬇ RAM ; `.wav`/`.aif` (ACI, décodé comme en v1.0), `.mon`/`.hex`/`.apl` (vidage Woz), `.bin`, `.prg` → mémoire.
- Play : RESET, chargement, puis `xxxxR` (adresse de lancement) ou, pour une cassette BASIC (page zéro `$004A`), `E2B3R` puis `RUN`.

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

## 16. À propos

« Retro Dev Kit Apple 1 », version 2.0.0, GPL-3.0 ; ROM Woz Monitor, ACI et 2513 ; BASIC non fourni.

## 17. État des pilotes

| Pilote | Utilisé | Personnalisé | Vide |
|---|---|---|---|
| `clavier-editeur` | oui | non (défaut) | |
| `clavier-machine` | oui | 3 | |
| `video-analogique` | | | oui |
| `video-numerique` | oui | 4 | |
| `video-couleur` | | | oui (pas de couleur) |
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
| `machine-etat` | oui | 12 | |

## 18. Points à préciser

1. Carte couleur GEN2 et carte P-LAB de la v1.0 : à reprendre en 2.x (option `video-couleur`) ?
2. Programmateur de PROM : fenêtre propre (choix de la puce, lecture, vérification) quand l'interface commune le permettra.
3. Bibliothèque d'origine de la v1.0 (programmes reconnus, art ASCII) : à reprendre ?

## 19. Restrictions

- Ne jamais modifier les dépôts de référence ni l'étiquette `v1.0`.
- Aucune ROM propriétaire, aucune image tierce, aucun secret dans le dépôt.

Rusty Sr IA
