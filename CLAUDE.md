# omz2cc

Rust CLI that formats oh-my-zsh-style status lines with ANSI colors for ccstatusline.

## Build

```sh
cargo build
```

## Testing

To test visually, the omz2cc command is configured as the **second line** in ccstatusline:

- Config: `~/.config/ccstatusline/settings.json`
- Line 2 runs: `/home/mike/Workspace/omz2cc/target/debug/omz2cc --theme ys --stdin --set user=@model`
- The widget must have `"preserveColors": true` so ccstatusline passes through ANSI color codes from omz2cc instead of stripping them.

After building, the status line in Claude Code should update to show the omz2cc output with colors. Verify it looks correct there rather than just checking stdout.

## Themes

142 supported themes (all oh-my-zsh themes). Use `--theme <name>` to select, `--list` to show all, `--theme random` for a random theme.

Themes are defined as data in `src/themes/defs.rs` using a template DSL (`src/themes/template.rs`). Complex themes that need logic beyond the DSL can use manual Rust implementations (see `src/themes/agnoster.rs` as an example).

## Field Overrides (--set)

Override any theme field with `--set key=value`. Special values:

- `@model` — pretty model name from Claude Code stdin JSON (e.g. "Opus 4.6")
- `@model-id` — raw model ID from stdin JSON (e.g. "claude-opus-4-6")
- `@time` — current time in default 24h format (HH:MM:SS)
- `@time:FORMAT` — current time with custom strftime format (e.g. `@time:%I:%M %p` for 12-hour)

Requires `--stdin` flag when using `@model` or `@model-id` (reads Claude Code JSON from stdin).

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
