# Installazione su Linux

## Decomprimere e avviare

<ol class="steps">
<li><strong>Decomprimete l'archivio.</strong> Clic destro su <code>apple1-emu.zip</code>, poi «Estrai qui». Compare una cartella <code>apple1-emu</code>.</li>
<li><strong>Fate doppio clic su <code>run.sh</code>.</strong> Se il gestore di file chiede che cosa fare del file (per esempio su Linux Mint), scegliete <strong>«Esegui»</strong> (non «Visualizza»). Si apre la finestra dell'Apple-1, senza terminale.</li>
<li><strong>Facoltativo: aggiungere l'emulatore al menu.</strong> Aprite un terminale nella cartella (clic destro → «Apri nel terminale») e digitate il comando qui sotto. Compare una voce <strong>Retro Dev Kit Apple 1</strong> nella sezione <em>Giochi</em> del menu, insieme ai comandi <code>apple1-emu-gui</code> (finestra) e <code>apple1-emu</code> (terminale).</li>
</ol>

```sh
./install.sh             # menu delle applicazioni + comandi in ~/.local/bin
./install.sh --bureau    # lo stesso, più un'icona sul desktop
./install.sh --desinstaller
```

La disinstallazione (`--desinstaller`) rimuove i programmi e la voce del menu, ma conserva i vostri programmi (`~/apple1/programmes`) e le vostre impostazioni (`~/.config/apple1-emu`). Nessun comando richiede `sudo`: tutto si installa nella vostra cartella personale. Il primo avvio di una nuova versione propone di ripartire dalle impostazioni originali; la libreria viene conservata. Il pulsante **Reimposta le preferenze** fa la stessa cosa. Un file vuoto chiamato `portable` accanto al programma (oppure `APPLE1_PORTABLE=1`) tiene preferenze, libreria predefinita e cache dentro la cartella del kit.

!!! astuce "Aggiornare da una versione precedente"
    Gli script `lancer.sh` e `installer.sh` delle versioni precedenti si chiamano ora `run.sh` e `install.sh`. Eseguite di nuovo `./install.sh`: sostituisce la vecchia installazione, la voce del menu e l'icona del desktop che puntavano a `lancer.sh`.

!!! astuce "La versione da terminale"
    `./run.sh --terminal` apre l'emulatore in un terminale (almeno 42 colonne × 28 righe), senza interfaccia grafica. Comodo attraverso una connessione SSH. Il Manuale completo ne descrive i tasti e i comandi.

## Se la finestra non si apre

`run.sh` rileva un errore immediato (driver grafico, macchina virtuale senza accelerazione) e riprova da solo con il rendering software di Mesa (`LIBGL_ALWAYS_SOFTWARE=1`). Se fallisce anche questo, compare un messaggio e il registro si trova in `~/.cache/apple1-emu/gui.log`.

| Sintomo | Che cosa fare |
|---|---|
| Il gestore di file apre `run.sh` nell'editor di testo | Clic destro → Proprietà → Permessi: spuntate «Consenti l'esecuzione del file come programma», poi fate di nuovo doppio clic e scegliete «Esegui». |
| «Permesso negato» nel terminale | `chmod +x run.sh install.sh bin/*/apple1-*` |
| Finestra nera o molto lenta | Normale in una macchina virtuale: il rendering software resta fluido per lo schermo dell'Apple-1. Se serve, disattivate lo «Schermo vintage». |
| Nessun suono della cassetta | Il suono passa per ALSA; senza scheda audio resta semplicemente muto. Il caricamento funziona lo stesso. |

## Dove sono i vostri file

| Posizione | Contenuto |
|---|---|
| `~/apple1/programmes` | La **libreria**: le vostre cassette, i programmi e gli esempi (creata al primo avvio). |
| `~/.config/apple1-emu/gui.json` | Impostazioni: zoom, colori, disposizione dei pannelli, schede dell'editor. |
| `~/.cache/apple1-emu/` | Cassette convertite, progetti GitHub scaricati, il registro `gui.log`. |
