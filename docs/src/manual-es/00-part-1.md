---
version: 1.0
date: noviembre de 2026
---

# El emulador <span class="part-sub">La ventana, la pantalla, la biblioteca, la interfaz de casete ACI, la tarjeta de color GEN2, el depurador y la línea de comandos</span> {: .part data-part="Parte I" }

# Acerca de este manual

<p class="lead"><strong>Retro Dev Kit Apple 1</strong> — «El pecado original» — reproduce ciclo a ciclo el Apple-1 de 1976: el 6502, el terminal de vídeo, el teclado, la interfaz de casete ACI y, como opciones, la tarjeta de color GEN2 de Uncle Bernie y la tarjeta gráfica P-LAB con TMS9918A. Trae su propio BASIC libre, <strong>RDK G2 Basic</strong>. Alrededor de la máquina añade un taller completo: una biblioteca de programas, un editor, un ensamblador, un depurador y la exportación a un Apple-1 real.</p>

Este manual describe todo el emulador, versión 1.0 (la primera versión pública; los números 7.x eran las versiones de prueba). Completa la *Guía de inicio rápido*, que basta para instalar el programa y reproducir un primer casete. Aquí se explican cada panel, cada botón y cada comando, con ejemplos que puede escribir tal cual.

## Cómo está organizado este manual

<div class="cards">
<div class="card"><h4>Parte I — El emulador</h4>La ventana y sus paneles, la pantalla vintage, la biblioteca, el casete y la ACI, las tarjetas gráficas GEN2 y P-LAB, el editor, el depurador, el arte ASCII, las preferencias, la línea de comandos y la resolución de problemas.</div>
<div class="card"><h4>Parte II — Integer BASIC</h4>El BASIC de Steve Wozniak y su clon libre RDK G2 Basic: cada instrucción, función y operador, la memoria, los mensajes de error y los fallos conocidos, con sus soluciones.</div>
<div class="card"><h4>Parte III — El ensamblador 6502</h4>El 6502 desde cero, el ensamblador integrado y sus directivas, las rutinas del Woz Monitor, ejemplos completos y las trampas que conviene evitar.</div>
<div class="card"><h4>Anexos</h4>Mapa de memoria, comandos del Woz Monitor, el formato ACI, el juego de caracteres del 2513, los atajos de teclado y el índice de comandos.</div>
</div>

## Convenciones

Las teclas se escriben así: <kbd>Ctrl</kbd>+<kbd>R</kbd>, <kbd>RETURN</kbd>. Los botones de la interfaz van en negrita: **▶ Ejecutar**. Las direcciones están en hexadecimal, precedidas de `$` como en los documentos de la época: `$FFEF`. El propio Woz Monitor las escribe sin `$`: `FFEF`.

Las sesiones en la pantalla del Apple-1 tienen fondo oscuro. Lo que escribe **usted** va en blanco, la respuesta de la máquina en verde:

```mon
» 300R

0300: A2
HOLA, APPLE-1!
```

A lo largo del camino aparecen cuatro tipos de recuadros:

!!! astuce "Consejo"
    Un atajo, o una forma mejor de hacer algo.

!!! attention "Atención"
    Una trampa que cuesta tiempo o datos.

!!! apple1 "En la máquina real"
    Lo que ocurre en un Apple-1 real y lo que difiere del emulador.

!!! bug "Fallo conocido"
    Un defecto de la ROM original, con su solución. Cada fallo tiene un estado: <span class="verified">verificado</span> (reproducido en el emulador con el Integer BASIC real) o <span class="unverified">no verificado</span> (citado de una fuente, no probado).

Todos los ejemplos de este manual se han probado en el emulador. Los programas 6502 de la Parte III están en la carpeta `docs/exemples/es/asm/` de la distribución. Producen exactamente los mismos bytes con el ensamblador integrado y con ca65.

## Lo que necesita

