# omz2cc

Oh My Zsh to Claude Code status line formatter. A Rust CLI that outputs a formatted status line matching popular oh-my-zsh themes, designed for use as a command widget in [ccstatusline](https://github.com/sirmalloc/ccstatusline).

## Usage

```sh
# Default theme (ys)
omz2cc

# Specify a theme
omz2cc --theme robbyrussell

# List available themes
omz2cc --list
```

## Supported Themes

| Theme | Format |
|-------|--------|
| `ys` | `# mike @ zulu in ~/project on git:main o [15:35:04]` |
| `robbyrussell` | `➜ project git:(main) ✓` |
| `agnoster` | `mike@zulu ~/project  main ✓` |
| `af-magic` | `~/project on git:main ✓` |
| `bira` | `mike@zulu:~/project on git:main ✓` |
| `bureau` | `mike ~/project [15:35:04] git:main ✓` |
| `candy` | `mike@zulu [15:35:04] [~/project] [main]` |
| `dallas` | `[15:35:04] zulu:~/project @main mike` |
| `gallois` | `~/project git:(main) ✓` |
| `maran` | `mike@zulu ~/project git:(main)` |

## Building

```sh
cargo build --release
```

The binary will be at `target/release/omz2cc`.
