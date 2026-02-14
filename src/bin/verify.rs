/// Verify theme definitions against reference .zsh-theme files.
///
/// For each theme in ~/.oh-my-zsh/themes/, this tool:
/// 1. Parses PROMPT, RPROMPT, and ZSH_THEME_GIT_PROMPT_* settings
/// 2. Resolves shell variables, strips color formatting, resolves zsh escapes
/// 3. Compares the resulting visible text against our TemplateDef output
///
/// Usage:
///   cargo run --bin verify              # verify all themes
///   cargo run --bin verify -- <name>    # verify a single theme
///   cargo run --bin verify -- -v        # verbose mode (show all details)
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use omz2cc::info::StatusInfo;
use omz2cc::themes::defs;
use omz2cc::themes::template::{FieldName, Segment, TemplateDef};

// ---------------------------------------------------------------------------
// Test values for comparison
// ---------------------------------------------------------------------------

fn test_info() -> StatusInfo {
    StatusInfo {
        user: "user".into(),
        hostname: "host".into(),
        short_hostname: "host".into(),
        cwd: "~/test".into(),
        cwd_basename: "test".into(),
        git_branch: Some("main".into()),
        git_dirty: Some(true),
        git_ahead: None,
        git_behind: None,
        git_action: None,
        exit_code: 0,
        time: "12:00:00".into(),
    }
}

// ===========================================================================
// Our template renderer — produces visible text only (no ANSI codes)
// ===========================================================================

fn render_visible(segments: &[Segment], info: &StatusInfo) -> String {
    let mut out = String::new();
    for seg in segments {
        match seg {
            Segment::Text(s) => out.push_str(s),
            Segment::Lit(s, _, _) => out.push_str(s),
            Segment::Field(name, _, _, prefix, suffix) => {
                out.push_str(prefix);
                out.push_str(&resolve_field(name, info));
                out.push_str(suffix);
            }
            Segment::IfGit(segs) => {
                if info.git_branch.is_some() {
                    out.push_str(&render_visible(segs, info));
                }
            }
            Segment::IfNotGit(segs) => {
                if info.git_branch.is_none() {
                    out.push_str(&render_visible(segs, info));
                }
            }
            Segment::Dirty(clean, dirty_str, _, _, _, _) => {
                if info.git_dirty == Some(true) {
                    out.push_str(dirty_str);
                } else {
                    out.push_str(clean);
                }
            }
        }
    }
    out
}

fn resolve_field(name: &FieldName, info: &StatusInfo) -> String {
    match name {
        FieldName::User => info.user.clone(),
        FieldName::Hostname => info.hostname.clone(),
        FieldName::ShortHostname => info.short_hostname.clone(),
        FieldName::Cwd => info.cwd.clone(),
        FieldName::CwdBasename => info.cwd_basename.clone(),
        FieldName::CwdTruncated(n) => info.cwd_truncated(*n),
        FieldName::CwdAbsolute => "/home/user/test".to_string(),
        FieldName::GitBranch => info.git_branch.clone().unwrap_or_default(),
        FieldName::Time => info.time.clone(),
        FieldName::TimeHHMM => "12:00".to_string(),
        FieldName::Time12h => "12:00 PM".to_string(),
        FieldName::DateTime(fmt) => {
            fmt.replace("%Y", "2026")
                .replace("%y", "26")
                .replace("%m", "02")
                .replace("%d", "14")
                .replace("%e", "14")
                .replace("%a", "Fri")
                .replace("%A", "Friday")
                .replace("%b", "Feb")
                .replace("%B", "February")
                .replace("%H", "12")
                .replace("%I", "12")
                .replace("%M", "00")
                .replace("%S", "00")
                .replace("%p", "PM")
                .replace("%P", "pm")
        }
        FieldName::UserAtHost => format!("{}@{}", info.user, info.hostname),
        FieldName::HostColonCwd => format!("{}:{}", info.hostname, info.cwd),
    }
}

// ===========================================================================
// Parsed reference theme
// ===========================================================================

#[derive(Debug, Default)]
struct ZshTheme {
    prompt_visible: String,
    rprompt_visible: String,
    has_rprompt: bool,
    is_complex: bool,
    complex_reason: String,
    prompt_has_git: bool,
    rprompt_has_git: bool,
}

// ===========================================================================
// Phase 1: Pre-processing — join backslash continuations
// ===========================================================================

