//! apple1-emu (Retro Dev Kit Apple 1) : émulateur Apple-1 (1976) en terminal, et outils cassette ACI.

mod tui;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use apple1_core::aciwav::{self, ExportOptions};
use apple1_core::hexdump;
use apple1_core::wav::{self, AudioFormat};
use apple1_core::program::{self, Program};
use apple1_core::display::CursorBlink;
use apple1_core::roms;
use apple1_core::{Config, Timing};
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "apple1-emu",
    version,
    about = "Retro Dev Kit Apple 1 — Le péché originel. Émulateur Apple-1 (1976) : 6502, PIA 6821, Woz Monitor, interface cassette ACI et fichiers WAV/AIFF",
    args_conflicts_with_subcommands = true
)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmd>,
    #[command(flatten)]
    run: RunArgs,
}

#[derive(Subcommand)]
enum Cmd {
    /// Lancer l'émulateur (commande par défaut)
    Run(RunArgs),
    /// Convertir un programme (hex Woz, binaire) en cassette ACI .wav ou .aif/.aiff (format selon l'extension)
    #[command(visible_alias = "export")]
    ExportWav(ExportArgs),
    /// Décoder une cassette ACI (.wav ou .aif/.aiff) vers un vidage hexadécimal ou un binaire
    #[command(visible_alias = "import")]
    ImportWav(ImportArgs),
    /// Afficher le contenu d'un fichier (WAV, hex, binaire)
    Info {
        /// Fichier (FICHIER ou FICHIER@ADR)
        file: String,
        /// Simuler l'entrée analogique de l'ACI (stock = carte 1976 0,01 µF ; mimeo = 0,1 µF ; gen2)
        #[arg(long, value_name = "MODELE")]
        aci_input: Option<String>,
        /// Tension crête à la prise pour la pleine échelle numérique (sinon : balayage 0,1 à 2 V)
        #[arg(long, value_name = "V")]
        volts: Option<f64>,
        /// Résistance série entre le lecteur et FROM TAPE (ohms)
        #[arg(long, value_name = "OHMS", default_value_t = 0.0)]
        series: f64,
    },
    /// Fichiers de PROM 256 x 4 (82S129, 3601...) : découper une ROM en quartets ou les réassembler
    #[command(subcommand)]
    Prom(PromCmd),
    /// Tokeniser un listing Integer BASIC (.bas) comme l'interpréteur, et
    /// l'écrire en cassette (.wav/.aif) ou en vidage (.txt) : bloc $004A-$00FF + programme
    Basic {
        /// Listing BASIC (texte, une ligne numérotée par ligne)
        input: PathBuf,
        /// Fichier de sortie (.wav, .aif, .txt) ; sinon, seulement vérifier
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// RAM de la machine cible en Ko : 8, 12, 16 ou 32 [défaut : la plus petite qui suffit]
        #[arg(long, value_name = "KO")]
        ram: Option<u32>,
    },
    /// Assembler un source 6502 (syntaxe ca65 : .org, .byte, .word, .res,
    /// .asciiz, .include, .macro…) et l'écrire en cassette (.wav/.aif),
    /// binaire (.bin) ou vidage Woz (.hex/.txt) ; sans .org : chargé en $0300
    Asm {
        /// Source assembleur (.s, .asm)
        input: PathBuf,
        /// Fichier de sortie (.wav, .aif, .bin, .hex, .txt) ; sinon, seulement vérifier
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Dossier supplémentaire où chercher les .include (répétable)
        #[arg(short = 'I', long = "include", value_name = "DOSSIER")]
        include: Vec<PathBuf>,
        /// Écrire le listing (adresse, octets, source) dans ce fichier
        #[arg(short, long, value_name = "FICHIER")]
        listing: Option<PathBuf>,
    },
    /// Photo (.jpg, .png) ou texte (.txt) → art ASCII 40x24 pour l'Apple-1 :
    /// affiché, ou écrit en texte (.txt), BASIC (.bas), assembleur ca65
    /// (.s/.asm) ou cassette (.wav/.aif, programme 6502 sauf --format basic)
    Ascii(AsciiArgs),
    /// Convertir entre vidage hexadécimal (.txt), binaire (.bin), WAV (.wav) et AIFF (.aif)
    Convert {
        /// Fichier d'entrée (FICHIER ou FICHIER@ADR)
        input: String,
        /// Fichier de sortie (format choisi par l'extension)
        output: PathBuf,
    },
}

