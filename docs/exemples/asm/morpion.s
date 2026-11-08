; morpion.s — MORPION : le joueur (X) contre l'Apple-1 (O).
; Claude IA, 2026. Les cases sont numérotées comme le pavé :
;    1 ! 2 ! 3
;   ---+---+---
;    4 ! 5 ! 6      tapez le chiffre de la case où vous jouez.
;   ---+---+---
;    7 ! 8 ! 9
; L'ordinateur gagne s'il le peut, sinon bloque, sinon prend le centre ou
; un coin au hasard. Il ne voit pas les fourchettes : il peut être battu.
        .include "apple1.inc"
        .org $0300

QUI     = $10           ; pion cherché : 1 = X (joueur), 2 = O (ordinateur)
NB      = $11
VIDE    = $12
CPT     = $13
COUPS   = $14           ; coups joués dans la partie
GRAINE  = $15           ; 2 octets
SAUVX   = $17
MSG     = $08

debut:  cld
        ldx #<titre
        ldy #>titre
        jsr prmsg
        lda #1
        sta GRAINE

nouvelle:
        ldx #8          ; plateau vide
        lda #0
:       sta B,x
        dex
        bpl :-
        sta COUPS
        ldx #<qcommence
        ldy #>qcommence
        jsr prmsg
        jsr touche      ; la graine change pendant l'attente
        jsr ECHO
        cmp #'N' | $80
        beq ordi

joueur: jsr plateau
        ldx #<qcase
        ldy #>qcase
        jsr prmsg
@k:     jsr touche
        cmp #'1' | $80
        bcc @k
        cmp #('9' | $80) + 1
        bcs @k
        jsr ECHO
        and #$0F
        tax
        dex             ; case 0..8
        lda B,x
        beq @libre
        ldx #<occupee
        ldy #>occupee
        jsr prmsg
        jmp joueur
@libre: lda #1
        sta B,x
        inc COUPS
        sta QUI
        jsr aligne
        bcc @nul
        jsr plateau
        ldx #<gagne
        ldy #>gagne
        jsr prmsg
        jmp fin
@nul:   lda COUPS
        cmp #9
        bne ordi
        jmp nul

ordi:   jsr choix       ; X = case choisie
        lda #2
        sta B,x
        inc COUPS
        stx SAUVX
        ldx #<jejoue
        ldy #>jejoue
        jsr prmsg
        lda SAUVX
        clc
        adc #'1' | $80
        jsr ECHO
        lda #2
        sta QUI
        jsr aligne
        bcc @nul
        jsr plateau
        ldx #<perdu
        ldy #>perdu
        jsr prmsg
        jmp fin
@nul:   lda COUPS
        cmp #9
        beq nul
        jmp joueur

nul:    jsr plateau
        ldx #<matchnul
        ldy #>matchnul
        jsr prmsg
fin:    ldx #<encore
        ldy #>encore
        jsr prmsg
        jsr touche
        jsr ECHO
        cmp #'O' | $80
        bne :+
        jmp nouvelle
:       jmp GETLINE     ; retour au Woz Monitor

; choix : le coup de l'ordinateur, rendu dans X
choix:  lda #2          ; 1. gagner
        sta QUI
        jsr deux
        bcc @prend
        lda #1          ; 2. bloquer le joueur
        sta QUI
        jsr deux
        bcc @prend
        ldx #4          ; 3. le centre
        lda B,x
        beq @fini
        jsr hasard      ; 4. un coin, en partant d'un coin au hasard
        and #3
        sta CPT
        lda #4
        sta NB
@coin:  ldx CPT
        lda coins,x
        tax
        lda B,x
        beq @fini
        inc CPT
        lda CPT
        and #3
        sta CPT
        dec NB
        bne @coin
        ldx #0          ; 5. la première case libre
@libre: lda B,x
        beq @fini
        inx
        cpx #9
        bne @libre
@fini:  rts
@prend: tax
        rts

