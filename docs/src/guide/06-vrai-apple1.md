# Sur un vrai Apple-1 : exporter en WAV

Tout ce que vous écrivez dans l’émulateur peut partir sur un vrai Apple-1, une réplique ou un kit équipé de la carte cassette ACI. Le fichier audio remplace la cassette ; un smartphone ou la sortie casque d’un PC sert de magnétophone.

## Fabriquer la cassette

Dans l’éditeur, cliquez sur **📼 Exporter WAV** et choisissez un nom de fichier. L’extension décide du format : `.wav`, ou `.aiff` pour un téléphone (le format conseillé, voir plus bas). Le bas de l’éditeur indique alors exactement ce qu’il faudra taper sur la vraie machine :

![Après l’export, le bas de l’éditeur rappelle la commande de chargement.](img/editeur-export-bas.png "large")

Pour `premier.bas`, la commande est `C100R` puis `004A.00FFR 0F55.0FFFR`. Le programme BASIC occupe deux enregistrements successifs sur la bande : d’abord la page zéro du BASIC (`$004A` à `$00FF`, qui contient les pointeurs du programme), puis le programme lui-même, rangé en haut de la mémoire. Le bouton **Copier** met la commande dans le presse-papiers.

!!! attention "Toujours quatre chiffres"
    Tapez `004A.00FF` et jamais `4A.FF`. La ROM de l’ACI n’efface pas les chiffres déjà lus quand elle rencontre le point : `4A.FF` serait compris comme `$004A` à `$4AFF`, et la machine attendrait 19 Ko qui ne viendront jamais. L’émulateur affiche toujours la forme correcte, et il reproduit ce comportement puisqu’il exécute la vraie ROM de l’ACI.

## Charger sur la vraie machine

Le BASIC doit déjà être en mémoire, chargé depuis sa propre cassette (`C100R` puis `E000.EFFFR`).

<ol class="steps">
<li><strong>Préparez le lecteur.</strong> Copiez le fichier sur le téléphone, branchez la sortie casque sur l’entrée « FROM TAPE » de l’ACI, mettez le volume au maximum.</li>
<li><strong>Revenez au moniteur</strong> par RESET, puis tapez <code>C100R</code>. L’ACI répond <code>C100: A9*</code> et passe à la ligne ; l’étoile est son invite.</li>
<li><strong>Tapez les plages</strong> <code>004A.00FFR 0F55.0FFFR</code>, mais n’appuyez pas encore sur RETURN.</li>
<li><strong>Lancez la lecture</strong>, puis appuyez sur <strong>RETURN pendant l’en-tête</strong>, le sifflement continu d’environ 10 secondes qui ouvre chaque enregistrement.</li>
<li><strong>Attendez le retour du <code>\</code></strong>, puis tapez <code>E2B3R</code> (BASIC à chaud, qui garde le programme) et <code>RUN</code>.</li>
</ol>

```mon
» C100R

C100: A9*
» 004A.00FFR 0F55.0FFFR
\
» E2B3R

E2B3: 20
» >RUN
BONJOUR, APPLE-1 !
```

!!! apple1 "Réussir la lecture avec un téléphone"
    - **Volume au maximum.** Acceptez l’avertissement « volume élevé » d’Android, sinon le volume reste bridé. Beaucoup de téléphones baissent le volume à chaque rebranchement du jack : vérifiez-le à chaque fois.
    - **Aucun effet sonore** : coupez l’égaliseur, Dolby, Mi Sound et toute « amélioration audio ».
    - **Mode avion** ou « Ne pas déranger », pour qu’aucune notification ne se mêle au signal. Pas de Bluetooth.
    - **Un lecteur simple** qui lit l’AIFF sans le transformer ; VLC fonctionne bien.
    - **Jamais de MP3 ni d’AAC** : la compression déforme les fronts du signal. Gardez du WAV ou de l’AIFF.
    - **Ne vous fiez pas à la LED** de l’ACI : elle ne s’allume que vers 1,2 V, bien au-dessus de ce dont la carte a besoin.

## Pourquoi l’AIFF à 48 kHz

Les fichiers de l’émulateur sont conçus pour l’entrée de l’ACI. L’AIFF 16 bits non compressé échappe à tout codec ; 48 kHz est la fréquence native d’Android, qui n’a donc pas à rééchantillonner. Les deux canaux stéréo sont strictement identiques, ce qui permet de relier gauche et droite ensemble sur FROM TAPE sans que les deux sorties se contrarient. Le signal est carré, à fronts raides, ce que le comparateur de l’ACI détecte le mieux.

!!! astuce "Vérifier un fichier avant de le jouer"
    En ligne de commande, `apple1-emu info premier.aiff --aci-input stock` simule l’entrée de la carte d’origine pour plusieurs niveaux de tension et indique à partir de quel niveau le fichier est lu correctement (environ 0,25 V crête pour nos fichiers).
