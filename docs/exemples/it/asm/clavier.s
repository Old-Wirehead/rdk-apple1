; clavier.s — legge la tastiera e mostra ogni tasto sullo schermo.
; Le cifre compaiono due volte; ESC torna al monitor.
        .include "apple1.inc"
        .org $0300

read:   lda KBDCR       ; un tasto?
        bpl read        ; bit 7 = 0: non ancora
        lda KBD         ; codice ASCII + $80; la lettura azzera il segnale del tasto
        cmp #$9B        ; ESC
        beq done
        jsr ECHO
        cmp #'0' | $80
        bcc read        ; < '0'
        cmp #'9' | $80 + 1
        bcs read        ; > '9'
        jsr ECHO        ; una cifra: la si stampa una seconda volta
        jmp read
done:   jmp GETLINE
