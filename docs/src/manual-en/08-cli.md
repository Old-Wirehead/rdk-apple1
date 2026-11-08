# The command line

<p class="lead">Two programs: apple1-emu-gui, the graphical interface, and apple1-emu, the terminal emulator and conversion toolbox. Everything the export buttons do can also be done from the command line, for your scripts.</p>

!!! attention "The command-line tools speak French"
    The graphical interface is available in English (`--langue en`), but `apple1-emu` prints its messages and help in French only, and the GUI options, the F2 commands and the debugger keywords are French words (`--carte-couleur`, `charger`, `pas`…). This chapter shows the real output and translates it where needed.

## The terminal emulator

`apple1-emu` with no command runs the Apple-1 in the terminal (at least 42 columns × 28 lines): 8 KB of RAM plus `$E000`, ACI, real speed.

```sh
apple1-emu                                  # 8 KB + E000, ACI, real speed
apple1-emu --fast                           # start in fast mode
apple1-emu --load samples/hello.txt --run 300
apple1-emu --tape samples/hello.wav         # tape in the virtual deck
apple1-emu --basic ~/apple1/integer_basic.bin
apple1-emu --ram 32 --no-e000               # 32 KB, no RAM at E000
```

| Option | Effect |
|---|---|
| `--ram KB` | RAM from `$0000`, 4 to 48 KB (8 by default) |
| `--no-e000` | no RAM at `$E000-$EFFF` (the board’s “W → E” jumper) |
| `--no-aci` | no cassette card |
| `--basic FILE` | your Integer BASIC (4 KB binary or hex dump), loaded at `$E000` |
| `--load FILE[@ADDR]` | a program into memory (`.txt`, `.bin@ADDR`, `.wav`, `.aif`); repeatable |
| `--tape FILE` | a tape in the deck |
| `--type FILE` | types a text file at startup |
| `--run ADDR` | types `ADDRR` after loading |
| `--fast`, `--no-throttle` | fast mode; display without the 60 characters/s limit |
| `--no-refresh` | without the DRAM refresh model (1.0227 MHz instead of 0.960) |
| `--carte-couleur`, `--couleur-50hz` | the GEN2 card (registers and memory; the color picture exists only in the graphical interface) |
| `--cursor-hz HZ`, `--cursor-duty F` | cursor blinking (2.18 Hz, 2/3 visible by default) |
| `--monitor-rom`, `--monitor-page` | another Woz Monitor image (256 or 512 bytes, page 0 or 1) |
| `--aci-rom`, `--aci-page` | another image of the ACI PROM |

**Terminal keys.** <kbd>Ctrl</kbd>+<kbd>R</kbd> RESET, <kbd>Ctrl</kbd>+<kbd>L</kbd> CLEAR SCREEN, <kbd>F1</kbd> help, <kbd>F2</kbd> command line, <kbd>F3</kbd> speed, <kbd>F4</kbd> display limit, <kbd>F5</kbd>/<kbd>F6</kbd> tape play/stop, <kbd>F10</kbd> or <kbd>Ctrl</kbd>+<kbd>Q</kbd> quit. Text pasted into the terminal is typed on the Apple-1 keyboard. Backspace types `_`, the Woz Monitor rub-out.

**F2 commands** (the command words are French).

| Command | Effect |
|---|---|
| {{charger}} `FILE[@ADDR]` | *load*: loads a `.txt`, a `.bin` (`@ADDR` required), a `.wav` or an `.aiff` into memory |
| {{sauver}} `FILE START.END …` | *save*: saves memory; the extension selects the format. The ranges can be copied from the ACI line: `sauver st.aiff 004A.00FFR 0300.0FFFR` |
| {{cassette}} `FILE` | *tape*: inserts a tape, which starts at the ACI’s first read |
| `lecture`, `stop`, `rembobiner`, `ejecter` | the deck: play, stop, rewind, eject |
| {{enregistrer}} `FILE` | *record*: arms the recorder; then type `C100R` and `0300.03FFW` |
| `basic FILE` | loads your Integer BASIC at `$E000` |
| `taper FILE` | *type*: types a text file on the keyboard |
| `reset`, `effacer`, `rapide`, `reel`, `debit`, `aide`, `quitter` | reset, clear, fast, real speed, display rate, help, quit |

## Conversions

The output format is chosen by the **extension**: `.aif`/`.aiff` gives an AIFF (smartphone profile), `.wav` a WAV, `.txt`/`.hex` a Woz dump, `.bin` a binary.

{{export}} (alias of `export-wav`) produces a tape. The signal is created by the real ACI ROM running in the emulated 6502: the bit timings are those of an Apple-1 executing `W`.

