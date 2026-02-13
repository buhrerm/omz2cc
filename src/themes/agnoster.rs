use crate::color::{self, Color};
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
            color::colored(&format!("{}@{}", info.user, info.hostname), Color::White),
            color::bold(&info.cwd, Color::Blue),
        );

        if let Some(ref branch) = info.git_branch {
            let (sym, clr) = if info.git_dirty == Some(true) {
                (" ✗", Color::Yellow)
            } else {
                (" ✓", Color::Green)
            };
            s.push_str(&format!(
                "  {}{}",
                color::colored(branch, Color::White),
                color::colored(sym, clr),
            ));
        }

        s
    }
}
