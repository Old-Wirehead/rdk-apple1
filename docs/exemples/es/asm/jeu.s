; jeu.s — adivine la cifra (0 a 9): un bucle de juego sencillo.
; El azar viene del tiempo que tarda el jugador en pulsar una tecla.
        .include "apple1.inc"
        .org $0300
SECRET  = $06
COUNT   = $07

start:  ldx #<title
        ldy #>title
        jsr prmsg
        ldx #0          ; X gira mientras esperamos: la semilla
@wait:  inx
        lda KBDCR
        bpl @wait
        lda KBD
        txa
@mod:   cmp #10         ; X módulo 10
        bcc @ok
        sbc #10         ; aquí C = 1
        bcs @mod
@ok:    sta SECRET
        lda #0
        sta COUNT

turn:   ldx #<question
        ldy #>question
        jsr prmsg
        jsr getdig      ; A = cifra 0..9 (las demás teclas se ignoran)
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
        ora #'0' | $80  ; < 10 intentos: una sola cifra
        cmp #'9' | $80 + 1
        bcc :+
        lda #'+' | $80
:       jsr ECHO
        ldx #<tries
        ldy #>tries
        jsr prmsg
        jmp GETLINE

; getdig: espera una cifra, la muestra y la devuelve en binario en A
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

; prmsg: muestra la cadena terminada en cero cuya dirección está en Y:X
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
          .asciiz "HE ELEGIDO UNA CIFRA. PULSE UNA TECLA."
question: .byte CR
          .asciiz "TU CIFRA? "
higher:   .asciiz " MAS ALTO."
lower:    .asciiz " MAS BAJO."
bravo:    .byte " GANADO EN ", 0
tries:    .asciiz " INTENTOS!"
