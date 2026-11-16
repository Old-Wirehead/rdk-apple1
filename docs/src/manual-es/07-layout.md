# Disposición de los paneles

<p class="lead">Cada panel es una pestaña que se coloca donde se quiera: uno al lado del otro, en pestañas, o en su propia ventana en una segunda pantalla.</p>

## Mover un panel

El editor, la pantalla del Apple-1, el monitor en color GEN2, el depurador, el desensamblado, el volcado, la herramienta ASCII y la biblioteca son todos **pestañas acoplables**. Arrastre una pestaña por su título: aparecen guías, y la suelta **a la izquierda de**, **a la derecha de**, **encima** o **debajo** de otro panel, o **en el mismo grupo de pestañas**. Arrastre los separadores para cambiar el tamaño. La disposición se guarda en `gui.json`.

## La disposición inicial

El editor está a la izquierda; en el centro, la pantalla del Apple-1, con el monitor en color debajo cuando la tarjeta GEN2 está conectada; a la derecha, el depurador con el desensamblado debajo, luego el volcado y la biblioteca en el extremo derecho. En el primerísimo arranque, la elección depende de la anchura del **monitor** (la ventana se abre pequeña y se maximiza un instante después): por debajo de 1400 puntos (un portátil de 1366 × 768, una pantalla de 1024 × 768) empieza con **editor y pantalla**, la pareja más útil; a partir de 1400 puntos, con todos los paneles. Elija otra disposición en el menú cuando quiera. Sigue la anchura de la ventana: cada panel conserva su parte, de un portátil a un monitor ancho. El menú **⊞ Disposición ▾** de la barra de herramientas la restablece en cualquier momento (**Predeterminada**, todos los paneles abiertos) y ofrece cuatro disposiciones más ya preparadas: **Juego** (pantalla del Apple-1 y biblioteca), **BASIC** (editor y pantalla), **Ensamblador** (editor, pantalla y depurador) y **Depuración** (editor, depurador, desensamblado y volcado). **Guardar la disposición actual…** pide un nombre y conserva la disposición actual, con sus ventanas separadas y el tamaño de la ventana principal (al cargarla, esas ventanas vuelven a abrirse donde estaban); sus disposiciones aparecen bajo un separador, por orden alfabético, y un clic derecho permite **Renombrar** o **Eliminar**. Se guardan en `gui.json`.

## El asistente de inicio

En el primer arranque, un asistente de tres pantallas sustituye a la antigua ventana «Su espacio de trabajo». Está dibujado en ámbar sobre negro, con el efecto de tubo (líneas de barrido, halo, viñeteado, curvatura 0,07) en el marco; el texto pequeño y las miniaturas siguen nítidos. Una pequeña **×** arriba a la derecha lo cierra. **‹ Anterior** y **Siguiente ›** pasan de una pantalla a otra; la última tiene **Terminar**.

1. **Bienvenida**: para qué sirve el kit.
2. **ROM y BASIC**: el Woz Monitor, la ROM del casete y G2 Basic vienen integrados. No hace falta nada más. Los puristas instalan su propio Integer BASIC original desde una fuente externa; el botón abre la guía («Instalar una ROM»), que solo tiene enlaces, nunca los archivos.
3. **Su espacio de trabajo**: las cinco disposiciones (**Predeterminada**, **Juego**, **BASIC**, **Ensamblador**, **Depuración**). Cada miniatura se puede pulsar (no hay botón «Elegir»). Los rectángulos conservan los números 1 a 6, y la pantalla en color (GEN2 / P-LAB) es el **7**, en su propio color, en las miniaturas que la muestran y en la leyenda.

**Mostrar esta ventana al iniciar** está en las tres pantallas, marcada por defecto. La misma casilla está en **⚙ Preferencias → Interfaz**. **⊞ Disposición ▾ → Elegir un espacio de trabajo…** lo vuelve a abrir. Es una ventana normal: no se queda encima de las demás. No se abre cuando la línea de comandos fija un tamaño (`--taille`), ni para las capturas.

## Tamaño del texto de cada ventana

El editor, el listado del ensamblador, el depurador, el desensamblado, el volcado y la biblioteca tienen cada uno un control compacto **A 13 ▲▼** en su barra: los triángulos cambian el tamaño, que se recuerda para esa ventana. Ctrl+rueda y Ctrl+más / Ctrl+menos actúan sobre la ventana que está bajo el puntero del ratón (sobre la pantalla del Apple-1, Ctrl+más / Ctrl+menos cambian el zoom). **Restablecer las preferencias** devuelve todos estos tamaños a 13.

