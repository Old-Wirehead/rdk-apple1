; macros.s — macros, .repeat and conditional assembly
        .include "apple1.inc"
        .org $0300

WIDTH = 20              ; try 10 or 40
.define CHARACTER '*'

; print a constant character (the macro expands to lda/jsr)
.macro putc c
        lda #(c) | $80
        jsr ECHO
.endmacro

; print a string: local labels for each call
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
        print "MACROS AND .REPEAT"
        putc 13
.repeat 3               ; three rows of stars, unrolled at assembly time
        ldx #WIDTH
:       putc CHARACTER
        dex
        bne :-
        putc 13
.endrep
.if WIDTH > 30
        print "LONG LINES"
.else
        print "SHORT LINES"
.endif
        jmp GETLINE

; a table computed by the assembler: the squares of 0 to 15
squares:
.repeat 16, I
        .byte I * I
.endrep
