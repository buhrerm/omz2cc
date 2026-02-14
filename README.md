# omz2cc

Your [oh-my-zsh](https://ohmyz.sh/) theme, right in [Claude Code](https://docs.anthropic.com/en/docs/claude-code).

omz2cc renders oh-my-zsh prompt themes as ANSI-colored status lines for Claude Code. All **142 built-in oh-my-zsh themes** are supported — whatever theme you use in your terminal, you can use it in Claude Code too.

### Theme Previews

| Theme | Preview |
|-------|---------|
| `robbyrussell` | `➜ project git:(main) ✓` |
| `agnoster` | `mike@zulu ~/project  main ✓` |
| `ys` | `# Opus 4.6 @ zulu in ~/project on git:main o [15:35:04]` |
| `af-magic` | `~/project on git:main ✓` |
| `bira` | `mike@zulu:~/project on git:main ✓` |
| `lambda` | `λ ~/project git:(main) ✓` |
| `candy` | `mike@zulu [15:35:04] [~/project] [main]` |
| `bureau` | `mike ~/project [15:35:04] git:main ✓` |
| `dallas` | `[15:35:04] zulu:~/project @main mike` |
| `gallois` | `~/project git:(main) ✓` |

## Install

```sh
npm install -g omz2cc
```

Or build from source with Rust:

```sh
cargo install --path .
```

## Setup

Add this to `~/.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "omz2cc --set user=@model",
    "padding": 0
  }
}
```

That's it. Claude Code will now show your oh-my-zsh theme in the status bar, with the current model name (e.g. "Opus 4.6") in place of your username.

## Usage

```sh
# Auto-detects your theme from $ZSH_THEME or ~/.zshrc
omz2cc

# Pick a specific theme
omz2cc --theme robbyrussell

# Random theme each time
omz2cc --theme random

# List all 142 themes
omz2cc --list
```

### Theme Detection

Your theme is resolved automatically in this order:

1. `--theme <name>` flag (explicit override)
2. `$ZSH_THEME` environment variable
3. Last `ZSH_THEME=` assignment in `~/.zshrc`
4. Fallback: `ys`

## Field Overrides

Override any field displayed in the theme with `--set key=value`:

```sh
# Show the Claude model name as the user field
omz2cc --set user=@model

# Multiple overrides
omz2cc --set user=Claude --set hostname=AI

# Custom time format (12-hour)
omz2cc --set time=@time:%I:%M\ %p
```

### Available Fields

| Field | Default | Description |
|-------|---------|-------------|
| `user` | `$USER` | Username |
| `hostname` | System hostname | Hostname |
| `cwd` | Current directory | Working directory (`~` abbreviated) |
| `git_branch` | Current branch | Git branch name |
| `time` | `HH:MM:SS` | Current time |

### Special Values

| Value | Description | Example |
|-------|-------------|---------|
| `@model` | Pretty model name from Claude Code | `Opus 4.6` |
| `@model-id` | Raw model ID | `claude-opus-4-6` |
| `@time` | Current time (24h) | `15:35:04` |
| `@time:FORMAT` | Custom strftime format | `03:35 PM` |

## Persistent Config

For mappings you want to keep across sessions, use a config file instead of repeating `--set` flags:

```sh
# Create the config file
omz2cc --init

# Edit ~/.config/omz2cc/mappings.conf
```

CLI `--set` flags take precedence over the config file.

## Options

| Flag | Description |
|------|-------------|
| `--theme <name>` | Use a specific theme (or `random`) |
| `--list` | List all available themes |
| `--set <KEY=VALUE>` | Override a field (repeatable) |
| `--init` | Create default mappings config |
| `--no-stdin` | Skip reading JSON from stdin |

## All 142 Themes

<details>
<summary>Click to expand full theme list</summary>

`3den` `Soliah` `adben` `af-magic` `afowler` `agnoster` `alanpeabody` `amuse` `apple` `arrow` `aussiegeek` `avit` `awesomepanda` `bira` `blinks` `bureau` `candy` `candy-kingdom` `clean` `cloud` `crcandy` `crunch` `cypher` `dallas` `darkblood` `daveverwer` `dieter` `dogenpunk` `dpoggi` `dst` `dstufft` `duellj` `eastwood` `edvardm` `emotty` `essembeh` `evan` `fino` `fino-time` `fishy` `flazz` `fletcherm` `fox` `frisk` `frontcube` `funky` `fwalch` `gallifrey` `gallois` `garyblessington` `gentoo` `geoffgarside` `gianu` `gnzh` `gozilla` `half-life` `humza` `imajes` `intheloop` `itchy` `jaischeema` `jbergantine` `jispwoso` `jnrowe` `jonathan` `josh` `jreese` `jtriley` `juanghurtado` `junkfood` `kafeitu` `kardan` `kennethreitz` `kiwi` `kolo` `kphoen` `lambda` `linuxonly` `lukerandall` `macovsky` `macovsky-ruby` `maran` `mgutz` `mh` `michelebologna` `mikeh` `miloshadzic` `minimal` `mira` `mlh` `mortalscumbag` `mrtazz` `murilasso` `muse` `nanotech` `nebirhos` `nicoulaj` `norm` `obraun` `oldgallois` `peepcode` `philips` `pmcgee` `pygmalion` `pygmalion-virtualenv` `re5et` `refined` `rgm` `risto` `rixius` `rkj` `rkj-repos` `robbyrussell` `sammy` `simonoff` `simple` `skaro` `smt` `sonicradish` `sorin` `sporty_256` `steeef` `strug` `sunaku` `sunrise` `superjarin` `suvash` `takashiyoshida` `terminalparty` `theunraveler` `tjkirch` `tjkirch_mod` `tonotdo` `trapd00r` `wedisagree` `wezm` `wezm+` `wuffers` `xiong-chiamiov` `xiong-chiamiov-plus` `ys` `zhann`

</details>

## License

MIT

