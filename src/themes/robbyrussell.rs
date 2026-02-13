use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Robbyrussell;

impl Theme for Robbyrussell {
    fn name(&self) -> &'static str {
        "robbyrussell"
    }

    // ➜ omz2cc git:(main) ✓
    fn format(&self, info: &StatusInfo) -> String {
        let mut s = format!("➜ {}", info.cwd_basename);

        if let Some(ref branch) = info.git_branch {
            s.push_str(&format!(" git:({})", branch));
            s.push_str(&format!(" {}", info.git_status_symbol()));
        }

        s
    }
}
