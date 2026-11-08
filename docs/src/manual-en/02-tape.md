# The tape and the ACI card

<p class="lead">The Apple-1 had neither floppy disk nor BASIC in ROM: everything went through an audio tape and the ACI card (Apple Cassette Interface). The emulator runs the real ACI ROM, so its WAV and AIFF files load on a real Apple-1, and recordings made on a real Apple-1 load in the emulator.</p>

## File formats

| Extension | Contents | Read | Write |
|---|---|---|---|
| `.wav` | ACI tape, 8- to 32-bit PCM or float, mono or stereo, any sample rate | yes | 44.1 kHz mono 16-bit by default |
| `.aif`, `.aiff` | ACI tape (AIFF and AIFF-C) | yes | smartphone profile: 48 kHz, 2 identical channels, 16-bit |
| `.txt`, `.hex`, `.apl` | Woz Monitor dump: lines `0300: A9 C1 20 …` | yes | yes |
| `.bin` | raw bytes, no address (`FILE.bin@300`) | yes | yes |
| `.bas` | Integer BASIC listing (text) | in the editor | in the editor |
| `.s`, `.asm`, `.a65` | 6502 source (ca65, or xa) | in the editor | in the editor |

Tapes exported by the emulator carry their addresses in a comment chunk of the file (`APPLE1 ACI 0300.031E`), which audio players ignore. A tape from elsewhere has no addresses: the library recognizes or guesses them (chapter 4), and the command line accepts `--addr` or `--range`.

The decoder tolerates noise, low levels, inverted polarity, a tape speed error of ±10 % and several records in one file. WAV files made by Egan Ford’s **c2t** tool are read too. A **mains hum** (50 or 60 Hz, even as strong as the tape signal itself) is filtered out before the edges are detected, so a long recording made with a poorly shielded cable still reads correctly.

## The virtual deck

The **Tape:** strip of the toolbar gathers every tape control: **📼 Insert**, **▶** play, **⏹** stop, **⏪** rewind, **⏺** record, **⏏** eject, then the tape’s name and position, the oscilloscope **📈** and the sound **🔊**. A tape is inserted with **📼 Insert** (a `.wav` or `.aiff`), or by right-clicking a library program (“Insert as a tape”): you then type `C100R` and the range yourself. The **📼** icon of a library program goes one step further: it inserts the tape and types the commands for you.

![The Tape: strip while BASIC.wav is being read: 7.6 seconds out of 35.8.](img/en/barre-cassette.png "large")

The tape follows **emulated** time: at fast speed (<kbd>F3</kbd>), loading takes far less than the 10-second header, and it still works. Without the tape, library loads are instant (chapter 4).

Loading Integer BASIC “the old way” goes like this:

```mon
» C100R
C100: A9*
» E000.EFFFR
» E000R

E000: 4C
>
```

