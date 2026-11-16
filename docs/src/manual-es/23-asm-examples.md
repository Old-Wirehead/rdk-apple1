# Ejemplos comentados

<p class="lead">Ocho programas, del más sencillo al más completo. Están en la carpeta <code>docs/exemples/es/asm/</code> de la distribución; cada uno se ha ensamblado con el ensamblador integrado y se ha ejecutado en el emulador. Las salidas que se muestran aquí son las del emulador.</p>

Para probarlos: abra el archivo en el editor y haga clic en **▶ Ejecutar**, o, en la línea de comandos, `apple1-emu asm hello.s -o hello.wav` y luego reproduzca el casete (capítulo 5). Deje `apple1.inc` (y `gen2.inc`) en la misma carpeta que las fuentes.

| Archivo | Bytes | Dirección | Qué muestra |
|---|---|---|---|
| `hello.s` | 32 | `$0300` | una cadena, `ECHO`, etiquetas locales, vuelta al monitor |
| `clavier.s` | 32 | `$0300` | leer el teclado, comparar caracteres |
| `hexa.s` | 64 | `$0300` | puntero en página cero, `(zp),Y`, suma de 16 bits, `PRBYTE` |
| `decimal.s` | 117 | `$0300` | números de 16 bits, restas, tablas de palabras |
| `jeu.s` | 237 | `$0300` | un juego completo: azar, subrutinas, mensajes |
| `macros.s` | 135 | `$0300` | macros, `.local`, `.repeat`, `.if`, tablas calculadas |
| `gen2-lores.s` | 110 | `$0C00` | la tarjeta GEN2 en LORES |
| `gen2-hires.s` | 94 | `$0C00` | la tarjeta GEN2 en HIRES |

Las versiones originales de estos programas también se ensamblaron con ca65 (mismos bytes, capítulo 21); las versiones en español solo difieren en los comentarios y los textos.

## `hello.s`: mostrar un mensaje

@@include exemples/es/asm/hello.s

X recorre la cadena hasta el byte 0 que pone `.asciiz`. El `bne @next` tras `inx` nunca es falso para una cadena de menos de 256 caracteres: es un salto corto y seguro. El programa termina con `jmp GETLINE`, nunca con `RTS` (capítulo 22).

```mon
\
» 300R
0300: A2
HOLA, APPLE-1!
```

El monitor muestra primero `0300: A2`, el byte de la dirección de inicio, y luego el programa toma el relevo.

## `clavier.s`: repetir las teclas

@@include exemples/es/asm/clavier.s

`cmp #'0' | $80` compara con el código de la cifra 0, con el bit 7 a uno como lo da el teclado. `bcc` («menor») y `bcs` («mayor o igual») enmarcan las cifras. Con las teclas `A`, `B`, `1`, `2` y luego ESC, la pantalla muestra:

```text
0300: ADAB1122
\
```

El `AD` viene del monitor (`lda KBDCR` empieza por el opcode `$AD`); `AB`, luego `11` y `22` vienen del programa; ESC devuelve el control al monitor, que muestra `\`.

## `hexa.s`: un volcado al estilo del monitor

@@include exemples/es/asm/hexa.s

`PTR` y `PTR+1` forman un **puntero de 16 bits** en página cero, byte bajo primero; `lda (PTR),y` lee el byte de la dirección PTR + Y. La suma `PTR += 8` muestra el acarreo en acción: `clc`, `adc #8`, y si sale un acarreo (`bcc` no tomado), `inc PTR+1`. El resultado es el principio del Woz Monitor:

```text
FF00: D8 58 A0 7F 8C 12 D0 A9
FF08: A7 8D 11 D0 8D 13 D0 C9
```

## `decimal.s`: mostrar un número en decimal

@@include exemples/es/asm/decimal.s

El 6502 no sabe dividir: `prdec` resta 10000 tantas veces como puede (la cuenta da la cifra de las decenas de millar), luego 1000, 100 y 10. La resta de 16 bits se hace en dos pasos, `sbc` sobre el byte bajo y luego sobre el alto, enlazados por el acarreo. `FLAG` suprime los ceros a la izquierda. Salida:

```text
0 7 42 1976 6502 65535
```

!!! astuce "Conservar X en una subrutina"
    `prdec` usa X para sí mismo, mientras que el bucle principal lo necesita: lo guarda con `txa` / `pha` al principio y lo restaura con `pla` / `tax` al final. El 6502 NMOS no tiene `PHX`.

## `jeu.s`: adivinar la cifra

@@include exemples/es/asm/jeu.s

El azar viene del jugador: X cuenta sin parar (`@wait`) hasta la primera tecla, y el resto de la división por 10 da la cifra secreta. `prmsg` recibe la dirección de un mensaje en **Y:X** (byte alto en Y), la guarda en el puntero `MSG` y la muestra con `(MSG),y`. Una partida, tecleando `x` y luego las cifras en orden:

```text
HE ELEGIDO UNA CIFRA. PULSE UNA TECLA.
TU CIFRA? 0 MAS ALTO.
TU CIFRA? 1 MAS ALTO.
…
TU CIFRA? 5 MAS ALTO.
TU CIFRA? 6 GANADO EN 7 INTENTOS!
```

## `macros.s`: dejar trabajar al ensamblador

@@include exemples/es/asm/macros.s

`putc` y `print` generan código en cada llamada; `.local` impide que las dos llamadas a `print` definan dos veces `loop`. `.repeat 3` despliega tres veces la línea de asteriscos (cada una con su propia etiqueta anónima), `.if` elige el mensaje según `WIDTH`, y la tabla `squares` se calcula al ensamblar. Salida:

```text
MACROS Y .REPEAT
********************
********************
********************
LINEAS CORTAS
```

Cambie `WIDTH = 20` por `40`: el editor vuelve a ensamblar enseguida, las líneas se alargan y el mensaje pasa a ser `LINEAS LARGAS`.

## `gen2-lores.s`: color en baja resolución

Estos dos programas necesitan la tarjeta GEN2 (**⚙ Preferencias ▸ Tarjeta de color Uncle Bernie GEN2**, capítulo 6) y empiezan en `$0C00`, por encima de la página LORES.

@@include exemples/es/asm/gen2-lores.s

Las dos tablas `rows_lo` y `rows_hi` las calcula `.repeat` con la fórmula de las direcciones de fila del capítulo 6. Cada byte lleva dos bloques, de ahí el nibble copiado en la mitad superior y en la inferior. Las instrucciones `bit SS_…` son **lecturas**: así se conmutan los modos de la tarjeta.

![gen2-lores.s: los dieciséis colores LORES, dos columnas cada uno, en el monitor en color.](img/es/brut/gen2-lores.png "large")

## `gen2-hires.s`: alta resolución

@@include exemples/es/asm/gen2-hires.s

En HIRES, la página `$2000-$3FFF` se divide en bloques de 128 bytes cuyos 40 primeros bytes van al tercio superior de la pantalla, los 40 siguientes al centro y los 40 posteriores abajo. El programa llena cada tercio con un motivo distinto, alternando dos bytes según la paridad de la columna: el resultado son tres franjas de color.

![gen2-hires.s: tres franjas de color en HIRES: violeta, verde y azul.](img/es/brut/gen2-hires.png "large")
