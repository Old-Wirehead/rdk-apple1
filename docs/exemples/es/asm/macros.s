; macros.s — macros, .repeat y ensamblado condicional
        .include "apple1.inc"
        .org $0300

WIDTH = 20              ; pruebe 10 o 40
.define CHARACTER '*'

; mostrar un carácter constante (la macro se expande en lda/jsr)
.macro putc c
        lda #(c) | $80
        jsr ECHO
.endmacro

; mostrar una cadena: etiquetas locales en cada llamada
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
        print "MACROS Y .REPEAT"
        putc 13
.repeat 3               ; tres filas de estrellas, desplegadas al ensamblar
        ldx #WIDTH
:       putc CHARACTER
        dex
        bne :-
        putc 13
.endrep
.if WIDTH > 30
        print "LINEAS LARGAS"
.else
        print "LINEAS CORTAS"
.endif
        jmp GETLINE

; una tabla calculada por el ensamblador: los cuadrados de 0 a 15
squares:
.repeat 16, I
        .byte I * I
.endrep
