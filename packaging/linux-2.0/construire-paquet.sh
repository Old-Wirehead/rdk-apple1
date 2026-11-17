#!/bin/sh
# Construit le paquet Linux x86_64 « retro-dev-kit-apple1-<version>-linux »
# et son zip, dans dist/.
#
#   packaging/linux-2.0/construire-paquet.sh
#
# Le paquet contient le programme, run.sh, la licence et la bibliothèque
# d'exemples. Seules les ROM de la v1.0 (Woz Monitor, ACI, 2513) sont dans
# le programme ; le BASIC d'origine n'est JAMAIS dans le paquet.
#
# Nécessite Rust 1.95+, l'accès au dépôt RDK-Interface-Commune et `zip` ou,
# à défaut, `python3`.
set -e
RACINE="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$RACINE"
ICI=packaging/linux-2.0

VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' crates/apple1-rdk/Cargo.toml | head -n 1)
VERSION_COURTE=$(echo "$VERSION" | sed 's/\.0$//')
NOM="retro-dev-kit-apple1-$VERSION_COURTE-linux"
CIBLE="dist/$NOM"

(cd crates/apple1-rdk && cargo build --release --locked)

rm -rf "$CIBLE" "dist/$NOM.zip"
mkdir -p "$CIBLE/Biblio/BASIC" "$CIBLE/Biblio/Assembleur" "$CIBLE/Biblio/Cassettes" "$CIBLE/Biblio/PROM" "$CIBLE/Biblio/Jeux"
cp crates/apple1-rdk/target/release/retro-dev-kit-apple1 "$CIBLE/"
strip "$CIBLE/retro-dev-kit-apple1" 2>/dev/null || true
cp "$ICI/run.sh" "$ICI/LISEZMOI-INSTALLATION.md" LICENSE NOTICE "$CIBLE/"
cp "$ICI"/Biblio/BASIC/*.bas "$CIBLE/Biblio/BASIC/"
cp "$ICI"/Biblio/Assembleur/*.asm "$CIBLE/Biblio/Assembleur/"
cp samples/hello.wav "$CIBLE/Biblio/Cassettes/"
chmod +x "$CIBLE/run.sh" "$CIBLE/retro-dev-kit-apple1"

if command -v zip >/dev/null 2>&1; then
    (cd dist && zip -r -X "$NOM.zip" "$NOM" >/dev/null)
else
    (cd dist && python3 - "$NOM" <<'PY'
import os, sys, zipfile
nom = sys.argv[1]
with zipfile.ZipFile(nom + ".zip", "w", zipfile.ZIP_DEFLATED) as z:
    for dossier, sous, fichiers in os.walk(nom):
        sous.sort()
        if not fichiers and not sous:
            info = zipfile.ZipInfo(dossier + "/")
            info.external_attr = (0o40755 << 16) | 0x10
            z.writestr(info, "")
        for f in sorted(fichiers):
            chemin = os.path.join(dossier, f)
            info = zipfile.ZipInfo.from_file(chemin)
            info.compress_type = zipfile.ZIP_DEFLATED
            with open(chemin, "rb") as e:
                z.writestr(info, e.read())
PY
    )
fi
(cd dist && sha256sum "$NOM.zip" > "$NOM.zip.sha256")
echo "Paquet : $RACINE/dist/$NOM.zip"
