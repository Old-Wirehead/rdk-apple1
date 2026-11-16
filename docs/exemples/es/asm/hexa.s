; hexa.s — volcado de memoria al estilo del monitor, con PRBYTE:
; 2 líneas de 8 bytes a partir de $FF00
        .include "apple1.inc"
        .org $0300
PTR     = $06           ; puntero en la página cero (no lo usa el monitor)

start:  lda #<$FF00
        sta PTR
        lda #>$FF00
        sta PTR+1
        ldx #2          ; número de líneas
line:   lda #CR
        jsr ECHO
        lda PTR+1       ; dirección: byte alto y luego byte bajo
        jsr PRBYTE
        lda PTR
        jsr PRBYTE
        lda #':' | $80
        jsr ECHO
        ldy #0
byte:   lda #' ' | $80
        jsr ECHO
        lda (PTR),y
        jsr PRBYTE      ; PRBYTE pierde A pero conserva X e Y
        iny
        cpy #8
        bne byte
        clc             ; PTR += 8
        lda PTR
        adc #8
        sta PTR
        bcc :+
        inc PTR+1
:       dex
        bne line
        jmp GETLINE
