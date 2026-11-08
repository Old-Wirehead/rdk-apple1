# Installing on Linux

## Unpack and run

<ol class="steps">
<li><strong>Unpack the archive.</strong> Right-click <code>apple1-emu.zip</code>, then “Extract Here”. An <code>apple1-emu</code> folder appears.</li>
<li><strong>Double-click <code>run.sh</code>.</strong> If the file manager asks what to do with the file (for example on Linux Mint), choose <strong>“Run”</strong> (not “Display”). The Apple-1 window opens, without a terminal.</li>
<li><strong>Optional: add the emulator to the menu.</strong> Open a terminal in the folder (right-click → “Open in Terminal”) and type the command below. A <strong>Retro Dev Kit Apple 1</strong> entry appears in the <em>Games</em> section of the menu, together with the commands <code>apple1-emu-gui</code> (window) and <code>apple1-emu</code> (terminal).</li>
</ol>

```sh
./install.sh             # applications menu + commands in ~/.local/bin
./install.sh --bureau    # the same, plus a desktop icon
./install.sh --desinstaller
```

Uninstalling (`--desinstaller`) removes the programs and the menu entry but keeps your programs (`~/apple1/programmes`) and your settings (`~/.config/apple1-emu`). No command needs `sudo`: everything is installed in your home folder. The first launch of a new version offers to start again from the original settings: the library is kept. The **Reset preferences** button does the same. An empty file named `portable` next to the program (or `APPLE1_PORTABLE=1`) keeps preferences, the default library and the cache inside the kit folder.

!!! astuce "Updating from an older version"
    The `lancer.sh` and `installer.sh` scripts of earlier versions are now called `run.sh` and `install.sh`. Run `./install.sh` again: it replaces the old installation, and the menu entry and desktop icon that pointed to `lancer.sh`.

!!! astuce "The terminal version"
    `./run.sh --terminal` opens the emulator in a terminal (at least 42 columns × 28 lines), without a graphical interface. Handy over an SSH connection. The Complete Manual describes its keys and commands.

## If the window does not open

`run.sh` detects an immediate failure (graphics driver, virtual machine without acceleration) and retries on its own with Mesa software rendering (`LIBGL_ALWAYS_SOFTWARE=1`). If that fails too, a message is shown and the log is in `~/.cache/apple1-emu/gui.log`.

| Symptom | What to do |
|---|---|
| The file manager opens `run.sh` in the text editor | Right-click → Properties → Permissions: tick “Allow executing file as program”, then double-click again and choose “Run”. |
| “Permission denied” in the terminal | `chmod +x run.sh install.sh bin/*/apple1-*` |
| Black or very slow window | Normal in a virtual machine: software rendering stays smooth for the Apple-1 screen. Turn off the “Vintage screen” if needed. |
| No tape sound | Sound goes through ALSA; without a sound card it is simply muted. Loading still works. |

## Where your files are kept

| Location | Contents |
|---|---|
| `~/apple1/programmes` | The **library**: your tapes, programs and examples (created on first launch). |
| `~/.config/apple1-emu/gui.json` | Settings: zoom, colors, panel layout, editor tabs. |
| `~/.cache/apple1-emu/` | Converted tapes, downloaded GitHub projects, the `gui.log` log. |
