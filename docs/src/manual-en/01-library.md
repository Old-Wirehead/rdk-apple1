# The library

<p class="lead">The library is the starting point of almost everything: it keeps your tapes, dumps and sources, recognizes the tapes of the time, and runs a program in one click, BASIC included.</p>

![The library panel: the main folder, the “Art ASCII” folder, recognized tapes and a GitHub project. BASIC.wav is playing.](img/en/bibliotheque.png "small")

## Folders

The main folder is `~/apple1/programmes`. It is created on first launch, with the examples sorted into five folders: **Art ASCII** (eleven 40 × 24 ASCII-art examples, chapter 9), **BASIC** (`.bas` and BASIC `.hex.txt` programs), **6502** (`.s`/`.asm` sources with their `.bin`/`.hex`), **GEN2 COLOR** (Bernie's GEN2 card tests, `gen2-*.apl`, and the colour games for G2 Basic) and **WAV** (all the cassettes). In the panel those five folders are listed directly at the top level, in the order **BASIC**, **6502**, **GEN2 COLOR**, **WAV**, **Art ASCII**: the root folder (the directory’s name on disk) is not shown above them, and files left loose in that folder are not listed. Put them into one of the five folders. A folder you add, or a GitHub project, keeps its own title. A library that already exists is never reorganised, except that its **Assembleur** and **Uncle Bernie** folders (the names of versions 7.8 to 7.9.1) are renamed **6502** and **GEN2 COLOR**, with their content kept. To choose another one: **⚙ Preferences → Library folder → 📂 Browse…** (**⟲ Default** goes back to `~/apple1/programmes`).

**+ Folder**, at the top of the panel, adds other folders; the main folder always stays. The library is a tree: folders and subfolders fold up, and their state is remembered. Each folder keeps its own settings in a `programs.json` file. Right-click an added folder to open it in the file manager or remove it from the list (nothing is deleted on disk).

**Narrow panel.** The library can be as narrow as you like (down to a minimum width, below which the divider stops). Each entry then adapts: the name takes the whole width, cut with “…” if it is too long (the tooltip shows the full name), the small icons ▶ ⬇ 📼 ⚙ move to the next line, and the description is limited to one or two lines, also cut with “…”, the full text being in the tooltip. In the header, **+ Folder** and **+ GitHub project** become icons with tooltips.

The files shown are: `.wav .aiff .bin .hex .txt .apl .bas .s .asm .prg`. The list updates itself every two seconds: copy a file into the folder and it appears.

For each program the list shows the name, the type, the address ranges, the tape length, the run command and your notes.

### The games of Claude IA

The **BASIC** and **6502** folders also receive five small games written by Claude IA, each in a French and an English version: *Hangman* (`pendu.bas`, `hangman.bas`), *Matches* (`allumettes.bas`, `matches.bas`) and *Battleship* (`bataille.bas`, `battleship.bas`) in BASIC; *Tunnel* (`tunnel.s`, `tunnel-en.s`) and *Tic-tac-toe* (`morpion.s`, `tictactoe.s`) in 6502 assembly. Double-click a BASIC game to run it on RDK G2 Basic (the emulator starts BASIC, loads the listing and types `RUN`); assembly sources are assembled by the built-in assembler. The assembly sources include `apple1.inc`, which is installed with them. The **GEN2 COLOR** folder also holds *Connect 4 in colour* (`puissance4-couleur.bas`), a game for G2 Basic and the GEN2 card, credited to Claude IA. Q and S move the counter above the grid, Space drops it. The games are part of the project and under its licence (appendix “Licence and credits”).

## Running a program

Each program has four small icons on its right:

| Icon | Action |
|---|---|
| **▶** | **Load and run**: loads the program if needed, then runs it: RESET, then its command typed on the keyboard (`300R`, or `E2B3R` and `RUN` for a BASIC program). If the program needs Integer BASIC and it is not at `$E000`, the emulator loads it first. A BASIC listing is typed into Integer BASIC, a 6502 source is assembled. |
| **⬇** | **Load**: puts the program into memory, instantly, without starting it. |
| **📼** | **Through the tape**: as on the real machine. The emulator presses RESET, types `C100R` then the ACI read command (for example `0300.031ER`), and the tape plays at real speed, with its sound and a progress bar. `.txt` and `.bin` files are converted to a tape first. Greyed out for a source (`.bas`, `.s`), which has no recording. |
| **⚙** | **Parameters**: ranges, load address of a `.bin`, run address and command, type, whether BASIC is needed, notes. |

{{!double-click}}Loading is **instant** unless you choose the tape with 📼. A **double-click** on a program loads it, and runs it if it has a run command (like ▶). The cassette is used only when you ask for it; the deck of the toolbar then shows the tape and its position.

When the editor is open, ▶, ⬇ or a double-click on a BASIC listing or a 6502 source also **opens it in a tab of the editor**, marked 📚; if it is already open, its tab is simply shown.

**Right-clicking** a program offers the rest: load, load and run, insert as a tape without typing anything, parameters, rename, delete (with confirmation), open the folder.

## A program’s settings

![The ⚙ settings of Hamurabi: two ranges (BASIC zero page, then the program), started with E2B3R then RUN.](img/en/brut/bibliotheque-params.png "large")

| Field | Role |
|---|---|
| **Ranges** | The memory areas, written as for the ACI: `0300.031E`, or `004A.00FF 0800.0FFF` for a BASIC program. Each record on the tape goes into its range, in order. |
| **Load address** | For a `.bin`, which holds no address. |
| **Run** | The address and the command typed by ▶. `\n` stands for RETURN between two lines: `E2B3R\nRUN`. |
| **Type** | **Machine code**, **G2 Basic program** (the default for a BASIC listing, since the emulator provides it) or **Integer BASIC program** (for those who prefer Apple’s original: ▶ then loads the original Integer BASIC you chose in the preferences). The type chosen for a program wins over the general BASIC setting of the preferences. |
| **Needs a BASIC** | ▶ will load the BASIC of the program’s type first. |
| **Notes** | Free text, shown in the list. |

Values are prefilled from the file and checked when saved; **⟲ Values from the file** reads them again.

!!! attention "Always four digits: 004A.00FF, never 4A.FF"
    The settings refuse `4A.FF`. This is not a whim: the ACI ROM does not clear its hex accumulator at the period, so the digits keep shifting in and it would read `$004A-$4AFF` (see section 5.6). The emulator therefore always writes addresses with four digits.

## Recognized tapes

When a file is **recognized**, its settings fill themselves in and the list shows **✔ recognised**. The emulator decodes the records and compares their sizes and SHA-256 fingerprints with a built-in table: the 24 tapes of the “Apple1Cassettes” collection on archive.org. The file name then does not matter at all. Failing that, it recognizes the **name** (case, spaces and dashes ignored) with the same record sizes; the help bubble then points out that the contents differ.

| Tape | Ranges | Run |
|---|---|---|
| `BASIC.wav` (Integer BASIC) | `E000.EFFF` | `E000R` |
| `Hamurabi.wav`, `StarTrek.wav`, `Wumpus.wav`, `Slots.wav`, `Craps.wav`, `Bowling.wav`, `Buzzword.wav`, `WordCross.wav`, `AppleiEnhancedCheckers.wav`, `LunarlanderasciiGraphics.wav` | `004A.00FF 0300.0FFF` | `E2B3R ⏎ RUN` |
| `AceyDucey.wav`, `Blackjack.wav`, `HiLo.wav` | `004A.00FF 0800.0FFF` | `E2B3R ⏎ RUN` |
| `Football.wav` | `004A.00FF 0400.2FFF` | `E2B3R ⏎ RUN` (12 KB) |
| `DealOrNoDeal.wav`, `StarTrek2003.wav` | `004A.00FF 0300.3FFF` | `E2B3R ⏎ RUN` (16 KB) |
| `APPLE30TH.wav` | `0280.0FFF` | `280R` |
| `ASMmchess.wav` | `0300.0BFF` | `300R` |
| `Microchess.wav` | `1000.18FF` | `1000R` |
| `LunarLander.wav` | `0300.09FF` | `300R` |
| `Mastermind.wav` | `0300.03FF` | `300R` |
| `Pasart.wav` | `0300.05FF` | `300R` |
| `Life.wav` | `2000.21FF` | `2000R` (12 KB) |

Your own settings are never overwritten: a program you have set up yourself keeps its settings, only the “recognised” mark appears. Programs that go beyond the basic 8 KB automatically get the RAM they need when started, as with an expansion card.

**Unknown tape.** The emulator guesses what it can: a 4096-byte record that starts with `JMP` is Integer BASIC; two records of which the first is 182 bytes long (`$4A-$FF`) are a BASIC program, placed according to its LOMEM and HIMEM pointers. The list then shows “(guessed)”; check with ⚙. A machine-code program without addresses is not guessed: give its range.

![Digitizing a real tape: from the cassette to a file, then to the library.](diagrams/en/workflow-capture.svg "diagram large")

## GitHub projects

**+ GitHub project** accepts a URL (`https://github.com/jefftranter/6502/tree/master/asm/jmon`) or simply `owner/repository`. The useful files are copied to `~/.cache/apple1-emu/github/<owner>/<repository>/<commit>/`, and the project becomes a node of the tree, with its commit, date and message. **⟳ Update** fetches the latest commit; 🌐 opens the repository page. All network access happens in the background.

{{Test the latest build}} tries, in order:

<ol class="steps">
<li>the files of the repository’s latest <strong>Release</strong> (<code>.wav</code>/<code>.aiff</code>, then <code>.bin</code>, then <code>.hex</code>/<code>.txt</code>);</li>
<li>the latest <strong>GitHub Actions artifact</strong> (a token is required);</li>
<li><strong>building from source</strong>: a BASIC listing is typed into Integer BASIC then started with <code>RUN</code>; a 6502 source is assembled with <strong>ca65/ld65</strong> (package <code>cc65</code>), with <code>-D APPLE1</code> by default; a Woz dump is loaded.</li>
</ol>

![A GitHub project, from the repository URL to the running program.](diagrams/en/workflow-github.svg "diagram large")

An `apple1.toml` (or `apple1.cfg`) file at the root of the repository says what to build:

```toml title="apple1.toml"
file    = "src/game.s"    # program to run
load    = $0300           # load address (otherwise the source's .org)
run     = $0300           # run address (otherwise the load address)
ld65    = "apple1.cfg"    # ld65 configuration (otherwise -t none)
defines = "APPLE1"        # -D symbols for ca65
```

!!! astuce "The GitHub token is optional"
    Public repositories do not need one. A token (`GITHUB_TOKEN` variable, or **⚙ Preferences**) raises the request limit and gives access to your private repositories and to artifacts. It is never displayed or written into messages, and `gui.json` then becomes readable by your account only.

## The “Art ASCII” folder

The library also contains an **Art ASCII** folder: drawings in the screen format (apple, Apple-1 board, 6502 chip, cassette, rocket…) and portraits of Steve Wozniak and Steve Jobs converted from freely licensed photos. An ASCII-art `.txt` shows a preview when selected, with **→ BASIC**, **→ 6502** and **🎨 Open in the tool**; **▶** runs its 6502 version right away. Chapter 9 describes the tool.
