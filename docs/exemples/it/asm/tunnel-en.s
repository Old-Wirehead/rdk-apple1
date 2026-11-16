; tunnel.s - TUNNEL: guidate l'auto in un tunnel che scorre e si restringe.
; Claude IA, 2026. Un gioco in tempo reale per lo schermo dell'Apple-1, che può solo
; aggiungere righe in basso: ogni riga stampata è una fetta del tunnel.
; J = sinistra, L = destra. Il punteggio conta le righe percorse.
        .include "apple1.inc"
        .org $0300

LEFTW   = $10           ; colonna della parete sinistra
WIDTH   = $11           ; larghezza interna del tunnel
CAR     = $12           ; colonna dell'auto
SEED    = $13           ; 2 byte: generatore pseudocasuale
SCORE   = $15           ; 2 byte in BCD (decimale)
COUNT   = $17           ; righe dall'ultimo restringimento
RIGHTW  = $18           ; colonna della parete destra
LAST    = $19           ; ultima colonna da stampare
MSG     = $08           ; puntatore al messaggio (2 byte)

start:  cld
        ldx #<title
        ldy #>title
        jsr prmsg
        lda #1
        sta SEED
@wait:  inc SEED+1      ; il seme cambia mentre aspettiamo
        bne :+
        inc SEED
:       lda KBDCR
        bpl @wait
        lda KBD
        lda SEED
        ora SEED+1
        bne game
        inc SEED        ; mai zero

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

loop:   lda KBDCR       ; un tasto?
        bpl @wall
        lda KBD
        cmp #'J' | $80
        bne :+
        dec CAR
:       cmp #'L' | $80
        bne @wall
        inc CAR
@wall:  jsr random      ; il tunnel curva a caso
        and #3
        bne @right
        lda LEFTW       ; 0: a sinistra
        cmp #2
        bcc @narrow
        dec LEFTW
        jmp @narrow
@right: cmp #1          ; 1: a destra (2 e 3: dritto)
        bne @narrow
        lda LEFTW
        clc
        adc WIDTH
        cmp #36         ; la parete destra resta prima della colonna 38
        bcs @narrow
        inc LEFTW
@narrow:
        inc COUNT       ; ogni 30 righe, il tunnel si restringe
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
        lda CAR         ; contro la parete sinistra?
        cmp LEFTW
        beq crash
        bcc crash
        cmp RIGHTW      ; contro la parete destra?
        bcs crash
        sed             ; punteggio + 1, in decimale
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
        lda KBD         ; dimenticare l'ultimo tasto della partita
@k:     lda KBDCR
        bpl @k
        lda KBD
        cmp #'S' | $80
        bne :+
        jmp game
:       jmp GETLINE     ; ritorno al Woz Monitor

; slice: stampa una fetta del tunnel, con l'auto
slice:  lda LEFTW
        clc
        adc WIDTH
        adc #1
        sta RIGHTW
        cmp CAR
        bcs @end        ; l'auto è prima della parete destra
        lda CAR
        cmp #40
        bcc @end        ; auto fuori a destra: la si mostra
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

; random: registro a scorrimento di Galois a 16 bit, restituisce un byte in A
random: lsr SEED+1
        ror SEED
        bcc :+
        lda SEED+1
        eor #$B4
        sta SEED+1
:       lda SEED
        rts

; pause: circa 50 ms, perché il gioco resti giocabile con la visualizzazione veloce
pause:  ldx #40
@x:     ldy #0
@y:     dey
        bne @y
        dex
        bne @x
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

title:  .byte CR
        .byte "TUNNEL - CLAUDE IA", CR
        .byte "GUIDA L'AUTO * NEL TUNNEL.", CR
        .byte "J = SINISTRA, L = DESTRA.", CR
        .byte "IL TUNNEL SI RESTRINGE PIANO...", CR
        .asciiz "PREMI UN TASTO PER INIZIARE."
bang:   .byte CR
        .asciiz "SCHIANTO! PUNTI: "
again:  .byte CR
        .asciiz "ANCORA (S/N)? "