```sh
apple1-emu export samples/hello.txt -o hello.aiff
apple1-emu export prog.bin@300 -o prog.wav --rate 48000 --bits 16
apple1-emu export startrek.txt --range "004A.00FFR 0300.0FFFR" -o startrek.aiff
```

| Option | WAV (default) | AIFF (default) |
|---|---|---|
| `--rate` | 44,100 Hz | 48,000 Hz |
| `--bits 8\|16` | 16 | 16 |
| `--channels 1\|2` | 1 | 2, identical |
| `--amplitude` (0.05 to 1) | 0.8 | 0.89 (−1 dBFS) |
| `--silence S` | 0.5 s | 1.0 s |
| `--range` | one range, or a whole ACI line; repeatable | |
| `--no-refresh`, `--no-embed` | bits 6 % shorter; no addresses in the file | |

{{import}} (alias of `import-wav`) decodes a tape: `-o prog.txt` or `-o prog.bin`, `--addr ADDR` for a tape without addresses, `--range` to place several records.

{{info}} describes a file (records, addresses, length); with `--aci-input stock|mimeo|gen2`, `--volts` and `--series`, it simulates the analog input of the ACI (section 5.8).

{{convert}} `INPUT OUTPUT` converts between `.txt`, `.bin`, `.wav` and `.aiff`.

## BASIC and assembler

```sh
apple1-emu basic game.bas -o game.wav          # tokenized listing -> tape
apple1-emu basic game.bas -o game.txt --ram 16 # Woz dump, for 16 KB
apple1-emu basic game.bas                      # check only
apple1-emu asm game.s -o game.wav              # 6502 source -> tape
apple1-emu asm game.s -o game.bin -l game.lst  # binary + listing
apple1-emu asm game.s -I ../lib                # check, extra include path
```

{{basic (command line)|`basic`}} tokenizes a listing as the interpreter does and writes the `$004A-$00FF` block, then the program; `--ram 8|12|16|32` selects the target machine (by default, the smallest that is enough). {{asm (command line)|`asm`}} assembles a ca65 (or xa) source; without `-o`, it only checks. Both print the line to type on the real Apple-1.

```text title="Output of apple1-emu asm hello.s -o hello.txt"
33 octets, $0300-$0320 ; 2 fichier(s), 13 symboles
Cassette (après C100R) : 0300.0320R   puis 300R
écrit : hello.txt
```

That is: “33 bytes, $0300-$0320; 2 file(s), 13 symbols / Tape (after C100R): 0300.0320R then 300R / written: hello.txt”.

## ASCII art

{{ascii}} `IMAGE`: `--cols`, `--rows` (23 to keep the prompt), `--zoom`, `--cx`, `--cy`, `--brightness`, `--contrast`, `--gamma`, `--invert`, `--dither`, `--no-auto-level`, `--ramp simple|full|slideshow|LIST`, `--format asm|basic`, `--title`, `-o` (`.txt`, `.bas`, `.s`, `.wav`, `.aif`).

## PROMs

{{prom}} prepares files for programming real 256 × 4 PROMs (82S129, 3601, 6301), in nibble-dump format:

```sh
apple1-emu prom split woz -o proms/     # PROM-A2.bin (D7-D4) and PROM-A1.bin (D3-D0)
apple1-emu prom split aci -o proms/     # PROM-A3.bin (D7-D4) and PROM-A4.bin (D3-D0)
apple1-emu prom join A2.bin A1.bin      # reassemble a read-out and compare it
```

`--swap-a1-a7` is only for a board wired after the 1976 Apple schematic, which crosses address lines 1 and 7 of PROM A2.

To burn a 2716-to-27C512 EPROM or a 28C EEPROM from the graphical interface (a TL866II+, T48 or T56 programmer with `minipro`), see the EPROM programmer, section 2.6.

## The graphical interface

`apple1-emu-gui [OPTIONS] [FILE…]` loads the given files at startup. Main options (the option names are French):

| Option | Effect |
|---|---|
| `--zoom N\|ajuster`, `--taille WxH` | screen zoom (`ajuster` = fit), window size |
| `--phosphore vert\|blanc\|ambre` | phosphor (green, white, amber) |
| `--screening`, `--sans-screening`, `--screening-options LIST` | vintage screen of the Apple-1 and its settings |
| `--gen2-screening` | vintage screen on the GEN2 color monitor |
| `--theme clair\|sombre\|systeme` | light, dark or system theme |
| `--taille-icones auto\|petite\|normale\|grande\|tres-grande` | icon and text size |
| `--barre-complete` | never compact the toolbar to icons only |
| `--bibliotheque FOLDER`, `--basic FILE` | library folder, BASIC file |
| `--carte-couleur`, `--sans-carte-couleur`, `--couleur-50hz` | GEN2 card |
| `--langue CODE\|auto` | interface language (`--langue en` for English) |
| `--mode-rapide`, `--mode-cassette` | library loading: instant (default), or through the tape at real speed |
| `--cassette FILE` | inserts a tape |
| `--ecran-separe`, `--debogueur` | screen in its own window, debugger open |
| `--editeur FILE`, `--ascii FILE` | opens a file in the editor or the ASCII tool |

