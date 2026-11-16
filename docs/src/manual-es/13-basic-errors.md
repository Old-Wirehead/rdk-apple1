# Mensajes de error y límites

## Mensajes

Un error muestra `*** MENSAJE ERR`, seguido de `STOPPED AT n` cuando hay un programa en marcha. Una línea rechazada al introducirla no se guarda: vuelva a escribirla.

| Mensaje | Causa | Ejemplo probado |
|---|---|---|
| {{SYNTAX ERR}} | instrucción desconocida, nombre de variable no válido, `:` en modo directo | `CNT=7`, `GET A$`, `X=5:PRINT X` en modo directo |
| {{>32767 ERR}} | desbordamiento, división por cero, número de línea > 32767 | `200*200`, `10/0`, `RND(0)`, `32768 PRINT` |
| {{TOO LONG ERR}} | línea demasiado compleja (expresión anidada en exceso) o demasiado larga | 13 paréntesis anidados |
| {{MEM FULL ERR}} | no queda sitio entre las variables y el programa | `DIM A(3000)` en 8 KB |
| {{TOO MANY PARENS ERR}} | paréntesis (previsto por el intérprete; lo enmascara el fallo 3) | — |
| {{STRING ERR}} | cadena mal usada | — |
| {{END ERR}} | el programa termina sin `END` | `10 PRINT "X"` y luego `RUN` |
| {{BAD BRANCH ERR}} | `GOTO`/`GOSUB` a una línea inexistente | `GOTO 500` |
| {{>8 GOSUBS ERR}} | más de 8 `GOSUB` anidados | recursión: 9 llamadas |
| {{BAD RETURN ERR}} | `RETURN` sin `GOSUB` | `RUN 30` sobre un `RETURN` |
| {{>8 FORS ERR}} | más de 8 `FOR` anidados, o el fallo 1 | 9 bucles en una línea |
| {{BAD NEXT ERR}} | `NEXT` sin el `FOR` correspondiente | `20 NEXT I` solo |
| {{STOPPED AT}} | se pulsó una tecla durante la ejecución | — |
| {{>255 ERR}} | valor > 255: `POKE`, `DIM` de una cadena | `POKE 768,300`, `DIM B$(256)` |
| {{RANGE ERR}} | índice de matriz fuera de límites | `A(0)`, `A(3)` tras `DIM A(2)` |
| {{DIM ERR}} | matriz o cadena dimensionada de nuevo | — |
| {{STR OVFL ERR}} | cadena más larga que su `DIM` | `A$="ABCDE"` con `DIM A$(3)` |
| {{RETYPE LINE}} | respuesta no válida a `INPUT` (letras para un número): vuelva a escribir | — |

Las filas con «—» vienen de la tabla de mensajes del intérprete; no se provocaron en nuestras pruebas.

## Límites

| Límite | Valor |
|---|---|
| enteros | −32767 a 32767 |
| números de línea | 0 a 32767 |
| nombres de variable | letra + cifra opcional |
| nombres de cadena | una letra + `$` |
| longitud de una cadena | 255 |
| dimensiones de una matriz | 1, índices de 1 a n |
| `FOR` anidados | 8 |
| `GOSUB` anidados | 8 |
| paréntesis anidados | 12 |
| longitud de una línea tecleada | pasan 120 caracteres, se rechazan 141 |


## Ahorrar memoria

Con 8 KB, el BASIC tiene unos 2 KB para el programa y las variables. Algunas formas de ganar espacio:

<ol class="steps">
<li><strong>Bajar LOMEM</strong> si no tiene código máquina: <code>LOMEM=768</code> (<code>$0300</code>) da 1280 bytes más a las variables. Probado: <code>DIM T(1200)</code> falla con <code>MEM FULL</code> con el LOMEM por defecto, y funciona tras <code>LOMEM=768</code>. Las adaptaciones de *101 BASIC Computer Games* hechas durante el desarrollo hacen justo eso.</li>
<li><strong>Escribir de forma compacta</strong>: el BASIC conserva los espacios y los <code>REM</code>. <code>LIST</code> vuelve a sangrar por sí solo, y una línea larga con varias instrucciones cuesta menos que varias líneas.</li>
<li><strong>Reutilizar matrices y cadenas</strong> en vez de dimensionar otras nuevas.</li>
<li><strong>Añadir RAM</strong>: los ajustes ⚙ de un programa, <code>--ram 16</code>, o una ampliación real. Un programa de más de 8 KB exige una máquina de 12, 16 o 32 KB: <code>apple1-emu basic --ram 16</code> lo coloca en lo alto de esa memoria.</li>
</ol>
