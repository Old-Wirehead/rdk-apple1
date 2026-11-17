//! Art ASCII de la v1.0 (crate `apple1-core::asciiart`) : un dessin `.txt`
//! (dossier « Art ASCII ») ou une photo `.png`/`.jpg` de la bibliothèque est
//! converti en programme 6502 (`300R`) qui l'affiche sur le terminal de
//! l'Apple 1. Le panneau de réglages Photo → ASCII de la v1.0 n'existe pas
//! dans l'interface commune : la conversion utilise les réglages par défaut
//! de la v1.0.

use apple1_core::asciiart::{self, Art, Grey, Params};
use std::path::Path;

/// Dossier de la bibliothèque des dessins (même nom qu'en v1.0).
pub const DOSSIER: &str = "Art ASCII";

/// Extension d'une photo convertie (comme `ascii::is_image` de la v1.0).
pub fn est_image(nom: &str) -> bool {
    matches!(Path::new(nom).extension().and_then(|e| e.to_str()).map(|e| e.to_ascii_lowercase()).as_deref(), Some("jpg" | "jpeg" | "png"))
}

/// Même heuristique que `ascii::looks_art` de la v1.0 : au plus 24 lignes de
/// 40 colonnes en ASCII, dans un dossier « ascii », ou au moins 40 % de symboles.
pub fn ressemble_art(t: &str, chemin: &Path) -> bool {
    let lignes: Vec<&str> = t.lines().map(str::trim_end).collect();
    let n = lignes.iter().rposition(|l| !l.is_empty()).map(|i| i + 1).unwrap_or(0);
    if n == 0 || n > asciiart::ROWS || lignes.iter().any(|l| l.chars().count() > asciiart::COLS) || !t.is_ascii() {
        return false;
    }
    let dans_dossier = chemin.parent().and_then(|p| p.file_name()).map(|n| n.to_string_lossy().to_ascii_lowercase().contains("ascii")).unwrap_or(false);
    let encre: Vec<char> = t.chars().filter(|c| !c.is_whitespace()).collect();
    let symboles = encre.iter().filter(|c| !c.is_ascii_alphanumeric()).count();
    dans_dossier || symboles * 10 >= encre.len() * 4
}

/// Le dessin d'une photo (réglages par défaut de la v1.0).
pub fn depuis_image(octets: &[u8]) -> Result<Art, String> {
    let img = image::load_from_memory(octets).map_err(|e| e.to_string())?;
    let rgba = img.to_rgba8();
    let gris = Grey::from_rgba(rgba.width() as usize, rgba.height() as usize, rgba.as_raw(), 4);
    Ok(asciiart::convert(&gris, &Params::default()))
}

/// Source 6502 qui affiche le dessin (`300R`).
pub fn source(art: &Art, titre: &str) -> String {
    asciiart::to_asm(art, titre)
}
