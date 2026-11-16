; apple1.inc — indirizzi di I/O e routine del Woz Monitor
; (verificato nella ROM: FFDC PRBYTE, FFE5 PRHEX, FFEF ECHO, FF1F GETLINE)
KBD     = $D010         ; dato della tastiera (bit 7 = 1)
KBDCR   = $D011         ; bit 7 = 1: un tasto è in attesa
DSP     = $D012         ; scrittura: carattere allo schermo (bit 7 letto = occupato)
DSPCR   = $D013
PRBYTE  = $FFDC         ; stampa A in 2 cifre esadecimali (A distrutto)
PRHEX   = $FFE5         ; stampa il nibble basso di A (A distrutto)
ECHO    = $FFEF         ; stampa il carattere in A (A, X, Y conservati)
GETLINE = $FF1F         ; ritorno al monitor: nuova riga, poi prompt
CR      = $8D           ; ritorno a capo (bit 7 a 1, come lo usa il monitor)
