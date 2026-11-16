; decimal.s — muestra un entero de 16 bits sin signo en decimal
; (restas repetidas de potencias de diez, sin ceros a la izquierda)
        .include "apple1.inc"
        .org $0300
NUM     = $06           ; número a mostrar (2 bytes)
FLAG    = $08           ; 1 en cuanto se ha mostrado una cifra distinta de cero

start:  lda #CR
        jsr ECHO
        ldx #0
@list:  lda numbers,x   ; mostrar cada número de la lista
        sta NUM
        lda numbers+1,x
        sta NUM+1
        jsr prdec
        lda #' ' | $80
        jsr ECHO
        inx
        inx
        cpx #end_numbers - numbers
        bne @list
        jmp GETLINE

; prdec: muestra NUM en decimal. Conserva X.
prdec:  txa
        pha
        lda #0
        sta FLAG
        ldx #0          ; índice en la tabla de potencias
@digit: ldy #'0' | $80  ; Y = cifra actual
@sub:   lda NUM         ; NUM - potencia
        sec
        sbc powers,x
        pha
        lda NUM+1
        sbc powers+1,x
        bcc @found      ; negativo: cifra encontrada
        sta NUM+1
        pla
        sta NUM
        iny
        bne @sub
@found: pla
        tya
        cmp #'0' | $80
        bne @print
        ldy FLAG        ; ¿cero a la izquierda? se omite
        beq @next
@print: jsr ECHO
        inc FLAG
@next:  inx
        inx
        cpx #8          ; 4 potencias: de 10000 a 10
        bne @digit
        lda NUM         ; resto: las unidades, siempre se muestran
        ora #'0' | $80
        jsr ECHO
        pla
        tax
        rts

powers: .word 10000, 1000, 100, 10
numbers:
        .word 0, 7, 42, 1976, 6502, 65535
end_numbers:
