; jeu.s — devinez le chiffre (0 à 9) : boucle de jeu simple.
; Le hasard vient du temps que met le joueur à appuyer sur une touche.
        .include "apple1.inc"
        .org $0300
SECRET  = $06
COMPTE  = $07

start:  ldx #<titre
        ldy #>titre
        jsr prmsg
        ldx #0          ; X tourne pendant l'attente : graine
@att:   inx
        lda KBDCR
        bpl @att
        lda KBD
        txa
@mod:   cmp #10         ; X modulo 10
        bcc @ok
        sbc #10         ; C = 1 ici
        bcs @mod
@ok:    sta SECRET
        lda #0
        sta COMPTE

tour:   ldx #<question
        ldy #>question
        jsr prmsg
        jsr touche      ; A = chiffre 0..9 (les autres touches sont ignorées)
        inc COMPTE
        cmp SECRET
        beq gagne
        bcc @plus
        ldx #<moins
        ldy #>moins
        jsr prmsg
        jmp tour
@plus:  ldx #<plus
        ldy #>plus
        jsr prmsg
        jmp tour

gagne:  ldx #<bravo
        ldy #>bravo
        jsr prmsg
        lda COMPTE
        ora #'0' | $80  ; < 10 coups : un seul chiffre
        cmp #'9' | $80 + 1
        bcc :+
        lda #'+' | $80
:       jsr ECHO
        ldx #<coups
        ldy #>coups
        jsr prmsg
        jmp GETLINE

; touche : attend un chiffre, l'affiche, le rend en binaire dans A
touche: lda KBDCR
        bpl touche
        lda KBD
        cmp #'0' | $80
        bcc touche
        cmp #'9' | $80 + 1
        bcs touche
        jsr ECHO
        and #$0F
        rts

; prmsg : affiche la chaîne terminée par 0 dont l'adresse est en Y:X
MSG     = $08
prmsg:  stx MSG
        sty MSG+1
        ldy #0
:       lda (MSG),y
        beq :+
        jsr ECHO
        iny
        bne :-
:       rts

titre:    .byte CR
          .asciiz "J'AI CHOISI UN CHIFFRE. APPUYEZ SUR UNE TOUCHE."
question: .byte CR
          .asciiz "VOTRE CHIFFRE ? "
plus:     .asciiz " C'EST PLUS."
moins:    .asciiz " C'EST MOINS."
bravo:    .byte " GAGNE EN ", 0
coups:    .asciiz " COUPS !"
