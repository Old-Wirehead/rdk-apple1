#!/usr/bin/env python3
"""Crops of the raw captures used by the manuals (docs/img/<lang>/brut -> docs/img/<lang>).

    python3 docs/tools/crops.py [fr|en|ru|de|es|it|pt]

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

# German, Spanish, Italian, Portuguese: the « State » menu with the toolbar around it
for _l in ("de", "es", "it", "pt"):
    LANG_BOXES.setdefault(_l, {})["menu-etat"] = ("menu-etat", 0, 0, 1280, 580)

# German, Spanish, Italian, Portuguese: the colour monitor pane only, below its display row
for _l in ("de", "es", "it", "pt"):
    LANG_BOXES.setdefault(_l, {})["gen2-moniteur"] = ("gen2-lores", 1295, 1270, 1900, 480)

LANG_STACKS = {
    "fr": {"editeur-erreur": ("editeur-erreur", 1474, [(112, 580), (1556, 1702)])},
    "ru": {"editeur-erreur": ("editeur-erreur", 1474, [(112, 580), (1560, 1702)])},
    # German, Spanish, Italian, Portuguese: wider editor in the capture (captures.py, EDIT_SCREEN_ERR)
    "de": {"editeur-erreur": ("editeur-erreur", 1974, [(112, 580), (1595, 1722)])},
    "es": {"editeur-erreur": ("editeur-erreur", 1974, [(112, 580), (1595, 1722)])},
    "it": {"editeur-erreur": ("editeur-erreur", 1974, [(112, 580), (1595, 1722)])},
    "pt": {"editeur-erreur": ("editeur-erreur", 1974, [(112, 580), (1595, 1722)])},
}

# dialogs whose place and size change with the language and the version: the
# crop follows their 1-point frame (grey 60,60,60), found on the raw capture
DIALOGS = {"preferences": ("preferences", None), "preferences-editeur": ("preferences-editeur", None),
           "programmateur-eprom": ("programmateur-eprom", (700, 250, 2500, 1500)), "fenetre-fichiers": ("fenetre-fichiers", None),
           "oscilloscope": ("cassette-basic", (20, 1250, 1600, 1750))}


def toolbar_groups(path):
    """x of the end of the graphics-card group, and of the start and end of the
    Cassette group, from their frames (grey 60) on row 33 of a raw capture."""
    im = Image.open(path).convert("RGB")
    px = im.load()
    xs = []
    for x in range(im.size[0]):
        p = px[x, 33]
        if all(abs(c - 60) <= 1 for c in p) and px[x, 8][0] == 27 and (not xs or x > xs[-1] + 2):
            xs.append(x)
    triples = [a for a in xs if 1500 < a < 2800 and [x for x in xs if a <= x <= a + 46] == [a, a + 23, a + 46]]
    if not triples:
        return None
    a = triples[-1]          # the separator just before the Cassette group
    rest = [x for x in xs if x > a + 46]
    end = next(x for p, x in zip([a + 46] + rest, rest) if x - p > 200)
    return a, a + 46, end


def dialog_box(path, region=None, minh=250):
    """(left, top, width, height) of the window frame on a capture (or in a region of it), or None."""
    im = Image.open(path).convert("RGB")
    ox, oy = 0, 0
    if region:
        im = im.crop(region)
        ox, oy = region[0], region[1]
    W, H = im.size
    px = im.load()
    grey = lambda p: all(abs(c - 60) <= 3 for c in p)
    cols = []
    for x in range(W):
        best = run = 0
        for y in range(H):
            run = run + 1 if grey(px[x, y]) else 0
            if run > best:
                best, end = run, y + 1
        if best >= minh:
            cols.append((x, end - best, end))
    if len(cols) < 2:
        return None
    left, right = cols[0][0], cols[-1][0]
    top = min(c[1] for c in cols)
    bot = max(c[2] for c in cols)
    # top and bottom: the rows where the frame runs across the whole width
    full = [y for y in range(max(0, top - 20), min(H, bot + 20))
            if sum(grey(px[x, y]) for x in range(left, right + 1, 4)) > 0.8 * (right - left) / 4]
    if full:
        top, bot = full[0], full[-1]
    return left + ox, top + oy, right - left + 1, bot - top + 1


if __name__ == "__main__":
    lang = sys.argv[1] if len(sys.argv) > 1 else "en"
    base = DOCS / "img" / ("" if lang == "fr" else lang)
    for name, (raw, x, y, w, h) in {**BOXES, **LANG_BOXES.get(lang, {})}.items():
        if not (base / "brut" / f"{raw}.png").exists():
            print(name, "skipped: no raw capture", raw)   # e.g. a language with the quick-start guide only
            continue
        im = Image.open(base / "brut" / f"{raw}.png").crop((x, y, x + w, y + h))
        im.save(base / f"{name}.png", optimize=True)
        print(name, im.size)
    if lang in ("de", "es", "it", "pt"):
        f = base / "brut" / "defaut.png"
        g = f.exists() and toolbar_groups(f)
        if g:   # the toolbar strips follow the width of the labels
            gfx_end, cas_start, cas_end = g
            for name, box in (("barre-1", (0, 0, gfx_end + 12, 66)),
                              ("barre-cassette", (cas_start - 9, 0, cas_end - cas_start + 13, 66))):
                x, y, w, h = box
                im = Image.open(f).crop((x, y, x + w, y + h))
                im.save(base / f"{name}.png", optimize=True)
                print(name, im.size, "(groups found)")
        for name, (raw, region) in DIALOGS.items():
            f = base / "brut" / f"{raw}.png"
            box = f.exists() and dialog_box(f, region)
            if not box:
                print(name, "skipped: no raw capture or no frame found")
                continue
            x, y, w, h = box
            im = Image.open(f).crop((x, y, x + w, y + h))
            im.save(base / f"{name}.png", optimize=True)
            print(name, im.size, "(frame found)")
    for name, (raw, w, bands) in {**STACKS, **LANG_STACKS.get(lang, {})}.items():
        src = Image.open(base / "brut" / f"{raw}.png")
        im = Image.new("RGB", (w, sum(b - a for a, b in bands)))
        y = 0
        for a, b in bands:
            im.paste(src.crop((0, a, w, b)), (0, y)); y += b - a
        im.save(base / f"{name}.png", optimize=True)
        print(name, im.size)
