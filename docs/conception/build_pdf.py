#!/usr/bin/env python3
"""Rend docs/conception/*.md en docs/pdf/Conception-BASIC.pdf avec le style des manuels
(réutilise docs/tools/build.py sans le modifier ni reconstruire les autres PDF).
Usage : ~/.venv-docs/bin/python docs/conception/build_pdf.py"""
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "tools"))
import build  # noqa: E402

build.DOCSET["conception"] = dict(
    src="../conception", out="Conception-BASIC.pdf",
    title="RDK BASIC — note de conception", kicker="Retro Dev Kit Apple 1",
    subtitle="Un clone en salle blanche de l’Integer BASIC de l’Apple-1, sans ses bugs, avec les extensions GEN2 — dédié à Uncle Bernie",
    cover_img="img/couverture-manuel.png", header="RDK BASIC — conception")
build.build("conception")
