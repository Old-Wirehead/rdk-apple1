# Variables, cadenas, matrices y memoria

## Números

El Integer BASIC solo conoce **enteros de 16 bits con signo**, de −32767 a 32767. Un cálculo que sale de ese rango se detiene con `*** >32767 ERR` (`200*200`, `32767+1`, y también `10/0` y `RND(0)`). El valor −32768 no se puede usar (capítulo 18, fallo 5). No hay coma decimal ni `INT`: `7/2` vale 3.

## Nombres de variable

Es el límite que más sorprende: un nombre de variable es **una letra, seguida opcionalmente de una cifra**.

| Nombre | Resultado (probado) |
|---|---|
| `A`, `Z`, `B1`, `Z9` | aceptado |
| `AB`, `CNT`, `SCORE`, `COUNTER` | `SYNTAX ERR` |
| `A12` | `SYNTAX ERR` |
| `A$`, `N$` | cadena: **una sola letra** |
| `B9$`, `Z9$` | `SYNTAX ERR` |

Eso da 26 × 11 = 286 variables enteras posibles, y 26 cadenas. Acostúmbrese a anotar el papel de cada variable en un `REM`.

## Cadenas

Una cadena debe **dimensionarse** antes de usarla, con su longitud máxima (1 a 255): `DIM N$(20)`. `DIM B$(256)` da `>255 ERR`.

| Operación | Se escribe | Ejemplo (N$ = "RADAR") |
|---|---|---|
| longitud | `LEN(N$)` | 5 |
| subcadena | `N$(i,j)`: caracteres i a j | `N$(1,3)` = "RAD" |
| final | `N$(i)`: del carácter i al final | `N$(4)` = "AR" |
| concatenar | `A$(LEN(A$)+1)=B$` | "APPLE" + "-1" → "APPLE-1" |
| comparar | solo `=` y `#` | `IF N$="WOZ" THEN …` |

Asignar una cadena más larga que su `DIM` da `*** STR OVFL ERR`, igual que una respuesta demasiado larga a `INPUT`. No hay `MID$`, `LEFT$`, `CHR$`, `STR$`, `VAL` ni `ASC` (este último está en la tabla pero no funciona).

!!! astuce "Sustituir CHR$ y VAL"
    Para mostrar un carácter a partir de su código, tenga una cadena de referencia y córtela: `DIM C$(26):C$="ABCDEFGHIJKLMNOPQRSTUVWXYZ"` y luego `PRINT C$(K,K)`. Para leer un número, use `INPUT N` en lugar de `INPUT N$`.

## Matrices

`DIM T(10)` crea una matriz de 10 enteros, **numerados de 1 a 10**. `T(0)` y `T(11)` dan `*** RANGE ERR`. Solo hay una dimensión: para una cuadrícula de 8 × 8, use `G((L-1)*8+C)`. En el emulador, los elementos valen 0 tras `DIM`; en una máquina real la memoria no se borra necesariamente: inicialice sus matrices. Una matriz demasiado grande para la memoria da `*** MEM FULL ERR` (`DIM A(3000)` en 8 KB).

## La memoria del BASIC

![Memoria del Apple-1: el BASIC en $E000, el programa por debajo de HIMEM, las variables por encima de LOMEM.](diagrams/es/memoire.svg "diagram large")

El BASIC guarda el **programa arriba** de la memoria, creciendo hacia abajo desde HIMEM, y las **variables abajo**, creciendo hacia arriba desde LOMEM. El espacio libre queda entre los dos.

| Puntero | Dirección | Valor con 8 KB | Papel |
|---|---|---|---|
| LOMEM | `$4A-$4B` (74-75) | `$0800` (2048) | inicio de las variables |
| HIMEM | `$4C-$4D` (76-77) | `$1000` (4096) | fin de la memoria del BASIC |
| PP | `$CA-$CB` (202-203) | baja con el programa | inicio del programa |
| PV | `$CC-$CD` (204-205) | sube con las variables | fin de las variables |

