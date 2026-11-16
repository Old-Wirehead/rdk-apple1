; macros.s — Makros, .repeat und bedingte Assemblierung
        .include "apple1.inc"
        .org $0300

WIDTH = 20              ; 10 oder 40 ausprobieren
.define CHARACTER '*'

; ein festes Zeichen ausgeben (das Makro wird zu lda/jsr)
.macro putc c
        lda #(c) | $80
        jsr ECHO
.endmacro

; eine Zeichenkette ausgeben: lokale Marken für jeden Aufruf
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
        print "MAKROS UND .REPEAT"
        putc 13
.repeat 3               ; drei Sternreihen, beim Assemblieren ausgerollt
        ldx #WIDTH
:       putc CHARACTER
        dex
        bne :-
        putc 13
.endrep
.if WIDTH > 30
        print "LANGE ZEILEN"
.else
        print "KURZE ZEILEN"
.endif
        jmp GETLINE

; eine vom Assembler berechnete Tabelle: die Quadrate von 0 bis 15
squares:
.repeat 16, I
        .byte I * I
.endrep
