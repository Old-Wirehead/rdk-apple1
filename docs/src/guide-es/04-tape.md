# Cargar y ejecutar un casete

En el Apple-1, todo pasa por el casete: el propio Integer BASIC se cargaba desde una cinta de audio. El emulador lee los archivos `.wav` y `.aiff` como lo hace la tarjeta ACI original, byte a byte, y reconoce los 24 casetes de la colección «Apple1Cassettes» de archive.org.

## Preparar la biblioteca

Copie sus casetes en la carpeta de la biblioteca, `~/apple1/programmes`: por ejemplo `Hamurabi.wav`. La lista de la derecha se actualiza sola. Las cinco carpetas BASIC, 6502, GEN2 COLOR, WAV y Art ASCII aparecen allí directamente, sin carpeta raíz por encima; los archivos sueltos en la carpeta de la biblioteca no se muestran. Cada casete reconocido lleva la marca **✔ reconocido**, con sus rangos de direcciones y su comando de ejecución ya ajustados.

!!! note "No necesita BASIC.wav"
    El emulador tiene su propio BASIC, RDK G2 Basic, así que los programas BASIC funcionan sin ningún casete. El Integer BASIC de Apple de 1976 sigue protegido por derechos de autor y no está incluido: si prefiere el original, use su propia copia (como `BASIC.wav`) y elíjala en **⚙ Preferencias → Avanzado → BASIC**.

## La vía más rápida: el botón ▶

Cada programa de la lista tiene cuatro pequeños iconos a su derecha: **▶** carga y ejecuta, **⬇** carga en memoria sin ejecutar, **📼** carga por el casete, **⚙** abre sus ajustes. Un doble clic hace lo mismo que ▶.

Haga clic en **▶** junto a `Hamurabi.wav`. El emulador carga el BASIC si aún no está en memoria, luego el programa, y escribe por usted `E2B3R` (vuelta al BASIC sin borrar el programa) y `RUN`. La carga es instantánea.

![Hamurabi lanzado con el botón ▶ de la biblioteca.](img/es/hamurabi-ecran.png "small")

Para un programa en lenguaje máquina, ▶ hace un RESET y luego escribe su comando de ejecución, por ejemplo `300R`. Un listado BASIC o una fuente 6502, ▶ los abre además en una pestaña del editor, si el editor está abierto.

## Como en 1976: el casete virtual

La carga es instantánea salvo que pida el casete. Haga clic en el icono **📼** de un programa para reproducir una carga real: el emulador escribe `C100R` para iniciar el programa de la ACI, luego el rango de direcciones, y el casete se reproduce a velocidad real. Aparece una barra de progreso bajo el nombre del programa, y la franja **Casete:** de la barra de herramientas muestra el casete y su posición.

![Carga de BASIC.wav por la ACI virtual, con el osciloscopio de casete abierto.](img/es/brut/cassette-basic.png "large")

Los botones 📈 y 🔊 al final de la franja **Casete:** abren el **osciloscopio** y le permiten **oír** la señal, como el altavoz de un magnetófono. Cargar el BASIC lleva unos 36 segundos a velocidad real.

Cuando el casete se detiene, el monitor vuelve a mostrar `\`. Escriba `E000R` para iniciar el BASIC. Su indicador es un `>`:

```mon
» E000R

E000: 4C
» >PRINT 6*7
42
```

!!! astuce "Cargar a mano"
    **📼 Insertar**, en la franja **Casete:** de la barra de herramientas, pone cualquier archivo `.wav` o `.aiff` en el magnetófono virtual. Luego escribe usted mismo `C100R` y después el rango, por ejemplo `E000.EFFFR` para el BASIC: el casete arranca en la primera lectura, como en la máquina real. El botón **Cargar** sirve para los archivos `.txt`, `.bin` y `.hex`, cargados al instante.

| Programa | Después de cargar |
|---|---|
| Integer BASIC | `E000R` (arranque en frío, borra el programa en memoria) |
| Programa BASIC | `E2B3R` (arranque en caliente, conserva el programa) y luego `RUN` |
| Lenguaje máquina | su dirección seguida de `R`, a menudo `300R` |
