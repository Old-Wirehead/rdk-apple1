# Journal de test — Kimi Testeur

## 17 novembre 2026 — version Linux 2.0

| Horaires Europe/Paris | Action | Temps passé |
|---|---|---:|
| 16:47–16:52 | Vérification du tag `v2.0` et clone en état détaché sur le commit `46c2478a1e405e57d78ea2155d1a60bbabb940ff`. | 5 min |
| 16:52–16:57 | Lecture de `AGENTS.md`, `CONTRIBUTING.md`, `docs/version-2.0.md`, du banc `crates/apple1-rdk/tests/bout_en_bout.rs` et du script officiel de paquet Linux. | 5 min |

**Total actuel : environ 10 minutes.** Les jetons et le coût API ne sont pas mesurés par les outils de cette session ; aucune valeur n’est inventée.

Périmètre : le programme `retro-dev-kit-apple1`, paquet `apple1-rdk` version `2.0.0`, sur Linux x86_64. La v1.0 n’est pas la cible, sauf essai témoin nécessaire à l’interprétation d’un défaut.

Limite : aucune release GitHub `v2.0` n’existe au 17 novembre 2026. Le test portera sur les sources du tag et sur le paquet produit par `packaging/linux-2.0/construire-paquet.sh`, et non sur un binaire publié.

— Kimi Testeur IA

## Accès à l’interface commune et reprise

| Horaires Europe/Paris | Action | Temps passé |
|---|---|---:|
| 16:57–16:58 | Premier `cargo test --locked` interrompu : Cargo ne peut pas authentifier la récupération de `RDK-Interface-Commune`. Arrêt immédiat, aucun test exécuté. | 1 min |
| 17:12–17:14 | Après le nouveau Go : révision vérifiée par l’API GitHub, puis nouvel échec d’authentification avec le client Git sous le `HOME` isolé. Nouvel arrêt. | 2 min |
| 17:24–17:25 | Avec l’autorisation explicite de Wirehead : `cargo fetch --locked` sous le compte GitHub déjà configuré. Succès en 11 s ; dépendances récupérées sans lancer le produit. | 1 min |

Ces échecs d’accès sont des artefacts de préparation, pas des défauts du produit. Ils ne donnent lieu à aucune issue.

— Kimi Testeur IA

## Banc automatique — 17 novembre 2026

| Horaires Europe/Paris | Action | Temps passé |
|---|---|---:|
| 17:24–17:41 | `cargo test --locked --offline -- --nocapture` dans `crates/apple1-rdk`, sous `HOME` isolé et sans `RDK_APPLE1_BASIC`. Compilation 16 min 10 s ; 20 tests unitaires et 30 tests de bout en bout réussis, 0 échec, 0 ignoré. | 17 min |

Preuve : `preuves/04-cargo-test-apple1-rdk-isole.log`, code de sortie 0, fin à 17:41:08.

**Non vérifié par ce banc :** l’exécution avec le BASIC d’origine. Le test `basic_lancer_avec_la_copie_de_l_utilisateur` retourne sans agir lorsque `RDK_APPLE1_BASIC` est absent, tout en étant compté « ok » par Rust. Il n’est donc pas une preuve de cette configuration.

— Kimi Testeur IA

## Paquet, essais graphiques et issue — 17 novembre 2026

| Horaires Europe/Paris | Action | Temps passé |
|---|---|---:|
| 17:41–18:00 | Construction release et du zip par `packaging/linux-2.0/construire-paquet.sh`, hors ligne. Succès en 18 min 15 s. | 19 min |
| 18:00–18:03 | Vérification SHA-256, `unzip -t`, modes, dépendances ELF et extraction dans un dossier séparé. | 3 min |
| 18:03–18:20 | Premier lancement sous Xephyr, consentement bibliothèque, clavier réel avec témoins, listing invalide, BASIC G2 valide et copie des exemples sans remplacement. | 17 min |
| 18:20–18:35 | Sauvegardes : création de dossier, chemin avec espace, refus d’écrasement ; confirmation du défaut `4A.FF` et publication de l’issue #18. | 15 min |
| 18:35–18:42 | Double clic sur ▶ Lancer, vérification de l’issue et rédaction du rapport de campagne. | 7 min |

**Durée totale de la session : environ 2 heures**, dont environ 34 minutes de compilation. Jetons et coût API non mesurés.

Défaut publié : https://github.com/Old-Wirehead/rdk-apple1/issues/18 — la saisie `4A.FF` exporte 182 octets de `$004A` à `$00FF`, alors que l’ACI la comprend comme `$004A-$4AFF`.

Les anomalies de `xdotool`, les changements de focus trop rapides, `/tmp` et l’absence de gestionnaire de fenêtres sont documentés comme artefacts ou limites, pas comme bugs du produit. Les chemins accentués, le matériel physique, Wayland, Windows et plusieurs autres fonctions restent explicitement non vérifiés dans le rapport `IA/kimi-testeur/campagnes/rdk-apple1-linux-2.0.md`.

— Kimi Testeur IA

## Compléments du référentiel révisé — 17 novembre 2026

| Horaires Europe/Paris | Action | Temps passé |
|---|---|---:|
| 18:42–18:48 | Lecture de la méthode révisée, des issues #17 et #12, puis second banc complet avec la copie confidentielle du BASIC : 50/50 réussis. | 6 min |
| 18:49–19:04 | Adaptation des bancs `basic`, `taille`, `biblio` et `erreurs` à mes chemins et compilation release. Succès en 14 min 48 s ; ca65/ld65 absents, donc comparaison assembleur non faite. | 15 min |
| 19:04–19:08 | Mesures BASIC et cassettes, témoin ligne zéro et Aventure ; publication du commentaire de confirmation dans #17. | 4 min |
| 19:08–19:11 | Récupération des dépendances manquantes du workspace puis tests `apple1-core` hors ligne : 120 réussis, 0 échec, Dormann compris. | 3 min |
| 19:11–19:13 | Mise à jour du rapport avec les 120 tests, les mesures différées et les fixtures non vérifiées. | 2 min |

**Durée totale de session : environ 2 h 35**, dont environ 52 minutes de compilation. Le commentaire de #17 est https://github.com/Old-Wirehead/rdk-apple1/issues/17#issuecomment-6956726073.

— Kimi Testeur IA

## Publication — 17 novembre 2026

| Horaires Europe/Paris | Action | Temps passé |
|---|---|---:|
| 19:13–19:16 | Vérification des seuls fichiers documentaires, création de la branche `kimi/test-linux-2.0-journal`, commit du journal, poussée et ouverture de la PR #19. | 3 min |

Lien : https://github.com/Old-Wirehead/rdk-apple1/pull/19

— Kimi Testeur IA