## Lo que se recuerda entre dos arranques

La disposición se guarda sola al salir y se recupera tal como estaba: la posición y el tamaño de cada panel, las ventanas separadas, el zoom y la disposición elegida. La ventana se restaura con su tamaño y en su sitio, y luego los paneles se vuelven a colocar en las mismas proporciones, sea cual sea la pantalla: la disposición se comprobó a 2880 × 1620, 2560 × 1440, 1920 × 1080 y 1366 × 768, con menos del 1 % de la imagen distinta tras un nuevo arranque. Una disposición guardada con nombre (**Guardar la disposición actual…**) conserva además las ventanas separadas. La interfaz nunca hace zoom por sí sola: solo el ajuste «Tamaño de iconos y texto» cambia el tamaño de los botones.

## Cuando la pantalla es mayor que su panel

Con un **zoom fijo** (2x, 3x… o <kbd>Ctrl</kbd>+<kbd>+</kbd>), la pantalla del Apple-1 conserva el tamaño pedido aunque el panel sea más pequeño: entonces **se desplaza** (rueda o barras) en lugar de reducirse, y la barra de estado lo indica, con una sugerencia para agrandar el panel. Solo **Ajustar** adapta el zoom al panel. En una ventana estrecha (1024 × 768, por ejemplo), la barra de herramientas se desplaza de lado en lugar de cortarse: el botón **»** trae el grupo Casete, y un segundo clic vuelve al principio. El botón **Ejecutar** del editor siempre queda visible.

## Separar un panel

Cada panel tiene un botón **Separar** arriba a la derecha (también con un clic derecho en la pestaña) que lo abre en **su propia ventana**, que puede llevar a una segunda pantalla; su tamaño y su sitio se recuerdan. **⊟ Acoplar** lo devuelve a su sitio inicial.

- La pantalla del Apple-1 y el monitor en color se separan por separado. El teclado va al Apple-1 desde cualquiera de sus ventanas y desde la ventana principal. Cerrar la ventana de una pantalla la vuelve a acoplar.
- Cerrar la ventana de una herramienta (editor, depurador…) cierra la herramienta; su botón de la barra de herramientas la vuelve a abrir en el mismo sitio.
- Los cuadros de diálogo (Abrir, Guardar como, Exportar) siempre se abren en la ventana principal. En los diálogos de archivos, <kbd>Esc</kbd> cierra el diálogo (también desde el campo del nombre de archivo), <kbd>Ctrl</kbd>+<kbd>L</kbd> edita la ruta (su texto queda seleccionado, así que lo que escriba lo sustituye); Guardar y Exportar de una pestaña empiezan en la carpeta de su archivo.
- Las disposiciones guardadas conservan las ventanas separadas; la ventana principal conserva el mismo tamaño de un arranque a otro, sea cual sea el zoom.
- El osciloscopio del casete sigue siendo una pequeña ventana flotante dentro de la ventana principal.

<div class="cards">
<div class="card"><h4>Para jugar</h4>Cierre el editor y el depurador, conserve la biblioteca. <strong>Ajustar</strong> agranda la pantalla; <kbd>F11</kbd> pasa a pantalla completa, con la pantalla vintage.</div>
<div class="card"><h4>Para programar</h4>Editor a la izquierda, pantalla en el centro, depurador y volcado a la derecha. Separe el editor en un segundo monitor para darle toda la altura.</div>
<div class="card"><h4>Para la GEN2</h4>Conecte la tarjeta: el monitor en color se coloca bajo la pantalla del Apple-1. Sepárelo para tenerlo grande, como un segundo monitor.</div>
<div class="card"><h4>Para una charla</h4><strong>Separe</strong> la pantalla del Apple-1 en el proyector, a pantalla completa, y la ventana principal en su portátil.</div>
</div>

# Preferencias

<p class="lead">⚙ Preferencias reúne todos los ajustes permanentes. Todo se guarda en ~/.config/apple1-emu/gui.json (o en la carpeta del kit, en modo portátil).</p>

![La pestaña Interfaz de las preferencias: idioma, tema, tamaño de iconos y texto, barra compacta, burbujas, ventana de inicio y colores de la interfaz.](img/es/preferences.png "large")