#[derive(Args, Clone)]
pub struct AsciiArgs {
    /// Image (.jpg, .jpeg, .png) ou art déjà en texte (.txt)
    input: PathBuf,
    /// Sortie : .txt, .bas, .s/.asm, .wav/.aif ; sinon, l'art est affiché
    #[arg(short, long)]
    output: Option<PathBuf>,
    /// Colonnes (1 à 40)
    #[arg(long, default_value_t = 40)]
    cols: usize,
    /// Lignes (1 à 24) ; 23 : l'invite ne fait pas défiler l'image
    #[arg(long, default_value_t = 24)]
    rows: usize,
    /// Zoom (1 = la plus grande partie de l'image au format de l'écran)
    #[arg(long, default_value_t = 1.0)]
    zoom: f32,
    /// Centre de la partie gardée, 0 à 1 (largeur)
    #[arg(long, default_value_t = 0.5)]
    cx: f32,
    /// Centre de la partie gardée, 0 à 1 (hauteur)
    #[arg(long, default_value_t = 0.5)]
    cy: f32,
    /// Luminosité, -1 à 1
    #[arg(long, default_value_t = 0.0, allow_negative_numbers = true)]
    brightness: f32,
    /// Contraste (1 = inchangé)
    #[arg(long, default_value_t = 1.0)]
    contrast: f32,
    /// Gamma (1 = inchangé, > 1 éclaircit)
    #[arg(long, default_value_t = 1.0)]
    gamma: f32,
    /// Inverser (sombre = beaucoup d'encre)
    #[arg(long)]
    invert: bool,
    /// Tramage Floyd-Steinberg
    #[arg(long)]
    dither: bool,
    /// Ne pas étirer les niveaux de gris (2 %-98 %) avant les réglages
    #[arg(long)]
    no_auto_level: bool,
    /// Caractères : simple, full, slideshow, ou la liste (majuscules et symboles)
    #[arg(long, default_value = "full")]
    ramp: String,
    /// Programme d'une cassette : asm (300R, sans BASIC) ou basic
    #[arg(long, default_value = "asm")]
    format: String,
    /// Titre (REM / commentaire)
    #[arg(long)]
    title: Option<String>,
}

#[derive(Args, Clone, Default)]
pub struct RunArgs {
    /// RAM principale en Ko à partir de $0000 (4 à 48)
    #[arg(long, default_value_t = 8, value_name = "KO")]
    pub ram: u32,
    /// Pas de RAM en $E000-$EFFF
    #[arg(long)]
    pub no_e000: bool,
    /// Pas de carte cassette ACI
    #[arg(long)]
    pub no_aci: bool,
    /// Charger l'Integer BASIC depuis votre fichier (binaire 4 Ko chargé en $E000, ou vidage hex)
    #[arg(long, value_name = "FICHIER")]
    pub basic: Option<PathBuf>,
    /// Charger un programme en mémoire : FICHIER[@ADR] (.txt hex, .bin, .wav, .aif) ; répétable
    #[arg(long, value_name = "FICHIER[@ADR]")]
    pub load: Vec<String>,
    /// Insérer un WAV ou AIFF dans le lecteur cassette virtuel
    #[arg(long, value_name = "WAV|AIFF")]
    pub tape: Option<PathBuf>,
    /// Taper le contenu d'un fichier texte au démarrage
    #[arg(long = "type", value_name = "FICHIER")]
    pub type_file: Option<PathBuf>,
    /// Lancer le programme à cette adresse après chargement (tape « ADRR »)
    #[arg(long, value_name = "ADR")]
    pub run: Option<String>,
    /// Démarrer en mode rapide (sinon vitesse réelle)
    #[arg(long)]
    pub fast: bool,
    /// Désactiver la limite de l'affichage (1 caractère par image, ~60 car./s)
    #[arg(long)]
    pub no_throttle: bool,
    /// Ne pas modéliser le rafraîchissement DRAM (1,0227 MHz au lieu de 0,960 MHz effectifs)
    #[arg(long)]
    pub no_refresh: bool,
    /// Carte couleur Uncle Bernie GEN2 : commutateurs $C250-$C257 et RAM $0000-$BFFF
    /// (l'image couleur n'est visible que dans apple1-emu-gui)
    #[arg(long = "carte-couleur", alias = "gen2")]
    pub color_card: bool,
    /// Carte couleur en 50 Hz (312 lignes au lieu de 262)
    #[arg(long = "couleur-50hz")]
    pub color_50hz: bool,
    /// Carte graphique P-LAB (TMS9918A en $CC00-$CFFF ; l'image n'est visible que dans apple1-emu-gui)
    #[arg(long = "plab")]
    pub plab: bool,
    /// Fréquence de clignotement du curseur en Hz [défaut : 2.18, valeur nominale du NE555 D13]
    #[arg(long, value_name = "HZ")]
    pub cursor_hz: Option<f64>,
    /// Part de la période où le curseur est visible, 0 à 1 [défaut : 0.667, comme le NE555]
    #[arg(long, value_name = "FRACTION")]
    pub cursor_duty: Option<f64>,
    /// Image du Woz Monitor (PROM A2/A1) : 256 octets, ou 512 octets (PROM 512 x 4, voir --monitor-page)
    #[arg(long, value_name = "FICHIER")]
    pub monitor_rom: Option<PathBuf>,
    /// Page de l'image --monitor-rom de 512 octets : 0 (A8 bas, standard) ou 1 (A8 haut, page cachée)
    #[arg(long, value_name = "N", default_value_t = 0)]
    pub monitor_page: u8,
    /// Image de la PROM ACI (A3/A4) : 256 octets, ou 512 octets (voir --aci-page)
    #[arg(long, value_name = "FICHIER")]
    pub aci_rom: Option<PathBuf>,
    /// Page de l'image --aci-rom de 512 octets : 0 (standard) ou 1 (page cachée)
    #[arg(long, value_name = "N", default_value_t = 0)]
    pub aci_page: u8,
}

