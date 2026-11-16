; tunnel.s - TUNNEL: steuern Sie das Auto durch einen rollenden, enger werdenden Tunnel.
; Claude IA, 2026. Ein Echtzeitspiel für den Apple-1-Bildschirm, der nur
; unten Zeilen anfügen kann: jede ausgegebene Zeile ist eine Scheibe des Tunnels.
; J = links, L = rechts. Die Punktzahl zählt die gefahrenen Zeilen.
        .include "apple1.inc"
        .org $0300

LEFTW   = $10           ; Spalte der linken Wand
WIDTH   = $11           ; Innenbreite des Tunnels
CAR     = $12           ; Spalte des Autos
SEED    = $13           ; 2 Bytes: Pseudozufallsgenerator
SCORE   = $15           ; 2 Bytes in BCD (dezimal)
COUNT   = $17           ; Zeilen seit der letzten Verengung
RIGHTW  = $18           ; Spalte der rechten Wand
LAST    = $19           ; letzte auszugebende Spalte
MSG     = $08           ; Zeiger auf die Meldung (2 Bytes)

start:  cld
        ldx #<title
        ldy #>title
        jsr prmsg
        lda #1
        sta SEED
@wait:  inc SEED+1      ; der Startwert ändert sich, während wir warten
        bne :+
        inc SEED
:       lda KBDCR
        bpl @wait
        lda KBD
        lda SEED
        ora SEED+1
        bne game
        inc SEED        ; niemals null

game:   lda #12
        sta LEFTW
        lda #14
        sta WIDTH
        lda #19
        sta CAR
        lda #0
        sta SCORE
        sta SCORE+1
        sta COUNT
        lda #CR
        jsr ECHO

loop:   lda KBDCR       ; eine Taste?
        bpl @wall
        lda KBD
        cmp #'J' | $80
        bne :+
        dec CAR
:       cmp #'L' | $80
        bne @wall
        inc CAR
@wall:  jsr random      ; der Tunnel biegt zufällig ab
        and #3
        bne @right
        lda LEFTW       ; 0: nach links
        cmp #2
        bcc @narrow
        dec LEFTW
        jmp @narrow
@right: cmp #1          ; 1: nach rechts (2 und 3: geradeaus)
        bne @narrow
        lda LEFTW
        clc
        adc WIDTH
        cmp #36         ; die rechte Wand bleibt vor Spalte 38
        bcs @narrow
        inc LEFTW
@narrow:
        inc COUNT       ; alle 30 Zeilen wird der Tunnel enger
        lda COUNT
        cmp #30
        bne @draw
        lda #0
        sta COUNT
        lda WIDTH
        cmp #5
        bcc @draw
        dec WIDTH
@draw:  jsr slice
        lda CAR         ; an der linken Wand?
        cmp LEFTW
        beq crash
        bcc crash
        cmp RIGHTW      ; an der rechten Wand?
        bcs crash
        sed             ; Punktzahl + 1, dezimal
        lda SCORE
        clc
        adc #1
        sta SCORE
        lda SCORE+1
        adc #0
        sta SCORE+1
        cld
        jsr pause
        jmp loop

crash:  ldx #<bang
        ldy #>bang
        jsr prmsg
        lda SCORE+1
        jsr PRBYTE
        lda SCORE
        jsr PRBYTE
        ldx #<again
        ldy #>again
        jsr prmsg
        lda KBD         ; die letzte Taste des Spiels vergessen
@k:     lda KBDCR
        bpl @k
        lda KBD
        cmp #'J' | $80
        bne :+
        jmp game
:       jmp GETLINE     ; zurück zum Woz Monitor

; slice: eine Scheibe des Tunnels ausgeben, mit dem Auto
slice:  lda LEFTW
        clc
        adc WIDTH
        adc #1
        sta RIGHTW
        cmp CAR
        bcs @end        ; das Auto ist vor der rechten Wand
        lda CAR
        cmp #40
        bcc @end        ; Auto rechts hinaus: anzeigen
        lda RIGHTW
@end:   sta LAST
        ldx #0
@col:   cpx CAR
        bne @nocar
        lda #'*' | $80
        bne @out
@nocar: cpx LEFTW
        beq @w
        cpx RIGHTW
        beq @w
        lda #' ' | $80
        bne @out
@w:     lda #'!' | $80
@out:   jsr ECHO
        inx
        cpx LAST
        bcc @col
        beq @col
        lda #CR
        jmp ECHO

; random: 16-Bit-Galois-Schieberegister, liefert ein Byte in A
random: lsr SEED+1
        ror SEED
        bcc :+
        lda SEED+1
        eor #$B4
        sta SEED+1
:       lda SEED
        rts

; pause: etwa 50 ms, damit das Spiel bei schneller Anzeige spielbar bleibt
pause:  ldx #40
@x:     ldy #0
@y:     dey
        bne @y
        dex
        bne @x
        rts

; prmsg: die mit null abgeschlossene Zeichenkette ausgeben, deren Adresse in Y:X steht
prmsg:  stx MSG
        sty MSG+1
        ldy #0
:       lda (MSG),y
        beq :+
        ora #$80
        jsr ECHO
        iny
        bne :-
:       rts

title:  .byte CR
        .byte "TUNNEL - CLAUDE IA", CR
        .byte "STEUERE DAS AUTO * DURCH DEN TUNNEL.", CR
        .byte "J = LINKS, L = RECHTS.", CR
        .byte "DER TUNNEL WIRD LANGSAM ENGER...", CR
        .asciiz "TASTE DRUECKEN ZUM STARTEN."
bang:   .byte CR
        .asciiz "UNFALL! PUNKTE: "
again:  .byte CR
        .asciiz "NOCH EINMAL (J/N)? "
