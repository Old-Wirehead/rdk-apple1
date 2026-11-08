# Astuces, pièges et limites

<p class="lead">Ce qu’on apprend en écrivant des programmes pour l’Apple-1, dans l’émulateur puis sur la vraie machine.</p>

## Où placer un programme

| Adresse | Pour |
|---|---|
| `$0280` | le plus bas possible : juste après le tampon du moniteur |
| `$0300` | l’usage courant (et l’adresse par défaut de l’assembleur) |
| `$0C00` | avec la carte GEN2 en LORES ou en texte (pages `$0400-$0BFF` libres pour l’écran) |
| sous HIMEM | avec l’Integer BASIC : réservez la place (chapitre 16) |

Pour la page zéro, les exemples utilisent `$06-$09`, que le moniteur laisse libres. Si le programme doit cohabiter avec le BASIC, choisissez des adresses que le BASIC n’utilise pas, ou sauvez-les et restaurez-les.

## Écrire du code qui tient dans 256 octets de saut

Un branchement ne va qu’à 127 octets en avant ou 128 en arrière. Quand l’assembleur répond `branchement trop loin`, inversez la condition et sautez par-dessus un `JMP` :

```asm
        beq :+          ; au lieu de : bne loin
        jmp loin
:
```

## Les chaînes et le bit 7

Le clavier rend les codes avec le bit 7 à 1, et le moniteur compare avec ces codes. L’écran, lui, ignore le bit 7 : `.asciiz "BONJOUR"` s’affiche très bien. Pour **comparer** une touche, mettez le bit 7 dans la constante : `cmp #'Q' | $80`. Les minuscules d’une chaîne s’affichent en majuscules.

## Attendre un peu

À 1,023 MHz (0,96 MHz effectifs), une boucle `dex` / `bne` coûte 5 cycles par tour :

```asm
attente:
        ldy #78         ; 78 x 256 tours de 5 cycles : environ 0,1 s
:       ldx #0
:       dex
        bne :-
        dey
        bne :--
        rts
```

L’émulateur en vitesse réelle respecte ces durées ; en vitesse rapide (<kbd>F3</kbd>), elles disparaissent. Les boucles d’attente de l’ACI sont comptées en cycles de la même façon (annexe C).

## Déboguer efficacement

- Posez le point d’arrêt sur une **étiquette** (`start`, `prdec`, `start@fin`) plutôt que sur une adresse : il reste juste quand le code bouge.
- Passez les `jsr ECHO` avec **↪ Par-dessus** : sinon vous suivez la boucle d’attente de l’écran, instruction par instruction.
- Si l’Apple-1 ne répond plus après votre programme, regardez PC : en page zéro, c’est presque toujours un `BRK` ou un `RTS` de trop (chapitre 22).
- Le **dump** montre la page zéro : vos pointeurs y sont visibles, octet bas d’abord.

![Le débogueur arrêté dans une source 6502 : registres, drapeaux, désassemblage avec les étiquettes, et mémoire.](img/brut/debogueur-asm.png "large")

## Vers le véritable Apple-1

<ol class="steps">
<li>Assemblez et essayez dans l’émulateur, <strong>en vitesse réelle</strong> et avec l’affichage limité à 60 caractères par seconde.</li>
<li>Exportez la cassette (<strong>📼 Exporter WAV</strong>, ou <code>-o prog.aif</code>) : le format AIFF est le plus sûr depuis un téléphone.</li>
<li>Sur l’Apple-1 : <code>C100R</code>, puis la plage affichée (<code>0300.0323R</code>), lancez la lecture, puis <code>300R</code>.</li>
<li>Sans ACI : le vidage <code>.hex</code> se tape (ou se colle dans un terminal série) directement dans le Woz Monitor ; sa dernière ligne lance le programme.</li>
</ol>

## Les limites de l’assembleur intégré

- Un seul bloc de sortie, dans l’ordre de la source : les segments n’ont pas d’effet, et il n’y a pas d’éditeur de liens. Pour du code réparti sur plusieurs zones, utilisez ca65 et ld65.
- Pas d’opérateur modulo, pas de `.proc`, `.scope`, `.struct` ni `.enum` : ce sont des directives de ca65 que l’assembleur intégré ne connaît pas.
- Les instructions du 65C02 sont assemblées mais le processeur émulé est un 6502 NMOS : elles ne fonctionnent pas dans l’émulateur (chapitre 20).
- Dans le **listing**, une ligne produite par une macro perd parfois l’espace entre l’instruction et l’opérande (`ldachaine, x` pour `lda chaine,x`). C’est un défaut d’affichage du listing seulement : les octets sont justes.