; deux : une ligne avec deux pions QUI et une case vide ?
; Sortie : C = 0 et A = la case vide, ou C = 1 s'il n'y en a pas.
deux:   ldx #0          ; X parcourt la table des lignes, 3 par 3
@ligne: lda #0
        sta NB
        lda #$FF
        sta VIDE
        lda #3
        sta CPT
@case:  ldy lignes,x
        lda B,y
        bne @pion
        sty VIDE
        jmp @suite
@pion:  cmp QUI
        bne @suite
        inc NB
@suite: inx
        dec CPT
        bne @case
        lda NB
        cmp #2
        bne @autre
        lda VIDE
        bmi @autre
        clc
        rts
@autre: cpx #24
        bne @ligne
        sec
        rts

; aligne : trois pions QUI alignés ? Sortie : C = 1 si oui.
aligne: ldx #0
@ligne: ldy lignes,x
        lda B,y
        cmp QUI
        bne @non
        ldy lignes+1,x
        lda B,y
        cmp QUI
        bne @non
        ldy lignes+2,x
        lda B,y
        cmp QUI
        bne @non
        sec
        rts
@non:   inx
        inx
        inx
        cpx #24
        bne @ligne
        clc
        rts

; plateau : affiche la grille (chiffre pour une case libre)
plateau:
        lda #CR
        jsr ECHO
        ldx #0
@rang:  jsr espace
        jsr pion
        jsr barre
        jsr pion
        jsr barre
        jsr pion
        lda #CR
        jsr ECHO
        cpx #9
        beq @fin
        ldy #0
:       lda trait,y
        beq :+
        jsr ECHO
        iny
        bne :-
:       jmp @rang
@fin:   rts

; pion : affiche la case X et passe à la suivante
pion:   lda B,x
        bne :+
        txa
        clc
        adc #'1' | $80
        bne @aff
:       cmp #1
        bne :+
        lda #'X' | $80
        bne @aff
:       lda #'O' | $80
@aff:   jsr ECHO
        inx
        rts

barre:  jsr espace
        lda #'!' | $80
        jsr ECHO
espace: lda #' ' | $80
        jmp ECHO

; touche : attend une touche (la graine tourne pendant l'attente)
touche: inc GRAINE+1
        bne :+
        inc GRAINE
:       lda KBDCR
        bpl touche
        lda KBD
        rts

; hasard : registre à décalage 16 bits (Galois), rend un octet dans A
hasard: lda GRAINE
        ora GRAINE+1
        bne :+
        inc GRAINE
:       lsr GRAINE+1
        ror GRAINE
        bcc :+
        lda GRAINE+1
        eor #$B4
        sta GRAINE+1
:       lda GRAINE
        rts

; prmsg : affiche la chaîne terminée par 0 dont l'adresse est en Y:X
prmsg:  stx MSG
        sty MSG+1
        ldy #0
:       lda (MSG),y
        beq :+
        ora #$80
        jsr ECHO
        iny
        bne :-
:       rts

lignes: .byte 0,1,2, 3,4,5, 6,7,8, 0,3,6, 1,4,7, 2,5,8, 0,4,8, 2,4,6
coins:  .byte 0,2,6,8
trait:  .byte "---+---+---", CR, 0
titre:  .byte CR
        .byte "MORPION - CLAUDE IA", CR
        .byte "VOUS AVEZ LES X, L'APPLE-1 LES O.", CR
        .asciiz "ALIGNEZ TROIS PIONS POUR GAGNER."
qcommence:
        .byte CR, CR
        .asciiz "VOUS COMMENCEZ (O/N) ? "
qcase:  .asciiz "VOTRE CASE (1-9) ? "
occupee:
        .byte CR
        .asciiz "CETTE CASE EST PRISE."
jejoue: .byte CR
        .asciiz "JE JOUE EN "
gagne:  .asciiz "BRAVO, VOUS AVEZ GAGNE !"
perdu:  .asciiz "J'AI GAGNE !"
matchnul:
        .asciiz "MATCH NUL."
encore: .byte CR
        .asciiz "REJOUER (O/N) ? "
B:      .res 9          ; le plateau : 0 vide, 1 X, 2 O