#[derive(Args)]
struct ExportArgs {
    /// Programme d'entrée : FICHIER[@ADR] (.txt hex Woz, .bin avec @ADR, .wav, .aif)
    input: String,
    /// Fichier de sortie : .aif/.aiff = AIFF (profil smartphone), sinon WAV
    #[arg(short, long)]
    output: PathBuf,
    /// Plage(s) à extraire, syntaxe Woz (ex. 300.3FF, ou toute une ligne
    /// « 4A.00FFR 300.FFFR ») ; répétable. Par défaut : tout le programme
    #[arg(long, value_name = "DEBUT.FIN")]
    range: Vec<String>,
    /// Fréquence d'échantillonnage en Hz [défaut : 48000 en AIFF, 44100 en WAV]
    #[arg(long)]
    rate: Option<u32>,
    /// Bits par échantillon, 8 ou 16 [défaut : 16]
    #[arg(long)]
    bits: Option<u16>,
    /// Nombre de canaux (identiques), 1 ou 2 [défaut : 2 en AIFF, 1 en WAV]
    #[arg(long)]
    channels: Option<u16>,
    /// Amplitude, 0.05 à 1.0 [défaut : 0.89 (-1 dBFS) en AIFF, 0.8 en WAV]
    #[arg(long)]
    amplitude: Option<f32>,
    /// Silence avant et après le signal, en secondes [défaut : 1.0 en AIFF, 0.5 en WAV]
    #[arg(long, value_name = "S")]
    silence: Option<f64>,
    /// Ne pas modéliser le rafraîchissement DRAM dans la durée des bits
    #[arg(long)]
    no_refresh: bool,
    /// Ne pas inscrire les adresses dans le fichier (chunk LIST/INFO/ICMT)
    #[arg(long)]
    no_embed: bool,
}

#[derive(Subcommand)]
enum PromCmd {
    /// Découper une ROM de 256 octets en deux fichiers de quartets (format des vidages A1-A4)
    Split {
        /// « woz » (PROM A2 = quartet haut, A1 = bas), « aci » (A3 = haut, A4 = bas) ou un fichier de 256 octets
        #[arg(default_value = "woz")]
        rom: String,
        /// Dossier de sortie
        #[arg(short, long, default_value = ".")]
        output: PathBuf,
        /// Croiser les lignes d'adresse 1 et 7 de la PROM A2 (seulement pour une carte câblée comme le schéma Apple, voir README)
        #[arg(long)]
        swap_a1_a7: bool,
    },
    /// Réassembler deux PROM de quartets (haut, bas) en une ROM de 256 octets et la comparer aux ROM intégrées
    Join {
        /// PROM du quartet haut (A2 ou A3)
        high: PathBuf,
        /// PROM du quartet bas (A1 ou A4)
        low: PathBuf,
        /// Fichier de sortie (facultatif)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// La PROM haute (A2) vient d'une carte aux lignes d'adresse 1 et 7 croisées
        #[arg(long)]
        swap_a1_a7: bool,
    },
}

#[derive(Args)]
struct ImportArgs {
    /// Fichier WAV ou AIFF (FICHIER ou FICHIER@ADR)
    input: String,
    /// Fichier de sortie (.txt = vidage hex, .bin = binaire). Sans -o : vidage sur la sortie standard
    #[arg(short, long)]
    output: Option<PathBuf>,
    /// Adresse de chargement (fichier d'un autre outil ; plages placées bout à bout)
    #[arg(long, value_name = "ADR")]
    addr: Option<String>,
    /// Plages de chaque enregistrement, comme la ligne ACI de lecture (ex. « 4A.00FFR 300.FFFR ») ; répétable
    #[arg(long, value_name = "DEBUT.FIN")]
    range: Vec<String>,
}

