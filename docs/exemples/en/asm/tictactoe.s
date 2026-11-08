; tictactoe.s - TIC-TAC-TOE: the player (X) against the Apple-1 (O).
; Claude IA, 2026. The squares are numbered like this:
;    1 ! 2 ! 3
;   ---+---+---
;    4 ! 5 ! 6      type the digit of the square you play.
;   ---+---+---
;    7 ! 8 ! 9
; The computer wins if it can, otherwise blocks, otherwise takes the centre
; or a random corner. It does not see forks: it can be beaten.
        .include "apple1.inc"
        .org $0300

WHO     = $10           ; piece looked for: 1 = X (player), 2 = O (computer)
NB      = $11
EMPTY   = $12
CNT     = $13
MOVES   = $14           ; moves played in this game
SEED    = $15           ; 2 bytes
NOTE    = $17           ; computer's last square (1-9), 0 = none
TAKEN   = $18           ; 1 = say "that square is taken"
MSG     = $08

start:  cld
        ldx #<title
        ldy #>title
        jsr prmsg
        lda #1
        sta SEED

newgame:
        ldx #8          ; empty board
        lda #0
:       sta B,x
        dex
        bpl :-
        sta MOVES
        sta NOTE
        sta TAKEN
        ldx #<qstart
        ldy #>qstart
        jsr prmsg
        jsr getkey      ; the seed changes while we wait
        jsr ECHO
        cmp #'N' | $80
        beq cpu

player: jsr board
        ldx #<qsquare
        ldy #>qsquare
        jsr prmsg
@k:     jsr getkey
        cmp #'1' | $80
        bcc @k
        cmp #('9' | $80) + 1
        bcs @k
        jsr ECHO
        and #$0F
        tax
        dex             ; square 0..8
        lda B,x
        beq @free
        lda #1
        sta TAKEN
        jmp player
@free:  lda #1
        sta B,x
        inc MOVES
        sta WHO
        lda #0
        sta NOTE
        jsr three
        bcc @draw
        jsr board
        ldx #<youwin
        ldy #>youwin
        jsr prmsg
        jmp done
@draw:  lda MOVES
        cmp #9
        bne cpu
        jmp draw

cpu:    jsr choose      ; X = chosen square
        lda #2
        sta B,x
        inc MOVES
        inx             ; square 1..9, shown by board
        stx NOTE
        lda #2
        sta WHO
        jsr three
        bcc @draw
        jsr board
        ldx #<iwin
        ldy #>iwin
        jsr prmsg
        jmp done
@draw:  lda MOVES
        cmp #9
        beq draw
        jmp player

draw:   jsr board
        ldx #<isdraw
        ldy #>isdraw
        jsr prmsg
done:   ldx #<again
        ldy #>again
        jsr prmsg
        jsr getkey
        jsr ECHO
        cmp #'Y' | $80
        bne :+
        jmp newgame
:       jmp GETLINE     ; back to the Woz Monitor

; choose: the computer's move, returned in X
choose: lda #2          ; 1. win
        sta WHO
        jsr two
        bcc @take
        lda #1          ; 2. block the player
        sta WHO
        jsr two
        bcc @take
        ldx #4          ; 3. the centre
        lda B,x
        beq @found
        jsr random      ; 4. a corner, starting from a random one
        and #3
        sta CNT
        lda #4
        sta NB
@corner:
        ldx CNT
        lda corners,x
        tax
        lda B,x
        beq @found
        inc CNT
        lda CNT
        and #3
        sta CNT
        dec NB
        bne @corner
        ldx #0          ; 5. the first free square
@free:  lda B,x
        beq @found
        inx
        cpx #9
        bne @free
@found: rts
@take:  tax
        rts

; two: a line with two WHO pieces and one empty square?
; Out: C = 0 and A = the empty square, or C = 1 if there is none.
two:    ldx #0          ; X walks the table of lines, 3 by 3
@line:  lda #0
        sta NB
        lda #$FF
        sta EMPTY
        lda #3
        sta CNT
@sq:    ldy lines,x
        lda B,y
        bne @piece
        sty EMPTY
        jmp @next
