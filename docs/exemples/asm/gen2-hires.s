; gen2-hires.s — trois bandes de couleur en HIRES (280 x 192)
; Dans chaque bloc de 128 octets de la page $2000, les octets 0-39 vont dans
; le tiers haut de l'écran, 40-79 au milieu, 80-119 en bas (120-127 : trous).
        .include "apple1.inc"
        .include "gen2.inc"
        .org $0C00
PTR     = $06

start:  lda #<$2000
        sta PTR
        lda #>$2000
        sta PTR+1
bloc:   ldy #0
        ldx #0          ; X = motif : 0 haut, 2 milieu, 4 bas
tiers:  lda #40
        sta $08
octet:  tya             ; colonne paire ou impaire ?
        and #1
        ora motif_idx,x
        sty $09
        tay
        lda motifs,y
        ldy $09
        sta (PTR),y
        iny
        dec $08
        bne octet
        inx
        cpx #3
        bne tiers
        clc             ; bloc suivant (+128)
        lda PTR
        adc #$80
        sta PTR
        bcc bloc
        inc PTR+1
        lda PTR+1
        cmp #$40        ; fin de la page 1 ($2000-$3FFF)
        bne bloc

        bit SS_HIRES
        bit SS_FULL
        bit SS_PAGE1
        bit SS_GR
attente:
        lda KBDCR
        bpl attente
        lda KBD
        bit SS_TEXT
        jmp GETLINE

motif_idx: .byte 0, 2, 4
; deux octets par motif (colonne paire, colonne impaire) :
; violet, vert, puis bleu et orange (bit 7 = 1 : l'autre palette)
motifs: .byte $55, $2A          ; violet
        .byte $2A, $55          ; vert
        .byte $D5, $AA          ; bleu
