; hexa.s — vidage mémoire façon moniteur avec PRBYTE :
; 2 lignes de 8 octets à partir de $FF00
        .include "apple1.inc"
        .org $0300
PTR     = $06           ; pointeur en page zéro (libre pour le moniteur)

start:  lda #<$FF00
        sta PTR
        lda #>$FF00
        sta PTR+1
        ldx #2          ; nombre de lignes
ligne:  lda #CR
        jsr ECHO
        lda PTR+1       ; adresse : octet fort puis faible
        jsr PRBYTE
        lda PTR
        jsr PRBYTE
        lda #':' | $80
        jsr ECHO
        ldy #0
octet:  lda #' ' | $80
        jsr ECHO
        lda (PTR),y
        jsr PRBYTE      ; PRBYTE détruit A mais garde X et Y
        iny
        cpy #8
        bne octet
        clc             ; PTR += 8
        lda PTR
        adc #8
        sta PTR
        bcc :+
        inc PTR+1
:       dex
        bne ligne
        jmp GETLINE
