/// Verify theme definitions against reference .zsh-theme files.
///
/// Parses PROMPT, RPROMPT, and ZSH_THEME_GIT_PROMPT_* from each reference file,
/// builds expected visible text, and compares against our template engine output.
use std::fs;
use std::path::Path;

use omz2cc::info::StatusInfo;
use omz2cc::themes::defs;
use omz2cc::themes::template::TemplateDef;
use omz2cc::themes::Theme;

// ---------------------------------------------------------------------------
// Test StatusInfo with known values for comparison
// ---------------------------------------------------------------------------

fn test_info() -> StatusInfo {
    StatusInfo {
        user: "user".to_string(),
        hostname: "host".to_string(),
        short_hostname: "host".to_string(),
        cwd: "~/test".to_string(),
        cwd_basename: "test".to_string(),
        git_branch: Some("main".to_string()),
        git_dirty: Some(true),
        git_ahead: None,
        git_behind: None,
        git_action: None,
        exit_code: 0,
        time: "12:00:00".to_string(),
    }
}

fn test_info_no_git() -> StatusInfo {
    StatusInfo {
        user: "user".to_string(),
        hostname: "host".to_string(),
        short_hostname: "host".to_string(),
        cwd: "~/test".to_string(),
        cwd_basename: "test".to_string(),
        git_branch: None,
        git_dirty: None,
        git_ahead: None,
        git_behind: None,
        git_action: None,
        exit_code: 0,
        time: "12:00:00".to_string(),
    }
}

// ---------------------------------------------------------------------------
// Zsh theme parser
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
struct ZshTheme {
    name: String,
    prompt_raw: String,
    rprompt_raw: String,
    has_rprompt: bool,
    git_prefix: String,
    git_suffix: String,
    git_dirty: String,
    git_clean: String,
    uses_vcs_info: bool,
    uses_custom_git_func: bool,
    uses_git_prompt_info: bool,
    uses_git_prompt_status: bool,
    prompt_has_git: bool,
    rprompt_has_git: bool,
}