fn load_spec(spec: &str, addr: Option<u16>) -> Result<Program, String> {
    let (path, a) = program::split_at_addr(spec)?;
    program::load_file(Path::new(&path), addr.or(a))
}

fn describe(p: &Program) -> String {
    let mut s = String::new();
    for (a, b) in p.ranges() {
        s.push_str(&format!("  {a:04X}.{b:04X}  ({} octets)\n", b as u32 - a as u32 + 1));
    }
    if let Some(r) = p.run {
        s.push_str(&format!("  exécution : {r:04X}R\n"));
    }
    for n in &p.notes {
        s.push_str(&format!("  attention : {n}\n"));
    }
    s
}

fn select_ranges(p: &Program, ranges: &[String]) -> Result<Program, String> {
    if ranges.is_empty() {
        return Ok(p.clone());
    }
    let mut mem = vec![None::<u8>; 65536];
    for (a, d) in &p.blocks {
        for (i, &b) in d.iter().enumerate() {
            mem[(*a as usize + i) & 0xFFFF] = Some(b);
        }
    }
    let mut out = Program { run: p.run, ..Default::default() };
    let mut list = Vec::new();
    for r in ranges {
        list.extend(program::parse_ranges(r)?);
    }
    for (a, b) in list {
        let r = format!("{a:04X}.{b:04X}");
        let d = (a as usize..=b as usize)
            .map(|x| mem[x].ok_or_else(|| format!("l'adresse {x:04X} de la plage {r} n'est pas dans le programme")))
            .collect::<Result<Vec<u8>, String>>()?;
        out.blocks.push((a, d));
    }
    Ok(out)
}

fn basic(input: &Path, output: Option<&Path>, ram: Option<u32>) -> Result<(), String> {
    use apple1_core::intbasic;
    if let Some(k) = ram.filter(|k| !intbasic::MEMORY.iter().any(|m| m.0 == *k)) {
        return Err(format!("--ram {k} : valeurs possibles 8, 12, 16 ou 32"));
    }
    let src = std::fs::read_to_string(input).map_err(|e| format!("{} : {e}", input.display()))?;
    let l = intbasic::tokenize(&src);
    for e in &l.errors {
        eprintln!("{} : {e}", input.display());
    }
    if !l.errors.is_empty() {
        return Err(format!("{} erreur(s), rien n'est écrit", l.errors.len()));
    }
    let (lo, hi) = intbasic::layout(l.size(), ram);
    let img = intbasic::Image::build(&l.lines, lo, hi).map_err(|_| format!("programme de {} octets : trop grand pour la mémoire choisie", l.size()))?;
    println!(
        "{} lignes, {} octets ; LOMEM ${:04X} HIMEM ${:04X} PP ${:04X} ; {} octets libres pour les variables ; RAM {} Ko",
        l.lines.len(),
        img.prog.len(),
        img.lomem,
        img.himem,
        img.pp,
        img.free(),
        img.ram_kb()
    );
    println!("Plages ACI (après C100R) : {}   puis E2B3R et RUN", img.aci_command());
    if let Some(out) = output {
        let p = Program { blocks: img.blocks(), run: None, notes: vec![] };
        let o = ExportOptions::for_format(AudioFormat::from_path(out));
        program::save_file(out, &p, &o)?;
        println!("écrit : {}", out.display());
    }
    Ok(())
}

fn asm(input: &Path, output: Option<&Path>, include: &[PathBuf], listing: Option<&Path>) -> Result<(), String> {
    use apple1_core::asm6502;
    let a = match asm6502::assemble_file(input, include) {
        Ok(a) => a,
        Err(errs) => {
            for e in &errs {
                eprintln!("{e}");
            }
            return Err(format!("{} erreur(s), rien n'est écrit", errs.len()));
        }
    };
    for w in &a.warnings {
        eprintln!("{w}");
    }
    println!("{} octets, ${:04X}-${:04X} ; {} fichier(s), {} symboles", a.bytes.len(), a.origin, a.end(), a.files.len(), a.symbols.len());
    println!("Cassette (après C100R) : {}   puis {}", a.aci_range(), a.run_command());
    if let Some(l) = listing {
        std::fs::write(l, asm6502::listing_text(&a)).map_err(|e| format!("{} : {e}", l.display()))?;
        println!("listing : {}", l.display());
    }
    if let Some(out) = output {
        let run = if program::kind_of(out, None) == program::Kind::Hex { Some(a.origin) } else { None };
        let p = Program { blocks: vec![(a.origin, a.bytes.clone())], run, notes: vec![] };
        let o = ExportOptions::for_format(AudioFormat::from_path(out));
        program::save_file(out, &p, &o)?;
        println!("écrit : {}", out.display());
    }
    Ok(())
}

