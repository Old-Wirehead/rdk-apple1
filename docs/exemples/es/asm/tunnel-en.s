; tunnel.s - TÚNEL: guíe el coche por un túnel que se desplaza y se estrecha.
; Claude IA, 2026. Un juego en tiempo real para la pantalla del Apple-1, que solo
; puede añadir líneas abajo: cada línea mostrada es una rebanada del túnel.
; J = izquierda, L = derecha. La puntuación cuenta las líneas recorridas.
        .include "apple1.inc"
        .org $0300

LEFTW   = $10           ; columna de la pared izquierda
WIDTH   = $11           ; anchura interior del túnel
CAR     = $12           ; columna del coche
SEED    = $13           ; 2 bytes: generador pseudoaleatorio
SCORE   = $15           ; 2 bytes en BCD (decimal)
COUNT   = $17           ; líneas desde el último estrechamiento
RIGHTW  = $18           ; columna de la pared derecha
LAST    = $19           ; última columna a mostrar
MSG     = $08           ; puntero al mensaje (2 bytes)

start:  cld
        ldx #<title
        ldy #>title
        jsr prmsg
        lda #1
        sta SEED
@wait:  inc SEED+1      ; la semilla cambia mientras esperamos
        bne :+
        inc SEED
:       lda KBDCR
        bpl @wait
        lda KBD
        lda SEED
        ora SEED+1
        bne game
        inc SEED        ; nunca cero

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

loop:   lda KBDCR       ; ¿una tecla?
        bpl @wall
        lda KBD
        cmp #'J' | $80
        bne :+
        dec CAR
:       cmp #'L' | $80
        bne @wall
        inc CAR
@wall:  jsr random      ; el túnel gira al azar
        and #3
        bne @right
        lda LEFTW       ; 0: a la izquierda
        cmp #2
        bcc @narrow
        dec LEFTW
        jmp @narrow
@right: cmp #1          ; 1: a la derecha (2 y 3: recto)
        bne @narrow
        lda LEFTW
        clc
        adc WIDTH
        cmp #36         ; la pared derecha se queda antes de la columna 38
        bcs @narrow
        inc LEFTW
@narrow:
        inc COUNT       ; cada 30 líneas, el túnel se estrecha
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
        lda CAR         ; ¿contra la pared izquierda?
        cmp LEFTW
        beq crash
        bcc crash
        cmp RIGHTW      ; ¿contra la pared derecha?
        bcs crash
        sed             ; puntuación + 1, en decimal
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
        lda KBD         ; olvidar la última tecla de la partida
@k:     lda KBDCR
        bpl @k
        lda KBD
        cmp #'S' | $80
        bne :+
        jmp game
:       jmp GETLINE     ; vuelta al Woz Monitor

; slice: muestra una rebanada del túnel, con el coche
slice:  lda LEFTW
        clc
        adc WIDTH
        adc #1
        sta RIGHTW
        cmp CAR
        bcs @end        ; el coche está antes de la pared derecha
        lda CAR
        cmp #40
        bcc @end        ; coche fuera por la derecha: se muestra
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

; random: registro de desplazamiento de Galois de 16 bits, devuelve un byte en A
random: lsr SEED+1
        ror SEED
        bcc :+
        lda SEED+1
        eor #$B4
        sta SEED+1
:       lda SEED
        rts

; pause: unos 50 ms, para que el juego siga siendo jugable con visualización rápida
pause:  ldx #40
@x:     ldy #0
@y:     dey
        bne @y
        dex
        bne @x
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

title:  .byte CR
        .byte "TUNEL - CLAUDE IA", CR
        .byte "GUIA EL COCHE * POR EL TUNEL.", CR
        .byte "J = IZQUIERDA, L = DERECHA.", CR
        .byte "EL TUNEL SE ESTRECHA POCO A POCO...", CR
        .asciiz "PULSA UNA TECLA PARA EMPEZAR."
bang:   .byte CR
        .asciiz "CHOQUE! PUNTOS: "
again:  .byte CR
        .asciiz "OTRA VEZ (S/N)? "
