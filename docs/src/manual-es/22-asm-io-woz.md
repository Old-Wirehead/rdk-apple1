# El teclado, la pantalla y el Woz Monitor

<p class="lead">El Apple-1 solo tiene cuatro registros de entrada/salida, en un PIA 6820 (o 6821), y un monitor de 256 bytes. No es mucho, pero basta: este capítulo muestra cómo leer una tecla, escribir un carácter y usar las rutinas de Steve Wozniak.</p>

## Los cuatro registros del PIA

| Dirección | Nombre | Lectura | Escritura |
|---|---|---|---|
| `$D010` | `KBD` | código de la tecla, bit 7 a uno; la lectura **acusa recibo** de la tecla | — |
| `$D011` | `KBDCR` | bit 7 = 1: hay una tecla esperando | configuración (la hace el monitor) |
| `$D012` | `DSP` | bit 7 = 1: la pantalla está ocupada | carácter que mostrar (7 bits) |
| `$D013` | `DSPCR` | — | configuración (la hace el monitor) |

{{!KBD}}{{!KBDCR}}{{!DSP}}

Estos nombres están definidos en `apple1.inc`, que incluyen todos los ejemplos:

@@include exemples/es/asm/apple1.inc

## Leer una tecla

Esperar a que el bit 7 de `KBDCR` pase a 1 y luego leer `KBD`:

```asm
leer:   lda KBDCR       ; ¿una tecla?
        bpl leer        ; bit 7 = 0: todavía no
        lda KBD         ; código ASCII + $80
```

El teclado solo produce mayúsculas (el emulador convierte las minúsculas), con el bit 7 a uno: `A` da `$C1`, RETURN `$8D`, ESC `$9B`. Para saber si se ha pulsado una tecla **sin esperar**, pruebe solo `KBDCR`:

```asm
        lda KBDCR
        bpl nada        ; ninguna tecla: seguir trabajando
        lda KBD         ; una tecla: leerla (y acusar recibo)
```

`jeu.s` (capítulo 23) lo usa para sacar un número al azar: cuenta en un bucle hasta que el jugador pulsa una tecla.

## Escribir un carácter

El terminal del Apple-1 acepta un carácter cada vez, y solo cuando está listo: el bit 7 de `DSP` sigue a 1 hasta que se ha mostrado el carácter anterior. La rutina `ECHO` del monitor espera por usted. Estas son sus cuatro instrucciones, leídas en el monitor del emulador:

```mon
\
» FFEF.FFFFR

FFEF: 2C
FFF0: 12 D0 30 FB 8D 12 D0 60
FFF8: 00 00 00 0F 00 FF 00 00
```

Es decir, `BIT DSP` / `BMI ECHO` / `STA DSP` / `RTS`. `ECHO` conserva A, X e Y. Como el terminal original solo muestra **un carácter por imagen**, unos 60 por segundo, un programa que muestra mucho está limitado por la pantalla, no por el procesador; el emulador respeta este límite por defecto (<kbd>F4</kbd> lo quita, capítulo 2).

Qué hace la pantalla con cada código:

- **RETURN** (`$8D`, o `$0D`) empieza una línea nueva; al final de la pantalla, todo sube una línea.
- Los **demás caracteres de control** (`$00-$1F`, con o sin el bit 7) se ignoran: ni borrado, ni retroceso, ni timbre.
- Las **minúsculas** `$60-$7F` se muestran como las mayúsculas `$40-$5F`: el terminal solo conserva 6 bits.
- Tras la columna 40, el cursor pasa solo a la línea siguiente.
- No hay código para borrar la pantalla: es un botón de la máquina (**Borrar pantalla**, <kbd>Ctrl</kbd>+<kbd>L</kbd>).

## Las rutinas del Woz Monitor

| Dirección | Nombre | Entrada | Efecto | Registros |
|---|---|---|---|---|
| `$FFEF` | `ECHO` | A = carácter | lo muestra | A, X, Y conservados |
| `$FFDC` | `PRBYTE` | A = byte | lo muestra con 2 cifras hexadecimales | A destruido; X, Y conservados |
| `$FFE5` | `PRHEX` | A (nibble bajo) | muestra una cifra hexadecimal | A destruido; X, Y conservados |
| `$FF1F` | `GETLINE` | — | vuelta al monitor: línea nueva y espera de un comando | — |
| `$FF1A` | `ESCAPE` | — | como `GETLINE`, mostrando antes `\` | — |
| `$FF00` | `RESET` | — | inicializa el PIA y el monitor | — |

{{!ECHO}}{{!PRBYTE}}{{!PRHEX}}{{!GETLINE}}

El monitor usa la página cero `$24-$2B` y el búfer de línea `$0200-$027F`; todo lo demás es suyo (salvo si el Integer BASIC debe sobrevivir, capítulo 16). Sus comandos (`300R`, `0300.031F`, `300: A9 8D`…) se resumen en el apéndice B.

## Arrancar y terminar un programa

Un programa se arranca con el comando `R` del monitor (`300R`), que hace un `JMP` a la dirección, no un `JSR`. No hay, pues, **ninguna dirección de retorno** en la pila: termine con `jmp GETLINE` (`$FF1F`), no con `RTS`.

!!! attention "RTS y BRK no devuelven el control"
    Un `RTS` al final de un programa arrancado con `R` saca lo que haya en la pila. Un `BRK` salta por el vector de interrupción de la ROM, que es `$0000` (las tres últimas palabras del monitor, arriba: NMI `$0F00`, RESET `$FF00`, IRQ/BRK `$0000`): el procesador ejecuta entonces la página cero. En ambos casos, pulse RESET (<kbd>Ctrl</kbd>+<kbd>R</kbd>).

En cambio, un programa llamado desde el BASIC con `CALL 768` se llama mediante un `JSR`: debe terminar con `RTS` para volver al BASIC. El capítulo 16 muestra cómo colocar código máquina por debajo del programa BASIC.

```mon
>» POKE 768,96
>» CALL 768
>
```

(`96` = `$60`, el opcode de `RTS`: la subrutina más pequeña posible.)

## El PIA, el casete y la tarjeta de color

La tarjeta ACI ocupa `$C000-$C0FF` (cada acceso invierte la salida) y su programa `$C100-$C1FF`; el capítulo 5 explica su funcionamiento y el apéndice C sus tiempos. La tarjeta GEN2 se controla simplemente **leyendo** `$C250-$C257` (`bit SS_GR`); sus direcciones están en `gen2.inc`:

@@include exemples/es/asm/gen2.inc
