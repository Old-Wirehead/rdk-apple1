# Su primer programa BASIC

El Integer BASIC, el BASIC de Steve Wozniak, cabe en 4 KB, y el **RDK G2 Basic** propio del emulador, que ejecuta los mismos programas, viene integrado. Solo conoce los números enteros (−32767 a 32767), pero es rápido y agradable. El editor integrado le ahorra escribir en la pantalla del Apple-1, donde un error de tecleo obliga a volver a escribir la línea entera.

## Escribir el programa

Haga clic en **📝 Editor** en la barra de herramientas, luego en el icono **Nuevo** (el primero) del editor, elija «Programa BASIC» y escriba este programa (también en `docs/exemples/es/basic/premier.bas`).

```basic title="premier.bas"
10 REM MI PRIMER PROGRAMA
20 PRINT "HOLA, APPLE-1!"
30 PRINT
40 FOR I=1 TO 8
50 PRINT I;" AL CUADRADO = ";I*I
60 NEXT I
70 PRINT
80 DIM N$(20)
90 INPUT "TU NOMBRE",N$
100 PRINT "ENCANTADO, ";N$;"!"
110 END
```

Las palabras clave, los números, las cadenas y los comentarios tienen cada uno su propio color (los colores se cambian en **⚙ Preferencias → Editor**); bajo el texto, el editor muestra el tamaño del programa y la memoria que queda para las variables. El Apple-1 no tiene tildes ni eñes: el programa usa solo mayúsculas sin acentos.

!!! astuce "Clic derecho en el texto"
    El menú del editor contiene cortar, copiar y pegar, **Buscar** (Ctrl+F) y **Reemplazar** (Ctrl+H), ir a una línea (Ctrl+G), REM / quitar REM en un bloque, **Renumerar…** (`GOTO`, `GOSUB` y `THEN` siguen), un punto de interrupción en la línea (F9) y **Enviar y ejecutar** (F5).

## Ejecutarlo

Haga clic en **▶ Ejecutar**. El programa va directamente a la memoria (después del BASIC, si hace falta) y luego el editor escribe por usted `E2B3R` y `RUN`:

![El editor a la izquierda, el programa en marcha a la derecha.](img/es/brut/editeur-premier.png "large")

El programa le pide su nombre. Haga clic en la pantalla del Apple-1, escríbalo y pulse Intro:

```mon
TU NOMBRE?WOZ
ENCANTADO, WOZ!
```

!!! attention "Tres particularidades del Integer BASIC"
    - **Las cadenas deben dimensionarse**: sin `DIM N$(20)`, el `INPUT` de una cadena se detiene con `*** STR OVFL ERR`.
    - **Los nombres de variables** tienen una letra, seguida opcionalmente de una cifra: `A`, `B2`, `N$`. `TOTAL` es un error de sintaxis.
    - **El `END` final es obligatorio**: sin él, el programa termina con `*** END ERR`.

## Cuando hay un error

El editor comprueba cada línea mientras escribe, con las reglas del BASIC real. Una línea errónea se pone en rojo y el mensaje explica el problema, con el número de línea y la columna:

![Dos errores señalados: una cadena sin cerrar y un nombre de variable demasiado largo.](img/es/editeur-erreur.png "medium")

Un programa con errores no se ejecuta: corrija primero las líneas rojas. Los errores de ejecución aparecen en la pantalla del Apple-1: `*** BAD BRANCH ERR` para un `GOTO` a una línea que no existe, `*** >32767 ERR` para un desbordamiento o una división por cero. El **Depurador** (Manual completo) avanza entonces línea a línea vigilando las variables.

![El ciclo de trabajo: escribir, ejecutar, corregir y luego exportar a la máquina real.](diagrams/es/workflow-basic.svg "diagram")

!!! astuce "Detener un programa"
    Durante un `RUN`, pulsar una tecla detiene el programa con `STOPPED AT 50` (la línea en curso). Si se queda bloqueado, **Reset** (Ctrl+R) vuelve al monitor y `E2B3R` vuelve al BASIC **sin borrar** el programa.
