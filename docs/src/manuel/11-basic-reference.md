# Référence des instructions et des fonctions

<p class="lead">Toutes les instructions reconnues par l’Integer BASIC de l’Apple-1, d’après la table de mots-clés de l’interpréteur, avec ce qu’elles font vraiment. Chaque exemple a été essayé.</p>

## Les instructions

`PRINT` liste `[;]`
{: .syntax }

{{PRINT}} affiche des nombres et des chaînes. `;` colle les éléments sans espace (`PRINT A;B` affiche `12` pour A=1, B=2) ; `,` passe à la colonne suivante, de 8 en 8 (`PRINT 1,2,3` affiche `1       2       3`). Un `;` final supprime le retour à la ligne. `PRINT` seul passe à la ligne.

`INPUT` `["invite",]` variable `[,variable…]`
{: .syntax }

{{INPUT}} lit un nombre ou une chaîne. Le BASIC affiche toujours `?` après l’invite : `INPUT "NOM",N$` affiche `NOM?`. Une chaîne trop longue pour son `DIM` donne `STR OVFL ERR`.

`LET` variable `=` expression
{: .syntax }

{{LET}} est facultatif : `A=5` suffit.

`IF` expression `THEN` instruction | numéro
{: .syntax }

{{IF}} exécute l’instruction quand l’expression n’est pas nulle (`IF A THEN …` est permis). `THEN 100` saute à la ligne 100. **Attention** : seule la première instruction dépend de la condition (chapitre 18, bug 4).

`GOTO` expression — `GOSUB` expression — `RETURN`
{: .syntax }

{{GOTO}} et {{GOSUB}} acceptent une **expression** : `GOTO K*100`, `GOSUB 300+K` (le « GOTO calculé » remplace le `ON … GOTO` absent). Une ligne inexistante donne `BAD BRANCH ERR`. Huit {{GOSUB}} imbriqués au plus (`>8 GOSUBS ERR` au neuvième), `RETURN` sans `GOSUB` donne `BAD RETURN ERR`.

`FOR` var `=` début `TO` fin `[STEP pas]` … `NEXT` var
{: .syntax }

{{FOR}}/{{NEXT}} : le corps est exécuté **au moins une fois**, même si début dépasse fin (`FOR I=5 TO 1` passe une fois). `STEP` peut être négatif. Huit boucles imbriquées au plus (`>8 FORS ERR`). `NEXT` doit nommer la variable ; un `NEXT` sans `FOR` donne `BAD NEXT ERR`.

`DIM` var`(n)` `[,var$(n)…]`
{: .syntax }

{{DIM}} crée un tableau d’entiers de `n` éléments, numérotés **de 1 à n** (`A(0)` donne `RANGE ERR`), ou réserve `n` caractères (au plus 255) pour une chaîne. Toute chaîne doit être dimensionnée avant d’être affectée.

`REM` texte
{: .syntax }

{{REM}} : commentaire, jusqu’à la fin de la ligne (un `:` après REM fait partie du commentaire).

`END`
{: .syntax }

{{END}} termine le programme. Un programme qui se termine sans `END` affiche `*** END ERR` : ce n’est pas grave, mais terminez vos programmes par `END`.

`TAB` n
{: .syntax }

{{TAB}} est une **instruction** (et non une fonction de PRINT comme en Applesoft) : `TAB 10:PRINT "*"` affiche l’étoile en colonne 10.

`POKE` adresse`,`valeur
{: .syntax }

{{POKE}} écrit un octet (0 à 255, sinon `>255 ERR`). Les adresses au-delà de 32767 s’écrivent en négatif : adresse − 65536.

`CALL` adresse
{: .syntax }

{{CALL}} appelle un programme en langage machine, qui revient par `RTS`. Exemple : `POKE 768,96 : CALL 768` (96 = `RTS`). `CALL -256` revient au Woz Monitor proprement (`$FF00`).

## Les fonctions

