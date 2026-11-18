//! Retro Dev Kit Apple 1 2.0 : RDK-Interface-Commune 0.1.9 avec l'Apple 1.
//!
//! Le dossier courant est le « dossier du kit » : préférences
//! `./preferences.json`, exemples du paquet `./Biblio`. La bibliothèque de
//! l'utilisateur est `~/Documents/RDK librairies/Apple 1` (0.1.5) ; le
//! BASIC d'origine de l'utilisateur (`integer_basic.bin`, non fourni) se
//! choisit dans le dossier des ROM de la partie commune. Le script `run.sh`
//! du paquet se place dans son dossier avant de lancer ce programme.
//!
//! La machine reçoit le contexte egui au lancement (`preparer`) : elle y
//! dessine ses fenêtres de la v1.0 (programmateur d'EPROM, Photo → ASCII,
//! Écran, Sauver, ⚙ Réglages du programme, 🐙 Projets GitHub, oscilloscope).

use apple1_rdk::MachineApple1;
use rdk_interface::preferences::{dossier_kit, FichiersJson};
use rdk_interface::OptionsLancement;

fn main() -> eframe::Result {
    let machine = MachineApple1::new(dossier_kit()).avec_bibliotheque(|| {
        let mut f = FichiersJson::charger_par_defaut();
        f.dossier_machine = apple1_rdk::DOSSIER_BIBLIOTHEQUE.into();
        f.dossier_bibliotheque()
    });
    let contexte = machine.contexte();
    let preparer: rdk_interface::Preparation = Box::new(move |ctx| {
        let _ = contexte.set(ctx.clone());
    });
    rdk_interface::lancer_avec(
        Box::new(machine),
        OptionsLancement {
            preparer: Some(preparer),
            ..Default::default()
        },
    )
}
