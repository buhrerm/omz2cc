use crate::color::*;
use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Bureau;

impl Theme for Bureau {
    fn name(&self) -> &'static str {
        "bureau"
    }

    // mike ~/Workspace/omz2cc [15:35:04] git:main ✓
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = format!(
            "{} {} {}",
            colored(&info.user, CYAN),
            bold(&info.cwd, BLUE),
            colored(&format!("[{}]", info.time), GRAY),
        );

        if let Some(ref branch) = info.git_branch {
            let (sym, clr) = if info.git_dirty == Some(true) {
                (" ✗", YELLOW)
            } else {
                (" ✓", GREEN)
            };
            s.push_str(&format!(
                " {}{}",
                colored(&format!("git:{}", branch), MAGENTA),
                colored(sym, clr),
            ));
        }

        s
    }
}
