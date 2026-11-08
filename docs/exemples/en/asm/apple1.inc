; apple1.inc — I/O addresses and Woz Monitor routines
; (checked in the ROM: FFDC PRBYTE, FFE5 PRHEX, FFEF ECHO, FF1F GETLINE)
KBD     = $D010         ; keyboard data (bit 7 = 1)
KBDCR   = $D011         ; bit 7 = 1: a key is waiting
DSP     = $D012         ; write: character to the display (bit 7 read = busy)
DSPCR   = $D013
PRBYTE  = $FFDC         ; prints A as 2 hex digits (A destroyed)
PRHEX   = $FFE5         ; prints the low nibble of A (A destroyed)
ECHO    = $FFEF         ; prints the character in A (A, X, Y preserved)
GETLINE = $FF1F         ; back to the monitor: new line, then prompt
CR      = $8D           ; carriage return (bit 7 set, as the monitor uses it)
