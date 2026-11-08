; clavier.s — reads the keyboard and echoes each key to the screen.
; Digits are printed twice; ESC returns to the monitor.
        .include "apple1.inc"
        .org $0300

read:   lda KBDCR       ; a key?
        bpl read        ; bit 7 = 0: not yet
        lda KBD         ; ASCII code + $80; reading clears the key strobe
        cmp #$9B        ; ESC
        beq done
        jsr ECHO
        cmp #'0' | $80
        bcc read        ; < '0'
        cmp #'9' | $80 + 1
        bcs read        ; > '9'
        jsr ECHO        ; a digit: print it a second time
        jmp read
done:   jmp GETLINE
