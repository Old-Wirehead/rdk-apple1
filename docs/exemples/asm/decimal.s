; decimal.s — affiche un entier 16 bits non signé en décimal
; (soustractions successives des puissances de 10, sans zéros de tête)
        .include "apple1.inc"
        .org $0300
NUM     = $06           ; nombre à afficher (2 octets)
FLAG    = $08           ; 1 dès qu'un chiffre non nul est sorti

start:  lda #CR
        jsr ECHO
        ldx #0
@liste: lda nombres,x   ; affiche chaque nombre de la liste
        sta NUM
        lda nombres+1,x
        sta NUM+1
        jsr prdec
        lda #' ' | $80
        jsr ECHO
        inx
        inx
        cpx #fin_nombres - nombres
        bne @liste
        jmp GETLINE

; prdec : affiche NUM en décimal. Conserve X.
prdec:  txa
        pha
        lda #0
        sta FLAG
        ldx #0          ; index dans la table des puissances
@chif:  ldy #'0' | $80  ; Y = chiffre courant
@sous:  lda NUM         ; NUM - puissance
        sec
        sbc puiss,x
        pha
        lda NUM+1
        sbc puiss+1,x
        bcc @fini       ; négatif : chiffre trouvé
        sta NUM+1
        pla
        sta NUM
        iny
        bne @sous
@fini:  pla
        tya
        cmp #'0' | $80
        bne @affi
        ldy FLAG        ; zéro de tête ? on le saute
        beq @suiv
@affi:  jsr ECHO
        inc FLAG
@suiv:  inx
        inx
        cpx #8          ; 4 puissances : 10000 à 10
        bne @chif
        lda NUM         ; reste : les unités, toujours affichées
        ora #'0' | $80
        jsr ECHO
        pla
        tax
        rts

puiss:  .word 10000, 1000, 100, 10
nombres:
        .word 0, 7, 42, 1976, 6502, 65535
fin_nombres:
