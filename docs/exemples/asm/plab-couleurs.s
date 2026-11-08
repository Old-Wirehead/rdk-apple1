; plab-couleurs.s — motif de couleurs sur la carte graphique P-LAB (TMS9918A)
; Il faut la carte P-LAB (bouton « P-LAB » de la barre, ou --plab).
; Mode « Graphics I » (32 x 24 tuiles) ; la carte est en $CC00 (données) / $CC01 (contrôle).
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

        ; table des motifs ($0000) : 256 tuiles pleines (2 Ko de $FF)
        lda #$00
        ldy #$40
        jsr vadr
        ldy #8              ; 8 pages de 256 octets
@pm:    ldx #0
@pl:    lda #$FF
        sta VDATA
        inx
        bne @pl
        dey
        bne @pm

        ; table des couleurs ($2000) : 32 groupes de 8 tuiles
        lda #$00
        ldy #$60
        jsr vadr
        ldx #0
@col:   txa
        asl a
        asl a
        asl a
        asl a               ; couleur d'avant-plan = X (0..15), puis on recommence
        ora #$01            ; fond noir
        sta VDATA
        inx
        cpx #32
        bne @col

        ; table des noms ($1400) : 24 rangées de 32 tuiles, décalées en diagonale
        lda #$00
        ldy #$54
        jsr vadr
        ldy #0              ; Y = rangée
@row:   ldx #0
@nm:    tya
        asl a
        asl a               ; rangée x 4
        stx $00
        clc
        adc $00             ; + colonne
        sta VDATA
        inx
        cpx #32
        bne @nm
        iny
        cpy #24
        bne @row

        ; table des sprites ($1B00) : aucun sprite ($D0 = fin de liste)
        lda #$00
        ldy #$5B
        jsr vadr
        lda #$D0
        sta VDATA

        jmp GETLINE         ; retour au Woz Monitor ; l'image reste affichée

; positionne l'adresse vidéo : A = octet bas, Y = octet haut (+ $40 pour écrire)
vadr:   sta VCTRL
        sty VCTRL
        rts

regs:   .byte $00           ; R0 : mode Graphics I
        .byte $C0           ; R1 : 16 Ko, image affichée, pas d'interruption
        .byte $05           ; R2 : noms en $1400
        .byte $80           ; R3 : couleurs en $2000
        .byte $00           ; R4 : motifs en $0000
        .byte $36           ; R5 : sprites en $1B00
        .byte $07           ; R6 : sprites en $3800
        .byte $04           ; R7 : bordure bleu foncé
