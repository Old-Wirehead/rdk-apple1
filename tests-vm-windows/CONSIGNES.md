# Consignes pour la session Claude de la VM Windows : chasse aux bugs

*Écrit par Claude IA (session cloud) pour la session Claude Code qui tourne dans
PowerShell sur la VM Windows 10 de Wirehead, à la demande de Wirehead, le
16/11/2026.*

## 0. Avant tout

- Lis les directives de Wirehead si tu les as, et respecte-les : il parle
  français, il dicte souvent au téléphone, **une seule étape à la fois**,
  messages courts, **formulaire à cases à cocher** pour tout choix, commandes
  avec des chemins absolus, jamais de texte à copier en ligne (toujours un
  bloc de code), signature « Claude IA » sur GitHub.
- **Ne lance aucun travail lourd sans le mot « Go » de Wirehead.** Commence
  par lui présenter ton plan (section 4) en quelques lignes, puis attends son
  Go.
- La VM a déjà fait un écran bleu (0x109) avec l'accélération 3D de
  VirtualBox : ne la réactive pas. Garde le volume bas pendant les cassettes.

## 1. La méthode à reprendre

Une session Claude précédente a mené une campagne de tests de l'émulateur
sous Linux (v7.8 → 7.9.1). Sa méthodologie est dans le dépôt d'archive, dans
une PR qui n'a pas été fusionnée (le fichier n'est donc pas sur la branche
principale) :

https://github.com/Old-Wirehead/rdk-apple1-archive/blob/8cd28162eb52c37d1fb7b9988cce3333ebde464a/docs/tests-claude/README.md

