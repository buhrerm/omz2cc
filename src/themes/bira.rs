use crate::color::*;
use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Bira;

impl Theme for Bira {
    fn name(&self) -> &'static str {
        "bira"
    }

    // mike@zulu:~/Workspace/omz2cc on git:main ✓
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = format!(
            "{}{}{}",
            colored(&info.user, GREEN),
            colored("@", WHITE),
            colored(&format!("{}:{}", info.hostname, info.cwd), GREEN),
        );

        if let Some(ref branch) = info.git_branch {
            let (sym, clr) = if info.git_dirty == Some(true) {
                (" ✗", YELLOW)
            } else {
                (" ✓", GREEN)
            };
            s.push_str(&format!(
                " {} {}{}",
                colored("on", WHITE),
                colored(&format!("git:{}", branch), MAGENTA),
                colored(sym, clr),
            ));
        }

        s
    }
}
