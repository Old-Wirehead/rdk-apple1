; hello.s — der Klassiker: eine Meldung, dann zurück zum Woz Monitor
        .include "apple1.inc"
        .org $0300

start:  ldx #0
@next:  lda message,x   ; nächstes Zeichen
        beq @done       ; 0 = Ende der Zeichenkette
        jsr ECHO
        inx
        bne @next
@done:  jmp GETLINE

message:
        .byte CR
        .asciiz "HALLO, APPLE-1!"
