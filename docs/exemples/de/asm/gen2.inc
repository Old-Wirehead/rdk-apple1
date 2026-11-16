; gen2.inc — Softswitches der GEN2-Farbkarte von Uncle Bernie
; (die $C050-$C057 des Apple II, verlegt nach $C25x; ein LESEZUGRIFF schaltet)
SS_GR    = $C250        ; Grafik (verlässt den Textmodus)
SS_TEXT  = $C251        ; Text
SS_FULL  = $C252        ; Vollbild
SS_MIX   = $C253        ; 4 Textzeilen unten
SS_PAGE1 = $C254
SS_PAGE2 = $C255
SS_LORES = $C256
SS_HIRES = $C257