@piece: cmp WHO
        bne @next
        inc NB
@next:  inx
        dec CNT
        bne @sq
        lda NB
        cmp #2
        bne @other
        lda EMPTY
        bmi @other
        clc
        rts
@other: cpx #24
        bne @line
        sec
        rts

; three: three WHO pieces in a row? Out: C = 1 if yes.
three:  ldx #0
@line:  ldy lines,x
        lda B,y
        cmp WHO
        bne @no
        ldy lines+1,x
        lda B,y
        cmp WHO
        bne @no
        ldy lines+2,x
        lda B,y
        cmp WHO
        bne @no
        sec
        rts
@no:    inx
        inx
        inx
        cpx #24
        bne @line
        clc
        rts

; board: clear the screen, print the grid (a digit for a free square),
; then the computer's last move and the "taken" warning.
; The Apple-1 cannot clear its screen by software: 24 empty lines push
; the old turn off the top.
board:  ldx #24
@clr:   lda #CR
        jsr ECHO
        dex
        bne @clr
        ldx #<head
        ldy #>head
        jsr prmsg
        ldx #0
@row:   jsr space
        jsr piece
        jsr bar
        jsr piece
        jsr bar
        jsr piece
        lda #CR
        jsr ECHO
        cpx #9
        beq @notes
        ldy #0
:       lda rule,y
        beq :+
        jsr ECHO
        iny
        bne :-
:       jmp @row
@notes: lda #CR
        jsr ECHO
        lda NOTE        ; the computer's last move
        beq @nonote
        ldx #<iplay
        ldy #>iplay
        jsr prmsg
        lda NOTE
        ora #'0' | $80
        jsr ECHO
        lda #CR
        jsr ECHO
@nonote:
        lda TAKEN       ; the player chose a taken square
        beq @end
        ldx #<taken
        ldy #>taken
        jsr prmsg
        lda #CR
        jsr ECHO
        lda #0
        sta TAKEN
@end:   rts

; piece: print square X and move on to the next one
piece:  lda B,x
        bne :+
        txa
        clc
        adc #'1' | $80
        bne @out
:       cmp #1
        bne :+
        lda #'X' | $80
        bne @out
:       lda #'O' | $80
@out:   jsr ECHO
        inx
        rts

bar:    jsr space
        lda #'!' | $80
        jsr ECHO
space:  lda #' ' | $80
        jmp ECHO

; getkey: wait for a key (the seed turns while waiting)
getkey: inc SEED+1
        bne :+
        inc SEED
:       lda KBDCR
        bpl getkey
        lda KBD
        rts

; random: 16-bit Galois shift register, returns a byte in A
random: lda SEED
        ora SEED+1
        bne :+
        inc SEED
:       lsr SEED+1
        ror SEED
        bcc :+
        lda SEED+1
        eor #$B4
        sta SEED+1
:       lda SEED
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

lines:  .byte 0,1,2, 3,4,5, 6,7,8, 0,3,6, 1,4,7, 2,5,8, 0,4,8, 2,4,6
corners:
        .byte 0,2,6,8
rule:   .byte "---+---+---", CR, 0
title:  .byte CR
        .byte "TIC-TAC-TOE - CLAUDE IA", CR
        .byte "YOU ARE X, THE APPLE-1 IS O.", CR
        .asciiz "GET THREE IN A ROW TO WIN."
qstart: .byte CR, CR
        .asciiz "DO YOU START (Y/N)? "
qsquare:
        .asciiz "YOUR SQUARE (1-9)? "
head:   .byte "TIC-TAC-TOE - CLAUDE IA", CR, CR, 0
taken:  .asciiz "THAT SQUARE IS TAKEN."
iplay:  .asciiz "I PLAY "
youwin: .asciiz "WELL DONE, YOU WIN!"
iwin:   .asciiz "I WIN!"
isdraw: .asciiz "IT IS A DRAW."
again:  .byte CR
        .asciiz "PLAY AGAIN (Y/N)? "
B:      .res 9          ; the board: 0 empty, 1 X, 2 O
