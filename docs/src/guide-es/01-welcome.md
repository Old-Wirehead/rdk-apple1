---
version: 1.0
date: noviembre de 2026
---

# Bienvenido

<p class="lead"><strong>Retro Dev Kit Apple 1</strong> — «El pecado original» — devuelve a la vida el Apple-1 de Steve Wozniak (1976) en un PC con Linux: el procesador 6502, la pantalla de 40 × 24 caracteres, el Woz Monitor, la interfaz de casete ACI y un BASIC integrado compatible con Integer BASIC (RDK G2 Basic). Sobre todo, crea casetes de audio que usted puede reproducir en un Apple-1 real.</p>

En una media hora, esta guía le lleva del archivo descargado a un programa BASIC propio, cargado en una máquina real, con una captura de pantalla en cada paso. El **Manual completo** describe cada panel en detalle, con guías completas del Integer BASIC y del ensamblador 6502.

## Lo que necesita

<div class="cards">
<div class="card"><h4>Un PC con Linux</h4>Una distribución Linux reciente, de 64 bits (x86_64) o de 32 bits (i686). Nada que instalar: el emulador usa las bibliotecas gráficas que ya están (X11 o Wayland, OpenGL/Mesa).</div>
<div class="card"><h4>El archivo <code>apple1-emu.zip</code></h4>Los dos programas (ventana y terminal), ejemplos, esta guía y el Manual completo en PDF.</div>
<div class="card"><h4>El BASIC está incluido</h4><strong>RDK G2 Basic</strong>, un BASIC libre compatible con Integer BASIC, viene integrado. El original de Apple de 1976 no está incluido (sigue protegido por derechos de autor): si lo quiere, use su propia copia, como el casete <code>BASIC.wav</code> de la colección «Apple1Cassettes» de archive.org.</div>
<div class="card"><h4>Opcional: un Apple-1 real</h4>Con una tarjeta de casete ACI: original, réplica o kit. Un smartphone o la salida de auriculares de un PC hace de magnetófono.</div>
</div>

## Cómo leer esta guía

Los comandos que se escriben en el Apple-1 aparecen `ASÍ`. Las sesiones se muestran sobre fondo negro, como la pantalla de la época; lo que usted escribe va en blanco y la respuesta de la máquina en verde:

```mon
» 300R
0300: A9A
```

A lo largo del camino aparecen tres tipos de recuadros:

!!! astuce "Consejo"
    Un atajo, o una forma más rápida de obtener el mismo resultado.

!!! attention "Atención"
    Una trampa frecuente, o una acción que puede hacerle perder su trabajo.

!!! apple1 "En el Apple-1 real"
    Lo que cambia al pasar del emulador a la máquina real.
