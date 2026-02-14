// ANSI color helpers

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Color256(u8),
}

impl Color {
    pub fn fg(self) -> std::borrow::Cow<'static, str> {
        use std::borrow::Cow;
        match self {
            Color::Black => Cow::Borrowed("\x1b[30m"),
            Color::Red => Cow::Borrowed("\x1b[31m"),
            Color::Green => Cow::Borrowed("\x1b[32m"),
            Color::Yellow => Cow::Borrowed("\x1b[33m"),
            Color::Blue => Cow::Borrowed("\x1b[34m"),
            Color::Magenta => Cow::Borrowed("\x1b[35m"),
            Color::Cyan => Cow::Borrowed("\x1b[36m"),
            Color::White => Cow::Borrowed("\x1b[37m"),
            Color::Gray => Cow::Borrowed("\x1b[90m"),
            Color::BrightRed => Cow::Borrowed("\x1b[91m"),
            Color::BrightGreen => Cow::Borrowed("\x1b[92m"),
            Color::BrightYellow => Cow::Borrowed("\x1b[93m"),
            Color::BrightBlue => Cow::Borrowed("\x1b[94m"),
            Color::BrightMagenta => Cow::Borrowed("\x1b[95m"),
            Color::BrightCyan => Cow::Borrowed("\x1b[96m"),
            Color::BrightWhite => Cow::Borrowed("\x1b[97m"),
            Color::Color256(n) => Cow::Owned(format!("\x1b[38;5;{}m", n)),
        }
    }

    pub fn bg(self) -> std::borrow::Cow<'static, str> {
        use std::borrow::Cow;
        match self {
            Color::Black => Cow::Borrowed("\x1b[40m"),
            Color::Red => Cow::Borrowed("\x1b[41m"),
            Color::Green => Cow::Borrowed("\x1b[42m"),
            Color::Yellow => Cow::Borrowed("\x1b[43m"),
            Color::Blue => Cow::Borrowed("\x1b[44m"),
            Color::Magenta => Cow::Borrowed("\x1b[45m"),
            Color::Cyan => Cow::Borrowed("\x1b[46m"),
            Color::White => Cow::Borrowed("\x1b[47m"),
            Color::Gray => Cow::Borrowed("\x1b[100m"),
            Color::BrightRed => Cow::Borrowed("\x1b[101m"),
            Color::BrightGreen => Cow::Borrowed("\x1b[102m"),
            Color::BrightYellow => Cow::Borrowed("\x1b[103m"),
            Color::BrightBlue => Cow::Borrowed("\x1b[104m"),
            Color::BrightMagenta => Cow::Borrowed("\x1b[105m"),
            Color::BrightCyan => Cow::Borrowed("\x1b[106m"),
            Color::BrightWhite => Cow::Borrowed("\x1b[107m"),
            Color::Color256(n) => Cow::Owned(format!("\x1b[48;5;{}m", n)),
        }
    }
}

pub fn colored(text: &str, color: Color) -> String {
    format!("{}{}{}", color.fg(), text, RESET)
}

pub fn bold(text: &str, color: Color) -> String {
    format!("{}{}{}{}", BOLD, color.fg(), text, RESET)
}
