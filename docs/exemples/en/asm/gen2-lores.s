; gen2-lores.s — color bars in LORES (40 x 48, 16 colors)
; Needs the GEN2 card (Settings ▸ Color card). Any key: back to text.
        .include "apple1.inc"
        .include "gen2.inc"
        .org $0C00      ; above LORES page 1 ($0400-$07FF)
PTR     = $06

start:  ldx #0          ; X = text row (0..23)
row:    lda rows_lo,x
        sta PTR
        lda rows_hi,x
        sta PTR+1
        ldy #39
column:
        tya             ; color = column / 2 (0..15, then again)
        lsr a
        and #$0F
        sta $00         ; one byte = 2 blocks: low nibble on top, high below
        asl a
        asl a
        asl a
        asl a
        ora $00
        sta (PTR),y
        dey
        bpl column
        inx
        cpx #24
        bne row

        bit SS_LORES    ; LORES, full screen, page 1, then graphics
        bit SS_FULL
        bit SS_PAGE1
        bit SS_GR
wait:
        lda KBDCR
        bpl wait
        lda KBD
        bit SS_TEXT
        jmp GETLINE

; addresses of the 24 rows: $400 + (r & 7) * $80 + (r >> 3) * $28
rows_lo:
.repeat 24, R
        .byte <($0400 + (R & 7) * $80 + (R >> 3) * $28)
.endrep
rows_hi:
.repeat 24, R
        .byte >($0400 + (R & 7) * $80 + (R >> 3) * $28)
.endrep
