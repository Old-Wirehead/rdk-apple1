# Fallos conocidos y cómo sortearlos

<p class="lead">El Integer BASIC del Apple-1 se escribió sin ensamblador, en 1976, y nunca se corrigió. Estos son sus defectos conocidos, cada uno reproducido en el emulador con el BASIC real, y cómo evitarlos.</p>

<span class="verified">comprobado</span>: reproducido en el emulador con el Integer BASIC real, con una transcripción que lo respalda. <span class="unverified">no comprobado</span>: señalado por una fuente, no reproducido. El fallo n corresponde a la sección 18.n. Como el emulador ejecuta el código original instrucción por instrucción, un fallo reproducido aquí existe también en un Apple-1 real.

## Salir de un bucle FOR con GOTO <span class="verified">comprobado</span>

Un bucle del que se sale con `GOTO` queda «abierto». Si el programa vuelve a pasar por el `FOR`, se apila una entrada nueva; a la novena pasada: `>8 FORS ERR`.

```basic
10 N=0
20 FOR I=1 TO 10
30 IF I=2 THEN GOTO 50
40 NEXT I
50 N=N+1:IF N<20 THEN 20
60 PRINT "HECHO ";N
70 END
```

```mon
>» RUN
*** >8 FORS ERR
STOPPED AT 20
```

!!! astuce "Rodeo"
    No salga nunca de un `FOR` con `GOTO`: dé a la variable su valor final y pase por `NEXT`. Aquí: `30 IF I=2 THEN I=10:GOTO 40`. El programa muestra entonces `HECHO 20`. Las adaptaciones de *101 BASIC Computer Games* siguen esta regla.

## El operador ^ cuelga la máquina <span class="verified">comprobado</span>

`^` (potencia) está en la tabla del intérprete, pero `PRINT 2^3` no vuelve nunca: solo RESET le saca de ahí.

!!! astuce "Rodeo"
    Multiplique: `2*2*2`, o un bucle `P=1:FOR I=1 TO N:P=P*X:NEXT I`.

## El mensaje «PPED AT» <span class="verified">comprobado</span>

Cuando la pila de operandos del intérprete se desborda (una expresión demasiado anidada en modo directo), el BASIC debería mostrar un mensaje de error, pero apunta **3 bytes demasiado lejos** en la tabla de mensajes (el desensamblado de Eric Smith lo señala: `ldy #erri_stopped_at+3 ; The "+3" is a bug!`). Lo que se lee es el final de `STOPPED AT`:

```mon
>» PRINT 1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1)))))))))))))
14
*** PPED AT  ERR
STOPPED AT 65535
```

!!! astuce "Rodeo"
    Divida las expresiones largas con variables intermedias. Por encima de 12 niveles de paréntesis, una línea de programa se rechaza de todos modos (`TOO LONG ERR`).

## IF … THEN solo gobierna la primera instrucción <span class="verified">comprobado</span>

En `IF A=2 THEN PRINT "X":PRINT "Y"`, solo `PRINT "X"` depende de la condición: `PRINT "Y"` se ejecuta **siempre** (con A=1, el programa muestra `Y`). Es la trampa más frecuente al adaptar un programa escrito para otro BASIC.

!!! astuce "Rodeo"
    Invierta la condición y salte por encima del bloque:

    ```basic
    80 IF A#2 THEN 90
    85 PRINT "X":PRINT "Y"
    90 REM SIGUE
    ```

## −32768 <span class="verified">comprobado</span>

`X=-32767-1` muestra un `-` y luego `*** >32767 ERR`. El valor más pequeño utilizable es −32767.

!!! astuce "Rodeo"
    Quédese dentro de −32767 … 32767. Para leer una palabra de 16 bits con `PEEK`, divídala: `PEEK(A)+PEEK(A+1)*256` se desborda por encima de 32767; pruebe primero el byte alto (`IF PEEK(A+1)>127 THEN …`).

## ASC( no funciona <span class="verified">comprobado</span>

`ASC("A")` está en la tabla de palabras clave, pero toda línea que lo usa se rechaza: `SYNTAX ERR`. Lo mismo ocurre con `SCRN(`.

!!! astuce "Rodeo (probado)"
    Busque el carácter en una cadena de referencia. La línea 60 cierra el bucle (`K=26:NEXT K`) para evitar el fallo 1:

```basic title="La posición de una letra, sin ASC"
10 DIM C$(26),L$(1)
20 C$="ABCDEFGHIJKLMNOPQRSTUVWXYZ"
30 INPUT "LETRA",L$
40 FOR K=1 TO 26:IF C$(K,K)=L$ THEN 60:NEXT K
50 PRINT "NO ES UNA LETRA":END
60 PRINT "POSICION ";K:K=26:NEXT K
70 GOTO 30
```

```mon
LETRA?» W
POSICION 23
LETRA?» E
POSICION 5
LETRA?» 5
NO ES UNA LETRA
```

## Palabras clave gráficas que no hacen nada <span class="verified">comprobado</span>

`COLOR=`, `PLOT` y `HLIN` se aceptan y no hacen nada; `VLIN` y `VTAB` dan `SYNTAX ERR`. Son restos del BASIC pensado para el Apple II.

