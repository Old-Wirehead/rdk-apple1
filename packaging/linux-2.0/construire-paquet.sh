#!/bin/sh
# Construit le paquet Linux x86_64 « retro-dev-kit-apple1-<version>-linux »
# et son zip, dans dist/.
#
#   packaging/linux-2.0/construire-paquet.sh
#
# Le paquet contient le programme, run.sh, la licence et la bibliothèque
# d'exemples. Seules les ROM de la v1.0 (Woz Monitor, ACI, 2513) et RDK G2
# Basic (clone du kit, MIT) sont dans le programme ; le BASIC d'origine et
# les images CodeTank ne sont JAMAIS dans le paquet.
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
mkdir -p "$CIBLE/Biblio/BASIC" "$CIBLE/Biblio/Assembleur" "$CIBLE/Biblio/Cassettes" "$CIBLE/Biblio/PROM" "$CIBLE/Biblio/Jeux" \
    "$CIBLE/Biblio/GEN2 COLOR" "$CIBLE/Biblio/P-LAB" "$CIBLE/Biblio/Art ASCII"
cp crates/apple1-rdk/target/release/retro-dev-kit-apple1 "$CIBLE/"
strip "$CIBLE/retro-dev-kit-apple1" 2>/dev/null || true
cp "$ICI/run.sh" "$ICI/LISEZMOI-INSTALLATION.md" LICENSE NOTICE "$CIBLE/"
cp "$ICI"/Biblio/BASIC/*.bas "$CIBLE/Biblio/BASIC/"
cp "$ICI"/Biblio/Assembleur/*.asm "$CIBLE/Biblio/Assembleur/"
cp samples/hello.wav "$CIBLE/Biblio/Cassettes/"
# exemples de la v1.0 (programmes des manuels, copiés sans les modifier ;
# erreur.bas, l'exemple d'erreur du manuel, n'est pas repris)
EX=docs/exemples
for f in "$EX"/basic/*.bas; do
    case "$(basename "$f")" in erreur.bas|puissance4-couleur.bas) ;; *) cp -n "$f" "$CIBLE/Biblio/BASIC/" ;; esac
done
for f in hello hexa clavier decimal jeu macros morpion tunnel; do cp "$EX/asm/$f.s" "$CIBLE/Biblio/Assembleur/"; done
cp "$EX/asm/apple1.inc" "$CIBLE/Biblio/Assembleur/"
# cartes de la v1.0 : GEN2 (démos originales du kit) et P-LAB ; aucun
# fichier d'Uncle Bernie (GEN2_TESTS_HOWTO, gen2-test*.apl) n'est repris
cp "$EX/asm/gen2-lores.s" "$EX/asm/gen2-hires.s" "$EX/asm/gen2.inc" "$EX/asm/apple1.inc" \
    "$EX/basic/puissance4-couleur.bas" "$CIBLE/Biblio/GEN2 COLOR/"
# gen2-barres est un vidage du Woz Monitor : livré en .woz, le format de la
# 2.0 pour ces vidages (.txt n'est lu que pour les anciens fichiers ;
# rdk-apple1#17, choix de Wirehead du 18/11/2026). samples/ reste en .txt
# pour la v1.0.
cp samples/gen2-barres.txt "$CIBLE/Biblio/GEN2 COLOR/gen2-barres.woz"
cp "$EX/asm/plab-couleurs.s" "$EX/asm/apple1.inc" "$CIBLE/Biblio/P-LAB/"
# art ASCII de la v1.0, avec ses licences (dessins originaux, portraits CC BY-SA 3.0)
cp crates/apple1-gui/ascii/*.txt crates/apple1-gui/ascii/LICENCES.md "$CIBLE/Biblio/Art ASCII/"
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
