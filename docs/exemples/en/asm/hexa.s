; hexa.s — monitor-style memory dump with PRBYTE:
; 2 lines of 8 bytes starting at $FF00
        .include "apple1.inc"
        .org $0300
PTR     = $06           ; zero-page pointer (not used by the monitor)

start:  lda #<$FF00
        sta PTR
        lda #>$FF00
        sta PTR+1
        ldx #2          ; number of lines
line:   lda #CR
        jsr ECHO
        lda PTR+1       ; address: high byte, then low byte
        jsr PRBYTE
        lda PTR
        jsr PRBYTE
        lda #':' | $80
        jsr ECHO
        ldy #0
byte:   lda #' ' | $80
        jsr ECHO
        lda (PTR),y
        jsr PRBYTE      ; PRBYTE destroys A but keeps X and Y
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
