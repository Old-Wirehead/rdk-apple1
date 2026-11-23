; gen2-lores.s — Farbbalken in LORES (40 x 48, 16 Farben)
; Braucht die GEN2-Karte (Schaltfläche 🌈 GEN2 oder Konfiguration „Apple 1 moniteur vert + couleur GEN2“). Beliebige Taste: zurück zum Text.
        .include "apple1.inc"
        .include "gen2.inc"
        .org $0C00      ; oberhalb von LORES-Seite 1 ($0400-$07FF)
PTR     = $06

start:  ldx #0          ; X = Textzeile (0..23)
row:    lda rows_lo,x
        sta PTR
        lda rows_hi,x
        sta PTR+1
        ldy #39
column:
        tya             ; Farbe = Spalte / 2 (0..15, dann von vorn)
        lsr a
        and #$0F
        sta $00         ; ein Byte = 2 Blöcke: unteres Nibble oben, oberes darunter
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

        bit SS_LORES    ; LORES, Vollbild, Seite 1, dann Grafik
        bit SS_FULL
        bit SS_PAGE1
        bit SS_GR
wait:
        lda KBDCR
        bpl wait
        lda KBD
        bit SS_TEXT
        jmp GETLINE

; Adressen der 24 Zeilen: $400 + (r & 7) * $80 + (r >> 3) * $28
rows_lo:
.repeat 24, R
        .byte <($0400 + (R & 7) * $80 + (R >> 3) * $28)
.endrep
rows_hi:
.repeat 24, R
        .byte >($0400 + (R & 7) * $80 + (R >> 3) * $28)
.endrep
