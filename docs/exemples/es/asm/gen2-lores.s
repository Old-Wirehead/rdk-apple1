; gen2-lores.s — barras de color en LORES (40 x 48, 16 colores)
; Necesita la tarjeta GEN2 (Preferencias ▸ Tarjeta de color). Cualquier tecla: vuelta al texto.
        .include "apple1.inc"
        .include "gen2.inc"
        .org $0C00      ; por encima de la página LORES 1 ($0400-$07FF)
PTR     = $06

start:  ldx #0          ; X = fila de texto (0..23)
row:    lda rows_lo,x
        sta PTR
        lda rows_hi,x
        sta PTR+1
        ldy #39
column:
        tya             ; color = columna / 2 (0..15, y otra vez)
        lsr a
        and #$0F
        sta $00         ; un byte = 2 bloques: nibble bajo arriba, alto debajo
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

        bit SS_LORES    ; LORES, pantalla completa, página 1 y luego gráficos
        bit SS_FULL
        bit SS_PAGE1
        bit SS_GR
wait:
        lda KBDCR
        bpl wait
        lda KBD
        bit SS_TEXT
        jmp GETLINE

; direcciones de las 24 filas: $400 + (r & 7) * $80 + (r >> 3) * $28
rows_lo:
.repeat 24, R
        .byte <($0400 + (R & 7) * $80 + (R >> 3) * $28)
.endrep
rows_hi:
.repeat 24, R
        .byte >($0400 + (R & 7) * $80 + (R >> 3) * $28)
.endrep
