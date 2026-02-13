# TODO

## Mapping Layer Vision

omz2cc should be a **mapping layer** that makes it trivial to take any oh-my-zsh theme and remap its fields to Claude Code concepts. The theme defines the layout/style, the mappings define what data fills each slot.

For example, the `ys` theme has slots: user, host, cwd, git, time. A mapping config could remap:
- `user` -> Claude Code model name
- `host` -> project name
- `cwd` -> working directory (keep as-is)
- `git` -> git info (keep as-is)
- `time` -> session cost, context %, or any custom value

This way adding a new theme is just defining its format — the data sources are configured separately.

### Future Ideas
- Config file (`~/.config/omz2cc/config.toml`) for persistent mappings
- Remap any field to: env var, command output, static text, or built-in source
- Theme gallery / preview command
