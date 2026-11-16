; hexa.s — dump da memória ao estilo do monitor, com PRBYTE:
; 2 linhas de 8 bytes a partir de $FF00
        .include "apple1.inc"
        .org $0300
PTR     = $06           ; ponteiro na página zero (não usado pelo monitor)

start:  lda #<$FF00
        sta PTR
        lda #>$FF00
        sta PTR+1
        ldx #2          ; número de linhas
line:   lda #CR
        jsr ECHO
        lda PTR+1       ; endereço: byte alto e depois byte baixo
        jsr PRBYTE
        lda PTR
        jsr PRBYTE
        lda #':' | $80
        jsr ECHO
        ldy #0
byte:   lda #' ' | $80
        jsr ECHO
        lda (PTR),y
        jsr PRBYTE      ; PRBYTE perde A mas conserva X e Y
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