fn ascii(a: &AsciiArgs) -> Result<(), String> {
    use apple1_core::asciiart::{self, Art, Grey, Params};
    use apple1_core::intbasic;
    let ext = |p: &Path| p.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    let ramp = match a.ramp.as_str() {
        "simple" => asciiart::RAMP_SIMPLE.to_string(),
        "full" => asciiart::RAMP_FULL.to_string(),
        "slideshow" => asciiart::RAMP_SLIDESHOW.to_string(),
        r => r.to_string(),
    };
    let art = if ext(&a.input) == "txt" {
        Art::from_text(&std::fs::read_to_string(&a.input).map_err(|e| format!("{} : {e}", a.input.display()))?)
    } else {
        let img = image::open(&a.input).map_err(|e| format!("{} : {e}", a.input.display()))?.to_rgba8();
        let g = Grey::from_rgba(img.width() as usize, img.height() as usize, img.as_raw(), 4);
        let p = Params {
            cols: a.cols,
            rows: a.rows,
            zoom: a.zoom,
            cx: a.cx,
            cy: a.cy,
            brightness: a.brightness,
            contrast: a.contrast,
            gamma: a.gamma,
            invert: a.invert,
            auto_level: !a.no_auto_level,
            dither: a.dither,
            ramp,
            ..Params::default()
        };
        asciiart::convert(&g, &p)
    };
    let title = a.title.clone().unwrap_or_else(|| a.input.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default());
    let Some(out) = &a.output else {
        print!("{}", art.text());
        return Ok(());
    };
    let basic = || {
        let l = intbasic::tokenize(&asciiart::to_basic(&art, &title));
        if let Some(e) = l.errors.first() {
            return Err(format!("BASIC : {e}"));
        }
        let (lo, hi) = intbasic::layout(l.size(), None);
        let img = intbasic::Image::build(&l.lines, lo, hi).map_err(|_| "BASIC : trop grand".to_string())?;
        Ok((Program { blocks: img.blocks(), run: None, notes: vec![] }, format!("E000R (BASIC), C100R, {}, E2B3R, RUN", img.aci_command())))
    };
    let asm = || {
        let a = apple1_core::asm6502::assemble_text(&asciiart::to_asm(&art, &title), None, &[]).map_err(|e| format!("{} erreur(s)", e.len()))?;
        let how = format!("C100R, {}, puis {}", a.aci_range(), a.run_command());
        Ok::<_, String>((Program { blocks: vec![(a.origin, a.bytes)], run: None, notes: vec![] }, how))
    };
    let w = |t: String| std::fs::write(out, t).map_err(|e| format!("{} : {e}", out.display()));
    match ext(out).as_str() {
        "txt" => w(art.text())?,
        "bas" => w(asciiart::to_basic(&art, &title))?,
        "s" | "asm" => w(asciiart::to_asm(&art, &title))?,
        "wav" | "aif" | "aiff" => {
            let (p, how) = if a.format == "basic" { basic()? } else { asm()? };
            let o = ExportOptions::for_format(AudioFormat::from_path(out));
            program::save_file(out, &p, &o)?;
            println!("{} : {} ; {how}", out.display(), p.blocks.iter().map(|(a, d)| format!("${a:04X}-${:04X}", *a as usize + d.len() - 1)).collect::<Vec<_>>().join(" "));
        }
        e => return Err(format!("extension inconnue : .{e} (txt, bas, s, asm, wav, aif)")),
    }
    println!("{} lignes : {}", art.rows(), out.display());
    Ok(())
}

