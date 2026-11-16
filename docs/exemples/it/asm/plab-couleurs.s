; plab-couleurs.s — motivo a colori sulla scheda grafica P-LAB (TMS9918A)
; Serve la scheda P-LAB (il pulsante «P-LAB» della barra degli strumenti, oppure --plab).
; Modo «Graphics I» (32 x 24 tessere); la scheda è in $CC00 (dati) / $CC01 (controllo).
        .include "apple1.inc"
        .org $0300

VDATA   = $CC00
VCTRL   = $CC01

start:  ldx #0
@regs:  lda regs,x          ; R0..R7
        sta VCTRL
        txa
        ora #$80
        sta VCTRL
        inx
        cpx #8
        bne @regs

        ; tabella dei motivi ($0000): 256 tessere piene (2 KB di $FF)
        lda #$00
        ldy #$40
        jsr vadr
        ldy #8              ; 8 pagine di 256 byte
@pm:    ldx #0
@pl:    lda #$FF
        sta VDATA
        inx
        bne @pl
        dey
        bne @pm

        ; tabella dei colori ($2000): 32 gruppi di 8 tessere
        lda #$00
        ldy #$60
        jsr vadr
        ldx #0
@col:   txa
        asl a
        asl a
        asl a
        asl a               ; colore di primo piano = X (0..15), poi ricomincia
        ora #$01            ; sfondo nero
        sta VDATA
        inx
        cpx #32
        bne @col

        ; tabella dei nomi ($1400): 24 righe di 32 tessere, sfalsate in diagonale
        lda #$00
        ldy #$54
        jsr vadr
        ldy #0              ; Y = riga
@row:   ldx #0
@nm:    tya
        asl a
        asl a               ; riga x 4
        stx $00
        clc
        adc $00             ; + colonna
        sta VDATA
        inx
        cpx #32
        bne @nm
        iny
        cpy #24
        bne @row

        ; tabella degli sprite ($1B00): nessuno sprite ($D0 = fine elenco)
        lda #$00
        ldy #$5B
        jsr vadr
        lda #$D0
        sta VDATA

        jmp GETLINE         ; ritorno al Woz Monitor; l'immagine resta

; impostare l'indirizzo video: A = byte basso, Y = byte alto (+ $40 per scrivere)
vadr:   sta VCTRL
        sty VCTRL
        rts

regs:   .byte $00           ; R0: modo Graphics I
        .byte $C0           ; R1: 16 KB, schermo attivo, nessun interrupt
        .byte $05           ; R2: nomi in $1400
        .byte $80           ; R3: colori in $2000
        .byte $00           ; R4: motivi in $0000
        .byte $36           ; R5: sprite in $1B00
        .byte $07           ; R6: motivi degli sprite in $3800
        .byte $04           ; R7: bordo blu scuro
