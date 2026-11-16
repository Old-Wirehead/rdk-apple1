; tictactoe.s - JOGO DO GALO: o jogador (X) contra o Apple-1 (O).
; Claude IA, 2026. As casas estão numeradas assim:
;    1 ! 2 ! 3
;   ---+---+---
;    4 ! 5 ! 6      escreva o algarismo da casa que joga.
;   ---+---+---
;    7 ! 8 ! 9
; O computador ganha se puder; senão bloqueia; senão fica com o centro
; ou um canto ao acaso. Não vê as forquilhas: pode ser vencido.
        .include "apple1.inc"
        .org $0300

WHO     = $10           ; peça procurada: 1 = X (jogador), 2 = O (computador)
NB      = $11
EMPTY   = $12
CNT     = $13
MOVES   = $14           ; jogadas feitas neste jogo
SEED    = $15           ; 2 bytes
NOTE    = $17           ; última casa do computador (1-9), 0 = nenhuma
TAKEN   = $18           ; 1 = avisar «casa ocupada»
MSG     = $08

start:  cld
        ldx #<title
        ldy #>title
        jsr prmsg
        lda #1
        sta SEED

newgame:
        ldx #8          ; tabuleiro vazio
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
        jsr getkey      ; a semente muda enquanto esperamos
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
        dex             ; casa 0..8
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

cpu:    jsr choose      ; X = casa escolhida
        lda #2
        sta B,x
        inc MOVES
        inx             ; casa 1..9, mostrada por board
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
:       jmp GETLINE     ; regresso ao Woz Monitor

; choose: a jogada do computador, devolvida em X
choose: lda #2          ; 1. ganhar
        sta WHO
        jsr two
        bcc @take
        lda #1          ; 2. bloquear o jogador
        sta WHO
        jsr two
        bcc @take
        ldx #4          ; 3. o centro
        lda B,x
        beq @found
        jsr random      ; 4. um canto, a começar por um ao acaso
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
        ldx #0          ; 5. a primeira casa livre
@free:  lda B,x
        beq @found
        inx
        cpx #9
        bne @free
@found: rts
@take:  tax
        rts

; two: uma linha com duas peças WHO e uma casa vazia?
; Saída: C = 0 e A = a casa vazia, ou C = 1 se não houver nenhuma.
two:    ldx #0          ; X percorre a tabela de linhas de 3 em 3
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

; three: três peças WHO em linha? Saída: C = 1 se sim.
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

; board: limpa o ecrã, mostra a grelha (um algarismo por casa livre),
; depois a última jogada do computador e o aviso «ocupada».
; O Apple-1 não consegue limpar o ecrã por software: 24 linhas vazias empurram
; a jogada anterior para fora, por cima.
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
        lda NOTE        ; a última jogada do computador
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
        lda TAKEN       ; o jogador escolheu uma casa ocupada
        beq @end
        ldx #<taken
        ldy #>taken
        jsr prmsg
        lda #CR
        jsr ECHO
        lda #0
        sta TAKEN
@end:   rts

; piece: mostra a casa X e passa à seguinte
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

; getkey: espera uma tecla (a semente gira entretanto)
getkey: inc SEED+1
        bne :+
        inc SEED
:       lda KBDCR
        bpl getkey
        lda KBD
        rts

; random: registo de deslocamento de Galois de 16 bits, devolve um byte em A
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

; prmsg: mostra a cadeia terminada em zero cujo endereço está em Y:X
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
        .byte "JOGO DO GALO - CLAUDE IA", CR
        .byte "TU ES X, O APPLE-1 E O.", CR
        .asciiz "TRES EM LINHA PARA GANHAR."
qstart: .byte CR, CR
        .asciiz "COMECAS TU (S/N)? "
qsquare:
        .asciiz "A TUA CASA (1-9)? "
head:   .byte "JOGO DO GALO - CLAUDE IA", CR, CR, 0
taken:  .asciiz "ESSA CASA ESTA OCUPADA."
iplay:  .asciiz "JOGO "
youwin: .asciiz "MUITO BEM, GANHASTE!"
iwin:   .asciiz "GANHEI EU!"
isdraw: .asciiz "EMPATE."
again:  .byte CR
        .asciiz "OUTRA VEZ (S/N)? "
B:      .res 9          ; o tabuleiro: 0 vazia, 1 X, 2 O
