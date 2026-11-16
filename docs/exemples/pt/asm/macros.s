; macros.s — macros, .repeat e montagem condicional
        .include "apple1.inc"
        .org $0300

WIDTH = 20              ; experimente 10 ou 40
.define CHARACTER '*'

; mostrar um carácter constante (a macro expande-se em lda/jsr)
.macro putc c
        lda #(c) | $80
        jsr ECHO
.endmacro

; mostrar uma cadeia: etiquetas locais em cada chamada
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
        print "MACROS E .REPEAT"
        putc 13
.repeat 3               ; três filas de estrelas, desdobradas na montagem
        ldx #WIDTH
:       putc CHARACTER
        dex
        bne :-
        putc 13
.endrep
.if WIDTH > 30
        print "LINHAS LONGAS"
.else
        print "LINHAS CURTAS"
.endif
        jmp GETLINE

; uma tabela calculada pelo assembler: os quadrados de 0 a 15
squares:
.repeat 16, I
        .byte I * I
.endrep
