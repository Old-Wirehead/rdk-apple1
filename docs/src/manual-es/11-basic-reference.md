# Referencia de instrucciones y funciones

<p class="lead">Todas las instrucciones que reconoce el Integer BASIC del Apple-1, sacadas de la tabla de palabras clave del intérprete, con lo que hacen de verdad. Todos los ejemplos se han probado.</p>

## Instrucciones

`PRINT` lista `[;]`
{: .syntax }

{{PRINT}} muestra números y cadenas. `;` une los elementos sin espacio (`PRINT A;B` muestra `12` para A=1, B=2); `,` pasa a la columna siguiente, de 8 en 8 (`PRINT 1,2,3` muestra `1       2       3`). Un `;` final suprime el salto de línea. `PRINT` solo pasa a la línea siguiente.

`INPUT` `["indicación",]` variable `[,variable…]`
{: .syntax }

{{INPUT}} lee un número o una cadena. El BASIC siempre muestra `?` tras la indicación: `INPUT "NOMBRE",N$` muestra `NOMBRE?`. Una cadena demasiado larga para su `DIM` da `STR OVFL ERR`.

`LET` variable `=` expresión
{: .syntax }

{{LET}} es opcional: basta con `A=5`.

`IF` expresión `THEN` instrucción | número
{: .syntax }

{{IF}} ejecuta la instrucción cuando la expresión no es cero (`IF A THEN …` está permitido). `THEN 100` salta a la línea 100. **Cuidado**: solo la primera instrucción depende de la condición (capítulo 18, fallo 4).

`GOTO` expresión — `GOSUB` expresión — `RETURN`
{: .syntax }

{{GOTO}} y {{GOSUB}} aceptan una **expresión**: `GOTO K*100`, `GOSUB 300+K` (el «GOTO calculado» sustituye al `ON … GOTO` que falta). Una línea inexistente da `BAD BRANCH ERR`. Como mucho ocho {{GOSUB}} anidados (`>8 GOSUBS ERR` en el noveno); `RETURN` sin `GOSUB` da `BAD RETURN ERR`.

`FOR` var `=` inicio `TO` fin `[STEP paso]` … `NEXT` var
{: .syntax }

{{FOR}}/{{NEXT}}: el cuerpo se ejecuta **al menos una vez**, aunque el inicio esté más allá del fin (`FOR I=5 TO 1` pasa una vez). `STEP` puede ser negativo. Como mucho ocho bucles anidados (`>8 FORS ERR`). `NEXT` debe nombrar la variable; un `NEXT` sin `FOR` da `BAD NEXT ERR`.

`DIM` var`(n)` `[,var$(n)…]`
{: .syntax }

{{DIM}} crea una matriz de `n` enteros, numerados **de 1 a n** (`A(0)` da `RANGE ERR`), o reserva `n` caracteres (como mucho 255) para una cadena. Toda cadena debe dimensionarse antes de asignarla.

`REM` texto
{: .syntax }

{{REM}}: un comentario, hasta el final de la línea (un `:` tras REM forma parte del comentario).

`END`
{: .syntax }

{{END}} termina el programa. Un programa que acaba sin `END` muestra `*** END ERR`: no tiene consecuencias, pero termine sus programas con `END`.

`TAB` n
{: .syntax }

{{TAB}} es una **instrucción** (no una función de PRINT como en Applesoft): `TAB 10:PRINT "*"` pone el asterisco en la columna 10.

`POKE` dirección`,`valor
{: .syntax }

{{POKE}} escribe un byte (0 a 255, si no `>255 ERR`). Las direcciones por encima de 32767 se escriben como números negativos: dirección − 65536.

`CALL` dirección
{: .syntax }

{{CALL}} llama a un programa en código máquina, que vuelve con `RTS`. Ejemplo: `POKE 768,96 : CALL 768` (96 = `RTS`). `CALL -256` vuelve limpiamente al Woz Monitor (`$FF00`).

## Funciones

