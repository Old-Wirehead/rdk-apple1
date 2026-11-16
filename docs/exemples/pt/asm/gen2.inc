; gen2.inc — comutadores da placa a cores GEN2 de Uncle Bernie
; (os $C050-$C057 do Apple II, movidos para $C25x; uma LEITURA aciona o comutador)
SS_GR    = $C250        ; gráficos (sai do modo de texto)
SS_TEXT  = $C251        ; texto
SS_FULL  = $C252        ; ecrã inteiro
SS_MIX   = $C253        ; 4 linhas de texto em baixo
SS_PAGE1 = $C254
SS_PAGE2 = $C255
SS_LORES = $C256
SS_HIRES = $C257
