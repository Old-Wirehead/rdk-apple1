# El casete y la tarjeta ACI

<p class="lead">El Apple-1 no tenía disquete ni BASIC en ROM: todo pasaba por un casete de audio y la tarjeta ACI (Apple Cassette Interface). El emulador ejecuta la ROM real de la ACI, así que sus archivos WAV y AIFF se cargan en un Apple-1 real, y las grabaciones hechas en un Apple-1 real se cargan en el emulador.</p>

## Formatos de archivo

| Extensión | Contenido | Lectura | Escritura |
|---|---|---|---|
| `.wav` | casete ACI, PCM de 8 a 32 bits o coma flotante, mono o estéreo, cualquier frecuencia | sí | 44,1 kHz mono de 16 bits por defecto |
| `.aif`, `.aiff` | casete ACI (AIFF y AIFF-C) | sí | perfil smartphone: 48 kHz, 2 canales idénticos, 16 bits |
| `.txt`, `.hex`, `.apl` | volcado del Woz Monitor: líneas `0300: A9 C1 20 …` | sí | sí |
| `.bin` | bytes en bruto, sin dirección (`ARCHIVO.bin@300`) | sí | sí |
| `.bas` | listado Integer BASIC (texto) | en el editor | en el editor |
| `.s`, `.asm`, `.a65` | fuente 6502 (ca65 o xa) | en el editor | en el editor |

Los casetes exportados por el emulador llevan sus direcciones en un bloque de comentario del archivo (`APPLE1 ACI 0300.031E`), que los reproductores de audio ignoran. Un casete de otra procedencia no tiene direcciones: la biblioteca las reconoce o las adivina (capítulo 4), y la línea de comandos acepta `--addr` o `--range`.

El decodificador tolera ruido, niveles bajos, polaridad invertida, un error de velocidad de la cinta de ±10 % y varias grabaciones en un mismo archivo. También lee los WAV de la herramienta **c2t** de Egan Ford. Un **zumbido de red** (50 o 60 Hz, incluso tan fuerte como la propia señal del casete) se filtra antes de detectar los flancos, de modo que una grabación larga hecha con un cable mal apantallado se sigue leyendo bien.

## El magnetófono virtual

La franja **Casete:** de la barra de herramientas reúne todos los controles: **📼 Insertar**, **▶** reproducir, **⏹** parar, **⏪** rebobinar, **⏺** grabar, **⏏** expulsar, y luego el nombre y la posición del casete, el osciloscopio **📈** y el sonido **🔊**. Un casete se inserta con **📼 Insertar** (un `.wav` o `.aiff`), o con un clic derecho en un programa de la biblioteca («Insertar como casete»): luego escribe usted `C100R` y el rango. El icono **📼** de un programa de la biblioteca va un paso más allá: inserta el casete y escribe los comandos por usted.

![La franja Casete: mientras se lee BASIC.wav: 7,6 segundos de 35,8.](img/es/barre-cassette.png "large")

El casete sigue el tiempo **emulado**: a velocidad rápida (<kbd>F3</kbd>), la carga dura mucho menos que la cabecera de 10 segundos, y funciona igual. Sin casete, las cargas desde la biblioteca son instantáneas (capítulo 4).

Cargar el Integer BASIC «a la antigua» se hace así:

```mon
» C100R
C100: A9*
» E000.EFFFR
» E000R

E000: 4C
>
```