El emulador funciona en Linux sin instalar nada. El Woz Monitor y la ROM de la ACI vienen integrados, igual que **RDK G2 Basic**, un BASIC libre compatible con Integer BASIC (capítulo 14): el botón **BASIC** funciona de inmediato, en una instalación nueva, sin buscar ningún archivo. El **Integer BASIC** de Apple de 1976 **no** se incluye, porque sigue protegido por derechos de autor. Si quiere el original, use su propia copia (por ejemplo el casete `BASIC.wav` de la colección «Apple1Cassettes» de archive.org) y elíjala en **⚙ Preferencias → Avanzado → BASIC**.

!!! attention "Las ROM y los casetes son cosa suya"
    El emulador no incluye ni el Integer BASIC de Apple ni casetes comerciales. Su tabla de reconocimiento solo contiene tamaños, huellas y parámetros. Guarde sus copias en su biblioteca personal.

!!! note "Licencia"
    Retro Dev Kit Apple 1 es software libre bajo la **GNU GPL 3.0** (sin garantía). Algunas partes conservan su propia licencia MIT: RDK G2 Basic y las pruebas y glifos GEN2 de Uncle Bernie. Vea `LICENSE` y `NOTICE` en el archivo, y el anexo «Licencia y créditos».

## Los recorridos principales

Cuatro formas de usar el emulador aparecen una y otra vez. Cada una se describe más adelante con su propio esquema:

| Quiero… | El recorrido | Ver |
|---|---|---|
| jugar a un programa de la época | biblioteca → **▶** | capítulo 4 |
| escribir un programa BASIC | **📝 Editor** → **▶ Ejecutar** → **📼 Exportar WAV** | Parte II |
| escribir un programa 6502 | **📝 Editor** (`.s`) → **📃 Listado** → **🐛 Depurador** | Parte III |
| enviar un programa a un Apple-1 real | **📼 Exportar WAV** (`.aiff`) → smartphone → ACI | capítulo 5 |

# La ventana y la barra de herramientas

## Vista general

![La disposición predeterminada: todos los paneles abiertos y acoplados.](img/es/defaut-annote.png "large")

<ol class="legend">
<li><strong>Cargar, Guardar, Estado</strong>: volcados hexadecimales <code>.txt</code>, binarios <code>.bin</code> y <code>.prg</code>; Guardar también escribe casetes <code>.wav</code>/<code>.aiff</code>; Estado guarda y restaura la máquina.</li>
<li><strong>RESET, Borrar pantalla, Velocidad (indicador), Pegar</strong>: los botones de la máquina (Ctrl+R, Ctrl+L, F3, Ctrl+V).</li>
<li><strong>BASIC</strong>: carga el RDK G2 Basic integrado en <code>$E000</code> (o su Integer BASIC, si lo ha elegido).</li>
<li><strong>Ventanas</strong>: el menú <strong>⊞ Disposición ▾</strong> y luego un botón por panel (Pantalla monocromo, Pantalla en color, Editor, Depurador, Desensamblado, Volcado, Biblioteca, ASCII).</li>
<li><strong>Tarjetas gráficas</strong>: <strong>GEN2</strong> y <strong>P-LAB</strong> conectan o desconectan las tarjetas.</li>
<li><strong>Casete</strong>: <strong>📼 Insertar</strong> un casete, y luego reproducir, parar, rebobinar, grabar, expulsar; osciloscopio y sonido.</li>
<li><strong>EPROM</strong>, <strong>⚙ Preferencias</strong>, <strong>?</strong> (burbujas de ayuda detalladas, también F1) e <strong>ℹ</strong> (acerca de, atajos).</li>
<li><strong>Fila de visualización de la pantalla</strong>: zoom (1x a 8x o Ajustar), fósforo, velocidad de visualización <strong>Normal</strong> o <strong>Rápido</strong>, pantalla vintage, pantalla completa, <strong>Separar</strong>.</li>
<li><strong>Editor</strong>: programas BASIC y fuentes 6502 en pestañas.</li>
<li><strong>Pantalla del Apple-1</strong>: 40 × 24 caracteres; haga clic en ella para escribir, clic derecho para Copiar, Pegar, Borrar pantalla y Reset.</li>
<li><strong>Depurador</strong>: pausa, paso a paso, registros, puntos de interrupción, variables.</li>
<li><strong>Desensamblado</strong>: el código alrededor del contador de programa (PC).</li>
<li><strong>Volcado</strong>: la memoria en hexadecimal y ASCII, modificable.</li>
<li><strong>Biblioteca</strong>: sus programas, cada uno con sus pequeños iconos ▶ ⬇ 📼 ⚙.</li>
<li><strong>Barra de estado</strong>: velocidad, ritmo de visualización, casete, destino del teclado; debajo, el último mensaje.</li>
</ol>

