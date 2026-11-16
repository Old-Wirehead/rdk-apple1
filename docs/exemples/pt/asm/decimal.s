; decimal.s — mostra um inteiro de 16 bits sem sinal em decimal
; (subtrações repetidas de potências de dez, sem zeros à esquerda)
        .include "apple1.inc"
        .org $0300
NUM     = $06           ; número a mostrar (2 bytes)
FLAG    = $08           ; 1 assim que foi mostrado um algarismo diferente de zero

start:  lda #CR
        jsr ECHO
        ldx #0
@list:  lda numbers,x   ; mostrar cada número da lista
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

; prdec: mostra NUM em decimal. Conserva X.
prdec:  txa
        pha
        lda #0
        sta FLAG
        ldx #0          ; índice na tabela de potências
@digit: ldy #'0' | $80  ; Y = algarismo atual
@sub:   lda NUM         ; NUM - potência
        sec
        sbc powers,x
        pha
        lda NUM+1
        sbc powers+1,x
        bcc @found      ; negativo: algarismo encontrado
        sta NUM+1
        pla
        sta NUM
        iny
        bne @sub
@found: pla
        tya
        cmp #'0' | $80
        bne @print
        ldy FLAG        ; zero à esquerda? salta-se
        beq @next
@print: jsr ECHO
        inc FLAG
@next:  inx
        inx
        cpx #8          ; 4 potências: de 10000 a 10
        bne @digit
        lda NUM         ; resto: as unidades, sempre mostradas
        ora #'0' | $80
        jsr ECHO
        pla
        tax
        rts

powers: .word 10000, 1000, 100, 10
numbers:
        .word 0, 7, 42, 1976, 6502, 65535
end_numbers:
