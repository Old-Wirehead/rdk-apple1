; clavier.s — lit le clavier et renvoie chaque touche à l'écran.
; Les chiffres sont doublés ; ÉCHAP (ESC) rend la main au moniteur.
        .include "apple1.inc"
        .org $0300

lire:   lda KBDCR       ; une touche ?
        bpl lire        ; bit 7 = 0 : pas encore
        lda KBD         ; code ASCII + $80 ; la lecture acquitte la touche
        cmp #$9B        ; ESC
        beq fin
        jsr ECHO
        cmp #'0' | $80
        bcc lire        ; < '0'
        cmp #'9' | $80 + 1
        bcs lire        ; > '9'
        jsr ECHO        ; un chiffre : on l'affiche une 2e fois
        jmp lire
fin:    jmp GETLINE
