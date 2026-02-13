use crate::color::*;
use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Robbyrussell;

impl Theme for Robbyrussell {
    fn name(&self) -> &'static str {
        "robbyrussell"
    }

    // ➜ omz2cc git:(main) ✓
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = format!(
            "{} {}",
            colored("➜", GREEN),
            colored(&info.cwd_basename, CYAN),
        );

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
