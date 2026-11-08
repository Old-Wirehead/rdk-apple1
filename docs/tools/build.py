#!/usr/bin/env python3
"""Build the Retro Dev Kit Apple 1 manuals (Markdown -> HTML -> PDF with WeasyPrint).

    python3 docs/tools/build.py [guide|manuel|all] [--png DIR]

Sources : docs/src/<doc>/*.md (front matter in doc.toml-like header of 00-*.md),
style docs/style/manuel.css, images docs/img, diagrams docs/diagrams/*.dot (graphviz -> SVG).
Output : docs/pdf/<name>.pdf.  Two passes: the second one writes the real page
numbers of the command index (the table of contents uses target-counter()).
"""
import html, os, re, subprocess, sys, unicodedata
from pathlib import Path

import markdown
from pygments import highlight
from pygments.formatters import HtmlFormatter
from pygments.lexers import get_lexer_by_name
import weasyprint

sys.path.insert(0, str(Path(__file__).parent))
from lexers import IntegerBasicLexer, WozMonLexer  # noqa: E402

DOCS = Path(__file__).resolve().parent.parent
DOCSET = {
    "guide": dict(src="guide", out="Guide-demarrage-rapide.pdf",
                  title="Guide de démarrage rapide", kicker="Retro Dev Kit Apple 1",
                  subtitle="Installer l’émulateur, lancer une cassette, écrire un premier programme BASIC et l’envoyer sur un vrai Apple-1",
                  cover_img="img/couverture-guide.png", header="Guide de démarrage rapide"),
    "manuel": dict(src="manuel", out="Manuel-complet.pdf",
                   title="Manuel complet", kicker="Retro Dev Kit Apple 1",
                   subtitle="L’émulateur, la cassette ACI, la carte GEN2, le débogueur — avec les guides complets de l’Integer BASIC et de l’assembleur 6502",
                   cover_img="img/couverture-manuel.png", header="Manuel complet"),
    "guide-en": dict(src="guide-en", out="Quick-Start-Guide.pdf", lang="en",
                     title="Quick Start Guide", kicker="Retro Dev Kit Apple 1",
                     subtitle="Install the emulator, load a tape, write a first BASIC program and send it to a real Apple-1",
                     cover_img="img/couverture-guide.png", header="Quick Start Guide"),
    "manual-en": dict(src="manual-en", out="Complete-Manual.pdf", lang="en",
                      title="Complete Manual", kicker="Retro Dev Kit Apple 1",
                      subtitle="The emulator, the ACI cassette interface, the GEN2 color card, the debugger — with complete guides to Integer BASIC and the 6502 assembler",
                      cover_img="img/couverture-manuel.png", header="Complete Manual"),
    "guide-ru": dict(src="guide-ru", out="Kratkoe-rukovodstvo.pdf", lang="ru",
                     title="Краткое руководство", kicker="Retro Dev Kit Apple 1",
                     subtitle="Установить эмулятор, загрузить кассету, написать первую программу на BASIC и отправить её на настоящий Apple-1",
                     cover_img="img/couverture-guide.png", header="Краткое руководство"),
    "manual-ru": dict(src="manual-ru", out="Polnoe-rukovodstvo.pdf", lang="ru",
                      title="Полное руководство", kicker="Retro Dev Kit Apple 1",
                      subtitle="Эмулятор, кассетный интерфейс ACI, цветная карта GEN2, отладчик — с полными руководствами по Integer BASIC и ассемблеру 6502",
                      cover_img="img/couverture-manuel.png", header="Полное руководство"),
}

# words of the page furniture; French is the default
L10N = {
    "fr": dict(machine="Le péché originel", toc="Sommaire", index="Index des commandes", edition="Édition française",
               version="Version", quotes=("«&#8239;", "&#8239;»"), css=""),
    "en": dict(machine="The Original Sin", toc="Contents", index="Index", edition="English edition",
               version="Version", quotes=("&ldquo;", "&rdquo;"),
               css='@page toc { @top-right { content: "Contents" } } '
                   'h1::before { content: "Chapter " counter(chapter); } '
                   'h1.appendix::before { content: "Appendix " counter(appendix, upper-alpha); }'),
    "ru": dict(machine="Первородный грех", toc="Содержание", index="Указатель", edition="Русское издание",
               version="Версия", quotes=("«", "»"),
               css='@page toc { @top-right { content: "Содержание" } } '
                   'h1::before { content: "Глава " counter(chapter); } '
                   'h1.appendix::before { content: "Приложение " counter(appendix, upper-alpha); } '
                   'figcaption::before { content: "Рисунок " counter(figure) " — "; }'),
}
LANG = "fr"

LEXERS = {"basic": IntegerBasicLexer(), "intbasic": IntegerBasicLexer(),
          "mon": WozMonLexer(), "woz": WozMonLexer()}
FMT = HtmlFormatter(nowrap=True)


def lexer(lang):
    if lang in LEXERS:
        return LEXERS[lang]
    alias = {"asm": "ca65", "s": "ca65", "sh": "bash", "shell": "bash"}.get(lang, lang)
    try:
        return get_lexer_by_name(alias)
    except Exception:
        return get_lexer_by_name("text")


def fence_validator(language, inputs, options, attrs, md):
    options.update(inputs)          # accept title="..." and any other option
    return True


def mon_session(source):
    """Woz Monitor / BASIC session: lines starting with "» " are typed by you."""
    out = []
    for line in source.rstrip("\n").split("\n"):
        if line.startswith("» "):
            t = line[2:]
            pre = ">" if t.startswith(">") else ""   # Integer BASIC prompt, not typed
            out.append(f'{pre}<span class="typed">{html.escape(t[len(pre):])}</span>')
        elif "» " in line:                           # "PROMPT?» answer": output, then typed
            pre, t = line.split("» ", 1)
            out.append(f'{html.escape(pre)}<span class="typed">{html.escape(t)}</span>')
        else:
            out.append(html.escape(line))
    return "\n".join(out)


def fence(source, language, css_class, options, md, **kw):
    title = options.get("title", "") if options else ""
    code = mon_session(source) if language == "mon" else highlight(source, lexer(language), FMT)
    lang_label = {"basic": "Integer BASIC", "asm": "6502", "s": "6502", "mon": "Woz Monitor",
                  "sh": "terminal", "bash": "terminal", "text": "", "toml": "toml"}.get(language, language)
    cap = f'<div class="code-title">{html.escape(title)}</div>' if title else ""
    long = " long" if source.count("\n") > 34 else ""
    return (f'<div class="code lang-{language}{long}" data-lang="{lang_label}">{cap}'
            f'<pre><code>{code}</code></pre></div>')


def slug(s):
    s = unicodedata.normalize("NFKD", s).encode("ascii", "ignore").decode()
    return re.sub(r"[^a-z0-9]+", "-", s.lower()).strip("-") or "x"


IDX_RE = re.compile(r"\{\{([^{}|]+?)(?:\|([^{}]+?))?\}\}")


INC_RE = re.compile(r"^@@include[ \t]+(\S+)(?:[ \t]+(\S+))?[ \t]*$", re.M)  # [ \t], not \s: \s would span the newline and eat the next line as the "lang"


def includes(text):
    """@@include path [lang]: a tested example file, shown as a titled code block."""
    def inc(m):
        path, lang = m.group(1), m.group(2)
        src = (DOCS / path).read_text(encoding="utf-8").rstrip("\n")
        lang = lang or {".s": "asm", ".inc": "asm", ".bas": "basic"}.get(Path(path).suffix, "text")
        fence = "````" if "```" in src else "```"
        return f'{fence}{lang} title="{Path(path).name}"\n{src}\n{fence}'
    return INC_RE.sub(inc, text)


def md_to_html(text, idx_terms):
    text = includes(text)
    # index markers {{TERM}} (shown) or {{TERM|shown text}} ; {{!TERM}} invisible
    def idx(m):
        term, shown = m.group(1).strip(), m.group(2)
        hidden = term.startswith("!")
        term = term.lstrip("!")
        n = len(idx_terms)
        idx_terms.append((term, f"idx{n}"))
        anchor = f'<span class="idx-a" id="idx{n}"></span>'
        if hidden:
            return anchor
        return anchor + (shown if shown is not None else term)
    text = IDX_RE.sub(idx, text)
    md = markdown.Markdown(extensions=[
        "extra", "admonition", "sane_lists", "smarty",
        "pymdownx.superfences", "pymdownx.betterem", "pymdownx.tilde"],
        extension_configs={
            "smarty": {"substitutions": {"left-double-quote": L10N[LANG]["quotes"][0], "right-double-quote": L10N[LANG]["quotes"][1]}},
            "pymdownx.superfences": {"custom_fences": [
                {"name": n, "class": "code", "format": fence, "validator": fence_validator} for n in
                ["basic", "asm", "s", "mon", "sh", "bash", "text", "toml", "json", "ca65", "xa"]]}})
    out = md.convert(text)
    # lone image paragraph -> numbered figure; title="large|small" sets the width class
    def fig(m):
        attrs = m.group(1)
        alt = re.search(r'alt="([^"]*)"', attrs).group(1)
        src = re.search(r'src="([^"]*)"', attrs).group(1)
        t = re.search(r'title="([^"]*)"', attrs)
        cls = t.group(1) if t else ""
        return (f'<figure class="{cls}"><img src="{src}" alt=""/>'
                f'<figcaption>{alt}</figcaption></figure>')
    out = re.sub(r"<p>\s*<img ([^>]*)/?>\s*</p>", fig, out)
    return out


def number_headings(body):
    """Ids on h1/h2, collect TOC. Chapter numbers come from CSS counters, but we
    also need them in the TOC, so they are computed here."""
    toc, ch, sec, app = [], 0, 0, 0
    def h(m):
        nonlocal ch, sec, app
        lvl, attrs, inner = int(m.group(1)), m.group(2) or "", m.group(3)
        plain = re.sub(r"<[^>]+>", "", re.sub(r'<span class="(?:un)?verified">.*?</span>', "", inner)).strip()
        unnum = "unnumbered" in attrs or "appendix" in attrs or "part" in attrs
        if lvl == 1 and "part" in attrs:
            hid = re.search(r'id="([^"]+)"', attrs)
            hid = hid.group(1) if hid else f"part-{slug(plain)}"
            attrs = re.sub(r'\s*id="[^"]*"', "", attrs)
            plain = re.sub(r"<[^>]+>", "", re.sub(r'<span class="part-sub">.*?</span>', "", inner)).strip()
            toc.append((0, hid, plain, "", False))
            return f'<h1 id="{hid}"{attrs}>{inner}</h1>'
        if lvl == 1:
            if "appendix" in attrs:
                app += 1
            elif not unnum:
                ch += 1
            sec = 0
        elif lvl == 2 and "unnumbered" not in attrs:
            sec += 1
        hid = re.search(r'id="([^"]+)"', attrs)
        hid = hid.group(1) if hid else f"h{lvl}-{slug(plain)}-{len(toc)}"
        attrs = re.sub(r'\s*id="[^"]*"', "", attrs)
        if lvl <= 2:
            num = (str(ch) if lvl == 1 else f"{ch}.{sec}") if not unnum else ""
            if lvl == 1 and "appendix" in attrs:
                num = chr(64 + app)
            elif lvl == 2 and app:
                num = ""
            # TOC entry keeps <code> (file names in the code font), drops any other markup
            rich = re.sub(r"<(?!/?code>)[^>]+>", "", re.sub(r'<span class="(?:un)?verified">.*?</span>', "", inner)).strip()
            toc.append((lvl, hid, rich, num, "appendix" in attrs))
        return f'<h{lvl} id="{hid}"{attrs}>{inner}</h{lvl}>'
    body = re.sub(r"<h([12])((?:\s+[a-z-]+=\"[^\"]*\")*)>(.*?)</h\1>", h, body, flags=re.S)
    # a section title stays on the page of the short code block that follows it
    body = re.sub(r'(<h[23][^>]*>(?:(?!</h[23]>).)*</h[23]>\s*(?:<p>(?:(?!</p>|<div).){0,400}</p>)?)\s*(?:<p>)?\s*(<div class="code lang-[^"]*"[^>]*>.*?</pre></div>)(?:\s*</p>)?',
                  lambda m: m.group(0) if "long" in m.group(2)[:40] else f'<div class="keep-with">{m.group(1)}{m.group(2)}</div>',
                  body, flags=re.S)
    return body, toc


def toc_html(toc):
    rows = []
    for lvl, hid, text, num, app in toc:
        if lvl == 0:
            rows.append(f'<li class="toc0"><a href="#{hid}">{html.escape(text)}</a></li>')
        elif lvl == 1:
            label = f'<span class="toc-num">{num}</span>' if num else ('<span class="toc-num app">A</span>' if app else '<span class="toc-num"></span>')
            rows.append(f'<li class="toc1"><a href="#{hid}">{label}<span class="toc-text">{text}</span></a></li>')
        else:
            rows.append(f'<li class="toc2"><a href="#{hid}"><span class="toc-num">{num}</span><span class="toc-text">{text}</span></a></li>')
    return f'<nav class="toc"><h1 class="unnumbered toc-title" id="sommaire">{L10N[LANG]["toc"]}</h1><ul>' + "".join(rows) + "</ul></nav>"


def index_html(idx_terms, pages):
    if not idx_terms:
        return ""
    groups = {}
    for term, aid in idx_terms:
        groups.setdefault(term, []).append(aid)
    def key(t):
        return unicodedata.normalize("NFKD", t).encode("ascii", "ignore").decode().upper().lstrip("$.#")
    items, letter = [], None
    for term in sorted(groups, key=key):
        k = key(term)
        L = k[:1] if k[:1].isalpha() else "#"
        if L != letter:
            letter = L
            items.append(f'<li class="idx-letter">{L}</li>')
        ps = sorted({pages.get(a, 0) for a in groups[term]})
        refs = ", ".join(f'<a href="#{next(a for a in groups[term] if pages.get(a, 0) == p)}">{p}</a>' for p in ps if p)
        items.append(f'<li><span class="idx-term">{html.escape(term)}</span><span class="idx-pages">{refs}</span></li>')
    return ('<section class="index"><h1 class="unnumbered" id="index">' + L10N[LANG]["index"] + '</h1>'
            '<ul class="idx-list">' + "".join(items) + "</ul></section>")


def build_diagrams():
    for dot in sorted((DOCS / "diagrams").rglob("*.dot")):
        svg = dot.with_suffix(".svg")
        if not svg.exists() or svg.stat().st_mtime < dot.stat().st_mtime:
            subprocess.run(["dot", "-Tsvg", str(dot), "-o", str(svg)], check=True)


def front(text):
    meta = {}
    m = re.match(r"^---\n(.*?)\n---\n", text, re.S)
    if m:
        for line in m.group(1).splitlines():
            k, _, v = line.partition(":")
            meta[k.strip()] = v.strip()
        text = text[m.end():]
    return meta, text


def build(name, png_dir=None):
    global LANG
    cfg = DOCSET[name]
    LANG = cfg.get("lang", "fr")
    t = L10N[LANG]
    src = DOCS / "src" / cfg["src"]
    files = sorted(src.glob("*.md"))
    idx_terms, parts, meta = [], [], {}
    for f in files:
        m, text = front(f.read_text(encoding="utf-8"))
        meta.update(m)
        parts.append(md_to_html(text, idx_terms))
    body, toc = number_headings("\n".join(parts))
    version = meta.get("version", "")
    date = meta.get("date", "")
    cover = f'''<section class="cover">
  <div class="cover-band"></div>
  <div class="cover-kicker">{cfg["kicker"]} — {t["machine"]}</div>
  <h1 class="cover-title unnumbered" id="couverture">{cfg["title"]}</h1>
  <div class="cover-sub">{cfg["subtitle"]}</div>
  <div class="cover-screen"><img src="{cfg["cover_img"]}" alt=""/></div>
  <div class="cover-foot"><span>{t["version"]} {version}</span><span>{date}</span><span>{t["edition"]}</span></div>
</section>'''

    def page(index_part):
        return f'''<!DOCTYPE html><html lang="{LANG}"><head><meta charset="utf-8">
<title>{cfg["title"]} — Retro Dev Kit Apple 1</title>
<meta name="author" content="Retro Dev Kit"><meta name="description" content="{html.escape(cfg["subtitle"])}">
<link rel="stylesheet" href="style/manuel.css">{f"<style>{t['css']}</style>" if t["css"] else ""}</head>
<body class="doc-{name}" data-header="{cfg["header"]}">{cover}{toc_html(toc)}<main>{body}</main>{index_part}</body></html>'''

    build_diagrams()
    html1 = page(index_html(idx_terms, {}))
    doc = weasyprint.HTML(string=html1, base_url=str(DOCS)).render()
    if idx_terms:
        pages = {}
        for i, p in enumerate(doc.pages, 1):
            for a in p.anchors:
                if a.startswith("idx"):
                    pages.setdefault(a, i)
        html2 = page(index_html(idx_terms, pages))
        doc = weasyprint.HTML(string=html2, base_url=str(DOCS)).render()
        html1 = html2
    (DOCS / "build").mkdir(exist_ok=True)
    (DOCS / "build" / f"{name}.html").write_text(html1, encoding="utf-8")
    out = DOCS / "pdf" / cfg["out"]
    out.parent.mkdir(exist_ok=True)
    doc.write_pdf(str(out))
    print(f"{out.relative_to(DOCS.parent)} : {len(doc.pages)} pages")
    if png_dir:
        Path(png_dir).mkdir(parents=True, exist_ok=True)
        subprocess.run(["pdftoppm", "-r", "70", "-png", str(out), str(Path(png_dir) / name)], check=True)
    return out


if __name__ == "__main__":
    args = sys.argv[1:]
    png = None
    if "--png" in args:
        i = args.index("--png"); png = args[i + 1]; del args[i:i + 2]
    which = args[0] if args else "all"
    sets = {"all": list(DOCSET), "fr": ["guide", "manuel"], "en": ["guide-en", "manual-en"], "ru": ["guide-ru", "manual-ru"]}
    for n in sets.get(which, [which]):
        build(n, png)
