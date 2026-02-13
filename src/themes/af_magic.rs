use crate::color::*;
use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct AfMagic;

impl Theme for AfMagic {
    fn name(&self) -> &'static str {
        "af-magic"
    }

    // ~/Workspace/omz2cc on git:main ✓
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = bold(&info.cwd, BLUE);

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
