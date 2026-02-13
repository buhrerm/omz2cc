use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Bureau;

impl Theme for Bureau {
    fn name(&self) -> &'static str {
        "bureau"
    }

    // mike ~/Workspace/omz2cc [15:35:04] git:main ✓
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = format!("{} {} [{}]", info.user, info.cwd, info.time);

        if let Some(ref branch) = info.git_branch {
            s.push_str(&format!(" git:{} {}", branch, info.git_status_symbol()));
        }

        s
    }
}
