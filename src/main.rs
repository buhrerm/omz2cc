mod color;
mod info;
mod themes;

use clap::Parser;
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

    let stdin_data = if cli.stdin {
        StdinData::read()
    } else {
        None
    };

    let mut info = StatusInfo::gather();
    info.apply_overrides(&cli.set, &stdin_data);
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