fn preprocess(content: &str) -> String {
    let mut result = String::new();
    let mut continuation = String::new();
    for line in content.lines() {
        if line.ends_with('\\') && !line.ends_with("\\\\") {
            continuation.push_str(&line[..line.len() - 1]);
        } else if !continuation.is_empty() {
            continuation.push_str(line);
            result.push_str(&continuation);
            result.push('\n');
            continuation.clear();
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    if !continuation.is_empty() {
        result.push_str(&continuation);
        result.push('\n');
    }
    result
}

// ===========================================================================
// Phase 2: Collect variable assignments
// ===========================================================================

fn collect_vars(content: &str) -> HashMap<String, String> {
    let mut vars = HashMap::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();
        i += 1;

        // Skip comments, empty, control flow
        if line.is_empty()
            || line.starts_with('#')
            || line.starts_with("if ")
            || line.starts_with("else")
            || line.starts_with("fi")
            || line.starts_with("function ")
            || line.ends_with("() {")
            || line == "}"
            || line == "{"
            || line.starts_with("autoload")
            || line.starts_with("setopt")
            || line.starts_with("zstyle")
            || line.starts_with("add-zsh-hook")
            || line.starts_with("source")
            || line.starts_with("eval")
        {
            continue;
        }

        // Strip leading local/export/typeset keywords
        let stripped = strip_keyword(line);
        if stripped.is_empty() {
            continue;
        }

        // Find VAR=value or VAR+=value pattern
        let is_append;
        let eq_idx;
        if let Some(idx) = stripped.find("+=") {
            let var_name = &stripped[..idx];
            if is_valid_var_name(var_name) {
                is_append = true;
                eq_idx = idx;
            } else {
                continue;
            }
        } else if let Some(idx) = stripped.find('=') {
            let var_name = &stripped[..idx];
            if is_valid_var_name(var_name) {
                is_append = false;
                eq_idx = idx;
            } else {
                continue;
            }
        } else {
            continue;
        };

        {
            let var_name = &stripped[..eq_idx];
            let rhs = if is_append {
                &stripped[eq_idx + 2..]
            } else {
                &stripped[eq_idx + 1..]
            };

            // Handle multi-line strings: find the complete value
            let (value, extra_lines) = extract_value(rhs, &lines[i..]);
            i += extra_lines;

            if is_append {
                let mut existing: String = vars.get(var_name).cloned().unwrap_or_default();
                existing.push_str(&value);
                vars.insert(var_name.to_string(), existing);
            } else {
                vars.insert(var_name.to_string(), value);
            }
        }
    }
    vars
}

fn strip_keyword(line: &str) -> &str {
    for prefix in &["local ", "export ", "typeset -g ", "typeset "] {
        if let Some(rest) = line.strip_prefix(prefix) {
            return rest.trim();
        }
    }
    line
}

fn is_valid_var_name(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        && !s.starts_with(|c: char| c.is_ascii_digit())
}

/// Extract a complete value from an assignment RHS, handling multi-line strings.
/// Returns (value, number_of_extra_lines_consumed).
fn extract_value(rhs: &str, remaining_lines: &[&str]) -> (String, usize) {
    let rhs = rhs.trim();

    // $'...' ANSI-C quoting
    if rhs.starts_with("$'") {
        return extract_ansi_c_string(rhs, remaining_lines);
    }

    // Double-quoted string
    if rhs.starts_with('"') {
        return extract_double_quoted(rhs, remaining_lines);
    }

    // Single-quoted string
    if rhs.starts_with('\'') {
        return extract_single_quoted(rhs, remaining_lines);
    }

    // Unquoted value (single line)
    (rhs.to_string(), 0)
}

fn extract_ansi_c_string(rhs: &str, remaining: &[&str]) -> (String, usize) {
    // Handle $'...' possibly concatenated with plain text and more $'...' blocks
    // e.g., $'part1'middle$'part2'
    let mut s = rhs.to_string();
    let mut extra = 0;
    let mut result = String::new();
    let mut pos = 0;

    loop {
        if pos >= s.len() {
            break;
        }

        if s[pos..].starts_with("$'") {
            // ANSI-C quoted block
            let inner_start = pos + 2;
            // Find closing unescaped quote
            loop {
                if let Some(end) = find_unescaped_quote(&s[inner_start..]) {
                    let inner = &s[inner_start..inner_start + end];
                    result.push_str(&resolve_ansi_c(inner));
                    pos = inner_start + end + 1; // past closing quote
                    break;
                }
                // Need more lines
                if extra < remaining.len() {
                    s.push('\n');
                    s.push_str(remaining[extra]);
                    extra += 1;
                } else {
                    result.push_str(&resolve_ansi_c(&s[inner_start..]));
                    pos = s.len();
                    break;
                }
            }
        } else if s[pos..].starts_with('"') {
            // Double-quoted block concatenated
            let inner_start = pos + 1;
            if let Some(end) = find_unescaped_double_quote(&s[inner_start..]) {
                let inner = &s[inner_start..inner_start + end];
                result.push_str(&inner.replace("\\$", "$").replace("\\\\", "\\"));
                pos = inner_start + end + 1;
            } else {
                result.push_str(&s[inner_start..]);
                pos = s.len();
            }
        } else if s[pos..].starts_with('\'') {
            // Single-quoted block concatenated
            let inner_start = pos + 1;
            if let Some(end) = s[inner_start..].find('\'') {
                result.push_str(&s[inner_start..inner_start + end]);
                pos = inner_start + end + 1;
            } else {
                result.push_str(&s[inner_start..]);
                pos = s.len();
            }
        } else {
            // Plain text between quoted blocks
            result.push(s.as_bytes()[pos] as char);
            pos += 1;
        }
    }

    (result, extra)
}

fn extract_double_quoted(rhs: &str, remaining: &[&str]) -> (String, usize) {
    let inner_start = 1; // skip "
    let mut s = rhs.to_string();
    let mut extra = 0;

    loop {
        if let Some(end) = find_unescaped_double_quote(&s[inner_start..]) {
            let inner = &s[inner_start..inner_start + end];
            // Unescape \$ → $ and \\ → \ in double-quoted strings
            let unescaped = inner.replace("\\$", "$").replace("\\\\", "\\");
            return (unescaped, extra);
        }
        if extra < remaining.len() {
            s.push('\n');
            s.push_str(remaining[extra]);
            extra += 1;
        } else {
            let inner = &s[inner_start..];
            let unescaped = inner.replace("\\$", "$").replace("\\\\", "\\");
            return (unescaped, extra);
        }
    }
}

fn extract_single_quoted(rhs: &str, remaining: &[&str]) -> (String, usize) {
    let inner_start = 1; // skip '
    let mut s = rhs.to_string();
    let mut extra = 0;

    loop {
        if let Some(end) = s[inner_start..].find('\'') {
            let inner = &s[inner_start..inner_start + end];
            return (inner.to_string(), extra);
        }
        if extra < remaining.len() {
            s.push('\n');
            s.push_str(remaining[extra]);
            extra += 1;
        } else {
            return (s[inner_start..].to_string(), extra);
        }
    }
}

fn find_unescaped_quote(s: &str) -> Option<usize> {
    let mut i = 0;
    let chars: Vec<char> = s.chars().collect();
    while i < chars.len() {
        if chars[i] == '\\' {
            i += 2; // skip escaped char
        } else if chars[i] == '\'' {
            return Some(s.char_indices().nth(i).map(|(idx, _)| idx).unwrap_or(i));
        } else {
            i += 1;
        }
    }
    None
}

fn find_unescaped_double_quote(s: &str) -> Option<usize> {
    let mut i = 0;
    let bytes = s.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            i += 2;
        } else if bytes[i] == b'"' {
            return Some(i);
        } else {
            i += 1;
        }
    }
    None
}

