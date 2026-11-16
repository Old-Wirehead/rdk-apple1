; tictactoe.s - TRIS: il giocatore (X) contro l'Apple-1 (O).
; Claude IA, 2026. Le caselle sono numerate così:
;    1 ! 2 ! 3
;   ---+---+---
;    4 ! 5 ! 6      digitate la cifra della casella che giocate.
;   ---+---+---
;    7 ! 8 ! 9
; Il computer vince se può, altrimenti blocca, altrimenti prende il centro
; o un angolo a caso. Non vede le forchette: si può battere.
        .include "apple1.inc"
        .org $0300

WHO     = $10           ; pedina cercata: 1 = X (giocatore), 2 = O (computer)
NB      = $11
EMPTY   = $12
CNT     = $13
MOVES   = $14           ; mosse giocate in questa partita
SEED    = $15           ; 2 byte
NOTE    = $17           ; ultima casella del computer (1-9), 0 = nessuna
TAKEN   = $18           ; 1 = avvisare «casella occupata»
MSG     = $08

start:  cld
        ldx #<title
        ldy #>title
        jsr prmsg
        lda #1
        sta SEED

newgame:
        ldx #8          ; griglia vuota
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
        jsr getkey      ; il seme cambia mentre aspettiamo
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
        dex             ; casella 0..8
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

cpu:    jsr choose      ; X = casella scelta
        lda #2
        sta B,x
        inc MOVES
        inx             ; casella 1..9, mostrata da board
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
        cmp #'S' | $80
        bne :+
        jmp newgame
:       jmp GETLINE     ; ritorno al Woz Monitor

; choose: la mossa del computer, restituita in X
choose: lda #2          ; 1. vincere
        sta WHO
        jsr two
        bcc @take
        lda #1          ; 2. bloccare il giocatore
        sta WHO
        jsr two
        bcc @take
        ldx #4          ; 3. il centro
        lda B,x
        beq @found
        jsr random      ; 4. un angolo, partendo da uno a caso
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
        ldx #0          ; 5. la prima casella libera
@free:  lda B,x
        beq @found
        inx
        cpx #9
        bne @free
@found: rts
@take:  tax
        rts

; two: una linea con due pedine WHO e una casella vuota?
; Uscita: C = 0 e A = la casella vuota, oppure C = 1 se non ce n'è.
two:    ldx #0          ; X percorre la tabella delle linee, 3 alla volta
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

; three: tre pedine WHO in fila? Uscita: C = 1 se sì.
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

; board: pulisce lo schermo, stampa la griglia (una cifra per casella libera),
; poi l'ultima mossa del computer e l'avviso «occupata».
; L'Apple-1 non può pulire lo schermo via software: 24 righe vuote spingono
; il turno precedente fuori dall'alto.
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
        lda NOTE        ; l'ultima mossa del computer
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
        lda TAKEN       ; il giocatore ha scelto una casella occupata
        beq @end
        ldx #<taken
        ldy #>taken
        jsr prmsg
        lda #CR
        jsr ECHO
        lda #0
        sta TAKEN
@end:   rts

; piece: stampa la casella X e passa alla seguente
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

; getkey: attende un tasto (il seme gira nell'attesa)
getkey: inc SEED+1
        bne :+
        inc SEED
:       lda KBDCR
        bpl getkey
        lda KBD
        rts

; random: registro a scorrimento di Galois a 16 bit, restituisce un byte in A
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

; prmsg: stampa la stringa terminata da zero il cui indirizzo è in Y:X
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
        .byte "TRIS - CLAUDE IA", CR
        .byte "TU SEI X, L'APPLE-1 E O.", CR
        .asciiz "TRE IN FILA PER VINCERE."
qstart: .byte CR, CR
        .asciiz "COMINCI TU (S/N)? "
qsquare:
        .asciiz "LA TUA CASELLA (1-9)? "
head:   .byte "TRIS - CLAUDE IA", CR, CR, 0
taken:  .asciiz "QUELLA CASELLA E OCCUPATA."
iplay:  .asciiz "GIOCO "
youwin: .asciiz "BRAVO, HAI VINTO!"
iwin:   .asciiz "HO VINTO IO!"
isdraw: .asciiz "PAREGGIO."
again:  .byte CR
        .asciiz "ANCORA (S/N)? "
B:      .res 9          ; la griglia: 0 vuota, 1 X, 2 O
