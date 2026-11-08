# La cassette et la carte ACI

<p class="lead">L’Apple-1 n’avait ni disquette ni ROM BASIC : tout passait par une bande audio et la carte ACI (Apple Cassette Interface). L’émulateur exécute la vraie ROM de l’ACI ; ses fichiers WAV et AIFF se lisent donc sur un vrai Apple-1, et ceux d’un vrai Apple-1 se lisent dans l’émulateur.</p>

## Les formats de fichiers

| Extension | Contenu | Lecture | Écriture |
|---|---|---|---|
| `.wav` | cassette ACI, PCM 8 à 32 bits ou flottant, mono ou stéréo, toute fréquence | oui | 44,1 kHz mono 16 bits par défaut |
| `.aif`, `.aiff` | cassette ACI (AIFF et AIFF-C) | oui | profil smartphone : 48 kHz, 2 canaux identiques, 16 bits |
| `.txt`, `.hex`, `.apl` | vidage du Woz Monitor : lignes `0300: A9 C1 20 …` | oui | oui |
| `.bin` | octets bruts, sans adresse (`FICHIER.bin@300`) | oui | oui |
| `.bas` | listing Integer BASIC (texte) | dans l’éditeur | dans l’éditeur |
| `.s`, `.asm`, `.a65` | source 6502 (ca65, ou xa) | dans l’éditeur | dans l’éditeur |

Les cassettes exportées par l’émulateur portent leurs adresses dans un commentaire du fichier (`APPLE1 ACI 0300.031E`), que les lecteurs audio ignorent. Une cassette d’une autre provenance n’a pas d’adresses : la bibliothèque les reconnaît ou les devine (chapitre 4), et la ligne de commande accepte `--addr` ou `--range`.

Le décodeur tolère le bruit, les faibles niveaux, l’inversion de polarité, une vitesse de bande de ±10 % et plusieurs enregistrements dans un même fichier. Les WAV de l’outil **c2t** d’Egan Ford sont lus aussi. Un **ronflement du secteur** (50 ou 60 Hz, même aussi fort que le signal de la cassette) est filtré avant la détection des fronts : un long enregistrement fait avec un câble mal blindé se lit donc correctement.

## Le lecteur virtuel

La zone **Cassette :** de la barre d’outils réunit toutes les commandes du magnétophone : **📼 Insérer**, **▶** lecture, **⏹** stop, **⏪** rembobiner, **⏺** enregistrer, **⏏** éjecter, puis le nom et la position de la bande, l’oscilloscope **📈** et le son **🔊**. Une cassette s’insère par **📼 Insérer** (un `.wav` ou un `.aiff`), ou par le clic droit d’un programme de la bibliothèque (« Insérer comme cassette ») : vous tapez alors vous-même `C100R` et la plage. L’icône **📼** d’un programme de la bibliothèque va plus loin : elle insère la cassette et tape les commandes pour vous.

![La zone Cassette : pendant la lecture de BASIC.wav : 7,6 secondes sur 35,8.](img/barre-cassette.png "large")

La cassette suit le temps **émulé** : en vitesse rapide (<kbd>F3</kbd>), le chargement est bien plus court que les 10 secondes d’en-tête, et il fonctionne quand même. Sans la cassette, les chargements de la bibliothèque sont instantanés (chapitre 4).

Charger l’Integer BASIC « à l’ancienne » se fait ainsi :

```mon
» C100R
C100: A9*
» E000.EFFFR
» E000R

E000: 4C
>
```

