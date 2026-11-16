# La biblioteca

<p class="lead">La biblioteca es el punto de partida de casi todo: guarda sus casetes, volcados y fuentes, reconoce los casetes de la época y ejecuta un programa con un clic, BASIC incluido.</p>

![El panel de la biblioteca: la carpeta principal, la carpeta «Art ASCII», casetes reconocidos y un proyecto de GitHub. BASIC.wav se está reproduciendo.](img/es/bibliotheque.png "small")

## Carpetas

La carpeta principal es `~/apple1/programmes`. Se crea en el primer arranque, con los ejemplos repartidos en cinco carpetas: **Art ASCII** (once ejemplos de arte ASCII en 40 × 24, capítulo 9), **BASIC** (programas `.bas` y volcados BASIC `.hex.txt`), **6502** (fuentes `.s`/`.asm` con sus `.bin`/`.hex`), **GEN2 COLOR** (las pruebas de la tarjeta GEN2 de Bernie, `gen2-*.apl`, y los juegos en color para G2 Basic) y **WAV** (todos los casetes). En el panel, esas cinco carpetas aparecen directamente en el nivel superior, en el orden **BASIC**, **6502**, **GEN2 COLOR**, **WAV**, **Art ASCII**: la carpeta raíz (el nombre del directorio en el disco) no se muestra encima, y los archivos sueltos en esa carpeta no aparecen. Póngalos en una de las cinco carpetas. Una carpeta que usted añade, o un proyecto de GitHub, conserva su propio título. Una biblioteca existente nunca se reorganiza, salvo que sus carpetas **Assembleur** y **Uncle Bernie** (los nombres de las versiones 7.8 a 7.9.1) pasan a llamarse **6502** y **GEN2 COLOR**, con su contenido. Para elegir otra: **⚙ Preferencias → Carpeta de la biblioteca → 📂 Examinar…** (**⟲ Predeterminada** vuelve a `~/apple1/programmes`).

**+ Carpeta**, en lo alto del panel, añade otras carpetas; la carpeta principal siempre se queda. La biblioteca es un árbol: carpetas y subcarpetas se pliegan, y su estado se recuerda. Cada carpeta guarda sus propios ajustes en un archivo `programs.json`. Clic derecho en una carpeta añadida para abrirla en el gestor de archivos o quitarla de la lista (no se borra nada del disco).

**Panel estrecho.** La biblioteca puede ser tan estrecha como quiera (hasta una anchura mínima, por debajo de la cual el separador se detiene). Cada entrada se adapta entonces: el nombre ocupa todo el ancho, recortado con «…» si es demasiado largo (la burbuja muestra el nombre completo), los pequeños iconos ▶ ⬇ 📼 ⚙ pasan a la línea siguiente y la descripción se limita a una o dos líneas, también recortada con «…», con el texto completo en la burbuja. En la cabecera, **+ Carpeta** y **+ Proyecto GitHub** se convierten en iconos con burbujas.

Los archivos que se muestran son: `.wav .aiff .bin .hex .txt .apl .bas .s .asm .prg`. La lista se actualiza sola cada dos segundos: copie un archivo en la carpeta y aparece.

Para cada programa, la lista muestra el nombre, el tipo, los rangos de direcciones, la duración del casete, el comando de ejecución y sus notas.

### Los juegos de Claude IA

Las carpetas **BASIC** y **6502** reciben también cinco pequeños juegos escritos por Claude IA, cada uno en versión francesa e inglesa: *Ahorcado* (`pendu.bas`, `hangman.bas`), *Cerillas* (`allumettes.bas`, `matches.bas`) y *Batalla naval* (`bataille.bas`, `battleship.bas`) en BASIC; *Túnel* (`tunnel.s`, `tunnel-en.s`) y *Tres en raya* (`morpion.s`, `tictactoe.s`) en ensamblador 6502. Un doble clic en un juego BASIC lo ejecuta con RDK G2 Basic (el emulador inicia el BASIC, carga el listado y escribe `RUN`); las fuentes en ensamblador las ensambla el ensamblador integrado. Las fuentes en ensamblador incluyen `apple1.inc`, que se instala con ellas. La carpeta **GEN2 COLOR** contiene además *Conecta 4 en color* (`puissance4-couleur.bas`), un juego para G2 Basic y la tarjeta GEN2, de Claude IA. Q y S mueven la ficha sobre la rejilla, Espacio la suelta. Los juegos forman parte del proyecto y están bajo su licencia (anexo «Licencia y créditos»). Las versiones en español de estos juegos están en `docs/exemples/es/`.

## Ejecutar un programa

Cada programa tiene cuatro pequeños iconos a su derecha:

