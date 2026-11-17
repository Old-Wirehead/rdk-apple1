//! Retro Dev Kit Apple 1 2.0 : RDK-Interface-Commune 0.1.4 avec l'Apple 1.
//!
//! Le dossier courant est le « dossier du kit » : bibliothèque `./Biblio`,
//! préférences `./preferences.json`, BASIC de l'utilisateur
//! `./integer_basic.bin` (non fourni). Le script `run.sh` du paquet se
//! place dans son dossier avant de lancer ce programme.

use apple1_rdk::MachineApple1;
use rdk_interface::preferences::dossier_kit;

fn main() -> eframe::Result {
    rdk_interface::lancer(Box::new(MachineApple1::new(dossier_kit())))
}
