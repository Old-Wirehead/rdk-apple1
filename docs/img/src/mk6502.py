L=["VSS","RDY","PHI1","IRQ","NC","NMI","SYNC","VCC"]+[f"A{i}" for i in range(12)]
R=["RES","PHI2","SO","PHI0","NC","NC","R/W"]+[f"D{i}" for i in range(8)]+["A15","A14","A13","A12","VSS"]
F={'6':[" ## ","#   ","### ","#  #"," ## "],'5':["####","#   ","### ","   #","### "],
   '0':[" ## ","#  #","#  #","#  #"," ## "],'2':[" ## ","#  #","  # "," #  ","####"]}
W=26
inner=[" "*W for _ in range(20)]
def put(r,s): inner[r]=s.center(W)
inner[0]="1"+" "*(W-3)+"40"
put(2,"MOS TECHNOLOGY")
for k in range(5): put(5+k,"  ".join(F[c][k] for c in "6502"))
put(12,"MCS 6502")
put(14,"7645")
inner[19]="20"+" "*(W-4)+"21"
rows=["      +"+"-"*11+"\\  /"+"-"*11+"+      "]
for i in range(20):
    rows.append(f"{L[i]:>4} -!{inner[i]}!- {R[i]:<4}")
rows.append("      +"+"-"*26+"+      ")
rows.append("MOS 6502  -  1 MHZ".center(40))
for r in rows: assert len(r)==40,(len(r),r)
open(__import__("os").path.join(__import__("os").path.dirname(__file__),"couv-6502.txt"),"w").write("\n".join(r.rstrip() for r in rows)+"\n")
print("\n".join(r+"|" for r in rows))