!!! astuce "Rodeo"
    Para colocar texto, use `TAB n` y `PRINT` vacíos. Para dibujar en color, la tarjeta GEN2 se maneja con `POKE` (capítulo 6).

## El fallo de los 251 caracteres del Apple II <span class="verified">ausente</span>

Una versión posterior del Integer BASIC (Apple II) se cuelga con ciertas líneas de 251 caracteres o más. En el Apple-1 la línea tecleada está limitada mucho antes (unos 128 caracteres): el fallo no puede producirse.

## STOPPED AT 65535 <span class="verified">comprobado</span>

Un error causado por una línea **rechazada mientras se introduce** (demasiado anidada) o por el fallo 3 en modo directo va seguido de `STOPPED AT 65535`: el BASIC cree estar en un programa y muestra un número de línea absurdo. No tiene consecuencias; el programa en memoria no se toca.

## USR cuelga la máquina <span class="verified">comprobado</span>

`X=USR(1)` salta a `$0000`, donde normalmente no hay código: la máquina se cuelga. RESET y luego `E2B3R` vuelven al BASIC sin perder el programa.

!!! astuce "Rodeo"
    Llame a sus programas en código máquina con `CALL dirección`; pase los valores con `POKE` y `PEEK`.

## HIMEM= no puede bajar <span class="verified">comprobado</span>

`HIMEM=3072` (y cualquier valor por debajo del actual) se **ignora en silencio**, incluso tras `SCR`, porque el intérprete compara el nuevo valor con el puntero de programa antes de aceptarlo.

!!! astuce "Rodeo (comprobado)"
    `POKE 76,0 : POKE 77,12 : SCR` pone HIMEM en 3072; el programa siguiente se guarda por debajo. Vea el capítulo 16.

## Los dos puntos rechazados en modo directo <span class="verified">comprobado</span>

`X=5:Y=X*2:PRINT Y` escrito sin número da `SYNTAX ERR`; la misma línea con número funciona. `FOR` y `GOSUB` también se rechazan en modo directo.

!!! astuce "Rodeo"
    Escriba una instrucción por línea, o escriba una línea provisional `0` y `RUN 0`… sin olvidar borrarla (`DEL 0`).

## INPUT siempre añade «?» <span class="verified">comprobado</span>

`INPUT "EDAD ? ",N` muestra `EDAD ? ?`: el BASIC añade su propio signo de interrogación. Escriba la indicación sin `?`: `INPUT "EDAD",N` muestra `EDAD?`.

## El bucle FOR se ejecuta al menos una vez <span class="verified">comprobado</span>

`FOR I=5 TO 1 … NEXT I` ejecuta el cuerpo una vez (con I=5), donde otros BASIC lo saltan.

!!! astuce "Rodeo"
    Compruebe antes: `IF N<1 THEN 100` antes de `FOR I=1 TO N`.

## Una pulsación detiene el programa <span class="verified">comprobado</span>

Cualquier tecla pulsada mientras el programa se ejecuta lo detiene (`STOPPED AT`), salvo durante un `INPUT`. No es un fallo sino una decisión de diseño: no hay tecla BREAK. Consecuencia: un juego en tiempo real no puede esperar una tecla sin detenerse. Leer el teclado con `PEEK(-12271)` (`$D011`) y `PEEK(-12272)` (`$D010`) parece posible, pero el intérprete también comprueba el teclado entre instrucciones y puede detenerse antes: no lo hemos probado (<span class="unverified">no comprobado</span>). Para un juego en tiempo real, pase al código máquina (Parte III).

## Resumen

| # | Fallo | Estado | Rodeo |
|---|---|---|---|
| 1 | salir de FOR con GOTO → `>8 FORS` | <span class="status-ok">comprobado</span> | terminar el bucle con NEXT |
| 2 | `^` cuelga | <span class="status-ok">comprobado</span> | multiplicar |
| 3 | «PPED AT» (+3) | <span class="status-ok">comprobado</span> | variables intermedias |
| 4 | `IF…THEN a:b` ejecuta b | <span class="status-ok">comprobado</span> | condición invertida + salto |
| 5 | −32768 | <span class="status-ok">comprobado</span> | quedarse ≥ −32767 |
| 6 | `ASC(`, `SCRN(` | <span class="status-ok">comprobado</span> | cadena de referencia |
| 7 | COLOR=, PLOT, HLIN no hacen nada; VLIN, VTAB rechazados | <span class="status-ok">comprobado</span> | TAB, POKE (GEN2) |
| 8 | 251 caracteres (Apple II) | <span class="status-ok">ausente</span> | — |
| 9 | STOPPED AT 65535 | <span class="status-ok">comprobado</span> | inofensivo |
| 10 | USR se cuelga | <span class="status-ok">comprobado</span> | CALL |
| 11 | HIMEM= no baja | <span class="status-ok">comprobado</span> | POKE 76/77 + SCR |
| 12 | `:` rechazado en modo directo | <span class="status-ok">comprobado</span> | una instrucción por línea |
| 13 | INPUT añade «?» | <span class="status-ok">comprobado</span> | indicación sin «?» |
| 14 | FOR se ejecuta una vez | <span class="status-ok">comprobado</span> | comprobar antes |
| 15 | leer el teclado con PEEK durante RUN | <span class="status-src">no comprobado</span> | código máquina |
