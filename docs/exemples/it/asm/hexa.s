; hexa.s — dump della memoria come nel monitor, con PRBYTE:
; 2 righe di 8 byte a partire da $FF00
        .include "apple1.inc"
        .org $0300
PTR     = $06           ; puntatore in pagina zero (non usato dal monitor)

start:  lda #<$FF00
        sta PTR
        lda #>$FF00
        sta PTR+1
        ldx #2          ; numero di righe
line:   lda #CR
        jsr ECHO
        lda PTR+1       ; indirizzo: byte alto, poi byte basso
        jsr PRBYTE
        lda PTR
        jsr PRBYTE
        lda #':' | $80
        jsr ECHO
        ldy #0
byte:   lda #' ' | $80
        jsr ECHO
        lda (PTR),y
        jsr PRBYTE      ; PRBYTE distrugge A ma conserva X e Y
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
