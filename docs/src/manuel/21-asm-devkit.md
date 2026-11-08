# L’assembleur du devkit

<p class="lead">L’émulateur contient un assembleur 6502 complet, à deux passes, compatible avec ca65. Il sert dans l’éditeur, qui assemble à chaque frappe, dans la ligne de commande, et pour les projets GitHub. Vous n’avez rien à installer.</p>

## Le cycle de travail

![Le cycle de travail en assembleur : écrire, assembler, lancer, déboguer, puis exporter vers le vrai Apple-1.](diagrams/workflow-asm.svg "diagram large")

<ol class="steps">
<li><strong>Écrire</strong> : <strong>📝 Éditeur</strong>, puis <strong>Nouveau</strong> ou <strong>Ouvrir</strong> un fichier <code>.s</code>, <code>.asm</code> ou <code>.a65</code>. La coloration distingue instructions, directives, étiquettes, nombres et commentaires.</li>
<li><strong>Assembler</strong> : rien à faire, l’assemblage suit chaque modification. Le bas du panneau donne la taille, les adresses et la ligne à taper sur la vraie machine ; les erreurs s’affichent avec leur ligne et leur colonne.</li>
<li><strong>Lancer</strong> : <strong>▶ Lancer</strong> met les octets en mémoire à l’adresse du <code>.org</code> et tape la commande du Woz Monitor (<code>300R</code>).</li>
<li><strong>Déboguer</strong> : <strong>⏺ Point d’arrêt</strong> sur la ligne du curseur, puis pas à pas dans le débogueur (chapitre 8), avec vos étiquettes dans le désassemblage.</li>
<li><strong>Exporter</strong> : <strong>📼 Exporter WAV</strong> écrit la cassette ; sur le vrai Apple-1, tapez <code>C100R</code>, la plage affichée (<code>0300.0323R</code>), puis <code>300R</code>.</li>
</ol>

![Une source dans l’éditeur : à gauche le texte coloré, en bas le résultat de l’assemblage ; à droite l’Apple-1 a exécuté le programme.](img/brut/editeur-asm.png "large")

## Le listing et les symboles

**📃 Listing** affiche, pour chaque ligne, l’adresse, les octets produits et la source ; les lignes produites par une macro sont marquées `+`, les fichiers inclus sont développés, et la table des symboles suit. Un clic sur une ligne ramène à la source ; un clic dans la marge pose un point d’arrêt. Voici le listing de `hello.s` (chapitre 23), produit par la ligne de commande :

```text title="hello.lst"
0300  A2 00       start:  ldx #0
0302  BD 10 03    @suite: lda message,x   ; caractère suivant
0305  F0 06               beq @fin        ; 0 = fin de la chaîne
0307  20 EF FF            jsr ECHO
030A  E8                  inx
030B  D0 F5               bne @suite
030D  4C 1F FF    @fin:   jmp GETLINE

                  message:
0310  8D                  .byte CR
0311  42 4F 4E 4A         .asciiz "BONJOUR, APPLE-1 !"
0315  4F 55 52 2C
…
; ---- Symboles (13)
; ECHO         $FFEF
; GETLINE      $FF1F
; message      $0310
; start        $0300
; start@fin    $030D
; start@suite  $0302
```

Les étiquettes locales `@suite` et `@fin` appartiennent à `start` : la table les nomme `start@suite` et `start@fin`, et le débogueur accepte ces noms comme adresses de points d’arrêt.

## La ligne de commande

{{asm (ligne de commande)|`apple1-emu asm`}} fait la même chose sans fenêtre :

```sh
apple1-emu asm hello.s                    # vérifier seulement
apple1-emu asm hello.s -l hello.lst       # avec le listing
apple1-emu asm hello.s -o hello.wav       # cassette (.wav ou .aif)
apple1-emu asm hello.s -o hello.bin       # binaire brut
apple1-emu asm hello.s -o hello.hex       # vidage Woz, à coller dans le moniteur
apple1-emu asm jeu.s -I ../commun         # autre dossier pour les .include
```

Chaque commande résume l’assemblage et donne la ligne pour le vrai Apple-1 :

```text
36 octets, $0300-$0323 ; 2 fichier(s), 13 symboles
Cassette (après C100R) : 0300.0323R   puis 300R
écrit : hello.wav
```

Sans `.org`, le programme est placé en `$0300`. Le vidage `.hex` a la forme que le Woz Monitor accepte au clavier, terminée par la commande de lancement :

```text title="t1.hex (extrait)"
0280: 34 12 34 12 10 05 FF 34
0288: 12 FF FF A8 02 EA EA EA
…
0280R
```

