; apple1.inc — direcciones de E/S y rutinas del Woz Monitor
; (comprobado en la ROM: FFDC PRBYTE, FFE5 PRHEX, FFEF ECHO, FF1F GETLINE)
KBD     = $D010         ; dato del teclado (bit 7 = 1)
KBDCR   = $D011         ; bit 7 = 1: hay una tecla esperando
DSP     = $D012         ; escritura: carácter a la pantalla (bit 7 leído = ocupado)
DSPCR   = $D013
PRBYTE  = $FFDC         ; muestra A en 2 cifras hexadecimales (A se pierde)
PRHEX   = $FFE5         ; muestra el nibble bajo de A (A se pierde)
ECHO    = $FFEF         ; muestra el carácter de A (A, X, Y se conservan)
GETLINE = $FF1F         ; vuelta al monitor: línea nueva y luego indicador
CR      = $8D           ; retorno de carro (bit 7 activado, como lo usa el monitor)
