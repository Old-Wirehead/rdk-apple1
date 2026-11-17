#!/bin/sh
# Lanceur du Retro Dev Kit Apple 1 2.0 (Linux).
#
# Le kit utilise le dossier courant comme « dossier du kit » : il y lit la
# bibliothèque (./Biblio), le BASIC de l'utilisateur (./integer_basic.bin,
# non fourni) et y écrit ses préférences (preferences.json…). Ce script se
# place donc d'abord dans son propre dossier.
set -e
DOSSIER="$(cd "$(dirname "$0")" && pwd)"
cd "$DOSSIER"
[ -x ./retro-dev-kit-apple1 ] || chmod +x ./retro-dev-kit-apple1 2>/dev/null || true
exec ./retro-dev-kit-apple1 "$@"
