; tunnel.s - TÚNEL: conduza o carro por um túnel que desliza e se estreita.
; Claude IA, 2026. Um jogo em tempo real para o ecrã do Apple-1, que só
; pode juntar linhas em baixo: cada linha mostrada é uma fatia do túnel.
; J = esquerda, L = direita. A pontuação conta as linhas percorridas.
        .include "apple1.inc"
        .org $0300

LEFTW   = $10           ; coluna da parede esquerda
WIDTH   = $11           ; largura interior do túnel
CAR     = $12           ; coluna do carro
SEED    = $13           ; 2 bytes: gerador pseudoaleatório
SCORE   = $15           ; 2 bytes em BCD (decimal)
COUNT   = $17           ; linhas desde o último estreitamento
RIGHTW  = $18           ; coluna da parede direita
LAST    = $19           ; última coluna a mostrar
MSG     = $08           ; ponteiro para a mensagem (2 bytes)

start:  cld
        ldx #<title
        ldy #>title
        jsr prmsg
        lda #1
        sta SEED
@wait:  inc SEED+1      ; a semente muda enquanto esperamos
        bne :+
        inc SEED
:       lda KBDCR
        bpl @wait
        lda KBD
        lda SEED
        ora SEED+1
        bne game
        inc SEED        ; nunca zero

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

loop:   lda KBDCR       ; uma tecla?
        bpl @wall
        lda KBD
        cmp #'J' | $80
        bne :+
        dec CAR
:       cmp #'L' | $80
        bne @wall
        inc CAR
@wall:  jsr random      ; o túnel vira ao acaso
        and #3
        bne @right
        lda LEFTW       ; 0: para a esquerda
        cmp #2
        bcc @narrow
        dec LEFTW
        jmp @narrow
@right: cmp #1          ; 1: para a direita (2 e 3: em frente)
        bne @narrow
        lda LEFTW
        clc
        adc WIDTH
        cmp #36         ; a parede direita fica antes da coluna 38
        bcs @narrow
        inc LEFTW
@narrow:
        inc COUNT       ; a cada 30 linhas, o túnel estreita-se
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
        lda CAR         ; contra a parede esquerda?
        cmp LEFTW
        beq crash
        bcc crash
        cmp RIGHTW      ; contra a parede direita?
        bcs crash
        sed             ; pontuação + 1, em decimal
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
        lda KBD         ; esquecer a última tecla do jogo
@k:     lda KBDCR
        bpl @k
        lda KBD
        cmp #'S' | $80
        bne :+
        jmp game
:       jmp GETLINE     ; regresso ao Woz Monitor

; slice: mostra uma fatia do túnel, com o carro
slice:  lda LEFTW
        clc
        adc WIDTH
        adc #1
        sta RIGHTW
        cmp CAR
        bcs @end        ; o carro está antes da parede direita
        lda CAR
        cmp #40
        bcc @end        ; carro fora pela direita: mostra-se
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

; random: registo de deslocamento de Galois de 16 bits, devolve um byte em A
random: lsr SEED+1
        ror SEED
        bcc :+
        lda SEED+1
        eor #$B4
        sta SEED+1
:       lda SEED
        rts

; pause: cerca de 50 ms, para o jogo continuar jogável com visualização rápida
pause:  ldx #40
@x:     ldy #0
@y:     dey
        bne @y
        dex
        bne @x
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

title:  .byte CR
        .byte "TUNEL - CLAUDE IA", CR
        .byte "CONDUZ O CARRO * PELO TUNEL.", CR
        .byte "J = ESQUERDA, L = DIREITA.", CR
        .byte "O TUNEL ESTREITA-SE AOS POUCOS...", CR
        .asciiz "CARREGA NUMA TECLA PARA COMECAR."
bang:   .byte CR
        .asciiz "CHOQUE! PONTOS: "
again:  .byte CR
        .asciiz "OUTRA VEZ (S/N)? "
