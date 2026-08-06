# ZTF (Zed Theme Fixer)
Anciennement `theme-fixer`

Un petit outil CLI en Rust pour corriger un thème [Zed](https://zed.dev) incomplet, en s'appuyant sur un thème de référence valide.

## Le problème

Un thème Zed personnalisé peut facilement se retrouver avec des tokens manquants (`border.disabled`, `terminal.ansi.red`, etc.) — souvent parce qu'ils ont été oubliés lors de la création manuelle du fichier. Plutôt que de les définir un par un à la main, `theme-fixer` déduit les valeurs manquantes à partir d'un thème source qui, lui, les définit tous.

## Comment ça marche

1. Le thème **source** (un thème officiel Zed, complet) est analysé : les tokens qui partagent la même couleur sont regroupés ensemble.
2. Le thème **destination** (celui à corriger) est parcouru groupe par groupe : si au moins un token du groupe a déjà une couleur définie dans la destination, cette couleur sert d'ancre et est appliquée aux autres tokens du même groupe qui existent dans le fichier mais n'ont pas de valeur.
3. Un backup du fichier original est créé avant toute écriture (si la sortie remplace la destination).
4. Le thème corrigé est écrit sur disque.

## Installation

### Binaire précompilé (recommandé)

**Linux / macOS**

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/reeves-48777/ztf/releases/latest/download/theme-fixer-installer.sh | sh
```

**Windows (PowerShell)**

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/reeves-48777/ztf/releases/latest/download/theme-fixer-installer.ps1 | iex"
```

Les binaires précompilés pour Linux, macOS (Intel + Apple Silicon) et Windows sont aussi disponibles directement sur la [page des Releases](https://github.com/reeves-48777/theme-fixer/releases).

### Depuis les sources

```bash
git clone https://github.com/reeves-48777/ztf
cd ztf
cargo build --release
```

Le binaire est disponible dans `target/release/ztf`.

## Usage

```bash
ztf --src <theme_source.json> --dst <theme_a_corriger.json> [--output <chemin_de_sortie.json>]
```

| Flag | Alias court | Description |
|---|---|---|
| `--src` | `-s` | Thème source valide (fait par l'équipe Zed), sert de référence pour déduire les couleurs manquantes |
| `--dst` | `-d` | Thème à corriger, peut contenir des tokens manquants |
| `--output` | `-o` | Chemin de sortie (optionnel — par défaut, écrase le fichier `--dst` après en avoir fait un backup `.bak`) |

### Exemple

```bash
ztf --src assets/one-dark.json --dst assets/mon-theme.json
```

Corrige `mon-theme.json` en place, en s'appuyant sur les groupes de couleurs déduits de `one-dark.json`. Un backup `mon-theme.json.bak` est créé avant l'écriture.

```bash
ztf --src assets/one-dark.json --dst assets/mon-theme.json --output assets/mon-theme.fixed.json
```

Écrit le résultat dans un nouveau fichier, sans toucher à `mon-theme.json`.

### Logs

Le niveau de log est contrôlable via la variable d'environnement `RUST_LOG` :

```bash
RUST_LOG=debug ztf --src assets/one-dark.json --dst assets/mon-theme.json
```

## Limites connues

- Un token qui n'appartient à aucun groupe partagé (couleur unique dans le thème source) ne peut pas être déduit automatiquement — il reste tel quel dans la destination.
- L'outil ne devine pas de nouvelles couleurs : il ne fait que propager des couleurs déjà présentes ailleurs dans le fichier de destination.
