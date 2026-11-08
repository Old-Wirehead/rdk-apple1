# Panel layout

<p class="lead">Each panel is a tab that you place wherever you like: side by side, in tabs, or in its own window on a second screen.</p>

## Moving a panel

The editor, the Apple-1 screen, the GEN2 color monitor, the debugger, the disassembly, the dump, the ASCII tool and the library are all **dockable tabs**. Drag a tab by its title: guides appear, and you drop it **left of**, **right of**, **above** or **below** another panel, or **into the same tab group**. Drag the dividers to resize. The layout is remembered in `gui.json`.

## The initial layout

The editor is on the left; in the middle, the Apple-1 screen, with the color monitor below it when the GEN2 card is plugged in; on the right, the debugger with the disassembly below it, then the dump, and the library at the far right. At the very first launch the choice is made from the width of the **monitor** (the window opens small and is maximized a moment later): below 1400 points (a 1366 × 768 laptop, a 1024 × 768 screen) it starts as **editor and screen**, which is the most useful pair; from 1400 points up, with every panel. Choose another layout from the menu at any time. It follows the width of the window: each panel keeps its share, from a laptop to a wide monitor. The **⊞ Layout ▾** menu of the toolbar restores it at any time (**Default**, every panel open) and offers four more ready-made layouts: **Game** (Apple-1 screen and library), **BASIC** (editor and screen), **Assembly** (editor, screen and debugger) and **Debugging** (editor, debugger, disassembly and dump). **Save the current layout…** asks for a name and keeps the current layout, with its detached windows and the size of the main window (loading it opens those windows again where they were); your layouts are listed under a separator, in alphabetical order, and a right-click lets you **Rename** or **Delete** them. They are kept in `gui.json`.

## The start-up guide

On first launch a three-screen guide replaces the old “Your workspace” window. It is drawn in amber on black, with the tube effect (scanlines, glow, vignette, curvature 0.07) on the frame; the small text and the thumbnails stay sharp. A small **×** at the top right closes it. **‹ Back** and **Next ›** move between screens; the last screen has **Done**.

1. **Welcome**: what the kit is for.
2. **ROM and BASIC**: the Woz Monitor, the cassette ROM and G2 Basic are built in. Nothing else is required. Purists install their own original Integer BASIC from an outside source; the button opens the guide (“Installing a ROM”), which has links only, never the files.
3. **Your workspace**: the five layouts (**Default**, **Game**, **BASIC**, **Assembly**, **Debugging**). Each thumbnail is clickable (there is no “Choose” button). The rectangles keep numbers 1 to 6, and the colour screen (GEN2 / P-LAB) is **7**, in its own colour, in the thumbnails that show it and in the legend.

**Show this guide at start-up** is on all three screens, ticked by default. The same box is in **⚙ Preferences → Interface**. **⊞ Layout ▾ → Start-up guide…** opens it again. It is an ordinary window: it does not stay above the others. It does not open when the command line sets a size (`--taille`), nor for captures.

## Text size of each window

The editor, the assembler listing, the debugger, the disassembly, the dump and the library each have a compact **A 13 ▲▼** control in their bar: the triangles change the size, which is remembered for that window. Ctrl+wheel and Ctrl+plus / Ctrl+minus act on the window under the mouse pointer (over the Apple-1 screen, Ctrl+plus / Ctrl+minus change the zoom). **Reset preferences** puts every one of these sizes back to 13.

## What is remembered between two launches

The layout is saved on its own when you quit and found again as it was: the position and size of every panel, the detached windows, the zoom and the layout you chose. The window is restored to its size and its place, then the panels are put back in the same proportions, whatever the screen: the layout was checked at 2880 × 1620, 2560 × 1440, 1920 × 1080 and 1366 × 768, with less than 1 % of the picture different after a relaunch. A layout saved under a name (**Save the current layout…**) keeps in addition the detached windows. The interface never zooms itself: only the setting “Icon and text size” changes the size of the buttons.

## When the screen is bigger than its pane

With a **fixed zoom** (2x, 3x…, or <kbd>Ctrl</kbd>+<kbd>+</kbd>), the Apple-1 screen keeps the size you asked for, even if the pane is smaller: it then **scrolls** (wheel or bars) instead of being shrunk, and the status bar says so, with a hint to enlarge the pane. Only **Fit** adapts the zoom to the pane. In a narrow window (1024 × 768, for instance) the toolbar scrolls sideways rather than being cut off: the **»** button brings the Cassette group, and a second click goes back to the start. The editor’s **Run** button always stays visible.

## Detaching a panel

Each panel has a **Detach** button at the top right (also from a right-click on the tab) that opens it in **its own window**, which you can move to a second screen; its size and place are remembered. **⊟ Dock** puts it back in its initial place.

