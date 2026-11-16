# Der Assembler des Devkits

<p class="lead">Der Emulator enthält einen vollständigen Zwei-Pass-Assembler für den 6502, kompatibel mit ca65. Er dient im Editor, der bei jedem Tastendruck assembliert, auf der Befehlszeile und für GitHub-Projekte. Es ist nichts zu installieren.</p>

## Der Arbeitszyklus

![Der Arbeitszyklus in Assembler: schreiben, assemblieren, ausführen, debuggen, dann zum echten Apple-1 exportieren.](diagrams/de/workflow-asm.svg "diagram large")

<ol class="steps">
<li><strong>Schreiben</strong>: <strong>📝 Editor</strong>, dann <strong>🗋 Neu</strong> oder <strong>📂 Öffnen</strong> einer Datei <code>.s</code>, <code>.asm</code> oder <code>.a65</code>. Die Syntaxfärbung unterscheidet Befehle, Direktiven, Marken, Zahlen und Kommentare.</li>
<li><strong>Assemblieren</strong>: nichts zu tun, die Assemblierung folgt jeder Änderung. Unten im Bereich stehen die Größe, die Adressen und die Zeile, die an der echten Maschine zu tippen ist; Fehler werden mit Zeile und Spalte angezeigt.</li>
<li><strong>Ausführen</strong>: <strong>▶ Starten</strong> legt die Bytes an der Adresse von <code>.org</code> in den Speicher und tippt den Befehl des Woz Monitors (<code>300R</code>).</li>
<li><strong>Debuggen</strong>: <strong>⏺ Haltepunkt</strong> auf der Zeile des Cursors, dann im Debugger schrittweise ausführen (Kapitel 8), mit Ihren Marken in der Disassemblierung.</li>
<li><strong>Exportieren</strong>: <strong>📼 WAV exportieren</strong> schreibt die Kassette; am echten Apple-1 tippen Sie <code>C100R</code>, den angezeigten Bereich (<code>0300.0320R</code>), dann <code>300R</code>.</li>
</ol>

![Ein Quelltext im Editor: links der gefärbte Text, unten das Ergebnis der Assemblierung; rechts hat der Apple-1 das Programm ausgeführt.](img/de/brut/editeur-asm.png "large")

## Das Listing und die Symbole

**📃 Listing** zeigt für jede Zeile die Adresse, die erzeugten Bytes und den Quelltext; von einem Makro erzeugte Zeilen sind mit `+` markiert, eingebundene Dateien werden aufgeklappt, und die Symboltabelle folgt. Ein Klick auf eine Zeile führt zum Quelltext zurück; ein Klick in den Rand setzt einen Haltepunkt. Hier das Listing von `hello.s` (Kapitel 23), von der Befehlszeile erzeugt:

```text title="hello.lst"
0300  A2 00       start:  ldx #0
0302  BD 10 03    @next:  lda message,x   ; nächstes Zeichen
0305  F0 06               beq @done       ; 0 = Ende der Zeichenkette
0307  20 EF FF            jsr ECHO
030A  E8                  inx
030B  D0 F5               bne @next
030D  4C 1F FF    @done:  jmp GETLINE

                  message:
0310  8D                  .byte CR
0311  48 41 4C 4C         .asciiz "HALLO, APPLE-1!"
0315  4F 2C 20 41
…
; ---- Symboles (13)
; ECHO        $FFEF
; GETLINE     $FF1F
; message     $0310
; start       $0300
; start@done  $030D
; start@next  $0302
```

Die lokalen Marken `@next` und `@done` gehören zu `start`: Die Tabelle nennt sie `start@next` und `start@done`, und der Debugger nimmt diese Namen als Haltepunktadressen an. (Das von der Befehlszeile geschriebene Listing behält seine französische Überschrift *Symboles*.)

## Die Befehlszeile

{{asm (Befehlszeile)|`apple1-emu asm`}} tut dasselbe ohne Fenster:

```sh
apple1-emu asm hello.s                    # nur prüfen
apple1-emu asm hello.s -l hello.lst       # mit dem Listing
apple1-emu asm hello.s -o hello.wav       # Kassette (.wav oder .aif)
apple1-emu asm hello.s -o hello.bin       # rohe Binärdatei
apple1-emu asm hello.s -o hello.hex       # Woz-Dump, zum Einfügen in den Monitor
apple1-emu asm jeu.s -I ../common         # ein weiterer Ordner für .include
```

Jeder Befehl fasst die Assemblierung zusammen und gibt die Zeile für den echten Apple-1. Die Zusammenfassung ist französisch („33 Bytes … 2 Datei(en), 13 Symbole / Kassette (nach C100R): 0300.0320R, dann 300R / geschrieben: hello.wav“):

```text
33 octets, $0300-$0320 ; 2 fichier(s), 13 symboles
Cassette (après C100R) : 0300.0320R   puis 300R
écrit : hello.wav
```

Ohne `.org` wird das Programm bei `$0300` platziert. Der Dump `.hex` hat die Form, die der Woz Monitor von der Tastatur annimmt, und endet mit dem Startbefehl:

```text title="t1.hex (Auszug)"
0280: 34 12 34 12 10 05 FF 34
0288: 12 FF FF A8 02 EA EA EA
…
0280R
```

## Syntax

Eine Zeile enthält in dieser Reihenfolge, alles freiwillig: eine Marke, einen Befehl oder eine Direktive und einen Kommentar nach `;`. Mnemonics und Direktiven dürfen klein oder groß geschrieben werden; Symbole unterscheiden Groß- und Kleinschreibung.

```asm
PTR     = $06           ; eine Konstante (kein Byte erzeugt)
start:  lda #0          ; eine globale Marke, gefolgt von einem Doppelpunkt
@loop:  inx             ; eine lokale Marke: sichtbar bis zur nächsten globalen
        bne @loop
:       dex             ; eine anonyme Marke
        bne :-          ; :- die vorige, :+ die nächste, :-- die davor
```

{{!lokale Marken}}{{!anonyme Marken}}

**Zahlen**: `255` dezimal, `$FF` hexadezimal, `%1010` binär, `'A'` für ein Zeichen ($41). Die Tastatur liefert ihre Codes mit **gesetztem Bit 7**: Um eine Taste zu vergleichen, schreiben Sie `'A' | $80`; `apple1.inc` definiert ebenso `CR = $8D`. `*` steht für die aktuelle Adresse.

**Operatoren**, vom schwächsten zum stärksten Vorrang:

| Vorrang | Operatoren | Bedeutung |
|---|---|---|
| 1 | <code>&#124;&#124;</code> | logisches Oder (Ergebnis 0 oder 1) |
| 2 | `&&` | logisches Und |
| 3 | `=` `<>` `<` `>` `<=` `>=` | Vergleiche (0 oder 1) |
| 4 | `+` `-` <code>&#124;</code> | Addition, Subtraktion, bitweises Oder |
| 5 | `*` `/` `&` `^` `<<` `>>` | Multiplikation, Division, Und, exklusives Oder, Schiebungen |
| 6 (einstellig) | `<` `>` `-` `~` `!` | niederwertiges Byte, höherwertiges Byte, Negation, Komplement, logisches Nicht |

So lädt `lda #<message` das niederwertige Byte der Adresse und `lda #>message` das höherwertige; `#>S+1` ist `(>S)+1`. Es gibt keinen Modulo-Operator (`%` oder `.mod` ergeben einen Syntaxfehler).

**Zero Page oder absolut.** Eine Adresse unter `$100`, die bei der Assemblierung bekannt ist, wird als Zero Page codiert (`lda $0006` ergibt `A5 06`, wie bei ca65). Um die absolute Form zu erzwingen, schreiben Sie `a:` (`lda a:$06` ergibt `AD 06 00`); um die Zero Page zu erzwingen, `z:`.

