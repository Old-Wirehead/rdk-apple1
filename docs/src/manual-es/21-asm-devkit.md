# El ensamblador del devkit

<p class="lead">El emulador contiene un ensamblador 6502 completo de dos pasadas, compatible con ca65. Se usa en el editor, que ensambla a cada pulsación, en la línea de comandos y para los proyectos de GitHub. No hay nada que instalar.</p>

## El ciclo de trabajo

![El ciclo de trabajo en ensamblador: escribir, ensamblar, ejecutar, depurar y luego exportar al Apple-1 real.](diagrams/es/workflow-asm.svg "diagram large")

<ol class="steps">
<li><strong>Escribir</strong>: <strong>📝 Editor</strong>, luego <strong>🗋 Nuevo</strong> o <strong>📂 Abrir</strong> un archivo <code>.s</code>, <code>.asm</code> o <code>.a65</code>. El coloreado de sintaxis distingue instrucciones, directivas, etiquetas, números y comentarios.</li>
<li><strong>Ensamblar</strong>: no hay nada que hacer, el ensamblado sigue a cada cambio. Al pie del panel aparecen el tamaño, las direcciones y la línea que hay que escribir en la máquina real; los errores se muestran con su línea y su columna.</li>
<li><strong>Ejecutar</strong>: <strong>▶ Ejecutar</strong> pone los bytes en memoria en la dirección de <code>.org</code> y escribe el comando del Woz Monitor (<code>300R</code>).</li>
<li><strong>Depurar</strong>: <strong>⏺ Punto de parada</strong> en la línea del cursor, y luego paso a paso en el depurador (capítulo 8), con sus etiquetas en el desensamblado.</li>
<li><strong>Exportar</strong>: <strong>📼 Exportar WAV</strong> escribe el casete; en el Apple-1 real, escriba <code>C100R</code>, el rango indicado (<code>0300.031FR</code>) y luego <code>300R</code>.</li>
</ol>

![Una fuente en el editor: a la izquierda el texto coloreado, abajo el resultado del ensamblado; a la derecha el Apple-1 ha ejecutado el programa.](img/es/brut/editeur-asm.png "large")

## El listado y los símbolos

**📃 Listado** muestra, para cada línea, la dirección, los bytes producidos y la fuente; las líneas producidas por una macro llevan la marca `+`, los archivos incluidos se despliegan y a continuación viene la tabla de símbolos. Un clic en una línea lleva de vuelta a la fuente; un clic en el margen pone un punto de parada. Este es el listado de `hello.s` (capítulo 23), producido por la línea de comandos:

```text title="hello.lst"
0300  A2 00       start:  ldx #0
0302  BD 10 03    @next:  lda message,x   ; carácter siguiente
0305  F0 06               beq @done       ; 0 = fin de la cadena
0307  20 EF FF            jsr ECHO
030A  E8                  inx
030B  D0 F5               bne @next
030D  4C 1F FF    @done:  jmp GETLINE

                  message:
0310  8D                  .byte CR
0311  48 4F 4C 41         .asciiz "HOLA, APPLE-1!"
0315  2C 20 41 50
…
; ---- Symboles (13)
; ECHO        $FFEF
; GETLINE     $FF1F
; message     $0310
; start       $0300
; start@done  $030D
; start@next  $0302
```

Las etiquetas locales `@next` y `@done` pertenecen a `start`: la tabla las llama `start@next` y `start@done`, y el depurador acepta estos nombres como direcciones de punto de parada. (El listado escrito por la línea de comandos conserva su encabezado francés, *Symboles*.)

## La línea de comandos

{{asm (línea de comandos)|`apple1-emu asm`}} hace lo mismo sin ventana:

```sh
apple1-emu asm hello.s                    # solo comprobar
apple1-emu asm hello.s -l hello.lst       # con el listado
apple1-emu asm hello.s -o hello.wav       # casete (.wav o .aif)
apple1-emu asm hello.s -o hello.bin       # binario en bruto
apple1-emu asm hello.s -o hello.hex       # volcado Woz, para pegar en el monitor
apple1-emu asm jeu.s -I ../common         # otra carpeta para .include
```

Cada comando resume el ensamblado y da la línea para el Apple-1 real. El resumen se muestra en francés («32 bytes … 2 archivo(s), 13 símbolos / Casete (tras C100R): 0300.031FR y luego 300R / escrito: hello.wav»):

```text
32 octets, $0300-$031F ; 2 fichier(s), 13 symboles
Cassette (après C100R) : 0300.031FR   puis 300R
écrit : hello.wav
```

Sin `.org`, el programa se coloca en `$0300`. El volcado `.hex` tiene la forma que el Woz Monitor acepta desde el teclado, y termina con el comando de ejecución:

