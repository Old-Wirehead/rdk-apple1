#!/usr/bin/env python3
"""Generate samples/gen2-barres.txt: an original LORES + MIXED demo for the
GEN2 colour card (soft switches $C250-$C257, Apple II LORES layout)."""
ORG = 0xC00
msg_screen = "UNCLE BERNIE GEN2 : 16 COULEURS LORES"
msg_term = "\rGEN2: 16 BARRES LORES + TEXTE (MIXED)\r"
code = []
labels = {}
fix = []
def b(*xs): code.extend(xs)
def lab(n): labels[n] = ORG + len(code)
def abs_(op, n_or_addr):
    code.append(op)
    if isinstance(n_or_addr, str):
        fix.append((len(code), n_or_addr, 'abs')); code.extend([0, 0])
    else:
        code.extend([n_or_addr & 0xFF, n_or_addr >> 8])
def br(op, n):
    code.append(op); fix.append((len(code), n, 'rel')); code.append(0)

for ss in (0xC250, 0xC256, 0xC253, 0xC254):   # TEXT off, LORES, MIX on, page 1
    abs_(0xAD, ss)                            # LDA ss (a READ flips the switch)
b(0xA9, 0x00, 0x85, 0x00, 0xA9, 0x04, 0x85, 0x01, 0xA0, 0x00)  # ($00)=$0400, Y=0
lab('fill')
b(0x98, 0x29, 0x7F, 0xAA)                     # TYA AND #$7F TAX
abs_(0xBD, 'tab')                             # LDA tab,X
b(0x91, 0x00, 0xC8)                           # STA ($00),Y  INY
br(0xD0, 'fill')                              # BNE fill
b(0xE6, 0x01, 0xA5, 0x01, 0xC9, 0x08)         # INC $01 LDA $01 CMP #8
br(0xD0, 'fill')
b(0xA2, 39, 0xA9, 0xA0)                       # LDX #39  LDA #$A0 (space)
lab('clr')
for a in (0x650, 0x6D0, 0x750, 0x7D0):        # text rows 20-23 (MIXED window)
    abs_(0x9D, a)
b(0xCA); br(0x10, 'clr')                      # DEX BPL clr
b(0xA2, len(msg_screen) - 1)
lab('cpy')
abs_(0xBD, 'smsg'); abs_(0x9D, 0x750 + (40 - len(msg_screen)) // 2)
b(0xCA); br(0x10, 'cpy')
b(0xA2, 0x00)
lab('prt')
abs_(0xBD, 'tmsg'); br(0xF0, 'done'); abs_(0x20, 0xFFEF); b(0xE8); br(0xD0, 'prt')
lab('done')
abs_(0x4C, 0xFF1F)                            # back to the Woz monitor
lab('tab')
for o in range(128):
    if o >= 120:
        code.append(0)                        # screen holes
    else:
        x = o % 40
        c = x * 16 // 40
        code.append(c * 0x11)                 # same colour, top and bottom block
lab('smsg')
code.extend(ord(ch) | 0x80 for ch in msg_screen)   # $80-$FF: normal video
lab('tmsg')
code.extend((ord(ch) | 0x80) for ch in msg_term); code.append(0)
for pos, n, kind in fix:
    t = labels[n]
    if kind == 'abs':
        code[pos], code[pos + 1] = t & 0xFF, t >> 8
    else:
        d = t - (ORG + pos + 1)
        assert -128 <= d < 128, n
        code[pos] = d & 0xFF
out = ["; Démo originale pour la carte couleur Uncle Bernie GEN2 (Grok Bot, domaine public)",
       "; 16 barres verticales LORES (couleurs 0 à 15 de gauche à droite) et, en mode MIXED,",
       "; une ligne de texte en bas de l'écran couleur. Lancer avec --carte-couleur, puis C00R."]
for i in range(0, len(code), 16):
    out.append(f"{ORG + i:04X}: " + " ".join(f"{v:02X}" for v in code[i:i + 16]))
out.append(f"{ORG:X}R")
open("samples/gen2-barres.txt", "w").write("\n".join(out) + "\n")
print(len(code), "octets")
