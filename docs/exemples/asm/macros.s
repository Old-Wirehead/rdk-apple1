; macros.s — macros, .repeat et assemblage conditionnel
        .include "apple1.inc"
        .org $0300

LARGEUR = 20            ; essayez 10 ou 40
.define CARACTERE '*'

; afficher un caractère constant (la macro produit lda/jsr)
.macro putc c
        lda #(c) | $80
        jsr ECHO
.endmacro

; afficher une chaîne : un label local à chaque appel
.macro print texte
        .local boucle, fin, chaine
        ldx #0
boucle: lda chaine,x
        beq fin
        jsr ECHO
        inx
        bne boucle
        beq fin
chaine: .asciiz texte
fin:
.endmacro

start:  putc 13
        print "MACROS ET .REPEAT"
        putc 13
.repeat 3               ; trois lignes d'étoiles, déroulées à l'assemblage
        ldx #LARGEUR
:       putc CARACTERE
        dex
        bne :-
        putc 13
.endrep
.if LARGEUR > 30
        print "LIGNES LONGUES"
.else
        print "LIGNES COURTES"
.endif
        jmp GETLINE

; une table calculée par l'assembleur : les carrés de 0 à 15
carres:
.repeat 16, I
        .byte I * I
.endrep
