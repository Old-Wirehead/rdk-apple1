; gen2.inc — interruttori software della scheda a colori GEN2 di Uncle Bernie
; (i $C050-$C057 dell'Apple II, spostati in $C25x; una LETTURA scatta l'interruttore)
SS_GR    = $C250        ; grafica (esce dal modo testo)
SS_TEXT  = $C251        ; testo
SS_FULL  = $C252        ; schermo intero
SS_MIX   = $C253        ; 4 righe di testo in basso
SS_PAGE1 = $C254
SS_PAGE2 = $C255
SS_LORES = $C256
SS_HIRES = $C257
