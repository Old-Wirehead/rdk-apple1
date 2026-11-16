; clavier.s — lê o teclado e mostra cada tecla no ecrã.
; Os algarismos aparecem duas vezes; ESC volta ao monitor.
        .include "apple1.inc"
        .org $0300

read:   lda KBDCR       ; uma tecla?
        bpl read        ; bit 7 = 0: ainda não
        lda KBD         ; código ASCII + $80; a leitura apaga o sinal da tecla
        cmp #$9B        ; ESC
        beq done
        jsr ECHO
        cmp #'0' | $80
        bcc read        ; < '0'
        cmp #'9' | $80 + 1
        bcs read        ; > '9'
        jsr ECHO        ; um algarismo: mostra-se uma segunda vez
        jmp read
done:   jmp GETLINE
