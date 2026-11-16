# Adaptar un programa de Applesoft o Microsoft BASIC

<p class="lead">La mayoría de los listados de los años 1970 y 1980 están escritos en Microsoft BASIC (Applesoft, el BASIC de «101 BASIC Computer Games»). Se pueden adaptar al Apple-1, siempre que se conozcan las equivalencias. Cada línea marcada abajo como «rechazada» se escribió en el BASIC real.</p>

## Equivalencias

| Applesoft / Microsoft | Integer BASIC del Apple-1 | Nota |
|---|---|---|
| `ON X GOTO 100,200,300` | `GOTO 100*X` (si los números de línea lo permiten) o una serie de `IF X=1 THEN 100` | rechazada |
| `GET A$` | `INPUT A$` | rechazada |
| `DATA` / `READ` / `RESTORE` | matriz llenada por asignaciones, o una cadena cortada | rechazada |
| `MID$(A$,I,N)` | `A$(I,I+N-1)` | rechazada |
| `LEFT$(A$,N)` / `RIGHT$(A$,N)` | `A$(1,N)` / `A$(LEN(A$)-N+1)` | — |
| `A$+B$` | `A$(LEN(A$)+1)=B$` | — |
| `CHR$(N)`, `ASC(A$)` | cadena de referencia (capítulo 18, fallo 6) | rechazada |
| `STR$(N)`, `VAL(A$)` | `PRINT N` directamente; `INPUT N` | rechazada |
| `INT(X)` | innecesario: todo es entero | rechazada |
| `RND(1)` (real de 0 a 1) | `RND(N)` (entero de 0 a N−1) | — |
| `X^2` | `X*X` | se cuelga (fallo 2) |
| `HOME` | 24 × `PRINT`, o CLEAR SCREEN (<kbd>Ctrl</kbd>+<kbd>L</kbd>) a mano | rechazada |
| `PRINT TAB(10);"X"` | `TAB 10:PRINT "X"` | — |
| `STOP` | `END` | rechazada |
| `DEF FN` | `GOSUB` | — |
| `A<>B` | `A#B` (`<>` también se acepta) | — |
| `IF A THEN X:Y` | condición invertida + salto (fallo 4) | trampa |
| `DIM A(10,10)` | `DIM A(121)` y `A(I*11+J+1)` | — |
| `A(0)` | los índices empiezan en 1 | `RANGE ERR` |
| `SCORE`, `NAME$` | `S`, `N$`: una letra (+ cifra) | `SYNTAX ERR` |
| números reales | enteros ×10 o ×100 (coma fija) | — |

{{!adaptación}}

## El método

<ol class="steps">
<li><strong>Haga la lista de las variables</strong> del programa original y asigne a cada una una letra (+ cifra). Anote la tabla en <code>REM</code> al principio.</li>
<li><strong>Sustituya los DATA</strong> por asignaciones en una subrutina de inicialización, o por una cadena que se corta.</li>
<li><strong>Localice cada <code>IF … THEN … : …</code></strong> y reescríbalo con un salto (fallo 4).</li>
<li><strong>Localice cada salida de un <code>FOR</code> con <code>GOTO</code></strong> y sustitúyala por «valor final + <code>NEXT</code>» (fallo 1).</li>
<li><strong>Desplace los índices de las matrices</strong> en 1 si el programa usa el índice 0.</li>
<li><strong>Compruebe en el editor</strong>: marca mientras escribe las palabras que el Integer BASIC no conoce. <strong>▶ Ejecutar</strong> teclea el programa: cualquier línea rechazada aparece en pantalla con <code>SYNTAX ERR</code>.</li>
<li><strong>Mida la memoria</strong>: <code>apple1-emu basic prog.bas</code> da el tamaño y la RAM necesaria. ¿Demasiado grande para 8 KB? <code>LOMEM=768</code> (capítulo 17), o una máquina de 16 KB.</li>
<li><strong>Exporte</strong> el casete (<strong>📼 Exportar WAV</strong>) y anote los rangos para la máquina real.</li>
</ol>

## Un ejemplo de adaptación

Un fragmento típico en Microsoft BASIC:

```text title="Original (Microsoft BASIC)"
100 READ N$,S
110 IF S>HI THEN HI=S:BEST$=N$
120 IF N$<>"END" THEN 100
130 PRINT "MEJOR: ";BEST$;" (";HI;")"
140 DATA "ANA",12,"BRUNO",17,"CLARA",9,"END",0
```

En el Apple-1: ni `READ`/`DATA`, ni nombres largos, y el `IF` de la línea 110 asignaría `BEST$` en cada pasada (fallo 4).

```basic title="Adaptación a Integer BASIC (probada)"
10 DIM N$(10),B$(10),S(3)
20 S(1)=12:S(2)=17:S(3)=9
30 H=0
40 FOR I=1 TO 3
50 GOSUB 200
60 IF S(I)<=H THEN 80
70 H=S(I):B$=N$
80 NEXT I
90 PRINT "MEJOR: ";B$;" (";H;")"
100 END
200 IF I=1 THEN N$="ANA"
210 IF I=2 THEN N$="BRUNO"
220 IF I=3 THEN N$="CLARA"
230 RETURN
```

```mon
>» RUN
MEJOR: BRUNO (17)
```

## Lecciones de las adaptaciones

Ocho juegos de *101 BASIC Computer Games* (DEC, 1975) se adaptaron así durante el desarrollo del emulador: BAGLES, MUGWMP, HEX, BULCOW, STARS, TRAP, CHOMP y WAR. Cada uno se jugó en el emulador, se exportó a casete y se volvió a cargar a través de la ROM de la ACI. Estas adaptaciones no se distribuyen (los listados originales no son libres), pero fijaron las reglas de este capítulo: variables de una letra, matrices que empiezan en 1, nada de `READ`/`DATA`, ningún `GOTO` fuera de un `FOR`, una sola instrucción gobernada por `IF`, y `LOMEM=768` para los más grandes.
