/// Verify theme definitions against reference .zsh-theme files.
///
/// Parses ZSH_THEME_GIT_PROMPT_{PREFIX,SUFFIX,DIRTY,CLEAN} and PROMPT from
/// each reference file, then compares against what our template engine produces.
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Extracted git prompt settings from a .zsh-theme file
#[derive(Debug, Default)]
struct ZshThemeGit {
    prefix: String,
    suffix: String,
    dirty: String,
    clean: String,
    // Stripped versions (text only, no color codes)
    prefix_text: String,
    suffix_text: String,
    dirty_text: String,
    clean_text: String,
    // Color info
    prefix_colors: Vec<String>,
    suffix_colors: Vec<String>,
    dirty_colors: Vec<String>,
    clean_colors: Vec<String>,
}

/// Extracted PROMPT structure
#[derive(Debug, Default)]
struct ZshThemePrompt {
    raw: String,
    uses_vcs_info: bool,
    uses_git_prompt_info: bool,
    uses_custom_git_func: bool,
    has_rprompt: bool,
    rprompt_raw: String,
}

#[derive(Debug)]
struct ZshTheme {
    name: String,
    git: ZshThemeGit,
    prompt: ZshThemePrompt,
}

/// Strip zsh color escape sequences to get plain text
fn strip_zsh_colors(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '%' {
            if let Some(&next) = chars.peek() {
                match next {
                    '{' => {
                        // Skip %{...%}
                        chars.next(); // consume '{'
                        let mut depth = 1;
                        while let Some(c) = chars.next() {
                            if c == '%' {
                                if let Some(&'}') = chars.peek() {
                                    chars.next();
                                    depth -= 1;
                                    if depth == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    'f' | 'F' | 'k' | 'K' | 'b' | 'B' | 'u' | 'U' | 's' | 'S' => {
                        chars.next();
                        // If followed by {color}, skip that too
                        if let Some(&'{') = chars.peek() {
                            chars.next();
                            while let Some(c) = chars.next() {
                                if c == '}' {
                                    break;
                                }
                            }
                        }
                    }
                    _ => {
                        result.push(ch);
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }
    result
}

/// Extract color names from zsh color sequences
fn extract_colors(s: &str) -> Vec<String> {
    let mut colors = Vec::new();
    // Normalize ${FG[...]} to FG[...]
    let s = s.replace("${FG[", "FG[").replace("${fg[", "$fg[").replace("${fg_bold[", "$fg_bold[");

    // Use find-based approach to avoid byte indexing issues with multi-byte chars
    let mut search = s.as_str();
    while !search.is_empty() {
        // $fg_bold[color]
        if let Some(pos) = search.find("$fg_bold[") {
            let after = &search[pos + "$fg_bold[".len()..];
            if let Some(end) = after.find(']') {
                colors.push(format!("bold_{}", &after[..end]));
                search = &after[end + 1..];
                continue;
            }
            search = &search[pos + 1..];
            continue;
        }
        // $fg[color]
        if let Some(pos) = search.find("$fg[") {
            let after = &search[pos + "$fg[".len()..];
            if let Some(end) = after.find(']') {
                colors.push(after[..end].to_string());
                search = &after[end + 1..];
                continue;
            }
            search = &search[pos + 1..];
            continue;
        }
        // FG[123] - 256-color
        if let Some(pos) = search.find("FG[") {
            // Make sure it's not $fg[
            let is_dollar_fg = pos > 0 && search.as_bytes().get(pos - 1) == Some(&b'$');
            if !is_dollar_fg {
                let after = &search[pos + "FG[".len()..];
                if let Some(end) = after.find(']') {
                    colors.push(format!("256:{}", &after[..end]));
                    search = &after[end + 1..];
                    continue;
                }
            }
            search = &search[pos + 1..];
            continue;
        }
        // %F{color}
        if let Some(pos) = search.find("%F{") {
            let after = &search[pos + "%F{".len()..];
            if let Some(end) = after.find('}') {
                let color = &after[..end];
                if color.chars().all(|c| c.is_ascii_digit()) {
                    colors.push(format!("256:{}", color));
                } else {
                    colors.push(color.to_string());
                }
                search = &after[end + 1..];
                continue;
            }
            search = &search[pos + 1..];
            continue;
        }
        break;
    }
    colors
}

/// Also strip $reset_color, %f, etc.
fn strip_reset(s: &str) -> String {
    s.replace("$reset_color", "")
        .replace("%{$reset_color%}", "")
        .replace("%f", "")
        .replace("%{%}", "")
}

/// Clean up extracted text: remove color codes and reset sequences
fn clean_text(s: &str) -> String {
    let s = strip_reset(s);
    strip_zsh_colors(&s).trim().to_string()
}

fn parse_zsh_theme(name: &str, content: &str) -> ZshTheme {
    let mut git = ZshThemeGit::default();
    let mut prompt = ZshThemePrompt::default();

    for line in content.lines() {
        let line = line.trim();

        // Extract git prompt settings
        if let Some(val) = extract_assignment(line, "ZSH_THEME_GIT_PROMPT_PREFIX") {
            git.prefix = val.clone();
            git.prefix_text = clean_text(&val);
            git.prefix_colors = extract_colors(&val);
        }
        if let Some(val) = extract_assignment(line, "ZSH_THEME_GIT_PROMPT_SUFFIX") {
            git.suffix = val.clone();
            git.suffix_text = clean_text(&val);
            git.suffix_colors = extract_colors(&val);
        }
        if let Some(val) = extract_assignment(line, "ZSH_THEME_GIT_PROMPT_DIRTY") {
            git.dirty = val.clone();
            git.dirty_text = clean_text(&val);
            git.dirty_colors = extract_colors(&val);
        }
        if let Some(val) = extract_assignment(line, "ZSH_THEME_GIT_PROMPT_CLEAN") {
            git.clean = val.clone();
            git.clean_text = clean_text(&val);
            git.clean_colors = extract_colors(&val);
        }

        // Extract PROMPT
        if line.starts_with("PROMPT=") || line.starts_with("PROMPT+=") || line.starts_with("PS1=") {
            if prompt.raw.is_empty() || line.starts_with("PROMPT=") || line.starts_with("PS1=") {
                prompt.raw = line.to_string();
            }
        }

        // Extract RPROMPT
        if line.starts_with("RPROMPT=") || line.starts_with("RPS1=") || line.starts_with("RPROMPT+=") {
            prompt.has_rprompt = true;
            if prompt.rprompt_raw.is_empty() || line.starts_with("RPROMPT=") || line.starts_with("RPS1=") {
                prompt.rprompt_raw = line.to_string();
            }
        }

        // Detect usage patterns
        if line.contains("vcs_info") {
            prompt.uses_vcs_info = true;
        }
        if line.contains("git_prompt_info") {
            prompt.uses_git_prompt_info = true;
        }
    }

    // Detect custom git functions
    if content.contains("_git_prompt") || content.contains("_git_info") || content.contains("git_status()") {
        prompt.uses_custom_git_func = true;
    }

    ZshTheme {
        name: name.to_string(),
        git,
        prompt,
    }
}

/// Extract value from ZSH variable assignment like: VAR="value" or VAR='value'
fn extract_assignment(line: &str, var: &str) -> Option<String> {
    let pattern = format!("{}=", var);
    if !line.starts_with(&pattern) {
        return None;
    }
    let rest = &line[pattern.len()..];
    // Strip quotes
    let val = rest.trim();
    let val = if (val.starts_with('"') && val.ends_with('"'))
        || (val.starts_with('\'') && val.ends_with('\''))
    {
        &val[1..val.len() - 1]
    } else {
        val
    };
    Some(val.to_string())
}

/// Classify the git block pattern
fn classify_git_pattern(theme: &ZshTheme) -> &'static str {
    let prefix = &theme.git.prefix_text;
    let suffix = &theme.git.suffix_text;
    let dirty = &theme.git.dirty_text;
    let clean = &theme.git.clean_text;

    if theme.prompt.uses_vcs_info {
        return "vcs_info";
    }
    if theme.prompt.uses_custom_git_func && !theme.prompt.uses_git_prompt_info {
        return "custom_func";
    }

    // Check for 256-color patterns
    let has_256 = theme.git.prefix_colors.iter().any(|c| c.starts_with("256:"))
        || theme.git.suffix_colors.iter().any(|c| c.starts_with("256:"));

    if prefix.contains('‹') || suffix.contains('›') {
        return "angle_yellow";
    }
    if prefix.contains("git:(") {
        if has_256 {
            return "git_colon_paren_256";
        }
        return "git_colon_paren";
    }
    if prefix.contains("git:") && !prefix.contains("git:(") {
        return "git_colon";
    }
    if prefix.contains('[') && prefix.contains("±") {
        return "bracket_pm";
    }
    if prefix.starts_with(' ') && prefix.trim().starts_with("on ") {
        return "on_branch";
    }
    if has_256 && (prefix.contains('(') || suffix.contains(')')) {
        return "paren_256";
    }
    if prefix.contains('(') || suffix.contains(')') {
        return "paren_simple";
    }
    if prefix.contains('[') || suffix.contains(']') {
        return "bracket";
    }
    if prefix.is_empty() && suffix.is_empty() && dirty.is_empty() && clean.is_empty()
        && !theme.prompt.uses_git_prompt_info
    {
        return "no_git";
    }
    if prefix.is_empty() && suffix.is_empty() {
        return "plain_branch";
    }

    "other"
}

/// Map zsh color name to our Color enum name
fn map_color(zsh_color: &str) -> &'static str {
    match zsh_color {
        "red" => "Red",
        "green" => "Green",
        "yellow" => "Yellow",
        "blue" => "Blue",
        "magenta" => "Magenta",
        "cyan" => "Cyan",
        "white" => "White",
        "black" => "Black",
        "grey" | "gray" => "Gray",
        "bold_red" => "Red(bold)",
        "bold_green" => "Green(bold)",
        "bold_yellow" => "Yellow(bold)",
        "bold_blue" => "Blue(bold)",
        "bold_magenta" => "Magenta(bold)",
        "bold_cyan" => "Cyan(bold)",
        "bold_white" => "White(bold)",
        _ => "unknown",
    }
}

fn format_colors(colors: &[String]) -> String {
    if colors.is_empty() {
        return "-".to_string();
    }
    colors
        .iter()
        .map(|c| {
            if c.starts_with("256:") {
                c.clone()
            } else {
                map_color(c).to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn main() {
    let ref_dir = Path::new("reference-themes");
    if !ref_dir.exists() {
        eprintln!("Error: reference-themes/ directory not found. Run from project root.");
        std::process::exit(1);
    }

    let mut themes: Vec<ZshTheme> = Vec::new();
    let mut entries: Vec<_> = fs::read_dir(ref_dir)
        .expect("cannot read reference-themes/")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "zsh-theme"))
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        let name = path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let content = fs::read_to_string(&path).unwrap_or_default();
        themes.push(parse_zsh_theme(&name, &content));
    }

    // Group by pattern
    let mut by_pattern: HashMap<&str, Vec<&ZshTheme>> = HashMap::new();
    for theme in &themes {
        let pattern = classify_git_pattern(theme);
        by_pattern.entry(pattern).or_default().push(theme);
    }

    // Print summary by pattern
    println!("=== Theme Classification Summary ===\n");
    let mut patterns: Vec<_> = by_pattern.keys().copied().collect();
    patterns.sort();
    for pattern in &patterns {
        let themes = &by_pattern[pattern];
        println!(
            "{:20} ({:3} themes): {}",
            pattern,
            themes.len(),
            themes
                .iter()
                .map(|t| t.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    println!("\n=== Detailed Git Settings ===\n");
    println!(
        "{:<22} {:<18} {:<20} {:<20} {:<16} {:<16} {:<20} {:<20}",
        "Theme", "Pattern", "Prefix(text)", "Suffix(text)", "Dirty(text)", "Clean(text)", "Prefix Colors", "Dirty Colors"
    );
    println!("{}", "-".repeat(160));

    for theme in &themes {
        let pattern = classify_git_pattern(theme);
        println!(
            "{:<22} {:<18} {:<20} {:<20} {:<16} {:<16} {:<20} {:<20}",
            theme.name,
            pattern,
            format!("'{}'", theme.git.prefix_text),
            format!("'{}'", theme.git.suffix_text),
            format!("'{}'", theme.git.dirty_text),
            format!("'{}'", theme.git.clean_text),
            format_colors(&theme.git.prefix_colors),
            format_colors(&theme.git.dirty_colors),
        );
    }

    // Print themes with RPROMPT
    println!("\n=== Themes with RPROMPT ===\n");
    for theme in &themes {
        if theme.prompt.has_rprompt {
            println!("{:<22} RPROMPT: {}", theme.name, theme.prompt.rprompt_raw);
        }
    }

    // Print 256-color themes
    println!("\n=== Themes with 256-color ===\n");
    for theme in &themes {
        let all_colors: Vec<&String> = theme
            .git
            .prefix_colors
            .iter()
            .chain(&theme.git.suffix_colors)
            .chain(&theme.git.dirty_colors)
            .chain(&theme.git.clean_colors)
            .filter(|c| c.starts_with("256:"))
            .collect();
        if !all_colors.is_empty() {
            println!(
                "{:<22} colors: {:?}",
                theme.name,
                all_colors
            );
        }
    }
}
