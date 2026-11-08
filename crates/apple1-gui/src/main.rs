//! Retro Dev Kit Apple 1 ("The Original Sin"): Apple-1 emulator, graphical interface (egui).

mod app;
mod fontsz;
mod wizard;
mod ascii;
mod editor;
mod eprom_ui;
mod debugger;
mod dock;
mod uicolors;
mod layout;
mod config;
mod audio;
mod build;
mod crt;
mod github;
mod gen2screen;
mod help;
mod i18n;
mod known;
mod launch;
mod library;
mod params;
mod screen;
mod session;
mod syntax;
mod tapesig;

use std::path::PathBuf;

use apple1_core::display::CursorBlink;
use apple1_core::Config;
use eframe::egui;

use crate::app::{unescape, App, Script};
use crate::config::{GuiConfig, Zoom, ZOOM_MAX};
use crate::crt::Screening;

const USAGE: &str = "\
Retro Dev Kit Apple 1 — Le péché originel\napple1-emu-gui : émulateur Apple-1, interface graphique

Usage : apple1-emu-gui [OPTIONS] [FICHIER...]

  FICHIER                 programme à charger en mémoire au démarrage (.wav .aiff .txt .bin)
  --zoom N|ajuster        zoom de l'écran (1 à 8, ou ajuster à la fenêtre)
  --phosphore vert|blanc|ambre
  --balayage, --sans-balayage   lignes de balayage (effet écran cathodique)
  --screening, --sans-screening écran vintage (moniteur à tube des années 1970)
  --screening-options LISTE     réglages (valeurs de 0 à 1), ex. intensite=0.7,
                                remanence=0.5, courbure=0.4, scintillement, sans-halo,
                                sans-separation, sans-adoucir, sans-remanence, sans-courbure,
                                masque, sans-masque (masque de phosphores RVB, GEN2)
  --bibliotheque DOSSIER  dossier de la bibliothèque de programmes
  --basic FICHIER         fichier du BASIC pour le bouton BASIC
  --taille LxH            taille de la fenêtre en points (ex. 1280x800)
  --cursor-hz HZ          fréquence de clignotement du curseur (défaut 2,18 Hz)
  --carte-couleur         carte couleur Uncle Bernie GEN2 (2e moniteur, couleur) ;
  --sans-carte-couleur    … la retirer (réglage aussi dans ⚙ Préférences)
  --plab                  carte graphique P-LAB (TMS9918A en $CC00-$CFFF, 2e écran) ;
  --sans-plab             … la retirer (bouton P-LAB de la barre)
  --couleur-50hz          carte couleur en 50 Hz (312 lignes) ; --couleur-60hz (défaut)
  --langue CODE|auto      langue de l'interface : fr, en, de, ru, es, it, pt, ou auto
                          (auto = disposition du clavier, puis LANG ; réglage dans ⚙ Préférences)
  --mode-cassette         la bibliothèque charge par la cassette (ACI) au lieu de l'instantané
  --gen2-screening        écran vintage aussi sur l'écran GEN2
  --theme clair|sombre|systeme   thème de l'interface
  --taille-icones petite|normale|grande|tres-grande
  --barre-complete        jamais de barre compacte (icônes seules)
  --charger-biblio S:NOM  après S secondes, charge NOM de la bibliothèque (comme un double-clic)
  --lancer-biblio S:NOM   … charge puis lance NOM (bouton ▶)
  --parametres S:NOM      … ouvre les paramètres ⚙ de NOM
  --preferences S         … ouvre la fenêtre ⚙ Préférences
  --onglet-prefs ONGLET   … sur cet onglet (interface, editeur, bibliotheque, cassette, gen2, avance)
  --editeur FICHIER       ouvre FICHIER (.bas, .s, .asm) dans l'éditeur (répétable) ;
  --editeur-lancer S      … et appuie sur « ▶ Lancer » après S secondes
  --editeur-recherche T   … barre de recherche ouverte sur T
  --editeur-menu          … menu du clic droit ouvert (captures)
  --ecran-menu            menu du clic droit de l'écran ouvert (captures)
  --menu-etat             menu « État » ouvert ; --eprom : programmateur d'EPROM ouvert ;
                          --fenetre-fichiers : fenêtre « Charger » ouverte (captures)
  --ascii FICHIER         ouvre l'outil « Photo → ASCII » avec FICHIER (.jpg, .png, .txt) ;
  --ascii-reglages R      … réglages k=v séparés par ; (zoom, cx, cy, lignes, colonnes,
                          luminosite, contraste, gamma, inverser, tramage, rampe, asm)
  --ascii-lancer S        … et appuie sur son « ▶ Lancer » après S secondes
  --editeur-exporter S:FICHIER  … et « 📼 Exporter WAV » (.wav/.aiff) après S secondes
  --ecran-separe          écran de l'Apple-1 dans sa propre fenêtre
  --debogueur             ouvre le débogueur (panneau à droite) ;
  --dbg-arret LISTE       … points d'arrêt : adresses ou étiquettes ($0300,loop), L190 = ligne BASIC
  --dbg-surveiller LISTE  … variables BASIC (A,C$,D(2)) et mémoire ($30,$0200.020F,$4A w)
  --dbg-commande S:CMD    … après S secondes : pause, continuer, pas, dessus, sortir, basic (répétable)
  --cassette FICHIER      insère FICHIER (.wav/.aiff) comme cassette virtuelle
  --taper TEXTE           tape TEXTE au clavier après 1 s (\\n = RETURN) ;
  --taper-a S:TEXTE       … ou après S secondes (répétable)
  --bienvenue, --welcome  assistant de démarrage (captures)
  --offre-version         question « nouvelle version du kit » (captures)
  --barre-fin             barre d'outils défilée jusqu'au groupe Cassette (captures)
  --assistant 1|2|3       assistant de démarrage, sur cet écran (captures)
  --capture IMAGE.png     enregistre une capture de la fenêtre (après --delai S, 5 s par défaut)
  --quitter               quitte après la capture
  -h, --help              cette aide