| Icono | Acción |
|---|---|
| **▶** | **Cargar y ejecutar**: carga el programa si hace falta y luego lo ejecuta: RESET y después su comando escrito en el teclado (`300R`, o `E2B3R` y `RUN` para un programa BASIC). Si el programa necesita Integer BASIC y no está en `$E000`, el emulador lo carga primero. Un listado BASIC se escribe en el Integer BASIC; una fuente 6502 se ensambla. |
| **⬇** | **Cargar**: pone el programa en memoria al instante, sin iniciarlo. |
| **📼** | **Por el casete**: como en la máquina real. El emulador pulsa RESET, escribe `C100R` y luego el comando de lectura de la ACI (por ejemplo `0300.031ER`), y el casete se reproduce a velocidad real, con su sonido y una barra de progreso. Los archivos `.txt` y `.bin` se convierten antes en casete. Atenuado para una fuente (`.bas`, `.s`), que no tiene grabación. |
| **⚙** | **Parámetros**: rangos, dirección de carga de un `.bin`, dirección y comando de ejecución, tipo, si hace falta BASIC, notas. |

{{!doble clic}}La carga es **instantánea** salvo que elija el casete con 📼. Un **doble clic** en un programa lo carga y lo ejecuta si tiene comando de ejecución (como ▶). El casete solo se usa cuando usted lo pide; el magnetófono de la barra de herramientas muestra entonces el casete y su posición.

Cuando el editor está abierto, ▶, ⬇ o un doble clic en un listado BASIC o una fuente 6502 también lo **abren en una pestaña del editor**, marcada con 📚; si ya está abierto, simplemente se muestra su pestaña.

Un **clic derecho** en un programa ofrece el resto: cargar, cargar y ejecutar, insertar como casete sin escribir nada, parámetros, renombrar, eliminar (con confirmación), abrir la carpeta.

## Los ajustes de un programa

![Los ajustes ⚙ de Hamurabi: dos rangos (página cero del BASIC y luego el programa), iniciado con E2B3R y luego RUN.](img/es/brut/bibliotheque-params.png "large")

| Campo | Función |
|---|---|
| **Rangos** | Las zonas de memoria, escritas como para la ACI: `0300.031E`, o `004A.00FF 0800.0FFF` para un programa BASIC. Cada grabación del casete va a su rango, por orden. |
| **Dirección de carga** | Para un `.bin`, que no contiene dirección. |
| **Ejecutar** | La dirección y el comando que escribe ▶. `\n` representa RETURN entre dos líneas: `E2B3R\nRUN`. |
| **Tipo** | **Código máquina**, **Programa G2 Basic** (el valor por defecto para un listado BASIC, ya que el emulador lo proporciona) o **Programa Integer BASIC** (para quien prefiere el original de Apple: ▶ carga entonces el Integer BASIC original elegido en las preferencias). El tipo elegido para un programa tiene prioridad sobre el ajuste general de BASIC de las preferencias. |
| **Necesita un BASIC** | ▶ cargará primero el BASIC del tipo del programa. |
| **Notas** | Texto libre, mostrado en la lista. |

Los valores se rellenan a partir del archivo y se comprueban al guardar; **⟲ Valores del archivo** los vuelve a leer.

!!! attention "Siempre cuatro cifras: 004A.00FF, nunca 4A.FF"
    Los ajustes rechazan `4A.FF`. No es un capricho: la ROM de la ACI no borra su acumulador hexadecimal en el punto, así que las cifras siguen entrando y leería `$004A-$4AFF` (ver sección 5.6). Por eso el emulador escribe siempre las direcciones con cuatro cifras.

## Casetes reconocidos

Cuando un archivo es **reconocido**, sus ajustes se rellenan solos y la lista muestra **✔ reconocido**. El emulador decodifica las grabaciones y compara sus tamaños y sus huellas SHA-256 con una tabla integrada: los 24 casetes de la colección «Apple1Cassettes» de archive.org. El nombre del archivo no importa en absoluto. A falta de eso, reconoce el **nombre** (sin tener en cuenta mayúsculas, espacios ni guiones) con los mismos tamaños de grabación; la burbuja de ayuda señala entonces que el contenido difiere.

