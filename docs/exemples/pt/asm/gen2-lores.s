; gen2-lores.s — barras de cor em LORES (40 x 48, 16 cores)
; Precisa da placa GEN2 (Preferências ▸ Placa a cores). Qualquer tecla: regresso ao texto.
        .include "apple1.inc"
        .include "gen2.inc"
        .org $0C00      ; acima da página LORES 1 ($0400-$07FF)
PTR     = $06

start:  ldx #0          ; X = linha de texto (0..23)
row:    lda rows_lo,x
        sta PTR
        lda rows_hi,x
        sta PTR+1
        ldy #39
column:
        tya             ; cor = coluna / 2 (0..15, e de novo)
        lsr a
        and #$0F
        sta $00         ; um byte = 2 blocos: nibble baixo em cima, alto em baixo
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

        bit SS_LORES    ; LORES, ecrã inteiro, página 1 e depois gráficos
        bit SS_FULL
        bit SS_PAGE1
        bit SS_GR
wait:
        lda KBDCR
        bpl wait
        lda KBD
        bit SS_TEXT
        jmp GETLINE

; endereços das 24 linhas: $400 + (r & 7) * $80 + (r >> 3) * $28
rows_lo:
.repeat 24, R
        .byte <($0400 + (R & 7) * $80 + (R >> 3) * $28)
.endrep
rows_hi:
.repeat 24, R
        .byte >($0400 + (R & 7) * $80 + (R >> 3) * $28)
.endrep
