# omz2cc

Bring your oh-my-zsh theme to Claude Code. A fast Rust CLI that renders oh-my-zsh prompt themes as colored status lines, with 142 themes built in.

## Quickstart

### Claude Code Status Line

Add to `~/.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "/path/to/omz2cc --set user=@model",
    "padding": 0
  }
}
```

This renders your oh-my-zsh theme in the Claude Code status bar, with the model name (e.g. "Opus 4.6") shown as the user field.

### With ccstatusline

Add omz2cc as a custom-command widget in `~/.config/ccstatusline/settings.json`:

```json
{
  "id": "omz2cc",
  "type": "custom-command",
  "commandPath": "/path/to/omz2cc --set user=@model",
  "preserveColors": true
}
```

## Install

```sh
cargo build --release
# Binary at target/release/omz2cc
```

## Usage

```sh
# Auto-detects theme from $ZSH_THEME or ~/.zshrc
omz2cc

# Pick a specific theme
omz2cc --theme robbyrussell

# Random theme
omz2cc --theme random

# List all 142 themes
omz2cc --list

# Override fields
omz2cc --set user=Claude --set hostname=AI

# Initialize persistent mappings config
omz2cc --init
```

### Options

| Flag | Description |
|------|-------------|
| `--theme <name>` | Use a specific theme (default: auto-detect) |
| `--list` | List all available themes |
| `--set <KEY=VALUE>` | Override a field (repeatable) |
| `--init` | Create `~/.config/omz2cc/mappings.conf` with defaults |

### Theme Detection

Themes are resolved in this order:

1. `--theme <name>` flag
2. `$ZSH_THEME` environment variable
3. Last `ZSH_THEME=` assignment in `~/.zshrc`
4. Fallback: `ys`

### Stdin

When stdin is piped (e.g. from Claude Code or ccstatusline), omz2cc automatically reads JSON to provide model context for `@model` and `@model-id` values. No flag needed.

## Field Overrides

Override theme fields with `--set key=value`. For persistent mappings, use `~/.config/omz2cc/mappings.conf` (run `omz2cc --init` to create it). CLI `--set` takes precedence over config file mappings.

### Fields

| Field | Default Value | Description |
|-------|---------------|-------------|
| `user` | `$USER` | Username displayed in prompt |
| `hostname` | System hostname | Full hostname |
| `cwd` | Current directory | Working directory (~ abbreviated) |
| `git_branch` | Current git branch | Git branch name |
| `time` | `HH:MM:SS` | Current time |

### Values

Fields can be set to plain text or special `@` tokens:

| Value | Description | Example Output |
|-------|-------------|----------------|
| `@model` | Pretty model name from Claude Code | `Opus 4.6`, `Sonnet 4.5` |
| `@model-id` | Raw model ID from Claude Code | `claude-opus-4-6` |
| `@time` | Current time (24h) | `15:35:04` |
| `@time:FORMAT` | Custom strftime format | `@time:%I:%M %p` → `03:35 PM` |
| Any text | Literal string | `Claude`, `myhost` |

### Examples

```sh
# Show model name as user
omz2cc --set user=@model

# Custom time format
omz2cc --set time=@time:%I:%M\ %p

# Multiple overrides
omz2cc --set user=Claude --set hostname=AI
```

## Supported Themes (142)

All 142 oh-my-zsh themes are supported with matching ANSI colors.

<details>
<summary>Full theme list</summary>

`3den` `Soliah` `adben` `af-magic` `afowler` `agnoster` `alanpeabody` `amuse` `apple` `arrow` `aussiegeek` `avit` `awesomepanda` `bira` `blinks` `bureau` `candy` `candy-kingdom` `clean` `cloud` `crcandy` `crunch` `cypher` `dallas` `darkblood` `daveverwer` `dieter` `dogenpunk` `dpoggi` `dst` `dstufft` `duellj` `eastwood` `edvardm` `emotty` `essembeh` `evan` `fino` `fino-time` `fishy` `flazz` `fletcherm` `fox` `frisk` `frontcube` `funky` `fwalch` `gallifrey` `gallois` `garyblessington` `gentoo` `geoffgarside` `gianu` `gnzh` `gozilla` `half-life` `humza` `imajes` `intheloop` `itchy` `jaischeema` `jbergantine` `jispwoso` `jnrowe` `jonathan` `josh` `jreese` `jtriley` `juanghurtado` `junkfood` `kafeitu` `kardan` `kennethreitz` `kiwi` `kolo` `kphoen` `lambda` `linuxonly` `lukerandall` `macovsky` `macovsky-ruby` `maran` `mgutz` `mh` `michelebologna` `mikeh` `miloshadzic` `minimal` `mira` `mlh` `mortalscumbag` `mrtazz` `murilasso` `muse` `nanotech` `nebirhos` `nicoulaj` `norm` `obraun` `oldgallois` `peepcode` `philips` `pmcgee` `pygmalion` `pygmalion-virtualenv` `re5et` `refined` `rgm` `risto` `rixius` `rkj` `rkj-repos` `robbyrussell` `sammy` `simonoff` `simple` `skaro` `smt` `sonicradish` `sorin` `sporty_256` `steeef` `strug` `sunaku` `sunrise` `superjarin` `suvash` `takashiyoshida` `terminalparty` `theunraveler` `tjkirch` `tjkirch_mod` `tonotdo` `trapd00r` `wedisagree` `wezm` `wezm+` `wuffers` `xiong-chiamiov` `xiong-chiamiov-plus` `ys` `zhann`

</details>

### Theme Previews

| Theme | Example |
|-------|---------|
| `ys` | `# Opus 4.6 @ zulu in ~/project on git:main o [15:35:04]` |
| `robbyrussell` | `➜ project git:(main) ✓` |
| `agnoster` | `mike@zulu ~/project  main ✓` |
| `af-magic` | `~/project on git:main ✓` |
| `bira` | `mike@zulu:~/project on git:main ✓` |
| `bureau` | `mike ~/project [15:35:04] git:main ✓` |
| `candy` | `mike@zulu [15:35:04] [~/project] [main]` |
| `dallas` | `[15:35:04] zulu:~/project @main mike` |
| `gallois` | `~/project git:(main) ✓` |
| `lambda` | `λ ~/project git:(main) ✓` |
