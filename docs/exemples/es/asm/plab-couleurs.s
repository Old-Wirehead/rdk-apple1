; plab-couleurs.s — motivo de colores en la tarjeta gráfica P-LAB (TMS9918A)
; Necesita la tarjeta P-LAB (el botón «P-LAB» de la barra de herramientas, o --plab).
; Modo «Graphics I» (32 x 24 mosaicos); la tarjeta está en $CC00 (datos) / $CC01 (control).
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

        ; tabla de motivos ($0000): 256 mosaicos llenos (2 KB de $FF)
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

        ; tabla de colores ($2000): 32 grupos de 8 mosaicos
        lda #$00
        ldy #$60
        jsr vadr
        ldx #0
@col:   txa
        asl a
        asl a
        asl a
        asl a               ; color de primer plano = X (0..15), y vuelta a empezar
        ora #$01            ; fondo negro
        sta VDATA
        inx
        cpx #32
        bne @col

        ; tabla de nombres ($1400): 24 filas de 32 mosaicos, desplazadas en diagonal
        lda #$00
        ldy #$54
        jsr vadr
        ldy #0              ; Y = fila
@row:   ldx #0
@nm:    tya
        asl a
        asl a               ; fila x 4
        stx $00
        clc
        adc $00             ; + columna
        sta VDATA
        inx
        cpx #32
        bne @nm
        iny
        cpy #24
        bne @row

        ; tabla de sprites ($1B00): sin sprites ($D0 = fin de lista)
        lda #$00
        ldy #$5B
        jsr vadr
        lda #$D0
        sta VDATA

        jmp GETLINE         ; vuelta al Woz Monitor; la imagen se queda

; fijar la dirección de vídeo: A = byte bajo, Y = byte alto (+ $40 para escribir)
vadr:   sta VCTRL
        sty VCTRL
        rts

regs:   .byte $00           ; R0: modo Graphics I
        .byte $C0           ; R1: 16 KB, pantalla activa, sin interrupción
        .byte $05           ; R2: nombres en $1400
        .byte $80           ; R3: colores en $2000
        .byte $00           ; R4: motivos en $0000
        .byte $36           ; R5: sprites en $1B00
        .byte $07           ; R6: motivos de sprites en $3800
        .byte $04           ; R7: borde azul oscuro
