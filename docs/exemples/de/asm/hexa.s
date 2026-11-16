; hexa.s — Speicherauszug wie im Monitor, mit PRBYTE:
; 2 Zeilen zu 8 Bytes ab $FF00
        .include "apple1.inc"
        .org $0300
PTR     = $06           ; Zeiger in der Zero Page (vom Monitor nicht benutzt)

start:  lda #<$FF00
        sta PTR
        lda #>$FF00
        sta PTR+1
        ldx #2          ; Anzahl der Zeilen
line:   lda #CR
        jsr ECHO
        lda PTR+1       ; Adresse: oberes Byte, dann unteres Byte
        jsr PRBYTE
        lda PTR
        jsr PRBYTE
        lda #':' | $80
        jsr ECHO
        ldy #0
byte:   lda #' ' | $80
        jsr ECHO
        lda (PTR),y
        jsr PRBYTE      ; PRBYTE zerstört A, behält aber X und Y
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
