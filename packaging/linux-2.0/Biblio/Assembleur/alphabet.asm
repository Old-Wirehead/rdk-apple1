; Alphabet - Retro Dev Kit Apple 1 2.0
; Affiche A a Z en boucle sur le terminal 40 x 24.
ECHO    = $FFEF
GETLINE = $FF1F
        .org $0300
start:  LDA #'A'
suite:  JSR ECHO
        CLC
        ADC #1
        CMP #'Z'+1
        BNE suite
        LDA #$8D        ; retour a la ligne
        JSR ECHO
        JMP GETLINE
