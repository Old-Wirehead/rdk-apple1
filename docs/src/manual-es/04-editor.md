# El editor

<p class="lead">El editor integrado escribe programas en Integer BASIC y fuentes 6502, los comprueba a cada pulsación, los ejecuta en el Apple-1 emulado con un clic y los exporta como casetes para la máquina real.</p>

## Pestañas y archivos

**📝 Editor** abre el panel, acoplado a la izquierda de la pantalla. Su barra de herramientas es compacta, para dejar el máximo de líneas al texto. Empieza con iconos, explicados por sus burbujas:

| Icono | Acción |
|---|---|
| **Nuevo** (primer icono) | Pestaña nueva: «Programa BASIC» o «Fuente en ensamblador 6502» |
| **📂** / **💾** / **💾…** | Abrir (`.bas`, `.txt`, `.s`, `.asm`, `.a65`, `.inc`) / Guardar (<kbd>Ctrl</kbd>+<kbd>S</kbd>) / Guardar como… |
| **✂**, copiar, **📋** | Cortar, copiar, pegar |
| **🔍** | Buscar y reemplazar (<kbd>Ctrl</kbd>+<kbd>F</kbd>, <kbd>Ctrl</kbd>+<kbd>H</kbd>) |
| **⟲ ⟳** | Deshacer (<kbd>Ctrl</kbd>+<kbd>Z</kbd>), rehacer (<kbd>Ctrl</kbd>+<kbd>Mayús</kbd>+<kbd>Z</kbd>) |
| **A▲▼** | Tamaño del texto: un control compacto, la letra A con dos pequeños triángulos superpuestos; ▲ agranda, ▼ reduce |

Después, en la misma fila (o en la siguiente si el panel es estrecho), vienen las acciones de la pestaña: **▶ Ejecutar**, **📼 Exportar WAV** y **Memoria** para BASIC, o **📃 Listado**, **⏺ Punto de parada** y **⏩ Hasta el cursor** para una fuente 6502.

Cada archivo es una pestaña. Un punto • marca una pestaña modificada; × la cierra (si no está guardada, hace falta un segundo clic). Una pestaña marcada con **📚** se abrió desde la biblioteca: abrir un listado o una fuente de la biblioteca (▶, ⬇ o doble clic) abre su pestaña cuando el editor está visible, o la muestra si ya está abierta. Las pestañas, incluido el texto sin guardar, vuelven en el arranque siguiente. Un clic derecho en una pestaña ofrece **Guardar como…** y **Cerrar**. En el texto, un **clic central** pega la selección primaria (Linux): el cursor va donde hace clic y luego se inserta el último texto resaltado en cualquier aplicación.

El tamaño del texto se ajusta con el control **A▲▼**, <kbd>Ctrl</kbd>+rueda, o <kbd>Ctrl</kbd>+<kbd>+</kbd> / <kbd>Ctrl</kbd>+<kbd>−</kbd> / <kbd>Ctrl</kbd>+<kbd>0</kbd> cuando el texto tiene el foco del teclado.

**Separar** pone el editor en su propia ventana, por ejemplo en una segunda pantalla; **⊟ Acoplar** lo devuelve a su sitio. Las teclas escritas en esa ventana van al editor, nunca al Apple-1.

## Buscar, reemplazar y el menú del clic derecho

**🔍** (o <kbd>Ctrl</kbd>+<kbd>F</kbd>) abre una barra de búsqueda sobre el texto: <kbd>Intro</kbd> va a la coincidencia siguiente, <kbd>Mayús</kbd>+<kbd>Intro</kbd> a la anterior, **Aa** distingue mayúsculas, **Palabra** busca palabras completas, y el contador muestra «3 de 7». <kbd>Ctrl</kbd>+<kbd>H</kbd> añade el campo **Reemplazar por**, con **Reemplazar** (la coincidencia actual) y **Reemplazar todo**. <kbd>Esc</kbd> cierra la barra.

![La barra de búsqueda abierta sobre ECHO, y el menú del clic derecho de una fuente 6502.](img/es/editeur-menu.png "medium")

Un **clic derecho** en el texto abre un menú con todos los comandos y sus atajos. El menú actúa sobre la **selección**, que se conserva al hacer clic derecho sobre ella: **Cortar** y **Copiar** trabajan con el texto seleccionado. Los comandos que usan una palabra (ir a la definición, ayuda) toman la **palabra bajo el clic**, no la que está bajo el cursor.

