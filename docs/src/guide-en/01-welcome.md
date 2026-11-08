---
version: 1.0
date: November 2026
---

# Welcome

<p class="lead"><strong>Retro Dev Kit Apple 1</strong> — “The Original Sin” — brings Steve Wozniak’s Apple-1 (1976) back to life on a Linux PC: the 6502 processor, the 40 × 24 character display, the Woz Monitor, the ACI cassette interface and a built-in Integer-BASIC-compatible BASIC (RDK G2 Basic). Above all, it makes audio tapes that you can play into a real Apple-1.</p>

In about half an hour, this guide takes you from the downloaded archive to a BASIC program of your own, loaded into a real machine, with a screenshot at every step. The **Complete Manual** covers every panel in detail, with full guides to Integer BASIC and the 6502 assembler.

## What you need

<div class="cards">
<div class="card"><h4>A PC running Linux</h4>A recent Linux distribution, 64-bit (x86_64) or 32-bit (i686). Nothing to install: the emulator uses the graphics libraries already there (X11 or Wayland, OpenGL/Mesa).</div>
<div class="card"><h4>The <code>apple1-emu.zip</code> archive</h4>Both programs (window and terminal), examples, this guide and the Complete Manual in PDF.</div>
<div class="card"><h4>BASIC is included</h4><strong>RDK G2 Basic</strong>, a free BASIC compatible with Integer BASIC, is built in. Apple’s 1976 original is not included (still under copyright): if you want it, use your own copy, such as the <code>BASIC.wav</code> tape of the archive.org “Apple1Cassettes” collection.</div>
<div class="card"><h4>Optional: a real Apple-1</h4>With an ACI cassette card: original, replica or kit. A smartphone or a PC headphone output is the tape player.</div>
</div>

## How to read this guide

Commands to type on the Apple-1 are written `LIKE THIS`. Sessions appear on a black background, like the screen of the time; what you type is white, the machine’s answer green:

```mon
» 300R
0300: A9A
```

Three kinds of boxes appear along the way:

!!! astuce "Tip"
    A shortcut, or a quicker way to get the same result.

!!! attention "Caution"
    A common trap, or an action that can lose your work.

!!! apple1 "On the real Apple-1"
    What changes when you move from the emulator to the real machine.
