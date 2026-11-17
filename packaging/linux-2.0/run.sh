#!/bin/sh
# Lanceur du Retro Dev Kit Apple 1 2.0 (Linux).
#
# Le kit utilise le dossier courant comme « dossier du kit » : il y lit les
# exemples du paquet (./Biblio) et y écrit ses préférences
# (preferences.json…). La bibliothèque est dans
# ~/Documents/RDK librairies/Apple 1 ; le BASIC d'origine de l'utilisateur
# (integer_basic.bin, non fourni) va dans son dossier ROM. Ce script se
# place donc d'abord dans son propre dossier.
set -e
DOSSIER="$(cd "$(dirname "$0")" && pwd)"
cd "$DOSSIER"
[ -x ./retro-dev-kit-apple1 ] || chmod +x ./retro-dev-kit-apple1 2>/dev/null || true
exec ./retro-dev-kit-apple1 "$@"
