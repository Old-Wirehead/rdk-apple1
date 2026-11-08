#!/usr/bin/env python3
"""Crop the Apple-1 screen (280x192 area, background ~#040C06) out of a capture.
   crop_screen.py IN.png OUT.png [margin_px]"""
import sys
from PIL import Image
import numpy as np

im = Image.open(sys.argv[1]).convert("RGB")
a = np.asarray(im).astype(int)
bg = (a[..., 1] >= 10) & (a[..., 1] > 1.8 * a[..., 0] + 1)
# keep the widest horizontal run of background columns (the main screen)
cols_all = np.where(bg.sum(0) > 200)[0]
runs, start = [], cols_all[0]
for p, c in zip(cols_all, cols_all[1:]):
    if c != p + 1:
        runs.append((start, p)); start = c
runs.append((start, cols_all[-1]))
c0, c1 = max(runs, key=lambda r: r[1] - r[0])
bg[:, :c0] = False; bg[:, c1 + 1:] = False
rows = np.where(bg.sum(1) > 200)[0]
cols = np.where(bg.sum(0) > 200)[0]
m = int(sys.argv[3]) if len(sys.argv) > 3 else 0
box = (cols.min() - m, rows.min() - m, cols.max() + 1 + m, rows.max() + 1 + m)
im.crop(box).save(sys.argv[2])
print(sys.argv[2], box)
