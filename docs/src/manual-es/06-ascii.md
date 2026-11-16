# Foto → ASCII

<p class="lead">La herramienta Foto → ASCII convierte una foto en una imagen del Apple-1 de 40 × 24 caracteres, y luego en un programa BASIC o 6502, listo para ejecutarse o exportarse a casete.</p>

![La herramienta Foto → ASCII: la foto y su marco de recorte, los ajustes y la vista previa dibujada con la fuente del 2513.](img/es/brut/ascii-outil.png "large")

## Abrir una imagen

**🎨 ASCII** abre la herramienta, un panel acoplable como los demás. **Arrastre y suelte** una foto `.jpg` o `.png` sobre la ventana, o use **📂 Abrir…** (una foto, o un arte ASCII ya hecho en `.txt`).

El marco verde sobre la foto muestra la parte que se convierte: arrástrelo para moverlo; la rueda del ratón hace zoom.

## Ajustes

| Ajuste | Función |
|---|---|
| **Columnas**, **Filas** | 40 × 24 como máximo. **23 filas** deja la última línea al indicador, para que la imagen no se desplace. |
| **Zoom**, **Centro** | la parte de la foto que se conserva |
| **Brillo**, **Contraste**, **Gamma** | los ajustes habituales (gamma > 1 aclara) |
| **Invertir** | oscuro = mucha tinta |
| **Niveles auto** | estira los niveles de gris (del 2 % al 98 %) antes de los demás ajustes |
| **Tramado** | Floyd-Steinberg: más matices, más ruido |
| **Caracteres** | la rampa **completa** (26 niveles), **Simple** (10), la del **pase de diapositivas de Schmenk**, o **Personalizada** |

Solo son posibles los caracteres del 2513: mayúsculas, cifras y símbolos, sin minúsculas, sin `"` y sin `_`. Se ordenan por el número de puntos encendidos en la fuente del Apple-1: ese número, y no la forma, da el nivel de gris.

!!! astuce "Para un retrato"
    40 × 24 caracteres es poco: recorte sobre la cara (zoom de 1,3 a 1,6), suba un poco el contraste y pruebe un gamma inferior a 1 para una foto clara. Un fondo oscuro y liso da los mejores resultados.

## El programa producido

- **BASIC**: una línea `PRINT "…"` por línea del dibujo. Una línea completa de 40 caracteres termina en `;`, porque la pantalla pasa sola a la línea siguiente. El programa tarda en escribirse (unos 2 s por línea).
- **6502**: una fuente ca65 en `$0300`, iniciada con `300R`. Tras borrar la pantalla, un bucle envía a ECHO una tabla comprimida (series de caracteres idénticos codificadas como «número + carácter»). La visualización es instantánea.

Con 24 filas, el programa espera una tecla antes de devolver el control; con 23, el indicador llega a la última línea y la imagen queda entera.

Los botones: **✎ Insertar en el editor** (pestaña BASIC o 6502 nueva), **▶ Ejecutar**, **📼 Exportar WAV…**, **💾 Guardar en la biblioteca** (un `.txt` en la carpeta «Art ASCII»), **📋 Copiar el texto**, **Ver el código**.

![De la foto al Apple-1 real: conversión, programa, casete.](diagrams/es/workflow-ascii.svg "diagram large")

## En la línea de comandos

```sh
apple1-emu ascii foto.jpg                        # solo el dibujo, en pantalla
apple1-emu ascii woz.jpg --zoom 1.3 --cx 0.45 --cy 0.32 \
          --contrast 1.3 --gamma 0.45 --rows 23 -o woz.txt
apple1-emu ascii woz.jpg -o woz.bas              # programa BASIC
apple1-emu ascii woz.jpg -o woz.s                # fuente 6502
apple1-emu ascii woz.jpg -o woz.wav              # casete 6502, iniciado con 300R
apple1-emu ascii woz.jpg -o woz.wav --format basic
```

El capítulo 12 da todas las opciones.

!!! apple1 "El pase de diapositivas de Dave Schmenk"
    En 2014, Dave Schmenk escribió un pase de diapositivas de retratos para el Apple-1. La biblioteca lo añade como proyecto de GitHub (`dschmenk/Apple1-Slideshow`): se descarga, no se incluye, porque el repositorio no tiene licencia. **▶ Probar la última compilación** lo ensambla (sintaxis de xa, traducida automáticamente; los mismos 3456 bytes que xa 2.4.1) y lo ejecuta con `280R`.
