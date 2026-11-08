; gen2.inc — soft switches of Uncle Bernie's GEN2 color card
; (the Apple II $C050-$C057, moved to $C25x; a READ flips the switch)
SS_GR    = $C250        ; graphics (leaves text mode)
SS_TEXT  = $C251        ; text
SS_FULL  = $C252        ; full screen
SS_MIX   = $C253        ; 4 lines of text at the bottom
SS_PAGE1 = $C254
SS_PAGE2 = $C255
SS_LORES = $C256
SS_HIRES = $C257
