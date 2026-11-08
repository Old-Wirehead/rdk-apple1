#!/usr/bin/env python3
"""Crops of the raw captures used by the manuals (docs/img/<lang>/brut -> docs/img/<lang>).

    python3 docs/tools/crops.py [fr|en|ru]

The boxes were measured on the captures of the one-row toolbar (same window
sizes in every language; a box may need a few pixels more in a language with
longer labels).
"""
import sys
from pathlib import Path
from PIL import Image

DOCS = Path(__file__).resolve().parent.parent
# name: (raw capture, left, top, width, height)
BOXES = {
    "barre-1": ("defaut", 0, 0, 2320, 66),
    "barre-2": ("hamurabi", 0, 122, 1230, 60),
    "barre-cassette": ("defaut", 2345, 0, 790, 66),
    "barre-etat": ("cassette-basic", 0, 1700, 1600, 100),
    "basic-pret-ecran": ("basic-pret", 240, 312, 1760, 1232),
    "bibliotheque": ("cassette-basic", 2244, 109, 952, 1096),
    "editeur-export-bas": ("editeur-premier", 0, 1610, 1474, 100),
    "gen2-moniteur": ("gen2-lores", 1290, 1090, 1910, 500),
    "hamurabi-ecran": ("hamurabi", 240, 312, 1760, 1232),
    "oscilloscope": ("cassette-basic", 40, 1312, 775, 348),
    "preferences": ("preferences", 940, 620, 1316, 560),
    "ecran-menu": ("ecran-menu", 240, 312, 1760, 1232),
    "editeur-menu": ("editeur-menu", 0, 100, 1470, 1000),
    "preferences-editeur": ("preferences-editeur", 940, 430, 1316, 940),
    "plab": ("plab", 0, 0, 3200, 1800),
    "bienvenue": ("bienvenue", 740, 610, 1640, 590),
}

# per language: the longer French labels compact the 1600-wide toolbar to its
# icons, so the toolbar strips come from the wider default capture
LANG_BOXES = {
    "fr": {
        "barre-1": ("defaut", 0, 0, 1215, 66),
        "barre-2": ("hamurabi", 0, 122, 1320, 60),
        "barre-cassette": ("cassette-basic", 778, 0, 810, 66),
    },
    "ru": {
        "barre-1": ("defaut", 0, 0, 1225, 66),
        "barre-2": ("hamurabi", 0, 122, 1440, 60),
        "barre-cassette": ("cassette-basic", 760, 0, 815, 66),
        "editeur-menu": ("editeur-menu", 0, 122, 1470, 980),
    },
}

# stacked bands of one capture: the top of the editor and its error messages
STACKS = {
    "editeur-erreur": ("editeur-erreur", 1474, [(112, 580), (1562, 1690)]),
}

LANG_STACKS = {
    "fr": {"editeur-erreur": ("editeur-erreur", 1474, [(112, 580), (1556, 1702)])},
    "ru": {"editeur-erreur": ("editeur-erreur", 1474, [(112, 580), (1560, 1702)])},
}

if __name__ == "__main__":
    lang = sys.argv[1] if len(sys.argv) > 1 else "en"
    base = DOCS / "img" / ("" if lang == "fr" else lang)
    for name, (raw, x, y, w, h) in {**BOXES, **LANG_BOXES.get(lang, {})}.items():
        im = Image.open(base / "brut" / f"{raw}.png").crop((x, y, x + w, y + h))
        im.save(base / f"{name}.png", optimize=True)
        print(name, im.size)
    for name, (raw, w, bands) in {**STACKS, **LANG_STACKS.get(lang, {})}.items():
        src = Image.open(base / "brut" / f"{raw}.png")
        im = Image.new("RGB", (w, sum(b - a for a, b in bands)))
        y = 0
        for a, b in bands:
            im.paste(src.crop((0, a, w, b)), (0, y)); y += b - a
        im.save(base / f"{name}.png", optimize=True)
        print(name, im.size)