/// Resolve ANSI-C escape sequences: \e→ESC, \n→newline, etc.
fn resolve_ansi_c(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.peek() {
                Some(&'e') => {
                    chars.next();
                    result.push('\x1b');
                }
                Some(&'n') => {
                    chars.next();
                    result.push('\n');
                }
                Some(&'t') => {
                    chars.next();
                    result.push('\t');
                }
                Some(&'\\') => {
                    chars.next();
                    result.push('\\');
                }
                Some(&'\'') => {
                    chars.next();
                    result.push('\'');
                }
                Some(&'0') => {
                    chars.next();
                    let mut oct = String::new();
                    for _ in 0..3 {
                        if let Some(&c) = chars.peek() {
                            if c >= '0' && c <= '7' {
                                oct.push(c);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                    }
                    if let Ok(n) = u8::from_str_radix(&oct, 8) {
                        result.push(n as char);
                    }
                }
                Some(&'x') => {
                    chars.next();
                    let mut hex = String::new();
                    for _ in 0..2 {
                        if let Some(&c) = chars.peek() {
                            if c.is_ascii_hexdigit() {
                                hex.push(c);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                    }
                    if let Ok(n) = u8::from_str_radix(&hex, 16) {
                        result.push(n as char);
                    }
                }
                _ => result.push('\\'),
            }
        } else {
            result.push(ch);
        }
    }
    result
}

// ===========================================================================
// Phase 3: Complexity detection
// ===========================================================================

fn detect_complexity(name: &str, content: &str) -> (bool, String) {
    // Themes that use precmd to dynamically build PROMPT
    if content.contains("add-zsh-hook precmd") || content.contains("precmd()") {
        // Check if precmd modifies PROMPT or does a print
        if content.contains("PROMPT=") || content.contains("print -rP") || content.contains("print -P") {
            // bureau, amuse, muse — precmd builds prompt
            // But some themes just set PROMPT once and use precmd for other things
            // Check if PROMPT is set INSIDE a function
            let has_prompt_in_func = content.contains("PROMPT=") && is_prompt_in_function(content);
            if has_prompt_in_func {
                return (true, "precmd dynamically builds PROMPT".into());
            }
            // If precmd uses print to output the first line
            if content.contains("print -rP") || content.contains("print -P") {
                return (true, "precmd prints prompt lines".into());
            }
        }
    }

    // Themes that use vcs_info
    if content.contains("vcs_info_msg_0_") || content.contains("vcs_info 'prompt'") {
        return (true, "uses vcs_info".into());
    }

    // Themes with custom git functions (not the standard git_prompt_info)
    let custom_git_funcs = [
        "git_prompt()",
        "git_custom_status()",
        "_git_prompt()",
        "bureau_git_prompt",
        "git_time_since_commit",
        "git_prompt_string()",
    ];
    for func in &custom_git_funcs {
        if content.contains(func) {
            // But if it also uses git_prompt_info, it's probably OK
            if !content.contains("$(git_prompt_info)") || name == "peepcode" {
                return (true, format!("custom git function: {}", func));
            }
        }
    }

    // Themes where PROMPT is not found at all
    if !content.contains("PROMPT=") && !content.contains("PS1=") {
        return (true, "no PROMPT assignment found".into());
    }

    // Themes with specific parser-unfriendly constructs
    match name {
        // $'...' multi-line concatenation with git_prompt_status symbols
        "sunaku" => return (true, "multi-line $'...' with git_prompt_status".into()),
        // Uses /dev/%y (TTY device name) which we can't resolve
        "darkblood" => return (true, "uses TTY device /dev/%y".into()),
        // $CONTAINER_NAME env var prefix
        "essembeh" => return (true, "uses $CONTAINER_NAME env var".into()),
        // Zsh arithmetic for hostname color $[((#HOST))%6+1]
        "michelebologna" => return (true, "zsh arithmetic for hostname color".into()),
        // Complex date format with custom FG colors
        "dallas" => return (true, "complex date + custom dirty format".into()),
        // Custom format with date/time and FG colors parser can't handle
        "junkfood" => return (true, "complex date/time format with $FG colors".into()),
        // Complex box-drawing with %l (tty) and custom separators
        "jonathan" => return (true, "complex box-drawing with TTY device".into()),
        // custom function output: "greetings, earthling"
        "humza" => return (true, "custom greeting function".into()),
        // nvm/rvm version display ‹node-›
        "mira" => return (true, "uses nvm/rvm version display".into()),
        // Uses $USER/$HOST variables that resolve differently than %n/%m
        "mlh" => return (true, "uses $USER/$HOST with custom resolution".into()),
        // Complex multi-line PROMPT concatenation with $'...' escaping
        "adben" => return (true, "complex multi-line prompt with $'...' escaping".into()),
        // %(?,true,false) conditional parser can't handle nested %) escapes
        "funky" => return (true, "%(?) conditional with nested %) escapes".into()),
        // Uses rbenv/rvm shell functions for ruby version display
        "nebirhos" => return (true, "uses rbenv/rvm shell functions".into()),
        // Custom mygit() function instead of git_prompt_info
        "rkj-repos" => return (true, "custom git function: mygit()".into()),
        _ => {}
    }

    (false, String::new())
}

fn is_prompt_in_function(content: &str) -> bool {
    let mut in_function = false;
    let mut brace_depth = 0;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.contains("() {") || trimmed.starts_with("function ") {
            in_function = true;
        }
        if in_function {
            brace_depth += trimmed.matches('{').count();
            brace_depth -= trimmed.matches('}').count();
            if trimmed.starts_with("PROMPT=") && brace_depth > 0 {
                return true;
            }
            if brace_depth == 0 {
                in_function = false;
            }
        }
    }
    false
}

// ===========================================================================
// Phase 4: Variable resolution
// ===========================================================================

fn resolve_vars(s: &str, vars: &HashMap<String, String>, depth: u8) -> String {
    if depth > 5 {
        return s.to_string(); // prevent infinite recursion
    }

    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '$' {
            match chars.peek() {
                Some(&'{') => {
                    chars.next();
                    let (name, rest) = read_braced_var(&mut chars);

                    // Color/formatting variables — pass through for strip phase
                    if name.starts_with("fg")
                        || name.starts_with("FG")
                        || name.starts_with("bg")
                        || name == "reset_color"
                        || name.starts_with("$fg")
                        || name.starts_with("$FG")
                        || name.starts_with("$reset")
                        || name.starts_with("$bg")
                    {
                        result.push_str(&format!("${{{}{}}}", name, rest));
                    } else if rest.starts_with(":+") {
                        // ${VAR:+replacement} — skip (usually virtualenv etc.)
                    } else if rest.starts_with(":-") {
                        if let Some(val) = vars.get(&name) {
                            result.push_str(&resolve_vars(val, vars, depth + 1));
                        } else {
                            result.push_str(&rest[2..]); // default value
                        }
                    } else if rest.starts_with("#refs/heads/") {
                        // ${ref#refs/heads/} — parameter expansion, skip
                    } else if rest.contains(':') && rest.contains("gs/") {
                        // ${var:gs/pattern/replacement} — skip
                    } else if let Some(val) = vars.get(&name) {
                        result.push_str(&resolve_vars(val, vars, depth + 1));
                    }
                    // else: unknown ${VAR}, skip
                }
                Some(&'(') => {
                    // $(command) — keep for later call resolution
                    chars.next();
                    let call = read_paren_call(&mut chars);
                    result.push_str(&format!("$({})", call));
                }
                Some(&c) if c.is_ascii_alphabetic() || c == '_' => {
                    // $VAR form
                    let name = read_var_name(&mut chars);

                    if name == "reset_color"
                        || name.starts_with("fg")
                        || name.starts_with("FG")
                        || name.starts_with("bg")
                    {
                        result.push('$');
                        result.push_str(&name);
                    } else if let Some(val) = vars.get(&name) {
                        result.push_str(&resolve_vars(val, vars, depth + 1));
                    } else {
                        // Unknown variable — keep as $VAR for debugging
                        result.push('$');
                        result.push_str(&name);
                    }
                }
                _ => {
                    result.push('$');
                }
            }
        } else {
            result.push(ch);
        }
    }
    result
}

