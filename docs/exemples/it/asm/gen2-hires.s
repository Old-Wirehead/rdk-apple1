; gen2-hires.s — tre bande di colore in HIRES (280 x 192)
; In ogni blocco di 128 byte della pagina $2000, i byte 0-39 vanno nel terzo
; superiore dello schermo, 40-79 in quello centrale, 80-119 in basso (120-127: buchi).
        .include "apple1.inc"
        .include "gen2.inc"
        .org $0C00
PTR     = $06

start:  lda #<$2000
        sta PTR
        lda #>$2000
        sta PTR+1
block:  ldy #0
        ldx #0          ; X = motivo: 0 in alto, 2 al centro, 4 in basso
third:  lda #40
        sta $08
byte:   tya             ; colonna pari o dispari?
        and #1
        ora pattern_idx,x
        sty $09
        tay
        lda patterns,y
        ldy $09
        sta (PTR),y
        iny
        dec $08
        bne byte
        inx
        cpx #3
        bne third
        clc             ; blocco seguente (+128)
        lda PTR
        adc #$80
        sta PTR
        bcc block
        inc PTR+1
        lda PTR+1
        cmp #$40        ; fine della pagina 1 ($2000-$3FFF)
        bne block

        bit SS_HIRES
        bit SS_FULL
        bit SS_PAGE1
        bit SS_GR
wait:
        lda KBDCR
        bpl wait
        lda KBD
        bit SS_TEXT
        jmp GETLINE

pattern_idx: .byte 0, 2, 4
; due byte per motivo (colonna pari, colonna dispari):
; viola, verde, poi blu e arancione (bit 7 = 1: l'altra tavolozza)
patterns: .byte $55, $2A        ; viola
        .byte $2A, $55          ; verde
        .byte $D5, $AA          ; blu