- The Apple-1 screen and the color monitor detach separately. The keyboard goes to the Apple-1 from either of their windows, and from the main window. Closing a screen’s window docks it again.
- Closing a tool’s window (editor, debugger…) closes the tool; its toolbar button reopens it in the same place.
- Dialog boxes (Open, Save as, Export) always open in the main window. In the file dialogs, <kbd>Esc</kbd> closes the dialog (also from the file name field), <kbd>Ctrl</kbd>+<kbd>L</kbd> edits the path (its text is selected, so typing replaces it); Save and Export of a tab start in the folder of its file.
- Saved layouts keep the detached windows; the main window keeps the same size from one launch to the next whatever the zoom.
- The tape oscilloscope stays a small floating window inside the main window.

<div class="cards">
<div class="card"><h4>To play</h4>Close the editor and the debugger, keep the library. <strong>Fit</strong> enlarges the screen; <kbd>F11</kbd> goes full screen, with the vintage screen.</div>
<div class="card"><h4>To program</h4>Editor on the left, screen in the middle, debugger and dump on the right. Detach the editor to a second monitor to give it the full height.</div>
<div class="card"><h4>For the GEN2</h4>Plug in the card: the color monitor goes below the Apple-1 screen. Detach it to get it large, like a second monitor.</div>
<div class="card"><h4>To give a talk</h4><strong>Detach</strong> the Apple-1 screen onto the projector, full screen, and the main window on your laptop.</div>
</div>

# Preferences

<p class="lead">⚙ Preferences gathers all the persistent settings. Everything is kept in ~/.config/apple1-emu/gui.json (or in the kit folder, in portable mode).</p>

![The Interface tab of the preferences: language, theme, icon and text size, compact bar, tooltips, start-up window and interface colours.](img/en/preferences.png "large")

The settings are spread over six tabs:

| Tab | Settings |
|---|---|
| **Interface** | language: automatic (from the keyboard layout), or forced: French, English, German, Russian, Spanish, Italian, Portuguese; the detected language and its source are shown. Theme: dark, light or system. Icon and text size: small, normal (the default), large, extra large. Compact bar (icons only when the window is narrow). Tooltips on hover. **Show this window at start-up** (the “Your workspace” window) and the button that opens it at once. **Interface colours**: the colours of the interface lists, chosen **separately for the dark and the light theme** (active buttons, current line of the debugger, highlighted line, selected cell, breakpoint line, addresses and ASCII column of the dump, changed values, labels); each colour has a swatch with a live preview, the defaults are readable (contrast of at least 4.5:1 on the theme background), and **Reset** buttons restore one theme or both. |
| **Editor** | syntax colours, for BASIC and for the 6502 assembler separately (chapter 7) |
| **Library** | the main folder (**Browse…**, **⟲ Default**, **Open folder**), the other folders, the GitHub projects and the optional GitHub token (only for private repositories and Actions artifacts; never displayed, and the `GITHUB_TOKEN` variable takes precedence); library position, left or right (right by default) |
| **Tape** | tape sound (muted at fast speed) and its volume, oscilloscope during playback or recording |
| **GEN2** | Uncle Bernie’s color card, 60 or 50 Hz scan, LORES palette (chapter 6) |
| **Advanced** | display and speed: show the library, display limited to ~60 characters/s like the real terminal, fast speed; **Vintage screen: fine settings** (shared by both screens, chapter 3); the Integer BASIC file loaded at `$E000` (**📂 Choose…** / **Forget**) |

Each slider of the preferences (vintage screen intensity, afterglow and curvature, tape volume) shows its **numerical value** next to it. At the first launch the curvature of the vintage screen is set to **0.07** (a slight rounding of the tube); an existing installation keeps its own value. The preview of the **Editor** tab follows the interface language.

The zoom, the phosphor and the vintage screen checkbox are not in the preferences: they are in the display row of each screen (chapter 3).

![The Editor tab: a color scheme for BASIC and for the 6502 assembler, or your own colors.](img/en/preferences-editeur.png "large")

**Automatic language.** The emulator follows the active keyboard layout: `fr` gives French, `us` or `gb` English, `de`, `ch` or `at` German, `ru` Russian, and so on. It asks GNOME, `setxkbmap`, Cinnamon (the Linux Mint case), `localectl`, then `/etc/default/keyboard`, and finally the `LANG` variables. On the command line: `--langue fr|en|de|ru|es|it|pt|auto`.

!!! astuce "Starting from scratch"
    **Reset preferences**, at the bottom of the window, restores the original settings after a confirmation (language, theme, layout, vintage screen…). Your library files stay where they are; editor tabs that are not saved are closed. The first launch of a new kit version asks the same question, because `gui.json` survives when the program folder is erased to install another one.

    For a portable kit, put an empty file named `portable` at the root of the kit (next to `run.sh`), or start it with `APPLE1_PORTABLE=1`: the preferences (`config/gui.json`), the default library (`programmes`) and the cache then live in the kit folder.

    Closing the emulator and renaming `~/.config/apple1-emu/gui.json` does the same as the button, at the next launch. Program settings stay in each folder’s `programs.json`.