Los ajustes se reparten en seis pestañas:

| Pestaña | Ajustes |
|---|---|
| **Interfaz** | idioma: automático (según la distribución del teclado) o forzado: francés, inglés, alemán, ruso, español, italiano, portugués; se muestran el idioma detectado y su origen. Tema: oscuro, claro o sistema. Tamaño de iconos y texto: pequeño, normal (por defecto), grande, muy grande. Barra compacta (solo iconos cuando la ventana es estrecha). Burbujas al pasar el ratón. **Mostrar esta ventana al iniciar** (la ventana «Su espacio de trabajo») y el botón que la abre al instante. **Colores de la interfaz**: los colores de las listas de la interfaz, elegidos **por separado para el tema oscuro y el claro** (botones activos, línea actual del depurador, línea resaltada, celda seleccionada, línea de punto de parada, direcciones y columna ASCII del volcado, valores cambiados, etiquetas); cada color tiene una muestra con vista previa en directo, los valores por defecto son legibles (contraste de al menos 4,5:1 sobre el fondo del tema), y los botones **Restablecer** devuelven un tema o ambos. |
| **Editor** | colores de sintaxis, por separado para BASIC y para el ensamblador 6502 (capítulo 7) |
| **Biblioteca** | la carpeta principal (**Examinar…**, **⟲ Predeterminada**, **Abrir la carpeta**), las demás carpetas, los proyectos de GitHub y el token de GitHub opcional (solo para repositorios privados y artefactos de Actions; nunca se muestra, y la variable `GITHUB_TOKEN` tiene prioridad); posición de la biblioteca, a la izquierda o a la derecha (a la derecha por defecto) |
| **Casete** | sonido del casete (mudo a velocidad rápida) y su volumen, osciloscopio durante la reproducción o la grabación |
| **GEN2** | la tarjeta de color de Uncle Bernie, barrido de 60 o 50 Hz, paleta LORES (capítulo 6) |
| **Avanzado** | visualización y velocidad: mostrar la biblioteca, visualización limitada a ~60 caracteres/s como el terminal real, velocidad rápida; **Pantalla vintage: ajustes finos** (comunes a las dos pantallas, capítulo 3); el archivo Integer BASIC cargado en `$E000` (**📂 Elegir…** / **Olvidar**) |

Cada deslizador de las preferencias (intensidad, remanencia y curvatura de la pantalla vintage, volumen del casete) muestra su **valor numérico** al lado. En el primer arranque, la curvatura de la pantalla vintage vale **0,07** (un ligero redondeo del tubo); una instalación existente conserva su propio valor. La vista previa de la pestaña **Editor** sigue el idioma de la interfaz.

El zoom, el fósforo y la casilla de la pantalla vintage no están en las preferencias: están en la fila de visualización de cada pantalla (capítulo 3).

![La pestaña Editor: un esquema de colores para BASIC y para el ensamblador 6502, o sus propios colores.](img/es/preferences-editeur.png "large")

**Idioma automático.** El emulador sigue la distribución activa del teclado: `fr` da francés, `us` o `gb` inglés, `de`, `ch` o `at` alemán, `ru` ruso, etc. Pregunta a GNOME, `setxkbmap`, Cinnamon (el caso de Linux Mint), `localectl`, luego a `/etc/default/keyboard` y, por último, a las variables `LANG`. En la línea de comandos: `--langue fr|en|de|ru|es|it|pt|auto`.

!!! astuce "Empezar de cero"
    **Restablecer las preferencias**, al pie de la ventana, restaura los ajustes de origen tras una confirmación (idioma, tema, disposición, pantalla vintage…). Los archivos de su biblioteca se quedan donde están; las pestañas del editor sin guardar se cierran. El primer arranque de una versión nueva del kit hace la misma pregunta, porque `gui.json` sobrevive cuando se borra la carpeta del programa para instalar otra.

    Para un kit portátil, ponga un archivo vacío llamado `portable` en la raíz del kit (junto a `run.sh`), o arránquelo con `APPLE1_PORTABLE=1`: las preferencias (`config/gui.json`), la biblioteca por defecto (`programmes`) y la caché quedan entonces en la carpeta del kit.

    Cerrar el emulador y cambiar el nombre de `~/.config/apple1-emu/gui.json` hace lo mismo que el botón, en el siguiente arranque. Los ajustes de los programas se quedan en el `programs.json` de cada carpeta.
