use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Candy;

impl Theme for Candy {
    fn name(&self) -> &'static str {
        "candy"
    }

    // mike@zulu [15:35:04] [~/Workspace/omz2cc] [main]
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = format!(
            "{}@{} [{}] [{}]",
            info.user, info.hostname, info.time, info.cwd
        );

        if let Some(ref branch) = info.git_branch {
            s.push_str(&format!(" [{}]", branch));
        }

        s
    }
}
