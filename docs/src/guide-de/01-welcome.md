---
version: 1.0
date: November 2026
---

# Willkommen

<p class="lead"><strong>Retro Dev Kit Apple 1</strong> — „Die Erbsünde“ — erweckt Steve Wozniaks Apple-1 (1976) auf einem Linux-PC wieder zum Leben: den Prozessor 6502, die Anzeige mit 40 × 24 Zeichen, den Woz Monitor, die Kassettenschnittstelle ACI und ein eingebautes, zum Integer BASIC kompatibles BASIC (RDK G2 Basic). Vor allem erzeugt es Audiokassetten, die Sie in einen echten Apple-1 einspielen können.</p>

In etwa einer halben Stunde führt Sie diese Anleitung vom heruntergeladenen Archiv bis zu einem eigenen BASIC-Programm, geladen in eine echte Maschine, mit einem Bildschirmfoto zu jedem Schritt. Das **vollständige Handbuch** beschreibt jedes Fenster im Detail, mit ausführlichen Anleitungen zum Integer BASIC und zum 6502-Assembler.

## Was Sie brauchen

<div class="cards">
<div class="card"><h4>Einen PC mit Linux</h4>Eine aktuelle Linux-Distribution, 64 Bit (x86_64) oder 32 Bit (i686). Nichts zu installieren: Der Emulator nutzt die bereits vorhandenen Grafikbibliotheken (X11 oder Wayland, OpenGL/Mesa).</div>
<div class="card"><h4>Das Archiv <code>apple1-emu.zip</code></h4>Beide Programme (Fenster und Terminal), Beispiele, diese Anleitung und das vollständige Handbuch als PDF.</div>
<div class="card"><h4>BASIC ist dabei</h4><strong>RDK G2 Basic</strong>, ein freies, zum Integer BASIC kompatibles BASIC, ist eingebaut. Apples Original von 1976 ist nicht enthalten (es ist noch urheberrechtlich geschützt): Wenn Sie es möchten, verwenden Sie Ihre eigene Kopie, etwa die Kassette <code>BASIC.wav</code> aus der Sammlung „Apple1Cassettes“ auf archive.org.</div>
<div class="card"><h4>Auf Wunsch: ein echter Apple-1</h4>Mit einer ACI-Kassettenkarte: Original, Nachbau oder Bausatz. Ein Smartphone oder der Kopfhörerausgang eines PCs dient als Kassettenrekorder.</div>
</div>

## So lesen Sie diese Anleitung

Befehle, die Sie am Apple-1 eingeben, sind `SO GESCHRIEBEN`. Sitzungen erscheinen auf schwarzem Grund, wie der Bildschirm von damals; was Sie eingeben, ist weiß, die Antwort der Maschine grün:

```mon
» 300R
0300: A9A
```

Unterwegs begegnen Ihnen drei Arten von Kästen:

!!! astuce "Tipp"
    Eine Abkürzung oder ein schnellerer Weg zum selben Ergebnis.

!!! attention "Achtung"
    Eine häufige Falle oder eine Handlung, bei der Sie Ihre Arbeit verlieren können.

!!! apple1 "Am echten Apple-1"
    Was sich ändert, wenn Sie vom Emulator zur echten Maschine wechseln.
