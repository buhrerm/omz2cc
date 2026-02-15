use std::env;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

/// Data parsed from Claude Code JSON piped to stdin
pub struct StdinData {
    pub model_id: Option<String>,
    pub model_display_name: Option<String>,
}

impl StdinData {
    pub fn read() -> Option<Self> {
        let mut input = String::new();
        std::io::stdin()
            .take(1_000_000)
            .read_to_string(&mut input)
            .ok()?;
        if input.trim().is_empty() {
            return None;
        }
        // Minimal JSON parsing without serde - just extract the fields we need
        let model_id = extract_json_string(&input, "id");
        let model_display_name = extract_json_string(&input, "display_name");
        Some(Self {
            model_id,
            model_display_name,
        })
    }
}

/// Extract a string value from JSON by key (simple, no serde dependency).
/// Handles escaped quotes within string values.
fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\"", key);
    let idx = json.find(&pattern)?;
    let after_key = &json[idx + pattern.len()..];
    // Skip whitespace and colon
    let after_colon = after_key.trim_start().strip_prefix(':')?;
    let after_colon = after_colon.trim_start();
    // Expect opening quote
    let after_quote = after_colon.strip_prefix('"')?;
    // Find closing quote, skipping escaped quotes
    let mut result = String::new();
    let mut chars = after_quote.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '\\' => {
                if let Some(escaped) = chars.next() {
                    match escaped {
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        'b' => result.push('\u{08}'),
                        'f' => result.push('\u{0C}'),
                        '/' => result.push('/'),
                        // \uXXXX intentionally unsupported — not needed for model fields
                        _ => {
                            result.push('\\');
                            result.push(escaped);
                        }
                    }
                }
            }
            '"' => return Some(result),
            _ => result.push(ch),
        }
    }
    None
}

pub struct StatusInfo {
    pub user: String,
    pub hostname: String,
    pub short_hostname: String,
    pub cwd: String,
    pub cwd_basename: String,
    pub git_branch: Option<String>,
    pub git_dirty: Option<bool>,
    pub git_ahead: Option<i32>,
    pub git_behind: Option<i32>,
    pub git_action: Option<String>,
    pub exit_code: i32,
    pub time: String,
}

impl StatusInfo {
    pub fn gather() -> Self {
        let user = env::var("USER")
            .or_else(|_| env::var("LOGNAME"))
            .unwrap_or_else(|_| String::from("unknown"));

        let hostname = hostname();
        let short_hostname = hostname
            .split('.')
            .next()
            .unwrap_or(&hostname)
            .to_string();
        let (cwd, cwd_basename) = cwd_info();
        let (git_branch, git_dirty) = git_info();
        let git_action = git_action();
        let time = current_time("%H:%M:%S");

        Self {
            user,
            hostname,
            short_hostname,
            cwd,
            cwd_basename,
            git_branch,
            git_dirty,
            git_ahead: None,
            git_behind: None,
            git_action,
            exit_code: 0,
            time,
        }
    }

    /// Return the last `n` path components of cwd (like zsh's `%N~`).
    /// If the path has fewer components, returns the full cwd.
    pub fn cwd_truncated(&self, n: u8) -> String {
        let n = n as usize;
        if n == 0 {
            return self.cwd.clone();
        }
        // Handle ~ prefix: split off ~ then count from right
        let (prefix, path) = if self.cwd.starts_with('~') {
            ("~", &self.cwd[1..])
        } else {
            ("", self.cwd.as_str())
        };
        let components: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if components.len() <= n {
            return self.cwd.clone();
        }
        let truncated: Vec<&str> = components[components.len() - n..].to_vec();
        if prefix == "~" && components.len() == n {
            format!("~/{}", truncated.join("/"))
        } else {
            truncated.join("/")
        }
    }

    pub fn apply_overrides(&mut self, overrides: &[String], stdin: &Option<StdinData>) {
        for entry in overrides {
            if let Some((key, val)) = entry.split_once('=') {
                let resolved = resolve_value(val, stdin);
                match key {
                    "user" => self.user = resolved,
                    "hostname" => self.hostname = resolved,
                    "cwd" => {
                        self.cwd_basename = resolved
                            .rsplit('/')
                            .next()
                            .unwrap_or(&resolved)
                            .to_string();
                        self.cwd = resolved;
                    }
                    "git_branch" => self.git_branch = Some(resolved),
                    "time" => self.time = resolved,
                    _ => eprintln!("Unknown field: {}", key),
                }
            }
        }
    }
}

