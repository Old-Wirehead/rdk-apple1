; gen2-hires.s — tres bandas de color en HIRES (280 x 192)
; En cada bloque de 128 bytes de la página $2000, los bytes 0-39 van al tercio
; superior de la pantalla, 40-79 al central, 80-119 al inferior (120-127: huecos).
        .include "apple1.inc"
        .include "gen2.inc"
        .org $0C00
PTR     = $06

start:  lda #<$2000
        sta PTR
        lda #>$2000
        sta PTR+1
block:  ldy #0
        ldx #0          ; X = motivo: 0 arriba, 2 centro, 4 abajo
third:  lda #40
        sta $08
byte:   tya             ; ¿columna par o impar?
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
        clc             ; bloque siguiente (+128)
        lda PTR
        adc #$80
        sta PTR
        bcc block
        inc PTR+1
        lda PTR+1
        cmp #$40        ; fin de la página 1 ($2000-$3FFF)
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
; dos bytes por motivo (columna par, columna impar):
; violeta, verde, luego azul y naranja (bit 7 = 1: la otra paleta)
patterns: .byte $55, $2A        ; violeta
        .byte $2A, $55          ; verde
        .byte $D5, $AA          ; azul
