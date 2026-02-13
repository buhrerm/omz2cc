use crate::color::*;
use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Maran;

impl Theme for Maran {
    fn name(&self) -> &'static str {
        "maran"
    }

    // mike@zulu ~/Workspace/omz2cc git:(main)
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = format!(
            "{}{}{} {}",
            colored(&info.user, GREEN),
            colored("@", WHITE),
            colored(&info.hostname, BLUE),
            bold(&info.cwd, YELLOW),
        );

        if let Some(ref branch) = info.git_branch {
            s.push_str(&format!(
                " {}{}{}",
                colored("git:(", BLUE),
                colored(branch, RED),
                colored(")", BLUE),
            ));
        }

        s
    }
}
