; tictactoe.s - TRES EN RAYA: el jugador (X) contra el Apple-1 (O).
; Claude IA, 2026. Las casillas están numeradas así:
;    1 ! 2 ! 3
;   ---+---+---
;    4 ! 5 ! 6      escriba la cifra de la casilla que juega.
;   ---+---+---
;    7 ! 8 ! 9
; El ordenador gana si puede; si no, bloquea; si no, toma el centro
; o una esquina al azar. No ve las horquillas: se le puede ganar.
        .include "apple1.inc"
        .org $0300

WHO     = $10           ; ficha buscada: 1 = X (jugador), 2 = O (ordenador)
NB      = $11
EMPTY   = $12
CNT     = $13
MOVES   = $14           ; jugadas hechas en esta partida
SEED    = $15           ; 2 bytes
NOTE    = $17           ; última casilla del ordenador (1-9), 0 = ninguna
TAKEN   = $18           ; 1 = avisar «casilla ocupada»
MSG     = $08

start:  cld
        ldx #<title
        ldy #>title
        jsr prmsg
        lda #1
        sta SEED

newgame:
        ldx #8          ; tablero vacío
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
        jsr getkey      ; la semilla cambia mientras esperamos
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
        dex             ; casilla 0..8
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

cpu:    jsr choose      ; X = casilla elegida
        lda #2
        sta B,x
        inc MOVES
        inx             ; casilla 1..9, la muestra board
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
:       jmp GETLINE     ; vuelta al Woz Monitor

; choose: la jugada del ordenador, devuelta en X
choose: lda #2          ; 1. ganar
        sta WHO
        jsr two
        bcc @take
        lda #1          ; 2. bloquear al jugador
        sta WHO
        jsr two
        bcc @take
        ldx #4          ; 3. el centro
        lda B,x
        beq @found
        jsr random      ; 4. una esquina, empezando por una al azar
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
        ldx #0          ; 5. la primera casilla libre
@free:  lda B,x
        beq @found
        inx
        cpx #9
        bne @free
@found: rts
@take:  tax
        rts

; two: ¿una línea con dos fichas WHO y una casilla vacía?
; Salida: C = 0 y A = la casilla vacía, o C = 1 si no hay ninguna.
two:    ldx #0          ; X recorre la tabla de líneas de 3 en 3
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

; three: ¿tres fichas WHO en línea? Salida: C = 1 si es así.
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

; board: borra la pantalla, muestra la cuadrícula (una cifra por casilla libre),
; luego la última jugada del ordenador y el aviso «ocupada».
; El Apple-1 no puede borrar su pantalla por software: 24 líneas vacías empujan
; el turno anterior fuera por arriba.
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
        lda NOTE        ; la última jugada del ordenador
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
        lda TAKEN       ; el jugador eligió una casilla ocupada
        beq @end
        ldx #<taken
        ldy #>taken
        jsr prmsg
        lda #CR
        jsr ECHO
        lda #0
        sta TAKEN
@end:   rts

; piece: muestra la casilla X y pasa a la siguiente
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

; getkey: espera una tecla (la semilla gira mientras tanto)
getkey: inc SEED+1
        bne :+
        inc SEED
:       lda KBDCR
        bpl getkey
        lda KBD
        rts

; random: registro de desplazamiento de Galois de 16 bits, devuelve un byte en A
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

; prmsg: muestra la cadena terminada en cero cuya dirección está en Y:X
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
        .byte "TRES EN RAYA - CLAUDE IA", CR
        .byte "TU ERES X, EL APPLE-1 ES O.", CR
        .asciiz "TRES EN LINEA PARA GANAR."
qstart: .byte CR, CR
        .asciiz "EMPIEZAS TU (S/N)? "
qsquare:
        .asciiz "TU CASILLA (1-9)? "
head:   .byte "TRES EN RAYA - CLAUDE IA", CR, CR, 0
taken:  .asciiz "ESA CASILLA ESTA OCUPADA."
iplay:  .asciiz "JUEGO "
youwin: .asciiz "BIEN HECHO, GANAS TU!"
iwin:   .asciiz "GANO YO!"
isdraw: .asciiz "EMPATE."
again:  .byte CR
        .asciiz "OTRA VEZ (S/N)? "
B:      .res 9          ; el tablero: 0 vacía, 1 X, 2 O
