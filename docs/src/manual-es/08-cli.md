# La línea de comandos

<p class="lead">Dos programas: apple1-emu-gui, la interfaz gráfica, y apple1-emu, el emulador en terminal y caja de herramientas de conversión. Todo lo que hacen los botones de exportación se puede hacer también desde la línea de comandos, para sus scripts.</p>

!!! attention "Las herramientas de línea de comandos hablan francés"
    La interfaz gráfica existe en español (`--langue es`), pero `apple1-emu` muestra sus mensajes y su ayuda solo en francés, y las opciones de la interfaz, los comandos de F2 y las palabras clave del depurador son palabras francesas (`--carte-couleur`, `charger`, `pas`…). Este capítulo muestra la salida real y la traduce donde hace falta.

## El emulador en terminal

`apple1-emu` sin comando ejecuta el Apple-1 en el terminal (al menos 42 columnas × 28 líneas): 8 KB de RAM más `$E000`, ACI, velocidad real.

```sh
apple1-emu                                  # 8 KB + E000, ACI, velocidad real
apple1-emu --fast                           # arrancar en modo rápido
apple1-emu --load samples/hello.txt --run 300
apple1-emu --tape samples/hello.wav         # casete en el magnetófono virtual
apple1-emu --basic ~/apple1/integer_basic.bin
apple1-emu --ram 32 --no-e000               # 32 KB, sin RAM en E000
```

| Opción | Efecto |
|---|---|
| `--ram KB` | RAM desde `$0000`, de 4 a 48 KB (8 por defecto) |
| `--no-e000` | sin RAM en `$E000-$EFFF` (el puente «W → E» de la placa) |
| `--no-aci` | sin tarjeta de casete |
| `--basic ARCHIVO` | su Integer BASIC (binario de 4 KB o volcado hexadecimal), cargado en `$E000` |
| `--load ARCHIVO[@DIR]` | un programa a la memoria (`.txt`, `.bin@DIR`, `.wav`, `.aif`); repetible |
| `--tape ARCHIVO` | un casete en el magnetófono |
| `--type ARCHIVO` | escribe un archivo de texto al arrancar |
| `--run DIR` | escribe `DIRR` después de cargar |
| `--fast`, `--no-throttle` | modo rápido; visualización sin el límite de 60 caracteres/s |
| `--no-refresh` | sin el modelo de refresco de la DRAM (1,0227 MHz en lugar de 0,960) |
| `--carte-couleur`, `--couleur-50hz` | la tarjeta GEN2 (registros y memoria; la imagen en color solo existe en la interfaz gráfica) |
| `--cursor-hz HZ`, `--cursor-duty F` | parpadeo del cursor (2,18 Hz, visible 2/3 por defecto) |
| `--monitor-rom`, `--monitor-page` | otra imagen del Woz Monitor (256 o 512 bytes, página 0 o 1) |
| `--aci-rom`, `--aci-page` | otra imagen de la PROM de la ACI |

**Teclas del terminal.** <kbd>Ctrl</kbd>+<kbd>R</kbd> RESET, <kbd>Ctrl</kbd>+<kbd>L</kbd> CLEAR SCREEN, <kbd>F1</kbd> ayuda, <kbd>F2</kbd> línea de comandos, <kbd>F3</kbd> velocidad, <kbd>F4</kbd> límite de visualización, <kbd>F5</kbd>/<kbd>F6</kbd> reproducir/parar el casete, <kbd>F10</kbd> o <kbd>Ctrl</kbd>+<kbd>Q</kbd> salir. El texto pegado en el terminal se escribe en el teclado del Apple-1. Retroceso escribe `_`, el borrado del Woz Monitor.

**Comandos de F2** (las palabras de los comandos están en francés).

