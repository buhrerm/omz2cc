use crate::color::*;
use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Ys;

impl Theme for Ys {
    fn name(&self) -> &'static str {
        "ys"
    }

    // # mike @ zulu in ~/Workspace/omz2cc on git:main o [15:35:04]
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = format!(
            "{} {} {} {} {} {}",
            colored("#", BLUE),
            bold(&info.user, CYAN),
            colored("@", WHITE),
            colored(&info.hostname, GREEN),
            colored("in", WHITE),
            bold(&info.cwd, YELLOW),
        );

        if let Some(ref branch) = info.git_branch {
            let dirty_sym = if info.git_dirty == Some(true) { "x" } else { "o" };
            let dirty_color = if info.git_dirty == Some(true) { RED } else { GREEN };
            s.push_str(&format!(
                " {} {}{}",
                colored("on", WHITE),
                colored(&format!("git:{}", branch), CYAN),
                colored(&format!(" {}", dirty_sym), dirty_color),
            ));
        }

        s.push_str(&format!(" {}", colored(&format!("[{}]", info.time), GRAY)));
        s
    }
}
