; hello.s — le classique : un message, puis retour au Woz Monitor
        .include "apple1.inc"
        .org $0300

start:  ldx #0
@suite: lda message,x   ; caractère suivant
        beq @fin        ; 0 = fin de la chaîne
        jsr ECHO
        inx
        bne @suite
@fin:   jmp GETLINE

message:
        .byte CR
        .asciiz "BONJOUR, APPLE-1 !"
