#!/usr/bin/env python3
"""Fresh screenshots of apple1-emu-gui for the manuals (headless, 2x HiDPI).

    python3 docs/tools/captures.py [NAME…]        (default: all)
    CAP_LANG=en CAP_WORK=/tmp/doc-captures-en python3 docs/tools/captures.py   (English UI)

Needs Xvfb on DISPLAY (default :97, 3840x2160) and a release build
(target/release/apple1-emu-gui). The Integer BASIC and the archive.org game
tapes are read from test-fixtures/ (never copied into docs/: only the
screenshots are kept, as for the README captures).
"""
import json, os, shutil, subprocess, sys, tempfile, time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DOCS = ROOT / "docs"
LANG = os.environ.get("CAP_LANG", "fr")   # UI language; English shots go to docs/img/en/brut
IMG = DOCS / "img" / ("brut" if LANG == "fr" else f"{LANG}/brut")
GUI = Path(os.environ.get("CAP_GUI", ROOT / "target" / "release" / "apple1-emu-gui"))
# every screenshot: vintage screen on, amber phosphor, slight tube curvature (Wirehead, 2026-10-26)
LOOK = ["--phosphore", "ambre", "--screening", "--screening-options", "courbure=0.15", "--gen2-screening"]
FIX = ROOT / "test-fixtures"
DISPLAY = os.environ.get("CAP_DISPLAY", ":97")
WORK = Path(os.environ.get("CAP_WORK", "/tmp/doc-captures"))
LIB = WORK / "lib"
CACHE = WORK / "cache"


def leaf(*tabs, active=0):
    z = {"max": {"x": 0.0, "y": 0.0}, "min": {"x": 0.0, "y": 0.0}}
    return {"Leaf": {"active": active, "collapsed": False, "rect": z, "scroll": 0.0,
                     "tab_bar_hidden": False, "tabs": list(tabs), "viewport": z}}


def split(kind, frac):
    z = {"max": {"x": 0.0, "y": 0.0}, "min": {"x": 0.0, "y": 0.0}}
    return {kind: {"collapsed_leaf_count": 0, "fraction": frac, "fully_collapsed": False, "rect": z}}


def tree(spec):
    """spec: ('H'|'V', fraction, a, b) or a list of tab names."""
    nodes = {}
    def put(i, s):
        if isinstance(s, list):
            nodes[i] = leaf(*s)
        else:
            nodes[i] = split("Horizontal" if s[0] == "H" else "Vertical", s[1])
            put(2 * i + 1, s[2]); put(2 * i + 2, s[3])
    put(0, spec)
    n = max(nodes) + 1
    return {"focused_surface": 0, "surfaces": [{"Main": {"collapsed": False, "collapsed_leaf_count": 0,
            "focused_node": 0, "nodes": [nodes.get(i, "Empty") for i in range(n)]}}]}


def prepare():
    WORK.mkdir(parents=True, exist_ok=True)
    LIB.mkdir(exist_ok=True)
    for f in ["BASIC.wav", "Hamurabi.wav", "LunarLander.wav", "StarTrek.wav"]:
        if not (LIB / f).exists():
            shutil.copy(FIX / f, LIB / f)
    for f in (ROOT / "samples").iterdir():
        if f.suffix in (".txt", ".apl", ".wav") and not (LIB / f.name).exists():
            shutil.copy(f, LIB / f.name)


def template():
    """gui.json of a first launch (default layout, examples installed)."""
    t = WORK / "template" / "apple1-emu" / "gui.json"
    if not t.exists():
        run("_template", None, [], delay=4, cfgdir=WORK / "template", fresh=True)
    return json.loads(t.read_text())