| Comando | Atajo |
|---|---|
| Cortar, Copiar, Pegar, Seleccionar todo | <kbd>Ctrl</kbd>+<kbd>X</kbd>, <kbd>C</kbd>, <kbd>V</kbd>, <kbd>A</kbd> |
| Buscar, Reemplazar | <kbd>Ctrl</kbd>+<kbd>F</kbd>, <kbd>Ctrl</kbd>+<kbd>H</kbd> |
| Ir a la línea… | <kbd>Ctrl</kbd>+<kbd>G</kbd> |
| Comentar / descomentar (6502), REM / quitar REM (BASIC) | <kbd>Ctrl</kbd>+<kbd>/</kbd> |
| Sangrar, Quitar sangría | |
| Ir a la definición de una etiqueta (6502) o de un número de línea (BASIC) | <kbd>F12</kbd> |
| Ayuda sobre *palabra* (en una palabra clave, una instrucción o una directiva: abre su entrada en la referencia) | |
| Ver en el desensamblado (6502) | |
| Renumerar… (BASIC: `GOTO`, `GOSUB` y `THEN` siguen) | |
| Punto de parada en esta línea | <kbd>F9</kbd> |
| Ensamblar y ejecutar (6502), Enviar y ejecutar (BASIC) | <kbd>F5</kbd> |

**Colores.** Los colores del texto siguen el tema de la interfaz. **⚙ Preferencias → Editor** ofrece, por separado para BASIC y para el ensamblador 6502, los esquemas *Automático*, *Clásico (oscuro)*, *Claro*, *Ámbar*, *Alto contraste* o sus propios colores, una muestra por tipo de palabra (capítulo 11).

## Un programa BASIC

![El editor BASIC: errores sobre fondo rojo, con su número de línea y su columna al pie del panel.](img/es/editeur-erreur.png "large")

El texto está coloreado: números de línea, palabras clave, números, cadenas, `REM`, operadores. Cada línea se **tokeniza** como lo haría el Integer BASIC, con un tokenizador propio comparado byte a byte con el real. Una línea que el BASIC rechazaría se pone en rojo, y el error indica la línea del texto, el número de línea BASIC y la columna; un clic lleva allí el cursor. Errores detectados: sintaxis, nombre de variable no válido, número mayor que 32767, comando de modo directo (`LIST`, `RUN`…) dentro del programa, cadena sin cerrar, línea de más de 255 bytes, memoria llena.

- **Memoria**: automática (8 KB con LOMEM `$0800`, o `$0300` si el programa es grande; si no, 12, 16 o 32 KB), o forzada.
- {{▶ Ejecutar}}: carga el Integer BASIC si falta (como el botón BASIC), pone en memoria el programa tokenizado, añade RAM si hace falta y luego escribe `E2B3R` y `RUN`. Es instantáneo: no se escribe nada línea a línea.
- {{📼 Exportar WAV}}: un casete (`.wav`, o `.aiff` para un smartphone) de dos grabaciones, con sus ajustes en la biblioteca. Al pie del panel aparece lo que hay que escribir en el Apple-1 real, por ejemplo `C100R`, luego `004A.00FFR 0C3B.0FFFR` y después `E2B3R` y `RUN`; **📋 Copiar** lo pone en el portapapeles.

![Del editor al Apple-1 real: escribir, ejecutar, corregir, exportar.](diagrams/es/workflow-basic.svg "diagram large")

## Una fuente 6502

![Una fuente 6502 en el editor: los colores, el resultado del ensamblado y la línea que se escribe en el Apple-1 real.](img/es/brut/editeur-asm.png "large")

Una fuente se ensambla **a cada cambio** con el ensamblador integrado de dos pasadas, que acepta la sintaxis de ca65 (y la de xa, traducida automáticamente). Al pie del panel aparecen el tamaño y las direcciones (`33 bytes, $0300-$0320 — 2 archivo(s), 13 símbolos`) y luego la línea para la máquina real (`C100R`, luego `0300.0320R` y después `300R`).

- **Errores**: archivo, línea y columna; un clic lleva allí, abriendo el archivo incluido si hace falta. Los avisos (página cero codificada como absoluta) van en amarillo.
- **📃 Listado**: dirección, bytes y fuente de cada línea, con los archivos incluidos y las macros desarrolladas, y luego la tabla de símbolos; un clic devuelve a la línea.
- **▶ Ejecutar**: pone los bytes en memoria en el origen (`.org`, `$0300` por defecto), añade RAM si hace falta y luego escribe por ejemplo `300R`.
- **📼 Exportar WAV**: el casete, como para BASIC.
- **⏺ Punto de parada** y **⏩ Hasta el cursor**: el vínculo con el depurador (capítulo 8).
- **Vínculo con el depurador**: cuando selecciona una instrucción en la fuente (o pone el cursor en su línea), los bytes que produjo se resaltan en el **desensamblado** del depurador y en el **volcado hexadecimal / ASCII**, si esos paneles están visibles; si están en otra parte de la memoria, los paneles se desplazan para mostrarlos. La tabla de líneas de la fuente y direcciones del ensamblador lo hace posible.

![Un error de ensamblado: ECO en lugar de ECHO, un símbolo desconocido, señalado con su línea y su columna.](img/es/brut/editeur-asm-erreur.png "large")

La Parte III está dedicada por completo al ensamblador: sintaxis, directivas, listado, ejemplos.
