; tunnel.s — TUNNEL : guidez la voiture dans un tunnel qui défile et rétrécit.
; Claude IA, 2026. Jeu en temps réel pour l'écran de l'Apple-1, qui ne sait
; qu'ajouter des lignes en bas : chaque ligne affichée est une tranche du
; tunnel. J = gauche, L = droite. Le score compte les lignes parcourues.
        .include "apple1.inc"
        .org $0300

GAUCHE  = $10           ; colonne du mur de gauche
LARGE   = $11           ; largeur intérieure du tunnel
VOIT    = $12           ; colonne de la voiture
GRAINE  = $13           ; 2 octets : générateur pseudo-aléatoire
SCORE   = $15           ; 2 octets en BCD (décimal)
COMPTE  = $17           ; lignes depuis le dernier rétrécissement
DROITE  = $18           ; colonne du mur de droite
FIN     = $19           ; dernière colonne à afficher
MSG     = $08           ; pointeur de message (2 octets)

debut:  cld
        ldx #<titre
        ldy #>titre
        jsr prmsg
        lda #1
        sta GRAINE
@att:   inc GRAINE+1    ; la graine change pendant l'attente
        bne :+
        inc GRAINE
:       lda KBDCR
        bpl @att
        lda KBD
        lda GRAINE
        ora GRAINE+1
        bne partie
        inc GRAINE      ; jamais zéro

partie: lda #12
        sta GAUCHE
        lda #14
        sta LARGE
        lda #19
        sta VOIT
        lda #0
        sta SCORE
        sta SCORE+1
        sta COMPTE
        lda #CR
        jsr ECHO

boucle: lda KBDCR       ; une touche ?
        bpl @mur
        lda KBD
        cmp #'J' | $80
        bne :+
        dec VOIT
:       cmp #'L' | $80
        bne @mur
        inc VOIT
@mur:   jsr hasard      ; le tunnel tourne au hasard
        and #3
        bne @adroite
        lda GAUCHE      ; 0 : vers la gauche
        cmp #2
        bcc @retr
        dec GAUCHE
        jmp @retr
@adroite:
        cmp #1          ; 1 : vers la droite (2 et 3 : tout droit)
        bne @retr
        lda GAUCHE
        clc
        adc LARGE
        cmp #36         ; le mur de droite reste avant la colonne 38
        bcs @retr
        inc GAUCHE
@retr:  inc COMPTE      ; toutes les 30 lignes, le tunnel rétrécit
        lda COMPTE
        cmp #30
        bne @trace
        lda #0
        sta COMPTE
        lda LARGE
        cmp #5
        bcc @trace
        dec LARGE
@trace: jsr ligne
        lda VOIT        ; contre le mur de gauche ?
        cmp GAUCHE
        beq crash
        bcc crash
        cmp DROITE      ; contre le mur de droite ?
        bcs crash
        sed             ; score + 1, en décimal
        lda SCORE
        clc
        adc #1
        sta SCORE
        lda SCORE+1
        adc #0
        sta SCORE+1
        cld
        jsr pause
        jmp boucle

crash:  ldx #<boum
        ldy #>boum
        jsr prmsg
        lda SCORE+1
        jsr PRBYTE
        lda SCORE
        jsr PRBYTE
        ldx #<encore
        ldy #>encore
        jsr prmsg
        lda KBD         ; oublie la dernière touche du jeu
@k:     lda KBDCR
        bpl @k
        lda KBD
        cmp #'O' | $80
        bne :+
        jmp partie
:       jmp GETLINE     ; retour au Woz Monitor

; ligne : affiche une tranche du tunnel, avec la voiture
ligne:  lda GAUCHE
        clc
        adc LARGE
        adc #1
        sta DROITE
        cmp VOIT
        bcs @fin        ; la voiture est avant le mur de droite
        lda VOIT
        cmp #40
        bcc @fin        ; voiture sortie à droite : on la montre
        lda DROITE
@fin:   sta FIN
        ldx #0
@col:   cpx VOIT
        bne @pasvoit
        lda #'*' | $80
        bne @aff
@pasvoit:
        cpx GAUCHE
        beq @m
        cpx DROITE
        beq @m
        lda #' ' | $80
        bne @aff
@m:     lda #'!' | $80
@aff:   jsr ECHO
        inx
        cpx FIN
        bcc @col
        beq @col
        lda #CR
        jmp ECHO

; hasard : registre à décalage 16 bits (Galois), rend un octet dans A
hasard: lsr GRAINE+1
        ror GRAINE
        bcc :+
        lda GRAINE+1
        eor #$B4
        sta GRAINE+1
:       lda GRAINE
        rts

; pause : environ 50 ms, pour garder le jeu jouable en affichage rapide
pause:  ldx #40
@x:     ldy #0
@y:     dey
        bne @y
        dex
        bne @x
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

titre:  .byte CR
        .byte "TUNNEL - CLAUDE IA", CR
        .byte "GUIDEZ LA VOITURE * DANS LE TUNNEL.", CR
        .byte "J = GAUCHE, L = DROITE.", CR
        .byte "LE TUNNEL RETRECIT PEU A PEU...", CR
        .asciiz "APPUYEZ SUR UNE TOUCHE POUR PARTIR."
boum:   .byte CR
        .asciiz "CRASH ! SCORE : "
encore: .byte CR
        .asciiz "REJOUER (O/N) ? "
