use crate::color::*;
use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Agnoster;

impl Theme for Agnoster {
    fn name(&self) -> &'static str {
        "agnoster"
    }

    // mike@zulu ~/Workspace/omz2cc  main ✓
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = format!(
            "{} {}",
            colored(&format!("{}@{}", info.user, info.hostname), WHITE),
            bold(&info.cwd, BLUE),
        );

        if let Some(ref branch) = info.git_branch {
            let (sym, clr) = if info.git_dirty == Some(true) {
                (" ✗", YELLOW)
            } else {
                (" ✓", GREEN)
            };
            s.push_str(&format!(
                "  {}{}",
                colored(branch, WHITE),
                colored(sym, clr),
            ));
        }

        s
    }
}
