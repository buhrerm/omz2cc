use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Agnoster;

impl Theme for Agnoster {
    fn name(&self) -> &'static str {
        "agnoster"
    }

    // mike@zulu ~/Workspace/omz2cc  main ✓
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = format!("{}@{} {}", info.user, info.hostname, info.cwd);

        if let Some(ref branch) = info.git_branch {
            s.push_str(&format!("  {} {}", branch, info.git_status_symbol()));
        }

        s
    }
}
