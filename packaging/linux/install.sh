#!/usr/bin/env bash
# Installe Retro Dev Kit Apple 1 (émulateur Apple-1) pour l'utilisateur courant (sans sudo) :
#   - programmes et exemples dans ~/.local/share/apple1-emu
#   - commandes apple1-emu et apple1-emu-gui dans ~/.local/bin
#   - entrée « Retro Dev Kit Apple 1 » dans le menu des applications (et sur le bureau avec --bureau)
# Désinstallation : ./install.sh --desinstaller
set -e
SRC="$(dirname "$(readlink -f "$0")")"
DEST="$HOME/.local/share/apple1-emu"
BIN="$HOME/.local/bin"
APPS="$HOME/.local/share/applications"
ICONS="$HOME/.local/share/icons/hicolor/256x256/apps"
# Binaires selon le processeur : bin/x86_64 (PC 64 bits) ou bin/i686 (PC 32 bits).
case "$(uname -m)" in
  x86_64|amd64) ARCH=x86_64 ;;
  i386|i486|i586|i686) ARCH=i686 ;;
  *) echo "Erreur : processeur « $(uname -m) » non pris en charge. Retro Dev Kit Apple 1 existe pour les PC 64 bits (x86_64) et 32 bits (i686)." >&2
     exit 1 ;;
esac

if [ "${1:-}" = "--desinstaller" ]; then
  rm -f "$APPS/apple1-emu.desktop" "$ICONS/apple1-emu.png" "$BIN/apple1-emu" "$BIN/apple1-emu-gui" "$BIN/apple1-mcp"
  rm -f "$(xdg-user-dir DESKTOP 2>/dev/null || echo "$HOME/Desktop")/apple1-emu.desktop"
  rm -rf "$DEST"
  update-desktop-database "$APPS" 2>/dev/null || true
  echo "Retro Dev Kit Apple 1 désinstallé. Vos programmes (~/apple1/programmes) et réglages (~/.config/apple1-emu) sont conservés."
  exit 0
fi

mkdir -p "$DEST" "$BIN" "$APPS" "$ICONS"
# Ancienne installation (lancer.sh / installer.sh) : remplacée par run.sh / install.sh
rm -f "$DEST/lancer.sh" "$DEST/installer.sh"
if [ ! -f "$SRC/bin/$ARCH/apple1-emu-gui" ]; then
  echo "Erreur : programmes $ARCH introuvables dans $SRC/bin/$ARCH (archive incomplète ?)." >&2
  exit 1
fi
if [ "$SRC" != "$DEST" ]; then
  # Seuls les programmes de ce processeur ($ARCH) sont installés.
  rm -f "$DEST/apple1-emu" "$DEST/apple1-emu-gui"   # ancienne disposition
  rm -rf "$DEST/bin"; mkdir -p "$DEST/bin"; cp -r "$SRC/bin/$ARCH" "$DEST/bin/$ARCH"
  cp -f "$SRC/run.sh" "$SRC/install.sh" "$SRC/apple1-emu.png" "$SRC/apple1-emu.desktop" "$DEST/"
  cp -f "$SRC"/LISEZMOI* "$DEST/" 2>/dev/null || true
  rm -rf "$DEST/exemples"; cp -r "$SRC/exemples" "$DEST/exemples"
  # les programmes des manuels (docs/exemples dans l'archive) et les PDF
  if [ -d "$SRC/docs" ]; then rm -rf "$DEST/docs"; cp -r "$SRC/docs" "$DEST/docs"; fi
fi
chmod +x "$DEST/bin/$ARCH"/apple1-* "$DEST/run.sh" "$DEST/install.sh"
ln -sf "$DEST/bin/$ARCH/apple1-emu" "$BIN/apple1-emu"
ln -sf "$DEST/bin/$ARCH/apple1-emu-gui" "$BIN/apple1-emu-gui"
if [ -f "$DEST/bin/$ARCH/apple1-mcp" ]; then ln -sf "$DEST/bin/$ARCH/apple1-mcp" "$BIN/apple1-mcp"; fi
cp -f "$DEST/apple1-emu.png" "$ICONS/apple1-emu.png"
sed -e "s#^Exec=\./run\.sh#Exec=\"$DEST/run.sh\"#" -e "s#^Icon=.*#Icon=$ICONS/apple1-emu.png#" \
    "$SRC/apple1-emu.desktop" > "$APPS/apple1-emu.desktop"
chmod +x "$APPS/apple1-emu.desktop"
update-desktop-database "$APPS" 2>/dev/null || true
gtk-update-icon-cache -q "$HOME/.local/share/icons/hicolor" 2>/dev/null || true
if [ "${1:-}" = "--bureau" ]; then
  BUREAU="$(xdg-user-dir DESKTOP 2>/dev/null || echo "$HOME/Desktop")"
  mkdir -p "$BUREAU"
  cp -f "$APPS/apple1-emu.desktop" "$BUREAU/apple1-emu.desktop"
  chmod +x "$BUREAU/apple1-emu.desktop"
  gio set "$BUREAU/apple1-emu.desktop" metadata::trusted true 2>/dev/null || true
fi
# Une ancienne icône du bureau qui pointe encore vers lancer.sh : mise à jour
BUREAU="$(xdg-user-dir DESKTOP 2>/dev/null || echo "$HOME/Desktop")"
if [ -f "$BUREAU/apple1-emu.desktop" ] && grep -q 'lancer\.sh' "$BUREAU/apple1-emu.desktop"; then
  cp -f "$APPS/apple1-emu.desktop" "$BUREAU/apple1-emu.desktop"
  chmod +x "$BUREAU/apple1-emu.desktop"
  gio set "$BUREAU/apple1-emu.desktop" metadata::trusted true 2>/dev/null || true
fi
echo "Retro Dev Kit Apple 1 ($ARCH) installé dans $DEST."
echo "Menu : « Retro Dev Kit Apple 1 » (rubrique Jeux). Commandes : apple1-emu-gui (fenêtre), apple1-emu (terminal)."
case ":$PATH:" in *":$BIN:"*) ;; *) echo "Note : ajoutez $BIN à votre PATH (ou reconnectez-vous) pour les commandes.";; esac
