use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Maran;

impl Theme for Maran {
    fn name(&self) -> &'static str {
        "maran"
    }

    // mike@zulu ~/Workspace/omz2cc git:(main)
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = format!("{}@{} {}", info.user, info.hostname, info.cwd);

        if let Some(ref branch) = info.git_branch {
            s.push_str(&format!(" git:({})", branch));
        }

        s
    }
}
