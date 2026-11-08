# Messages d’erreur et limites

## Les messages

Une erreur affiche `*** MESSAGE ERR`, suivi en cours de programme de `STOPPED AT n`. Une ligne refusée à la saisie n’est pas rangée : retapez-la.

| Message | Cause | Exemple essayé |
|---|---|---|
| {{SYNTAX ERR}} | instruction inconnue, nom de variable invalide, `:` en mode direct | `CPT=7`, `GET A$`, `X=5:PRINT X` en direct |
| {{>32767 ERR}} | dépassement, division par zéro, numéro de ligne > 32767 | `200*200`, `10/0`, `RND(0)`, `32768 PRINT` |
| {{TOO LONG ERR}} | ligne trop complexe (expression trop imbriquée) ou trop longue | 13 parenthèses imbriquées |
| {{MEM FULL ERR}} | plus de place entre variables et programme | `DIM A(3000)` en 8 Ko |
| {{TOO MANY PARENS ERR}} | parenthèses (prévu par l’interpréteur ; masqué par le bug 3) | — |
| {{STRING ERR}} | chaîne mal employée | — |
| {{END ERR}} | le programme se termine sans `END` | `10 PRINT "X"` puis `RUN` |
| {{BAD BRANCH ERR}} | `GOTO`/`GOSUB` vers une ligne inexistante | `GOTO 500` |
| {{>8 GOSUBS ERR}} | plus de 8 `GOSUB` imbriqués | récursion : 9 appels |
| {{BAD RETURN ERR}} | `RETURN` sans `GOSUB` | `RUN 30` sur un `RETURN` |
| {{>8 FORS ERR}} | plus de 8 `FOR` imbriqués, ou bug 1 | 9 boucles sur une ligne |
| {{BAD NEXT ERR}} | `NEXT` sans `FOR` correspondant | `20 NEXT I` seul |
| {{STOPPED AT}} | une touche a été frappée pendant l’exécution | — |
| {{>255 ERR}} | valeur > 255 : `POKE`, `DIM` d’une chaîne | `POKE 768,300`, `DIM B$(256)` |
| {{RANGE ERR}} | indice de tableau hors limites | `A(0)`, `A(3)` après `DIM A(2)` |
| {{DIM ERR}} | tableau ou chaîne redimensionné(e) | — |
| {{STR OVFL ERR}} | chaîne plus longue que son `DIM` | `A$="ABCDE"` avec `DIM A$(3)` |
| {{RETYPE LINE}} | réponse à `INPUT` invalide (lettres pour un nombre) : retapez | — |

Les lignes « — » viennent de la table des messages de l’interpréteur ; elles n’ont pas été provoquées dans nos essais.

## Les limites

| Limite | Valeur |
|---|---|
| entiers | −32767 à 32767 |
| numéros de ligne | 0 à 32767 |
| noms de variables | lettre + chiffre facultatif |
| noms de chaînes | une lettre + `$` |
| longueur d’une chaîne | 255 |
| dimensions d’un tableau | 1, indices de 1 à n |
| `FOR` imbriqués | 8 |
| `GOSUB` imbriqués | 8 |
| parenthèses imbriquées | 12 |
| longueur d’une ligne tapée | 120 caractères passent, 141 sont refusés |


## Gagner de la mémoire

En 8 Ko, le BASIC dispose d’environ 2 Ko pour le programme et les variables. Quelques moyens d’en gagner :

<ol class="steps">
<li><strong>Abaissez LOMEM</strong> si vous n’avez pas de langage machine : <code>LOMEM=768</code> (<code>$0300</code>) donne 1280 octets de plus aux variables. Essayé : <code>DIM T(1200)</code> échoue en <code>MEM FULL</code> avec LOMEM par défaut, et passe après <code>LOMEM=768</code>. Les portages des *101 BASIC Computer Games* faits pendant le développement procèdent ainsi.</li>
<li><strong>Écrivez court</strong> : le BASIC garde les espaces et les <code>REM</code>. <code>LIST</code> réindente tout seul, et une ligne longue regroupant plusieurs instructions coûte moins que plusieurs lignes.</li>
<li><strong>Réutilisez les tableaux et les chaînes</strong> plutôt que d’en dimensionner de nouveaux.</li>
<li><strong>Ajoutez de la RAM</strong> : la fiche ⚙ d’un programme, <code>--ram 16</code>, ou une vraie extension. Un programme de plus de 8 Ko exige une machine de 12, 16 ou 32 Ko : <code>apple1-emu basic --ram 16</code> le place en haut de cette mémoire.</li>
</ol>
