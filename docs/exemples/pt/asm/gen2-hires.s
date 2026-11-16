; gen2-hires.s — três faixas de cor em HIRES (280 x 192)
; Em cada bloco de 128 bytes da página $2000, os bytes 0-39 vão para o terço
; superior do ecrã, 40-79 para o do meio, 80-119 para o de baixo (120-127: buracos).
        .include "apple1.inc"
        .include "gen2.inc"
        .org $0C00
PTR     = $06

start:  lda #<$2000
        sta PTR
        lda #>$2000
        sta PTR+1
block:  ldy #0
        ldx #0          ; X = padrão: 0 em cima, 2 no meio, 4 em baixo
third:  lda #40
        sta $08
byte:   tya             ; coluna par ou ímpar?
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
        clc             ; bloco seguinte (+128)
        lda PTR
        adc #$80
        sta PTR
        bcc block
        inc PTR+1
        lda PTR+1
        cmp #$40        ; fim da página 1 ($2000-$3FFF)
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
; dois bytes por padrão (coluna par, coluna ímpar):
; violeta, verde, depois azul e laranja (bit 7 = 1: a outra paleta)
patterns: .byte $55, $2A        ; violeta
        .byte $2A, $55          ; verde
        .byte $D5, $AA          ; azul