Cada panel (pantalla, editor, depurador, biblioteca…) es una pestaña que puede mover, agrupar o separar en su propia ventana: es el tema del capítulo 10. En el primer arranque la ventana se abre maximizada y una ventana **Su espacio de trabajo** propone cinco disposiciones (capítulo 10); el tamaño y la posición de la ventana y la disposición de sus paneles se recuerdan y vuelven a aparecer tal cual en el arranque siguiente.

## La barra de herramientas, botón a botón

La barra de herramientas es una sola fila: la máquina y sus herramientas a la izquierda, el magnetófono en el centro, los paneles en el grupo Ventanas, las tarjetas y, a la derecha, los ajustes. Cuando la ventana es demasiado estrecha, los botones conservan solo su icono (el texto queda en la burbuja de ayuda), o la barra pasa a varias líneas si desmarca **Barra compacta** en las preferencias.

![La barra de herramientas hasta el grupo Tarjetas gráficas: archivos, máquina, ventanas y tarjetas. Luego viene la franja Casete (capítulo 5) y después los ajustes.](img/es/barre-1.png "large")

| Botón | Lo que hace |
|---|---|
| {{Cargar}} | Abre un programa y lo pone en memoria: un `.txt` (volcado del Woz Monitor) o un `.hex`; para un `.bin`, se pide la dirección. Los casetes (`.wav`, `.aiff`) no pasan por aquí: se insertan con **📼 Insertar**, en la franja Casete. |
| {{Guardar}} | Pide uno o varios rangos (`0300.03FF`, `004A.00FF 0300.0FFF`…) y luego un archivo. La extensión elige el formato: `.wav`, `.aiff`, `.txt` o `.bin`. |
| **RESET** (botón rojo) | El botón RESET del Apple-1 (<kbd>Ctrl</kbd>+<kbd>R</kbd>). El 6502 y el PIA se reinician; la memoria y la pantalla se conservan. |
| **🆑 Borrar pantalla** | El botón CLEAR SCREEN (<kbd>Ctrl</kbd>+<kbd>L</kbd>). |
| **Indicador (Velocidad real / Velocidad rápida)** | Alterna entre el 1 MHz original y la velocidad máxima (<kbd>F3</kbd>). Un velocímetro: iluminado en azul mientras la velocidad rápida está activa. |
| **📋 Pegar** | Escribe el portapapeles en el teclado del Apple-1, al ritmo del teclado (<kbd>Ctrl</kbd>+<kbd>V</kbd>). Ideal para un volcado hexadecimal. |
| {{BASIC|**BASIC**}} | Carga el **RDK G2 Basic** integrado en `$E000` (7933 bytes; luego `E000R` lo inicia). Si eligió el Integer BASIC original en **⚙ Preferencias → Avanzado → BASIC**, carga ese en su lugar: un casete, un volcado `.txt` o un `.bin` de 4096 bytes, buscado en la biblioteca (`BASIC.wav`, `integer_basic.bin`, cualquier nombre que contenga «basic»), y si no, se le pide. Clic derecho: elegir otro archivo. |
| **Ventanas** (grupo: **⊞ Disposición ▾** y un botón por panel) | Muestra u oculta cada panel: **Pantalla monocromo** (el terminal del Apple-1), **Pantalla en color** (la pantalla de la tarjeta gráfica conectada, GEN2 o P-LAB: el mismo botón sirve para ambas; sin tarjeta conectada, propone conectar una, y **P-LAB** muestra entonces su pantalla), **📝 Editor** (capítulo 7), **🐛 Depurador**, **Desensamblado**, **Volcado** (capítulo 8), **📚 Biblioteca** y **🎨 ASCII** (capítulo 9). Un botón iluminado indica que el panel está visible; un panel que ya se muestra también se cierra con la cruz de su pestaña. |
| **Tarjetas gráficas: GEN2 y P-LAB** | Un grupo con nombre. **GEN2** conecta o desconecta la tarjeta de color de Uncle Bernie (capítulo 6); **P-LAB** hace lo mismo con la tarjeta P-LAB de TMS9918A (sección 6.6). Un botón está iluminado mientras su tarjeta está conectada; la pantalla de una tarjeta aparece en su propio panel, que muestra el botón **Pantalla en color**. |
| **📸 Estado ▾** | Un solo botón con un menú: **Guardar estado…** (<kbd>Ctrl</kbd>+<kbd>Mayús</kbd>+<kbd>S</kbd>), **Cargar un estado…** (<kbd>Ctrl</kbd>+<kbd>Mayús</kbd>+<kbd>O</kbd>) y luego los últimos estados, cada uno con su título y su fecha (sección 2.5). |
| **EPROM** (chip con ventana UV, a la derecha de la barra, antes de **⊞ Disposición**) | Abre el programador de EPROM: leer, escribir o verificar un chip con un programador `minipro` (sección 2.6). |
| **Casete: 📼 Insertar ▶ ⏹ ⏪ ⏺ ⏏** | El magnetófono, reunido en una franja. **📼 Insertar** elige un `.wav` o `.aiff` y lo pone en el magnetófono como casete virtual; luego reproducir, parar, rebobinar, grabar, expulsar. La franja muestra el nombre, la posición y la duración. |
| **📈** y **🔊** | El osciloscopio de casete y el sonido de la señal (capítulo 5). |
| **⊞ Disposición ▾** (primer botón del grupo Ventanas) | Un menú: **Predeterminada** (todos los paneles, como en el primer arranque), **Juego** (pantalla del Apple-1 y biblioteca), **BASIC** (editor y pantalla), **Ensamblador** (editor, pantalla y depurador), **Depuración** (editor, depurador, desensamblado y volcado); luego **Guardar la disposición actual…**, que pide un nombre, y sus disposiciones guardadas por orden alfabético (clic derecho: Renombrar, Eliminar). Una marca señala la última elección; todo se guarda en `gui.json`. |
| **⚙** | Las preferencias (capítulo 11). |
| **?** | Modo ayuda (<kbd>F1</kbd>): burbujas detalladas en cada control, en los 7 idiomas de la interfaz. |
| **ℹ** | «Acerca de», con la lista de atajos de teclado. |