fn export(a: ExportArgs) -> Result<(), String> {
    let p = select_ranges(&load_spec(&a.input, None)?, &a.range)?;
    let fmt = AudioFormat::from_path(&a.output);
    let mut o = ExportOptions::for_format(fmt);
    o.sample_rate = a.rate.unwrap_or(o.sample_rate);
    o.bits = a.bits.unwrap_or(o.bits);
    o.channels = a.channels.unwrap_or(o.channels);
    o.amplitude = a.amplitude.unwrap_or(o.amplitude).clamp(0.05, 1.0);
    if let Some(s) = a.silence {
        o.lead_silence = s.clamp(0.0, 30.0);
        o.tail_silence = o.lead_silence;
    }
    o.timing = Timing { refresh_stall: !a.no_refresh, ..Timing::default() };
    o.embed_address = !a.no_embed;
    if !(8000..=192000).contains(&o.sample_rate) {
        return Err("fréquence d'échantillonnage hors limites (8000 à 192000)".into());
    }
    if o.bits != 8 && o.bits != 16 {
        return Err("--bits doit valoir 8 ou 16".into());
    }
    if o.channels != 1 && o.channels != 2 {
        return Err("--channels doit valoir 1 ou 2".into());
    }
    if fmt == AudioFormat::Wav && o.channels == 2 {
        o.bits = 16;
    }
    let segs: Vec<(u16, &[u8])> = p.blocks.iter().map(|(a, d)| (*a, d.as_slice())).collect();
    let bytes = aciwav::export_wav_bytes(&segs, &o)?;
    std::fs::write(&a.output, &bytes).map_err(|e| format!("{} : {e}", a.output.display()))?;
    let secs = wav::parse_audio(&bytes).map(|x| x.duration_secs()).unwrap_or(0.0);
    eprintln!(
        "Écrit {} ({}, {:.1} s, {} Hz, {} bits, {}, amplitude {:.2}) :",
        a.output.display(),
        if fmt == AudioFormat::Aiff { "AIFF" } else { "WAV" },
        secs,
        o.sample_rate,
        o.bits,
        if o.channels == 2 { "stéréo à canaux identiques" } else { "mono" },
        o.amplitude
    );
    eprint!("{}", describe(&p));
    eprintln!("Sur l'Apple-1 : tapez  C100R  puis  {}  puis RETURN, et lancez la lecture.", p.aci_read_command());
    if let Some(r) = p.run {
        eprintln!("Ensuite :  {r:X}R");
    }
    Ok(())
}

fn print_imported(path: &str, imp: &aciwav::Imported) {
    eprintln!(
        "{} : {} Hz, {} canal(aux), {} bits, {:.1} s, {} enregistrement(s)",
        path,
        imp.sample_rate,
        imp.channels,
        imp.bits,
        imp.duration,
        imp.segments.len()
    );
    for (i, s) in imp.segments.iter().enumerate() {
        eprintln!(
            "  enregistrement {} : {} octets, début à {:.2} s, en-tête {:.1} s ({:.0} µs/demi-période), seuil {:.0} µs",
            i + 1,
            s.data.len(),
            s.start_secs,
            s.header_secs,
            s.header_half * 1e6,
            s.threshold * 1e6
        );
    }
    if !imp.embedded.is_empty() {
        let r: Vec<String> = imp.embedded.iter().map(|(a, b)| format!("{a:04X}.{b:04X}")).collect();
        eprintln!("  adresses inscrites dans le fichier : {}", r.join(" "));
    }
}

fn import(a: ImportArgs) -> Result<(), String> {
    let addr = a.addr.as_deref().map(program::parse_addr).transpose()?;
    let (path, at) = program::split_at_addr(&a.input)?;
    let mut imp = aciwav::import_wav(Path::new(&path))?;
    print_imported(&path, &imp);
    if !a.range.is_empty() {
        let mut list = Vec::new();
        for r in &a.range {
            list.extend(program::parse_ranges(r)?);
        }
        if list.len() != imp.segments.len() {
            eprintln!("  attention : {} plage(s) indiquée(s), {} enregistrement(s) décodé(s)", list.len(), imp.segments.len());
        }
        imp.embedded = list;
    }
    let p = program::wav_to_program(&imp, addr.or(at))?;
    eprint!("{}", describe(&p));
    match a.output {
        Some(out) => {
            program::save_file(&out, &p, &ExportOptions::default())?;
            eprintln!("Écrit {}", out.display());
        }
        None => {
            for (a, d) in &p.blocks {
                print!("{}", hexdump::format(*a, d, None));
            }
        }
    }
    Ok(())
}

