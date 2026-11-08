# Contribuer à Retro Dev Kit Apple 1

Ce dépôt est développé par Wirehead (Old-Wirehead) avec plusieurs IA. Depuis le 8 novembre 2026, **Claude** modifie le code et fusionne lui-même ses demandes de fusion dans `main` ; **Rusty** relit, coordonne et fabrique les versions. Chaque IA lit ce fichier en premier quand elle reprend le travail.

## Rôles

| Qui | Rôle |
|---|---|
| Wirehead | Chef de projet : décide des priorités et de tout ce qui est listé dans « Décisions réservées à Wirehead ». |
| Claude | Mainteneur du code : corrige et fait évoluer le code et la documentation sur des branches `claude/…`, ouvre une demande de fusion, vérifie les tests, puis la fusionne lui-même dans `main`. |
| Rusty | Relit les changements fusionnés, coordonne le travail, fabrique les zips et les brouillons de release. |
| Autres IA | Proposent sur leur branche, via une demande de fusion que Claude relit et fusionne. |

## Décisions réservées à Wirehead

- Les releases et les tags, y compris la publication d'un brouillon de release (aucune IA ne crée de tag ni ne publie de release).
- La licence du projet.
- Le passage du dépôt en public.
- Les suppressions importantes (fonctions, dossiers, pans de documentation) et toute réécriture de l'historique git.
- L'ajout de ROM ou de tout matériel sous droits, y compris celui d'Uncle Bernie.

## Identité et signature

Tant que tout le monde publie avec le compte Old-Wirehead, chaque IA **signe** ses issues, commentaires et demandes de fusion à la fin : « — Claude IA », « — Rusty », etc. Les messages de commit commencent par le nom de l'IA entre crochets : `[Claude] Corrige …`.

## Branches

- `main` : version courante. Seul Claude y fusionne, toujours par une demande de fusion (jamais de poussée directe).
- `<ia>/<sujet>` : une branche par sujet, préfixée par le nom de l'IA en minuscules, par exemple `claude/debogueur-etiquettes` ou `rusty/zip-7.9.4`.
- Une branche par sujet, partie du dernier `main`. Ne pas réécrire l'historique d'une branche déjà relue.

## Demandes de fusion (pull requests)

1. Ouvrir la demande vers `main` depuis sa branche, avec un titre clair et une description : ce qui change, pourquoi, comment c'est testé.
2. Lier les issues concernées (`Corrige #42`).
3. Avant de fusionner, Claude vérifie que tout est propre :
   - `cargo test --workspace` passe (lancer avec `HOME=/tmp/h` pour ne pas être influencé par un `~/apple1/integer_basic.bin` local) ;
   - `cargo clippy --workspace --all-targets` ne donne aucun avertissement ;
   - un changement visible dans l'interface est vérifié par une capture (`--capture`) ;
   - si `docs/src` a changé, les PDF sont reconstruits (`docs/build.sh all`) et commités dans la même demande.
4. Claude fusionne ensuite lui-même. Rusty relit le changement après la fusion et signale ce qui ne va pas par une issue.
5. Les demandes des autres IA sont relues par Claude, qui peut demander des corrections ou reprendre une partie du travail en le signalant.

## Ce que chaque IA peut modifier

- **Code, tests et documentation** : Claude, par demande de fusion, dans le périmètre des issues traitées ou des demandes de Wirehead.
- **Jeux et programmes d'exemple** : dans `contributions/<ia>/basic/` et `contributions/<ia>/asm/`, avec un `README.md`. Ils sont repris dans la bibliothèque par Claude.
- **Interface graphique** : pas de refonte sans accord de Wirehead (maquette d'abord).
- **Zips et brouillons de release** : Rusty.

## Ce qui ne doit jamais être commité

- Aucune ROM, ni l'Integer BASIC original, ni aucun fichier protégé par le droit d'auteur (jeux commerciaux, cassettes audio du commerce, livres, PDF de tiers).
- Aucun document d'Uncle Bernie (documentation préliminaire de la carte GEN2, *Tips & Tricks*), ni résumé ou extrait de ces documents, sauf accord écrit de Wirehead.
- Aucun jeton, mot de passe ni clé.

## Étiquettes

| Étiquette | Sens |
|---|---|
| `bug` | Défaut constaté. |
| `amélioration` | Demande de fonctionnalité. |
| `doc` | Documentation. |
| `à-faire-par-claude`, `à-faire-par-rusty`, … | Qui doit s'en occuper. |
| `en-revue` | Demande de fusion en attente de relecture. |
| `bloqué` | En attente d'une décision de Wirehead ou d'une information extérieure. |

## Discussions

- Catégorie **« Coordination IA »** : répartition du travail, questions entre IA, comptes rendus de session. Mentionner l'IA visée avec `@nom` dans le texte, puisque toutes publient avec le même compte.
- Les défauts et les demandes précises vont dans les **issues**, pas dans les discussions.
- Les décisions de Wirehead priment toujours.

## Langue

Issues, discussions et commentaires en français. Le code et la documentation anglaise restent en anglais. En français, « Apple-1 » est masculin.

## Licence

Le projet est sous GNU GPL 3.0. G2 Basic et les programmes TEST1-4 d'Uncle Bernie restent sous licence MIT. Toute contribution est acceptée sous la GPL 3.0.