```text title="t1.hex (extracto)"
0280: 34 12 34 12 10 05 FF 34
0288: 12 FF FF A8 02 EA EA EA
…
0280R
```

## Sintaxis

Una línea contiene, en este orden y todo opcional: una etiqueta, una instrucción o una directiva, y un comentario tras `;`. Mnemónicos y directivas pueden escribirse en minúsculas o mayúsculas; los símbolos distinguen mayúsculas y minúsculas.

```asm
PTR     = $06           ; una constante (no produce ningún byte)
start:  lda #0          ; una etiqueta global, seguida de dos puntos
@loop:  inx             ; una etiqueta local: visible hasta la siguiente global
        bne @loop
:       dex             ; una etiqueta anónima
        bne :-          ; :- la anterior, :+ la siguiente, :-- la de antes
```

{{!etiquetas locales}}{{!etiquetas anónimas}}

**Números**: `255` en decimal, `$FF` en hexadecimal, `%1010` en binario, `'A'` para un carácter ($41). El teclado devuelve sus códigos con el **bit 7 a uno**: para comparar una tecla, escriba `'A' | $80`; `apple1.inc` define igualmente `CR = $8D`. `*` representa la dirección actual.

**Operadores**, de menor a mayor prioridad:

| Prioridad | Operadores | Significado |
|---|---|---|
| 1 | <code>&#124;&#124;</code> | o lógico (resultado 0 o 1) |
| 2 | `&&` | y lógico |
| 3 | `=` `<>` `<` `>` `<=` `>=` | comparaciones (0 o 1) |
| 4 | `+` `-` <code>&#124;</code> | suma, resta, o bit a bit |
| 5 | `*` `/` `&` `^` `<<` `>>` | multiplicación, división, y, o exclusivo, desplazamientos |
| 6 (unarios) | `<` `>` `-` `~` `!` | byte bajo, byte alto, negación, complemento, no lógico |

Así, `lda #<message` carga el byte bajo de la dirección y `lda #>message` el byte alto; `#>S+1` es `(>S)+1`. No hay operador módulo (`%` o `.mod` dan un error de sintaxis).

**Página cero o absoluto.** Una dirección por debajo de `$100` conocida en el momento del ensamblado se codifica en página cero (`lda $0006` da `A5 06`, como con ca65). Para forzar la forma absoluta, escriba `a:` (`lda a:$06` da `AD 06 00`); para forzar la página cero, `z:`.

!!! attention "Defina las constantes de página cero antes de usarlas"
    Si `PTR = $06` está escrito **después** de `lda PTR`, el ensamblador aún no conoce el valor y codifica la dirección en 2 bytes. Lo señala en amarillo: `aviso: dirección de página cero codificada en 2 bytes (símbolo definido más abajo; defínalo antes, o escriba z: o a:)`. El programa funciona, pero es más largo, y los modos `(zp),Y` se vuelven imposibles.

## Directivas

| Directiva | Efecto | Ejemplo |
|---|---|---|
| `.org dir` | dirección de ensamblado (y de carga) | `.org $0300` |
| `.byte` (`.byt`) | bytes, cadenas incluidas | `.byte CR, "OK", 0` |
| `.word` (`.addr`) | palabras de 16 bits, byte bajo primero | `.word 10000, end` |
| `.asciiz` | cadena terminada en cero | `.asciiz "HOLA"` |
| `.res n[, val]` | reserva n bytes (0, o val) | `.res 3, $EA` |
| `.include "a"` | inserta un archivo | `.include "apple1.inc"` |
| `.macro nombre args` … `.endmacro` | define una macro (`.mac`, `.endmac`) | vea `macros.s` |
| `.local a, b` | etiquetas propias de cada llamada a la macro | |
| `.repeat n[, var]` … `.endrep` | repite un bloque, var = 0 … n−1 | `.repeat 16, I` |
| `.if` `.elseif` `.else` `.endif` | ensamblado condicional | `.if WIDTH > 30` |
| `.ifdef` `.ifndef` | según exista o no un símbolo | `.ifdef APPLE1` |
| `.ifblank` `.ifnblank` | argumento de macro vacío o no | |
| `.defined(sím)` | 1 si el símbolo existe (en una expresión) | `.if .defined(d)` |
| `.define nombre valor` | sustitución textual (`.undefine`) | `.define CHARACTER '*'` |
| `.setcpu "65C02"` | acepta las instrucciones del 65C02 (`.pc02`, `.p02`) | |
| `.segment` `.code` `.data` `.rodata` `.bss` | aceptadas por compatibilidad | |
| `.feature string_escapes` | `\n`, `\"`… en las cadenas | |

{{!.org}}{{!.byte}}{{!.word}}{{!.asciiz}}{{!.res}}{{!.include}}{{!.macro}}{{!.repeat}}{{!.if}}{{!.setcpu}}

