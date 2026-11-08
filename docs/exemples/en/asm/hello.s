; hello.s — the classic: a message, then back to the Woz Monitor
        .include "apple1.inc"
        .org $0300

start:  ldx #0
@next:  lda message,x   ; next character
        beq @done       ; 0 = end of the string
        jsr ECHO
        inx
        bne @next
@done:  jmp GETLINE

message:
        .byte CR
        .asciiz "HELLO, APPLE-1!"