`C100R` démarre le programme de l’ACI, qui répond par son invite `*`. La ligne `E000.EFFFR` demande de **lire** la bande dans `$E000-$EFFF`. Quand la bande s’arrête, l’ACI rend la main au moniteur (`\`) ; `E000R` démarre le BASIC, dont l’invite est `>`.

## Son et oscilloscope

![L’oscilloscope pendant la lecture de BASIC.wav : la case « Son de la cassette », le volume, et le signal à 1 ms par division.](img/oscilloscope.png "small")

Pendant une lecture ou un enregistrement, **🔊** fait entendre le signal, comme le haut-parleur d’un magnétophone, à volume faible par défaut. **📈** ouvre l’**oscilloscope** : une trace verte sur une grille sombre, 1 ms par division, qui montre 10 ms autour de la tête de lecture, ou les 10 dernières millisecondes écrites par l’Apple-1. On y reconnaît l’en-tête (≈ 800 Hz), les bits 0 (≈ 2 kHz) et les bits 1 (≈ 1 kHz). Le son est muet en vitesse maximale.

## Enregistrer depuis l’Apple-1 émulé

Pour produire une cassette comme en 1976, armez l’enregistreur, puis demandez à l’ACI d’écrire :

<ol class="steps">
<li>Cliquez sur <strong>⏺</strong> et choisissez le fichier (<code>.wav</code> ou <code>.aiff</code>).</li>
<li>Tapez <code>C100R</code>, puis la plage suivie de <code>W</code>, par exemple <code>0300.03FFW</code>, puis <kbd>RETURN</kbd>.</li>
<li>L’ACI écrit un en-tête de 10 secondes, puis les données. Le fichier est écrit 1,5 s après la fin, ou au <strong>⏹</strong>.</li>
</ol>

Le plus simple reste **Sauver** dans la barre d’outils, ou **📼 Exporter WAV** dans l’éditeur : le signal est le même, au cycle près, et il est produit instantanément.

## Comment fonctionne l’ACI

![Les deux sens de l’ACI : à l’écriture, chaque accès à $C0xx fait basculer la sortie ; à la lecture, le comparateur modifie la ligne A0 de la PROM.](diagrams/aci.svg "diagram large")

La carte est d’une simplicité remarquable : une PROM de 256 octets visible en `$C000` et `$C100`, une bascule et un comparateur.

- **Écriture.** Chaque accès à une adresse `$C000-$C0FF` fait basculer la sortie (74LS74). Le programme de la ROM règle la durée entre deux accès : c’est elle qui code les bits.
- **Lecture.** Le niveau de l’entrée bande, après le comparateur LM311, modifie la ligne d’adresse A0 de la PROM quand A7 = 1. La même instruction lit donc `$C080` ou `$C081` selon le niveau : la ROM mesure les demi-périodes par une boucle de 12 cycles.
- **Codage.** Un « 1 » dure environ deux fois plus longtemps qu’un « 0 » : 1 kHz pour un 1, 2 kHz pour un 0. Les octets partent bit de poids fort en premier, sans parité ni somme de contrôle.

L’annexe C donne les temps exacts, en cycles et en microsecondes.

## Les commandes de l’ACI

Après `C100R`, l’ACI attend une ligne au format du moniteur, terminée par `R` (lire) ou `W` (écrire) :

| Ligne | Effet |
|---|---|
| `0300.03FFR` | lit la bande dans `$0300-$03FF` |
| `0300.03FFW` | écrit `$0300-$03FF` sur la bande, avec un en-tête de 10 s |
| `004A.00FFR 0300.0FFFR` | lit **deux** enregistrements successifs : la page zéro du BASIC, puis le programme |
| `004A.00FFW800.FFFW` | la forme « collée » du manuel BASIC de 1976, acceptée aussi |

{{!C100R}}{{!R (lire)}}{{!W (écrire)}}

!!! attention "Les zéros de 004A.00FF sont obligatoires"
    L’analyseur de la ROM n’efface pas son accumulateur hexadécimal au point (« Doesn’t clear HEX1!!! », dit le listing). Les chiffres de la fin s’ajoutent derrière ceux du début : `4A.FF` signifie `$004A-$4AFF`, soit près de 19 Ko ! `0300.0FFF` fonctionne, parce que les quatre chiffres chassent le 3 de `0300`. Écrivez toujours les deux adresses sur quatre chiffres. L’émulateur reproduit ce comportement, puisqu’il exécute la vraie ROM.

Un programme BASIC se sauve en deux enregistrements : la page zéro `$004A-$00FF` (où sont les pointeurs LOMEM, HIMEM et la fin du programme), puis le programme lui-même, qui finit toujours à HIMEM. Pour le recharger, le BASIC doit déjà être en mémoire, puis on revient au BASIC « à chaud » par `E2B3R`, qui garde le programme.

```mon
» C100R
C100: A9*
» 004A.00FFR 0300.0FFFR
» E2B3R

E2B3: 20
» >RUN
```

## Vers un vrai Apple-1

C’est l’usage qui justifie l’émulateur : préparer un programme sur le PC, puis le jouer dans l’ACI d’une vraie machine, depuis un smartphone ou la sortie casque d’un ordinateur.

<ol class="steps">
<li><strong>Préparez le fichier.</strong> <strong>📼 Exporter WAV</strong> dans l’éditeur, ou <code>apple1-emu export prog.txt -o prog.aiff</code>. Choisissez <code>.aiff</code> pour un smartphone. Notez la ligne affichée, par exemple <code>C100R</code> puis <code>0300.031ER</code>.</li>
<li><strong>Réglez le lecteur.</strong> Volume au maximum (acceptez l’avertissement « volume élevé »), tous les effets audio coupés (égaliseur, Dolby, « amélioration audio »), mode avion, pas de Bluetooth. Un lecteur simple comme VLC convient.</li>
<li><strong>Sur l’Apple-1</strong>, tapez <code>C100R</code> : le moniteur répond <code>C100: A9*</code>.</li>
<li>Tapez la ligne de lecture, par exemple <code>0300.031ER</code>, mais <strong>n’appuyez pas encore sur RETURN</strong>.</li>
<li>Lancez la lecture, puis appuyez sur <kbd>RETURN</kbd> <strong>pendant l’en-tête</strong> (il dure environ 10 s, après une seconde de silence).</li>
<li>À la fin, l’ACI revient au moniteur (<code>\</code>). Lancez le programme : <code>300R</code>, ou <code>E2B3R</code> puis <code>RUN</code> pour un programme BASIC.</li>
</ol>

**Pourquoi ce profil AIFF ?** 16 bits non compressé, sans codec avec perte : un MP3 ou un AAC arrondirait les fronts. 48 kHz, la fréquence native d’Android, pour éviter tout rééchantillonnage. Deux canaux strictement identiques, parce qu’un câble qui relie gauche et droite sur FROM TAPE ne doit jamais recevoir deux signaux différents. Un signal carré à −1 dBFS : l’entrée de l’ACI d’origine est un passe-haut (0,01 µF) qui ne voit guère que les fronts, et un carré à fronts raides est ce qui lui convient le mieux. Une seconde de silence au début et à la fin, parce qu’un téléphone coupe parfois le début d’un son.

!!! apple1 "Ne vous fiez pas à la LED"
    Le manuel de 1976 demande un niveau tel que la LED de l’ACI soit « juste allumée ». Elle ne s’allume pourtant qu’à environ 1,2 V, bien plus que ce dont le comparateur a besoin. Une LED éteinte ne prouve donc pas que le signal est trop faible. Si la lecture ne finit jamais, la ROM attend encore des fronts : faites RESET et montez le volume.

!!! attention "Jamais de MP3 ni d’AAC"
    Un format avec perte décale les fronts et rend la cassette illisible. Utilisez WAV, AIFF, ou FLAC si votre lecteur le décode.

## Quel niveau faut-il ? Les modèles d’entrée

La commande `info --aci-input` simule l’entrée analogique de la carte (condensateur, polarisation, hystérésis du LM311) pour une série de tensions crête, et dit à partir de quel niveau le fichier serait lu :

```sh
apple1-emu info prog.aiff --aci-input stock     # carte de 1976, 0,01 µF
apple1-emu info prog.aiff --aci-input mimeo     # 0,1 µF (conseil du guide Mimeo)
apple1-emu info prog.aiff --aci-input gen2      # ACI Gen2 d'Uncle Bernie
apple1-emu info prog.aiff --aci-input stock --volts 0.5 --series 1200
```

| Modèle | Ce qu’il représente |
|---|---|
| `stock` | l’ACI d’origine : 0,01 µF vers le comparateur polarisé à 2,5 V, réaction 3k/47k |
| `mimeo` | la modification de Mike Willegal : 0,1 µF, plus fiable |
| `gen2` | l’ACI Gen2 d’Uncle Bernie : 100 nF appairé, 300k au lieu de 47k (moins d’hystérésis) |

`--volts` fixe la tension crête à la prise pour la pleine échelle numérique ; `--series` ajoute une résistance série (par exemple les 1,2 kΩ d’un câble de smartphone). Avec le modèle `stock`, les fichiers de l’émulateur sont lus dès **environ 0,25 V crête** ; une sortie casque de smartphone au maximum donne en général 0,5 à 1 V. Ce modèle est simplifié : ni bruit, ni dérive du point de polarisation.

## Numériser une vraie cassette ou un enregistrement de l’Apple-1

Pour récupérer un programme tapé sur un vrai Apple-1, enregistrez la sortie **TO TAPE** de l’ACI par une entrée micro (téléphone, ou carte son du PC avec Audacity en 44,1 ou 48 kHz), en coupant le gain automatique et la réduction de bruit. Sur l’Apple-1 : `C100R`, puis `0300.03FFW` et <kbd>RETURN</kbd>. Arrêtez quelques secondes après le retour du `\`, puis :

```sh
apple1-emu import enregistrement.aiff --addr 300 -o prog.txt
apple1-emu import startrek.aiff --range "004A.00FFR 0300.0FFFR" -o startrek.txt
```

!!! apple1 "La sortie TO TAPE est un niveau micro"
    Elle ne fournit qu’environ 50 mV crête à crête. Regardez la forme d’onde dans Audacity : ni écrêtage, ni saut de niveau pendant l’en-tête. L’import de l’émulateur tolère un gain quelconque et un décalage continu.
