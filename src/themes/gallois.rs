use crate::color::*;
use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Gallois;

impl Theme for Gallois {
    fn name(&self) -> &'static str {
        "gallois"
    }

    // ~/Workspace/omz2cc git:(main) ✓
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = colored(&info.cwd, CYAN);

        if let Some(ref branch) = info.git_branch {
            s.push_str(&format!(
                " {}{}{}",
                colored("git:(", BLUE),
                colored(branch, RED),
                colored(")", BLUE),
            ));
            let (sym, clr) = if info.git_dirty == Some(true) {
                ("✗", YELLOW)
            } else {
                ("✓", GREEN)
            };
            s.push_str(&format!(" {}", colored(sym, clr)));
        }

        s
    }
}