| Comando | Efecto |
|---|---|
| {{charger}} `ARCHIVO[@DIR]` | *cargar*: carga en memoria un `.txt`, un `.bin` (`@DIR` obligatorio), un `.wav` o un `.aiff` |
| {{sauver}} `ARCHIVO INICIO.FIN …` | *guardar*: guarda la memoria; la extensión elige el formato. Los rangos se pueden copiar de la línea de la ACI: `sauver st.aiff 004A.00FFR 0300.0FFFR` |
| {{cassette}} `ARCHIVO` | *casete*: inserta un casete, que arranca en la primera lectura de la ACI |
| `lecture`, `stop`, `rembobiner`, `ejecter` | el magnetófono: reproducir, parar, rebobinar, expulsar |
| {{enregistrer}} `ARCHIVO` | *grabar*: arma la grabadora; luego escriba `C100R` y `0300.03FFW` |
| `basic ARCHIVO` | carga su Integer BASIC en `$E000` |
| `taper ARCHIVO` | *teclear*: escribe un archivo de texto en el teclado |
| `reset`, `effacer`, `rapide`, `reel`, `debit`, `aide`, `quitter` | reset, borrar, rápido, velocidad real, ritmo de visualización, ayuda, salir |

## Conversiones

El formato de salida lo elige la **extensión**: `.aif`/`.aiff` da un AIFF (perfil smartphone), `.wav` un WAV, `.txt`/`.hex` un volcado Woz, `.bin` un binario.

{{export}} (alias de `export-wav`) produce un casete. La señal la crea la ROM real de la ACI ejecutándose en el 6502 emulado: los tiempos de los bits son los de un Apple-1 que ejecuta `W`.

```sh
apple1-emu export samples/hello.txt -o hello.aiff
apple1-emu export prog.bin@300 -o prog.wav --rate 48000 --bits 16
apple1-emu export startrek.txt --range "004A.00FFR 0300.0FFFR" -o startrek.aiff
```

| Opción | WAV (por defecto) | AIFF (por defecto) |
|---|---|---|
| `--rate` | 44 100 Hz | 48 000 Hz |
| `--bits 8\|16` | 16 | 16 |
| `--channels 1\|2` | 1 | 2, idénticos |
| `--amplitude` (de 0,05 a 1) | 0,8 | 0,89 (−1 dBFS) |
| `--silence S` | 0,5 s | 1,0 s |
| `--range` | un rango o una línea ACI entera; repetible | |
| `--no-refresh`, `--no-embed` | bits un 6 % más cortos; sin direcciones en el archivo | |

{{import}} (alias de `import-wav`) decodifica un casete: `-o prog.txt` o `-o prog.bin`, `--addr DIR` para un casete sin direcciones, `--range` para colocar varias grabaciones.

{{info}} describe un archivo (grabaciones, direcciones, duración); con `--aci-input stock|mimeo|gen2`, `--volts` y `--series` simula la entrada analógica de la ACI (sección 5.8).

{{convert}} `ENTRADA SALIDA` convierte entre `.txt`, `.bin`, `.wav` y `.aiff`.

## BASIC y ensamblador

```sh
apple1-emu basic juego.bas -o juego.wav          # listado tokenizado -> casete
apple1-emu basic juego.bas -o juego.txt --ram 16 # volcado Woz, para 16 KB
apple1-emu basic juego.bas                       # solo comprobar
apple1-emu asm juego.s -o juego.wav              # fuente 6502 -> casete
apple1-emu asm juego.s -o juego.bin -l juego.lst # binario + listado
apple1-emu asm juego.s -I ../lib                 # comprobar, ruta de inclusión adicional
```

{{basic (línea de comandos)|`basic`}} tokeniza un listado como lo hace el intérprete y escribe el bloque `$004A-$00FF` y luego el programa; `--ram 8|12|16|32` elige la máquina de destino (por defecto, la más pequeña que basta). {{asm (línea de comandos)|`asm`}} ensambla una fuente ca65 (o xa); sin `-o`, solo comprueba. Ambos muestran la línea que hay que escribir en el Apple-1 real.

```text title="Salida de apple1-emu asm hello.s -o hello.txt"
33 octets, $0300-$0320 ; 2 fichier(s), 13 symboles
Cassette (après C100R) : 0300.0320R   puis 300R
écrit : hello.txt
```

Es decir: «33 bytes, $0300-$0320; 2 archivo(s), 13 símbolos / Casete (tras C100R): 0300.0320R y luego 300R / escrito: hello.txt».

## Arte ASCII

