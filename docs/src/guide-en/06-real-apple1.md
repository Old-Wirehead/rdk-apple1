# On a real Apple-1: exporting to WAV

Everything you write in the emulator can go to a real Apple-1, a replica or a kit fitted with the ACI cassette card. The audio file replaces the tape; a smartphone or a PC headphone output serves as the tape recorder.

## Making the tape

In the editor, click **📼 Export WAV** and choose a file name. The extension decides the format: `.wav`, or `.aiff` for a phone (the recommended format, see below). The bottom of the editor then shows exactly what to type on the real machine:

![After the export, the bottom of the editor shows the load command.](img/en/editeur-export-bas.png "large")

For `premier.bas`, the command is `C100R` then `004A.00FFR 0F54.0FFFR`. The BASIC program takes two consecutive records on the tape: first BASIC’s zero page (`$004A` to `$00FF`, which holds the program pointers), then the program itself, stored at the top of memory. The **Copy the ranges** button puts the command on the clipboard.

!!! attention "Always four digits"
    Type `004A.00FF`, never `4A.FF`. The ACI ROM does not clear the digits already read when it meets the dot: `4A.FF` would be taken as `$004A` to `$4AFF`, and the machine would wait for 19 KB that never come. The emulator always shows the correct form, and it reproduces this behavior because it runs the real ACI ROM.

## Loading on the real machine

BASIC must already be in memory, loaded from its own tape (`C100R` then `E000.EFFFR`).

<ol class="steps">
<li><strong>Prepare the player.</strong> Copy the file to the phone, connect the headphone output to the ACI “FROM TAPE” input, and set the volume to maximum.</li>
<li><strong>Go back to the monitor</strong> with RESET, then type <code>C100R</code>. The ACI answers <code>C100: A9*</code> and starts a new line; the star is its prompt.</li>
<li><strong>Type the ranges</strong> <code>004A.00FFR 0F54.0FFFR</code>, but do not press RETURN yet.</li>
<li><strong>Start playback</strong>, then press <strong>RETURN during the header</strong>, the steady tone of about 10 seconds that opens each record.</li>
<li><strong>Wait for the <code>\</code> to come back</strong>, then type <code>E2B3R</code> (BASIC warm start, which keeps the program) and <code>RUN</code>.</li>
</ol>

```mon
» C100R

C100: A9*
» 004A.00FFR 0F54.0FFFR
\
» E2B3R

E2B3: 20
» >RUN
HELLO, APPLE-1!
```

!!! apple1 "Reading reliably from a phone"
    - **Volume at maximum.** Accept Android’s “high volume” warning, otherwise the volume stays capped. Many phones lower the volume every time the jack is plugged in again: check it each time.
    - **No sound effects**: turn off the equalizer, Dolby, Mi Sound and any “audio enhancement”.
    - **Airplane mode** or “Do not disturb”, so that no notification mixes into the signal. No Bluetooth.
    - **A simple player** that plays AIFF without processing it; VLC works well.
    - **Never MP3 or AAC**: compression distorts the signal edges. Stay with WAV or AIFF.
    - **Do not trust the ACI LED**: it only lights at about 1.2 V, well above what the card needs.

## Why AIFF at 48 kHz

The emulator’s files are designed for the ACI input. Uncompressed 16-bit AIFF escapes every codec; 48 kHz is Android’s native rate, so the phone does not resample. Both stereo channels are strictly identical, so left and right can be tied together on FROM TAPE without the two outputs fighting each other. The signal is a square wave with steep edges, which is what the ACI comparator detects best.

!!! astuce "Checking a file before playing it"
    On the command line, `apple1-emu info premier.aiff --aci-input stock` simulates the input of the original card at several voltage levels and tells you from which level the file reads correctly (about 0.25 V peak for our files).