`C100R` inicia el programa de la ACI, que responde con su indicador `*`. La línea `E000.EFFFR` le pide **leer** el casete en `$E000-$EFFF`. Al final del casete, la ACI vuelve al monitor (`\`); `E000R` inicia el BASIC, cuyo indicador es `>`.

## Sonido y osciloscopio

![El osciloscopio mientras suena BASIC.wav: la casilla «Sonido del casete», el volumen y la señal a 1 ms por división.](img/es/oscilloscope.png "small")

Durante la reproducción o la grabación, **🔊** le deja oír la señal, como el altavoz de un magnetófono, a volumen bajo por defecto. **📈** abre el **osciloscopio**: una traza verde sobre una cuadrícula oscura, 1 ms por división, que muestra 10 ms alrededor del cabezal o los últimos 10 milisegundos escritos por el Apple-1. Se distinguen la cabecera (≈ 800 Hz), los bits 0 (≈ 2 kHz) y los bits 1 (≈ 1 kHz). A velocidad rápida el sonido queda en silencio.

## Grabar desde el Apple-1 emulado

Para producir un casete como en 1976, arme la grabadora y luego pida a la ACI que escriba:

<ol class="steps">
<li>Haga clic en <strong>⏺</strong> y elija el archivo (<code>.wav</code> o <code>.aiff</code>).</li>
<li>Escriba <code>C100R</code>, luego el rango seguido de <code>W</code>, por ejemplo <code>0300.03FFW</code>, y <kbd>RETURN</kbd>.</li>
<li>La ACI escribe una cabecera de 10 segundos y luego los datos. El archivo se escribe 1,5 s después del final, o al pulsar <strong>⏹</strong>.</li>
</ol>

Lo más sencillo sigue siendo **Guardar** en la barra de herramientas, o **📼 Exportar WAV** en el editor: la señal es la misma, ciclo a ciclo, y se produce al instante.

## Cómo funciona la ACI

![Los dos sentidos de la ACI: al escribir, cualquier acceso a $C0xx invierte la salida; al leer, el comparador gobierna la línea de dirección A0 de la PROM.](diagrams/es/aci.svg "diagram large")

La tarjeta es notablemente sencilla: una PROM de 256 bytes visible en `$C000` y `$C100`, un biestable y un comparador.

- **Escritura.** Cada acceso a una dirección de `$C000-$C0FF` invierte la salida (74LS74). El programa de la ROM fija el tiempo entre dos accesos: ese tiempo codifica los bits.
- **Lectura.** El nivel de la entrada de cinta, tras el comparador LM311, gobierna la línea de dirección A0 de la PROM cuando A7 = 1. La misma instrucción lee por tanto `$C080` o `$C081` según el nivel: la ROM mide los semiperiodos con un bucle de 12 ciclos.
- **Codificación.** Un «1» dura aproximadamente el doble que un «0»: 1 kHz para un 1, 2 kHz para un 0. Los bytes salen con el bit más significativo primero, sin paridad ni suma de control.

El Anexo C da los tiempos exactos, en ciclos y en microsegundos.

## Los comandos de la ACI

Tras `C100R`, la ACI espera una línea con el formato del monitor, terminada en `R` (lectura) o `W` (escritura):

| Línea | Efecto |
|---|---|
| `0300.03FFR` | lee el casete en `$0300-$03FF` |
| `0300.03FFW` | escribe `$0300-$03FF` en el casete, con 10 s de cabecera |
| `004A.00FFR 0300.0FFFR` | lee **dos** grabaciones seguidas: la página cero del BASIC y luego el programa |
| `004A.00FFW800.FFFW` | la forma pegada del manual BASIC de 1976, también aceptada |

{{!C100R}}{{!R (lectura)}}{{!W (escritura)}}

!!! attention "Los ceros de 004A.00FF son obligatorios"
    El analizador de la ROM no borra su acumulador hexadecimal en el punto («Doesn’t clear HEX1!!!», dice el listado). Las cifras del final se añaden detrás de las del principio: `4A.FF` significa `$004A-$4AFF`, ¡casi 19 KB! `0300.0FFF` funciona porque las cuatro cifras empujan fuera el 3 de `0300`. Escriba siempre las dos direcciones con cuatro cifras. El emulador reproduce este comportamiento, puesto que ejecuta la ROM real.

Un programa BASIC se guarda en dos grabaciones: la página cero `$004A-$00FF` (que contiene los punteros LOMEM y HIMEM y el final del programa) y luego el propio programa, que siempre termina en HIMEM. Para volver a cargarlo, el BASIC ya debe estar en memoria; luego se vuelve al BASIC «en caliente» con `E2B3R`, que conserva el programa.

```mon
» C100R
C100: A9*
» 004A.00FFR 0300.0FFFR
» E2B3R

E2B3: 20
» >RUN
```

## Hacia un Apple-1 real

Esto es lo que justifica el emulador: preparar un programa en el PC y reproducirlo en la ACI de una máquina real, desde un smartphone o la salida de auriculares de un ordenador.

<ol class="steps">
<li><strong>Prepare el archivo.</strong> <strong>📼 Exportar WAV</strong> en el editor, o <code>apple1-emu export prog.txt -o prog.aiff</code>. Elija <code>.aiff</code> para un smartphone. Anote la línea que aparece, por ejemplo <code>C100R</code> y luego <code>0300.031ER</code>.</li>
<li><strong>Prepare el reproductor.</strong> Volumen al máximo (acepte el aviso de «volumen alto»), todos los efectos de audio desactivados (ecualizador, Dolby, «mejora de audio»), modo avión, sin Bluetooth. Un reproductor sencillo como VLC sirve.</li>
<li><strong>En el Apple-1</strong>, escriba <code>C100R</code>: el monitor responde <code>C100: A9*</code>.</li>
<li>Escriba la línea de lectura, por ejemplo <code>0300.031ER</code>, pero <strong>no pulse RETURN todavía</strong>.</li>
<li>Inicie la reproducción y pulse <kbd>RETURN</kbd> <strong>durante la cabecera</strong> (dura unos 10 s, tras un segundo de silencio).</li>
<li>Al final, la ACI vuelve al monitor (<code>\</code>). Ejecute el programa: <code>300R</code>, o <code>E2B3R</code> y luego <code>RUN</code> para un programa BASIC.</li>
</ol>

**¿Por qué este perfil AIFF?** Sin comprimir, 16 bits, sin códec con pérdidas: MP3 o AAC redondearían los flancos. 48 kHz, la frecuencia nativa de Android, para evitar cualquier remuestreo. Dos canales estrictamente idénticos, porque un cable que une izquierdo y derecho en FROM TAPE nunca debe recibir dos señales distintas. Una onda cuadrada a −1 dBFS: la entrada de la ACI original es un filtro paso alto (0,01 µF) que apenas ve algo más que los flancos, y una onda cuadrada de flancos abruptos es lo que mejor le conviene. Un segundo de silencio al principio y al final, porque un teléfono a veces corta el comienzo de un sonido.

!!! apple1 "No se fíe del LED"
    El manual de 1976 pide un nivel en el que el LED de la ACI esté «justo encendido». Pero solo se enciende hacia 1,2 V, mucho más de lo que necesita el comparador. Un LED apagado no demuestra por tanto que la señal sea demasiado débil. Si la carga no termina nunca, la ROM sigue esperando flancos: pulse RESET y suba el volumen.

!!! attention "Nunca MP3 ni AAC"
    Un formato con pérdidas desplaza los flancos y hace ilegible el casete. Use WAV, AIFF, o FLAC si su reproductor lo decodifica.

## ¿Qué nivel hace falta? Los modelos de entrada

El comando `info --aci-input` simula la entrada analógica de la tarjeta (condensador de acoplamiento, polarización, histéresis del LM311) para una serie de tensiones de pico, y le dice a partir de qué nivel se leería el archivo:

```sh
apple1-emu info prog.aiff --aci-input stock     # tarjeta de 1976, 0,01 µF
apple1-emu info prog.aiff --aci-input mimeo     # 0,1 µF (consejo de la guía Mimeo)
apple1-emu info prog.aiff --aci-input gen2      # ACI Gen2 de Uncle Bernie
apple1-emu info prog.aiff --aci-input stock --volts 0.5 --series 1200
```

| Modelo | Lo que representa |
|---|---|
| `stock` | la ACI original: 0,01 µF hacia el comparador polarizado a 2,5 V, realimentación 3k/47k |
| `mimeo` | la modificación de Mike Willegal: 0,1 µF, más fiable |
| `gen2` | la ACI Gen2 de Uncle Bernie: 100 nF emparejados, 300k en lugar de 47k (menos histéresis) |

`--volts` fija la tensión de pico en el jack para la escala digital completa; `--series` añade una resistencia en serie (por ejemplo los 1,2 kΩ de un cable para smartphone). Con el modelo `stock`, los archivos del emulador se leen a partir de **unos 0,25 V de pico**; la salida de auriculares de un smartphone al máximo suele dar de 0,5 a 1 V. Este modelo es simplificado: sin ruido ni deriva del punto de polarización.

## Digitalizar un casete real o una grabación del Apple-1

Para recuperar un programa escrito en un Apple-1 real, grabe la salida **TO TAPE** de la ACI por una entrada de micrófono (un teléfono, o la tarjeta de sonido de un PC con Audacity a 44,1 o 48 kHz), con el control automático de ganancia y la reducción de ruido desactivados. En el Apple-1: `C100R`, luego `0300.03FFW` y <kbd>RETURN</kbd>. Pare unos segundos después de que vuelva el `\`, y luego:

```sh
apple1-emu import grabacion.aiff --addr 300 -o prog.txt
apple1-emu import startrek.aiff --range "004A.00FFR 0300.0FFFR" -o startrek.txt
```

!!! apple1 "TO TAPE es una salida de nivel de micrófono"
    Solo entrega unos 50 mV de pico a pico. Compruebe la forma de onda en Audacity: sin recortes ni saltos de nivel durante la cabecera. La importación del emulador tolera cualquier ganancia y un desplazamiento de continua.
