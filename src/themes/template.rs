use crate::color::{self, Color};
use crate::info::StatusInfo;
use crate::themes::Theme;

// ---------------------------------------------------------------------------
// Segment DSL types
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub enum FieldName {
    User,
    Hostname,
    ShortHostname,
    Cwd,
    CwdBasename,
    CwdTruncated(u8),
    GitBranch,
    Time,
    UserAtHost,
    HostColonCwd,
}

pub enum Segment {
    /// Plain uncolored text
    Text(&'static str),
    /// Colored literal (optionally bold)
    Lit(&'static str, Color, bool),
    /// Colored field value with optional prefix/suffix (optionally bold)
    Field(FieldName, Color, bool, &'static str, &'static str),
    /// Conditional block: only rendered when inside a git repo
    IfGit(&'static [Segment]),
    /// Conditional block: only rendered when NOT in a git repo
    IfNotGit(&'static [Segment]),
    /// Clean/dirty symbol with respective colors (clean_str, dirty_str, clean_color, dirty_color, clean_bold, dirty_bold)
    Dirty(&'static str, &'static str, Color, Color, bool, bool),
}

// ---------------------------------------------------------------------------
// Const helper functions — keep theme definitions concise
// ---------------------------------------------------------------------------

pub const fn text(s: &'static str) -> Segment {
    Segment::Text(s)
}

pub const fn lit(s: &'static str, color: Color) -> Segment {
    Segment::Lit(s, color, false)
}

pub const fn lit_bold(s: &'static str, color: Color) -> Segment {
    Segment::Lit(s, color, true)
}

pub const fn field(name: FieldName, color: Color) -> Segment {
    Segment::Field(name, color, false, "", "")
}

pub const fn field_bold(name: FieldName, color: Color) -> Segment {
    Segment::Field(name, color, true, "", "")
}

pub const fn field_fmt(
    name: FieldName,
    color: Color,
    prefix: &'static str,
    suffix: &'static str,
) -> Segment {
    Segment::Field(name, color, false, prefix, suffix)
}

pub const fn field_fmt_bold(
    name: FieldName,
    color: Color,
    prefix: &'static str,
    suffix: &'static str,
) -> Segment {
    Segment::Field(name, color, true, prefix, suffix)
}

pub const fn if_git(segs: &'static [Segment]) -> Segment {
    Segment::IfGit(segs)
}

pub const fn if_not_git(segs: &'static [Segment]) -> Segment {
    Segment::IfNotGit(segs)
}

pub const fn dirty(
    clean: &'static str,
    dirty: &'static str,
    clean_color: Color,
    dirty_color: Color,
) -> Segment {
    Segment::Dirty(clean, dirty, clean_color, dirty_color, false, false)
}

pub const fn dirty_bold(
    clean: &'static str,
    dirty: &'static str,
    clean_color: Color,
    dirty_color: Color,
) -> Segment {
    Segment::Dirty(clean, dirty, clean_color, dirty_color, true, true)
}

// ---------------------------------------------------------------------------
// TemplateDef — a theme defined purely as data
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct TemplateDef {
    pub name: &'static str,
    pub segments: &'static [Segment],
    pub rprompt: &'static [Segment],
}

impl Theme for TemplateDef {
    fn name(&self) -> &'static str {
        self.name
    }

    fn format(&self, info: &StatusInfo) -> String {
        let mut out = String::with_capacity(128);
        render_segments(self.segments, info, &mut out);
        if !self.rprompt.is_empty() {
            let mut rp = String::new();
            render_segments(self.rprompt, info, &mut rp);
            if !rp.is_empty() {
                if let Some(width) = terminal_width() {
                    let left_w = visible_width(&out);
                    let right_w = visible_width(&rp);
                    let needed = left_w + 1 + right_w; // 1 for minimum gap
                    if needed <= width {
                        let pad = width - left_w - right_w;
                        for _ in 0..pad {
                            out.push(' ');
                        }
                    } else {
                        out.push(' ');
                    }
                } else {
                    out.push(' ');
                }
                out.push_str(&rp);
            }
        }
        out
    }
}

// ---------------------------------------------------------------------------
// Render engine
// ---------------------------------------------------------------------------

fn render_segments(segments: &[Segment], info: &StatusInfo, out: &mut String) {
    for seg in segments {
        match seg {
            Segment::Text(s) => out.push_str(s),

            Segment::Lit(s, c, bold) => {
                if *bold {
                    out.push_str(&color::bold(s, *c));
                } else {
                    out.push_str(&color::colored(s, *c));
                }
            }

            Segment::Field(name, c, bold, prefix, suffix) => {
                let val = resolve_field(name, info);
                let text = if !prefix.is_empty() || !suffix.is_empty() {
                    format!("{}{}{}", prefix, val, suffix)
                } else {
                    val
                };
                if *bold {
                    out.push_str(&color::bold(&text, *c));
                } else {
                    out.push_str(&color::colored(&text, *c));
                }
            }

            Segment::IfGit(segs) => {
                if info.git_branch.is_some() {
                    render_segments(segs, info, out);
                }
            }

            Segment::IfNotGit(segs) => {
                if info.git_branch.is_none() {
                    render_segments(segs, info, out);
                }
            }

            Segment::Dirty(clean, dirty_str, clean_color, dirty_color, clean_bold, dirty_bold) => {
                let (sym, c, is_bold) = if info.git_dirty == Some(true) {
                    (*dirty_str, dirty_color, *dirty_bold)
                } else {
                    (*clean, clean_color, *clean_bold)
                };
                if is_bold {
                    out.push_str(&color::bold(sym, *c));
                } else {
                    out.push_str(&color::colored(sym, *c));
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Terminal / string width helpers
// ---------------------------------------------------------------------------

/// Get terminal width from COLUMNS env var or ioctl on stderr.
fn terminal_width() -> Option<usize> {
    if let Ok(cols) = std::env::var("COLUMNS") {
        if let Ok(w) = cols.parse::<usize>() {
            if w > 0 {
                return Some(w);
            }
        }
    }

    #[cfg(unix)]
    {
        #[repr(C)]
        struct Winsize {
            ws_row: u16,
            ws_col: u16,
            ws_xpixel: u16,
            ws_ypixel: u16,
        }
        extern "C" {
            fn ioctl(fd: i32, request: u64, argp: *mut Winsize) -> i32;
        }
        unsafe {
            let mut ws = std::mem::zeroed::<Winsize>();
            // TIOCGWINSZ = 0x5413 on Linux
            if ioctl(2, 0x5413, &mut ws) == 0 && ws.ws_col > 0 {
                return Some(ws.ws_col as usize);
            }
        }
    }

    None
}

/// Calculate visible width of a string by stripping ANSI escape sequences.
fn visible_width(s: &str) -> usize {
    let mut width = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else if c == '\x1b' {
            in_escape = true;
        } else {
            width += 1;
        }
    }
    width
}

fn resolve_field(name: &FieldName, info: &StatusInfo) -> String {
    match name {
        FieldName::User => info.user.clone(),
        FieldName::Hostname => info.hostname.clone(),
        FieldName::ShortHostname => info.short_hostname.clone(),
        FieldName::Cwd => info.cwd.clone(),
        FieldName::CwdBasename => info.cwd_basename.clone(),
        FieldName::CwdTruncated(n) => info.cwd_truncated(*n),
        FieldName::GitBranch => info.git_branch.clone().unwrap_or_default(),
        FieldName::Time => info.time.clone(),
        FieldName::UserAtHost => format!("{}@{}", info.user, info.hostname),
        FieldName::HostColonCwd => format!("{}:{}", info.hostname, info.cwd),
    }
}
