; clavier.s — liest die Tastatur und gibt jede Taste auf dem Bildschirm aus.
; Ziffern erscheinen doppelt; ESC kehrt zum Monitor zurück.
        .include "apple1.inc"
        .org $0300

read:   lda KBDCR       ; eine Taste?
        bpl read        ; Bit 7 = 0: noch nicht
        lda KBD         ; ASCII-Code + $80; das Lesen löscht das Tastensignal
        cmp #$9B        ; ESC
        beq done
        jsr ECHO
        cmp #'0' | $80
        bcc read        ; < '0'
        cmp #'9' | $80 + 1
        bcs read        ; > '9'
        jsr ECHO        ; eine Ziffer: ein zweites Mal ausgeben
        jmp read
done:   jmp GETLINE
