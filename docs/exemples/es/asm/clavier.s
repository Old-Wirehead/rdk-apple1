; clavier.s — lee el teclado y muestra cada tecla en la pantalla.
; Las cifras se muestran dos veces; ESC vuelve al monitor.
        .include "apple1.inc"
        .org $0300

read:   lda KBDCR       ; ¿una tecla?
        bpl read        ; bit 7 = 0: todavía no
        lda KBD         ; código ASCII + $80; la lectura borra el aviso de tecla
        cmp #$9B        ; ESC
        beq done
        jsr ECHO
        cmp #'0' | $80
        bcc read        ; < '0'
        cmp #'9' | $80 + 1
        bcs read        ; > '9'
        jsr ECHO        ; una cifra: se muestra una segunda vez
        jmp read
done:   jmp GETLINE
