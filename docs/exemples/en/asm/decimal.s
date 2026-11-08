; decimal.s — prints an unsigned 16-bit integer in decimal
; (repeated subtraction of powers of ten, no leading zeros)
        .include "apple1.inc"
        .org $0300
NUM     = $06           ; number to print (2 bytes)
FLAG    = $08           ; 1 once a non-zero digit has been printed

start:  lda #CR
        jsr ECHO
        ldx #0
@list:  lda numbers,x   ; print each number of the list
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

; prdec: prints NUM in decimal. Preserves X.
prdec:  txa
        pha
        lda #0
        sta FLAG
        ldx #0          ; index into the table of powers
@digit: ldy #'0' | $80  ; Y = current digit
@sub:   lda NUM         ; NUM - power
        sec
        sbc powers,x
        pha
        lda NUM+1
        sbc powers+1,x
        bcc @found      ; negative: digit found
        sta NUM+1
        pla
        sta NUM
        iny
        bne @sub
@found: pla
        tya
        cmp #'0' | $80
        bne @print
        ldy FLAG        ; leading zero? skip it
        beq @next
@print: jsr ECHO
        inc FLAG
@next:  inx
        inx
        cpx #8          ; 4 powers: 10000 down to 10
        bne @digit
        lda NUM         ; remainder: the units, always printed
        ora #'0' | $80
        jsr ECHO
        pla
        tax
        rts

powers: .word 10000, 1000, 100, 10
numbers:
        .word 0, 7, 42, 1976, 6502, 65535
end_numbers:
