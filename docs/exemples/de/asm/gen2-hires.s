; gen2-hires.s — drei Farbbänder in HIRES (280 x 192)
; In jedem 128-Byte-Block der Seite $2000 gehen die Bytes 0-39 ins obere
; Drittel des Bildschirms, 40-79 ins mittlere, 80-119 ins untere (120-127: Lücken).
        .include "apple1.inc"
        .include "gen2.inc"
        .org $0C00
PTR     = $06

start:  lda #<$2000
        sta PTR
        lda #>$2000
        sta PTR+1
block:  ldy #0
        ldx #0          ; X = Muster: 0 oben, 2 Mitte, 4 unten
third:  lda #40
        sta $08
byte:   tya             ; gerade oder ungerade Spalte?
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
        clc             ; nächster Block (+128)
        lda PTR
        adc #$80
        sta PTR
        bcc block
        inc PTR+1
        lda PTR+1
        cmp #$40        ; Ende von Seite 1 ($2000-$3FFF)
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
; zwei Bytes je Muster (gerade Spalte, ungerade Spalte):
; Violett, Grün, dann Blau und Orange (Bit 7 = 1: die andere Palette)
patterns: .byte $55, $2A        ; Violett
        .byte $2A, $55          ; Grün
        .byte $D5, $AA          ; Blau