{{ascii}} `IMAGEN`: `--cols`, `--rows` (23 para conservar el indicador), `--zoom`, `--cx`, `--cy`, `--brightness`, `--contrast`, `--gamma`, `--invert`, `--dither`, `--no-auto-level`, `--ramp simple|full|slideshow|LISTA`, `--format asm|basic`, `--title`, `-o` (`.txt`, `.bas`, `.s`, `.wav`, `.aif`).

## PROM

{{prom}} prepara archivos para programar PROM reales de 256 × 4 (82S129, 3601, 6301), en formato de volcado de nibbles:

```sh
apple1-emu prom split woz -o proms/     # PROM-A2.bin (D7-D4) y PROM-A1.bin (D3-D0)
apple1-emu prom split aci -o proms/     # PROM-A3.bin (D7-D4) y PROM-A4.bin (D3-D0)
apple1-emu prom join A2.bin A1.bin      # reunir una lectura y compararla
```

`--swap-a1-a7` solo sirve para una placa cableada según el esquema de Apple de 1976, que cruza las líneas de dirección 1 y 7 de la PROM A2.

Para grabar una EPROM de 2716 a 27C512 o una EEPROM 28C desde la interfaz gráfica (un programador TL866II+, T48 o T56 con `minipro`), vea el programador de EPROM, sección 2.6.

## La interfaz gráfica

`apple1-emu-gui [OPCIONES] [ARCHIVO…]` carga los archivos indicados al arrancar. Opciones principales (los nombres de las opciones están en francés):

| Opción | Efecto |
|---|---|
| `--zoom N\|ajuster`, `--taille AxA` | zoom de la pantalla (`ajuster` = ajustar), tamaño de la ventana |
| `--phosphore vert\|blanc\|ambre` | fósforo (verde, blanco, ámbar) |
| `--screening`, `--sans-screening`, `--screening-options LISTA` | pantalla vintage del Apple-1 y sus ajustes |
| `--gen2-screening` | pantalla vintage en el monitor en color GEN2 |
| `--theme clair\|sombre\|systeme` | tema claro, oscuro o del sistema |
| `--taille-icones auto\|petite\|normale\|grande\|tres-grande` | tamaño de iconos y texto |
| `--barre-complete` | no reducir nunca la barra de herramientas a iconos |
| `--bibliotheque CARPETA`, `--basic ARCHIVO` | carpeta de la biblioteca, archivo BASIC |
| `--carte-couleur`, `--sans-carte-couleur`, `--couleur-50hz` | tarjeta GEN2 |
| `--langue CÓDIGO\|auto` | idioma de la interfaz (`--langue es` para español) |
| `--mode-rapide`, `--mode-cassette` | carga desde la biblioteca: instantánea (por defecto) o por el casete a velocidad real |
| `--cassette ARCHIVO` | inserta un casete |
| `--ecran-separe`, `--debogueur` | pantalla en su propia ventana, depurador abierto |
| `--editeur ARCHIVO`, `--ascii ARCHIVO` | abre un archivo en el editor o en la herramienta ASCII |

Para demostraciones y capturas, algunas opciones reproducen acciones al cabo de unos segundos: `--lancer-biblio S:NOMBRE`, `--charger-biblio S:NOMBRE`, `--parametres S:NOMBRE`, `--preferences S` (y `--onglet-prefs interface|editeur|bibliotheque|cassette|gen2|avance`), `--editeur-lancer S`, `--editeur-recherche TEXTO` (barra de búsqueda abierta), `--editeur-menu` y `--ecran-menu` (menús del clic derecho abiertos), `--menu-etat` (el menú Estado abierto), `--eprom` (el programador de EPROM abierto), `--fenetre-fichiers` (la ventana Cargar abierta), `--editeur-exporter S:ARCHIVO`, `--ascii-reglages R`, `--ascii-lancer S`, `--taper TEXTO`, `--taper-a S:TEXTO`, `--dbg-arret`, `--dbg-surveiller`, `--dbg-commande S:CMD`, `--capture IMAGEN.png` (tras `--delai S`) y `--quitter`.

# Resolución de problemas y preguntas frecuentes

## La ventana no se abre