**Ventanas de archivos.** Abrir, Guardar como y Exportar usan la ventana de archivos propia del emulador. Un botón azul **📁+ Nueva carpeta**, arriba a la izquierda, crea una carpeta en la carpeta mostrada, con una burbuja que lo explica; escriba el nombre, pulse <kbd>Intro</kbd> y la carpeta nueva está lista para recibir su archivo.

![La ventana Cargar, con el botón 📁+ Nueva carpeta arriba a la izquierda.](img/es/fenetre-fichiers.png "medium")

Los ajustes de visualización no están en la barra de herramientas: cada pantalla tiene su propia **fila de visualización**, en lo alto de su panel (sección 3.2).

![La fila de visualización de la pantalla del Apple-1: zoom, fósforo, velocidad de visualización, pantalla vintage y pantalla completa; Separar está en el extremo derecho de la fila.](img/es/barre-2.png "large")

!!! astuce "Modo ayuda"
    Cuando explore un panel, active **?**: cada botón explica lo que hace al pasar el ratón por encima. Desactivado, solo quedan las burbujas cortas habituales (se pueden desactivar en **⚙ → Interfaz**). El ajuste se recuerda.

## La barra de estado

![La barra de estado mientras se reproduce BASIC.wav: velocidad, límite de visualización, casete, teclado y, debajo, el último mensaje.](img/es/barre-etat.png "large")