| Fonction | Résultat | Exemple vérifié |
|---|---|---|
| {{PEEK}}`(adr)` | l’octet à l’adresse | `PEEK(76)+PEEK(77)*256` donne HIMEM (4096) |
| {{RND}}`(n)` | entier aléatoire de 0 à n−1 ; `RND(0)` : `>32767 ERR` ; `RND(-5)` donne un nombre négatif | `RND(6)+1` : un dé |
| {{SGN}}`(x)` | −1, 0 ou 1 (aussi pour un nombre de 256 ou plus) | `SGN(-5);SGN(0)` affiche `-10` |
| {{ABS}}`(x)` | valeur absolue | `ABS(-9)` = 9 |
| `KEY(n)` (G2 Basic seulement) | code de la touche frappée, sans attendre (voir plus bas) | `K=KEY(0)` |
| {{LEN}}`(a$)` | longueur de la chaîne | `LEN("ABC")` = 3 |
| {{USR}}`(x)` | appelle `$0000` : **plante** sans programme préparé | — |
| {{ASC}}`(`, {{SCRN}}`(` | présents dans la table, mais `SYNTAX ERR` | — |

## Les opérateurs

| Priorité | Opérateurs |
|---|---|
| 1 | `-` et `+` unaires, `NOT` |
| 2 | `*` `/` `MOD` |
| 3 | `+` `-` |
| 4 | `=` `#` (différent, aussi `<>`) `<` `>` `<=` `>=` |
| 5 | `AND` |
| 6 | `OR` |

{{MOD}}, {{AND}}, {{OR}} et {{NOT}} :

- La division **tronque vers zéro** : `7/2` = 3, `-7/2` = −3. `MOD` prend le signe du dividende : `-7 MOD 3` = −1.
- Les comparaisons donnent 1 (vrai) ou 0 (faux) : `PRINT 5>3;5=3` affiche `10`. `NOT 0` = 1, `NOT 5` = 0.
- `AND` et `OR` sont logiques, pas bit à bit : `3 AND 0` = 0, `3 OR 0` = 1.
- `3*-2` est accepté et vaut −6.
- `^` existe dans la table, mais **bloque la machine** (chapitre 18, bug 2).
- Les chaînes ne se comparent qu’avec `=` et `#` ; `"ABC"<"ABD"` est refusé (`SYNTAX ERR`).

## Les extensions du RDK G2 Basic

Elles ne marchent qu’avec le **RDK G2 Basic** (l’Integer BASIC d’origine répond `SYNTAX ERR`).

`KEY(n)`
{: .syntax }

`KEY(0)` rend le **code de la touche** frappée (le code ASCII sur 7 bits : 65 pour `A`, 13 pour Retour, 32 pour la barre d’espace) et la consomme, ou `0` si aucune touche n’a été frappée. Elle n’attend jamais : un programme peut donc tourner tout en surveillant le clavier. `KEY(1)` (tout autre argument non nul) **attend** une touche.

```basic title="Attendre une touche sans INPUT"
10 K=KEY(0)
20 IF K=0 THEN 10
30 PRINT K
```

Dans l’original, une touche frappée pendant qu’un programme tourne **l’arrête** (`STOPPED AT n`), et `INPUT` bloque jusqu’à Retour. Avec le G2 Basic, dès qu’un programme a appelé `KEY`, une touche ne l’arrête plus : seule `KEY` voit le clavier, et <kbd>Ctrl</kbd>+<kbd>C</kbd> (code 3) lu par `KEY` arrête le programme au prochain appel. Appelez donc `KEY` dans la boucle principale du programme.

`LOMEM=` n — `HIMEM=` n
{: .syntax }

Des commandes immédiates qui prennent une expression (`LOMEM=2048+512`) ; dans un programme elles donnent `SYNTAX ERR`, comme dans l’original. Elles sont décrites avec la mémoire au chapitre 16.

`LIST` `[`a`[,`b`]]` — `DEL` a`[,`b`]`
{: .syntax }

`LIST` liste tout, `LIST n` une ligne, `LIST a,b` les lignes de `a` à `b`. `DEL n` supprime une ligne, `DEL a,b` les lignes de `a` à `b` (si `b` est inférieur à `a`, seule la ligne `a`). Une ligne listée plus longue que l’écran passe à la ligne comme dans l’original (les lignes de suite sont décalées de six espaces).

Les tranches de chaînes suivent l’original : `A$(I)` avec `I` de 256 ou plus, ou négatif, donne `>255 ERR`, et `A$(I,J)` avec `J` inférieur à `I-1` donne `STRING ERR`.

## Les mots-clés sans effet

La table contient `COLOR=`, `PLOT`, `HLIN`, `VLIN` et `VTAB`, hérités du projet de BASIC pour l’Apple II. Sur l’Apple-1, `COLOR=` et `PLOT` sont acceptés sans rien faire, `VTAB` donne `SYNTAX ERR`. N’en mettez pas dans vos programmes.