/// Resolve special value tokens:
///   @model        -> pretty model from stdin JSON (e.g. "Opus 4.6")
///   @model-id     -> raw model ID from stdin JSON (e.g. "claude-opus-4-6")
///   @time         -> current time in default format (HH:MM:SS)
///   @time:FORMAT  -> current time with custom strftime format (e.g. @time:%I:%M %p)
///   plain text    -> used as-is
fn resolve_value(val: &str, stdin: &Option<StdinData>) -> String {
    match val {
        "@model" => {
            if let Some(data) = stdin {
                if let Some(ref id) = data.model_id {
                    return pretty_model(id);
                }
                if let Some(ref name) = data.model_display_name {
                    return name.clone();
                }
            }
            "unknown".to_string()
        }
        "@model-id" => {
            if let Some(data) = stdin {
                if let Some(ref id) = data.model_id {
                    return id.clone();
                }
            }
            "unknown".to_string()
        }
        "@time" => current_time("%H:%M:%S"),
        _ if val.starts_with("@time:") => {
            let fmt = &val["@time:".len()..];
            current_time(fmt)
        }
        _ => val.to_string(),
    }
}

/// Convert a Claude model ID to a friendly display name.
///   claude-opus-4-6           -> "Opus 4.6"
///   claude-sonnet-4-5-20250929 -> "Sonnet 4.5"
///   claude-haiku-4-5-20251001  -> "Haiku 4.5"
///   anything-else             -> returned as-is
fn pretty_model(raw: &str) -> String {
    // Strip "claude-" prefix if present
    let s = raw.strip_prefix("claude-").unwrap_or(raw);

    // Known model families
    for family in &["opus", "sonnet", "haiku"] {
        if let Some(rest) = s.strip_prefix(family) {
            let name = capitalize(family);
            // rest is like "-4-6" or "-4-5-20250929"
            let rest = rest.strip_prefix('-').unwrap_or(rest);
            let version = parse_version(rest);
            if version.is_empty() {
                return name;
            }
            return format!("{} {}", name, version);
        }
    }

    raw.to_string()
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
    }
}

/// Parse version from strings like "4-6", "4-5-20250929"
/// Takes first two numeric segments and joins with "."
fn parse_version(s: &str) -> String {
    let parts: Vec<&str> = s.split('-').collect();
    let nums: Vec<&str> = parts
        .iter()
        .take_while(|p| p.len() <= 2 && p.chars().all(|c| c.is_ascii_digit()))
        .copied()
        .collect();
    nums.join(".")
}

fn hostname() -> String {
    std::fs::read_to_string("/etc/hostname")
        .map(|s| s.trim().to_string())
        .or_else(|_| {
            Command::new("hostname")
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        })
        .unwrap_or_else(|_| String::from("localhost"))
}

fn cwd_info() -> (String, String) {
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let home = env::var("HOME").unwrap_or_default();

    let cwd_str = cwd.to_string_lossy().to_string();
    let display = if !home.is_empty() && cwd_str.starts_with(&home) {
        format!("~{}", &cwd_str[home.len()..])
    } else {
        cwd_str
    };

    let basename = cwd
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| display.clone());

    (display, basename)
}

fn git_info() -> (Option<String>, Option<bool>) {
    let branch = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());

    if branch.is_none() {
        return (None, None);
    }

    let dirty = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .ok()
        .map(|o| !o.stdout.is_empty());

    (branch, dirty)
}

fn git_action() -> Option<String> {
    let git_dir = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())?;

    let path = std::path::Path::new(&git_dir);
    if path.join("rebase-merge").exists() || path.join("rebase-apply").exists() {
        Some("rebase".to_string())
    } else if path.join("MERGE_HEAD").exists() {
        Some("merge".to_string())
    } else if path.join("CHERRY_PICK_HEAD").exists() {
        Some("cherry-pick".to_string())
    } else if path.join("BISECT_LOG").exists() {
        Some("bisect".to_string())
    } else {
        None
    }
}

fn current_time(fmt: &str) -> String {
    let output = Command::new("date")
        .arg(format!("+{}", fmt))
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());

    output.unwrap_or_else(|| String::from("00:00:00"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_extract_simple() {
        let json = r#"{"id": "claude-opus-4-6"}"#;
        assert_eq!(
            extract_json_string(json, "id"),
            Some("claude-opus-4-6".into())
        );
    }

    #[test]
    fn json_extract_second_field() {
        let json = r#"{"id": "claude-opus-4-6", "display_name": "Claude"}"#;
        assert_eq!(
            extract_json_string(json, "display_name"),
            Some("Claude".into())
        );
    }

    #[test]
    fn json_extract_escaped_quotes() {
        let json = r#"{"name": "say \"hello\""}"#;
        assert_eq!(
            extract_json_string(json, "name"),
            Some(r#"say "hello""#.into())
        );
    }

    #[test]
    fn json_extract_escaped_backslash() {
        let json = r#"{"path": "C:\\Users\\test"}"#;
        assert_eq!(
            extract_json_string(json, "path"),
            Some(r"C:\Users\test".into())
        );
    }

    #[test]
    fn json_extract_missing_key() {
        let json = r#"{"id": "value"}"#;
        assert_eq!(extract_json_string(json, "missing"), None);
    }

    #[test]
    fn json_extract_unterminated_string() {
        let json = r#"{"id": "unterminated}"#;
        assert_eq!(extract_json_string(json, "id"), None);
    }

    #[test]
    fn json_extract_empty_value() {
        let json = r#"{"id": ""}"#;
        assert_eq!(extract_json_string(json, "id"), Some(String::new()));
    }
}
