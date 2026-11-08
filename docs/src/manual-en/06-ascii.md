# Photo → ASCII

<p class="lead">The Photo → ASCII tool turns a photo into a 40 × 24-character Apple-1 picture, then into a BASIC or 6502 program, ready to run or to export to tape.</p>

![The Photo → ASCII tool: the photo and its crop frame, the settings, and the preview drawn with the 2513 font.](img/en/brut/ascii-outil.png "large")

## Opening a picture

**🎨 ASCII** opens the tool, a dockable panel like the others. **Drag and drop** a `.jpg` or `.png` photo onto the window, or use **📂 Open…** (a photo, or a finished ASCII art in `.txt`).

The green frame on the photo shows the part being converted: drag it to move it, the mouse wheel zooms.

## Settings

| Setting | Role |
|---|---|
| **Columns**, **Rows** | 40 × 24 at most. **23 rows** leaves the last line to the prompt, so that the picture does not scroll. |
| **Zoom**, **Centre** | the part of the photo kept |
| **Brightness**, **Contrast**, **Gamma** | the usual settings (gamma > 1 lightens) |
| **Invert** | dark = a lot of ink |
| **Auto levels** | stretches the gray levels (2 % to 98 %) before the other settings |
| **Dithering** | Floyd-Steinberg: more shades, more noise |
| **Characters** | **Full** ramp (26 levels), **Simple** (10), the one from **Schmenk’s slideshow**, or **Custom** |

Only the characters of the 2513 are possible: upper case, digits and symbols, with no lower case, no `"` and no `_`. They are ranked by the number of dots lit in the Apple-1 font: that number, not the shape, gives the gray level.

!!! astuce "For a portrait"
    40 × 24 characters is not much: crop on the face (zoom 1.3 to 1.6), raise the contrast a little, and try a gamma below 1 for a bright photo. A plain dark background gives the best results.

## The program produced

- **BASIC**: one `PRINT "…"` line per line of the art. A full 40-character line ends with `;`, because the screen wraps by itself. The program is slow to type in (about 2 s per line).
- **6502**: a ca65 source at `$0300`, started with `300R`. After clearing the screen, a loop sends a compressed table to ECHO (runs of identical characters encoded as “count + character”). The display is instant.

With 24 rows, the program waits for a key before returning; with 23, the prompt lands on the last line and the picture stays whole.

The buttons: **✎ Insert into the editor** (new BASIC or 6502 tab), **▶ Run**, **📼 Export WAV…**, **💾 Save to the library** (a `.txt` in the “Art ASCII” folder), **📋 Copy the text**, **Show the code**.

![From the photo to the real Apple-1: conversion, program, tape.](diagrams/en/workflow-ascii.svg "diagram large")

## On the command line

```sh
apple1-emu ascii photo.jpg                       # the art alone, on screen
apple1-emu ascii woz.jpg --zoom 1.3 --cx 0.45 --cy 0.32 \
          --contrast 1.3 --gamma 0.45 --rows 23 -o woz.txt
apple1-emu ascii woz.jpg -o woz.bas              # BASIC program
apple1-emu ascii woz.jpg -o woz.s                # 6502 source
apple1-emu ascii woz.jpg -o woz.wav              # 6502 tape, started with 300R
apple1-emu ascii woz.jpg -o woz.wav --format basic
```

Chapter 12 gives all the options.

!!! apple1 "Dave Schmenk’s slideshow"
    In 2014, Dave Schmenk wrote a portrait slideshow for the Apple-1. The library adds it as a GitHub project (`dschmenk/Apple1-Slideshow`): it is downloaded, not supplied, because the repository has no license. **▶ Test the latest build** assembles it (xa syntax, translated automatically; the same 3456 bytes as xa 2.4.1) and runs it with `280R`.
