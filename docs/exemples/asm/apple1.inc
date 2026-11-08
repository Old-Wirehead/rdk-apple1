; apple1.inc — adresses d'E/S et routines du Woz Monitor
; (vérifiées dans la ROM : FFDC PRBYTE, FFE5 PRHEX, FFEF ECHO, FF1F GETLINE)
KBD     = $D010         ; donnée clavier (bit 7 = 1)
KBDCR   = $D011         ; bit 7 = 1 : une touche attend
DSP     = $D012         ; écriture : caractère vers l'écran (bit 7 lu = occupé)
DSPCR   = $D013
PRBYTE  = $FFDC         ; affiche A en 2 chiffres hexa (détruit A)
PRHEX   = $FFE5         ; affiche le quartet bas de A (détruit A)
ECHO    = $FFEF         ; affiche le caractère de A (A, X, Y conservés)
GETLINE = $FF1F         ; retour au moniteur : retour à la ligne, puis invite
CR      = $8D           ; retour chariot (bit 7 mis, comme le moniteur)
