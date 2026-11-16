; apple1.inc — endereços de E/S e rotinas do Woz Monitor
; (verificado na ROM: FFDC PRBYTE, FFE5 PRHEX, FFEF ECHO, FF1F GETLINE)
KBD     = $D010         ; dado do teclado (bit 7 = 1)
KBDCR   = $D011         ; bit 7 = 1: há uma tecla à espera
DSP     = $D012         ; escrita: carácter para o ecrã (bit 7 lido = ocupado)
DSPCR   = $D013
PRBYTE  = $FFDC         ; mostra A em 2 algarismos hexadecimais (A perdido)
PRHEX   = $FFE5         ; mostra o nibble baixo de A (A perdido)
ECHO    = $FFEF         ; mostra o carácter de A (A, X, Y conservados)
GETLINE = $FF1F         ; regresso ao monitor: linha nova e depois prompt
CR      = $8D           ; retorno de carro (bit 7 a 1, como o monitor o usa)