- **Velocidad medida**: unos 0,960 MHz a velocidad real. Es el reloj efectivo del Apple-1, que detiene el 6502 cuatro ciclos de cada sesenta y cinco para refrescar su memoria dinámica.
- **Pantalla ~60 car./s**: el terminal del Apple-1 solo muestra un carácter por imagen, unos 60 por segundo. Un clic alterna entre este límite y la visualización instantánea, como los botones **Normal** / **Rápido** de las filas de visualización.
- **Casete**: nombre, posición y duración, o «sin casete».
- **Teclado**: adónde van las teclas (al Apple-1 o a un campo de texto de la interfaz).

## El teclado del Apple-1

Haga clic en la pantalla: las teclas van al Apple-1. El teclado original solo tiene mayúsculas, y el emulador convierte las minúsculas.

| Tecla del PC | Para el Apple-1 |
|---|---|
| letras, cifras, símbolos | el carácter, en mayúscula |
| <kbd>Intro</kbd> | RETURN (código `$8D`) |
| <kbd>Retroceso</kbd> | `_`: el Woz Monitor y el BASIC borran el carácter anterior (queda en pantalla, seguido de `_`) |
| <kbd>Esc</kbd> | ESC: anula la línea en curso en el Woz Monitor |
| <kbd>Ctrl</kbd>+letra | el carácter de control (`Ctrl+D` = `$84`…) |
| <kbd>Ctrl</kbd>+<kbd>R</kbd> / <kbd>Ctrl</kbd>+<kbd>L</kbd> | los botones RESET y CLEAR SCREEN |

Puede **seleccionar texto con el ratón** en ambas pantallas: pulse y arrastre sobre los caracteres, y la selección se resalta. Un **clic derecho** en cualquiera de las pantallas (la del Apple-1 o el monitor GEN2) abre un pequeño menú: **Copiar** pone el texto seleccionado en el portapapeles (o todo el texto de la pantalla si no hay selección), **Pegar (teclear el texto)** escribe el portapapeles en el teclado del Apple-1, como si lo tecleara usted, y luego **Borrar pantalla** y **Reset**.

![El menú del clic derecho de la pantalla del Apple-1, durante una partida de Hamurabi.](img/es/ecran-menu.png "medium")

!!! apple1 "La pantalla del Apple-1 no borra nada"
    El terminal solo sabe escribir un carácter y pasar a la línea siguiente. No hay cursor móvil ni forma de borrar un carácter: la tecla de borrado escribe `_` y el carácter se queda en pantalla. Es normal.

## Estado de la máquina

El botón **📸 Estado** de la barra de herramientas toma una instantánea completa de la máquina y la restaura más tarde, exactamente en el mismo punto: puede detener un juego o un programa en marcha, cerrar el emulador y continuar al día siguiente. El botón abre un menú:

![El menú Estado: guardar, cargar y luego los últimos estados con su fecha.](img/es/menu-etat.png "medium")

- **Guardar estado…** (<kbd>Ctrl</kbd>+<kbd>Mayús</kbd>+<kbd>S</kbd>) pide un nombre de archivo y escribe un archivo `.a1state`. La carpeta propuesta es `etats`, junto a la carpeta de la biblioteca.
- **Cargar un estado…** (<kbd>Ctrl</kbd>+<kbd>Mayús</kbd>+<kbd>O</kbd>) sustituye la máquina por el estado elegido. La barra de estado muestra el archivo y el valor de PC.
- **Últimos estados** muestra los ocho últimos, con su título y la fecha en que se guardaron; un clic carga uno al instante. Si el archivo se ha movido o eliminado, un mensaje lo indica.

Un estado contiene la memoria, los registros del 6502, el PIA, la pantalla, la tarjeta GEN2 si está conectada y el casete virtual con su posición. No contiene sus preferencias ni la biblioteca. Un archivo que no se puede leer (dañado, o que no es un estado) se rechaza con un mensaje, y la máquina no se toca.

