; jeu.s — guess the digit (0 to 9): a simple game loop.
; The randomness comes from how long the player takes to press a key.
        .include "apple1.inc"
        .org $0300
SECRET  = $06
COUNT   = $07

start:  ldx #<title
        ldy #>title
        jsr prmsg
        ldx #0          ; X spins while we wait: the seed
@wait:  inx
        lda KBDCR
        bpl @wait
        lda KBD
        txa
@mod:   cmp #10         ; X modulo 10
        bcc @ok
        sbc #10         ; C = 1 here
        bcs @mod
@ok:    sta SECRET
        lda #0
        sta COUNT

turn:   ldx #<question
        ldy #>question
        jsr prmsg
        jsr getdig      ; A = digit 0..9 (other keys are ignored)
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
        ora #'0' | $80  ; < 10 tries: a single digit
        cmp #'9' | $80 + 1
        bcc :+
        lda #'+' | $80
:       jsr ECHO
        ldx #<tries
        ldy #>tries
        jsr prmsg
        jmp GETLINE

; getdig: waits for a digit, echoes it, returns it in binary in A
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

; prmsg: prints the zero-terminated string whose address is in Y:X
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
          .asciiz "I PICKED A DIGIT. PRESS ANY KEY."
question: .byte CR
          .asciiz "YOUR DIGIT? "
higher:   .asciiz " HIGHER."
lower:    .asciiz " LOWER."
bravo:    .byte " WON IN ", 0
tries:    .asciiz " TRIES!"
