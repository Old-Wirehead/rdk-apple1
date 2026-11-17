; Bonjour - Retro Dev Kit Apple 1 2.0
; Affiche un message par ECHO ($FFEF) puis revient au Woz Monitor.
ECHO    = $FFEF
GETLINE = $FF1F
        .org $0300
start:  LDX #0
boucle: LDA message,X
        BEQ fin
        ORA #$80
        JSR ECHO
        INX
        BNE boucle
fin:    JMP GETLINE
message: .byte $0D, "BONJOUR DE L'APPLE 1 !", $0D, 0
