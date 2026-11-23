; gen2-lores.s — barre di colore in LORES (40 x 48, 16 colori)
; Richiede la scheda GEN2 (pulsante 🌈 GEN2, o la configurazione «Apple 1 moniteur vert + couleur GEN2»). Un tasto qualsiasi: ritorno al testo.
        .include "apple1.inc"
        .include "gen2.inc"
        .org $0C00      ; sopra la pagina LORES 1 ($0400-$07FF)
PTR     = $06

start:  ldx #0          ; X = riga di testo (0..23)
row:    lda rows_lo,x
        sta PTR
        lda rows_hi,x
        sta PTR+1
        ldy #39
column:
        tya             ; colore = colonna / 2 (0..15, poi di nuovo)
        lsr a
        and #$0F
        sta $00         ; un byte = 2 blocchi: nibble basso sopra, alto sotto
        asl a
        asl a
        asl a
        asl a
        ora $00
        sta (PTR),y
        dey
        bpl column
        inx
        cpx #24
        bne row

        bit SS_LORES    ; LORES, schermo intero, pagina 1, poi grafica
        bit SS_FULL
        bit SS_PAGE1
        bit SS_GR
wait:
        lda KBDCR
        bpl wait
        lda KBD
        bit SS_TEXT
        jmp GETLINE

; indirizzi delle 24 righe: $400 + (r & 7) * $80 + (r >> 3) * $28
rows_lo:
.repeat 24, R
        .byte <($0400 + (R & 7) * $80 + (R >> 3) * $28)
.endrep
rows_hi:
.repeat 24, R
        .byte >($0400 + (R & 7) * $80 + (R >> 3) * $28)
.endrep
