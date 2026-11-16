# Instalación en Linux

## Descomprimir y ejecutar

<ol class="steps">
<li><strong>Descomprima el archivo.</strong> Clic derecho en <code>apple1-emu.zip</code> y luego «Extraer aquí». Aparece una carpeta <code>apple1-emu</code>.</li>
<li><strong>Haga doble clic en <code>run.sh</code>.</strong> Si el gestor de archivos pregunta qué hacer con el archivo (por ejemplo en Linux Mint), elija <strong>«Ejecutar»</strong> (no «Mostrar»). Se abre la ventana del Apple-1, sin terminal.</li>
<li><strong>Opcional: añadir el emulador al menú.</strong> Abra un terminal en la carpeta (clic derecho → «Abrir en un terminal») y escriba el comando siguiente. Aparece una entrada <strong>Retro Dev Kit Apple 1</strong> en la sección <em>Juegos</em> del menú, junto con los comandos <code>apple1-emu-gui</code> (ventana) y <code>apple1-emu</code> (terminal).</li>
</ol>

```sh
./install.sh             # menú de aplicaciones + comandos en ~/.local/bin
./install.sh --bureau    # lo mismo, más un icono en el escritorio
./install.sh --desinstaller
```

La desinstalación (`--desinstaller`) quita los programas y la entrada del menú, pero conserva sus programas (`~/apple1/programmes`) y sus ajustes (`~/.config/apple1-emu`). Ningún comando necesita `sudo`: todo se instala en su carpeta personal. El primer arranque de una versión nueva propone volver a los ajustes de origen; la biblioteca se conserva. El botón **Restablecer las preferencias** hace lo mismo. Un archivo vacío llamado `portable` junto al programa (o `APPLE1_PORTABLE=1`) guarda las preferencias, la biblioteca por defecto y la caché dentro de la carpeta del kit.

!!! astuce "Actualizar desde una versión anterior"
    Los scripts `lancer.sh` e `installer.sh` de las versiones anteriores se llaman ahora `run.sh` e `install.sh`. Vuelva a ejecutar `./install.sh`: sustituye la instalación antigua, así como la entrada del menú y el icono del escritorio que apuntaban a `lancer.sh`.

!!! astuce "La versión de terminal"
    `./run.sh --terminal` abre el emulador en un terminal (al menos 42 columnas × 28 líneas), sin interfaz gráfica. Práctico a través de una conexión SSH. El Manual completo describe sus teclas y comandos.

## Si la ventana no se abre

`run.sh` detecta un fallo inmediato (controlador gráfico, máquina virtual sin aceleración) y vuelve a intentarlo por sí solo con el renderizado por software de Mesa (`LIBGL_ALWAYS_SOFTWARE=1`). Si también falla, aparece un mensaje y el registro queda en `~/.cache/apple1-emu/gui.log`.

| Síntoma | Qué hacer |
|---|---|
| El gestor de archivos abre `run.sh` en el editor de texto | Clic derecho → Propiedades → Permisos: marque «Permitir ejecutar el archivo como un programa», vuelva a hacer doble clic y elija «Ejecutar». |
| «Permiso denegado» en el terminal | `chmod +x run.sh install.sh bin/*/apple1-*` |
| Ventana negra o muy lenta | Normal en una máquina virtual: el renderizado por software sigue siendo fluido para la pantalla del Apple-1. Desactive la «Pantalla vintage» si hace falta. |
| Sin sonido del casete | El sonido pasa por ALSA; sin tarjeta de sonido simplemente queda mudo. La carga funciona igual. |

## Dónde se guardan sus archivos

| Ubicación | Contenido |
|---|---|
| `~/apple1/programmes` | La **biblioteca**: sus casetes, programas y ejemplos (creada en el primer arranque). |
| `~/.config/apple1-emu/gui.json` | Ajustes: zoom, colores, disposición de los paneles, pestañas del editor. |
| `~/.cache/apple1-emu/` | Casetes convertidos, proyectos de GitHub descargados, el registro `gui.log`. |
