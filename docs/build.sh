#!/usr/bin/env bash
# Construit les manuels PDF : docs/pdf/Guide-demarrage-rapide.pdf, Manuel-complet.pdf, Quick-Start-Guide.pdf, Complete-Manual.pdf,
#   Kratkoe-rukovodstvo.pdf, Polnoe-rukovodstvo.pdf (russe)
# Prérequis (une fois) : sudo apt install graphviz poppler-utils fonts-... (voir docs/LISEZMOI.md)
#   python3 -m venv ~/.venv-docs && ~/.venv-docs/bin/pip install weasyprint markdown pygments pymdown-extensions
# Usage : docs/build.sh [guide|manuel|guide-en|manual-en|guide-ru|manual-ru|fr|en|ru|all] [--png DOSSIER]
set -e
cd "$(dirname "$(readlink -f "$0")")"
PY=${DOCS_PYTHON:-}
for c in "$PY" "$HOME/.venv-docs/bin/python" /opt/docvenv/bin/python python3; do
  [ -n "$c" ] && [ -x "$(command -v "$c")" ] && "$c" -c "import weasyprint, markdown, pygments, pymdownx" 2>/dev/null && { PY=$c; break; }
done
exec "$PY" tools/build.py "$@"
