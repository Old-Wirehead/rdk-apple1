# Porter un programme Applesoft ou Microsoft BASIC

<p class="lead">La plupart des listings des années 1970-1980 sont écrits en Microsoft BASIC (Applesoft, BASIC des « 101 BASIC Computer Games »). Ils se portent sur l’Apple-1, à condition de connaître les équivalences. Chaque ligne « refusé » ci-dessous a été tapée dans le vrai BASIC.</p>

## Les équivalences

| Applesoft / Microsoft | Integer BASIC de l’Apple-1 | Note |
|---|---|---|
| `ON X GOTO 100,200,300` | `GOTO 100*X` (si les lignes s’y prêtent) ou une suite de `IF X=1 THEN 100` | refusé |
| `GET A$` | `INPUT A$` | refusé |
| `DATA` / `READ` / `RESTORE` | tableau rempli par des affectations, ou chaîne découpée | refusés |
| `MID$(A$,I,N)` | `A$(I,I+N-1)` | refusé |
| `LEFT$(A$,N)` / `RIGHT$(A$,N)` | `A$(1,N)` / `A$(LEN(A$)-N+1)` | — |
| `A$+B$` | `A$(LEN(A$)+1)=B$` | — |
| `CHR$(N)`, `ASC(A$)` | chaîne de référence (chapitre 18, bug 6) | refusés |
| `STR$(N)`, `VAL(A$)` | `PRINT N` direct ; `INPUT N` | refusés |
| `INT(X)` | inutile : tout est entier | refusé |
| `RND(1)` (réel de 0 à 1) | `RND(N)` (entier de 0 à N−1) | — |
| `X^2` | `X*X` | bloque (bug 2) |
| `HOME` | 24 × `PRINT`, ou CLEAR SCREEN (<kbd>Ctrl</kbd>+<kbd>L</kbd>) à la main | refusé |
| `PRINT TAB(10);"X"` | `TAB 10:PRINT "X"` | — |
| `STOP` | `END` | refusé |
| `DEF FN` | `GOSUB` | — |
| `A<>B` | `A#B` (`<>` est aussi accepté) | — |
| `IF A THEN X:Y` | condition inverse + saut (bug 4) | piège |
| `DIM A(10,10)` | `DIM A(121)` et `A(I*11+J+1)` | — |
| `A(0)` | indices à partir de 1 | `RANGE ERR` |
| `SCORE`, `NAME$` | `S`, `N$` : une lettre (+ chiffre) | `SYNTAX ERR` |
| nombres réels | entiers ×10 ou ×100 (virgule fixe) | — |

{{!portage}}

## La méthode

<ol class="steps">
<li><strong>Listez les variables</strong> du programme d’origine et attribuez à chacune une lettre (+ chiffre). Notez la table dans des <code>REM</code> en tête.</li>
<li><strong>Remplacez les DATA</strong> par des affectations dans un sous-programme d’initialisation, ou par une chaîne qu’on découpe.</li>
<li><strong>Cherchez chaque <code>IF … THEN … : …</code></strong> et réécrivez-le avec un saut (bug 4).</li>
<li><strong>Cherchez chaque sortie de <code>FOR</code> par <code>GOTO</code></strong> et remplacez-la par « valeur finale + <code>NEXT</code> » (bug 1).</li>
<li><strong>Décalez les indices de tableau</strong> de 1 si le programme utilise l’indice 0.</li>
<li><strong>Vérifiez dans l’éditeur</strong> : il signale à la frappe les mots inconnus de l’Integer BASIC. <strong>▶ Lancer</strong> tape le programme : toute ligne refusée apparaît à l’écran avec <code>SYNTAX ERR</code>.</li>
<li><strong>Mesurez la mémoire</strong> : <code>apple1-emu basic prog.bas</code> indique la taille et la RAM nécessaire. Trop gros pour 8 Ko ? <code>LOMEM=768</code> (chapitre 17), ou une machine de 16 Ko.</li>
<li><strong>Exportez</strong> la cassette (<strong>📼 Exporter WAV</strong>) et notez les plages pour la vraie machine.</li>
</ol>

## Un exemple de portage

Un fragment typique en Microsoft BASIC :

```text title="Original (Microsoft BASIC)"
100 READ N$,S
110 IF S>HI THEN HI=S:BEST$=N$
120 IF N$<>"FIN" THEN 100
130 PRINT "MEILLEUR : ";BEST$;" (";HI;")"
140 DATA "ANNE",12,"BOB",17,"CLAIRE",9,"FIN",0
```

Sur l’Apple-1 : pas de `READ`/`DATA`, pas de noms longs, et le `IF` de la ligne 110 affecterait `BEST$` à chaque passage (bug 4).

```basic title="Port en Integer BASIC (essayé)"
10 DIM N$(10),B$(10),S(3)
20 S(1)=12:S(2)=17:S(3)=9
30 H=0
40 FOR I=1 TO 3
50 GOSUB 200
60 IF S(I)<=H THEN 80
70 H=S(I):B$=N$
80 NEXT I
90 PRINT "MEILLEUR : ";B$;" (";H;")"
100 END
200 IF I=1 THEN N$="ANNE"
210 IF I=2 THEN N$="BOB"
220 IF I=3 THEN N$="CLAIRE"
230 RETURN
```

```mon
>» RUN
MEILLEUR : BOB (17)
```

## L’expérience des portages

Huit jeux des *101 BASIC Computer Games* (DEC, 1975) ont été portés de cette façon pendant le développement de l’émulateur : BAGLES, MUGWMP, HEX, BULCOW, STARS, TRAP, CHOMP et WAR. Chacun a été joué dans l’émulateur, exporté en cassette et rechargé par la ROM de l’ACI. Ces portages ne sont pas distribués (les listings d’origine ne sont pas libres), mais ils ont fixé les règles de ce chapitre : variables d’une lettre, tableaux à partir de 1, pas de `READ`/`DATA`, pas de `GOTO` hors d’un `FOR`, une seule instruction conditionnée par `IF`, et `LOMEM=768` pour les plus gros.
