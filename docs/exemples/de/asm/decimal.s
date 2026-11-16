; decimal.s — gibt eine vorzeichenlose 16-Bit-Zahl dezimal aus
; (wiederholtes Abziehen von Zehnerpotenzen, ohne führende Nullen)
        .include "apple1.inc"
        .org $0300
NUM     = $06           ; auszugebende Zahl (2 Bytes)
FLAG    = $08           ; 1, sobald eine Ziffer ungleich null ausgegeben wurde

start:  lda #CR
        jsr ECHO
        ldx #0
@list:  lda numbers,x   ; jede Zahl der Liste ausgeben
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

; prdec: gibt NUM dezimal aus. Erhält X.
prdec:  txa
        pha
        lda #0
        sta FLAG
        ldx #0          ; Index in der Tabelle der Potenzen
@digit: ldy #'0' | $80  ; Y = aktuelle Ziffer
@sub:   lda NUM         ; NUM - Potenz
        sec
        sbc powers,x
        pha
        lda NUM+1
        sbc powers+1,x
        bcc @found      ; negativ: Ziffer gefunden
        sta NUM+1
        pla
        sta NUM
        iny
        bne @sub
@found: pla
        tya
        cmp #'0' | $80
        bne @print
        ldy FLAG        ; führende Null? überspringen
        beq @next
@print: jsr ECHO
        inc FLAG
@next:  inx
        inx
        cpx #8          ; 4 Potenzen: 10000 bis 10
        bne @digit
        lda NUM         ; Rest: die Einer, immer ausgegeben
        ora #'0' | $80
        jsr ECHO
        pla
        tax
        rts

powers: .word 10000, 1000, 100, 10
numbers:
        .word 0, 7, 42, 1976, 6502, 65535
end_numbers:
