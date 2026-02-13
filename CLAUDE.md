# omz2cc

Rust CLI that formats oh-my-zsh-style status lines with ANSI colors for Claude Code's status line.

## Build

```sh
cargo build
```

## Testing

omz2cc is configured directly as Claude Code's status line command:

- Config: `~/.claude/settings.json`
- Command: `/home/mike/Workspace/omz2cc/target/debug/omz2cc --stdin --set user=@model`

After building, the status line in Claude Code should update to show the omz2cc output with colors. Verify it looks correct there rather than just checking stdout.

## Themes

142 supported themes (all oh-my-zsh themes).

By default, omz2cc auto-detects your theme from `ZSH_THEME` in `~/.zshrc` (or `$ZDOTDIR/.zshrc`). Falls back to `ys` if not found.

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

Stdin JSON from Claude Code is read automatically when piped (no flag needed).

Overridable fields: `user`, `hostname`, `cwd`, `git_branch`, `time`.

## Parallel Development

When working on multiple features simultaneously, use `git worktree` to check out separate branches in their own directories:

```sh
git worktree add ../omz2cc-<branch-name> -b feature/<branch-name>
```

Then run a separate Claude Code instance in each worktree directory. This avoids branch conflicts between concurrent sessions. Clean up when done:

```sh
git worktree remove ../omz2cc-<branch-name>
```