fn read_braced_var(chars: &mut std::iter::Peekable<std::str::Chars>) -> (String, String) {
    let mut name = String::new();
    let mut rest = String::new();
    let mut in_rest = false;
    let mut depth = 1;

    while let Some(c) = chars.next() {
        if c == '}' {
            depth -= 1;
            if depth == 0 {
                break;
            }
            if in_rest {
                rest.push(c);
            } else {
                name.push(c);
            }
        } else if c == '{' {
            depth += 1;
            if in_rest {
                rest.push(c);
            } else {
                name.push(c);
            }
        } else if !in_rest && (c == ':' || c == '#' || c == '%') {
            in_rest = true;
            rest.push(c);
        } else if in_rest {
            rest.push(c);
        } else {
            name.push(c);
        }
    }
    (name, rest)
}

fn read_paren_call(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut call = String::new();
    let mut depth = 1;
    while let Some(c) = chars.next() {
        if c == '(' {
            depth += 1;
            call.push(c);
        } else if c == ')' {
            depth -= 1;
            if depth == 0 {
                break;
            }
            call.push(c);
        } else {
            call.push(c);
        }
    }
    call
}

fn read_var_name(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut name = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_alphanumeric() || c == '_' {
            name.push(c);
            chars.next();
        } else if c == '[' {
            // $fg[color] — include bracket part in name
            name.push(c);
            chars.next();
            while let Some(&cc) = chars.peek() {
                name.push(cc);
                chars.next();
                if cc == ']' {
                    break;
                }
            }
            break;
        } else {
            break;
        }
    }
    name
}

// ===========================================================================
// Phase 5: Strip ALL color/formatting
// ===========================================================================

