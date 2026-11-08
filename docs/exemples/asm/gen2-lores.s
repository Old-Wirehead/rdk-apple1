; gen2-lores.s — barres de couleurs en LORES (40 x 48, 16 couleurs)
; Il faut la carte GEN2 (Paramètres ▸ Carte couleur). Une touche : retour texte.
        .include "apple1.inc"
        .include "gen2.inc"
        .org $0C00      ; au-dessus de la page LORES 1 ($0400-$07FF)
PTR     = $06

start:  ldx #0          ; X = rangée de texte (0..23)
rangee: lda lignes_lo,x
        sta PTR
        lda lignes_hi,x
        sta PTR+1
        ldy #39
colonne:
        tya             ; couleur = colonne / 2 (0..15, puis on recommence)
        lsr a
        and #$0F
        sta $00         ; un octet = 2 blocs : quartet bas en haut, haut en bas
        asl a
        asl a
        asl a
        asl a
        ora $00
        sta (PTR),y
        dey
        bpl colonne
        inx
        cpx #24
        bne rangee

        bit SS_LORES    ; LORES, plein écran, page 1, puis graphique
        bit SS_FULL
        bit SS_PAGE1
        bit SS_GR
attente:
        lda KBDCR
        bpl attente
        lda KBD
        bit SS_TEXT
        jmp GETLINE

; adresses des 24 rangées : $400 + (r & 7) * $80 + (r >> 3) * $28
lignes_lo:
.repeat 24, R
        .byte <($0400 + (R & 7) * $80 + (R >> 3) * $28)
.endrep
lignes_hi:
.repeat 24, R
        .byte >($0400 + (R & 7) * $80 + (R >> 3) * $28)
.endrep