For demonstrations and screenshots, some options replay actions after a few seconds: `--lancer-biblio S:NAME`, `--charger-biblio S:NAME`, `--parametres S:NAME`, `--preferences S` (and `--onglet-prefs interface|editeur|bibliotheque|cassette|gen2|avance`), `--editeur-lancer S`, `--editeur-recherche TEXT` (search bar open), `--editeur-menu` and `--ecran-menu` (right-click menus open), `--menu-etat` (the State menu open), `--eprom` (the EPROM programmer open), `--fenetre-fichiers` (the Load window open), `--editeur-exporter S:FILE`, `--ascii-reglages R`, `--ascii-lancer S`, `--taper TEXT`, `--taper-a S:TEXT`, `--dbg-arret`, `--dbg-surveiller`, `--dbg-commande S:CMD`, `--capture IMAGE.png` (after `--delai S`) and `--quitter`.

# Troubleshooting and FAQ

## The window does not open

The OpenGL driver is missing, or you are in a virtual machine. `run.sh` automatically retries with Mesa software rendering; by hand: `LIBGL_ALWAYS_SOFTWARE=1 ./apple1-emu-gui`. The vintage screen stays smooth with software rendering.

## Keys do not reach the Apple-1

Click the Apple-1 screen. When a text field or a dialog box is active, the keyboard goes to that field; the status bar shows where the keyboard goes.

## “Integer BASIC not found”

BASIC is not supplied. Put your `BASIC.wav` (or a 4096-byte `.bin`) in the library, or point to it with the **BASIC** button or **⚙ Preferences → Integer BASIC**. The emulator then finds it by itself.

## A BASIC program stops with STOPPED AT

A key was pressed while it was running: on the Apple-1, any key interrupts a BASIC program. Type `RUN` to start again, or `GOTO` followed by the line number to resume without clearing the variables.

## The program does not start after loading

- BASIC program: you need `E2B3R` (warm entry), not `E000R`, which erases the program. BASIC must be in memory **before** the program.
- Machine code: check the run address in the ⚙ settings; it is not always `300R` (`280R`, `1000R`, `2000R`…).
- Program larger than 8 KB: the RAM is added by ▶, but not by a manual load. Start it with ▶, or use `--ram 16`.

## The tape does not load on the real Apple-1

<ol class="steps">
<li>Volume at maximum, every audio effect off, no notifications.</li>
<li>RETURN during the header, not before playback starts nor after the header ends.</li>
<li>The exact ranges, with four digits: <code>004A.00FFR</code>, never <code>4A.FFR</code>.</li>
<li>Try another player (VLC), then <code>--rate 44100</code>, <code>--channels 1</code>, or <code>--no-refresh</code>.</li>
<li>Check the level with <code>apple1-emu info prog.aiff --aci-input stock</code> (section 5.8).</li>
<li>If loading never ends: RESET, then try again with more volume.</li>
</ol>

## The screen is full of inverse @

That is the GEN2 color monitor: its memory starts at zero, and code `$00` is an inverse `@`. The program has not filled the text page yet. On the real card the contents would be random.

## Characters come out slowly

This is normal: the Apple-1 terminal displays about 60 characters per second. Click **Display: ~60 chars/s** in the status bar (or **Fast** in the display row of the screen) for instant display, and switch to fast mode (<kbd>F3</kbd>) to speed everything up.

## The tape sound is muted

It is off at fast speed, and when ALSA or the sound card is missing: the oscilloscope says so (“muted at full speed”, “no audio output”). The volume is set in the oscilloscope; 🔊 in the toolbar turns the sound off or on.

## The GitHub project does not build

“Test the latest build” builds with ca65/ld65 (package `cc65`) if it finds neither a Release nor an artifact. Install `cc65`, or add an `apple1.toml` that names the right file. Some sources need a recent ca65 (2.19 or later); ca65’s error is then shown as is.

## What the emulator does not do

- Undocumented 6502 opcodes are treated as NOPs of the right length, and JAM opcodes halt the processor; their exact effects are not reproduced (Apple-1 software does not use them).
- The ACI comparator is ideal during emulation; only the `info` command simulates it as an analog circuit.
- The emulation has been checked against the ROMs, the listings and independent tools (c2t, ca65, xa), but the audio files have not yet been played into a real Apple-1 by the author of this manual.
