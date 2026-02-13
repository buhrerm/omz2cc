mod color;
mod info;
mod themes;

use clap::Parser;
use info::{StdinData, StatusInfo};
use themes::{all_themes, get_theme};

#[derive(Parser)]
#[command(name = "omz2cc", about = "Oh My Zsh to Claude Code status line")]
struct Cli {
    /// Theme name (default: auto-detect from ~/.zshrc ZSH_THEME, fallback "ys")
    #[arg(short, long)]
    theme: Option<String>,

    /// List available themes
    #[arg(short, long)]
    list: bool,

    /// Override a field: --set user=@model --set hostname=myhost
    #[arg(short, long = "set", value_name = "KEY=VALUE")]
    set: Vec<String>,

    /// Read Claude Code JSON from stdin (provides @model, @model-id)
    #[arg(long)]
    stdin: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.list {
        for theme in all_themes() {
            println!("{}", theme.name());
        }
        return;
    }

    let theme_name = cli
        .theme
        .unwrap_or_else(|| detect_omz_theme().unwrap_or_else(|| "ys".to_string()));

    let theme = match get_theme(&theme_name) {
        Some(t) => t,
        None => {
            eprintln!("Unknown theme: {}", theme_name);
            eprintln!("Use --list to see available themes");
            std::process::exit(1);
        }
    };

    let stdin_data = if cli.stdin {
        StdinData::read()
    } else {
        None
    };

    let mut info = StatusInfo::gather();
    info.apply_overrides(&cli.set, &stdin_data);
    println!("{}", theme.format(&info));
}

/// Read ZSH_THEME from ~/.zshrc (or $ZDOTDIR/.zshrc)
fn detect_omz_theme() -> Option<String> {
    let zshrc = std::env::var("ZDOTDIR")
        .ok()
        .map(std::path::PathBuf::from)
        .or_else(|| {
            std::env::var("HOME")
                .ok()
                .map(std::path::PathBuf::from)
        })?
        .join(".zshrc");

    let contents = std::fs::read_to_string(zshrc).ok()?;

    for line in contents.lines() {
        let trimmed = line.trim();
        // Skip comments
        if trimmed.starts_with('#') {
            continue;
        }
        // Match ZSH_THEME="theme" or ZSH_THEME='theme' or ZSH_THEME=theme
        if let Some(rest) = trimmed
            .strip_prefix("ZSH_THEME=")
            .or_else(|| trimmed.strip_prefix("ZSH_THEME ="))
        {
            let rest = rest.trim();
            let name = rest
                .trim_matches('"')
                .trim_matches('\'')
                .trim();
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
    }

    None
}