Raccourcis : Ctrl+plus / Ctrl+moins / Ctrl+0 (zoom de l'écran, ou taille du texte de la fenêtre
sous la souris), F11 (plein écran), Ctrl+R (RESET),
Ctrl+L (effacer l'écran), Ctrl+V (coller), F3 (vitesse réelle / rapide).
Réglages enregistrés dans ~/.config/apple1-emu/gui.json ; en mode portable (fichier « portable »\nà la racine du kit, ou APPLE1_PORTABLE=1), dans config/gui.json du dossier du kit.";

/// Title bar of the main window: the same in every language (Wirehead, #59).
/// The « À propos » box keeps the translated subtitle.
pub const WINDOW_TITLE: &str = "Retro Dev Kit Apple 1 — Original Sin";

fn icon() -> Option<egui::IconData> {
    let dec = png::Decoder::new(std::io::Cursor::new(&include_bytes!("../assets/icon.png")[..]));
    let mut r = dec.read_info().ok()?;
    let mut buf = vec![0; r.output_buffer_size()];
    let info = r.next_frame(&mut buf).ok()?;
    if info.color_type != png::ColorType::Rgba {
        return None;
    }
    buf.truncate(info.buffer_size());
    Some(egui::IconData { rgba: buf, width: info.width, height: info.height })
}

fn parse_args() -> Result<(Script, GuiConfig, bool, Option<f64>), String> {
    let (mut cfg, from_disk) = GuiConfig::load();
    let mut s = Script::default();
    let mut delay = 5.0;
    let mut hz = None;
    let mut args = std::env::args().skip(1);
    let mut capture = None;
    while let Some(a) = args.next() {
        let mut val = |name: &str| args.next().ok_or(format!("{name} : valeur manquante"));
        match a.as_str() {
            "-h" | "--help" => {
                println!("{USAGE}");
                std::process::exit(0);
            }
            "--zoom" => {
                let v = val("--zoom")?;
                s.zoom = Some(match v.to_lowercase().as_str() {
                    "ajuster" | "fit" | "0" => Zoom::Ajuster,
                    n => {
                        let z: u32 = n.trim_end_matches('x').parse().map_err(|_| format!("--zoom : « {v} » invalide"))?;
                        Zoom::Fixe(z.clamp(1, ZOOM_MAX))
                    }
                });
            }
            "--phosphore" => {
                cfg.phosphor = match val("--phosphore")?.to_lowercase().as_str() {
                    "vert" | "green" => screen::Phosphor::Vert,
                    "blanc" | "white" => screen::Phosphor::Blanc,
                    "ambre" | "amber" => screen::Phosphor::Ambre,
                    o => return Err(format!("--phosphore : « {o} » (vert, blanc ou ambre)")),
                }
            }
            "--balayage" => cfg.scanlines = true,
            "--sans-balayage" => cfg.scanlines = false,
            "--screening" => cfg.screening.on = true,
            "--gen2-screening" => cfg.screening2_on = true,
            "--theme" => {
                cfg.theme = match val("--theme")?.as_str() {
                    "clair" | "light" => config::Theme::Clair,
                    "sombre" | "dark" => config::Theme::Sombre,
                    "systeme" | "system" => config::Theme::Systeme,
                    _ => return Err("--theme : clair, sombre ou systeme".into()),
                }
            }
            "--taille-icones" => {
                cfg.icon_size = match val("--taille-icones")?.as_str() {
                    "auto" | "normale" => config::IconSize::Normale,
                    "petite" => config::IconSize::Petite,
                    "grande" => config::IconSize::Grande,
                    "tres-grande" => config::IconSize::TresGrande,
                    _ => return Err("--taille-icones : petite, normale, grande, tres-grande (auto = normale, ancien nom)".into()),
                }
            }
            "--barre-complete" => cfg.compact_bar = false,
            "--sans-screening" => cfg.screening.on = false,
            "--screening-options" => {
                let v = val("--screening-options")?;
                let sc = &mut cfg.screening;
                sc.on = true;
                for o in v.split(',').map(str::trim).filter(|o| !o.is_empty()) {
                    let (name, flag) = match o.strip_prefix("sans-") {
                        Some(n) => (n, false),
                        None => (o, true),
                    };
                    match name {
                        "separation" => sc.gaps = flag,
                        "adoucir" => sc.soften = flag,
                        "halo" => sc.glow = flag,
                        "remanence" => sc.afterglow = if flag { Screening::default().afterglow } else { 0.0 },
                        "courbure" => sc.curvature = flag,
                        "scintillement" => sc.flicker = flag,
                        "masque" => sc.rgb_mask = flag,
                        _ => {
                            let (key, x) = name.split_once('=').ok_or(format!("--screening-options : « {o} » inconnu"))?;
                            let x = x.parse::<f32>().map_err(|_| format!("--screening-options : « {o} »"))?.clamp(0.0, 1.0);
                            match key {
                                "intensite" => sc.intensity = x,
                                "remanence" => sc.afterglow = x,
                                "courbure" => {
                                    sc.curvature = x > 0.0;
                                    sc.curve = x;
                                }
                                _ => return Err(format!("--screening-options : « {o} » inconnu")),
                            }
                        }
                    }
                }
            }
            "--bibliotheque" | "--library" => cfg.library = PathBuf::from(val("--bibliotheque")?),
            "--basic" => {
                cfg.basic = Some(PathBuf::from(val("--basic")?));
                cfg.basic_g2 = false;
            }
            "--taille" => {
                let v = val("--taille")?;
                let (w, h) = v.split_once(['x', 'X']).ok_or("--taille : LxH")?;
                s.window = Some([w.parse().map_err(|_| "--taille : LxH")?, h.parse().map_err(|_| "--taille : LxH")?]);
            }
            "--cursor-hz" => {
                let v: f64 = val("--cursor-hz")?.replace(',', ".").parse().map_err(|_| "--cursor-hz : nombre attendu")?;
                if v <= 0.0 || v.is_nan() {
                    return Err("--cursor-hz : doit être > 0".into());
                }
                hz = Some(v);
            }
            "--taper" => s.type_text.push((1.0, unescape(&val("--taper")?))),
            "--taper-a" => {
                let v = val("--taper-a")?;
                let (t, txt) = v.split_once(':').ok_or("--taper-a S:TEXTE")?;
                s.type_text.push((t.replace(',', ".").parse().map_err(|_| "--taper-a S:TEXTE")?, unescape(txt)));
            }
            "--carte-couleur" | "--gen2" => cfg.carte_couleur = true,
            "--sans-carte-couleur" => cfg.carte_couleur = false,
            "--plab" | "--carte-plab" => cfg.carte_plab = true,
            "--sans-plab" => cfg.carte_plab = false,
            "--couleur-50hz" => cfg.couleur_50hz = true,
            "--couleur-60hz" => cfg.couleur_50hz = false,
            "--langue" | "--lang" => {
                let v = val("--langue")?.to_lowercase();
                if v != "auto" && i18n::Lang::from_code(&v).is_none() {
                    return Err(format!("--langue : « {v} » (fr, en, de, ru, es, it, pt ou auto)"));
                }
                cfg.langue = v;
            }
            "--mode-rapide" => cfg.cassette_mode = false,
            "--mode-cassette" => cfg.cassette_mode = true,
            "--charger-biblio" | "--lancer-biblio" | "--parametres" => {
                let v = val(&a)?;
                let (t, name) = v.split_once(':').ok_or(format!("{a} S:NOM"))?;
                let t: f64 = t.replace(',', ".").parse().map_err(|_| format!("{a} S:NOM"))?;
                match a.as_str() {
                    "--charger-biblio" => s.lib_load.push((t, name.to_string())),
                    "--lancer-biblio" => s.lib_run.push((t, name.to_string())),
                    _ => s.lib_params = Some((t, name.to_string())),
                }
            }
            "--preferences" => s.preferences = Some(val("--preferences")?.replace(',', ".").parse().map_err(|_| "--preferences : secondes")?),
            "--editeur" | "--editor" => s.editor_files.push(PathBuf::from(val("--editeur")?)),
            "--ascii" => s.ascii = Some(PathBuf::from(val("--ascii")?)),
            "--ascii-reglages" => {
                for kv in val("--ascii-reglages")?.split(';') {
                    let (k, v) = kv.split_once('=').unwrap_or((kv, "1"));
                    s.ascii_settings.push((k.trim().to_string(), v.trim().to_string()));
                }
            }
            "--ascii-lancer" => s.ascii_run = Some(val("--ascii-lancer")?.replace(',', ".").parse().map_err(|_| "--ascii-lancer : secondes")?),
            "--editeur-lancer" => s.editor_run = Some(val("--editeur-lancer")?.replace(',', ".").parse().map_err(|_| "--editeur-lancer : secondes")?),
            "--editeur-exporter" => {
                let v = val(&a)?;
                let (t, f) = v.split_once(':').ok_or("--editeur-exporter S:FICHIER")?;
                let t: f64 = t.replace(',', ".").parse().map_err(|_| "--editeur-exporter S:FICHIER")?;
                s.editor_export = Some((t, PathBuf::from(f)));
            }
            "--debogueur" | "--debugger" => s.debugger = true,
            "--ecran-separe" | "--screen-window" => s.screen_window = true,
            "--onglet-prefs" => s.prefs_tab = Some(app::PrefTab::parse(&val("--onglet-prefs")?).ok_or("--onglet-prefs : interface, editeur, bibliotheque, cassette, gen2, avance")?),
            "--editeur-recherche" => s.editor_find = Some(val("--editeur-recherche")?),
            "--editeur-menu" => s.editor_menu = true,
            "--ecran-menu" => s.screen_menu = true,
            "--menu-etat" => s.state_menu = true,
            "--bienvenue" | "--welcome" => s.welcome = true,
            "--offre-version" => s.fresh = true,
            "--barre-fin" => s.bar_end = true,
            "--assistant" => {
                let n: u8 = val("--assistant")?.parse().map_err(|_| "--assistant : 1, 2 ou 3")?;
                if !(1..=3).contains(&n) {
                    return Err("--assistant : 1, 2 ou 3".into());
                }
                s.welcome = true;
                s.wizard_page = Some(n - 1);
            },
            "--eprom" => s.eprom = true,
            "--fenetre-fichiers" => s.dialog = true,
            "--dbg-arret" => s.dbg_breaks.extend(val(&a)?.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty())),
            "--dbg-surveiller" => s.dbg_watch.extend(val(&a)?.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty())),
            "--dbg-commande" => {
                let v = val(&a)?;
                let (t, c) = v.split_once(':').ok_or("--dbg-commande S:CMD")?;
                let t: f64 = t.replace(',', ".").parse().map_err(|_| "--dbg-commande S:CMD")?;
                s.dbg_cmds.push((t, crate::debugger::Cmd::parse(c.trim()).ok_or(format!("--dbg-commande : {c} ?"))?));
            }
            "--cassette" => s.tapes.push(PathBuf::from(val("--cassette")?)),
            "--capture" => capture = Some(PathBuf::from(val("--capture")?)),
            "--delai" => delay = val("--delai")?.replace(',', ".").parse().map_err(|_| "--delai : secondes")?,
            "--quitter" => s.quit_after_capture = true,
            o if o.starts_with('-') => return Err(format!("option inconnue : {o} (voir --help)")),
            f => s.load.push(PathBuf::from(f)),
        }
    }
    s.capture = capture.map(|p| (delay, p));
    Ok((s, cfg, from_disk, hz))
}