Les scripts sont à côté, dans `docs/tests-claude/scripts-vm/` (pilotage de
l'interface) et `docs/tests-claude/outils-cloud/` (tests sans interface), au
même commit. Ce qu'il faut en garder :

1. **Réglages et bibliothèque temporaires** à chaque lancement : sous
   Windows, la configuration est dans `%APPDATA%\apple1-emu\gui.json` et le
   journal d'erreurs dans `%LOCALAPPDATA%\apple1-emu\gui.log` ; sauf en mode
   portable (variable `APPLE1_PORTABLE`, ou fichier nommé `portable` à côté de
   l'exécutable), où ils sont dans le dossier du kit (`config\`, `cache\`).
   Le mode portable est pratique pour isoler chaque test. Sauvegarde le
   `gui.json` de Wirehead avant de commencer, et remets-le à la fin ; ne
   l'écrase jamais.
2. **Options de démonstration de l'application plutôt que des clics** :
   `--editeur FICHIER`, `--editeur-lancer S`, `--taper TEXTE`,
   `--taper-a S:TEXTE`, `--preferences S`, `--fenetre-fichiers`,
   `--menu-etat`, `--eprom`, `--dbg-arret`, `--dbg-surveiller`,
   `--capture IMAGE.png` après `--delai S`, `--quitter`… La liste complète :
   `apple1-emu-gui --help`, et le chapitre « La ligne de commande » du manuel.
3. **Mesures objectives** : capture d'écran avant/après, texte de l'éditeur
   relu (Ctrl+A, Ctrl+C), tailles relues dans `gui.json`, journal `gui.log`.
4. **Un essai témoin** dans chaque test, pour distinguer un défaut de
   l'application d'un artefact de ton script.
5. **Vérifier la source avant de conclure** (une fausse alerte de la campagne
   précédente venait d'un fichier de test abîmé).

Adaptation à Windows : pas de `xdotool`. Pour les clics et les touches, utilise
PowerShell avec .NET (`System.Windows.Forms.SendKeys`, `user32.dll`
`SetCursorPos`/`mouse_event`) ou UI Automation ; pour les captures,
`System.Drawing` (`CopyFromScreen`), ou mieux l'option `--capture` de
l'application. Une seule commande à faire taper à Wirehead par script, comme
dans la campagne précédente.

## 2. Ce qu'il faut tester

La version Windows de l'**émulateur Apple-1** installée sur la VM (celle avec
le son, artefact de la PR n°6). Note son numéro de version et l'origine du zip
dans chaque issue.

Simule **un vrai humain qui développe** des programmes en **Integer BASIC /
RDK G2 Basic** et en **assembleur 6502**, et qui fait le **maximum d'erreurs
de saisie et d'utilisation**. Le but est de trouver des bugs, pas de réussir
les programmes.

### 2.1 Erreurs de saisie, en BASIC

- Fautes de frappe dans les mots clés (`PIRNT`, `GOTP`, `NETX I`), minuscules,
  guillemet oublié, ligne sans numéro, numéro hors limites (`40000`).
- `:` en mode direct, `FOR` en mode direct, `GOTO` vers une ligne absente.
- Oublier de lancer le BASIC (`E000R`), ou `E000R` au lieu de `E2B3R` après
  un chargement.
- Appuyer sur une touche pendant `RUN`, RESET au milieu, Échap au milieu d'une
  ligne, effacement avec `_`.
- Ligne de plus de 128 caractères, `AUTO` sans en sortir, `LIST` d'un long
  programme pendant que l'affichage est limité.
- Coller (Ctrl+V) un long programme, un programme avec des fins de ligne
  Windows (CRLF), un fichier `.bas` enregistré par le Bloc-notes (UTF-8 avec
  BOM), avec des accents, avec des tabulations.

### 2.2 Erreurs de saisie, en assembleur

- Étiquette sans deux-points, faute de frappe (`jsr ECO`), symbole non défini,
  `.include` oublié ou mal orthographié, saut conditionnel trop loin.
- Instruction 65C02 sans `.setcpu`, constante de page zéro définie après son
  usage, `.org` qui chevauche le BASIC ou la page zéro.
- Programme qui finit par `RTS` ou `BRK` au lieu de `jmp GETLINE`.
- Point d'arrêt sur une étiquette qui n'existe pas encore, pas à pas dans
  `ECHO`, RESET pendant le débogage.

### 2.3 Erreurs d'utilisation de l'environnement

- Le clavier au mauvais endroit (taper dans l'éditeur en croyant taper sur
  l'Apple-1, et l'inverse).
- Fermer tous les onglets de l'éditeur, détacher puis fermer des panneaux, les
  rouvrir, changer de disposition, réduire la fenêtre à 1024 × 768, agrandir,
  plein écran (F11).
- Enregistrer sans nom, « Enregistrer sous » sur un fichier déjà ouvert,
  quitter sans enregistrer.
- Cassette : lecture, arrêt, éjection pendant un chargement ; export WAV avec
  de mauvaises plages ; charger un `.wav` qui n'est pas une cassette ACI.
- Mode rapide / vitesse réelle en plein milieu d'un programme.
- **Spécifique à Windows** : clavier AZERTY (touches mortes `^` `¨`, AltGr
  pour `#` `@` `\` `[` `]` `|`), chemins avec espaces et accents
  (`C:\Users\Prénom Nom\Documents\`), dossier OneDrive, chemin très long,
  dossier en lecture seule, mise à l'échelle de l'affichage 125 % / 150 %,
  deux instances lancées en même temps, Alt+Tab pendant une frappe
  automatique, langue de l'interface changée en cours de route.

## 3. Où ranger les résultats

**Une issue GitHub par bug**, dans `Old-Wirehead/rdk-apple1`, en français,
signée « Claude IA », avec :

- les étapes exactes pour le reproduire, et ce qui était attendu ;
- la preuve (capture, extrait de `gui.log`, texte relu) ;
- la version testée et la configuration de la VM ;
- « Windows seulement » ou « à vérifier sous Linux » (la session cloud peut le
  vérifier sous Linux si Wirehead le demande).

Avant d'ouvrir une issue, vérifie qu'elle n'existe pas déjà : les issues
ouvertes du dépôt, et le fichier `NOTES-PROCHAIN-BUILD.md` de la branche
`claude/notes-build` (notes 1 à 10, déjà connues : panneau de l'éditeur fermé
avec le dernier onglet, fenêtre Dump qui change de taille, dossiers BASIC et
6502 de la bibliothèque, saccades du son pendant le chargement d'une
cassette…).

**Ne corrige rien dans le code** : la campagne trouve et documente ; les
corrections viendront plus tard, après le Go de Wirehead.
