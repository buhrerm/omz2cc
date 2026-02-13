mod info;
mod themes;

use clap::Parser;
use info::StatusInfo;
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

    let info = StatusInfo::gather();
    println!("{}", theme.format(&info));
}