fn main() -> eframe::Result {
    let (script, cfg, from_disk, hz) = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("apple1-emu-gui : {e}");
            std::process::exit(2);
        }
    };
    let detected = i18n::detect();
    if std::env::var_os("APPLE1_DEBUG").is_some() {
        eprintln!("langue détectée : {} ({})", detected.lang.code(), detected.source);
    }
    app::apply_language(&cfg, &detected);
    // cfg.window is in egui points (system scale x interface zoom): logical size for the builder (#39)
    let size = script.window.or(cfg.window.map(|w| dock::to_logical(w, cfg.icon_size.zoom()))).unwrap_or([1100.0, 760.0]);
    let mut vp = egui::ViewportBuilder::default()
        .with_title(WINDOW_TITLE)
        .with_app_id("apple1-emu")
        .with_inner_size(size)
        .with_min_inner_size([480.0, 360.0])
        .with_maximized(script.window.is_none());
    if let Some(i) = icon() {
        vp = vp.with_icon(i);
    }
    let options = eframe::NativeOptions { viewport: vp, ..Default::default() };
    let session = session::Session::new(Config { color_card: cfg.carte_couleur, color_50hz: cfg.couleur_50hz, plab_card: cfg.carte_plab, plab_nmi: cfg.plab_nmi, codetank: cfg.plab_rom.as_ref().and_then(|p| std::fs::read(p).ok()), ..Config::default() });
    eframe::run_native(
        "apple1-emu",
        options,
        Box::new(move |cc| {
            let mut app = App::new(cc, session, cfg, !from_disk, script, detected);
            if let Some(h) = hz {
                app.set_blink(CursorBlink::with_hz(h));
            }
            Ok(Box::new(app))
        }),
    )
}
