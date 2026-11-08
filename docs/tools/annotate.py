#!/usr/bin/env python3
"""Numbered call-outs on screenshots (docs/img/brut -> docs/img; --en / --ru: docs/img/<lang>).

Each call-out: (number, (x, y) badge centre, optional (x, y) target). With a
target, the badge sits in the top margin and a thin line goes to the target.
"""
import sys
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

DOCS = Path(__file__).resolve().parent.parent
ACCENT = (194, 83, 27)
FONT = "/usr/share/fonts/truetype/sand-box/google/Inter/Inter-VariableFont_opsz,wght.ttf"

SPECS = {
    "defaut": dict(top=150, callouts=[
        (1, (130, 26)), (2, (378, 26)), (3, (566, 26)), (4, (900, 26)), (5, (1605, 26)),
        (6, (3510, 26)), (7, (3735, 26)), (8, (1800, 262), None),
        (9, (375, 563), None), (10, (1575, 488), None), (11, (2325, 281), None), (12, (2625, 1163), None),
        (13, (3300, 1125), None), (14, (3675, 142), None), (15, (1575, 2029), None),
    ]),
}
# English UI: the toolbar labels have other widths
SPECS_EN = {
    "defaut": dict(top=150, callouts=[
        (1, (110, 26)), (2, (370, 26)), (3, (563, 26)), (4, (1260, 26)), (5, (2115, 26)),
        (6, (2740, 26)), (7, (3700, 26)), (8, (1540, 190), None),
        (9, (500, 800), None), (10, (1300, 1350), None), (11, (2450, 225), None), (12, (2440, 1076), None),
        (13, (3400, 90), None), (14, (3640, 155), None), (15, (3375, 2066), None),
    ]),
}
SPECS_RU = {
    "defaut": dict(top=150, callouts=[
        (1, (130, 26)), (2, (378, 26)), (3, (566, 26)), (4, (915, 26)), (5, (1620, 26)),
        (6, (3510, 26)), (7, (3735, 26)), (8, (1800, 262), None),
        (9, (375, 563), None), (10, (1575, 488), None), (11, (2325, 281), None), (12, (2625, 1163), None),
        (13, (3300, 1125), None), (14, (3675, 142), None), (15, (1575, 2029), None),
    ]),
}
SPECS_BY = {"fr": SPECS, "en": SPECS_EN, "ru": SPECS_RU}


def font(size, weight=700):
    f = ImageFont.truetype(FONT, size)
    try:
        f.set_variation_by_axes([14, weight])  # opsz, wght
    except Exception:
        pass
    return f


def badge(d, c, n, r=27):
    x, y = c
    d.ellipse((x - r - 3, y - r - 3, x + r + 3, y + r + 3), fill=(255, 255, 255))
    d.ellipse((x - r, y - r, x + r, y + r), fill=ACCENT)
    f = font(int(r * 1.15))
    d.text((x, y + 1), str(n), font=f, fill="white", anchor="mm")


def annotate(name, lang="fr"):
    spec = SPECS_BY[lang][name]
    base = DOCS / "img" / ("" if lang == "fr" else lang)
    src = Image.open(base / "brut" / f"{name}.png").convert("RGB")
    top = spec.get("top", 0)
    im = Image.new("RGB", (src.width, src.height + top), (255, 255, 255))
    im.paste(src, (0, top))
    d = ImageDraw.Draw(im)
    for c in spec["callouts"]:
        n, (x, y) = c[0], c[1]
        if len(c) == 2:          # badge in the top margin, line to the target
            bx, by = x, top // 2 - 8
            d.line((bx, by, x, y + top - 14), fill=ACCENT, width=5)
            d.ellipse((x - 9, y + top - 9 - 14, x + 9, y + top + 9 - 14), fill=ACCENT, outline="white", width=3)
            badge(d, (bx, by), n)
        else:
            badge(d, (x, y + top), n)
    out = base / f"{name}-annote.png"
    im.save(out, optimize=True)
    print(out.name, im.size)


if __name__ == "__main__":
    args = sys.argv[1:]
    lang = "fr"
    if args and args[0][2:] in SPECS_BY:
        lang = args.pop(0)[2:]
    for n in args or SPECS_BY[lang]:
        annotate(n, lang)