!!! astuce "Uso típico"
    Guarde un estado justo antes de un pasaje difícil de un juego, o al final de una larga sesión de BASIC: un estado es también una forma cómoda de compartir un programa en marcha con un amigo que use la misma versión.

## El programador de EPROM

**EPROM** en la barra de herramientas abre una ventana para leer, escribir y verificar un chip EPROM o EEPROM desde el emulador, por ejemplo para grabar la ROM del BASIC, un programa para la ampliación GEN2 o el contenido de su propia memoria.

![El programador de EPROM: programador, chip, origen de los datos y las tres operaciones.](img/es/programmateur-eprom.png "medium")

Solo se admiten los programadores que maneja la herramienta libre **`minipro`**: los XGecu **TL866II+**, **T48** y **T56**. En Debian, Ubuntu y Linux Mint se instala con `sudo apt install minipro`; conecte el programador a un puerto USB y pulse **Detectar**. Si falta `minipro`, la ventana lo dice y explica qué hacer; en el lado del emulador no hace falta nada más.

| Ajuste | Función |
|---|---|
| **Programador** | el programador detectado, con su firmware (por ejemplo «Found TL866II+ 04.2.132 (USB)») |
| **Chip** | los chips habituales del mundo Apple-1: 2716, 2732, 2764, 27C64, 27C128, 27C256, 27C512, 28C64, 28C256, SST27SF256, SST27SF512 y la ROM de máscara 23128 (solo lectura) |
| **Datos** | la **Memoria del Apple-1** (un rango de direcciones, `$E000`–`$EFFF` por defecto) o un **archivo** (`.bin`, `.hex`, `.aiff`…) |

Tres botones actúan sobre el chip, y una barra de progreso y un registro muestran lo que ocurre:

- **Leer el chip** lee el chip entero. Si los datos son la memoria o un archivo existente, el contenido se **compara** y se listan las diferencias: un archivo existente nunca se sobrescribe. Con un nombre de archivo nuevo, el contenido se guarda en él.
- **Escribir el chip** programa el chip con los datos elegidos. El chip debe estar virgen o ser borrable; al final se confirma el resultado.
- **Verificar** compara el chip con los datos elegidos, byte a byte, e indica cada diferencia (`$E010: chip $4C, datos $A9`).

**Cerrar** cierra la ventana.

La ventana nunca modifica la memoria emulada: escribir y verificar la leen, y leer solo la compara o llena un archivo.

# La pantalla: zoom, fósforo y efecto vintage

## La pantalla original

El Apple-1 muestra 40 columnas × 24 líneas de caracteres en mayúsculas. Cada carácter es una matriz de 5 × 7 puntos en una celda de 7 × 8, leída de la ROM del generador de caracteres **Signetics 2513**: el emulador usa el contenido de esa ROM, punto a punto (Anexo D). La imagen tiene por tanto 280 × 192 puntos.

El cursor es una `@` parpadeante, gobernada por un NE555: 2,18 Hz, visible unos dos tercios del tiempo. En algunas placas, la tolerancia del condensador da 1,7 Hz; la opción `--cursor-hz 1.7` lo reproduce.

## La fila de visualización

Cada pantalla tiene su propia fila de ajustes en lo alto de su panel: la pantalla del Apple-1 y el monitor en color cuando la tarjeta GEN2 está conectada (capítulo 6).

