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
        std::io::stdin().read_to_string(&mut input).ok()?;
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

/// Extract a string value from JSON by key (simple, no serde dependency)
fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\"", key);
    let idx = json.find(&pattern)?;
    let after_key = &json[idx + pattern.len()..];
    // Skip whitespace and colon
    let after_colon = after_key.trim_start().strip_prefix(':')?;
    let after_colon = after_colon.trim_start();
    // Expect opening quote
    let after_quote = after_colon.strip_prefix('"')?;
    let end = after_quote.find('"')?;
    Some(after_quote[..end].to_string())
}

pub struct StatusInfo {
    pub user: String,
    pub hostname: String,
    pub cwd: String,
    pub cwd_basename: String,
    pub git_branch: Option<String>,
    pub git_dirty: Option<bool>,
    pub time: String,
}

impl StatusInfo {
    pub fn gather() -> Self {
        let user = env::var("USER")
            .or_else(|_| env::var("LOGNAME"))
            .unwrap_or_else(|_| String::from("unknown"));

        let hostname = hostname();
        let (cwd, cwd_basename) = cwd_info();
        let (git_branch, git_dirty) = git_info();
        let time = current_time();

        Self {
            user,
            hostname,
            cwd,
            cwd_basename,
            git_branch,
            git_dirty,
            time,
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
///   @model     -> pretty model from stdin JSON (e.g. "Opus 4.6")
///   @model-id  -> raw model ID from stdin JSON (e.g. "claude-opus-4-6")
///   plain text -> used as-is
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

fn current_time() -> String {
    let output = Command::new("date")
        .arg("+%H:%M:%S")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());

    output.unwrap_or_else(|| String::from("00:00:00"))
}