!!! attention "Los segmentos no mueven nada"
    Con ca65 y ld65, `.code`, `.data` y `.bss` envían el código a zonas definidas por la configuración del enlazador. El ensamblador integrado produce **un único bloque**, en el orden de la fuente: las directivas de segmento se aceptan pero no tienen efecto, y `.bss` produce ceros. Para un proyecto que necesite un verdadero archivo `.cfg`, la biblioteca usa ca65 y ld65 (capítulo 4).

## Macros, .repeat y ensamblado condicional

Una macro es un modelo de líneas, copiado en cada llamada con sus argumentos. `.local` da a cada llamada sus propias etiquetas, sin conflictos. `.repeat` despliega un bloque al ensamblar, lo que también resulta práctico para **calcular tablas**:

```asm
cuadrados:
.repeat 16, I
        .byte I * I     ; 0, 1, 4, 9 … 225
.endrep
```

El capítulo 23 presenta `macros.s`, que usa todo esto, y `gen2-lores.s`, que calcula las direcciones de las 24 filas de la pantalla con `.repeat`.

## Mensajes de error

Cada error indica el archivo, la línea, la columna y el texto en cuestión; no se escribe ningún byte mientras quede un error. El editor muestra los mensajes en el idioma de la interfaz (abajo, en español); la línea de comandos los muestra en francés.

| Mensaje | Causa habitual |
|---|---|
| `error de sintaxis` | operador desconocido, paréntesis que falta |
| `instrucción, directiva o macro desconocida` | errata; instrucción del 65C02 sin `.setcpu`; etiqueta sin dos puntos |
| `modo de direccionamiento imposible para esta instrucción` | `sta #5`, `ldx ($06),y`… |
| `símbolo no definido` | `jsr ECO` en lugar de `ECHO`; `.include` olvidado |
| `símbolo ya definido` | dos etiquetas con el mismo nombre |
| `valor fuera de rango` | `lda #300`, dirección más allá de `$FFFF` |
| `salto demasiado lejos (-128 a +127 bytes)` | sustituya `bne lejos` por `beq :+` / `jmp lejos` / `:` |
| `fichero .include no encontrado` | nombre, o carpeta: añada `-I` |
| `bloque mal cerrado (.macro/.endmacro, .if/.endif)` | `.endif` o `.endmacro` olvidado |
| `división por cero` | en una expresión |
| `cadena o carácter sin comilla de cierre` | `"HOLA` |
| `valor desconocido en este punto (.org, .res, .if, .repeat)` | estas directivas necesitan un valor ya definido |

![Un error en el editor: ECO en lugar de ECHO. Un clic en el mensaje lleva a la línea.](img/es/brut/editeur-asm-erreur.png "large")

## Compatibilidad con ca65 y xa

El ensamblador integrado sigue la sintaxis de **ca65** (paquete cc65) en todo lo que necesita un programa de un solo bloque. Los ocho ejemplos del capítulo 23 se ensamblaron en su versión original con las dos herramientas y se compararon byte a byte. Las versiones en español solo difieren en los comentarios y los textos; su tamaño con el ensamblador integrado figura en la última columna (no se han vuelto a comparar con ca65):

| Fuente | Bytes (original) | Integrado = ca65 | Bytes (español) |
|---|---|---|---|
| `hello.s` | 33 | <span class="status-ok">idéntico</span> | 32 |
| `clavier.s` | 32 | <span class="status-ok">idéntico</span> | 32 |
| `hexa.s` | 64 | <span class="status-ok">idéntico</span> | 64 |
| `decimal.s` | 117 | <span class="status-ok">idéntico</span> | 117 |
| `jeu.s` | 222 | <span class="status-ok">idéntico</span> | 237 |
| `macros.s` | 135 | <span class="status-ok">idéntico</span> | 135 |
| `gen2-lores.s` | 110 | <span class="status-ok">idéntico</span> | 110 |
| `gen2-hires.s` | 94 | <span class="status-ok">idéntico</span> | 94 |

Las fuentes escritas para **xa** (xa65) se reconocen por sus líneas `#include` o `#define`, y se traducen línea a línea antes de ensamblar (los números de línea de los errores siguen siendo correctos): `#define`, `#include`, `*=`, `.BYT`, `.ASC`, comentarios `/* */` y etiquetas en la columna 0 sin dos puntos. La presentación de diapositivas de Dave Schmenk (capítulo 9) se ensambla así, con los mismos 3456 bytes que xa 2.4.1.

Para los **proyectos de GitHub**, la biblioteca prefiere ca65 y ld65 si están instalados (o si el proyecto aporta una configuración de ld65), con `-D APPLE1` por defecto; si no, usa el ensamblador integrado. El archivo `apple1.toml` del repositorio indica las direcciones y los símbolos (capítulo 4).
