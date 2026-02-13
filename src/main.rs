mod color;
mod info;
mod themes;

use clap::Parser;
use info::{StdinData, StatusInfo};
use themes::{all_themes, get_theme};

#[derive(Parser)]
#[command(name = "omz2cc", about = "Oh My Zsh to Claude Code status line")]
struct Cli {
    /// Theme name to use for formatting
    #[arg(short, long, default_value = "ys")]
    theme: String,

    /// List available themes
    #[arg(short, long)]
    list: bool,

    /// Override a field: --set user=@model --set hostname=claude
    /// Special values: @model (pretty model from stdin JSON), @model-id (raw model ID)
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

    let theme = match get_theme(&cli.theme) {
        Some(t) => t,
        None => {
            eprintln!("Unknown theme: {}", cli.theme);
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
