# Consejos, trampas y límites

<p class="lead">Lo que se aprende escribiendo programas para el Apple-1, primero en el emulador y luego en la máquina real.</p>

## Dónde poner un programa

| Dirección | Para |
|---|---|
| `$0280` | lo más abajo posible: justo después del búfer del monitor |
| `$0300` | la elección habitual (y la dirección por defecto del ensamblador) |
| `$0C00` | con la tarjeta GEN2 en LORES o texto (páginas `$0400-$0BFF` libres para la pantalla) |
| por debajo de HIMEM | con el Integer BASIC: reserve el espacio (capítulo 16) |

Para la página cero, los ejemplos usan `$06-$09`, que el monitor deja libres. Si el programa debe convivir con el BASIC, elija direcciones que el BASIC no use, o guárdelas y restáurelas.

## Vivir con el alcance de las bifurcaciones

Una bifurcación solo llega 127 bytes hacia delante o 128 hacia atrás. Cuando el ensamblador responde `salto demasiado lejos`, invierta la condición y salte por encima de un `JMP`:

```asm
        beq :+          ; en lugar de: bne lejos
        jmp lejos
:
```

## Las cadenas y el bit 7

El teclado devuelve códigos con el bit 7 a uno, y el monitor compara con esos códigos. La pantalla, en cambio, ignora el bit 7: `.asciiz "HOLA"` se muestra perfectamente. Para **comparar** una tecla, ponga el bit 7 en la constante: `cmp #'Q' | $80`. Las minúsculas de una cadena se muestran en mayúsculas.

## Esperar un poco

A 1,023 MHz (0,96 MHz efectivos), un bucle `dex` / `bne` cuesta 5 ciclos por vuelta:

```asm
pausa:
        ldy #78         ; 78 x 256 vueltas de 5 ciclos: unos 0,1 s
:       ldx #0
:       dex
        bne :-
        dey
        bne :--
        rts
```

El emulador a velocidad real respeta estas duraciones; a velocidad rápida (<kbd>F3</kbd>), desaparecen. Los bucles de tiempo de la ACI se cuentan igualmente en ciclos (apéndice C).

## Depurar con eficacia

- Ponga los puntos de parada en una **etiqueta** (`start`, `prdec`, `start@done`) en lugar de en una dirección: sigue siendo correcto cuando el código se desplaza.
- Pase por encima de `jsr ECHO` con **↪ Saltar**: si no, seguirá el bucle de espera de la pantalla instrucción a instrucción.
- Si el Apple-1 deja de responder tras su programa, mire PC: en página cero, casi siempre es un `BRK` o un `RTS` perdido (capítulo 22).
- El **volcado** muestra la página cero: allí se ven sus punteros, byte bajo primero.

![El depurador detenido en una fuente 6502: registros, indicadores, desensamblado con etiquetas y memoria.](img/es/brut/debogueur-asm.png "large")

## Hacia el Apple-1 real

<ol class="steps">
<li>Ensamble y pruebe en el emulador, <strong>a velocidad real</strong> y con la pantalla limitada a 60 caracteres por segundo.</li>
<li>Exporte el casete (<strong>📼 Exportar WAV</strong>, o <code>-o prog.aif</code>): el AIFF es el formato más seguro si reproduce desde un smartphone.</li>
<li>En el Apple-1: <code>C100R</code>, luego el rango indicado (<code>0300.031FR</code>), inicie la reproducción y después <code>300R</code>.</li>
<li>Sin ACI: el volcado <code>.hex</code> se puede teclear (o pegar en un terminal serie) directamente en el Woz Monitor; su última línea ejecuta el programa.</li>
</ol>

## Límites del ensamblador integrado

- Un único bloque de salida, en el orden de la fuente: los segmentos no tienen efecto y no hay enlazador. Para código repartido en varias zonas, use ca65 y ld65.
- Ni operador módulo, ni `.proc`, `.scope`, `.struct` o `.enum`: son directivas de ca65 que el ensamblador integrado no conoce.
- Las instrucciones del 65C02 se ensamblan, pero el procesador emulado es un 6502 NMOS: no funcionan en el emulador (capítulo 20).
- En el **listado**, una línea producida por una macro pierde a veces el espacio entre la instrucción y el operando (`ldastring, x` por `lda string,x`). Es solo un defecto de visualización del listado: los bytes son correctos.
