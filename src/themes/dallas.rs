use crate::color::*;
use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Dallas;

impl Theme for Dallas {
    fn name(&self) -> &'static str {
        "dallas"
    }

    // [15:35:04] zulu:~/Workspace/omz2cc @main mike
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = format!(
            "{} {}",
            colored(&format!("[{}]", info.time), GRAY),
            colored(&format!("{}:{}", info.hostname, info.cwd), CYAN),
        );

        if let Some(ref branch) = info.git_branch {
            s.push_str(&format!(" {}", colored(&format!("@{}", branch), GREEN)));
        }

        s.push_str(&format!(" {}", colored(&info.user, YELLOW)));
        s
    }
}
