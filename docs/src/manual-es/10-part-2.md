# Integer BASIC <span class="part-sub">El BASIC de Steve Wozniak y su clon libre RDK G2 Basic: todas las instrucciones, la memoria, los mensajes de error, los fallos conocidos y cómo sortearlos, y la adaptación desde Applesoft</span> {: .part data-part="Parte II" }

# Primeros pasos en BASIC

<p class="lead">El Integer BASIC del Apple-1 cabe en 4 KB. Solo conoce los enteros y los nombres de variable de una letra, y no tiene ni DATA ni funciones de cadena: es pequeño, rápido y tiene sus trampas.</p>

!!! apple1 "De dónde viene este BASIC"
    Steve Wozniak lo escribió en 1976, a mano, en ensamblador, sin un ordenador para ensamblarlo. Se vendía en casete (75 $) y se carga en `$E000-$EFFF`. El emulador no incluye el original de Apple: use su propio `BASIC.wav` o imagen de 4 KB (capítulo 4) si lo quiere. Todas las pruebas de esta parte se hicieron en el emulador, con ese original.

## Dos BASIC: RDK G2 Basic y el original

Desde la versión 7.9 el emulador tiene su propio BASIC, **RDK G2 Basic**, integrado en el programa. Es un clon en sala limpia del Integer BASIC, escrito desde cero en ensamblador 6502 (no contiene código del intérprete de Apple) y distribuido bajo licencia MIT. Está dedicado a Uncle Bernie.

- **Compatible**: misma sintaxis, mismos tokens, mismo formato de programa en memoria, mismo formato de casete. Los programas, listados y casetes escritos para el original funcionan sin cambios. Se ha comprobado frente al original con 22 sesiones de prueba de salida idéntica.
- **Corregido**: los fallos conocidos del original, descritos en el capítulo 18, están corregidos.
- **Más grande**: 7933 bytes en lugar de 4 KB. Necesita RAM de `$E000` a `$FEFF`; el emulador la proporciona, pero no cabría en la ventana de EPROM de 4 KB del original.
- **Conoce la GEN2**: al arrancar busca la tarjeta GEN2 y, si la encuentra, inicia el controlador de la tarjeta (y entonces pone LOMEM en `$2000`, por encima de las páginas gráficas). Las instrucciones gráficas propiamente dichas están previstas para la versión 2 de G2 Basic.
- **Saludo**: `RDK G2 BASIC 1.0`, y luego `DEDICATED TO UNCLE BERNIE`.
- **Ampliado** (versión 7.9.2): `LOMEM=` y `HIMEM=` funcionan como en el original, y una nueva función **`KEY(n)`** lee el teclado sin esperar, para que un juego pueda mirar las teclas sin que `INPUT` lo bloquee y sin que una pulsación detenga el programa (capítulo 15). `LIST a,b` y `DEL a,b` usan los rangos del original, y una línea listada más larga que la pantalla se parte como en el original.

El botón **BASIC** carga G2 Basic. Para usar en su lugar el original de Apple, abra **⚙ Preferencias → Avanzado → BASIC**, elija **Integer BASIC original (su propio archivo, cargado en $E000)** y el archivo; la elección se recuerda y el botón lo carga desde entonces. Los dos se comportan igual en todo lo que describe esta parte; las diferencias se indican donde importan (HIMEM al arrancar, los fallos del capítulo 18).

!!! note "Fuente y pruebas"
    G2 Basic tiene su propio repositorio, `Old-Wirehead/rdk-gen2basic-apple1`, con su manual, su fuente y sus pruebas de regresión.

## Arrancar el BASIC

