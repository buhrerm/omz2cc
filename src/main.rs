mod color;
mod config;
mod info;
mod themes;

use std::io::IsTerminal;

use clap::Parser;
use config::{load_mappings, merge_mappings};
use info::{StdinData, StatusInfo};
use themes::{all_themes, get_theme};

#[derive(Parser)]
#[command(name = "omz2cc", about = "Oh My Zsh to Claude Code status line")]
struct Cli {
    /// Theme name (default: auto-detect from $ZSH_THEME or ~/.zshrc, fallback "ys")
    #[arg(short, long)]
    theme: Option<String>,

    /// List available themes
    #[arg(short, long)]
    list: bool,

    /// Override a field: --set user=@model --set hostname=myhost
    #[arg(short, long = "set", value_name = "KEY=VALUE")]
    set: Vec<String>,

    /// Skip reading JSON from stdin (stdin is read automatically when piped)
    #[arg(long)]
    no_stdin: bool,

    /// Initialize default mappings config at ~/.config/omz2cc/mappings.conf
    #[arg(long)]
    init: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.init {
        init_config();
        return;
    }

    if cli.list {
        for theme in all_themes() {
            println!("{}", theme.name());
        }
        return;
    }

    let theme_name = cli
        .theme
        .or_else(|| std::env::var("ZSH_THEME").ok().filter(|s| !s.is_empty()))
        .or_else(detect_theme_from_zshrc)
        .unwrap_or_else(|| "ys".to_string());

    let theme = match get_theme(&theme_name) {
        Some(t) => t,
        None => {
            eprintln!("Unknown theme: {}", theme_name);
            eprintln!("Use --list to see available themes");
            std::process::exit(1);
        }
    };

    // Auto-read stdin when piped (not a terminal), unless --no-stdin
    let stdin_data = if !cli.no_stdin && !std::io::stdin().is_terminal() {
        StdinData::read()
    } else {
        None
    };

    // Load mappings from config file, merge with CLI --set overrides
    let config_mappings = load_mappings();
    let all_overrides = merge_mappings(config_mappings, &cli.set);

    let mut info = StatusInfo::gather();
    info.apply_overrides(&all_overrides, &stdin_data);
    println!("{}", theme.format(&info));
}

/// Parse ~/.zshrc to find the last ZSH_THEME="..." assignment
fn detect_theme_from_zshrc() -> Option<String> {
    let home = std::env::var("HOME").ok()?;
    let contents = std::fs::read_to_string(format!("{}/.zshrc", home)).ok()?;
    let mut theme = None;
    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("ZSH_THEME=") {
            let val = rest.trim_matches('"').trim_matches('\'');
            if !val.is_empty() {
                theme = Some(val.to_string());
            }
        }
    }
    theme
}

fn init_config() {
    let dir = config::config_dir();
    let path = format!("{}/mappings.conf", dir);

    if std::path::Path::new(&path).exists() {
        eprintln!("Config already exists: {}", path);
        eprintln!("Edit it directly or delete it to re-initialize.");
        return;
    }

    if let Err(e) = std::fs::create_dir_all(&dir) {
        eprintln!("Failed to create config directory: {}", e);
        std::process::exit(1);
    }

    let default_config = r#"# omz2cc field mappings
# Format: field=value
#
# Fields: user, hostname, cwd, git_branch, time
#
# Special values:
#   @model       — Claude Code model name (e.g. "Opus 4.6")
#   @model-id    — raw model ID (e.g. "claude-opus-4-6")
#   @time        — current time (HH:MM:SS)
#   @time:FORMAT — custom strftime format (e.g. @time:%I:%M %p)
#
# Stdin JSON from Claude Code is read automatically when piped.
# CLI --set overrides take precedence over these mappings.
#
# Examples:
# user=@model
# hostname=myhost
# time=@time:%I:%M %p
"#;

    if let Err(e) = std::fs::write(&path, default_config) {
        eprintln!("Failed to write config: {}", e);
        std::process::exit(1);
    }

    println!("Created {}", path);
    println!("Edit this file to configure persistent field mappings.");
}