| Función | Resultado | Ejemplo comprobado |
|---|---|---|
| {{PEEK}}`(dir)` | el byte de la dirección | `PEEK(76)+PEEK(77)*256` da HIMEM (4096) |
| {{RND}}`(n)` | entero aleatorio de 0 a n−1; `RND(0)`: `>32767 ERR`; `RND(-5)` da un número negativo | `RND(6)+1`: un dado |
| {{SGN}}`(x)` | −1, 0 o 1 (también para un número de 256 o más) | `SGN(-5);SGN(0)` muestra `-10` |
| {{ABS}}`(x)` | valor absoluto | `ABS(-9)` = 9 |
| `KEY(n)` (solo G2 Basic) | código de la tecla pulsada, sin esperar (ver más abajo) | `K=KEY(0)` |
| {{LEN}}`(a$)` | longitud de la cadena | `LEN("ABC")` = 3 |
| {{USR}}`(x)` | llama a `$0000`: **se cuelga** sin un programa preparado | — |
| {{ASC}}`(`, {{SCRN}}`(` | presentes en la tabla, pero `SYNTAX ERR` | — |

## Operadores

| Prioridad | Operadores |
|---|---|
| 1 | `-` y `+` unarios, `NOT` |
| 2 | `*` `/` `MOD` |
| 3 | `+` `-` |
| 4 | `=` `#` (distinto, también `<>`) `<` `>` `<=` `>=` |
| 5 | `AND` |
| 6 | `OR` |

{{MOD}}, {{AND}}, {{OR}} y {{NOT}}:

- La división **trunca hacia cero**: `7/2` = 3, `-7/2` = −3. `MOD` toma el signo del dividendo: `-7 MOD 3` = −1.
- Las comparaciones dan 1 (verdadero) o 0 (falso): `PRINT 5>3;5=3` muestra `10`. `NOT 0` = 1, `NOT 5` = 0.
- `AND` y `OR` son lógicos, no bit a bit: `3 AND 0` = 0, `3 OR 0` = 1.
- `3*-2` se acepta y vale −6.
- `^` existe en la tabla, pero **cuelga la máquina** (capítulo 18, fallo 2).
- Las cadenas solo se pueden comparar con `=` y `#`; `"ABC"<"ABD"` se rechaza (`SYNTAX ERR`).

## Extensiones de RDK G2 Basic

Solo funcionan con **RDK G2 Basic** (el Integer BASIC original responde `SYNTAX ERR`).

`KEY(n)`
{: .syntax }

`KEY(0)` devuelve el **código de la tecla** pulsada (el código ASCII de 7 bits: 65 para `A`, 13 para Return, 32 para la barra espaciadora) y la consume, o `0` si no se ha pulsado ninguna. Nunca espera, así que un programa puede seguir funcionando mientras vigila el teclado. `KEY(1)` (cualquier otro argumento distinto de cero) **espera** una tecla.

```basic title="Esperar una tecla sin INPUT"
10 K=KEY(0)
20 IF K=0 THEN 10
30 PRINT K
```

En el original, una tecla pulsada mientras un programa se ejecuta **lo detiene** (`STOPPED AT n`), e `INPUT` bloquea hasta Return. Con G2 Basic, en cuanto un programa ha llamado a `KEY`, una pulsación ya no lo detiene: solo `KEY` ve el teclado, y un <kbd>Ctrl</kbd>+<kbd>C</kbd> (código 3) leído por `KEY` detiene el programa en la llamada siguiente. Llame, pues, a `KEY` en el bucle principal del programa.

`LOMEM=` n — `HIMEM=` n
{: .syntax }

Comandos inmediatos que aceptan una expresión (`LOMEM=2048+512`); dentro de un programa dan `SYNTAX ERR`, como en el original. Se describen con la memoria en el capítulo 16.

`LIST` `[`a`[,`b`]]` — `DEL` a`[,`b`]`
{: .syntax }

`LIST` lo lista todo, `LIST n` una línea, `LIST a,b` las líneas de `a` a `b`. `DEL n` borra una línea, `DEL a,b` las líneas de `a` a `b` (si `b` es menor que `a`, solo la línea `a`). Una línea listada más larga que la pantalla se parte como en el original (las líneas de continuación van sangradas seis espacios).

Los trozos de cadena siguen al original: `A$(I)` con `I` de 256 o más, o negativo, da `>255 ERR`, y `A$(I,J)` con `J` menor que `I-1` da `STRING ERR`.

## Palabras clave que no hacen nada

La tabla contiene `COLOR=`, `PLOT`, `HLIN`, `VLIN` y `VTAB`, heredadas del proyecto de BASIC para el Apple II. En el Apple-1, `COLOR=` y `PLOT` se aceptan y no hacen nada, `VTAB` da `SYNTAX ERR`. No las use en sus programas.
