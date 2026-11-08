; gen2-hires.s — three color bands in HIRES (280 x 192)
; In each 128-byte block of the $2000 page, bytes 0-39 go to the top
; third of the screen, 40-79 to the middle, 80-119 to the bottom (120-127: holes).
        .include "apple1.inc"
        .include "gen2.inc"
        .org $0C00
PTR     = $06

start:  lda #<$2000
        sta PTR
        lda #>$2000
        sta PTR+1
block:  ldy #0
        ldx #0          ; X = pattern: 0 top, 2 middle, 4 bottom
third:  lda #40
        sta $08
byte:   tya             ; even or odd column?
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
        clc             ; next block (+128)
        lda PTR
        adc #$80
        sta PTR
        bcc block
        inc PTR+1
        lda PTR+1
        cmp #$40        ; end of page 1 ($2000-$3FFF)
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
; two bytes per pattern (even column, odd column):
; violet, green, then blue and orange (bit 7 = 1: the other palette)
patterns: .byte $55, $2A        ; violet
        .byte $2A, $55          ; green
        .byte $D5, $AA          ; blue