| Ajuste | Función |
|---|---|
| **Zoom** | una lista: `1x` a `8x` amplían la imagen por un factor entero, así los puntos siguen cuadrados y nítidos; **Ajustar** elige el mayor factor que cabe en el panel (el factor elegido aparece entre paréntesis). Con el teclado: <kbd>Ctrl</kbd>+<kbd>+</kbd>, <kbd>Ctrl</kbd>+<kbd>−</kbd>, <kbd>Ctrl</kbd>+<kbd>0</kbd> (ajustar). |
| **Fósforo** | solo pantalla del Apple-1: el verde de los monitores de la época, el blanco de un televisor o el ámbar. |
| **Visualización: Normal / Rápido** | **Normal** muestra los caracteres al ritmo del terminal real, unos 60 por segundo; **Rápido** lo muestra todo de golpe. Este ajuste es compartido: cambiarlo en una pantalla lo cambia en la otra. |
| **Pantalla vintage** | el efecto de tubo de los años 70 (sección siguiente); cada pantalla tiene su propia casilla. |
| **⛶** | pantalla completa (<kbd>F11</kbd>). |
| **Separar** | pone la pantalla en su propia ventana (capítulo 10). |

Cada pantalla está **centrada** en su panel, en horizontal y en vertical, con su leyenda centrada bajo la imagen. Si el zoom elegido es mayor que el panel, la pantalla se muestra con el mayor zoom que cabe en lugar de cortarse. Cuando la ventana es más estrecha que la fila de visualización, la fila pasa a una segunda línea, y **Separar** sigue en el extremo derecho de la última línea.

![Fósforo ámbar, con la pantalla vintage y una ligera curvatura.](img/es/brut/ambre.png "medium")

## La pantalla vintage

La casilla **Pantalla vintage** de la fila de visualización imita un monitor de tubo de los años 70. El efecto principal es la fina línea oscura entre dos líneas de barrido: cada fila de puntos se convierte en una banda horizontal, como en la foto de un Apple-1 real. Esta línea conserva el mismo grosor en píxeles de su pantalla a cualquier zoom: sigue fina y nítida, incluso en pantalla completa.

![La pantalla vintage, con una ligera curvatura del tubo (0,15) y esquinas oscuras, durante una partida de Hamurabi.](img/es/brut/vintage.png "medium")

En **⚙ Preferencias → Avanzado → Pantalla vintage: ajustes finos**, usted fija la **Intensidad** y elige los efectos; estos ajustes son comunes a las dos pantallas. Cada deslizador muestra su **valor numérico** al lado (curvatura, intensidad…):

| Efecto | Por defecto | Función |
|---|---|---|
| separación de las líneas de barrido | activado | la línea oscura entre dos filas de puntos |
| puntos suavizados en horizontal | activado | un ligero desenfoque, solo horizontal, como el haz |
| remanencia del fósforo | corta | los puntos se apagan en menos de una décima de segundo (algo más despacio en ámbar); a la izquierda del todo: ninguna |
| halo | activado, discreto | un resplandor alrededor de los puntos, que siguen nítidos |
| tubo abombado y esquinas oscuras | desactivado | con un deslizador **Curvatura** |
| parpadeo | desactivado | una variación muy leve del brillo |
| máscara de fósforo RGB | activada | solo la pantalla en color de la tarjeta GEN2: finas franjas verticales rojas, verdes y azules separadas por líneas oscuras, como en el tubo de un televisor en color. Es visible a partir del zoom 3, y su fuerza sigue a la **Intensidad** |

El efecto también puede aplicarse al monitor en color de la tarjeta GEN2, con la casilla de su propia fila de visualización. Lo calcula el procesador, a la resolución del Apple-1 y solo cuando la imagen cambia: sigue siendo fluido incluso sin aceleración gráfica.

En la línea de comandos: `--screening`, `--sans-screening`, `--gen2-screening` (monitor GEN2), o por ejemplo `--screening-options intensite=0.7,remanence=0.5,courbure=0.4,sans-halo` (valores de 0 a 1; los nombres de las opciones están en francés).

!!! astuce "Pantalla separada y segundo monitor"
    **Separar**, al final de la fila de visualización, pone la pantalla del Apple-1 en su propia ventana. Colóquela en un segundo monitor, a pantalla completa, con el efecto vintage: el teclado va al Apple-1 en cuanto esa ventana tiene el foco. Con **Ajustar**, la imagen sigue el tamaño de la ventana, por pasos enteros (**Escala entera**, en la barra de título de la ventana, la más nítida) o rellenándola.
