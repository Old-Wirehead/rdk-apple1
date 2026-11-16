; jeu.s — adivinhe o algarismo (0 a 9): um ciclo de jogo simples.
; O acaso vem do tempo que o jogador demora a carregar numa tecla.
        .include "apple1.inc"
        .org $0300
SECRET  = $06
COUNT   = $07

start:  ldx #<title
        ldy #>title
        jsr prmsg
        ldx #0          ; X gira enquanto esperamos: a semente
@wait:  inx
        lda KBDCR
        bpl @wait
        lda KBD
        txa
@mod:   cmp #10         ; X módulo 10
        bcc @ok
        sbc #10         ; aqui C = 1
        bcs @mod
@ok:    sta SECRET
        lda #0
        sta COUNT

turn:   ldx #<question
        ldy #>question
        jsr prmsg
        jsr getdig      ; A = algarismo 0..9 (as outras teclas são ignoradas)
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
        ora #'0' | $80  ; < 10 tentativas: um só algarismo
        cmp #'9' | $80 + 1
        bcc :+
        lda #'+' | $80
:       jsr ECHO
        ldx #<tries
        ldy #>tries
        jsr prmsg
        jmp GETLINE

; getdig: espera um algarismo, mostra-o e devolve-o em binário em A
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

; prmsg: mostra a cadeia terminada em zero cujo endereço está em Y:X
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
          .asciiz "ESCOLHI UM DIGITO. CARREGUE NUMA TECLA."
question: .byte CR
          .asciiz "O TEU ALGARISMO? "
higher:   .asciiz " MAIS ALTO."
lower:    .asciiz " MAIS BAIXO."
bravo:    .byte " GANHO EM ", 0
tries:    .asciiz " TENTATIVAS!"
