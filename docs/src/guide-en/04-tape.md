# Loading and running a tape

On the Apple-1, everything goes through the cassette: Integer BASIC itself was loaded from an audio tape. The emulator reads `.wav` and `.aiff` files the way the original ACI card does, byte by byte, and recognizes the 24 tapes of the “Apple1Cassettes” collection on archive.org.

## Preparing the library

Copy your tapes into the library folder, `~/apple1/programmes`: for instance `Hamurabi.wav`. The list on the right updates by itself. The five folders BASIC, 6502, GEN2 COLOR, WAV and Art ASCII are listed there directly, with no root folder above them; files left loose in the library folder are not shown. Each recognized tape is marked **✔ recognised**, with its address ranges and run command already set.

!!! note "You do not need BASIC.wav"
    The emulator has its own BASIC, RDK G2 Basic, so BASIC programs run without any tape. Apple’s 1976 Integer BASIC is still under copyright and is not included: if you prefer the original, use your own copy (such as `BASIC.wav`) and choose it in **⚙ Preferences → Advanced → BASIC**.

## The quickest way: the ▶ button

Each program in the list has four small icons on its right: **▶** loads and runs, **⬇** loads into memory without running, **📼** loads through the tape, **⚙** opens its settings. A double-click does the same as ▶.

Click **▶** next to `Hamurabi.wav`. The emulator loads BASIC if it is not already in memory, then the program, and types `E2B3R` (back to BASIC without erasing the program) and `RUN` for you. Loading is instant.

![Hamurabi started with the ▶ button of the library.](img/en/hamurabi-ecran.png "small")

For a machine-language program, ▶ does a RESET and then types its run command, for example `300R`. For a BASIC listing or a 6502 source, ▶ also opens it in a tab of the editor, if the editor is open.

## As in 1976: the virtual tape

Loading is instant unless you ask for the tape. Click the **📼** icon of a program to reproduce a real load: the emulator types `C100R` to start the ACI program, then the address range, and the tape plays at real speed. A progress bar appears under the program’s name, and the **Tape:** strip of the toolbar shows the tape and its position.

![Loading BASIC.wav through the virtual ACI, with the tape oscilloscope open.](img/en/brut/cassette-basic.png "large")

The 📈 and 🔊 buttons at the end of the **Tape:** strip open the **oscilloscope** and let you **hear** the signal, like a tape recorder’s speaker. Loading BASIC takes about 36 seconds at real speed.

When the tape stops, the monitor shows `\` again. Type `E000R` to start BASIC. Its prompt is a `>`:

```mon
» E000R

E000: 4C
» >PRINT 6*7
42
```

!!! astuce "Loading by hand"
    **📼 Insert**, in the **Tape:** strip of the toolbar, puts any `.wav` or `.aiff` file in the virtual deck. You then type `C100R` yourself, then the range, for example `E000.EFFFR` for BASIC: the tape starts at the first read, as on the real machine. The **Load** button is for `.txt`, `.bin` and `.hex` files, loaded instantly.

| Program | After loading |
|---|---|
| Integer BASIC | `E000R` (cold start, erases the program in memory) |
| BASIC program | `E2B3R` (warm start, keeps the program), then `RUN` |
| Machine language | its address followed by `R`, often `300R` |
