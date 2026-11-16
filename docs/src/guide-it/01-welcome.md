---
version: 1.0
date: novembre 2026
---

# Benvenuti

<p class="lead"><strong>Retro Dev Kit Apple 1</strong> — «Il peccato originale» — riporta in vita l'Apple-1 di Steve Wozniak (1976) su un PC con Linux: il processore 6502, lo schermo da 40 × 24 caratteri, il Woz Monitor, l'interfaccia a cassetta ACI e un BASIC integrato compatibile con l'Integer BASIC (RDK G2 Basic). Soprattutto, crea cassette audio che potete riprodurre su un vero Apple-1.</p>

In circa mezz'ora, questa guida vi porta dall'archivio scaricato a un vostro programma BASIC, caricato su una macchina vera, con una schermata a ogni passo. Il **Manuale completo** descrive ogni pannello in dettaglio, con guide complete all'Integer BASIC e all'assembler 6502.

## Che cosa serve

<div class="cards">
<div class="card"><h4>Un PC con Linux</h4>Una distribuzione Linux recente, a 64 bit (x86_64) o a 32 bit (i686). Niente da installare: l'emulatore usa le librerie grafiche già presenti (X11 o Wayland, OpenGL/Mesa).</div>
<div class="card"><h4>L'archivio <code>apple1-emu.zip</code></h4>I due programmi (finestra e terminale), esempi, questa guida e il Manuale completo in PDF.</div>
<div class="card"><h4>Il BASIC è incluso</h4><strong>RDK G2 Basic</strong>, un BASIC libero compatibile con l'Integer BASIC, è integrato. L'originale di Apple del 1976 non è incluso (è ancora protetto dal diritto d'autore): se lo volete, usate una vostra copia, come la cassetta <code>BASIC.wav</code> della raccolta «Apple1Cassettes» di archive.org.</div>
<div class="card"><h4>Facoltativo: un vero Apple-1</h4>Con una scheda a cassetta ACI: originale, replica o kit. Uno smartphone o l'uscita cuffie di un PC fa da registratore.</div>
</div>

## Come leggere questa guida

I comandi da digitare sull'Apple-1 sono scritti `COSÌ`. Le sessioni compaiono su fondo nero, come lo schermo dell'epoca; ciò che digitate è in bianco, la risposta della macchina in verde:

```mon
» 300R
0300: A9A
```

Lungo il percorso trovate tre tipi di riquadri:

!!! astuce "Suggerimento"
    Una scorciatoia, o un modo più rapido per ottenere lo stesso risultato.

!!! attention "Attenzione"
    Una trappola frequente, o un'azione che può farvi perdere il lavoro.

!!! apple1 "Sul vero Apple-1"
    Che cosa cambia passando dall'emulatore alla macchina vera.
