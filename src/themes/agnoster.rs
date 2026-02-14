use crate::color::{Color, RESET};
use crate::info::StatusInfo;
use crate::themes::Theme;

pub struct Agnoster;

// Powerline segment separator (U+E0B0)
const SEP: &str = "\u{e0b0}";
// Powerline branch symbol (U+E0A0)
const BRANCH: &str = "\u{e0a0}";

struct SegmentBuilder {
    buf: String,
    current_bg: Option<Color>,
}

impl SegmentBuilder {
    fn new() -> Self {
        Self {
            buf: String::new(),
            current_bg: None,
        }
    }

    fn segment(&mut self, bg: Color, fg: Color, text: &str) {
        if let Some(prev_bg) = self.current_bg {
            if prev_bg != bg {
                // Transition: prev_bg fg color on new bg, then separator
                self.buf
                    .push_str(&format!(" {}{}{}", bg.bg(), prev_bg.fg(), SEP));
                self.buf.push_str(&format!("{} ", fg.fg()));
            } else {
                // Same bg, just continue
                self.buf.push_str(&format!("{} ", fg.fg()));
            }
        } else {
            // First segment
            self.buf
                .push_str(&format!("{}{} ", bg.bg(), fg.fg()));
        }
        self.buf.push_str(text);
        self.current_bg = Some(bg);
    }

    fn finish(mut self) -> String {
        if let Some(bg) = self.current_bg {
            // Close: separator with prev bg as fg on default bg
            self.buf
                .push_str(&format!(" {}{}{}", RESET, bg.fg(), SEP));
            self.buf.push_str(RESET);
        }
        self.buf
    }
}

impl Theme for Agnoster {
    fn name(&self) -> &'static str {
        "agnoster"
    }

    fn format(&self, info: &StatusInfo) -> String {
        let mut sb = SegmentBuilder::new();

        // Context segment: user@host (black bg, default fg)
        sb.segment(
            Color::Black,
            Color::White,
            &format!("{}@{}", info.user, info.short_hostname),
        );

        // Dir segment: cwd (blue bg, black fg)
        sb.segment(Color::Blue, Color::Black, &info.cwd);

        // Git segment (green bg clean / yellow bg dirty)
        if let Some(ref branch) = info.git_branch {
            let dirty = info.git_dirty == Some(true);
            let (bg, fg) = if dirty {
                (Color::Yellow, Color::Black)
            } else {
                (Color::Green, Color::Black)
            };
            sb.segment(bg, fg, &format!("{} {}", BRANCH, branch));
        }

        sb.finish()
    }
}