def run(name, cfg, args, delay, size="1920x1050", cfgdir=None, fresh=False, out=None):
    cfgdir = Path(cfgdir or tempfile.mkdtemp(prefix="cap-", dir=WORK))
    if cfg is not None:
        (cfgdir / "apple1-emu").mkdir(parents=True, exist_ok=True)
        (cfgdir / "apple1-emu" / "gui.json").write_text(json.dumps(cfg))
    out = out or (IMG / f"{name}.png")
    out.parent.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ, DISPLAY=DISPLAY, WINIT_X11_SCALE_FACTOR="2",
               XDG_CONFIG_HOME=str(cfgdir), XDG_CACHE_HOME=str(CACHE))
    subprocess.run(["xdotool", "mousemove", "3800", "2150"], env=env)
    cmd = [str(GUI), "--bibliotheque", str(LIB), "--taille", size, "--langue", LANG,
           *LOOK, *args, "--quitter", "--capture", str(out), "--delai", str(delay)]
    subprocess.run(cmd, env=env, timeout=delay + 60, check=True,
                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    print("capture", out.name)
    return out


def cfg_with(layout=None, **over):
    c = template()
    c.update(over)
    if layout is not None:
        c["layout"] = tree(layout)
        tabs = set()
        def walk(sp):
            if isinstance(sp, list):
                tabs.update(sp)
            else:
                walk(sp[2]); walk(sp[3])
        walk(layout)
        c["editor"]["open"] = "Editor" in tabs
        c["debugger"]["open"] = "Debugger" in tabs
        c["debugger"]["disasm"]["open"] = "Disasm" in tabs
        c["debugger"]["dump_win"]["open"] = "Dump" in tabs
        c["ascii"]["open"] = "Ascii" in tabs
        c["show_library"] = "Library" in tabs
        c["carte_couleur"] = "Color" in tabs
    return c


EX = DOCS / "exemples" / ("" if LANG == "fr" else "en")   # examples with English comments for the other UIs
SHOTS = {}


def shot(f):
    SHOTS[f.__name__] = f
    return f


@shot
def defaut():
    run("defaut", None, ["--taper-a", "2:300: A9 C1 20 EF FF 4C 1F FF\\n300R\\n"], delay=6,
        cfgdir=tempfile.mkdtemp(prefix="cap-", dir=WORK), fresh=True)


SCREEN_LIB = ("H", 0.70, ["Screen"], ["Library"])


@shot
def cassette_basic():
    run("cassette-basic", cfg_with(SCREEN_LIB, cassette_mode=True, tape_scope=True),
        ["--mode-cassette", "--charger-biblio", "1:BASIC.wav"], delay=9, size="1600x900")


@shot
def basic_pret():
    run("basic-pret", cfg_with(SCREEN_LIB),
        ["--charger-biblio", "1:BASIC.wav", "--taper-a", "2:E000R\\n", "--taper-a", "4:PRINT 6*7\\n"],
        delay=7, size="1600x900")


@shot
def hamurabi():
    run("hamurabi", cfg_with(SCREEN_LIB), ["--lancer-biblio", "1:Hamurabi.wav"], delay=14, size="1600x900")


EDIT_SCREEN = ("H", 0.46, ["Editor"], ["Screen"])


@shot
def editeur_premier():
    run("editeur-premier", cfg_with(EDIT_SCREEN),
        ["--editeur", str(EX / "basic" / "premier.bas"), "--editeur-lancer", "2"], delay=16, size="1600x900")


@shot
def editeur_erreur():
    run("editeur-erreur", cfg_with(EDIT_SCREEN),
        ["--editeur", str(EX / "basic" / "erreur.bas")], delay=3, size="1600x900")


@shot
def editeur_export():
    run("editeur-export", cfg_with(EDIT_SCREEN),
        ["--editeur", str(EX / "basic" / "premier.bas"), "--editeur-exporter", f"2:{WORK}/premier.wav"],
        delay=5, size="1600x900")


@shot
def couverture_guide():
    run("couverture-guide", cfg_with(["Screen"], screening={"on": True, "intensity": 0.6, "gaps": True, "soften": True,
        "glow": True, "afterglow": 0.3, "curvature": False, "curve": 0.3, "flicker": False, "version": 2}, fast=True, throttle=False),
        ["--lancer-biblio", "1:pomme.txt"], delay=8, size="1200x900")


@shot
def couverture_manuel():
    """Raw capture; docs/img/couverture-manuel.png is its 1200x848 screen area
    (same framing as couverture-guide.png). Drawing: docs/img/src/mk6502.py."""
    cfg = cfg_with(["Screen"])        # first launch installs the "Art ASCII" folder
    shutil.copy(DOCS / "img" / "src" / "couv-6502.txt", LIB / "Art ASCII" / "couv-6502.txt")
    run("couverture-manuel", cfg_with(["Screen"], screening={"on": True, "intensity": 0.6, "gaps": True, "soften": True,
        "glow": True, "afterglow": 0.3, "curvature": False, "curve": 0.3, "flicker": False, "version": 2}, fast=True, throttle=False),
        ["--lancer-biblio", "1:couv-6502.txt"], delay=8, size="1200x900")


ASM = EX / "asm"


@shot
def editeur_asm():
    run("editeur-asm", cfg_with(EDIT_SCREEN),
        ["--editeur", str(ASM / "hello.s"), "--editeur-lancer", "2"], delay=6, size="1600x900")


@shot
def editeur_asm_erreur():
    bad = WORK / "erreur.s"
    bad.write_text((ASM / "hello.s").read_text().replace("jsr ECHO", "jsr ECO"))
    run("editeur-asm-erreur", cfg_with(EDIT_SCREEN), ["--editeur", str(bad)], delay=3, size="1600x900")


DBG = ("H", 0.42, ("V", 0.62, ["Editor"], ["Screen"]), ("V", 0.55, ["Debugger"], ("H", 0.5, ["Disasm"], ["Dump"])))


@shot
def debogueur_basic():
    run("debogueur-basic", cfg_with(("H", 0.42, ("V", 0.6, ["Editor"], ["Screen"]), ["Debugger"])),
        ["--editeur", str(EX / "basic" / "boucle.bas"), "--editeur-lancer", "2", "--debogueur",
         "--dbg-arret", "L30", "--dbg-surveiller", "I,S"], delay=14, size="1800x1000")


@shot
def debogueur_asm():
    run("debogueur-asm", cfg_with(DBG),
        ["--editeur", str(ASM / "decimal.s"), "--editeur-lancer", "2", "--debogueur",
         "--dbg-arret", "prdec", "--dbg-surveiller", "$06.$08"], delay=7, size="1900x1050")


@shot
def bibliotheque_params():
    run("bibliotheque-params", cfg_with(SCREEN_LIB), ["--parametres", "2:Hamurabi.wav"], delay=4, size="1600x900")


@shot
def preferences():
    run("preferences", cfg_with(SCREEN_LIB), ["--preferences", "2"], delay=4, size="1600x900")


COLOR = ("H", 0.40, ["Editor"], ("V", 0.5, ["Screen"], ["Color"]))


@shot
def gen2_lores():
    run("gen2-lores", cfg_with(COLOR, carte_couleur=True), ["--carte-couleur",
        "--editeur", str(ASM / "gen2-lores.s"), "--editeur-lancer", "2"], delay=6, size="1600x1000")


@shot
def gen2_hires():
    run("gen2-hires", cfg_with(COLOR, carte_couleur=True), ["--carte-couleur",
        "--editeur", str(ASM / "gen2-hires.s"), "--editeur-lancer", "2"], delay=6, size="1600x1000")


@shot
def gen2_test2():
    run("gen2-test2", cfg_with(("H", 0.5, ["Screen"], ["Color"]), carte_couleur=True), ["--carte-couleur",
        "--lancer-biblio", "1:gen2-test2.apl"], delay=14, size="1600x800")


@shot
def ascii_outil():
    run("ascii-outil", cfg_with(("H", 0.55, ["Ascii"], ["Screen"])),
        ["--ascii", str(DOCS / "exemples" / "images" / "pomme.png"), "--ascii-lancer", "3"], delay=12, size="1800x1000")


@shot
def vintage():
    run("vintage", cfg_with(["Screen"], screening={"on": True, "intensity": 0.7, "gaps": True, "soften": True,
        "glow": True, "afterglow": 0.4, "curvature": True, "curve": 0.4, "flicker": False, "version": 2}),
        ["--lancer-biblio", "1:Hamurabi.wav"], delay=14, size="1200x900")


@shot
def ambre():
    run("ambre", cfg_with(["Screen"]), ["--phosphore", "ambre", "--lancer-biblio", "1:Hamurabi.wav"], delay=14, size="1200x900")


@shot
def preferences_editeur():
    run("preferences-editeur", cfg_with(SCREEN_LIB), ["--preferences", "2", "--onglet-prefs", "editeur"], delay=4, size="1600x900")


@shot
def ecran_menu():
    """Right-click menu of the Apple-1 screen."""
    run("ecran-menu", cfg_with(SCREEN_LIB), ["--lancer-biblio", "1:Hamurabi.wav", "--ecran-menu"], delay=14, size="1600x900")


@shot
def editeur_menu():
    """Editor: find bar open and the right-click menu."""
    run("editeur-menu", cfg_with(EDIT_SCREEN),
        ["--editeur", str(ASM / "hello.s"), "--editeur-recherche", "ECHO", "--editeur-menu"], delay=4, size="1600x900")


PLAB_LAYOUT = ("H", 0.46, ["Editor"], ("V", 0.5, ["Screen"], ["Plab"]))


@shot
def plab():
    """P-LAB graphics card: the colour pattern example drawn on the card's own screen."""
    c = cfg_with(PLAB_LAYOUT)
    c["carte_plab"] = True
    c["plab_open"] = True
    run("plab", c, ["--plab", "--editeur", str(ASM / "plab-couleurs.s"), "--editeur-lancer", "2"], delay=7, size="1600x900")


@shot
def bienvenue():
    """First-launch workspace dialog."""
    run("bienvenue", None, ["--bienvenue"], delay=4, size="1600x900",
        cfgdir=tempfile.mkdtemp(prefix="cap-", dir=WORK), fresh=True)


if __name__ == "__main__":
    prepare()
    names = sys.argv[1:] or list(SHOTS)
    for n in names:
        SHOTS[n.replace("-", "_")]()
