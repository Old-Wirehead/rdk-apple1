; gen2.inc — conmutadores de la tarjeta de color GEN2 de Uncle Bernie
; (los $C050-$C057 del Apple II, movidos a $C25x; una LECTURA activa el conmutador)
SS_GR    = $C250        ; gráficos (sale del modo texto)
SS_TEXT  = $C251        ; texto
SS_FULL  = $C252        ; pantalla completa
SS_MIX   = $C253        ; 4 líneas de texto abajo
SS_PAGE1 = $C254
SS_PAGE2 = $C255
SS_LORES = $C256
SS_HIRES = $C257
