---
version: 1.0
date: Novembre 2026
---

# Bienvenue

<p class="lead"><strong>Retro Dev Kit Apple 1</strong> — «&#8239;Le péché originel&#8239;» — fait revivre l’Apple-1 de Steve Wozniak (1976) sur un PC Linux : le processeur 6502, l’écran de 40 × 24 caractères, le Woz Monitor, l’interface cassette ACI et un BASIC intégré compatible avec l’Integer BASIC (le RDK G2 Basic). Surtout, il fabrique des cassettes audio que l’on peut jouer dans un vrai Apple-1.</p>

Ce guide vous emmène, en une demi-heure environ, de l’archive téléchargée jusqu’à un programme BASIC écrit par vous et chargé dans une véritable machine. Chaque étape est illustrée par une capture de l’émulateur tel que vous le verrez. Le **Manuel complet** reprend ensuite chaque panneau en détail et contient les guides de l’Integer BASIC et de l’assembleur 6502.

## Ce qu’il vous faut

<div class="cards">
<div class="card"><h4>Un PC sous Linux</h4>Une distribution Linux récente, en 64 bits (x86_64) ou en 32 bits (i686). Rien à installer : l’émulateur utilise les bibliothèques graphiques déjà présentes (X11 ou Wayland, OpenGL/Mesa).</div>
<div class="card"><h4>L’archive <code>apple1-emu.zip</code></h4>Elle contient les deux programmes (fenêtre et terminal), des exemples, ce guide et le manuel complet en PDF.</div>
<div class="card"><h4>Le BASIC est fourni</h4>Le <strong>RDK G2 Basic</strong>, un BASIC libre compatible avec l’Integer BASIC, est intégré. L’original d’Apple (1976) n’est pas fourni (il reste protégé) : si vous le voulez, utilisez votre propre copie, par exemple la cassette <code>BASIC.wav</code> de la collection « Apple1Cassettes » d’archive.org.</div>
<div class="card"><h4>En option : un vrai Apple-1</h4>Avec sa carte cassette ACI, une réplique ou un kit. Un smartphone ou la sortie casque d’un PC sert de lecteur de cassettes.</div>
</div>

## Comment lire ce guide

Les commandes à taper au clavier de l’Apple-1 sont écrites `COMME CECI`. Les sessions complètes apparaissent sur fond noir, comme sur l’écran de l’époque ; ce que vous tapez s’y détache en blanc, la réponse de la machine reste en vert :

```mon
» 300R
0300: A9A
```

Trois sortes d’encadrés jalonnent le texte :

!!! astuce "Astuce"
    Un raccourci ou une façon plus rapide d’arriver au même résultat.

!!! attention "Attention"
    Un piège courant, ou une manipulation qui peut faire perdre du travail.

!!! apple1 "Sur le vrai Apple-1"
    Ce qui change quand vous passez de l’émulateur à la machine réelle.