## La syntaxe

Une ligne contient, dans l’ordre et toutes facultatives : une étiquette, une instruction ou une directive, et un commentaire après `;`. Les mnémoniques et les directives s’écrivent en minuscules ou en majuscules ; les symboles distinguent les deux.

```asm
PTR     = $06           ; une constante (pas d'octet produit)
start:  lda #0          ; une étiquette globale, suivie de deux-points
@boucle: inx            ; une étiquette locale : visible jusqu'à la globale suivante
        bne @boucle
:       dex             ; une étiquette anonyme
        bne :-          ; :- la précédente, :+ la suivante, :-- l'avant-dernière
```

{{!étiquettes locales}}{{!étiquettes anonymes}}

**Les nombres** : `255` en décimal, `$FF` en hexadécimal, `%1010` en binaire, `'A'` pour un caractère ($41). Le clavier rend ses codes avec le **bit 7 à 1** : pour comparer une touche, écrivez `'A' | $80` ; `apple1.inc` définit de même `CR = $8D`. `*` désigne l’adresse courante.

**Les opérateurs**, du moins prioritaire au plus prioritaire :

| Priorité | Opérateurs | Sens |
|---|---|---|
| 1 | <code>&#124;&#124;</code> | ou logique (résultat 0 ou 1) |
| 2 | `&&` | et logique |
| 3 | `=` `<>` `<` `>` `<=` `>=` | comparaisons (0 ou 1) |
| 4 | `+` `-` <code>&#124;</code> | addition, soustraction, ou bit à bit |
| 5 | `*` `/` `&` `^` `<<` `>>` | multiplication, division, et, ou exclusif, décalages |
| 6 (unaires) | `<` `>` `-` `~` `!` | octet bas, octet haut, opposé, complément, non logique |

Ainsi `lda #<message` charge l’octet bas de l’adresse et `lda #>message` l’octet haut ; `#>S+1` vaut `(>S)+1`. Il n’y a pas d’opérateur modulo (`%` ou `.mod` donnent une erreur de syntaxe).

**Page zéro ou absolu.** Une adresse inférieure à `$100` connue au moment de l’assemblage est codée en page zéro (`lda $0006` donne `A5 06`, comme ca65). Pour forcer la forme absolue, écrivez `a:` (`lda a:$06` donne `AD 06 00`) ; pour forcer la page zéro, `z:`.

!!! attention "Définissez les constantes de page zéro avant de les utiliser"
    Si `PTR = $06` est écrit **après** `lda PTR`, l’assembleur ne connaît pas encore la valeur et code l’adresse sur 2 octets. Il le signale en jaune : `avertissement : adresse en page zéro codée sur 2 octets (symbole défini plus bas ; définissez-le avant, ou écrivez z: ou a:)`. Le programme marche, mais il est plus long, et les modes `(zp),Y` deviennent impossibles.

## Les directives

| Directive | Effet | Exemple |
|---|---|---|
| `.org adr` | adresse d’assemblage (et de chargement) | `.org $0300` |
| `.byte` (`.byt`) | octets, chaînes comprises | `.byte CR, "OK", 0` |
| `.word` (`.addr`) | mots de 16 bits, octet bas d’abord | `.word 10000, fin` |
| `.asciiz` | chaîne terminée par un 0 | `.asciiz "BONJOUR"` |
| `.res n[, val]` | réserve n octets (0, ou val) | `.res 3, $EA` |
| `.include "f"` | insère un fichier | `.include "apple1.inc"` |
| `.macro nom args` … `.endmacro` | définit une macro (`.mac`, `.endmac`) | voir `macros.s` |
| `.local a, b` | étiquettes propres à chaque appel de macro | |
| `.repeat n[, var]` … `.endrep` | répète un bloc, var = 0 … n−1 | `.repeat 16, I` |
| `.if` `.elseif` `.else` `.endif` | assemblage conditionnel | `.if LARGEUR > 30` |
| `.ifdef` `.ifndef` | selon qu’un symbole existe | `.ifdef APPLE1` |
| `.ifblank` `.ifnblank` | argument de macro vide ou non | |
| `.defined(sym)` | 1 si le symbole existe (dans une expression) | `.if .defined(d)` |
| `.define nom valeur` | substitution textuelle (`.undefine`) | `.define CARACTERE '*'` |
| `.setcpu "65C02"` | accepte les instructions du 65C02 (`.pc02`, `.p02`) | |
| `.segment` `.code` `.data` `.rodata` `.bss` | acceptés pour la compatibilité | |
| `.feature string_escapes` | `\n`, `\"`… dans les chaînes | |

