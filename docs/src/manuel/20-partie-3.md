# L’assembleur 6502 <span class="part-sub">Le processeur, l’assembleur intégré et ses outils, les entrées-sorties et les routines du Woz Monitor, des exemples commentés et des astuces</span> {: .part data-part="Partie III" }

# Le 6502 en bref

<p class="lead">Le 6502 de l’Apple-1 est un processeur 8 bits à 1,023 MHz, avec trois registres, 56 instructions et 64 Ko d’adresses. Ce chapitre en donne l’essentiel pour lire et écrire les exemples de cette partie.</p>

## Les registres

| Registre | Taille | Rôle |
|---|---|---|
| **A** | 8 bits | l’accumulateur : calculs, comparaisons, lectures et écritures |
| **X**, **Y** | 8 bits | les index : compteurs de boucle, décalage dans un tableau |
| **S** | 8 bits | le pointeur de pile ; la pile occupe la page `$0100-$01FF` |
| **PC** | 16 bits | le compteur ordinal : l’adresse de l’instruction suivante |
| **P** | 8 bits | les drapeaux N V – B D I Z C |

{{!registres (6502)}}

Les drapeaux sont mis à jour par presque toutes les instructions :

<dl class="ref">
<dt>N (négatif)</dt><dd>copie du bit 7 du résultat ; testé par <code>BMI</code> / <code>BPL</code>.</dd>
<dt>V (débordement)</dt><dd>débordement signé d’une addition ou d’une soustraction ; <code>BVS</code> / <code>BVC</code>.</dd>
<dt>Z (zéro)</dt><dd>le résultat est nul ; <code>BEQ</code> / <code>BNE</code>.</dd>
<dt>C (retenue)</dt><dd>retenue d’une addition, absence d’emprunt d’une soustraction, bit sorti d’un décalage ; <code>BCS</code> / <code>BCC</code>.</dd>
<dt>D (décimal)</dt><dd><code>ADC</code> et <code>SBC</code> calculent en BCD. Le Woz Monitor fait <code>CLD</code> au démarrage.</dd>
<dt>I (interruptions)</dt><dd>masque l’IRQ. L’Apple-1 n’en a pas en standard.</dd>
</dl>

## Les modes d’adressage

| Mode | Écriture | Octets | Exemple | Effet |
|---|---|---|---|---|
| implicite | `INX` | 1 | `inx` | X ← X + 1 |
| accumulateur | `ASL A` | 1 | `asl a` | décale A |
| immédiat | `#val` | 2 | `lda #$8D` | A ← $8D |
| page zéro | `zp` | 2 | `lda $06` | A ← mémoire $0006 |
| page zéro, X | `zp,X` | 2 | `lda $06,x` | |
| absolu | `adr` | 3 | `lda $D010` | |
| absolu, X / Y | `adr,X` | 3 | `lda message,x` | tableau indexé |
| indirect indexé | `(zp),Y` | 2 | `lda ($06),y` | pointeur 16 bits en page zéro, plus Y |
| indexé indirect | `(zp,X)` | 2 | `lda ($06,x)` | table de pointeurs |
| relatif | `étiquette` | 2 | `bne boucle` | branchement de −128 à +127 octets |
| indirect | `(adr)` | 3 | `jmp ($FFFC)` | `JMP` seulement |

La **page zéro** (`$0000-$00FF`) est la plus précieuse : ses instructions sont plus courtes et plus rapides, et seul ce mode permet les pointeurs `(zp),Y`. Le Woz Monitor n’utilise que `$24-$2B` ; l’Integer BASIC en utilise beaucoup plus (chapitre 16).

## Les instructions par famille

| Famille | Instructions |
|---|---|
| chargement, rangement | `LDA LDX LDY STA STX STY` |
| transferts | `TAX TAY TXA TYA TSX TXS` |
| pile | `PHA PLA PHP PLP` |
| arithmétique | `ADC SBC` (avec la retenue : `CLC` avant `ADC`, `SEC` avant `SBC`) |
| incréments | `INC INX INY DEC DEX DEY` |
| logique | `AND ORA EOR BIT` |
| décalages | `ASL LSR ROL ROR` |
| comparaisons | `CMP CPX CPY` (C = 1 si registre ≥ valeur, Z = 1 si égal) |
| branchements | `BEQ BNE BCC BCS BMI BPL BVC BVS` |
| sauts | `JMP JSR RTS RTI BRK` |
| drapeaux | `CLC SEC CLD SED CLI SEI CLV` |
| rien | `NOP` |

{{!instructions (6502)}}

!!! astuce "Comparer sans signe"
    Après `CMP #n`, `BCC` saute si A < n et `BCS` si A ≥ n. Pour « A > n », comparez avec n + 1 : c’est ce que font les exemples avec `cmp #'9' | $80 + 1`.

## Un 6502 NMOS, pas un 65C02

L’Apple-1 a un 6502 d’origine (NMOS), et l’émulateur aussi. Les opcodes non documentés s’exécutent comme des `NOP` de la bonne longueur, et les opcodes `JAM` arrêtent le processeur. L’assembleur accepte les instructions du 65C02 après `.setcpu "65C02"` (`BRA`, `STZ`, `PHX`, `PLX`, `PHY`, `PLY`, `INC A`…), pour une carte équipée d’un 65C02, mais elles **ne fonctionnent pas** dans l’émulateur :

```asm title="c02.s (essai)"
        .setcpu "65C02"
        .org $0300
        lda #$C1        ; "A"
        bra suite       ; sur un 6502 NMOS : NOP de 2 octets
        lda #$D8        ; "X"
suite:  jsr $FFEF
        jmp $FF1F
```

Ce programme affiche `A` sur un 65C02, et `X` dans l’émulateur : `BRA` ($80) y est un `NOP` qui saute son opérande, et le `LDA #$D8` est exécuté.

## La carte mémoire en une figure

| Adresses | Contenu |
|---|---|
| `$0000-$00FF` | page zéro (Woz Monitor : `$24-$2B` ; BASIC : beaucoup plus) |
| `$0100-$01FF` | pile |
| `$0200-$027F` | tampon de ligne du Woz Monitor |
| `$0280-$1FFF` | libre : vos programmes, souvent en `$0280` ou `$0300` (RAM de 8 Ko par défaut dans l’émulateur ; 4 Ko, jusqu’à `$0FFF`, sur un Apple-1 de base) |
| `$C000-$C0FF` | carte ACI (cassette) ; `$C100-$C1FF` : son programme |
| `$C250-$C257` | commutateurs de la carte GEN2 |
| `$D010-$D013` | PIA : clavier et écran |
| `$E000-$EFFF` | Integer BASIC (si chargé) |
| `$FF00-$FFFF` | Woz Monitor |

L’annexe A donne la carte complète.
