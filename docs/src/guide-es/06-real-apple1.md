# En un Apple-1 real: exportar a WAV

Todo lo que escribe en el emulador puede ir a un Apple-1 real, a una réplica o a un kit equipado con la tarjeta de casete ACI. El archivo de audio sustituye a la cinta; un smartphone o la salida de auriculares de un PC hace de magnetófono.

## Crear el casete

En el editor, haga clic en **📼 Exportar WAV** y elija un nombre de archivo. La extensión decide el formato: `.wav`, o `.aiff` para un teléfono (el formato recomendado, ver más abajo). La parte inferior del editor muestra entonces exactamente lo que hay que escribir en la máquina real:

![Después de la exportación, la parte inferior del editor muestra el comando de carga.](img/es/editeur-export-bas.png "large")

Para `premier.bas`, el comando es `C100R` y luego `004A.00FFR 0F5C.0FFFR`. El programa BASIC ocupa dos grabaciones seguidas en el casete: primero la página cero del BASIC (`$004A` a `$00FF`, que contiene los punteros del programa) y luego el propio programa, guardado en lo alto de la memoria. El botón **Copiar los rangos** pone el comando en el portapapeles.

!!! attention "Siempre cuatro cifras"
    Escriba `004A.00FF`, nunca `4A.FF`. La ROM de la ACI no borra las cifras ya leídas cuando encuentra el punto: `4A.FF` se entendería como `$004A` a `$4AFF`, y la máquina esperaría 19 KB que nunca llegan. El emulador muestra siempre la forma correcta, y reproduce este comportamiento porque ejecuta la ROM real de la ACI.

## Cargar en la máquina real

El BASIC ya debe estar en memoria, cargado desde su propio casete (`C100R` y luego `E000.EFFFR`).

<ol class="steps">
<li><strong>Prepare el reproductor.</strong> Copie el archivo en el teléfono, conecte la salida de auriculares a la entrada «FROM TAPE» de la ACI y ponga el volumen al máximo.</li>
<li><strong>Vuelva al monitor</strong> con RESET y escriba <code>C100R</code>. La ACI responde <code>C100: A9*</code> y empieza una línea nueva; el asterisco es su indicador.</li>
<li><strong>Escriba los rangos</strong> <code>004A.00FFR 0F5C.0FFFR</code>, pero no pulse todavía RETURN.</li>
<li><strong>Inicie la reproducción</strong> y pulse <strong>RETURN durante la cabecera</strong>, el tono constante de unos 10 segundos con el que empieza cada grabación.</li>
<li><strong>Espere a que vuelva el <code>\</code></strong> y escriba <code>E2B3R</code> (arranque en caliente del BASIC, que conserva el programa) y <code>RUN</code>.</li>
</ol>

```mon
» C100R

C100: A9*
» 004A.00FFR 0F5C.0FFFR
\
» E2B3R

E2B3: 20
» >RUN
HOLA, APPLE-1!
```

!!! apple1 "Leer de forma fiable desde un teléfono"
    - **Volumen al máximo.** Acepte el aviso de Android sobre el «volumen alto»; si no, el volumen queda limitado. Muchos teléfonos bajan el volumen cada vez que se vuelve a conectar el jack: compruébelo cada vez.
    - **Sin efectos de sonido**: desactive el ecualizador, Dolby, Mi Sound y cualquier «mejora de audio».
    - **Modo avión** o «No molestar», para que ninguna notificación se mezcle con la señal. Sin Bluetooth.
    - **Un reproductor sencillo** que lea AIFF sin procesarlo; VLC funciona bien.
    - **Nunca MP3 ni AAC**: la compresión deforma los flancos de la señal. Quédese con WAV o AIFF.
    - **No se fíe del LED de la ACI**: solo se enciende hacia 1,2 V, muy por encima de lo que necesita la tarjeta.

## Por qué AIFF a 48 kHz

Los archivos del emulador están pensados para la entrada de la ACI. El AIFF de 16 bits sin comprimir escapa a todos los códecs; 48 kHz es la frecuencia nativa de Android, así que el teléfono no remuestrea. Los dos canales estéreo son estrictamente idénticos, de modo que izquierdo y derecho pueden unirse en FROM TAPE sin que las dos salidas se enfrenten. La señal es una onda cuadrada de flancos abruptos, lo que mejor detecta el comparador de la ACI.

!!! astuce "Comprobar un archivo antes de reproducirlo"
    En la línea de comandos, `apple1-emu info premier.aiff --aci-input stock` simula la entrada de la tarjeta original con varios niveles de tensión y le dice a partir de qué nivel el archivo se lee bien (unos 0,25 V de pico para nuestros archivos).