{{!.org}}{{!.byte}}{{!.word}}{{!.asciiz}}{{!.res}}{{!.include}}{{!.macro}}{{!.repeat}}{{!.if}}{{!.setcpu}}

!!! attention "Les segments ne déplacent rien"
    Avec ca65 et ld65, `.code`, `.data` et `.bss` envoient le code dans des zones définies par la configuration de l’éditeur de liens. L’assembleur intégré produit **un seul bloc**, dans l’ordre de la source : les directives de segment sont acceptées mais n’ont pas d’effet, et `.bss` produit des zéros. Pour un projet qui a besoin d’un vrai fichier `.cfg`, la bibliothèque utilise ca65 et ld65 (chapitre 4).

## Les macros, .repeat et l’assemblage conditionnel

Une macro est un modèle de lignes, recopié à chaque appel avec ses arguments. `.local` donne à chaque appel ses propres étiquettes, sans conflit. `.repeat` déroule un bloc à l’assemblage, ce qui sert aussi à **calculer des tables** :

```asm
carres:
.repeat 16, I
        .byte I * I     ; 0, 1, 4, 9 … 225
.endrep
```

Le chapitre 23 présente `macros.s`, qui utilise tout cela, et `gen2-lores.s`, qui calcule les adresses des 24 rangées de l’écran avec `.repeat`.

## Les messages d’erreur

Chaque erreur donne le fichier, la ligne, la colonne et le texte en cause ; aucun octet n’est écrit tant qu’il reste une erreur.

| Message | Cause habituelle |
|---|---|
| `erreur de syntaxe` | opérateur inconnu, parenthèse manquante |
| `instruction, directive ou macro inconnue` | faute de frappe ; instruction du 65C02 sans `.setcpu` ; étiquette sans deux-points |
| `mode d'adressage impossible pour cette instruction` | `sta #5`, `ldx ($06),y`… |
| `symbole non défini` | `jsr ECO` au lieu de `ECHO` ; `.include` oublié |
| `symbole déjà défini` | deux étiquettes du même nom |
| `valeur hors limites` | `lda #300`, adresse au-delà de `$FFFF` |
| `branchement trop loin (-128 à +127 octets)` | remplacez `bne loin` par `beq :+` / `jmp loin` / `:` |
| `fichier .include introuvable` | nom, ou dossier : ajoutez `-I` |
| `bloc mal fermé (.macro/.endmacro, .if/.endif)` | `.endif` ou `.endmacro` oublié |
| `division par zéro` | dans une expression |
| `chaîne ou caractère sans guillemet fermant` | `"BONJOUR` |
| `valeur inconnue à cet endroit (.org, .res, .if, .repeat)` | ces directives veulent une valeur déjà définie |

![Une erreur dans l’éditeur : ECO au lieu de ECHO. Un clic sur le message mène à la ligne.](img/brut/editeur-asm-erreur.png "large")

## Compatibilité avec ca65 et xa

L’assembleur intégré suit la syntaxe de **ca65** (paquet cc65) pour tout ce qui sert à un programme d’un seul bloc. Les huit exemples du chapitre 23 ont été assemblés avec les deux outils et comparés octet par octet :

| Source | Octets | Intégré = ca65 |
|---|---|---|
| `hello.s` | 36 | <span class="status-ok">identique</span> |
| `clavier.s` | 32 | <span class="status-ok">identique</span> |
| `hexa.s` | 64 | <span class="status-ok">identique</span> |
| `decimal.s` | 117 | <span class="status-ok">identique</span> |
| `jeu.s` | 254 | <span class="status-ok">identique</span> |
| `macros.s` | 137 | <span class="status-ok">identique</span> |
| `gen2-lores.s` | 110 | <span class="status-ok">identique</span> |
| `gen2-hires.s` | 94 | <span class="status-ok">identique</span> |

Les sources écrites pour **xa** (xa65) sont reconnues à leurs lignes `#include` ou `#define`, et traduites ligne pour ligne avant l’assemblage (les numéros de ligne des erreurs restent justes) : `#define`, `#include`, `*=`, `.BYT`, `.ASC`, commentaires `/* */` et étiquettes en colonne 0 sans deux-points. Le diaporama de Dave Schmenk (chapitre 9) est assemblé ainsi, aux mêmes 3456 octets que xa 2.4.1.

Pour les **projets GitHub**, la bibliothèque préfère ca65 et ld65 s’ils sont installés (ou si le projet fournit une configuration ld65), avec `-D APPLE1` par défaut ; sinon, elle utilise l’assembleur intégré. Le fichier `apple1.toml` du dépôt précise les adresses et les symboles (chapitre 4).
