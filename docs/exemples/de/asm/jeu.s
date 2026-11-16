; jeu.s — errate die Ziffer (0 bis 9): eine einfache Spielschleife.
; Der Zufall kommt daher, wie lange der Spieler bis zum Tastendruck braucht.
        .include "apple1.inc"
        .org $0300
SECRET  = $06
COUNT   = $07

start:  ldx #<title
        ldy #>title
        jsr prmsg
        ldx #0          ; X zählt, während wir warten: der Startwert
@wait:  inx
        lda KBDCR
        bpl @wait
        lda KBD
        txa
@mod:   cmp #10         ; X modulo 10
        bcc @ok
        sbc #10         ; hier ist C = 1
        bcs @mod
@ok:    sta SECRET
        lda #0
        sta COUNT

turn:   ldx #<question
        ldy #>question
        jsr prmsg
        jsr getdig      ; A = Ziffer 0..9 (andere Tasten werden ignoriert)
        inc COUNT
        cmp SECRET
        beq won
        bcc @higher
        ldx #<lower
        ldy #>lower
        jsr prmsg
        jmp turn
@higher:
        ldx #<higher
        ldy #>higher
        jsr prmsg
        jmp turn

won:    ldx #<bravo
        ldy #>bravo
        jsr prmsg
        lda COUNT
        ora #'0' | $80  ; < 10 Versuche: eine einzige Ziffer
        cmp #'9' | $80 + 1
        bcc :+
        lda #'+' | $80
:       jsr ECHO
        ldx #<tries
        ldy #>tries
        jsr prmsg
        jmp GETLINE

; getdig: wartet auf eine Ziffer, gibt sie aus und liefert sie binär in A
getdig: lda KBDCR
        bpl getdig
        lda KBD
        cmp #'0' | $80
        bcc getdig
        cmp #'9' | $80 + 1
        bcs getdig
        jsr ECHO
        and #$0F
        rts

; prmsg: gibt die mit null abgeschlossene Zeichenkette aus, deren Adresse in Y:X steht
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

title:    .byte CR
          .asciiz "ICH HABE EINE ZIFFER. TASTE DRUECKEN."
question: .byte CR
          .asciiz "DEINE ZIFFER? "
higher:   .asciiz " HOEHER."
lower:    .asciiz " NIEDRIGER."
bravo:    .byte " GEWONNEN IN ", 0
tries:    .asciiz " VERSUCHEN!"
