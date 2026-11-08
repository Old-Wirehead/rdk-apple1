#!/bin/sh
# Régénère les captures d'écran de l'interface graphique.
# Usage : scripts/captures.sh DOSSIER_SORTIE [DISPLAY]
# Il faut un serveur X qui garde ses réglages entre deux clients, par ex. :
#   Xvfb :95 -noreset -screen 0 1600x1000x24 &
# (sans -noreset, Xvfb oublie la disposition du clavier dès que setxkbmap se termine).
set -e
OUT=${1:?dossier de sortie}
export DISPLAY=${2:-:95}
HERE=$(cd "$(dirname "$0")/.." && pwd)
G=${G:-$HERE/target/x86_64-unknown-linux-gnu/release/apple1-emu-gui}
T=$(mktemp -d)
mkdir -p "$OUT"
run() { # run NOM [options…] : bibliothèque et réglages neufs à chaque capture
    name=$1; shift
    rm -rf "$T/cfg" "$T/cache"
    XDG_CONFIG_HOME=$T/cfg XDG_CACHE_HOME=$T/cache "$G" --bibliotheque "$T/biblio" --taille 1500x800 --quitter --capture "$OUT/$name.png" "$@" 2>&1 | grep '^capture'
}
setxkbmap -layout us
rm -rf "$T/biblio"
# interface en français
run capture-zoom-3x --langue fr --zoom 3 --lancer-biblio 1:hello.txt --delai 3
run capture-zoom-1x --langue fr --zoom 1 --lancer-biblio 1:hello.txt --delai 3
run capture-ajuster-ambre --langue fr --phosphore ambre --balayage --lancer-biblio 1:hello.txt --delai 3
run capture-cassette --langue fr --cassette "$T/biblio/hello.wav" --taper-a 1:C100R\\n300.31ER\\n --delai 5
run capture-chargement-cassette --langue fr --mode-cassette --charger-biblio 1:hello.wav --delai 6
run capture-parametres --langue fr --parametres 1:multi.wav --delai 3
run capture-preferences --langue fr --preferences 1 --delai 3
run capture-gen2-preferences --langue fr --carte-couleur --preferences 1 --delai 3
run capture-gen2-barres --langue fr --carte-couleur --lancer-biblio 1:gen2-barres.txt --delai 4
# langue détectée d'après la disposition du clavier (us -> anglais, ru -> russe)
rm -rf "$T/biblio"
run capture-ui-anglais --lancer-biblio 1:hello.txt --delai 3
setxkbmap -layout ru
rm -rf "$T/biblio"
run capture-ui-russe --mode-cassette --lancer-biblio 1:hello.wav --delai 8
run capture-preferences-russe --preferences 1 --delai 3
setxkbmap -layout us
rm -rf "$T"