| Casete | Rangos | Ejecución |
|---|---|---|
| `BASIC.wav` (Integer BASIC) | `E000.EFFF` | `E000R` |
| `Hamurabi.wav`, `StarTrek.wav`, `Wumpus.wav`, `Slots.wav`, `Craps.wav`, `Bowling.wav`, `Buzzword.wav`, `WordCross.wav`, `AppleiEnhancedCheckers.wav`, `LunarlanderasciiGraphics.wav` | `004A.00FF 0300.0FFF` | `E2B3R ⏎ RUN` |
| `AceyDucey.wav`, `Blackjack.wav`, `HiLo.wav` | `004A.00FF 0800.0FFF` | `E2B3R ⏎ RUN` |
| `Football.wav` | `004A.00FF 0400.2FFF` | `E2B3R ⏎ RUN` (12 KB) |
| `DealOrNoDeal.wav`, `StarTrek2003.wav` | `004A.00FF 0300.3FFF` | `E2B3R ⏎ RUN` (16 KB) |
| `APPLE30TH.wav` | `0280.0FFF` | `280R` |
| `ASMmchess.wav` | `0300.0BFF` | `300R` |
| `Microchess.wav` | `1000.18FF` | `1000R` |
| `LunarLander.wav` | `0300.09FF` | `300R` |
| `Mastermind.wav` | `0300.03FF` | `300R` |
| `Pasart.wav` | `0300.05FF` | `300R` |
| `Life.wav` | `2000.21FF` | `2000R` (12 KB) |

Sus propios ajustes nunca se sobrescriben: un programa que usted mismo ha configurado conserva sus ajustes, solo aparece la marca «reconocido». Los programas que superan los 8 KB básicos reciben automáticamente la RAM que necesitan al iniciarse, como con una tarjeta de ampliación.

**Casete desconocido.** El emulador adivina lo que puede: una grabación de 4096 bytes que empieza por `JMP` es el Integer BASIC; dos grabaciones de las que la primera mide 182 bytes (`$4A-$FF`) son un programa BASIC, colocado según sus punteros LOMEM y HIMEM. La lista muestra entonces «(supuestos)»; compruébelo con ⚙. Un programa en código máquina sin direcciones no se adivina: indique su rango.

![Digitalizar un casete real: del casete a un archivo y luego a la biblioteca.](diagrams/es/workflow-capture.svg "diagram large")

## Proyectos de GitHub

**+ Proyecto GitHub** acepta una dirección (`https://github.com/jefftranter/6502/tree/master/asm/jmon`) o simplemente `propietario/repositorio`. Los archivos útiles se copian en `~/.cache/apple1-emu/github/<propietario>/<repositorio>/<commit>/`, y el proyecto se convierte en un nodo del árbol, con su commit, su fecha y su mensaje. **⟳ Actualizar** trae el último commit; 🌐 abre la página del repositorio. Todo el acceso a la red se hace en segundo plano.

{{Probar la última compilación}} intenta, por orden:

<ol class="steps">
<li>los archivos de la última <strong>Release</strong> del repositorio (<code>.wav</code>/<code>.aiff</code>, luego <code>.bin</code>, luego <code>.hex</code>/<code>.txt</code>);</li>
<li>el último <strong>artefacto de GitHub Actions</strong> (hace falta un token);</li>
<li><strong>compilar desde las fuentes</strong>: un listado BASIC se escribe en el Integer BASIC y se inicia con <code>RUN</code>; una fuente 6502 se ensambla con <strong>ca65/ld65</strong> (paquete <code>cc65</code>), con <code>-D APPLE1</code> por defecto; un volcado Woz se carga.</li>
</ol>

![Un proyecto de GitHub, de la dirección del repositorio al programa en marcha.](diagrams/es/workflow-github.svg "diagram large")

Un archivo `apple1.toml` (o `apple1.cfg`) en la raíz del repositorio indica qué hay que compilar:

```toml title="apple1.toml"
file    = "src/game.s"    # programa que se ejecuta
load    = $0300           # dirección de carga (si no, el .org de la fuente)
run     = $0300           # dirección de ejecución (si no, la de carga)
ld65    = "apple1.cfg"    # configuración de ld65 (si no, -t none)
defines = "APPLE1"        # símbolos -D para ca65
```

!!! astuce "El token de GitHub es opcional"
    Los repositorios públicos no lo necesitan. Un token (variable `GITHUB_TOKEN`, o **⚙ Preferencias**) aumenta el límite de peticiones y da acceso a sus repositorios privados y a los artefactos. Nunca se muestra ni se escribe en los mensajes, y `gui.json` pasa entonces a ser legible solo por su cuenta.

## La carpeta «Art ASCII»

La biblioteca contiene también una carpeta **Art ASCII**: dibujos en el formato de la pantalla (manzana, placa del Apple-1, chip 6502, casete, cohete…) y retratos de Steve Wozniak y Steve Jobs convertidos a partir de fotos de licencia libre. Un `.txt` de arte ASCII muestra una vista previa al seleccionarlo, con **→ BASIC**, **→ 6502** y **🎨 Abrir en la herramienta**; **▶** ejecuta al instante su versión 6502. El capítulo 9 describe la herramienta.
