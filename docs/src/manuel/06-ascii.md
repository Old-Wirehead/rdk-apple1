# Photo → ASCII

<p class="lead">L’outil Photo → ASCII transforme une photo en image de 40 × 24 caractères de l’Apple-1, puis en programme BASIC ou 6502, prêt à être lancé ou exporté sur cassette.</p>

![L’outil Photo → ASCII : la photo et son cadre de recadrage, les réglages, et l’aperçu dessiné avec la police du 2513.](img/brut/ascii-outil.png "large")

## Ouvrir une image

**🎨 ASCII** ouvre l’outil, un panneau ancrable comme les autres. **Glissez-déposez** une photo `.jpg` ou `.png` sur la fenêtre, ou utilisez **📂 Ouvrir…** (une photo, ou un art ASCII déjà fait en `.txt`).

Le cadre vert sur la photo montre la partie convertie : faites-le glisser pour le déplacer, la molette zoome.

## Les réglages

| Réglage | Rôle |
|---|---|
| **Colonnes**, **Lignes** | 40 × 24 au plus. **23 lignes** laisse la dernière ligne à l’invite, pour que l’image ne défile pas. |
| **Zoom**, **Centre** | la partie de la photo gardée |
| **Luminosité**, **Contraste**, **Gamma** | les réglages habituels (gamma > 1 éclaircit) |
| **Inverser** | sombre = beaucoup d’encre |
| **Niveaux auto** | étire les niveaux de gris (2 % à 98 %) avant les autres réglages |
| **Tramage** | Floyd-Steinberg : plus de nuances, plus de bruit |
| **Caractères** | rampe **complète** (26 niveaux), **simple** (10), celle du **diaporama de Schmenk**, ou **personnalisée** |

Seuls les caractères du 2513 sont possibles : majuscules, chiffres et symboles, sans minuscules, ni `"`, ni `_`. Ils sont classés par le nombre de points allumés dans la police de l’Apple-1 : c’est ce nombre, et non la forme, qui donne le niveau de gris.

!!! astuce "Pour un portrait"
    40 × 24 caractères, c’est peu : recadrez sur le visage (zoom 1,3 à 1,6), montez un peu le contraste, et essayez un gamma inférieur à 1 pour une photo claire. Un fond uni et sombre donne les meilleurs résultats.

## Le programme produit

- **BASIC** : une ligne `PRINT "…"` par ligne de l’art. Une ligne pleine de 40 caractères finit par `;`, car l’écran passe tout seul à la ligne. Le programme est lent à taper (environ 2 s par ligne).
- **6502** : une source ca65 en `$0300`, lancée par `300R`. Une boucle envoie à ECHO une table compressée (suites de caractères identiques codées « nombre + caractère »), après avoir vidé l’écran. L’affichage est instantané.

Avec 24 lignes, le programme attend une touche avant de rendre la main ; avec 23, l’invite arrive sur la dernière ligne et l’image reste entière.

Les boutons : **✎ Insérer dans l’éditeur** (nouvel onglet BASIC ou 6502), **▶ Lancer**, **📼 Exporter WAV…**, **💾 Enregistrer dans la bibliothèque** (un `.txt` dans le dossier « Art ASCII »), **📋 Copier le texte**, **Voir le code**.

![De la photo au vrai Apple-1 : conversion, programme, cassette.](diagrams/workflow-ascii.svg "diagram large")

## En ligne de commande

```sh
apple1-emu ascii photo.jpg                       # l'art seul, à l'écran
apple1-emu ascii woz.jpg --zoom 1.3 --cx 0.45 --cy 0.32 \
          --contrast 1.3 --gamma 0.45 --rows 23 -o woz.txt
apple1-emu ascii woz.jpg -o woz.bas              # programme BASIC
apple1-emu ascii woz.jpg -o woz.s                # source 6502
apple1-emu ascii woz.jpg -o woz.wav              # cassette 6502, lancée par 300R
apple1-emu ascii woz.jpg -o woz.wav --format basic
```

Le chapitre 12 donne toutes les options.

!!! apple1 "Le diaporama de Dave Schmenk"
    En 2014, Dave Schmenk a écrit un diaporama de portraits pour l’Apple-1. La bibliothèque l’ajoute comme projet GitHub (`dschmenk/Apple1-Slideshow`) : il est téléchargé, pas fourni, car le dépôt n’a pas de licence. **▶ Tester le dernier build** l’assemble (syntaxe xa, traduite automatiquement ; mêmes 3456 octets que xa 2.4.1) et le lance par `280R`.
