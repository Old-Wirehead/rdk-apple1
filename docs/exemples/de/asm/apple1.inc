; apple1.inc — E/A-Adressen und Routinen des Woz Monitors
; (im ROM geprüft: FFDC PRBYTE, FFE5 PRHEX, FFEF ECHO, FF1F GETLINE)
KBD     = $D010         ; Tastaturdaten (Bit 7 = 1)
KBDCR   = $D011         ; Bit 7 = 1: eine Taste wartet
DSP     = $D012         ; Schreiben: Zeichen an die Anzeige (Bit 7 gelesen = belegt)
DSPCR   = $D013
PRBYTE  = $FFDC         ; gibt A als 2 Hex-Ziffern aus (A zerstört)
PRHEX   = $FFE5         ; gibt das untere Nibble von A aus (A zerstört)
ECHO    = $FFEF         ; gibt das Zeichen in A aus (A, X, Y bleiben erhalten)
GETLINE = $FF1F         ; zurück zum Monitor: neue Zeile, dann Eingabeaufforderung
CR      = $8D           ; Wagenrücklauf (Bit 7 gesetzt, wie der Monitor ihn verwendet)