fn simulate_input(bytes: &[u8], model: &str, volts: Option<f64>, series: f64, reference: &aciwav::Imported) -> Result<(), String> {
    use apple1_core::aciinput::{comparator_edges, AciInput};
    let a = wav::parse_audio(bytes).map_err(|e| e.to_string())?;
    let list: Vec<f64> = match volts {
        Some(v) => vec![v],
        None => vec![0.05, 0.1, 0.2, 0.3, 0.5, 0.7, 1.0, 1.5, 2.0],
    };
    let m0 = AciInput::by_name(model, 1.0).ok_or("modèle inconnu (stock, mimeo ou gen2)")?;
    eprintln!(
        "Simulation de l'entrée ACI « {model} » : C = {:.0} nF, coupure {:.0} Hz, hystérésis ±{:.2} V (estimée), série {series:.0} Ω",
        m0.coupling_farads * 1e9,
        AciInput { series_ohms: series, ..m0 }.corner_hz(),
        m0.hysteresis_volts
    );
    // L and R are tied together on the cable: the ACI sees their average
    let n = a.channels[0].len();
    let mix: Vec<f32> = (0..n).map(|i| a.channels.iter().map(|c| c[i]).sum::<f32>() / a.channels.len() as f32).collect();
    for v in list {
        let m = AciInput { volts_peak: v, series_ohms: series, ..m0 };
        let segs = apple1_core::tape::decode_edges(&comparator_edges(&mix, a.sample_rate, &m));
        let same = segs.len() == reference.segments.len()
            && segs.iter().zip(&reference.segments).all(|(x, y)| x.data == y.data && x.weak_bits == 0);
        eprintln!(
            "  {v:4.2} V crête : {}",
            if same { "OK (identique au décodage idéal)".to_string() } else { format!("ÉCHEC ({} enregistrement(s) décodé(s))", segs.len()) }
        );
    }
    eprintln!("  (modèle simplifié du schéma de 1976 ; tensions non mesurées sur votre téléphone)");
    Ok(())
}

fn info(spec: &str, aci_input: Option<&str>, volts: Option<f64>, series: f64) -> Result<(), String> {
    let (path, at) = program::split_at_addr(spec)?;
    let bytes = std::fs::read(&path).map_err(|e| format!("{path} : {e}"))?;
    let kind = program::kind_of(Path::new(&path), Some(&bytes));
    if kind == program::Kind::Wav {
        let imp = aciwav::import_wav_bytes(&bytes)?;
        if let Some(model) = aci_input {
            simulate_input(&bytes, model, volts, series, &imp)?;
        }
        print_imported(&path, &imp);
        match program::wav_to_program(&imp, at) {
            Ok(p) => {
                eprint!("{}", describe(&p));
                eprintln!("Commande Apple-1 :  C100R  puis  {}", p.aci_read_command());
            }
            Err(e) => eprintln!("  {e}"),
        }
        return Ok(());
    }
    let p = program::load_bytes(&bytes, kind, at)?;
    eprintln!("{path} : {:?}, {} octets", kind, p.total_len());
    eprint!("{}", describe(&p));
    Ok(())
}

fn convert(input: &str, output: &Path) -> Result<(), String> {
    let p = load_spec(input, None)?;
    let k = program::save_file(output, &p, &ExportOptions::default())?;
    eprintln!("Écrit {} ({:?}) :", output.display(), k);
    eprint!("{}", describe(&p));
    Ok(())
}

fn load_rom(path: &Option<PathBuf>, page: u8, default: &[u8; 256]) -> Result<[u8; 256], String> {
    match path {
        None if page == 0 => Ok(*default),
        None => Err("--monitor-page / --aci-page demandent une image de 512 octets (--monitor-rom / --aci-rom)".into()),
        Some(p) => {
            let b = std::fs::read(p).map_err(|e| format!("{} : {e}", p.display()))?;
            roms::rom_page(&b, page).map_err(|e| format!("{} : {e}", p.display()))
        }
    }
}

pub fn config_from(a: &RunArgs) -> Result<Config, String> {
    Ok(Config {
        ram_kb: a.ram.clamp(4, 48),
        ram_e000: !a.no_e000,
        aci: !a.no_aci,
        display_throttle: !a.no_throttle,
        timing: Timing { refresh_stall: !a.no_refresh, ..Timing::default() },
        monitor_rom: load_rom(&a.monitor_rom, a.monitor_page, roms::WOZ_MONITOR)?,
        aci_rom: load_rom(&a.aci_rom, a.aci_page, roms::ACI_ROM)?,
        color_card: a.color_card,
        color_50hz: a.color_50hz,
        plab_card: a.plab,
        ..Config::default()
    })
}

/// Cursor blink settings from the command line.
pub fn cursor_from(a: &RunArgs) -> Result<CursorBlink, String> {
    let mut b = CursorBlink::apple1();
    if let Some(hz) = a.cursor_hz {
        if !(0.05..=50.0).contains(&hz) {
            return Err(format!("--cursor-hz {hz} : 0.05 à 50 Hz attendus"));
        }
        b.hz = hz;
    }
    if let Some(d) = a.cursor_duty {
        if !(0.0..=1.0).contains(&d) {
            return Err(format!("--cursor-duty {d} : 0 à 1 attendu"));
        }
        b.duty = d;
    }
    Ok(b)
}

