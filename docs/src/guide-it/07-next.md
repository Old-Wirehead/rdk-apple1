# E adesso? {: .newpage }

Ora sapete caricare una cassetta, scrivere ed eseguire un programma BASIC e trasferirlo su una macchina vera. Il **Manuale completo** (in inglese: `docs/Complete-Manual.pdf`) va molto più lontano:

- **Guida all'Integer BASIC**: ogni istruzione con esempi, i limiti, i messaggi di errore, i bug noti e come aggirarli, l'adattamento dei programmi Applesoft.
- **Guida all'assembler 6502**: l'assembler integrato (sintassi di ca65, macro, listato), le routine del Woz Monitor, esempi provati nell'emulatore.
- **Giochi da giocare e da leggere**: la libreria contiene cinque piccoli giochi di Claude IA (Impiccato, Fiammiferi, Battaglia navale in BASIC; Tunnel e Tris in assembler), ciascuno in francese e in inglese.
- **Gli strumenti**: debugger, libreria e progetti GitHub, scheda a colori GEN2 e scheda grafica P-LAB, foto → ASCII, schermo vintage, riga di comando.

## Scorciatoie essenziali

| Tasto | Effetto |
|---|---|
| Ctrl+R | RESET (ritorno al monitor) |
| Ctrl+L | CLEAR SCREEN |
| F3 | velocità reale o velocità massima |
| Ctrl+V | digita gli appunti sulla tastiera dell'Apple-1 (anche: clic destro sullo schermo → Incolla) |
| Ctrl+più, Ctrl+meno, Ctrl+0 | zoom dello schermo e zoom adattato alla finestra (nell'editor: dimensione del testo) |
| F11 | schermo intero |
| F1 | fumetti di aiuto dettagliati (come il pulsante ?) |
| Backspace | `_`, la cancellazione del monitor |
| Ctrl+F, Ctrl+H, Ctrl+G | nell'editor: cerca, sostituisci, vai a una riga |
| F5, F9 | nell'editor: esegui la scheda, punto di interruzione sulla riga |

## Comandi da ricordare

| Comando | Effetto |
|---|---|
| `E000R` | avvia l'Integer BASIC (a freddo: cancella il programma) |
| `E2B3R` | torna al BASIC senza cancellare il programma |
| `C100R` | avvia il programma dell'ACI (prompt `*`) |
| `inizio.fineR` | legge la cassetta in quell'intervallo, per esempio `0300.031ER` |
| `300R` | esegue un programma in linguaggio macchina in `$0300` |
