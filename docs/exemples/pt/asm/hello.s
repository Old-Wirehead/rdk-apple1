; hello.s — o clássico: uma mensagem e regresso ao Woz Monitor
        .include "apple1.inc"
        .org $0300

start:  ldx #0
@next:  lda message,x   ; carácter seguinte
        beq @done       ; 0 = fim da cadeia
        jsr ECHO
        inx
        bne @next
@done:  jmp GETLINE

message:
        .byte CR
        .asciiz "OLA, APPLE-1!"
