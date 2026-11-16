# ¿Y ahora qué? {: .newpage }

Ya sabe cargar un casete, escribir y ejecutar un programa BASIC y transferirlo a una máquina real. El **Manual completo** (en inglés: `docs/Complete-Manual.pdf`) va mucho más lejos:

- **Guía del Integer BASIC**: cada instrucción con ejemplos, los límites, los mensajes de error, los fallos conocidos y cómo esquivarlos, la adaptación de programas Applesoft.
- **Guía del ensamblador 6502**: el ensamblador integrado (sintaxis de ca65, macros, listado), las rutinas del Woz Monitor, ejemplos probados en el emulador.
- **Juegos para jugar y leer**: la biblioteca trae cinco pequeños juegos de Claude IA (Ahorcado, Cerillas, Batalla naval en BASIC; Túnel y Tres en raya en ensamblador), cada uno en francés e inglés.
- **Las herramientas**: depurador, biblioteca y proyectos de GitHub, tarjeta de color GEN2 y tarjeta gráfica P-LAB, foto → ASCII, pantalla vintage, línea de comandos.

## Atajos esenciales

| Tecla | Efecto |
|---|---|
| Ctrl+R | RESET (vuelta al monitor) |
| Ctrl+L | CLEAR SCREEN |
| F3 | velocidad real o velocidad máxima |
| Ctrl+V | escribe el portapapeles en el teclado del Apple-1 (también: clic derecho en la pantalla → Pegar) |
| Ctrl+más, Ctrl+menos, Ctrl+0 | zoom de la pantalla y zoom ajustado a la ventana (en el editor: tamaño del texto) |
| F11 | pantalla completa |
| F1 | burbujas de ayuda detalladas (como el botón ?) |
| Retroceso | `_`, el borrado del monitor |
| Ctrl+F, Ctrl+H, Ctrl+G | en el editor: buscar, reemplazar, ir a una línea |
| F5, F9 | en el editor: ejecutar la pestaña, punto de interrupción en la línea |

## Comandos para recordar

| Comando | Efecto |
|---|---|
| `E000R` | inicia el Integer BASIC (en frío: borra el programa) |
| `E2B3R` | vuelve al BASIC sin borrar el programa |
| `C100R` | inicia el programa de la ACI (indicador `*`) |
| `inicio.finR` | lee el casete en ese rango, por ejemplo `0300.031ER` |
| `300R` | ejecuta un programa en lenguaje máquina en `$0300` |
