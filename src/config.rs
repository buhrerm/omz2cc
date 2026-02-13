use std::collections::HashMap;

/// Load mappings from ~/.config/omz2cc/mappings.conf
///
/// File format (one per line):
///   user=@model
///   hostname=myhost
///   time=@time:%I:%M %p
///
/// Lines starting with # are comments. Empty lines are ignored.
pub fn load_mappings() -> Vec<(String, String)> {
    let path = match config_path() {
        Some(p) => p,
        None => return Vec::new(),
    };

    let contents = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    parse_mappings(&contents)
}

pub fn config_dir() -> String {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        if !xdg.is_empty() {
            return format!("{}/omz2cc", xdg);
        }
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!("{}/.config/omz2cc", home)
}

fn config_path() -> Option<String> {
    Some(format!("{}/mappings.conf", config_dir()))
}

fn parse_mappings(contents: &str) -> Vec<(String, String)> {
    contents
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                return None;
            }
            let (key, val) = trimmed.split_once('=')?;
            Some((key.trim().to_string(), val.trim().to_string()))
        })
        .collect()
}

/// Merge config mappings with CLI --set overrides.
/// CLI overrides take precedence over config file mappings.
pub fn merge_mappings(config: Vec<(String, String)>, cli_overrides: &[String]) -> Vec<String> {
    let mut map: HashMap<String, String> = HashMap::new();

    // Config file first (lower priority)
    for (k, v) in config {
        map.insert(k, v);
    }

    // CLI --set overrides (higher priority)
    for entry in cli_overrides {
        if let Some((key, val)) = entry.split_once('=') {
            map.insert(key.to_string(), val.to_string());
        }
    }

    // Convert back to key=value strings for apply_overrides
    map.into_iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mappings() {
        let input = r#"
# Map user to model name
user=@model
hostname=myhost

# Blank lines are ignored
time=@time:%I:%M %p
"#;
        let result = parse_mappings(input);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&("user".to_string(), "@model".to_string())));
        assert!(result.contains(&("hostname".to_string(), "myhost".to_string())));
        assert!(result.contains(&("time".to_string(), "@time:%I:%M %p".to_string())));
    }

    #[test]
    fn test_merge_cli_overrides_config() {
        let config = vec![
            ("user".to_string(), "@model".to_string()),
            ("hostname".to_string(), "from-config".to_string()),
        ];
        let cli = vec!["hostname=from-cli".to_string()];
        let merged = merge_mappings(config, &cli);
        // CLI should override config for hostname
        assert!(merged.contains(&"hostname=from-cli".to_string()));
        assert!(merged.contains(&"user=@model".to_string()));
        assert!(!merged.iter().any(|s| s == "hostname=from-config"));
    }
}