!!! attention "Zero-Page-Konstanten vor ihrer Verwendung definieren"
    Steht `PTR = $06` **nach** `lda PTR`, kennt der Assembler den Wert noch nicht und codiert die Adresse mit 2 Bytes. Er meldet das gelb: `Warnung: Zero-Page-Adresse mit 2 Bytes kodiert (Symbol erst weiter unten definiert; vorher definieren oder z: bzw. a: schreiben)`. Das Programm funktioniert, ist aber länger, und die Arten `(zp),Y` werden unmöglich.

## Direktiven

| Direktive | Wirkung | Beispiel |
|---|---|---|
| `.org adr` | Assemblier- (und Lade-)adresse | `.org $0300` |
| `.byte` (`.byt`) | Bytes, Zeichenketten eingeschlossen | `.byte CR, "OK", 0` |
| `.word` (`.addr`) | 16-Bit-Wörter, niederwertiges Byte zuerst | `.word 10000, end` |
| `.asciiz` | mit Null abgeschlossene Zeichenkette | `.asciiz "HALLO"` |
| `.res n[, wert]` | reserviert n Bytes (0 oder wert) | `.res 3, $EA` |
| `.include "d"` | fügt eine Datei ein | `.include "apple1.inc"` |
| `.macro name args` … `.endmacro` | definiert ein Makro (`.mac`, `.endmac`) | siehe `macros.s` |
| `.local a, b` | Marken, die jedem Makroaufruf eigen sind | |
| `.repeat n[, var]` … `.endrep` | wiederholt einen Block, var = 0 … n−1 | `.repeat 16, I` |
| `.if` `.elseif` `.else` `.endif` | bedingte Assemblierung | `.if WIDTH > 30` |
| `.ifdef` `.ifndef` | je nachdem, ob ein Symbol existiert | `.ifdef APPLE1` |
| `.ifblank` `.ifnblank` | Makroargument leer oder nicht | |
| `.defined(sym)` | 1, wenn das Symbol existiert (in einem Ausdruck) | `.if .defined(d)` |
| `.define name wert` | Textersetzung (`.undefine`) | `.define CHARACTER '*'` |
| `.setcpu "65C02"` | nimmt Befehle des 65C02 an (`.pc02`, `.p02`) | |
| `.segment` `.code` `.data` `.rodata` `.bss` | aus Kompatibilitätsgründen angenommen | |
| `.feature string_escapes` | `\n`, `\"` … in Zeichenketten | |

{{!.org}}{{!.byte}}{{!.word}}{{!.asciiz}}{{!.res}}{{!.include}}{{!.macro}}{{!.repeat}}{{!.if}}{{!.setcpu}}

!!! attention "Segmente verschieben nichts"
    Mit ca65 und ld65 schicken `.code`, `.data` und `.bss` den Code in Bereiche, die die Linker-Konfiguration festlegt. Der eingebaute Assembler erzeugt **einen einzigen Block** in der Reihenfolge des Quelltexts: Segmentdirektiven werden angenommen, wirken aber nicht, und `.bss` erzeugt Nullen. Für ein Projekt, das eine echte `.cfg`-Datei braucht, verwendet die Bibliothek ca65 und ld65 (Kapitel 4).

## Makros, .repeat und bedingte Assemblierung

Ein Makro ist eine Vorlage aus Zeilen, die bei jedem Aufruf mit ihren Argumenten kopiert wird. `.local` gibt jedem Aufruf eigene Marken, ohne Konflikte. `.repeat` entrollt einen Block bei der Assemblierung, was auch praktisch ist, um **Tabellen zu berechnen**:

```asm
quadrate:
.repeat 16, I
        .byte I * I     ; 0, 1, 4, 9 … 225
.endrep
```

Kapitel 23 stellt `macros.s` vor, das all das verwendet, und `gen2-lores.s`, das die Adressen der 24 Bildschirmzeilen mit `.repeat` berechnet.

## Fehlermeldungen

Jeder Fehler nennt die Datei, die Zeile, die Spalte und den beanstandeten Text; solange ein Fehler bleibt, wird kein Byte geschrieben. Der Editor zeigt die Meldungen in der Sprache der Oberfläche (unten auf Deutsch); die Befehlszeile gibt sie auf Französisch aus.

