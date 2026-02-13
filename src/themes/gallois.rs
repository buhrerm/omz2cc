use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Gallois;

impl Theme for Gallois {
    fn name(&self) -> &'static str {
        "gallois"
    }

    // ~/Workspace/omz2cc git:(main) ✓
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = info.cwd.clone();

        if let Some(ref branch) = info.git_branch {
            s.push_str(&format!(" git:({})", branch));
            s.push_str(&format!(" {}", info.git_status_symbol()));
        }

        s
    }
}