Haga clic en **BASIC** en la barra de herramientas: el emulador carga el BASIC elegido (G2 Basic por defecto) y usted lo arranca con `E000R`. El indicador del BASIC es `>`, el del Woz Monitor `\`.

```mon
\
» E000R
E000: 4C
>
```

- `E000R`: **arranque en frío**, borra el programa en memoria.
- `E2B3R`: **arranque en caliente**, conserva el programa. Es la dirección que hay que usar después de cargar un programa desde casete, y para volver al BASIC tras un paso por el monitor (RESET).

{{E000R}}{{E2B3R}}

## Modo directo y modo programa

Una línea que empieza por un **número** (0 a 32767) se guarda en el programa; una línea sin número se ejecuta enseguida.

```mon
>» PRINT 2*3+4*5
26
>» 10 PRINT "HOLA"
>» 20 GOTO 10
>» RUN
HOLA
HOLA
…
```

Pulse una tecla para detener el programa: el BASIC muestra `STOPPED AT 10`. Volver a escribir una línea con el mismo número la sustituye; un número solo la borra.

!!! attention "El modo directo es limitado"
    En modo directo, el BASIC rechaza las instrucciones separadas por `:` (`SYNTAX ERR`), así como `FOR` y `GOSUB`. Para probar un bucle, escríbalo en una línea numerada.

## Corregir una línea

El teclado del Apple-1 no tiene tecla de borrado: el **Retroceso** del emulador escribe `_`, que anula el carácter anterior (la línea en pantalla no se corrige, pero el BASIC recibe la correcta). <kbd>Esc</kbd> anula la línea entera: el BASIC muestra `\` y espera la línea siguiente. Para editar con más comodidad, use el editor (capítulo 7): colorea la sintaxis, señala los errores antes de enviar y escribe el programa por usted.

## Un primer programa

@@include exemples/es/basic/boucle.bas

```mon
>» RUN
SUMA = 55
>
```

En el editor, **▶ Ejecutar** escribe el programa y `RUN`; en el depurador puede seguirlo línea a línea (figura en el capítulo 8).

## Comandos

| Comando | Efecto |
|---|---|
| {{LIST}} `[n[,m]]` | lista el programa, la línea `n`, o las líneas `n` a `m` |
| {{RUN}} `[n]` | borra las variables y ejecuta el programa, desde la línea `n` si se indica |
| {{DEL}} `n[,m]` | borra la línea `n`, o las líneas `n` a `m` |
| {{SCR}} | borra el programa y las variables |
| {{CLR}} | borra las variables, conserva el programa |
| {{AUTO}} `n[,paso]` | numeración automática desde `n` (paso 10 por defecto); <kbd>Ctrl</kbd>+<kbd>D</kbd> y luego <kbd>RETURN</kbd> la termina, <kbd>Esc</kbd> no |
| {{MAN}} | vuelve a la numeración manual |
| {{HIMEM}}`=n`, {{LOMEM}}`=n` | los límites de la memoria del BASIC (capítulo 16) |

`GOTO n` en modo directo reanuda un programa detenido **sin** borrar las variables, a diferencia de `RUN n`.

!!! astuce "LIST y la visualización lenta"
    El terminal del Apple-1 muestra unos 60 caracteres por segundo: listar un programa largo lleva tiempo. Desactive el límite de visualización (**Rápido** en la fila de visualización, o en la barra de estado) para leer más deprisa, o liste por tramos: `LIST 100,200`.

## Tres programas para teclear

### Adivinar un número

@@include exemples/es/basic/devine.bas

```mon
>» RUN
HE ELEGIDO UN NUMERO DEL 1 AL 100.
TU APUESTA?» 50
MAS ALTO!
TU APUESTA?» 75
MAS BAJO!
TU APUESTA?» 62
MAS BAJO!
TU APUESTA?» 56
MAS ALTO!
TU APUESTA?» 57
BIEN, EN 5 INTENTOS.
```

`RND(100)` da un entero de 0 a 99; `#` significa «distinto de».

### Una tabla de multiplicar

@@include exemples/es/basic/table.bas

### Jugar con las cadenas

@@include exemples/es/basic/chaines.bas

```mon
>» RUN
TU NOMBRE?» RADAR
INICIAL: R
FINAL: AR
AL REVES: RADAR
ES UN PALINDROMO!
```

Las cadenas se cortan con `N$(inicio,fin)` y se alargan con `R$(LEN(R$)+1)=…`: no hay ni `MID$` ni `+` para las cadenas (capítulo 16).

Los cuatro programas están en `docs/exemples/es/basic/`: ábralos en el editor, o haga un casete con `apple1-emu basic devine.bas -o devine.aiff`.
