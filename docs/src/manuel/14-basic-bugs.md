# Les bugs connus et leurs contournements

<p class="lead">L’Integer BASIC de l’Apple-1 a été écrit sans assembleur, en 1976, et n’a jamais été corrigé. Voici ses défauts connus, chacun reproduit dans l’émulateur avec le vrai BASIC, et comment les éviter.</p>

<span class="verified">vérifié</span> : reproduit dans l’émulateur avec l’Integer BASIC réel, transcription à l’appui. <span class="unverified">non vérifié</span> : rapporté par une source, pas reproduit. Le bug n correspond à la section 18.n. Comme l’émulateur exécute le code d’origine instruction par instruction, un bug reproduit ici existe aussi sur un vrai Apple-1.

## Sortir d’une boucle FOR par GOTO <span class="verified">vérifié</span>

Une boucle quittée par `GOTO` reste « ouverte ». Si le programme repasse par le `FOR`, une nouvelle entrée s’empile ; au neuvième passage : `>8 FORS ERR`.

```basic
10 N=0
20 FOR I=1 TO 10
30 IF I=2 THEN GOTO 50
40 NEXT I
50 N=N+1:IF N<20 THEN 20
60 PRINT "FINI ";N
70 END
```

```mon
>» RUN
*** >8 FORS ERR
STOPPED AT 20
```

!!! astuce "Contournement"
    Ne sortez jamais d’un `FOR` par `GOTO` : donnez à la variable sa valeur finale et passez par `NEXT`. Ici : `30 IF I=2 THEN I=10:GOTO 40`. Le programme affiche alors `FINI 20`. Les portages des *101 BASIC Computer Games* suivent cette règle.

## L’opérateur ^ bloque la machine <span class="verified">vérifié</span>

`^` (puissance) est dans la table de l’interpréteur, mais `PRINT 2^3` ne rend jamais la main : seul RESET en sort.

!!! astuce "Contournement"
    Multipliez : `2*2*2`, ou une boucle `P=1:FOR I=1 TO N:P=P*X:NEXT I`.

## Le message « PPED AT » <span class="verified">vérifié</span>

Quand la pile des opérandes de l’interpréteur déborde (une expression trop imbriquée en mode direct), le BASIC devrait afficher un message d’erreur, mais il pointe **3 octets trop loin** dans la table des messages (le désassemblage d’Eric Smith le note : `ldy #erri_stopped_at+3 ; The "+3" is a bug!`). On lit alors la fin de `STOPPED AT` :

```mon
>» PRINT 1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1)))))))))))))
14
*** PPED AT  ERR
STOPPED AT 65535
```

!!! astuce "Contournement"
    Découpez les expressions longues avec des variables intermédiaires. Au-delà de 12 niveaux de parenthèses, une ligne de programme est de toute façon refusée (`TOO LONG ERR`).

## IF … THEN ne conditionne que la première instruction <span class="verified">vérifié</span>

Dans `IF A=2 THEN PRINT "X":PRINT "Y"`, seul `PRINT "X"` dépend de la condition : `PRINT "Y"` est **toujours** exécuté (avec A=1, le programme affiche `Y`). C’est le piège le plus fréquent quand on porte un programme écrit pour un autre BASIC.

!!! astuce "Contournement"
    Inversez la condition et sautez par-dessus le bloc :

    ```basic
    80 IF A#2 THEN 90
    85 PRINT "X":PRINT "Y"
    90 REM SUITE
    ```

## −32768 <span class="verified">vérifié</span>

`X=-32767-1` affiche un `-` puis `*** >32767 ERR`. La plus petite valeur utilisable est −32767.

!!! astuce "Contournement"
    Restez dans −32767 … 32767. Pour lire un mot de 16 bits par `PEEK`, découpez-le : `PEEK(A)+PEEK(A+1)*256` déborde au-delà de 32767 ; testez d’abord l’octet haut (`IF PEEK(A+1)>127 THEN …`).

## ASC( ne fonctionne pas <span class="verified">vérifié</span>

`ASC("A")` est dans la table des mots-clés, mais toute ligne qui l’emploie est refusée : `SYNTAX ERR`. Même chose pour `SCRN(`.

!!! astuce "Contournement (essayé)"
    Cherchez le caractère dans une chaîne de référence. La ligne 60 referme la boucle (`K=26:NEXT K`) pour éviter le bug 1 :

```basic title="Le rang d’une lettre, sans ASC"
10 DIM C$(26),L$(1)
20 C$="ABCDEFGHIJKLMNOPQRSTUVWXYZ"
30 INPUT "LETTRE",L$
40 FOR K=1 TO 26:IF C$(K,K)=L$ THEN 60:NEXT K
50 PRINT "PAS UNE LETTRE":END
60 PRINT "RANG ";K:K=26:NEXT K
70 GOTO 30
```

```mon
LETTRE?» W
RANG 23
LETTRE?» E
RANG 5
LETTRE?» 5
PAS UNE LETTRE
```

## Les mots-clés graphiques sans effet <span class="verified">vérifié</span>

`COLOR=`, `PLOT` et `HLIN` sont acceptés et ne font rien ; `VLIN` et `VTAB` donnent `SYNTAX ERR`. Ce sont des restes du BASIC destiné à l’Apple II.