{{!LOMEM}}{{!HIMEM}}{{!PP (puntero de programa)}}

`PRINT PEEK(202)+PEEK(203)*256` da el inicio del programa y `PRINT PEEK(204)+PEEK(205)*256` el fin de las variables: el espacio libre es la diferencia entre ambos. Una variable entera como `A` ocupa 6 bytes.

!!! apple1 "Por qué un casete BASIC empieza en $004A"
    Estos punteros están en la página cero, de `$4A` a `$FF`. Un casete BASIC graba, pues, **dos rangos**: `004A.00FF` (los punteros) y luego el programa de PP a HIMEM, por ejemplo `0F2C.0FFF`. El comando `apple1-emu basic` los calcula por usted (su salida está en francés: «10 líneas, 212 bytes… 1836 bytes libres para las variables… rangos ACI (tras C100R)… luego E2B3R y RUN»):

    ```text
    10 lignes, 212 octets ; LOMEM $0800 HIMEM $1000 PP $0F2C ; 1836 octets libres pour les variables ; RAM 8 Ko
    Plages ACI (après C100R) : 004A.00FFR 0F2C.0FFFR   puis E2B3R et RUN
    ```

### Cambiar LOMEM y HIMEM

La sintaxis es `LOMEM=n` y `HIMEM=n` (con `=`, **no** `:` como en el Apple II, que da `SYNTAX ERR`).

- `LOMEM=2304` desplaza el inicio de las variables y **borra las variables** (el programa se queda). Útil para reservar `$0800-$08FF` a un programa en código máquina.
- `HIMEM=` **más alto** (con más RAM, por ejemplo `HIMEM=8192` con 16 KB) funciona y conserva el programa.
- `HIMEM=` **más bajo** lo **ignora en silencio** el original, incluso tras `SCR` (capítulo 18, fallo 11). **RDK G2 Basic lo acepta**: el programa en memoria se desplaza hacia abajo con el límite, o se da `MEM FULL ERR` si ya no cabría por encima de las variables (`RANGE ERR` para un valor negativo; `LOMEM=` da `RANGE ERR` si no queda por debajo del inicio del programa, y `>32767 ERR` por encima de 32767). Con el original hay un rodeo comprobado: escribir el puntero directamente y luego `SCR`:

```basic title="Bajar HIMEM a 3072 ($0C00)"
POKE 76,0 : POKE 77,12 : SCR
```

Al arrancar G2 Basic, HIMEM es el tope de la RAM encontrada y LOMEM es `$0800` (`$2000` con un controlador GEN2). (`76`/`77` = `$4C`/`$4D`, 12 × 256 = 3072.) Escriba estas tres instrucciones en una línea numerada o por separado en modo directo, **antes** de introducir el programa: `SCR` lo borra.

## Mezclar BASIC y código máquina

| Zona (8 KB) | ¿Utilizable para código 6502? |
|---|---|
| `$0000-$00FF` | página cero: el Woz Monitor usa `$24-$2B`, el BASIC al menos `$4A-$FF`; evítela, o guárdela y restáurela |
| `$0100-$01FF` | pila del 6502: no |
| `$0200-$027F` | búfer de línea del Woz Monitor; lo sobrescribe cada línea tecleada |
| `$0280-$07FF` | libre mientras el BASIC no la toque: el mejor sitio |
| `$0800-…` | variables del BASIC (salvo si se sube LOMEM) |
| `… -$0FFF` | programa BASIC |

El programa en código máquina se carga desde casete o por POKE, y se llama con `CALL 768` (para `$0300`). Debe terminar con `RTS` y respetar la página cero del BASIC. Ejemplo mínimo:

```mon
>» POKE 768,96
>» CALL 768
>
```

(96 = `$60` = `RTS`: la llamada vuelve enseguida.)
