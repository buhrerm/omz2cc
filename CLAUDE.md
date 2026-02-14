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

> **MANDATORY: Use git worktrees for ALL feature work. Never commit feature changes directly on main. No exceptions.**

Every new feature branch must get its own worktree. Do NOT edit files in the main worktree and commit to a feature branch — create the worktree FIRST, then do all work inside it.

```sh
# 1. Create worktree + branch (from the main worktree)
git worktree add ../omz2cc-<branch-name> -b feature/<branch-name> main

# 2. Do ALL work inside the worktree directory
#    Edit files, build, test — everything happens in ../omz2cc-<branch-name>/

# 3. Commit, push, and create PR from the worktree

# 4. Clean up after merge
git worktree remove ../omz2cc-<branch-name>
```

The worktree path pattern is always `../omz2cc-<branch-name>` (sibling to the main checkout).

## Git / GitHub

- `gh` CLI is **not available** — do not attempt to use it
- PRs are created manually by the user; just push the branch
- Push with `git push -u origin <branch>` and report the branch name

## Verify Tool (`src/bin/verify.rs`)

Run: `cargo run --bin verify` (or `cargo run --bin verify -- -v <theme>` for verbose)

Parses reference .zsh-theme files from `~/.oh-my-zsh/themes/`, simulates output, compares against defs.rs.

### Current state (feature/fix-themes2, worktree ../omz2cc-fix-themes2)

103/142 PASS, 38 COMPLEX, 1 NOT_FOUND (random). Goal: get all 38 COMPLEX to PASS.

### The COMPLEX gate (`detect_complexity`, ~line 495)

`detect_complexity()` runs BEFORE parsing. If it returns true, the theme is skipped entirely. The gate has:
- Generic checks: precmd+PROMPT-in-function, vcs_info, custom git functions
- 16 hardcoded theme names in a `match` block (~line 543) that bail out early

**To fix themes: remove them from the hardcoded block and fix whatever parser gap caused them to be added.**

### 38 COMPLEX themes by fix needed

**Remove hardcoded override (parser can already handle or needs small fix):**
blinks, essembeh, dallas, funky, humza, mira, nebirhos, mlh, darkblood, jonathan, adben, junkfood, michelebologna, kardan, sunaku, rkj-repos

**Add vcs_info simulation (10 themes):**
apple, emotty, gentoo, half-life, jnrowe, kolo, mikeh, nicoulaj, steeef, zhann
→ Simulate `${vcs_info_msg_0_}` in resolve_calls like git_prompt_info

**Handle custom git functions (6 themes):**
eastwood, gallois, mortalscumbag, oldgallois, peepcode, sunrise
→ Simulate `$(git_custom_status)`, `$(git_prompt)`, `$(mygit)` in resolve_calls

**Handle precmd (5 themes, hardest):**
bureau, linuxonly, pygmalion-virtualenv, refined, simonoff, trapd00r
→ Need to trace PROMPT through precmd function body

### Parsing pipeline

`preprocess → detect_complexity (GATE) → collect_vars → resolve_vars → strip_formatting → resolve_zsh_escapes → resolve_calls → normalize`

### Test values

user="user", host="host", cwd="~/test", branch="main", time="12:00:00"

### Component extraction TODO

Extract repeated defs.rs patterns to components.rs (box-drawing lines, user@host, git blocks).