/// Extract value from ZSH variable assignment like: VAR="value" or VAR='value'
fn extract_assignment(line: &str, var: &str) -> Option<String> {
    let pattern = format!("{}=", var);
    if !line.starts_with(&pattern) {
        return None;
    }
    let rest = &line[pattern.len()..];
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

fn parse_zsh_theme(name: &str, content: &str) -> ZshTheme {
    let mut theme = ZshTheme {
        name: name.to_string(),
        ..Default::default()
    };

    for line in content.lines() {
        let line = line.trim();

        if let Some(val) = extract_assignment(line, "ZSH_THEME_GIT_PROMPT_PREFIX") {
            theme.git_prefix = val;
        }
        if let Some(val) = extract_assignment(line, "ZSH_THEME_GIT_PROMPT_SUFFIX") {
            theme.git_suffix = val;
        }
        if let Some(val) = extract_assignment(line, "ZSH_THEME_GIT_PROMPT_DIRTY") {
            theme.git_dirty = val;
        }
        if let Some(val) = extract_assignment(line, "ZSH_THEME_GIT_PROMPT_CLEAN") {
            theme.git_clean = val;
        }

        // Extract PROMPT
        if line.starts_with("PROMPT=") || line.starts_with("PS1=") {
            let val = if line.starts_with("PROMPT=") {
                &line["PROMPT=".len()..]
            } else {
                &line["PS1=".len()..]
            };
            theme.prompt_raw = val.to_string();
            if val.contains("git_prompt_info") || val.contains("git_prompt_short_sha") {
                theme.prompt_has_git = true;
            }
        }

        // Extract RPROMPT
        if line.starts_with("RPROMPT=") || line.starts_with("RPS1=") {
            theme.has_rprompt = true;
            let val = if line.starts_with("RPROMPT=") {
                &line["RPROMPT=".len()..]
            } else {
                &line["RPS1=".len()..]
            };
            theme.rprompt_raw = val.to_string();
            if val.contains("git_prompt_info") || val.contains("git_prompt")
                || val.contains("git_time_since_commit")
            {
                theme.rprompt_has_git = true;
            }
        }

        if line.contains("vcs_info") {
            theme.uses_vcs_info = true;
        }
        if line.contains("git_prompt_info") {
            theme.uses_git_prompt_info = true;
        }
        if line.contains("git_prompt_status") {
            theme.uses_git_prompt_status = true;
        }
    }

    // Detect custom git functions
    if content.contains("git_time_since_commit")
        || content.contains("_git_prompt")
        || content.contains("_git_info")
        || content.contains("git_prompt()")
        || content.contains("_fishy_collapsed_wd")
    {
        theme.uses_custom_git_func = true;
    }

    theme
}

// ---------------------------------------------------------------------------
// Zsh escape resolution — convert %n, %m, etc. to test values
// ---------------------------------------------------------------------------

/// Strip zsh color escape sequences: %{...%}, $fg[...], $reset_color, etc.
fn strip_zsh_colors(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '%' {
            if let Some(&next) = chars.peek() {
                match next {
                    '{' => {
                        // Skip %{...%}
                        chars.next();
                        while let Some(c) = chars.next() {
                            if c == '%' {
                                if let Some(&'}') = chars.peek() {
                                    chars.next();
                                    break;
                                }
                            }
                        }
                    }
                    'B' | 'b' => {
                        chars.next(); // bold on/off — skip
                    }
                    'f' | 'F' | 'k' | 'K' => {
                        chars.next();
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
                        // Keep as-is for resolution later
                        result.push(ch);
                        result.push(chars.next().unwrap());
                    }
                }
            }
        } else if ch == '$' {
            // Skip shell variables like $fg[...], $reset_color, etc.
            let rest: String = chars.clone().collect();
            if rest.starts_with("reset_color") {
                for _ in 0.."reset_color".len() {
                    chars.next();
                }
            } else if rest.starts_with("fg_bold[") || rest.starts_with("fg_no_bold[") || rest.starts_with("fg[") {
                while let Some(c) = chars.next() {
                    if c == ']' {
                        break;
                    }
                }
            } else if rest.starts_with("{$fg") || rest.starts_with("{$reset_color") {
                // ${fg[...]} or ${$reset_color}
                while let Some(c) = chars.next() {
                    if c == '}' {
                        break;
                    }
                }
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }
    result
}

/// Resolve zsh prompt escapes to test values
fn resolve_zsh_escapes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '%' {
            match chars.peek() {
                Some(&'n') => {
                    chars.next();
                    result.push_str("user");
                }
                Some(&'m') => {
                    chars.next();
                    result.push_str("host");
                }
                Some(&'M') => {
                    chars.next();
                    result.push_str("host");
                }
                Some(&'~') => {
                    chars.next();
                    result.push_str("~/test");
                }
                Some(&'c') => {
                    chars.next();
                    result.push_str("test");
                }
                Some(&'*') => {
                    chars.next();
                    result.push_str("12:00:00");
                }
                Some(&'T') => {
                    chars.next();
                    result.push_str("12:00");
                }
                Some(&'t') | Some(&'@') => {
                    chars.next();
                    result.push_str("12:00 PM");
                }
                Some(&'D') => {
                    chars.next();
                    // %D{format}
                    if let Some(&'{') = chars.peek() {
                        chars.next();
                        let mut fmt = String::new();
                        while let Some(c) = chars.next() {
                            if c == '}' {
                                break;
                            }
                            fmt.push(c);
                        }
                        // Resolve common strftime patterns
                        let resolved = fmt
                            .replace("%L", "12")
                            .replace("%M", "00")
                            .replace("%R", "12:00")
                            .replace("%S", "00")
                            .replace("%p", "PM")
                            .replace("%a", "Fri")
                            .replace("%b", "Feb")
                            .replace("%d", "14")
                            .replace("%Y", "2026");
                        result.push_str(&resolved);
                    }
                }
                Some(&'!') => {
                    chars.next();
                    result.push_str("1");
                }
                Some(&'#') => {
                    chars.next();
                    result.push_str("$");
                }
                Some(c) if c.is_ascii_digit() => {
                    // %N~ — truncated cwd
                    let n = *c;
                    chars.next();
                    if let Some(&'~') = chars.peek() {
                        chars.next();
                        match n {
                            '1' => result.push_str("test"),
                            '2' => result.push_str("~/test"),
                            _ => result.push_str("~/test"),
                        }
                    } else {
                        result.push('%');
                        result.push(n);
                    }
                }
                Some(&'(') => {
                    // Conditional: %(?.true.false)
                    chars.next();
                    let mut cond = String::new();
                    let mut depth = 1;
                    while let Some(c) = chars.next() {
                        if c == '(' {
                            depth += 1;
                        } else if c == ')' {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        cond.push(c);
                    }
                    // Just take the "true" branch (exit code 0)
                    if let Some(dot1) = cond.find('.') {
                        let rest = &cond[dot1 + 1..];
                        if let Some(dot2) = rest.find('.') {
                            result.push_str(&rest[..dot2]);
                        }
                    }
                }
                _ => {
                    result.push('%');
                }
            }
        } else {
            result.push(ch);
        }
    }
    result
}

/// Build expected visible text from a PROMPT string
fn build_expected(raw: &str, theme: &ZshTheme) -> String {
    // First strip colors
    let stripped = strip_zsh_colors(raw);
    // Then resolve zsh escapes
    let resolved = resolve_zsh_escapes(&stripped);
    // Replace $(git_prompt_info) with the expected git output
    let with_git = resolve_git_calls(&resolved, theme);
    // Clean up shell artifacts
    let cleaned = with_git
        .replace("$(ruby_prompt_info)", "")
        .replace("$(svn_prompt_info)", "")
        .replace("$(git_prompt_status)", "")
        .replace("$(git_prompt_ahead)", "")
        .replace("$(prompt_char)", "")
        .replace("$(git_time_since_commit)", "")
        .replace("$(_fishy_collapsed_wd)", "~/test")
        .replace("$(prompt_context)", "user@host")
        .replace("$(prompt_header)", "")
        .replace("$(rvm_gemset)", "")
        .replace("${VIRTUAL_ENV:+\"($VIRTUAL_ENV) \"}", "")
        .replace("${return_status}", "")
        .replace("${smiley}", "☺")
        .replace("${user}", "user")
        .replace("${host}", "host")
        .replace("${pwd}", "~/test")
        .replace("${time}", "12:00:00")
        .replace("${RPROMPT}", "")
        .replace("$ZSH_THEME_CLOUD_PREFIX", "☁");
    // Remove surrounding quotes
    let trimmed = cleaned.trim_matches(|c| c == '\'' || c == '"');
    // Collapse multiple spaces and trim
    let mut result = String::new();
    let mut last_space = false;
    for ch in trimmed.chars() {
        if ch == ' ' {
            if !last_space {
                result.push(' ');
            }
            last_space = true;
        } else {
            last_space = false;
            result.push(ch);
        }
    }
    result.trim().to_string()
}

fn resolve_git_calls(s: &str, theme: &ZshTheme) -> String {
    // $(git_prompt_info) → prefix + branch + dirty + suffix
    let git_text = format!(
        "{}main{}{}",
        strip_zsh_colors_and_resolve(&theme.git_prefix),
        strip_zsh_colors_and_resolve(&theme.git_dirty),
        strip_zsh_colors_and_resolve(&theme.git_suffix),
    );
    s.replace("$(git_prompt_info)", &git_text)
}

fn strip_zsh_colors_and_resolve(s: &str) -> String {
    resolve_zsh_escapes(&strip_zsh_colors(s))
}

/// Strip ANSI escape sequences from our rendered output
fn strip_ansi(s: &str) -> String {
    let mut result = String::new();
    let mut in_escape = false;
    for c in s.chars() {
        if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else if c == '\x1b' {
            in_escape = true;
        } else {
            result.push(c);
        }
    }
    result
}

// ---------------------------------------------------------------------------
// Comparison logic
// ---------------------------------------------------------------------------

#[derive(Debug)]
enum VerifyResult {
    Pass,
    Mismatch { expected: String, got: String },
    GitPlacement { detail: String },
    Complex,
    NotFound,
}

fn verify_theme(zsh: &ZshTheme, tpl: &TemplateDef) -> VerifyResult {
    // Flag complex themes
    if zsh.uses_custom_git_func && !zsh.uses_git_prompt_info {
        return VerifyResult::Complex;
    }

    // Check git placement
    let our_has_git_in_main = has_git_in_segments(tpl.segments);
    let our_has_git_in_rprompt = has_git_in_segments(tpl.rprompt);

    if zsh.rprompt_has_git && !zsh.prompt_has_git && our_has_git_in_main && !our_has_git_in_rprompt {
        return VerifyResult::GitPlacement {
            detail: "git should be in RPROMPT, currently in main segments".to_string(),
        };
    }

    // Build expected text from reference
    let expected_main = build_expected(&zsh.prompt_raw, zsh);

    // Render our template
    let info = test_info();
    let our_output = tpl.format(&info);
    let our_visible = strip_ansi(&our_output);

    // Normalize both for comparison: collapse whitespace, trim
    let expected_norm = normalize(&expected_main);
    let our_norm = normalize(&our_visible);

    if expected_norm == our_norm {
        VerifyResult::Pass
    } else {
        VerifyResult::Mismatch {
            expected: expected_norm,
            got: our_norm,
        }
    }
}

fn has_git_in_segments(segments: &[omz2cc::themes::template::Segment]) -> bool {
    use omz2cc::themes::template::Segment;
    for seg in segments {
        match seg {
            Segment::Field(omz2cc::themes::template::FieldName::GitBranch, _, _, _, _) => return true,
            Segment::IfGit(_) => return true,
            Segment::Dirty(_, _, _, _, _, _) => return true,
            _ => {}
        }
    }
    false
}

fn normalize(s: &str) -> String {
    let mut result = String::new();
    let mut last_space = false;
    for ch in s.chars() {
        if ch == '\n' {
            continue; // Skip newlines for single-line comparison
        }
        if ch == ' ' {
            if !last_space {
                result.push(' ');
            }
            last_space = true;
        } else {
            last_space = false;
            result.push(ch);
        }
    }
    result.trim().to_string()
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    let ref_dir = Path::new(&std::env::var("HOME").unwrap_or_default())
        .join(".oh-my-zsh/themes");
    if !ref_dir.exists() {
        eprintln!("Error: ~/.oh-my-zsh/themes/ not found");
        std::process::exit(1);
    }

    let mut entries: Vec<_> = fs::read_dir(&ref_dir)
        .expect("cannot read themes dir")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "zsh-theme"))
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let mut pass = 0u32;
    let mut mismatch = 0u32;
    let mut git_placement = 0u32;
    let mut complex = 0u32;
    let mut not_found = 0u32;
    let mut issues: Vec<String> = Vec::new();

    for entry in &entries {
        let path = entry.path();
        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        let content = fs::read_to_string(&path).unwrap_or_default();
        let zsh = parse_zsh_theme(&name, &content);

        // Find our template
        let tpl = defs::get_template(&name);
        if tpl.is_none() {
            // Skip agnoster (manual impl) and themes we don't have
            if name == "agnoster" {
                continue;
            }
            not_found += 1;
            issues.push(format!("NOT_FOUND  {}", name));
            continue;
        }
        let tpl = tpl.unwrap();

        match verify_theme(&zsh, tpl) {
            VerifyResult::Pass => {
                pass += 1;
                println!("  PASS       {}", name);
            }
            VerifyResult::Mismatch { expected, got } => {
                mismatch += 1;
                let msg = format!("  MISMATCH   {}  expected: {:?}  got: {:?}", name, expected, got);
                println!("{}", msg);
                issues.push(msg);
            }
            VerifyResult::GitPlacement { detail } => {
                git_placement += 1;
                let msg = format!("  GIT_PLACE  {}  {}", name, detail);
                println!("{}", msg);
                issues.push(msg);
            }
            VerifyResult::Complex => {
                complex += 1;
                println!("  COMPLEX    {}  (custom functions, skipped)", name);
            }
            VerifyResult::NotFound => {
                not_found += 1;
                issues.push(format!("  NOT_FOUND  {}", name));
            }
        }
    }

    println!("\n=== Summary ===");
    println!("  PASS:          {}", pass);
    println!("  MISMATCH:      {}", mismatch);
    println!("  GIT_PLACEMENT: {}", git_placement);
    println!("  COMPLEX:       {}", complex);
    println!("  NOT_FOUND:     {}", not_found);
    println!("  TOTAL:         {}", entries.len());

    if !issues.is_empty() {
        println!("\n=== Issues ===");
        for issue in &issues {
            println!("{}", issue);
        }
    }
}
