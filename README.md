# omz2cc

Oh My Zsh to Claude Code status line. A Rust CLI that outputs a formatted, colored status line matching popular oh-my-zsh themes — designed for use as a command widget in [ccstatusline](https://github.com/sirmalloc/ccstatusline).

Map theme fields to Claude Code data (like the current model name) using `--set` overrides with `--stdin` to read the Claude Code session JSON.

## Install

```sh
cargo build --release
# Binary at target/release/omz2cc
```

## Usage

```sh
# Default theme (ys)
omz2cc

# Pick a theme
omz2cc --theme robbyrussell

# List available themes
omz2cc --list
```

### With ccstatusline

Add omz2cc as a custom-command widget in your `~/.config/ccstatusline/settings.json`:

```json
{
  "id": "omz2cc",
  "type": "custom-command",
  "commandPath": "/path/to/omz2cc --theme ys --stdin --set user=@model",
  "preserveColors": true
}
```

- `--stdin` reads the Claude Code JSON piped through ccstatusline
- `--set user=@model` maps the theme's "user" slot to the current model name (e.g. "Opus 4.6")
- `preserveColors: true` tells ccstatusline to pass through ANSI color codes

### Field Overrides

Override any theme field with `--set key=value`:

```sh
# Show model name instead of username
omz2cc --theme ys --stdin --set user=@model

# Show raw model ID instead
omz2cc --theme ys --stdin --set user=@model-id

# Override with literal text
omz2cc --theme ys --set user=Claude --set hostname=AI
```

Special `@` values (require `--stdin`):
- `@model` — pretty model name (e.g. "Opus 4.6", "Sonnet 4.5")
- `@model-id` — raw model ID (e.g. "claude-opus-4-6")

Overridable fields: `user`, `hostname`, `cwd`, `git_branch`, `time`

## Supported Themes

| Theme | Example Output |
|-------|---------------|
| `ys` | `# Opus 4.6 @ zulu in ~/project on git:main o [15:35:04]` |
| `robbyrussell` | `➜ project git:(main) ✓` |
| `agnoster` | `mike@zulu ~/project  main ✓` |
| `af-magic` | `~/project on git:main ✓` |
| `bira` | `mike@zulu:~/project on git:main ✓` |
| `bureau` | `mike ~/project [15:35:04] git:main ✓` |
| `candy` | `mike@zulu [15:35:04] [~/project] [main]` |
| `dallas` | `[15:35:04] zulu:~/project @main mike` |
| `gallois` | `~/project git:(main) ✓` |
| `maran` | `mike@zulu ~/project git:(main)` |

All themes include ANSI colors matching their oh-my-zsh counterparts.
