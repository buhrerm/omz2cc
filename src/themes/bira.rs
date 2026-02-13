use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Bira;

impl Theme for Bira {
    fn name(&self) -> &'static str {
        "bira"
    }

    // mike@zulu:~/Workspace/omz2cc on git:main ✓
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = format!("{}@{}:{}", info.user, info.hostname, info.cwd);

        if let Some(ref branch) = info.git_branch {
            s.push_str(&format!(" on git:{} {}", branch, info.git_status_symbol()));
        }

        s
    }
}
