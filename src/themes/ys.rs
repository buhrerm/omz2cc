use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Ys;

impl Theme for Ys {
    fn name(&self) -> &'static str {
        "ys"
    }

    // # mike @ zulu in ~/Workspace/omz2cc on git:main o [15:35:04]
    fn format(&self, info: &StatusInfo) -> String {
        let mut parts = vec![
            format!("# {} @ {} in {}", info.user, info.hostname, info.cwd),
        ];

        if let Some(ref branch) = info.git_branch {
            let dirty = if info.git_dirty == Some(true) { " x" } else { " o" };
            parts.push(format!("on git:{}{}", branch, dirty));
        }

        parts.push(format!("[{}]", info.time));

        parts.join(" ")
    }
}