`C100R` starts the ACI program, which answers with its `*` prompt. The line `E000.EFFFR` asks it to **read** the tape into `$E000-$EFFF`. When the tape ends, the ACI returns to the monitor (`\`); `E000R` starts BASIC, whose prompt is `>`.

## Sound and oscilloscope

![The oscilloscope while BASIC.wav plays: the “Tape sound” checkbox, the volume, and the signal at 1 ms per division.](img/en/oscilloscope.png "small")

During playback or recording, **🔊** lets you hear the signal, like the speaker of a tape recorder, at low volume by default. **📈** opens the **oscilloscope**: a green trace on a dark graticule, 1 ms per division, showing 10 ms around the playback head, or the last 10 milliseconds written by the Apple-1. You can make out the header (≈ 800 Hz), the 0 bits (≈ 2 kHz) and the 1 bits (≈ 1 kHz). Sound is muted at fast speed.

## Recording from the emulated Apple-1

To produce a tape as in 1976, arm the recorder, then ask the ACI to write:

<ol class="steps">
<li>Click <strong>⏺</strong> and choose the file (<code>.wav</code> or <code>.aiff</code>).</li>
<li>Type <code>C100R</code>, then the range followed by <code>W</code>, for example <code>0300.03FFW</code>, then <kbd>RETURN</kbd>.</li>
<li>The ACI writes a 10-second header, then the data. The file is written 1.5 s after the end, or on <strong>⏹</strong>.</li>
</ol>

The simplest way is still **Save** in the toolbar, or **📼 Export WAV** in the editor: the signal is the same, cycle for cycle, and it is produced instantly.

## How the ACI works

![The two directions of the ACI: when writing, any access to $C0xx toggles the output; when reading, the comparator drives address line A0 of the PROM.](diagrams/en/aci.svg "diagram large")

The card is remarkably simple: a 256-byte PROM visible at `$C000` and `$C100`, a flip-flop and a comparator.

- **Writing.** Every access to an address in `$C000-$C0FF` toggles the output (74LS74). The ROM program sets the time between two accesses: that time encodes the bits.
- **Reading.** The level of the tape input, after the LM311 comparator, drives PROM address line A0 when A7 = 1. The same instruction therefore reads `$C080` or `$C081` depending on the level: the ROM measures the half-periods with a 12-cycle loop.
- **Encoding.** A “1” lasts about twice as long as a “0”: 1 kHz for a 1, 2 kHz for a 0. Bytes go out most significant bit first, with no parity and no checksum.

Appendix C gives the exact timings, in cycles and in microseconds.

## ACI commands

After `C100R`, the ACI waits for a line in monitor format, ending with `R` (read) or `W` (write):

| Line | Effect |
|---|---|
| `0300.03FFR` | reads the tape into `$0300-$03FF` |
| `0300.03FFW` | writes `$0300-$03FF` to tape, with a 10 s header |
| `004A.00FFR 0300.0FFFR` | reads **two** consecutive records: the BASIC zero page, then the program |
| `004A.00FFW800.FFFW` | the run-together form from the 1976 BASIC manual, also accepted |

{{!C100R}}{{!R (read)}}{{!W (write)}}

!!! attention "The zeros of 004A.00FF are mandatory"
    The ROM parser does not clear its hex accumulator at the period (“Doesn’t clear HEX1!!!”, says the listing). The end digits are shifted in behind those of the start: `4A.FF` means `$004A-$4AFF`, almost 19 KB! `0300.0FFF` works because the four digits push the 3 of `0300` out. Always write both addresses with four digits. The emulator reproduces this behavior, since it runs the real ROM.

A BASIC program is saved as two records: the zero page `$004A-$00FF` (which holds the LOMEM and HIMEM pointers and the end of the program), then the program itself, which always ends at HIMEM. To load it back, BASIC must already be in memory; you then re-enter BASIC “warm” with `E2B3R`, which keeps the program.

```mon
» C100R
C100: A9*
» 004A.00FFR 0300.0FFFR
» E2B3R

E2B3: 20
» >RUN
```

## To a real Apple-1

This is what justifies the emulator: prepare a program on the PC, then play it into the ACI of a real machine, from a smartphone or a computer’s headphone output.

<ol class="steps">
<li><strong>Prepare the file.</strong> <strong>📼 Export WAV</strong> in the editor, or <code>apple1-emu export prog.txt -o prog.aiff</code>. Choose <code>.aiff</code> for a smartphone. Note the line displayed, for example <code>C100R</code> then <code>0300.031ER</code>.</li>
<li><strong>Set up the player.</strong> Volume at maximum (accept the “high volume” warning), every audio effect off (equalizer, Dolby, “audio enhancement”), airplane mode, no Bluetooth. A simple player such as VLC is fine.</li>
<li><strong>On the Apple-1</strong>, type <code>C100R</code>: the monitor answers <code>C100: A9*</code>.</li>
<li>Type the read line, for example <code>0300.031ER</code>, but <strong>do not press RETURN yet</strong>.</li>
<li>Start playback, then press <kbd>RETURN</kbd> <strong>during the header</strong> (it lasts about 10 s, after one second of silence).</li>
<li>At the end, the ACI returns to the monitor (<code>\</code>). Run the program: <code>300R</code>, or <code>E2B3R</code> then <code>RUN</code> for a BASIC program.</li>
</ol>

**Why this AIFF profile?** Uncompressed 16-bit, no lossy codec: MP3 or AAC would round off the edges. 48 kHz, Android’s native rate, to avoid any resampling. Two strictly identical channels, because a cable that ties left and right together onto FROM TAPE must never receive two different signals. A square wave at −1 dBFS: the input of the original ACI is a high-pass network (0.01 µF) that sees little more than the edges, and a square wave with steep edges suits it best. One second of silence at the start and at the end, because a phone sometimes clips the beginning of a sound.

!!! apple1 "Do not trust the LED"
    The 1976 manual asks for a level at which the ACI LED is “just on”. Yet it only lights at about 1.2 V, far more than the comparator needs. An unlit LED therefore does not prove that the signal is too weak. If loading never ends, the ROM is still waiting for edges: press RESET and turn the volume up.

!!! attention "Never MP3 or AAC"
    A lossy format shifts the edges and makes the tape unreadable. Use WAV, AIFF, or FLAC if your player decodes it.

## What level is needed? The input models

The `info --aci-input` command simulates the analog input of the card (coupling capacitor, bias, LM311 hysteresis) for a series of peak voltages, and tells you from what level the file would be read:

```sh
apple1-emu info prog.aiff --aci-input stock     # 1976 card, 0.01 µF
apple1-emu info prog.aiff --aci-input mimeo     # 0.1 µF (Mimeo guide advice)
apple1-emu info prog.aiff --aci-input gen2      # Uncle Bernie's ACI Gen2
apple1-emu info prog.aiff --aci-input stock --volts 0.5 --series 1200
```

| Model | What it represents |
|---|---|
| `stock` | the original ACI: 0.01 µF into the comparator biased at 2.5 V, 3k/47k feedback |
| `mimeo` | Mike Willegal’s modification: 0.1 µF, more reliable |
| `gen2` | Uncle Bernie’s ACI Gen2: matched 100 nF, 300k instead of 47k (less hysteresis) |

`--volts` sets the peak voltage at the jack for digital full scale; `--series` adds a series resistance (for example the 1.2 kΩ of a smartphone cable). With the `stock` model, the emulator’s files are read from **about 0.25 V peak**; a smartphone headphone output at maximum usually gives 0.5 to 1 V. This model is simplified: no noise, no drift of the bias point.

## Digitizing a real tape or an Apple-1 recording

To recover a program typed on a real Apple-1, record the ACI’s **TO TAPE** output through a microphone input (a phone, or a PC sound card with Audacity at 44.1 or 48 kHz), with automatic gain control and noise reduction turned off. On the Apple-1: `C100R`, then `0300.03FFW` and <kbd>RETURN</kbd>. Stop a few seconds after the `\` comes back, then:

```sh
apple1-emu import recording.aiff --addr 300 -o prog.txt
apple1-emu import startrek.aiff --range "004A.00FFR 0300.0FFFR" -o startrek.txt
```

!!! apple1 "TO TAPE is a microphone-level output"
    It delivers only about 50 mV peak to peak. Check the waveform in Audacity: no clipping, no level jump during the header. The emulator’s import tolerates any gain and a DC offset.
