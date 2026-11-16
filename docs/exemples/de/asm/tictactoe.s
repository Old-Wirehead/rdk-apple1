; tictactoe.s - TIC-TAC-TOE: der Spieler (X) gegen den Apple-1 (O).
; Claude IA, 2026. Die Felder sind so nummeriert:
;    1 ! 2 ! 3
;   ---+---+---
;    4 ! 5 ! 6      tippen Sie die Ziffer des gespielten Feldes.
;   ---+---+---
;    7 ! 8 ! 9
; Der Computer gewinnt, wenn er kann, sonst blockiert er, sonst nimmt er die Mitte
; oder eine zufällige Ecke. Gabeln sieht er nicht: er ist schlagbar.
        .include "apple1.inc"
        .org $0300

WHO     = $10           ; gesuchter Stein: 1 = X (Spieler), 2 = O (Computer)
NB      = $11
EMPTY   = $12
CNT     = $13
MOVES   = $14           ; in dieser Partie gespielte Züge
SEED    = $15           ; 2 Bytes
NOTE    = $17           ; letztes Feld des Computers (1-9), 0 = keins
TAKEN   = $18           ; 1 = „Feld besetzt“ melden
MSG     = $08

start:  cld
        ldx #<title
        ldy #>title
        jsr prmsg
        lda #1
        sta SEED

newgame:
        ldx #8          ; leeres Spielfeld
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
        jsr getkey      ; der Startwert ändert sich, während wir warten
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
        dex             ; Feld 0..8
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

cpu:    jsr choose      ; X = gewähltes Feld
        lda #2
        sta B,x
        inc MOVES
        inx             ; Feld 1..9, von board angezeigt
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
        cmp #'J' | $80
        bne :+
        jmp newgame
:       jmp GETLINE     ; zurück zum Woz Monitor

; choose: der Zug des Computers, zurückgegeben in X
choose: lda #2          ; 1. gewinnen
        sta WHO
        jsr two
        bcc @take
        lda #1          ; 2. den Spieler blockieren
        sta WHO
        jsr two
        bcc @take
        ldx #4          ; 3. die Mitte
        lda B,x
        beq @found
        jsr random      ; 4. eine Ecke, ab einer zufälligen
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
        ldx #0          ; 5. das erste freie Feld
@free:  lda B,x
        beq @found
        inx
        cpx #9
        bne @free
@found: rts
@take:  tax
        rts

; two: eine Reihe mit zwei Steinen WHO und einem leeren Feld?
; Ausgabe: C = 0 und A = das leere Feld, oder C = 1, wenn es keine gibt.
two:    ldx #0          ; X durchläuft die Tabelle der Reihen, 3 auf einmal
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

; three: drei Steine WHO in einer Reihe? Ausgabe: C = 1, wenn ja.
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

; board: Bildschirm leeren, Gitter ausgeben (eine Ziffer für ein freies Feld),
; dann den letzten Zug des Computers und die Warnung „besetzt“.
; Der Apple-1 kann seinen Bildschirm nicht per Software löschen: 24 Leerzeilen schieben
; die alte Runde oben hinaus.
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
        lda NOTE        ; der letzte Zug des Computers
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
        lda TAKEN       ; der Spieler hat ein besetztes Feld gewählt
        beq @end
        ldx #<taken
        ldy #>taken
        jsr prmsg
        lda #CR
        jsr ECHO
        lda #0
        sta TAKEN
@end:   rts

; piece: Feld X ausgeben und zum nächsten gehen
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

; getkey: auf eine Taste warten (der Startwert läuft dabei weiter)
getkey: inc SEED+1
        bne :+
        inc SEED
:       lda KBDCR
        bpl getkey
        lda KBD
        rts

; random: 16-Bit-Galois-Schieberegister, liefert ein Byte in A
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

; prmsg: die mit null abgeschlossene Zeichenkette ausgeben, deren Adresse in Y:X steht
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
        .byte "DU BIST X, DER APPLE-1 IST O.", CR
        .asciiz "DREI IN EINER REIHE GEWINNEN."
qstart: .byte CR, CR
        .asciiz "FAENGST DU AN (J/N)? "
qsquare:
        .asciiz "DEIN FELD (1-9)? "
head:   .byte "TIC-TAC-TOE - CLAUDE IA", CR, CR, 0
taken:  .asciiz "DIESES FELD IST BESETZT."
iplay:  .asciiz "ICH SPIELE "
youwin: .asciiz "BRAVO, DU GEWINNST!"
iwin:   .asciiz "ICH GEWINNE!"
isdraw: .asciiz "UNENTSCHIEDEN."
again:  .byte CR
        .asciiz "NOCH EINMAL (J/N)? "
B:      .res 9          ; das Spielfeld: 0 leer, 1 X, 2 O
