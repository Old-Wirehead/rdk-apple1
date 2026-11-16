# Primer arranque

En el primer arranque la ventana se abre maximizada, y una ventana **Su espacio de trabajo** propone cinco disposiciones: **Predeterminada**, **Juego**, **BASIC**, **Ensamblador** y **Depuración**. Cada miniatura numerada muestra la disposición, dice para qué sirve e indica el tamaño mínimo de pantalla que necesita: haga clic en la que desee. Desmarque **Mostrar esta ventana al iniciar** si prefiere no volver a verla; **⊞ Disposición ▾ → Elegir un espacio de trabajo…** la vuelve a abrir. Puede cambiar la disposición en cualquier momento con el menú **⊞ Disposición ▾**. La disposición predeterminada muestra todas las herramientas de un vistazo. Cada zona es un panel acoplable que puede mover, agrupar en pestañas o separar en su propia ventana. La pantalla en color (GEN2 / P-LAB) es el rectángulo número 7, en las miniaturas que la muestran y en la leyenda; los números 1 a 6 no cambian. Esta ventana no se queda encima de las demás. Si la ventana es estrecha, la barra de herramientas se desplaza: el botón **»** trae el grupo Casete.

![La disposición predeterminada. La pantalla ya muestra el pequeño programa de la sección siguiente.](img/es/defaut-annote.png "large")

<ol class="legend">
<li><strong>Cargar, Guardar, Estado</strong>: volcados hexadecimales <code>.txt</code>, binarios <code>.bin</code> y <code>.prg</code>; Guardar también escribe casetes <code>.wav</code>/<code>.aiff</code>; Estado guarda y restaura la máquina.</li>
<li><strong>RESET, Borrar pantalla, Velocidad (indicador), Pegar</strong>: los botones de la máquina (Ctrl+R, Ctrl+L, F3, Ctrl+V).</li>
<li><strong>BASIC</strong>: carga el RDK G2 Basic integrado en <code>$E000</code>.</li>
<li><strong>Ventanas</strong>: el menú <strong>⊞ Disposición ▾</strong> y luego un botón por panel (pantallas, Editor, Depurador, Desensamblado, Volcado, Biblioteca, ASCII).</li>
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

!!! astuce "Hacer sitio"
    Para solo jugar, cierre el editor y el depurador con sus botones del grupo Ventanas (4): la pantalla crece. En la lista de zoom (8), **Ajustar** elige el mayor zoom entero que cabe, **F11** pasa a pantalla completa, **⊞ Disposición ▾ → Predeterminada** (4) lo restablece todo; el mismo menú ofrece también una disposición **Juego** (pantalla del Apple-1 y biblioteca). Si los botones son demasiado pequeños o grandes para su pantalla, cambie **⚙ Preferencias → Interfaz → Tamaño de iconos y texto**.

!!! note "Acceso a la red en el primer arranque"
    La primera vez que se inicia, el emulador contacta con GitHub (`api.github.com`, `raw.githubusercontent.com`) para descargar el proyecto de demostración (`dschmenk/Apple1-Slideshow`, que figura en «Proyectos GitHub» de **⚙ Preferencias**), y muestra el progreso en la barra de estado. No se envía nada más. Sin conexión, simplemente se lo salta; quite el proyecto de esa lista para detener la descarga para siempre.

## El Woz Monitor

La barra invertida `\` es el indicador del **Woz Monitor**, el programa de 256 bytes que arranca el Apple-1. Haga clic en la pantalla y escriba: las minúsculas se convierten en mayúsculas, **Intro** es RETURN y **Retroceso** da el `_` que borra el último carácter, como en 1976.

El monitor entiende cuatro tipos de comandos:

| Usted escribe | Efecto |
|---|---|
| `300` | muestra el byte de la dirección `$0300` |
| `300.30F` | muestra los bytes de `$0300` a `$030F` |
| `300: A9 C1` | escribe `A9` en `$0300` y luego `C1` en `$0301` |
| `300R` | ejecuta el programa que empieza en `$0300` (*Run*) |

## Su primer programa en lenguaje máquina

Escriba estas dos líneas, cada una seguida de Intro:

```mon
» 300: A9 C1 20 EF FF 4C 1F FF

0300: 00
» 300R

0300: A9A
```

La `A` final la ha escrito su programa. Los ocho bytes son tres instrucciones del 6502:

| Bytes | Instrucción | Función |
|---|---|---|
| `A9 C1` | `LDA #$C1` | carga el código de la letra «A» (con el bit 7 activado, como espera el Apple-1) |
| `20 EF FF` | `JSR $FFEF` | llama a ECHO, la rutina del monitor que muestra un carácter |
| `4C 1F FF` | `JMP $FF1F` | vuelve al monitor, que espera el comando siguiente |

El monitor muestra primero el byte de la dirección de inicio (`0300: A9`) y luego su programa escribe la `A`. El ensamblador integrado (Manual completo) le ahorra calcular estos bytes a mano.

!!! apple1 "En el Apple-1 real"
    Los comandos son idénticos: el emulador ejecuta la ROM real del Woz Monitor. La máquina real muestra unos 60 caracteres por segundo. El emulador mantiene ese ritmo cuando la fila de visualización de la pantalla está en **Normal**; **Rápido** lo muestra todo de golpe. El ajuste es común a la pantalla del Apple-1 y a la pantalla GEN2.
