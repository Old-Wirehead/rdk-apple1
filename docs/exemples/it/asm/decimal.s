; decimal.s — stampa un intero a 16 bit senza segno in decimale
; (sottrazioni ripetute di potenze di dieci, senza zeri iniziali)
        .include "apple1.inc"
        .org $0300
NUM     = $06           ; numero da stampare (2 byte)
FLAG    = $08           ; 1 appena è stata stampata una cifra diversa da zero

start:  lda #CR
        jsr ECHO
        ldx #0
@list:  lda numbers,x   ; stampare ogni numero dell'elenco
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

; prdec: stampa NUM in decimale. Conserva X.
prdec:  txa
        pha
        lda #0
        sta FLAG
        ldx #0          ; indice nella tabella delle potenze
@digit: ldy #'0' | $80  ; Y = cifra corrente
@sub:   lda NUM         ; NUM - potenza
        sec
        sbc powers,x
        pha
        lda NUM+1
        sbc powers+1,x
        bcc @found      ; negativo: cifra trovata
        sta NUM+1
        pla
        sta NUM
        iny
        bne @sub
@found: pla
        tya
        cmp #'0' | $80
        bne @print
        ldy FLAG        ; zero iniziale? lo si salta
        beq @next
@print: jsr ECHO
        inc FLAG
@next:  inx
        inx
        cpx #8          ; 4 potenze: da 10000 a 10
        bne @digit
        lda NUM         ; resto: le unità, sempre stampate
        ora #'0' | $80
        jsr ECHO
        pla
        tax
        rts

powers: .word 10000, 1000, 100, 10
numbers:
        .word 0, 7, 42, 1976, 6502, 65535
end_numbers:
