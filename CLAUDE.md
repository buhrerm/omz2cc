# omz2cc

Rust CLI that formats oh-my-zsh-style status lines with ANSI colors for Claude Code's status line.

## Build

```sh
cargo build
```

## Testing

omz2cc is configured directly as Claude Code's status line command:

- Config: `~/.claude/settings.json`
- Command: `/home/mike/Workspace/omz2cc/target/debug/omz2cc --set user=@model`

After building, the status line in Claude Code should update to show the omz2cc output with colors. Verify it looks correct there rather than just checking stdout.

## Themes

142 supported themes (all oh-my-zsh themes).

Theme detection order: `--theme` flag > `$ZSH_THEME` env var > last `ZSH_THEME=` in `~/.zshrc` > fallback `ys`. The .zshrc parsing is needed because Claude Code doesn't inherit the shell's `$ZSH_THEME` env var.

- `--theme <name>` — override with a specific theme
- `--theme random` — pick a random theme
- `--list` — show all available themes

Themes are defined as data in `src/themes/defs.rs` using a template DSL (`src/themes/template.rs`). Complex themes that need logic beyond the DSL can use manual Rust implementations (see `src/themes/agnoster.rs` as an example).

## Field Overrides (--set)

Override any theme field with `--set key=value`. Special values:

- `@model` — pretty model name from Claude Code stdin JSON (e.g. "Opus 4.6")
- `@model-id` — raw model ID from stdin JSON (e.g. "claude-opus-4-6")
- `@time` — current time in default 24h format (HH:MM:SS)
- `@time:FORMAT` — current time with custom strftime format (e.g. `@time:%I:%M %p` for 12-hour)

Stdin JSON from Claude Code is read automatically when piped (no flag needed). `@model` and `@model-id` resolve from it.

Overridable fields: `user`, `hostname`, `cwd`, `git_branch`, `time`.

## Mappings Config

Persistent field mappings can be set in `~/.config/omz2cc/mappings.conf`. Run `omz2cc --init` to scaffold the config. CLI `--set` overrides take precedence over config mappings.

## Development Workflow

**Always use git worktrees for feature work.** Every new feature branch gets its own worktree:

```sh
git worktree add ../omz2cc-<branch-name> -b feature/<branch-name> main
# ... do work in ../omz2cc-<branch-name> ...
git worktree remove ../omz2cc-<branch-name>
```

Do not develop features directly on main.
