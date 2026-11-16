# El depurador, el desensamblado y el volcado

<p class="lead">El depurador detiene el Apple-1 y lo hace avanzar paso a paso, una instrucción cada vez en 6502 o una línea cada vez en BASIC, mostrando los registros, las variables y la memoria.</p>

## El panel del depurador

**🐛 Depurador** abre el panel, acoplado a la derecha o separable como el editor. Sus comandos:

| Comando | Efecto |
|---|---|
| **⏸ Pausa** / **▶ Continuar** | detiene o reanuda el Apple-1 |
| **⬇ Paso** | ejecuta una instrucción |
| **↪ Saltar** | ejecuta un `JSR` de una vez y se detiene al volver |
| **⤴ Salir** | ejecuta hasta el `RTS` de la subrutina actual |
| **⏭ Línea BASIC** | avanza una línea del programa BASIC |
| **⏩ Hasta el cursor** | (en el editor) ejecuta hasta la línea del cursor |

{{!Paso (depurador)}}{{!Saltar}}{{!Salir}}

El marco **Procesador** muestra A, X, Y, S, PC y los indicadores N V B D I Z C; todos se pueden cambiar haciendo clic en el valor. Debajo aparece el número de ciclos e instrucciones ejecutados.

## Puntos de parada

Un punto de parada se puede poner de cuatro maneras:

- **⏺ Punto de parada** en el editor (o <kbd>F9</kbd>, o el menú del clic derecho), en la línea del cursor: en una fuente `.s` (gracias a la tabla de direcciones del ensamblador) o en un programa BASIC (número de línea);
- un clic en el margen del **📃 Listado**;
- un clic en una línea del desensamblado;
- escribir una dirección o una etiqueta: `$0368`, `ASK`, `PUTS+3`, o `L190` para la línea BASIC 190. Una etiqueta que todavía no existe no se rechaza: el punto de parada se conserva y se coloca en cuanto la etiqueta existe (cuando ensambla la fuente del editor o la ejecuta), con un mensaje que lo indica.

Los puntos de parada, las variables vigiladas y las vigilancias de memoria se recuerdan de una sesión a otra, pero cargar otro programa borra los puntos de parada y las etiquetas del anterior. Cuando está cargado el BASIC integrado, la sección BASIC del depurador se titula **RDK G2 Basic**; sin ningún BASIC en `$E000`, lo dice de forma neutra. Los colores de la línea actual, de las direcciones del volcado y de la columna ASCII se cambian en **⚙ Preferencias → Interfaz** («Colores de la interfaz»). En los paneles estrechos desaparece la columna de bytes del desensamblado y la fila de escritura pasa a la línea siguiente en lugar de cortarse.

## Depurar un programa BASIC

![boucle.bas detenido en un punto de parada en la línea 30: la línea actual en verde en el editor, las variables I y S vigiladas en el depurador.](img/es/brut/debogueur-basic.png "large")

La línea actual se resalta en verde en el editor, los puntos de parada en violeta. El marco BASIC (**RDK G2 Basic** o **Integer BASIC**, según el que esté cargado) muestra la línea actual, los puntos de parada en números de línea y las **variables vigiladas**: enteras (`A`, `B1`), elementos de tabla (`D(2)`), tablas enteras (`D`), cadenas (`C$`), más **Todas las variables**.

La depuración funciona con **RDK G2 Basic**, el BASIC integrado en el emulador, igual que con el original: ya no hace falta el Integer BASIC real. El depurador detecta el inicio de cada línea (la línea actual, los puntos de parada de línea, **⏭ Línea BASIC**) y lee las variables en las tablas del intérprete. En el original lee la línea en los punteros de página cero `$DC`/`$DD`; G2 Basic la guarda en otro sitio, y el depurador conoce ambas disposiciones.

!!! attention "Una tecla detiene el programa"
    Como en la máquina real, una tecla pulsada mientras el programa BASIC se ejecuta lo detiene (`STOPPED AT`). Durante una sesión de depuración, mantenga el foco del teclado en el panel del depurador, no en la pantalla del Apple-1. Un programa G2 Basic puede leer el teclado sin detenerse con `KEY(0)` (capítulo 15).

## Depurar un programa 6502

![decimal.s detenido en la etiqueta prdec: registros, desensamblado con las etiquetas del programa, vigilancia de memoria $06.$08.](img/es/brut/debogueur-asm.png "large")

El **desensamblado** muestra 24 líneas alrededor de PC, con las etiquetas del programa ensamblado en su propia columna. Solo se recentra cuando PC sale de la zona visible, así que la visualización no salta a cada paso. **Ir** fija la lista en una dirección o una etiqueta; **Seguir PC** hace que vuelva a seguir la ejecución.

Cuando selecciona una instrucción en la fuente 6502 del editor, sus bytes también se **resaltan** en el desensamblado y en el volcado (capítulo 7), y ambos paneles se desplazan hasta ellos si no están a la vista; el desensamblado deja entonces de seguir PC hasta que pulse **Seguir PC**.

La **vigilancia de memoria** muestra un byte, una palabra o ASCII (`$30`, `$30 w`, `0200.020F`, una etiqueta), modificable, en amarillo cuando el valor cambia.

## El volcado

El **Volcado** muestra la memoria en hexadecimal y ASCII, 16 bytes por línea (8 si el panel es estrecho), con el bit 7 quitado para el ASCII. **Ir** salta a una dirección o a los atajos ZP, `$0200`, `$0300`, PC, `$E000`. Los bytes cambiados aparecen en amarillo, el byte de PC sobre fondo verde. **Escribir** deposita bytes (`0300 = A9 00`); un clic en un byte rellena la línea de escritura.

El desensamblado y el volcado son paneles por derecho propio: separados por defecto, se pueden acoplar junto al depurador (capítulo 10).

## En la línea de comandos

Para demostraciones y pruebas automáticas: `--debogueur` abre el panel, `--dbg-arret ASK,L190` pone puntos de parada, `--dbg-surveiller 'A,C$,$30 w'` vigila variables y memoria, y `--dbg-commande 5:pas` envía un comando al cabo de 5 segundos (`pause`, `continuer`, `pas`, `dessus`, `sortir`, `basic`). Estos nombres de opciones y palabras clave están en francés.
