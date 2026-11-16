; hello.s — il classico: un messaggio, poi ritorno al Woz Monitor
        .include "apple1.inc"
        .org $0300

start:  ldx #0
@next:  lda message,x   ; carattere seguente
        beq @done       ; 0 = fine della stringa
        jsr ECHO
        inx
        bne @next
@done:  jmp GETLINE

message:
        .byte CR
        .asciiz "CIAO, APPLE-1!"
