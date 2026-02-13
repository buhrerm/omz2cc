use std::env;
use std::path::PathBuf;
use std::process::Command;

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
