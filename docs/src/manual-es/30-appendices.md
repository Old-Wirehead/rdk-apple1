# Apéndices <span class="part-sub">Mapa de memoria, comandos del Woz Monitor, tiempos de la ACI, juego de caracteres, atajos de teclado, trampas, licencia y créditos</span> {: .part data-part="Apéndices" }

# Mapa de memoria {: .appendix }

![La memoria del Apple-1 en el emulador, de los vectores del monitor a la página cero.](diagrams/es/memoire.svg "diagram large")

| Direcciones | Contenido | Detalles |
|---|---|---|
| `$0000-$00FF` | página cero | Woz Monitor `$24-$2B`; Integer BASIC `$4A-$FF` (punteros LOMEM `$4A`, HIMEM `$4C`, PP `$CA`, PV `$CC`) |
| `$0100-$01FF` | pila del 6502 | |
| `$0200-$027F` | búfer de línea del monitor | 128 caracteres |
| `$0280-$0FFF` | RAM de base (4 KB) | programas en código máquina; el BASIC entre LOMEM (`$0800`) y HIMEM (`$1000`) |
| `$1000-$BFFF` | RAM de ampliación | 8 KB por defecto en el emulador (hasta `$1FFF`), hasta 48 KB con `--ram` |
| `$C000-$C0FF` | ACI: inversión de la salida, entrada de casete | cualquier acceso invierte la salida |
| `$C100-$C1FF` | ACI: PROM (el programa de la ACI) | se inicia con `C100R` |
| `$4000-$7FFF` | P-LAB CodeTank | EPROM de 16 KB, solo lectura, cuando hay una ROM insertada (por encima de la RAM) |
| `$C250-$C257` | tarjeta GEN2: conmutadores | lectura; páginas texto/LORES `$0400`, `$0800`, HIRES `$2000`, `$4000` |
| `$CC00-$CC01` | tarjeta P-LAB: TMS9918A | `$CC00` datos, `$CC01` control y estado; repetido en `$CC00-$CFFF` |
| `$D010-$D013` | PIA: `KBD`, `KBDCR`, `DSP`, `DSPCR` | capítulo 22 |
| `$E000-$EFFF` | BASIC | RDK G2 Basic (7933 bytes, necesita RAM hasta `$FEFF`), o el Integer BASIC de Apple (4 KB de RAM, cargado desde casete o desde una imagen) |
| `$FF00-$FFFF` | Woz Monitor | vectores: NMI `$0F00`, RESET `$FF00`, IRQ/BRK `$0000` |

# Comandos del Woz Monitor {: .appendix }

El monitor espera una línea terminada con <kbd>RETURN</kbd>. Las direcciones y los bytes se escriben en hexadecimal, sin `$`; una línea puede contener varios comandos separados por espacios.

