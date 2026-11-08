; plab-couleurs.s — colour pattern on the P-LAB graphics card (TMS9918A)
; You need the P-LAB card (the "P-LAB" toolbar button, or --plab).
; "Graphics I" mode (32 x 24 tiles); the card sits at $CC00 (data) / $CC01 (control).
        .include "apple1.inc"
        .org $0300

VDATA   = $CC00
VCTRL   = $CC01

start:  ldx #0
@regs:  lda regs,x          ; R0..R7
        sta VCTRL
        txa
        ora #$80
        sta VCTRL
        inx
        cpx #8
        bne @regs

        ; pattern table ($0000): 256 solid tiles (2 KB of $FF)
        lda #$00
        ldy #$40
        jsr vadr
        ldy #8              ; 8 pages of 256 bytes
@pm:    ldx #0
@pl:    lda #$FF
        sta VDATA
        inx
        bne @pl
        dey
        bne @pm

        ; colour table ($2000): 32 groups of 8 tiles
        lda #$00
        ldy #$60
        jsr vadr
        ldx #0
@col:   txa
        asl a
        asl a
        asl a
        asl a               ; foreground colour = X (0..15), then it wraps
        ora #$01            ; black background
        sta VDATA
        inx
        cpx #32
        bne @col

        ; name table ($1400): 24 rows of 32 tiles, shifted diagonally
        lda #$00
        ldy #$54
        jsr vadr
        ldy #0              ; Y = row
@row:   ldx #0
@nm:    tya
        asl a
        asl a               ; row x 4
        stx $00
        clc
        adc $00             ; + column
        sta VDATA
        inx
        cpx #32
        bne @nm
        iny
        cpy #24
        bne @row

        ; sprite table ($1B00): no sprites ($D0 = end of list)
        lda #$00
        ldy #$5B
        jsr vadr
        lda #$D0
        sta VDATA

        jmp GETLINE         ; back to the Woz Monitor; the picture stays on

; set the video address: A = low byte, Y = high byte (+ $40 to write)
vadr:   sta VCTRL
        sty VCTRL
        rts

regs:   .byte $00           ; R0: Graphics I mode
        .byte $C0           ; R1: 16 KB, display on, no interrupt
        .byte $05           ; R2: names at $1400
        .byte $80           ; R3: colours at $2000
        .byte $00           ; R4: patterns at $0000
        .byte $36           ; R5: sprites at $1B00
        .byte $07           ; R6: sprite patterns at $3800
        .byte $04           ; R7: dark blue border
