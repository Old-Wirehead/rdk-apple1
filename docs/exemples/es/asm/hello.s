; hello.s — el clásico: un mensaje y vuelta al Woz Monitor
        .include "apple1.inc"
        .org $0300

start:  ldx #0
@next:  lda message,x   ; carácter siguiente
        beq @done       ; 0 = fin de la cadena
        jsr ECHO
        inx
        bne @next
@done:  jmp GETLINE

message:
        .byte CR
        .asciiz "HOLA, APPLE-1!"