| Meldung | Übliche Ursache |
|---|---|
| `Syntaxfehler` | unbekannter Operator, fehlende Klammer |
| `unbekannter Befehl, Direktive oder Makro` | Tippfehler; Befehl des 65C02 ohne `.setcpu`; Marke ohne Doppelpunkt |
| `Adressierungsart für diesen Befehl nicht möglich` | `sta #5`, `ldx ($06),y` … |
| `undefiniertes Symbol` | `jsr ECO` statt `ECHO`; vergessenes `.include` |
| `Symbol bereits definiert` | zwei Marken mit demselben Namen |
| `Wert außerhalb des Bereichs` | `lda #300`, Adresse über `$FFFF` |
| `Sprung zu weit (-128 bis +127 Bytes)` | `bne weit` durch `beq :+` / `jmp weit` / `:` ersetzen |
| `.include-Datei nicht gefunden` | Name oder Ordner: `-I` hinzufügen |
| `schlecht geschlossener Block (.macro/.endmacro, .if/.endif)` | vergessenes `.endif` oder `.endmacro` |
| `Division durch null` | in einem Ausdruck |
| `Zeichenkette oder Zeichen ohne schließendes Anführungszeichen` | `"HALLO` |
| `Wert an dieser Stelle unbekannt (.org, .res, .if, .repeat)` | diese Direktiven brauchen einen bereits definierten Wert |

![Ein Fehler im Editor: ECO statt ECHO. Ein Klick auf die Meldung führt zur Zeile.](img/de/brut/editeur-asm-erreur.png "large")

## Kompatibilität mit ca65 und xa

Der eingebaute Assembler folgt der Syntax von **ca65** (Paket cc65) in allem, was ein Programm aus einem Block braucht. Die acht Beispiele aus Kapitel 23 wurden in ihrer Originalfassung mit beiden Werkzeugen assembliert und Byte für Byte verglichen. Die deutschen Fassungen unterscheiden sich nur in Kommentaren und Texten; ihre Größe mit dem eingebauten Assembler steht in der letzten Spalte (sie wurden nicht erneut mit ca65 verglichen):

| Quelltext | Bytes (Original) | Eingebaut = ca65 | Bytes (deutsch) |
|---|---|---|---|
| `hello.s` | 33 | <span class="status-ok">identisch</span> | 33 |
| `clavier.s` | 32 | <span class="status-ok">identisch</span> | 32 |
| `hexa.s` | 64 | <span class="status-ok">identisch</span> | 64 |
| `decimal.s` | 117 | <span class="status-ok">identisch</span> | 117 |
| `jeu.s` | 222 | <span class="status-ok">identisch</span> | 242 |
| `macros.s` | 135 | <span class="status-ok">identisch</span> | 136 |
| `gen2-lores.s` | 110 | <span class="status-ok">identisch</span> | 110 |
| `gen2-hires.s` | 94 | <span class="status-ok">identisch</span> | 94 |

Für **xa** (xa65) geschriebene Quelltexte werden an ihren Zeilen `#include` oder `#define` erkannt und vor der Assemblierung Zeile für Zeile übersetzt (die Zeilennummern der Fehler bleiben richtig): `#define`, `#include`, `*=`, `.BYT`, `.ASC`, Kommentare `/* */` und Marken in Spalte 0 ohne Doppelpunkt. Die Diashow von Dave Schmenk (Kapitel 9) wird so assembliert, auf dieselben 3456 Bytes wie mit xa 2.4.1.

Für **GitHub-Projekte** bevorzugt die Bibliothek ca65 und ld65, wenn sie installiert sind (oder wenn das Projekt eine ld65-Konfiguration mitbringt), standardmäßig mit `-D APPLE1`; sonst verwendet sie den eingebauten Assembler. Die Datei `apple1.toml` des Repositorys legt die Adressen und Symbole fest (Kapitel 4).
