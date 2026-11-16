; plab-couleurs.s — Farbmuster auf der P-LAB-Grafikkarte (TMS9918A)
; Sie brauchen die P-LAB-Karte (Schaltfläche „P-LAB“ der Werkzeugleiste, oder --plab).
; Modus „Graphics I“ (32 x 24 Kacheln); die Karte liegt bei $CC00 (Daten) / $CC01 (Steuerung).
        .include "apple1.inc"
        .org $0300

VDATA   = $CC00
VCTRL   = $CC01

start:  ldx #0
@regs:  lda regs,x          ; R0..R7
        sta VCTRL
        txa
        ora #$80
        sta VCTRL
        inx
        cpx #8
        bne @regs

        ; Mustertabelle ($0000): 256 volle Kacheln (2 KB $FF)
        lda #$00
        ldy #$40
        jsr vadr
        ldy #8              ; 8 Seiten zu 256 Bytes
@pm:    ldx #0
@pl:    lda #$FF
        sta VDATA
        inx
        bne @pl
        dey
        bne @pm

        ; Farbtabelle ($2000): 32 Gruppen zu 8 Kacheln
        lda #$00
        ldy #$60
        jsr vadr
        ldx #0
@col:   txa
        asl a
        asl a
        asl a
        asl a               ; Vordergrundfarbe = X (0..15), dann von vorn
        ora #$01            ; schwarzer Hintergrund
        sta VDATA
        inx
        cpx #32
        bne @col

        ; Namenstabelle ($1400): 24 Zeilen zu 32 Kacheln, diagonal versetzt
        lda #$00
        ldy #$54
        jsr vadr
        ldy #0              ; Y = Zeile
@row:   ldx #0
@nm:    tya
        asl a
        asl a               ; Zeile x 4
        stx $00
        clc
        adc $00             ; + Spalte
        sta VDATA
        inx
        cpx #32
        bne @nm
        iny
        cpy #24
        bne @row

        ; Spritetabelle ($1B00): keine Sprites ($D0 = Listenende)
        lda #$00
        ldy #$5B
        jsr vadr
        lda #$D0
        sta VDATA

        jmp GETLINE         ; zurück zum Woz Monitor; das Bild bleibt stehen

; Videoadresse setzen: A = unteres Byte, Y = oberes Byte (+ $40 zum Schreiben)
vadr:   sta VCTRL
        sty VCTRL
        rts

regs:   .byte $00           ; R0: Modus Graphics I
        .byte $C0           ; R1: 16 KB, Anzeige an, kein Interrupt
        .byte $05           ; R2: Namen bei $1400
        .byte $80           ; R3: Farben bei $2000
        .byte $00           ; R4: Muster bei $0000
        .byte $36           ; R5: Sprites bei $1B00
        .byte $07           ; R6: Spritemuster bei $3800
        .byte $04           ; R7: dunkelblauer Rand
