# Premier lancement

Au premier lancement, la fenêtre s’ouvre agrandie, et une fenêtre **Votre espace de travail** propose cinq dispositions : **Par défaut**, **Jeu**, **BASIC**, **Assembleur** et **Débogage**. Chaque vignette numérotée montre la disposition, dit à quoi elle sert et donne la taille d’écran minimale qu’elle demande : cliquez sur celle que vous voulez. Décochez **Afficher cette fenêtre au démarrage** si vous préférez ne plus la voir ; **⊞ Disposition ▾ → Choisir un espace de travail…** la rouvre. Vous pouvez changer de disposition à tout moment avec le menu **⊞ Disposition ▾**. La disposition par défaut rend tous les outils visibles d’un coup d’œil. Chaque zone est un panneau ancrable que vous pourrez déplacer, regrouper en onglets ou détacher dans sa propre fenêtre. L’écran couleur (GEN2 / P-LAB) est le rectangle numéro 7, dans les vignettes qui l’affichent et dans la légende ; les numéros 1 à 6 ne changent pas. Cette fenêtre ne reste pas au-dessus des autres. Si la fenêtre est étroite, la barre d’outils défile : le bouton **»** amène le groupe Cassette.

![La disposition par défaut. L’écran montre déjà le petit programme de la section suivante.](img/defaut-annote.png "large")

<ol class="legend">
<li><strong>Charger, Sauver, État</strong> : vidages hexadécimaux <code>.txt</code>, binaires <code>.bin</code> et <code>.prg</code> ; Sauver écrit aussi des cassettes <code>.wav</code>/<code>.aiff</code> ; État enregistre et restaure la machine.</li>
<li><strong>RESET, Effacer l’écran, Vitesse (jauge), Coller</strong> : les boutons de la machine (Ctrl+R, Ctrl+L, F3, Ctrl+V).</li>
<li><strong>BASIC</strong> : charge le RDK G2 Basic intégré en <code>$E000</code>.</li>
<li><strong>Fenêtres</strong> : le menu <strong>⊞ Disposition ▾</strong>, puis un bouton par panneau (écrans, Éditeur, Débogueur, Désassemblage, Dump, Bibliothèque, ASCII).</li>
<li><strong>Cartes graphiques</strong> : <strong>GEN2</strong> et <strong>P-LAB</strong> branchent ou débranchent les cartes.</li>
<li><strong>Cassette</strong> : <strong>📼 Insérer</strong> une cassette, puis lecture, stop, rembobiner, enregistrer, éjecter ; oscilloscope et son.</li>
<li><strong>EPROM</strong>, <strong>⚙ Préférences</strong>, <strong>?</strong> (bulles d’aide détaillées, aussi F1) et <strong>ℹ</strong> (à propos, raccourcis).</li>
<li><strong>Rangée d’affichage de l’écran</strong> : zoom (1x à 8x ou Ajuster), phosphore, affichage <strong>Normal</strong> ou <strong>Rapide</strong>, écran vintage, plein écran, <strong>Détacher</strong>.</li>
<li><strong>Éditeur</strong> : programmes BASIC et sources 6502 en onglets.</li>
<li><strong>Écran Apple-1</strong> : 40 × 24 caractères ; cliquez dessus pour taper, clic droit pour Copier, Coller, Effacer l’écran et Reset.</li>
<li><strong>Débogueur</strong> : pause, pas à pas, registres, points d’arrêt, variables.</li>
<li><strong>Désassemblage</strong> : le code autour du compteur ordinal (PC).</li>
<li><strong>Dump</strong> : la mémoire en hexadécimal et en ASCII, modifiable.</li>
<li><strong>Bibliothèque</strong> : vos programmes, chacun avec ses petites icônes ▶ ⬇ 📼 ⚙.</li>
<li><strong>Barre d’état</strong> : vitesse, débit d’affichage, cassette, destination du clavier ; dessous, le dernier message.</li>
</ol>

!!! astuce "Faire de la place"
    Pour seulement jouer, fermez l’éditeur et le débogueur avec leurs boutons du groupe Fenêtres (4) : l’écran s’agrandit. Dans la liste du zoom (8), **Ajuster** choisit le plus grand zoom entier qui tient dans la fenêtre, et **F11** passe en plein écran. **⊞ Disposition ▾ → Par défaut** (4) remet tout en place ; le même menu propose aussi une disposition **Jeu** (écran Apple-1 et bibliothèque). Si les boutons sont trop petits ou trop grands pour votre écran, changez **⚙ Préférences → Interface → Taille des icônes et du texte**.

!!! note "Accès au réseau au premier lancement"
    La première fois qu’il démarre, l’émulateur contacte GitHub (`api.github.com`, `raw.githubusercontent.com`) pour récupérer le projet de démonstration (`dschmenk/Apple1-Slideshow`, listé sous « Projets GitHub » dans **⚙ Préférences**), et montre sa progression dans la barre d’état. Rien d’autre n’est envoyé. Hors ligne, il le saute simplement ; retirez le projet de cette liste pour arrêter le téléchargement une fois pour toutes.

## Le Woz Monitor

L’écran affiche une barre oblique inversée `\` : c’est l’invite du **Woz Monitor**, le programme de 256 octets qui démarre l’Apple-1. Cliquez sur l’écran, puis tapez. Les minuscules deviennent des majuscules, **Entrée** vaut RETURN et **Retour arrière** produit le `_` qui efface le dernier caractère, comme en 1976.

Le moniteur comprend quatre formes de commandes :

| Vous tapez | Effet |
|---|---|
| `300` | affiche l’octet à l’adresse `$0300` |
| `300.30F` | affiche les octets de `$0300` à `$030F` |
| `300: A9 C1` | écrit `A9` en `$0300`, puis `C1` en `$0301` |
| `300R` | exécute le programme qui commence en `$0300` (*Run*) |

## Votre premier programme en langage machine

Tapez ces deux lignes, chacune suivie d’Entrée :

```mon
» 300: A9 C1 20 EF FF 4C 1F FF

0300: 00
» 300R

0300: A9A
```

Le `A` final a été affiché par votre programme. Les huit octets sont trois instructions du 6502 :

| Octets | Instruction | Rôle |
|---|---|---|
| `A9 C1` | `LDA #$C1` | charge le code de la lettre « A » (avec le bit 7 à 1, comme l’attend l’Apple-1) |
| `20 EF FF` | `JSR $FFEF` | appelle ECHO, la routine du moniteur qui affiche un caractère |
| `4C 1F FF` | `JMP $FF1F` | revient au moniteur, qui attend la commande suivante |

Le moniteur affiche d’abord le contenu de l’adresse de départ (`0300: A9`), puis votre programme écrit le `A`. Le Manuel complet consacre un chapitre entier à l’assembleur intégré, qui vous évite de calculer ces octets à la main.

!!! apple1 "Sur le vrai Apple-1"
    Les mêmes commandes fonctionnent à l’identique : l’émulateur exécute la vraie ROM du Woz Monitor. La machine réelle affiche environ 60 caractères par seconde. L’émulateur respecte ce rythme quand la rangée d’affichage de l’écran est sur **Normal** ; **Rapide** affiche tout d’un coup. Ce réglage est commun à l’écran de l’Apple-1 et à celui de la GEN2.