fn rom_name(img: &[u8; 256]) -> &'static str {
    if img == roms::WOZ_MONITOR {
        "identique au Woz Monitor intégré"
    } else if img == roms::ACI_ROM {
        "identique à la ROM ACI intégrée"
    } else {
        "différente des ROM intégrées"
    }
}

fn prom(c: PromCmd) -> Result<(), String> {
    use roms::prom;
    match c {
        PromCmd::Split { rom, output, swap_a1_a7 } => {
            let (img, names) = match rom.to_ascii_lowercase().as_str() {
                "woz" | "wozmon" | "monitor" => (*roms::WOZ_MONITOR, ["A2", "A1"]),
                "aci" => (*roms::ACI_ROM, ["A3", "A4"]),
                _ => {
                    let b = std::fs::read(&rom).map_err(|e| format!("{rom} : {e}"))?;
                    (roms::rom_page(&b, 0)?, ["HAUT", "BAS"])
                }
            };
            if swap_a1_a7 && names[0] != "A2" {
                return Err("--swap-a1-a7 ne concerne que la PROM A2 du Woz Monitor (rom « woz »)".into());
            }
            let (hi, lo) = prom::split(&img);
            // Willegal: only PROM A2 has lines 1 and 7 drawn crossed.
            let hi = if swap_a1_a7 { prom::swap_address_lines(&hi, 1, 7) } else { hi };
            std::fs::create_dir_all(&output).map_err(|e| format!("{} : {e}", output.display()))?;
            let tag = if swap_a1_a7 { "_A1-A7-croisees" } else { "" };
            for (n, d, what) in [(names[0], hi, "quartet haut D7-D4"), (names[1], lo, "quartet bas D3-D0")] {
                let t = if n == "A2" { tag } else { "" };
                let p = output.join(format!("PROM-{n}{t}.bin"));
                std::fs::write(&p, d).map_err(|e| format!("{} : {e}", p.display()))?;
                println!("{} : {what}, 256 octets (valeurs 0-F)", p.display());
            }
            if swap_a1_a7 {
                println!("attention : PROM A2 aux lignes d'adresse 1 et 7 croisées (carte câblée comme le schéma Apple, pas comme la carte d'origine)");
            }
            Ok(())
        }
        PromCmd::Join { high, low, output, swap_a1_a7 } => {
            let rd = |p: &PathBuf| std::fs::read(p).map_err(|e| format!("{} : {e}", p.display()));
            let (h, l) = (rd(&high)?, rd(&low)?);
            if h.iter().chain(l.iter()).any(|b| *b > 0x0F) {
                println!("attention : des octets ont leurs 4 bits hauts non nuls (ignorés)");
            }
            let h = if swap_a1_a7 {
                let a: [u8; 256] = h.as_slice().try_into().map_err(|_| "la PROM haute doit faire 256 octets".to_string())?;
                prom::swap_address_lines(&a, 1, 7).to_vec()
            } else {
                h
            };
            let img = prom::join(&h, &l)?;
            println!("ROM réassemblée : {}", rom_name(&img));
            for (name, r) in [("Woz Monitor", roms::WOZ_MONITOR), ("ACI", roms::ACI_ROM)] {
                let d: Vec<usize> = (0..256).filter(|&i| img[i] != r[i]).collect();
                if !d.is_empty() && d.len() < 32 {
                    let list: Vec<String> = d.iter().map(|&i| format!("+{i:02X}: {:02X}/{:02X}", img[i], r[i])).collect();
                    println!("  écarts avec {name} (fichier/intégrée) : {}", list.join(" "));
                }
            }
            if let Some(o) = output {
                std::fs::write(&o, img).map_err(|e| format!("{} : {e}", o.display()))?;
                println!("écrit : {}", o.display());
            }
            Ok(())
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let r = match cli.cmd {
        None => tui::run(cli.run),
        Some(Cmd::Run(a)) => tui::run(a),
        Some(Cmd::ExportWav(a)) => export(a),
        Some(Cmd::ImportWav(a)) => import(a),
        Some(Cmd::Info { file, aci_input, volts, series }) => info(&file, aci_input.as_deref(), volts, series),
        Some(Cmd::Convert { input, output }) => convert(&input, &output),
        Some(Cmd::Basic { input, output, ram }) => basic(&input, output.as_deref(), ram),
        Some(Cmd::Ascii(a)) => ascii(&a),
        Some(Cmd::Asm { input, output, include, listing }) => asm(&input, output.as_deref(), &include, listing.as_deref()),
        Some(Cmd::Prom(c)) => prom(c),
    };
    match r {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Erreur : {e}");
            ExitCode::FAILURE
        }
    }
}