!!! astuce "Contournement"
    Pour positionner le texte, utilisez `TAB n` et des `PRINT` vides. Pour dessiner en couleur, la carte GEN2 se pilote par `POKE` (chapitre 6).

## Le bug des 251 caractères de l’Apple II <span class="verified">absent</span>

Une version plus tardive de l’Integer BASIC (Apple II) plante sur certaines lignes de 251 caractères et plus. Sur l’Apple-1, la ligne tapée est limitée bien avant (environ 128 caractères) : le bug ne peut pas se produire.

## STOPPED AT 65535 <span class="verified">vérifié</span>

Une erreur provoquée par une ligne **refusée pendant sa saisie** (trop imbriquée) ou par le bug 3 en mode direct est suivie de `STOPPED AT 65535` : le BASIC croit être dans un programme et affiche un numéro de ligne absurde. C’est sans conséquence ; le programme en mémoire n’est pas touché.

## USR plante la machine <span class="verified">vérifié</span>

`X=USR(1)` saute en `$0000`, où il n’y a normalement pas de code : la machine se bloque. RESET puis `E2B3R` reviennent au BASIC sans perdre le programme.

!!! astuce "Contournement"
    Appelez vos programmes machine avec `CALL adresse` ; passez les valeurs par `POKE` et `PEEK`.

## HIMEM= ne peut pas baisser <span class="verified">vérifié</span>

`HIMEM=3072` (et toute valeur inférieure à l’actuelle) est **ignoré sans message**, même après `SCR`, parce que l’interpréteur compare la nouvelle valeur au pointeur de programme avant de l’accepter.

!!! astuce "Contournement (vérifié)"
    `POKE 76,0 : POKE 77,12 : SCR` met HIMEM à 3072 ; le programme suivant est rangé en dessous. Voir le chapitre 16.

## Le deux-points refusé en mode direct <span class="verified">vérifié</span>

`X=5:Y=X*2:PRINT Y` tapé sans numéro donne `SYNTAX ERR` ; la même ligne numérotée fonctionne. `FOR` et `GOSUB` sont aussi refusés en mode direct.

!!! astuce "Contournement"
    Tapez une instruction par ligne, ou écrivez une ligne `0` temporaire et `RUN 0`… sans oublier de l’effacer (`DEL 0`).

## INPUT ajoute toujours « ? » <span class="verified">vérifié</span>

`INPUT "AGE ? ",N` affiche `AGE ? ?` : le BASIC ajoute son propre point d’interrogation. Écrivez l’invite sans `?` : `INPUT "AGE",N` affiche `AGE?`.

## La boucle FOR passe au moins une fois <span class="verified">vérifié</span>

`FOR I=5 TO 1 … NEXT I` exécute le corps une fois (avec I=5), là où d’autres BASIC le sautent.

!!! astuce "Contournement"
    Testez avant : `IF N<1 THEN 100` avant `FOR I=1 TO N`.

## Une touche arrête le programme <span class="verified">vérifié</span>

N’importe quelle touche frappée pendant l’exécution l’arrête (`STOPPED AT`), sauf pendant un `INPUT`. Ce n’est pas un bug mais un choix : il n’y a pas de touche BREAK. Conséquence : un jeu en temps réel ne peut pas attendre une touche sans s’arrêter. Lire le clavier par `PEEK(-12271)` (`$D011`) et `PEEK(-12272)` (`$D010`) semble possible, mais l’interpréteur teste lui aussi le clavier entre deux instructions et risque de s’arrêter avant : nous ne l’avons pas essayé (<span class="unverified">non vérifié</span>). Pour un jeu en temps réel, passez au langage machine (partie III).

## Résumé

| # | Bug | Statut | Contournement |
|---|---|---|---|
| 1 | sortie de FOR par GOTO → `>8 FORS` | <span class="status-ok">vérifié</span> | finir la boucle par NEXT |
| 2 | `^` bloque | <span class="status-ok">vérifié</span> | multiplier |
| 3 | « PPED AT » (+3) | <span class="status-ok">vérifié</span> | variables intermédiaires |
| 4 | `IF…THEN a:b` exécute b | <span class="status-ok">vérifié</span> | condition inverse + saut |
| 5 | −32768 | <span class="status-ok">vérifié</span> | rester ≥ −32767 |
| 6 | `ASC(`, `SCRN(` | <span class="status-ok">vérifié</span> | chaîne de référence |
| 7 | COLOR=, PLOT, HLIN sans effet ; VLIN, VTAB refusés | <span class="status-ok">vérifié</span> | TAB, POKE (GEN2) |
| 8 | 251 caractères (Apple II) | <span class="status-ok">absent</span> | — |
| 9 | STOPPED AT 65535 | <span class="status-ok">vérifié</span> | sans conséquence |
| 10 | USR plante | <span class="status-ok">vérifié</span> | CALL |
| 11 | HIMEM= ne baisse pas | <span class="status-ok">vérifié</span> | POKE 76/77 + SCR |
| 12 | `:` refusé en direct | <span class="status-ok">vérifié</span> | une instruction par ligne |
| 13 | INPUT ajoute « ? » | <span class="status-ok">vérifié</span> | invite sans « ? » |
| 14 | FOR passe une fois | <span class="status-ok">vérifié</span> | tester avant |
| 15 | lecture du clavier par PEEK pendant RUN | <span class="status-src">non vérifié</span> | langage machine |