Falta el controlador OpenGL, o está en una máquina virtual. `run.sh` vuelve a intentarlo automáticamente con el renderizado por software de Mesa; a mano: `LIBGL_ALWAYS_SOFTWARE=1 ./apple1-emu-gui`. La pantalla vintage sigue siendo fluida con el renderizado por software.

## Las teclas no llegan al Apple-1

Haga clic en la pantalla del Apple-1. Cuando un campo de texto o un cuadro de diálogo está activo, el teclado va a ese campo; la barra de estado indica adónde va el teclado.

## «Integer BASIC introuvable» (Integer BASIC no encontrado)

El BASIC no se incluye. Ponga su `BASIC.wav` (o un `.bin` de 4096 bytes) en la biblioteca, o indíquelo con el botón **BASIC** o en **⚙ Preferencias → Integer BASIC**. El emulador lo encuentra entonces solo.

## Un programa BASIC se detiene con STOPPED AT

Se pulsó una tecla mientras se ejecutaba: en el Apple-1, cualquier tecla interrumpe un programa BASIC. Escriba `RUN` para volver a empezar, o `GOTO` seguido del número de línea para continuar sin borrar las variables.

## El programa no arranca después de cargarlo

- Programa BASIC: hace falta `E2B3R` (entrada en caliente), no `E000R`, que borra el programa. El BASIC debe estar en memoria **antes** que el programa.
- Código máquina: compruebe la dirección de ejecución en los ajustes ⚙; no siempre es `300R` (`280R`, `1000R`, `2000R`…).
- Programa de más de 8 KB: ▶ añade la RAM, pero no una carga manual. Inícielo con ▶, o use `--ram 16`.

## El casete no carga en el Apple-1 real

<ol class="steps">
<li>Volumen al máximo, todos los efectos de audio desactivados, sin notificaciones.</li>
<li>RETURN durante la cabecera, ni antes de que empiece la reproducción ni después de que termine la cabecera.</li>
<li>Los rangos exactos, con cuatro cifras: <code>004A.00FFR</code>, nunca <code>4A.FFR</code>.</li>
<li>Pruebe otro reproductor (VLC), luego <code>--rate 44100</code>, <code>--channels 1</code> o <code>--no-refresh</code>.</li>
<li>Compruebe el nivel con <code>apple1-emu info prog.aiff --aci-input stock</code> (sección 5.8).</li>
<li>Si la carga no termina nunca: RESET y vuelva a intentarlo con más volumen.</li>
</ol>

## La pantalla está llena de @ en vídeo inverso

Es el monitor en color GEN2: su memoria empieza a cero, y el código `$00` es una `@` en vídeo inverso. El programa todavía no ha llenado la página de texto. En la tarjeta real el contenido sería aleatorio.

## Los caracteres salen despacio

Es normal: el terminal del Apple-1 muestra unos 60 caracteres por segundo. Haga clic en **Pantalla: ~60 car./s** en la barra de estado (o en **Rápido** en la fila de visualización de la pantalla) para una visualización instantánea, y pase al modo rápido (<kbd>F3</kbd>) para acelerarlo todo.

## El sonido del casete está mudo

Está desactivado a velocidad rápida, y cuando faltan ALSA o la tarjeta de sonido: el osciloscopio lo indica («mudo a velocidad máxima», «sin salida de audio»). El volumen se ajusta en el osciloscopio; 🔊 en la barra de herramientas activa o desactiva el sonido.

## El proyecto de GitHub no se compila

«Probar la última compilación» compila con ca65/ld65 (paquete `cc65`) si no encuentra ni una Release ni un artefacto. Instale `cc65`, o añada un `apple1.toml` que nombre el archivo correcto. Algunas fuentes necesitan un ca65 reciente (2.19 o posterior); el error de ca65 se muestra entonces tal cual.

## Lo que el emulador no hace

- Los opcodes no documentados del 6502 se tratan como NOP de la longitud correcta, y los opcodes JAM detienen el procesador; sus efectos exactos no se reproducen (el software del Apple-1 no los usa).
- El comparador de la ACI es ideal durante la emulación; solo el comando `info` lo simula como circuito analógico.
- La emulación se ha comprobado con las ROM, los listados y herramientas independientes (c2t, ca65, xa), pero el autor de este manual aún no ha reproducido los archivos de audio en un Apple-1 real.
