#!/usr/bin/env bash
# Lance Retro Dev Kit Apple 1 (émulateur Apple-1) depuis le dossier où se trouve ce script.
#   ./run.sh               fenêtre graphique (par défaut, double-clic possible)
#   ./run.sh --terminal    version terminal (ouvre un terminal si besoin)
#   ./run.sh FICHIER...    options et fichiers passés au programme
DIR="$(dirname "$(readlink -f "$0")")"
cd "$DIR" || exit 1
# Binaires selon le processeur : bin/x86_64 (PC 64 bits) ou bin/i686 (PC 32 bits).
case "$(uname -m)" in
  x86_64|amd64) ARCH=x86_64 ;;
  i386|i486|i586|i686) ARCH=i686 ;;
  *) echo "Erreur : processeur « $(uname -m) » non pris en charge. Retro Dev Kit Apple 1 existe pour les PC 64 bits (x86_64) et 32 bits (i686)." >&2
     exit 1 ;;
esac
BINDIR="$DIR/bin/$ARCH"
# Ancienne disposition (binaires à côté de run.sh)
[ -x "$BINDIR/apple1-emu-gui" ] || [ ! -f "$DIR/apple1-emu-gui" ] || BINDIR="$DIR"
if [ ! -f "$BINDIR/apple1-emu-gui" ]; then
  echo "Erreur : programmes introuvables dans $BINDIR (archive incomplète ?)." >&2
  exit 1
fi
chmod +x "$BINDIR"/apple1-* ./install.sh 2>/dev/null

terminal() {
  # Si on n'est pas dans un terminal (double-clic), on en ouvre un.
  if [ ! -t 0 ]; then
    for T in x-terminal-emulator gnome-terminal mate-terminal xfce4-terminal konsole xterm; do
      if command -v "$T" >/dev/null; then
        case "$T" in
          gnome-terminal) exec gnome-terminal -- "$DIR/run.sh" --terminal "$@";;
          *) exec "$T" -e "$DIR/run.sh" --terminal "$@";;
        esac
      fi
    done
  fi
  "$BINDIR/apple1-emu" "$@"
  echo; read -rp "Appuyez sur Entrée pour fermer..." _
  exit 0
}

if [ "${1:-}" = "--terminal" ] || [ "${1:-}" = "-t" ]; then
  shift
  terminal "$@"
fi

# Pas d'écran graphique (connexion SSH, console) : version terminal.
if [ -z "${DISPLAY:-}${WAYLAND_DISPLAY:-}" ]; then
  echo "Pas d'affichage graphique : lancement de la version terminal."
  terminal "$@"
fi

LOGDIR="${XDG_CACHE_HOME:-$HOME/.cache}/apple1-emu"
mkdir -p "$LOGDIR"
LOG="$LOGDIR/gui.log"
debut=$(date +%s)
"$BINDIR/apple1-emu-gui" "$@" 2>"$LOG"
rc=$?
# Échec immédiat (contexte OpenGL impossible : pilote, machine virtuelle...) :
# on réessaie avec le rendu logiciel de Mesa.
if [ $rc -ne 0 ] && [ $(( $(date +%s) - debut )) -lt 5 ] && [ -z "${LIBGL_ALWAYS_SOFTWARE:-}" ]; then
  echo "--- nouvel essai avec LIBGL_ALWAYS_SOFTWARE=1 ---" >>"$LOG"
  debut=$(date +%s)
  LIBGL_ALWAYS_SOFTWARE=1 "$BINDIR/apple1-emu-gui" "$@" 2>>"$LOG"
  rc=$?
fi
if [ $rc -ne 0 ] && [ $(( $(date +%s) - debut )) -lt 5 ]; then
  msg="L'interface graphique n'a pas pu démarrer (journal : $LOG).
Essayez la version terminal : ./run.sh --terminal

$(tail -n 5 "$LOG")"
  if [ -t 1 ]; then echo "$msg" >&2
  elif command -v zenity >/dev/null; then zenity --error --no-markup --title="Retro Dev Kit Apple 1" --text="$msg"
  elif command -v notify-send >/dev/null; then notify-send "Retro Dev Kit Apple 1" "$msg"
  fi
fi
exit $rc
