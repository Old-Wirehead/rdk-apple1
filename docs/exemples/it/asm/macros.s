; macros.s — macro, .repeat e assemblaggio condizionale
        .include "apple1.inc"
        .org $0300

WIDTH = 20              ; provate 10 o 40
.define CHARACTER '*'

; stampare un carattere costante (la macro si espande in lda/jsr)
.macro putc c
        lda #(c) | $80
        jsr ECHO
.endmacro

; stampare una stringa: etichette locali a ogni chiamata
.macro print text
        .local loop, done, string
        ldx #0
loop:   lda string,x
        beq done
        jsr ECHO
        inx
        bne loop
        beq done
string: .asciiz text
done:
.endmacro

start:  putc 13
        print "MACRO E .REPEAT"
        putc 13
.repeat 3               ; tre righe di stelle, srotolate durante l'assemblaggio
        ldx #WIDTH
:       putc CHARACTER
        dex
        bne :-
        putc 13
.endrep
.if WIDTH > 30
        print "RIGHE LUNGHE"
.else
        print "RIGHE CORTE"
.endif
        jmp GETLINE

; una tabella calcolata dall'assembler: i quadrati da 0 a 15
squares:
.repeat 16, I
        .byte I * I
.endrep
