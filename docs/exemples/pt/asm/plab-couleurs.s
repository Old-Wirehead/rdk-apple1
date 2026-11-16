; plab-couleurs.s — padrão de cores na placa gráfica P-LAB (TMS9918A)
; Precisa da placa P-LAB (o botão «P-LAB» da barra de ferramentas, ou --plab).
; Modo «Graphics I» (32 x 24 mosaicos); a placa está em $CC00 (dados) / $CC01 (controlo).
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

        ; tabela de padrões ($0000): 256 mosaicos cheios (2 KB de $FF)
        lda #$00
        ldy #$40
        jsr vadr
        ldy #8              ; 8 páginas de 256 bytes
@pm:    ldx #0
@pl:    lda #$FF
        sta VDATA
        inx
        bne @pl
        dey
        bne @pm

        ; tabela de cores ($2000): 32 grupos de 8 mosaicos
        lda #$00
        ldy #$60
        jsr vadr
        ldx #0
@col:   txa
        asl a
        asl a
        asl a
        asl a               ; cor de primeiro plano = X (0..15), e recomeça
        ora #$01            ; fundo preto
        sta VDATA
        inx
        cpx #32
        bne @col

        ; tabela de nomes ($1400): 24 linhas de 32 mosaicos, desfasadas na diagonal
        lda #$00
        ldy #$54
        jsr vadr
        ldy #0              ; Y = linha
@row:   ldx #0
@nm:    tya
        asl a
        asl a               ; linha x 4
        stx $00
        clc
        adc $00             ; + coluna
        sta VDATA
        inx
        cpx #32
        bne @nm
        iny
        cpy #24
        bne @row

        ; tabela de sprites ($1B00): sem sprites ($D0 = fim da lista)
        lda #$00
        ldy #$5B
        jsr vadr
        lda #$D0
        sta VDATA

        jmp GETLINE         ; regresso ao Woz Monitor; a imagem fica

; definir o endereço de vídeo: A = byte baixo, Y = byte alto (+ $40 para escrever)
vadr:   sta VCTRL
        sty VCTRL
        rts

regs:   .byte $00           ; R0: modo Graphics I
        .byte $C0           ; R1: 16 KB, ecrã ativo, sem interrupção
        .byte $05           ; R2: nomes em $1400
        .byte $80           ; R3: cores em $2000
        .byte $00           ; R4: padrões em $0000
        .byte $36           ; R5: sprites em $1B00
        .byte $07           ; R6: padrões de sprites em $3800
        .byte $04           ; R7: margem azul-escura
