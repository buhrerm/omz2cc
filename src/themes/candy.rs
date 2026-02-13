use crate::color::*;
use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Candy;

impl Theme for Candy {
    fn name(&self) -> &'static str {
        "candy"
    }

    // mike@zulu [15:35:04] [~/Workspace/omz2cc] [main]
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = format!(
            "{}{}{} {} {}",
            colored(&info.user, GREEN),
            colored("@", WHITE),
            colored(&info.hostname, BLUE),
            colored(&format!("[{}]", info.time), GRAY),
            bold(&format!("[{}]", info.cwd), YELLOW),
        );

        if let Some(ref branch) = info.git_branch {
            s.push_str(&format!(" {}", colored(&format!("[{}]", branch), CYAN)));
        }

        s
    }
}