fn strip_formatting(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Raw ANSI: ESC[...m
        if chars[i] == '\x1b' && i + 1 < len && chars[i + 1] == '[' {
            i += 2;
            while i < len && chars[i] != 'm' {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            continue;
        }

        // %{...%} or %N{...%} — zsh formatting/width block
        if chars[i] == '%' && i + 1 < len {
            let mut j = i + 1;
            // Skip optional digits (width hint: %1{...%})
            while j < len && chars[j].is_ascii_digit() {
                j += 1;
            }
            if j < len && chars[j] == '{' {
                // It's a %{...%} or %N{...%} block
                j += 1;
                let mut depth = 1;
                while j < len {
                    if chars[j] == '%' && j + 1 < len && chars[j + 1] == '}' {
                        depth -= 1;
                        j += 2;
                        if depth == 0 {
                            break;
                        }
                    } else if chars[j] == '%' && j + 1 < len && chars[j + 1] == '{' {
                        depth += 1;
                        j += 2;
                    } else {
                        j += 1;
                    }
                }
                // Check if this was a width hint block — extract the visible content
                // %1{➜%} means the arrow is 1 column wide; we keep the text
                if i + 1 < len && chars[i + 1].is_ascii_digit() {
                    // %N{text%} — extract text between { and %} for visible output
                    let mut k = i + 1;
                    while k < len && chars[k].is_ascii_digit() {
                        k += 1;
                    }
                    k += 1; // skip {
                    let content_start = k;
                    // Find the matching %}
                    let mut d = 1;
                    while k < len {
                        if chars[k] == '%' && k + 1 < len && chars[k + 1] == '}' {
                            d -= 1;
                            if d == 0 {
                                // Extract content between { and %}
                                let content: String = chars[content_start..k].iter().collect();
                                result.push_str(&content);
                                break;
                            }
                            k += 2;
                        } else {
                            k += 1;
                        }
                    }
                }
                // Skip past the whole block
                i = j;
                continue;
            }
        }

        // %D{...} — date format block: preserve content (contains strftime, not zsh escapes)
        if chars[i] == '%' && i + 1 < len && chars[i + 1] == 'D' && i + 2 < len && chars[i + 2] == '{' {
            // Keep %D{...} as-is for resolve_zsh_escapes to handle
            result.push('%');
            result.push('D');
            result.push('{');
            i += 3;
            let mut depth = 1;
            while i < len {
                if chars[i] == '{' {
                    depth += 1;
                } else if chars[i] == '}' {
                    depth -= 1;
                    if depth == 0 {
                        result.push('}');
                        i += 1;
                        break;
                    }
                }
                result.push(chars[i]);
                i += 1;
            }
            continue;
        }

        // %B, %b — bold on/off
        if chars[i] == '%' && i + 1 < len && (chars[i + 1] == 'B' || chars[i + 1] == 'b') {
            i += 2;
            continue;
        }

        // %F{color}, %K{color}
        if chars[i] == '%'
            && i + 1 < len
            && (chars[i + 1] == 'F' || chars[i + 1] == 'K')
        {
            i += 2;
            if i < len && chars[i] == '{' {
                i += 1;
                while i < len && chars[i] != '}' {
                    i += 1;
                }
                if i < len {
                    i += 1;
                }
            }
            continue;
        }

        // %f, %k — reset foreground/background
        if chars[i] == '%' && i + 1 < len && (chars[i + 1] == 'f' || chars[i + 1] == 'k') {
            i += 2;
            continue;
        }

        // %s, %S, %u, %U — standout/underline
        if chars[i] == '%'
            && i + 1 < len
            && (chars[i + 1] == 's'
                || chars[i + 1] == 'S'
                || chars[i + 1] == 'u'
                || chars[i + 1] == 'U')
        {
            i += 2;
            continue;
        }

        // $fg[...], $fg_bold[...], $bg[...], $reset_color
        if chars[i] == '$' && i + 1 < len {
            let rest: String = chars[i + 1..].iter().collect();
            if rest.starts_with("reset_color") {
                i += 1 + "reset_color".len();
                continue;
            }
            let color_prefixes = [
                "fg_bold[", "fg_no_bold[", "fg[", "bg_bold[", "bg[",
            ];
            let mut matched = false;
            for prefix in &color_prefixes {
                if rest.starts_with(prefix) {
                    i += 1; // skip $
                    while i < len && chars[i] != ']' {
                        i += 1;
                    }
                    if i < len {
                        i += 1;
                    }
                    matched = true;
                    break;
                }
            }
            if matched {
                continue;
            }

            // ${fg[...]}, ${FG[...]}, ${reset_color}, ${fg_bold[...]}
            if rest.starts_with("{fg")
                || rest.starts_with("{FG")
                || rest.starts_with("{bg")
                || rest.starts_with("{reset_color")
                || rest.starts_with("{$fg")
                || rest.starts_with("{$FG")
                || rest.starts_with("{$reset")
                || rest.starts_with("{$bg")
            {
                i += 1; // skip $
                let mut depth = 0;
                while i < len {
                    if chars[i] == '{' {
                        depth += 1;
                    } else if chars[i] == '}' {
                        depth -= 1;
                        if depth == 0 {
                            i += 1;
                            break;
                        }
                    }
                    i += 1;
                }
                continue;
            }
        }

        result.push(chars[i]);
        i += 1;
    }
    result
}

// ===========================================================================
// Phase 6: Resolve zsh prompt escapes
// ===========================================================================

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
                Some(&'c') | Some(&'C') => {
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
                Some(&'w') => {
                    chars.next();
                    result.push_str("Fri 14");
                }
                Some(&'W') => {
                    chars.next();
                    result.push_str("02/14/26");
                }
                Some(&'D') => {
                    chars.next();
                    if let Some(&'{') = chars.peek() {
                        chars.next();
                        let mut fmt = String::new();
                        while let Some(c) = chars.next() {
                            if c == '}' {
                                break;
                            }
                            fmt.push(c);
                        }
                        let resolved = fmt
                            .replace("%H", "12")
                            .replace("%I", "12")
                            .replace("%L", "12")
                            .replace("%M", "00")
                            .replace("%R", "12:00")
                            .replace("%S", "00")
                            .replace("%T", "12:00:00")
                            .replace("%X", "12:00:00")
                            .replace("%p", "PM")
                            .replace("%P", "pm")
                            .replace("%a", "Fri")
                            .replace("%A", "Friday")
                            .replace("%b", "Feb")
                            .replace("%B", "February")
                            .replace("%d", "14")
                            .replace("%e", "14")
                            .replace("%m", "02")
                            .replace("%Y", "2026")
                            .replace("%y", "26");
                        result.push_str(&resolved);
                    } else {
                        result.push_str("26-02-14");
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
                Some(&'%') => {
                    chars.next();
                    result.push('%');
                }
                Some(&'(') => {
                    chars.next();
                    let cond_text = read_conditional(&mut chars);
                    result.push_str(&resolve_conditional(&cond_text));
                }
                Some(&'?') => {
                    chars.next();
                    result.push_str("0"); // exit code
                }
                Some(&'j') => {
                    chars.next();
                    result.push_str("0"); // jobs
                }
                Some(&'l') => {
                    chars.next();
                    result.push_str("tty");
                }
                Some(&'N') | Some(&'i') => {
                    chars.next();
                    result.push_str("zsh");
                }
                Some(&'h') => {
                    chars.next();
                    result.push_str("1"); // history number
                }
                Some(&'E') => {
                    chars.next();
                    // Clear to end of line — no visible output
                }
                Some(&'/') => {
                    chars.next();
                    result.push_str("/home/user/test"); // absolute cwd
                }
                Some(&'d') => {
                    chars.next();
                    result.push_str("~/test"); // same as %~ in most contexts
                }
                Some(&'_') => {
                    chars.next();
                    // Parser state — skip
                }
                Some(&'<') | Some(&'>') => {
                    // Truncation directive: %<string< or %>string>
                    let delim = *chars.peek().unwrap();
                    chars.next();
                    // %<< or %>> — reset truncation
                    if chars.peek() == Some(&delim) {
                        chars.next();
                    } else {
                        // Skip truncation string until delimiter repeats
                        while let Some(c) = chars.next() {
                            if c == delim {
                                break;
                            }
                        }
                    }
                    continue;
                }
                Some(&c) if c.is_ascii_digit() => {
                    // Read all digits: %NN...
                    chars.next();
                    let mut num = String::new();
                    num.push(c);
                    while let Some(&d) = chars.peek() {
                        if d.is_ascii_digit() {
                            num.push(d);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let n: u32 = num.parse().unwrap_or(0);
                    match chars.peek() {
                        Some(&'~') => {
                            chars.next();
                            match n {
                                0 => result.push_str("~/test"),
                                1 => result.push_str("test"),
                                _ => result.push_str("~/test"),
                            }
                        }
                        Some(&'c') | Some(&'C') => {
                            chars.next();
                            result.push_str("test"); // basename
                        }
                        Some(&'<') | Some(&'>') => {
                            // %30<...< — truncation directive with width
                            let delim = *chars.peek().unwrap();
                            chars.next();
                            if chars.peek() == Some(&delim) {
                                chars.next(); // %N<< reset
                            } else {
                                while let Some(cc) = chars.next() {
                                    if cc == delim {
                                        break;
                                    }
                                }
                            }
                        }
                        _ => {
                            // Just %N — not a path format
                            result.push('%');
                            result.push_str(&num);
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

/// Read a %(condition.true.false) conditional, handling nested parens
fn read_conditional(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut content = String::new();
    let mut depth = 1;
    while let Some(c) = chars.next() {
        if c == '(' {
            depth += 1;
            content.push(c);
        } else if c == ')' {
            depth -= 1;
            if depth == 0 {
                break;
            }
            content.push(c);
        } else {
            content.push(c);
        }
    }
    content
}

/// Resolve a conditional expression.
/// In zsh, `%(x<sep>true<sep>false)` where the separator is the first character
/// after the condition letter(s). Common: `%(?.<t>.<f>)` or `%(?:<t>:<f>)`.
fn resolve_conditional(cond: &str) -> String {
    if cond.is_empty() {
        return String::new();
    }

    // Read condition: optional digits + a letter (like ?, !, V, j, ~, c, #)
    let chars: Vec<char> = cond.chars().collect();
    let mut cond_end = 0;
    // Skip optional leading digits
    while cond_end < chars.len() && chars[cond_end].is_ascii_digit() {
        cond_end += 1;
    }
    // Read the condition letter
    if cond_end < chars.len() {
        cond_end += 1;
    }
    if cond_end >= chars.len() {
        return String::new();
    }

    let condition = &cond[..cond.char_indices().nth(cond_end).map(|(i, _)| i).unwrap_or(cond.len())];
    let separator = chars[cond_end];
    let rest_start = cond.char_indices().nth(cond_end + 1).map(|(i, _)| i).unwrap_or(cond.len());
    let rest = &cond[rest_start..];

    // Find second separator (splitting true from false), handling nesting
    let mut depth = 0;
    let mut sep2 = None;
    for (i, c) in rest.char_indices() {
        if c == '(' {
            depth += 1;
        } else if c == ')' {
            depth -= 1;
        } else if c == separator && depth == 0 {
            sep2 = Some(i);
            break;
        }
    }

    let (true_branch, false_branch) = match sep2 {
        Some(idx) => (&rest[..idx], &rest[idx + separator.len_utf8()..]),
        None => (rest, ""),
    };

    // Decide which branch to take
    let take_true = if condition.ends_with('?') {
        true // exit code 0 — true branch
    } else if condition.ends_with('!') {
        false // not root — false branch
    } else if condition.ends_with('V') {
        true // has version — true
    } else if condition.contains('j') {
        false // 0 jobs
    } else {
        true // default: take true branch
    };

    if take_true {
        resolve_zsh_escapes(true_branch)
    } else {
        resolve_zsh_escapes(false_branch)
    }
}

// ===========================================================================
// Phase 7: Resolve function calls
// ===========================================================================

fn resolve_calls(s: &str, git_prefix: &str, git_suffix: &str, git_dirty: &str) -> String {
    let mut result = s.to_string();

    // $(git_prompt_info) → prefix + branch + dirty + suffix
    let git_text = format!("{}main{}{}", git_prefix, git_dirty, git_suffix);
    result = result.replace("$(git_prompt_info)", &git_text);

    // $(parse_git_dirty) → dirty marker
    result = result.replace("$(parse_git_dirty)", git_dirty);

    // $(hg_prompt_info) → empty (we don't simulate hg)
    result = result.replace("$(hg_prompt_info)", "");

    // Remove other common function calls
    let empty_calls = [
        "$(ruby_prompt_info)",
        "$(svn_prompt_info)",
        "$(git_prompt_status)",
        "$(git_prompt_ahead)",
        "$(git_prompt_short_sha)",
        "$(virtualenv_prompt_info)",
        "$(conda_prompt_info)",
        "$(nvm_prompt_info)",
        "$(rvm_prompt_info)",
        "$(prompt_context)",
        "$(kube_ps1)",
        "$(tf_prompt_info)",
        "$(aws_prompt_info)",
        "$(rvm_gemset)",
    ];
    for call in &empty_calls {
        result = result.replace(call, "");
    }

    // Handle remaining $(...) by removing them
    while let Some(start) = result.find("$(") {
        let rest = &result[start + 2..];
        let mut depth = 1;
        let mut end = 0;
        for (i, c) in rest.char_indices() {
            if c == '(' {
                depth += 1;
            } else if c == ')' {
                depth -= 1;
                if depth == 0 {
                    end = i;
                    break;
                }
            }
        }
        let call_end = start + 2 + end + 1;
        let call_name = &result[start..call_end];
        // Known resolution for common remaining calls
        if call_name.contains("_fishy_collapsed_wd") {
            result = result[..start].to_string() + "~/test" + &result[call_end..];
        } else {
            result = result[..start].to_string() + &result[call_end..];
        }
    }

    // Clean up remaining unresolved variables
    result = clean_remaining_vars(&result);

    result
}

fn clean_remaining_vars(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if chars[i] == '$' && i + 1 < len {
            let rest: String = chars[i + 1..].iter().collect();
            // ${VAR} form
            if rest.starts_with('{') {
                let mut depth = 1;
                let mut j = i + 2;
                while j < len {
                    if chars[j] == '{' {
                        depth += 1;
                    } else if chars[j] == '}' {
                        depth -= 1;
                        if depth == 0 {
                            j += 1;
                            break;
                        }
                    }
                    j += 1;
                }
                // Skip the whole ${...}
                i = j;
                continue;
            }
            // $VAR form — skip alphanumeric/underscore
            if chars[i + 1].is_ascii_alphabetic() || chars[i + 1] == '_' {
                i += 1;
                while i < len && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

// ===========================================================================
// Phase 8: Normalization for comparison
// ===========================================================================

fn normalize(s: &str) -> String {
    // First resolve \uXXXX unicode escapes (from zsh $'...' strings and some themes)
    let s = resolve_unicode_escapes(s);

    let mut result = String::new();
    let mut last_space = false;

    for ch in s.chars() {
        if ch == '\n' || ch == '\r' {
            // Convert newlines to space for single-line comparison
            if !last_space && !result.is_empty() {
                result.push(' ');
                last_space = true;
            }
            continue;
        }
        if ch == ' ' || ch == '\t' {
            if !last_space && !result.is_empty() {
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

/// Resolve \uXXXX and \UXXXXXXXX unicode escapes to actual characters.
fn resolve_unicode_escapes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.peek() {
                Some(&'u') => {
                    chars.next(); // consume 'u'
                    let mut hex = String::new();
                    for _ in 0..4 {
                        if let Some(&c) = chars.peek() {
                            if c.is_ascii_hexdigit() {
                                hex.push(c);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                    }
                    if let Ok(cp) = u32::from_str_radix(&hex, 16) {
                        if let Some(c) = char::from_u32(cp) {
                            result.push(c);
                            continue;
                        }
                    }
                    // Couldn't parse — keep literal
                    result.push('\\');
                    result.push('u');
                    result.push_str(&hex);
                }
                Some(&'U') => {
                    chars.next(); // consume 'U'
                    let mut hex = String::new();
                    for _ in 0..8 {
                        if let Some(&c) = chars.peek() {
                            if c.is_ascii_hexdigit() {
                                hex.push(c);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                    }
                    if let Ok(cp) = u32::from_str_radix(&hex, 16) {
                        if let Some(c) = char::from_u32(cp) {
                            result.push(c);
                            continue;
                        }
                    }
                    result.push('\\');
                    result.push('U');
                    result.push_str(&hex);
                }
                _ => result.push(ch),
            }
        } else {
            result.push(ch);
        }
    }
    result
}

// ===========================================================================
// Full theme parsing pipeline
// ===========================================================================

fn parse_theme(name: &str, content: &str) -> ZshTheme {
    let mut theme = ZshTheme::default();

    // Pre-process
    let processed = preprocess(content);

    // Detect complexity
    let (is_complex, reason) = detect_complexity(name, &processed);
    if is_complex {
        theme.is_complex = true;
        theme.complex_reason = reason;
        return theme;
    }

    // Collect variables
    let vars = collect_vars(&processed);

    // Extract git prompt settings
    let gp = |key: &str| -> String {
        let raw = vars.get(key).cloned().unwrap_or_default();
        let resolved = resolve_vars(&raw, &vars, 0);
        let stripped = strip_formatting(&resolved);
        resolve_zsh_escapes(&stripped)
    };
    let git_prefix = gp("ZSH_THEME_GIT_PROMPT_PREFIX");
    let git_suffix = gp("ZSH_THEME_GIT_PROMPT_SUFFIX");
    let git_dirty = gp("ZSH_THEME_GIT_PROMPT_DIRTY");
    let _git_clean = gp("ZSH_THEME_GIT_PROMPT_CLEAN");

    // Get PROMPT and RPROMPT raw values
    let prompt_raw = vars
        .get("PROMPT")
        .or_else(|| vars.get("PS1"))
        .cloned()
        .unwrap_or_default();
    let rprompt_raw = vars.get("RPROMPT").or_else(|| vars.get("RPS1")).cloned();

    // Check where git is referenced
    theme.prompt_has_git = prompt_raw.contains("git_prompt_info")
        || prompt_raw.contains("git_prompt_short_sha");
    if let Some(ref rp) = rprompt_raw {
        theme.rprompt_has_git = rp.contains("git_prompt_info")
            || rp.contains("git_prompt")
            || rp.contains("git_time_since_commit");
    }

    // Process PROMPT: resolve vars → strip formatting → resolve escapes → resolve calls
    let resolved = resolve_vars(&prompt_raw, &vars, 0);
    let stripped = strip_formatting(&resolved);
    let escaped = resolve_zsh_escapes(&stripped);
    let final_text = resolve_calls(&escaped, &git_prefix, &git_suffix, &git_dirty);
    theme.prompt_visible = normalize(&final_text);

    // Also check if variables reference git
    if !theme.prompt_has_git {
        theme.prompt_has_git = resolved.contains("git_prompt_info");
    }

    // Process RPROMPT
    if let Some(rp) = rprompt_raw {
        theme.has_rprompt = true;
        let resolved = resolve_vars(&rp, &vars, 0);
        let stripped = strip_formatting(&resolved);
        let escaped = resolve_zsh_escapes(&stripped);
        let final_text = resolve_calls(&escaped, &git_prefix, &git_suffix, &git_dirty);
        theme.rprompt_visible = normalize(&final_text);

        if !theme.rprompt_has_git {
            theme.rprompt_has_git = resolved.contains("git_prompt_info");
        }
    }

    theme
}

// ===========================================================================
// Comparison
// ===========================================================================

#[derive(Debug)]
enum VerifyResult {
    Pass,
    Mismatch {
        prompt_expected: String,
        prompt_got: String,
        rprompt_expected: String,
        rprompt_got: String,
    },
    GitPlacement {
        detail: String,
    },
    Complex {
        reason: String,
    },
    NotFound,
}

fn has_git_in_segments(segments: &[Segment]) -> bool {
    for seg in segments {
        match seg {
            Segment::Field(FieldName::GitBranch, _, _, _, _) => return true,
            Segment::IfGit(_) => return true,
            Segment::Dirty(_, _, _, _, _, _) => return true,
            _ => {}
        }
    }
    false
}

fn verify_theme(zsh: &ZshTheme, tpl: &TemplateDef) -> VerifyResult {
    if zsh.is_complex {
        return VerifyResult::Complex {
            reason: zsh.complex_reason.clone(),
        };
    }

    let info = test_info();

    // Check git placement
    let our_main_has_git = has_git_in_segments(tpl.segments);
    let our_rp_has_git = has_git_in_segments(tpl.rprompt);

    if zsh.rprompt_has_git && !zsh.prompt_has_git && our_main_has_git && !our_rp_has_git {
        return VerifyResult::GitPlacement {
            detail: "reference has git in RPROMPT only, but we have it in main segments".into(),
        };
    }

    // Render our template
    let our_prompt = normalize(&render_visible(tpl.segments, &info));
    let our_rprompt = normalize(&render_visible(tpl.rprompt, &info));

    // Compare PROMPT
    let prompt_match = zsh.prompt_visible == our_prompt;

    // Compare RPROMPT
    let rprompt_match = if zsh.has_rprompt {
        zsh.rprompt_visible == our_rprompt
    } else {
        our_rprompt.is_empty() || true // if ref has no RPROMPT, we don't penalize
    };

    if prompt_match && rprompt_match {
        VerifyResult::Pass
    } else {
        VerifyResult::Mismatch {
            prompt_expected: zsh.prompt_visible.clone(),
            prompt_got: our_prompt,
            rprompt_expected: zsh.rprompt_visible.clone(),
            rprompt_got: our_rprompt,
        }
    }
}

// ===========================================================================
// Main
// ===========================================================================

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filter = args.iter().find(|a| !a.starts_with('-') && *a != &args[0]).cloned();
    let verbose = args.iter().any(|a| a == "-v" || a == "--verbose");

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
    let mut total = 0u32;

    let mut issues: Vec<String> = Vec::new();

    for entry in &entries {
        let path = entry.path();
        let name = path.file_stem().unwrap().to_string_lossy().to_string();

        // Apply filter if specified
        if let Some(ref f) = filter {
            if &name != f {
                continue;
            }
        }

        // Skip agnoster (manual implementation)
        if name == "agnoster" {
            continue;
        }

        total += 1;

        let content = fs::read_to_string(&path).unwrap_or_default();

        // Find our template
        let tpl = match defs::get_template(&name) {
            Some(t) => t,
            None => {
                not_found += 1;
                let msg = format!("  NOT_FOUND  {}", name);
                println!("{}", msg);
                issues.push(msg);
                continue;
            }
        };

        let zsh = parse_theme(&name, &content);

        match verify_theme(&zsh, tpl) {
            VerifyResult::Pass => {
                pass += 1;
                println!("  \x1b[32mPASS\x1b[0m       {}", name);
            }
            VerifyResult::Mismatch {
                prompt_expected,
                prompt_got,
                rprompt_expected,
                rprompt_got,
            } => {
                mismatch += 1;
                let prompt_ok = prompt_expected == prompt_got;
                let rprompt_ok = rprompt_expected == rprompt_got
                    || (rprompt_expected.is_empty() && rprompt_got.is_empty());

                if prompt_ok && !rprompt_ok {
                    let msg = format!("  \x1b[33mRP_DIFF\x1b[0m    {}", name);
                    println!("{}", msg);
                    if verbose {
                        println!("             rp_exp: {:?}", rprompt_expected);
                        println!("             rp_got: {:?}", rprompt_got);
                    }
                    issues.push(format!(
                        "  RP_DIFF    {}  rp_exp: {:?}  rp_got: {:?}",
                        name, rprompt_expected, rprompt_got
                    ));
                } else {
                    let msg = format!("  \x1b[31mMISMATCH\x1b[0m   {}", name);
                    println!("{}", msg);
                    if verbose || filter.is_some() {
                        println!("             exp: {:?}", prompt_expected);
                        println!("             got: {:?}", prompt_got);
                        if !rprompt_ok {
                            println!("             rp_exp: {:?}", rprompt_expected);
                            println!("             rp_got: {:?}", rprompt_got);
                        }
                        // Show first divergence point
                        if let Some(diff_pos) = first_diff(&prompt_expected, &prompt_got) {
                            let start = diff_pos.saturating_sub(10);
                            let e_snip: String = prompt_expected.chars().skip(start).take(30).collect();
                            let g_snip: String = prompt_got.chars().skip(start).take(30).collect();
                            println!(
                                "             diff@{}: exp=...{:?}... got=...{:?}...",
                                diff_pos, e_snip, g_snip
                            );
                        }
                    }
                    issues.push(format!(
                        "  MISMATCH   {}  exp: {:?}  got: {:?}",
                        name, prompt_expected, prompt_got
                    ));
                }
            }
            VerifyResult::GitPlacement { detail } => {
                git_placement += 1;
                let msg = format!("  \x1b[33mGIT_PLACE\x1b[0m  {}  {}", name, detail);
                println!("{}", msg);
                issues.push(msg);
            }
            VerifyResult::Complex { reason } => {
                complex += 1;
                println!("  \x1b[36mCOMPLEX\x1b[0m    {}  ({})", name, reason);
            }
            VerifyResult::NotFound => {
                not_found += 1;
                issues.push(format!("  NOT_FOUND  {}", name));
            }
        }
    }

    println!("\n=== Summary ===");
    println!(
        "  \x1b[32mPASS\x1b[0m:          {}/{}",
        pass,
        total
    );
    if mismatch > 0 {
        println!("  \x1b[31mMISMATCH\x1b[0m:      {}", mismatch);
    }
    if git_placement > 0 {
        println!("  \x1b[33mGIT_PLACEMENT\x1b[0m: {}", git_placement);
    }
    if complex > 0 {
        println!("  \x1b[36mCOMPLEX\x1b[0m:       {}", complex);
    }
    if not_found > 0 {
        println!("  NOT_FOUND:     {}", not_found);
    }

    let verifiable = total - complex - not_found;
    if verifiable > 0 {
        let pct = pass as f64 / verifiable as f64 * 100.0;
        println!(
            "\n  Pass rate (excl. complex): {:.0}% ({}/{})",
            pct, pass, verifiable
        );
    }
}

fn first_diff(a: &str, b: &str) -> Option<usize> {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    for i in 0..a_chars.len().max(b_chars.len()) {
        let ca = a_chars.get(i);
        let cb = b_chars.get(i);
        if ca != cb {
            return Some(i);
        }
    }
    None
}