| Comando | Efecto | Ejemplo |
|---|---|---|
| `dir` | muestra el byte de esa dirección | `FFEF` |
| `dir.dir` | muestra un rango, 8 bytes por línea | `FF00.FF1F` |
| `.dir` | continúa la visualización hasta esa dirección | `.FF3F` |
| `dir: byte byte …` | guarda bytes a partir de la dirección | `300: A9 C1 20 EF FF` |
| `: byte …` | continúa guardando | `: 4C 1F FF` |
| `dirR` | ejecuta el programa en esa dirección (`JMP`) | `300R` |
| <kbd>Esc</kbd> | anula la línea (muestra `\`) | |
| `_` (retroceso) | anula el último carácter | |

{{!dirR}}{{!dir.dir}}

Una línea de más de 127 caracteres se anula como con Esc. Tras `R`, el monitor muestra primero el byte de la dirección (`0300: A2`) y luego el programa toma el relevo.

| Dirección | Comando | Papel |
|---|---|---|
| `E000R` | arranque en frío del BASIC | borra el programa |
| `E2B3R` | arranque en caliente del BASIC | conserva el programa |
| `C100R` | el programa de la ACI | luego `inicio.finR` (leer) o `inicio.finW` (escribir) |
| `FF00R` | RESET por software del monitor | |

# Tiempos de la ACI {: .appendix }

Las duraciones las produce el programa de la ACI, contando ciclos entre dos inversiones de la salida. Valores medidos en el emulador, que ejecuta ese programa en su 6502 (reloj de 1,022727 MHz, con la parada de 4 ciclos cada 65 ciclos para el refresco de la memoria):

| Elemento | Ciclos | Duración | Frecuencia |
|---|---|---|---|
| semiperiodo de la cabecera | 592 | 618 µs | ≈ 809 Hz |
| primera mitad del bit de arranque | 180 | 188 µs | |
| semiperiodo de un «0» | 238 | 248 µs | ≈ 2,0 kHz |
| semiperiodo de un «1» | 473 | 494 µs | ≈ 1,0 kHz |
| cabecera completa | 16 390 semiperiodos | ≈ 10,1 s | |

Formato de una grabación:

<ol class="steps">
<li>la <strong>cabecera</strong>: unos 10 segundos de semiperiodos de 618 µs;</li>
<li>el <strong>bit de arranque</strong>: un semiperiodo corto (188 µs) y luego un semiperiodo de «0»;</li>
<li>los <strong>datos</strong>: cada byte, bit más significativo primero, un periodo completo por bit;</li>
<li>nada más: ni longitud, ni dirección, ni suma de control. El comando de lectura da el rango.</li>
</ol>

Al leer, el programa de la ACI mide los semiperiodos con un bucle de 12 ciclos. Toma como bit de arranque cualquier semiperiodo de menos de 31 vueltas (≈ 372 ciclos, 388 µs), y separa «0» de «1» hacia 0,74 ms por periodo completo. El decodificador de archivos del emulador acepta cabeceras de 0,40 a 0,90 ms por semiperiodo, lo que cubre la herramienta c2t (1 kHz) y las variaciones de velocidad de la cinta.

# El juego de caracteres del 2513 {: .appendix }

![Los 64 caracteres del Signetics 2513, tal como los dibuja el emulador: códigos $20 a $5F (con el bit 7, $A0 a $DF).](img/charset-2513.png "large")

El generador de caracteres solo conoce 64 símbolos: espacio, puntuación, cifras y mayúsculas. El terminal solo conserva 6 bits del código que recibe:

| Código enviado | Visualización |
|---|---|
| `$20-$5F` (o `$A0-$DF`) | el carácter de la tabla |
| `$60-$7F` (minúsculas, o `$E0-$FF`) | la mayúscula correspondiente (`$40-$5F`) |
| `$0D` (o `$8D`) | línea nueva |
| otros `$00-$1F` | nada: se ignoran |

Cada carácter es una matriz de 5 × 7 puntos en una celda de 7 × 8; la pantalla tiene 40 × 24 caracteres, es decir 280 × 192 puntos.

# Atajos de teclado {: .appendix }

| Tecla | Efecto |
|---|---|
| <kbd>Ctrl</kbd>+<kbd>R</kbd> | RESET del Apple-1 |
| <kbd>Ctrl</kbd>+<kbd>L</kbd> | CLEAR SCREEN |
| <kbd>Ctrl</kbd>+<kbd>V</kbd> | pegar: escribe el portapapeles en el teclado del Apple-1 |
| <kbd>F1</kbd> | modo ayuda (burbujas de ayuda detalladas) |
| <kbd>F3</kbd> | velocidad real o rápida |
| <kbd>F11</kbd> | pantalla completa |
| <kbd>Ctrl</kbd>+<kbd>+</kbd> / <kbd>Ctrl</kbd>+<kbd>−</kbd> / <kbd>Ctrl</kbd>+<kbd>0</kbd> | zoom de la pantalla (o del texto del editor, cuando el editor tiene el foco del teclado) |
| <kbd>Ctrl</kbd>+<kbd>S</kbd> | guardar el archivo del editor |
| <kbd>Ctrl</kbd>+<kbd>F</kbd> / <kbd>Ctrl</kbd>+<kbd>H</kbd> / <kbd>Ctrl</kbd>+<kbd>G</kbd> | editor: buscar, reemplazar, ir a la línea |
| <kbd>Ctrl</kbd>+<kbd>/</kbd> | editor: comentar / descomentar (6502), poner / quitar REM (BASIC) |
| <kbd>F5</kbd> / <kbd>F9</kbd> / <kbd>F12</kbd> | editor: ejecutar, punto de parada en la línea, ir a la definición (6502) |
| <kbd>Ctrl</kbd>+letra | carácter de control (`Ctrl+D` = `$84`) |
| clic derecho en una pantalla | Copiar, Pegar (escribir el texto), Borrar pantalla, Reset |
| <kbd>Esc</kbd> | ESC (`$9B`): anula la línea en el monitor y en el BASIC |
| retroceso | escribe `_`, el borrado del monitor y del BASIC |

En el **terminal** (`apple1-emu` sin ventana, capítulo 12):

| Tecla | Efecto |
|---|---|
| <kbd>F2</kbd> | línea de comandos del emulador |
| <kbd>F5</kbd> / <kbd>F6</kbd> | casete: reproducir, parar |
| <kbd>F10</kbd> o <kbd>Ctrl</kbd>+<kbd>Q</kbd> | salir |

# Trampas y comportamientos que conviene conocer {: .appendix }

Pequeñas cosas que sorprenden la primera vez. Ninguna es un fallo; cada una tiene su razón.

- **BASIC y RUN.** **▶ Ejecutar** del editor arranca el BASIC, carga el listado y escribe `RUN` por usted. Un programa que pide datos espera sus teclas en la pantalla del Apple-1: haga clic primero en la pantalla.
- **RND.** El primer `RND` de una ejecución sería siempre el mismo en una máquina recién reiniciada; **▶ Ejecutar** espera un momento aleatorio antes de `RUN`, para que un juego no empiece siempre igual.
- **Comandos de casete.** La línea «Casete, en el Apple-1 real:» bajo el editor es el comando del Woz Monitor que hay que escribir en una máquina real (`C100R`, luego los comandos `R` de los rangos) antes de reproducir el casete. Es una indicación, no algo que haya que escribir en el emulador.
- **El teclado va a un solo sitio a la vez.** La línea de abajo de la ventana dice adónde: al Apple-1, al editor, a un campo. Un clic en la pantalla del Apple-1 envía las teclas a la máquina; un clic en el editor las recupera. Tras hacer clic en un error de la lista del editor, las teclas vuelven al editor.
- **Depurador en pausa.** Las teclas pulsadas mientras la máquina está en pausa no se pierden, pero solo se conserva la **última** (el registro del teclado real guarda un carácter); se entrega cuando continúa.
- **Los puntos de parada van con el programa.** Cargar otro programa (archivo, biblioteca, ensamblador) borra los puntos de parada y las etiquetas del anterior. Los puntos de parada que haya puesto se conservan de una sesión a otra solo mientras no cargue otra cosa.
- **Pegar con el botón central.** En el editor, un clic central lleva el cursor allí y pega la **selección primaria** de X11/Wayland (el texto que acaba de resaltar en cualquier aplicación). El editor todavía no *escribe* su propia selección en la selección primaria: para copiar desde el editor, use <kbd>Ctrl</kbd>+<kbd>C</kbd>.
- **Ventanas pequeñas.** En un monitor de menos de 1400 puntos de ancho, la disposición del primer arranque es editor y pantalla; en el menú **⊞ Disposición ▾** puede elegir otra. En una pantalla de 1024 × 768, la barra de herramientas se desplaza de lado: el botón **»** alcanza el grupo Casete, en lugar de cortarlo por la derecha. Un zoom fijo (<kbd>Ctrl</kbd>+<kbd>+</kbd>) mayor que el panel de la pantalla se conserva, y la pantalla se desplaza dentro de su panel; la barra de estado se lo indica.
- **Guardar una pestaña sin nombre.** **Guardar como…** (icono, <kbd>Ctrl</kbd>+<kbd>S</kbd> o el menú contextual de la pestaña) nunca deja dos pestañas para el mismo archivo: una pestaña ya abierta con ese archivo se cierra.
- **Primer arranque y la red.** El primer arranque contacta con GitHub para descargar el proyecto de demostración (capítulo 4); quite el proyecto de la lista «Proyectos GitHub» en **⚙ Preferencias** para que deje de hacerlo.
- **Leer el teclado en G2 Basic.** Una tecla pulsada mientras un programa se ejecuta lo detiene, como en el original, e `INPUT` espera. Un juego que necesite vigilar las teclas llama a `KEY(0)` en su bucle principal (capítulo 15); a partir de entonces, una tecla ya no detiene el programa.
- **Etiquetas y puntos de parada.** Un punto de parada en una etiqueta que aún no existe (escrito antes de ensamblar o ejecutar) se conserva y se coloca cuando la etiqueta aparece.
- **Disposiciones y ventanas.** Una disposición guardada recuerda los paneles acoplados, las ventanas separadas (posición y tamaño) y el tamaño de la ventana principal. No recuerda qué ventana tiene el teclado.

# Licencia y créditos {: .appendix }

**Retro Dev Kit Apple 1** es software libre, publicado bajo la **GNU General Public License, versión 3** (GPL-3.0), sin ninguna garantía. El texto completo está en el archivo `LICENSE` del paquete y del repositorio.

Algunas partes vienen de otros proyectos y conservan su propia licencia, la licencia **MIT**, compatible con la GPL; se enumeran en el archivo `NOTICE`:

- **RDK G2 Basic**, el BASIC integrado, escrito para este proyecto (repositorio `Old-Wirehead/rdk-gen2basic-apple1`, MIT);
- las **pruebas GEN2 de Uncle Bernie** (`gen2-test1` a `gen2-test4`, las barras) y los **glifos GEN2**, proporcionados por Uncle Bernie.

Los juegos de la biblioteca escritos por **Claude IA** (Ahorcado, Cerillas, Batalla naval, Túnel y Tres en raya) se distribuyen con el proyecto.

Lo que el kit **no** contiene: el Integer BASIC de Apple, los casetes comerciales y las demos de la tarjeta P-LAB. Siguen perteneciendo a sus propietarios; usted aporta sus propias copias.

La tarjeta P-LAB es una creación de P-LAB; el TMS9918A es un chip de Texas Instruments. Este proyecto no está afiliado a Apple, Texas Instruments ni P-LAB.
