; tunnel.s - TUNNEL: steer the car through a scrolling, narrowing tunnel.
; Claude IA, 2026. A real-time game for the Apple-1 screen, which can only
; add lines at the bottom: each line printed is a slice of the tunnel.
; J = left, L = right. The score counts the lines travelled.
        .include "apple1.inc"
        .org $0300

LEFTW   = $10           ; column of the left wall
WIDTH   = $11           ; inside width of the tunnel
CAR     = $12           ; column of the car
SEED    = $13           ; 2 bytes: pseudo-random generator
SCORE   = $15           ; 2 bytes in BCD (decimal)
COUNT   = $17           ; lines since the tunnel last narrowed
RIGHTW  = $18           ; column of the right wall
LAST    = $19           ; last column to print
MSG     = $08           ; message pointer (2 bytes)

start:  cld
        ldx #<title
        ldy #>title
        jsr prmsg
        lda #1
        sta SEED
@wait:  inc SEED+1      ; the seed changes while we wait
        bne :+
        inc SEED
:       lda KBDCR
        bpl @wait
        lda KBD
        lda SEED
        ora SEED+1
        bne game
        inc SEED        ; never zero

game:   lda #12
        sta LEFTW
        lda #14
        sta WIDTH
        lda #19
        sta CAR
        lda #0
        sta SCORE
        sta SCORE+1
        sta COUNT
        lda #CR
        jsr ECHO

loop:   lda KBDCR       ; a key?
        bpl @wall
        lda KBD
        cmp #'J' | $80
        bne :+
        dec CAR
:       cmp #'L' | $80
        bne @wall
        inc CAR
@wall:  jsr random      ; the tunnel turns at random
        and #3
        bne @right
        lda LEFTW       ; 0: to the left
        cmp #2
        bcc @narrow
        dec LEFTW
        jmp @narrow
@right: cmp #1          ; 1: to the right (2 and 3: straight on)
        bne @narrow
        lda LEFTW
        clc
        adc WIDTH
        cmp #36         ; the right wall stays before column 38
        bcs @narrow
        inc LEFTW
@narrow:
        inc COUNT       ; every 30 lines, the tunnel gets narrower
        lda COUNT
        cmp #30
        bne @draw
        lda #0
        sta COUNT
        lda WIDTH
        cmp #5
        bcc @draw
        dec WIDTH
@draw:  jsr slice
        lda CAR         ; against the left wall?
        cmp LEFTW
        beq crash
        bcc crash
        cmp RIGHTW      ; against the right wall?
        bcs crash
        sed             ; score + 1, in decimal
        lda SCORE
        clc
        adc #1
        sta SCORE
        lda SCORE+1
        adc #0
        sta SCORE+1
        cld
        jsr pause
        jmp loop

crash:  ldx #<bang
        ldy #>bang
        jsr prmsg
        lda SCORE+1
        jsr PRBYTE
        lda SCORE
        jsr PRBYTE
        ldx #<again
        ldy #>again
        jsr prmsg
        lda KBD         ; forget the last key of the game
@k:     lda KBDCR
        bpl @k
        lda KBD
        cmp #'Y' | $80
        bne :+
        jmp game
:       jmp GETLINE     ; back to the Woz Monitor

; slice: print one slice of the tunnel, with the car
slice:  lda LEFTW
        clc
        adc WIDTH
        adc #1
        sta RIGHTW
        cmp CAR
        bcs @end        ; the car is before the right wall
        lda CAR
        cmp #40
        bcc @end        ; car out on the right: show it
        lda RIGHTW
@end:   sta LAST
        ldx #0
@col:   cpx CAR
        bne @nocar
        lda #'*' | $80
        bne @out
@nocar: cpx LEFTW
        beq @w
        cpx RIGHTW
        beq @w
        lda #' ' | $80
        bne @out
@w:     lda #'!' | $80
@out:   jsr ECHO
        inx
        cpx LAST
        bcc @col
        beq @col
        lda #CR
        jmp ECHO

; random: 16-bit Galois shift register, returns a byte in A
random: lsr SEED+1
        ror SEED
        bcc :+
        lda SEED+1
        eor #$B4
        sta SEED+1
:       lda SEED
        rts

; pause: about 50 ms, to keep the game playable with fast display
pause:  ldx #40
@x:     ldy #0
@y:     dey
        bne @y
        dex
        bne @x
        rts

; prmsg: print the zero-terminated string whose address is in Y:X
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

title:  .byte CR
        .byte "TUNNEL - CLAUDE IA", CR
        .byte "STEER THE CAR * THROUGH THE TUNNEL.", CR
        .byte "J = LEFT, L = RIGHT.", CR
        .byte "THE TUNNEL SLOWLY GETS NARROWER...", CR
        .asciiz "PRESS A KEY TO START."
bang:   .byte CR
        .asciiz "CRASH! SCORE: "
again:  .byte CR
        .asciiz "PLAY AGAIN (Y/N)? "
