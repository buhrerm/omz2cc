use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct AfMagic;

impl Theme for AfMagic {
    fn name(&self) -> &'static str {
        "af-magic"
    }

    // ~/Workspace/omz2cc on git:main ✓
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = info.cwd.clone();

        if let Some(ref branch) = info.git_branch {
            s.push_str(&format!(" on git:{} {}", branch, info.git_status_symbol()));
        }

        s
    }
}
