// ---------------------------------------------------------------------------
// Theme definitions — each theme is a static TemplateDef
//
// To add a new theme: define a const TemplateDef and add it to TEMPLATES.
// Each definition should match its reference .zsh-theme source in oh-my-zsh.
// ---------------------------------------------------------------------------

use crate::color::Color::*;
use crate::themes::components::*;
use crate::themes::template::*;
use FieldName::*;

// ===== Original 10 themes (migrated from individual .rs files) =============

// Source: ys.zsh-theme
// # user @ host in ~/dir on git:branch x [HH:MM:SS]
const YS: TemplateDef = TemplateDef {
    name: "ys",
    segments: &[
        lit_bold("#", Blue),
        text(" "),
        field(User, Cyan),
        text(" "),
        lit("@", White),
        text(" "),
        field(Hostname, Green),
        text(" "),
        lit("in", White),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit("on", White),
            text(" "),
            field_fmt(GitBranch, Cyan, "git:", ""),
            dirty(" o", " x", Green, Red),
        ]),
        text(" "),
        field_fmt(Time, White, "[", "]"),
    ],
    rprompt: &[],
};

// Source: robbyrussell.zsh-theme
// ➜ dir git:(branch) ✗
const ROBBYRUSSELL: TemplateDef = TemplateDef {
    name: "robbyrussell",
    segments: &[
        lit_bold("➜", Green),
        text(" "),
        field(CwdBasename, Cyan),
        if_git(GIT_COLON_PAREN_ROBBYRUSSELL),
    ],
    rprompt: &[],
};

// agnoster — manual Rust impl in agnoster.rs (powerline segments with bg colors)

// Source: af-magic.zsh-theme
// ~/dir (branch*) » — 256-color: FG[032] cwd, FG[075] parens, FG[078] branch, FG[214] dirty
const AF_MAGIC: TemplateDef = TemplateDef {
    name: "af-magic",
    segments: &[
        field(Cwd, Color256(32)),
        if_git(&[
            text(" "),
            lit("(", Color256(75)),
            field(GitBranch, Color256(78)),
            dirty(")", ")*", Color256(75), Color256(214)),
        ]),
        text(" "),
        lit("»", Color256(105)),
    ],
    rprompt: &[],
};

// Source: bira.zsh-theme
// ╭─ user@host ~/dir ‹branch●›
// ╰─$
const BIRA: TemplateDef = TemplateDef {
    name: "bira",
    segments: &[
        BOX_TOP,
        field_bold(User, Green),
        lit("@", Green),
        field_bold(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(GIT_ANGLE_YELLOW_DIRTY_DOT),
    ],
    rprompt: &[],
};

// Source: bureau.zsh-theme
// user@host ~/dir [±branch ✓]
const BUREAU: TemplateDef = TemplateDef {
    name: "bureau",
    segments: &[
        field_bold(User, White),
        lit("@", White),
        field(ShortHostname, White),
        text(" "),
        field_bold(Cwd, White),
        if_git(&[
            text(" "),
            lit("[", White),
            lit_bold("±", Green),
            field_bold(GitBranch, White),
            dirty(" ✓", "", Green, White),
            lit("]", White),
        ]),
    ],
    rprompt: &[],
};

// Source: candy.zsh-theme
// user@host [HH:MM:SS] [~/dir] [branch *]
const CANDY: TemplateDef = TemplateDef {
    name: "candy",
    segments: &[
        field_bold(User, Green),
        lit("@", White),
        field(ShortHostname, Blue),
        text(" "),
        field_fmt(Time, Blue, "[", "]"),
        text(" "),
        field_fmt(Cwd, White, "[", "]"),
        if_git(GIT_BRACKET_GREEN),
    ],
    rprompt: &[],
};

// Source: dallas.zsh-theme
// {date time} host:~/dir@branch✗✗✗ user
const DALLAS: TemplateDef = TemplateDef {
    name: "dallas",
    segments: &[
        field(ShortHostname, Green),
        lit(":", White),
        field(Cwd, Cyan),
        if_git(&[
            lit("@", White),
            field(GitBranch, Blue),
            dirty("", "✗✗✗", Blue, Cyan),
        ]),
        text(" "),
        field(User, Red),
    ],
    rprompt: &[],
};

// Source: gallois.zsh-theme — uses vcs_info
// [~/dir] [branch●]
const GALLOIS: TemplateDef = TemplateDef {
    name: "gallois",
    segments: &[
        field_fmt(Cwd, Cyan, "[", "]"),
        if_git(&[
            text(" "),
            lit("[", Cyan),
            field(GitBranch, Green),
            dirty("", "●", Green, Red),
            lit("]", Cyan),
        ]),
    ],
    rprompt: &[],
};

// Source: maran.zsh-theme
// user@host ~/dir git:(branch)
const MARAN: TemplateDef = TemplateDef {
    name: "maran",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Blue),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Cyan, "git:(", ")"),
        ]),
    ],
    rprompt: &[],
};

// ===== Additional themes (alphabetical) ====================================

// Source: 3den.zsh-theme
// [~/dir] user@host (branch *)
const THREEDEN: TemplateDef = TemplateDef {
    name: "3den",
    segments: &[
        field_fmt(Cwd, Cyan, "[", "]"),
        text(" "),
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, White),
            dirty(")", " *)", White, White),
        ]),
    ],
    rprompt: &[],
};

// Source: Soliah.zsh-theme
// ~/dir (branch *)
const SOLIAH: TemplateDef = TemplateDef {
    name: "Soliah",
    segments: &[
        field(Cwd, White),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, White),
            dirty(")", " *)", White, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: adben.zsh-theme
// user@host ~/dir ‹git:branch ✘/✔›
// (git is in RPROMPT, we append it)
const ADBEN: TemplateDef = TemplateDef {
    name: "adben",
    segments: &[
        field(User, Red),
        lit("@", Red),
        field(ShortHostname, Red),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit("‹", Red),
            field_fmt(GitBranch, Red, "git:", ""),
            dirty(" ✔", " ✘", Green, Yellow),
            lit("›", Red),
        ]),
    ],
    rprompt: &[],
};

// Source: afowler.zsh-theme
// user@host ~/dir ‹branch›
const AFOWLER: TemplateDef = TemplateDef {
    name: "afowler",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field(Cwd, Magenta),
        if_git(GIT_ANGLE_YELLOW),
    ],
    rprompt: &[],
};

// Source: alanpeabody.zsh-theme
// [user@host ~/dir] (branch)  — git in RPROMPT
const ALANPEABODY: TemplateDef = TemplateDef {
    name: "alanpeabody",
    segments: &[
        lit("[", White),
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        lit("]", White),
        if_git(&[
            text(" "),
            field(GitBranch, Green),
        ]),
    ],
    rprompt: &[],
};

// Source: amuse.zsh-theme
// ~/dir on branch! ⌚ HH:MM:SS
const AMUSE: TemplateDef = TemplateDef {
    name: "amuse",
    segments: &[
        field_bold(Cwd, Green),
        if_git(&[
            text(" on "),
            field(GitBranch, Magenta),
            dirty("", "!", Red, Red),
        ]),
        text(" ⌚ "),
        field_bold(Time, Red),
    ],
    rprompt: &[],
};

// Source: apple.zsh-theme — uses vcs_info
// user@host ~/dir (branch●)
const APPLE: TemplateDef = TemplateDef {
    name: "apple",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(GIT_VCS_INFO),
    ],
    rprompt: &[],
};

// Source: arrow.zsh-theme
// dir ➤  — git in RPROMPT: git:branch*
const ARROW: TemplateDef = TemplateDef {
    name: "arrow",
    segments: &[
        field(CwdBasename, Yellow),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Yellow, "git:", ""),
            dirty("", "*", Yellow, Yellow),
        ]),
        text(" "),
        lit("➤", Yellow),
    ],
    rprompt: &[],
};

// Source: aussiegeek.zsh-theme
// user@host [~/dir] (branch ✗/✔)
const AUSSIEGEEK: TemplateDef = TemplateDef {
    name: "aussiegeek",
    segments: &[
        field_bold(User, Green),
        lit("@", White),
        field_bold(ShortHostname, Green),
        text(" "),
        field_fmt(Cwd, White, "[", "]"),
        if_git(&[
            text(" "),
            lit_bold("(", Green),
            field_bold(GitBranch, Green),
            dirty(" ✔", " ✗", Green, Green),
            lit_bold(")", Green),
        ]),
    ],
    rprompt: &[],
};

// Source: avit.zsh-theme
// ~/dir branch ✔/✗
const AVIT: TemplateDef = TemplateDef {
    name: "avit",
    segments: &[
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            field(GitBranch, Green),
            dirty(" ✔", " ✗", Green, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: awesomepanda.zsh-theme
// 🐼 ~/dir git:(branch) ✗
const AWESOMEPANDA: TemplateDef = TemplateDef {
    name: "awesomepanda",
    segments: &[
        text("🐼 "),
        field_bold(Cwd, Blue),
        if_git(GIT_COLON_PAREN_RED),
    ],
    rprompt: &[],
};

// Source: blinks.zsh-theme
// user@host:~/dir [branch *]
const BLINKS: TemplateDef = TemplateDef {
    name: "blinks",
    segments: &[
        field(User, Cyan),
        lit("@", White),
        field(ShortHostname, Green),
        lit(":", White),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit("[", Blue),
            field(GitBranch, Blue),
            dirty("]", " *]", Blue, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: candy-kingdom.zsh-theme
// user@host ~/dir (branch:branch!)
const CANDY_KINGDOM: TemplateDef = TemplateDef {
    name: "candy-kingdom",
    segments: &[
        field(User, Magenta),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", Magenta),
            field(GitBranch, Magenta),
            dirty(")", "!)", Magenta, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: clean.zsh-theme
// user:dir/ (branch✗) [HH:MM:SS]
const CLEAN: TemplateDef = TemplateDef {
    name: "clean",
    segments: &[
        field_bold(User, White),
        lit(":", White),
        field_fmt_bold(CwdBasename, Blue, "", "/"),
        if_git(&[
            text(" "),
            lit_bold("(", Blue),
            field_bold(GitBranch, Yellow),
            dirty("", "✗", Yellow, Red),
            lit_bold(")", Blue),
        ]),
    ],
    rprompt: RPROMPT_TIME_BRACKET,
};

// Source: cloud.zsh-theme
// ☁ dir [branch]⚡
const CLOUD: TemplateDef = TemplateDef {
    name: "cloud",
    segments: &[
        lit_bold("☁", Cyan),
        text("  "),
        field(CwdBasename, Green),
        if_git(&[
            text(" "),
            lit("[", Green),
            field(GitBranch, Cyan),
            dirty("]", "] ⚡", Green, Yellow),
        ]),
    ],
    rprompt: &[],
};

// Source: crcandy.zsh-theme (same git as candy)
// user@host ~/dir [branch *]
const CRCANDY: TemplateDef = TemplateDef {
    name: "crcandy",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(GIT_BRACKET_GREEN),
    ],
    rprompt: &[],
};

// Source: crunch.zsh-theme
// [HH:MM] ~/dir user@host :branch ✓/✗
const CRUNCH: TemplateDef = TemplateDef {
    name: "crunch",
    segments: &[
        field_fmt(Time, Cyan, "[", "]"),
        text(" "),
        field(Cwd, Yellow),
        text(" "),
        lit("[", White),
        field(User, Red),
        lit("@", White),
        field(ShortHostname, Red),
        lit("]", White),
        if_git(&[
            lit(":", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: cypher.zsh-theme — no git
// host :: ~/dir
const CYPHER: TemplateDef = TemplateDef {
    name: "cypher",
    segments: &[
        field(ShortHostname, White),
        lit(" :: ", Red),
        field(Cwd, Green),
    ],
    rprompt: &[],
};

// Source: dallas.zsh-theme — already defined above as DALLAS

// Source: darkblood.zsh-theme
// user@host ~/dir [branch ⚡]
const DARKBLOOD: TemplateDef = TemplateDef {
    name: "darkblood",
    segments: &[
        field(User, Red),
        lit("@", White),
        field(ShortHostname, Red),
        text(" "),
        field(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit_bold("[", White),
            field(GitBranch, White),
            dirty("]", " ⚡]", White, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: daveverwer.zsh-theme
// user@host ~/dir (branch)
const DAVEVERWER: TemplateDef = TemplateDef {
    name: "daveverwer",
    segments: &[
        field(User, Yellow),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", Blue),
            field(GitBranch, Blue),
            lit(")", Blue),
        ]),
    ],
    rprompt: &[],
};

// Source: dieter.zsh-theme
// HH:MM:SS user@host ~/dir branch?
const DIETER: TemplateDef = TemplateDef {
    name: "dieter",
    segments: &[
        field(Time, Blue),
        text(" "),
        field(User, White),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field(Cwd, White),
        if_git(&[
            text(" "),
            field(GitBranch, Yellow),
            dirty("", "?", Green, Yellow),
        ]),
    ],
    rprompt: &[],
};

// Source: dogenpunk.zsh-theme — git in RPROMPT
// user@host ~/dir (branch)
const DOGENPUNK: TemplateDef = TemplateDef {
    name: "dogenpunk",
    segments: &[
        field(User, Yellow),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit_bold("(", Green),
            field_bold(GitBranch, Black),
            dirty(")", "!)", Green, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: dpoggi.zsh-theme
// user@host:~/dir (branch○/⚡)
const DPOGGI: TemplateDef = TemplateDef {
    name: "dpoggi",
    segments: &[
        field(User, Cyan),
        lit("@", White),
        field(ShortHostname, Cyan),
        lit(":", White),
        field(Cwd, Magenta),
        if_git(&[
            text(" "),
            lit("(", Yellow),
            field(GitBranch, Yellow),
            dirty("○", "⚡", Green, Red),
            lit(")", Yellow),
        ]),
    ],
    rprompt: &[],
};

// Source: dst.zsh-theme
// user@host: ~/dir branch! [HH:MM:SS]
const DST: TemplateDef = TemplateDef {
    name: "dst",
    segments: &[
        field(User, Magenta),
        lit("@", White),
        field(ShortHostname, Yellow),
        lit(":", White),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            field(GitBranch, Green),
            dirty("", "!", Red, Red),
        ]),
    ],
    rprompt: RPROMPT_TIME_BRACKET,
};

// Source: dstufft.zsh-theme
// user at host in ~/dir on branch!
const DSTUFFT: TemplateDef = TemplateDef {
    name: "dstufft",
    segments: &[
        field(User, Magenta),
        text(" "),
        lit("at", White),
        text(" "),
        field(ShortHostname, Yellow),
        text(" "),
        lit("in", White),
        text(" "),
        field_bold(Cwd, Green),
        if_git(&[
            text(" "),
            lit("on", White),
            text(" "),
            field(GitBranch, Magenta),
            dirty("", "!", Green, Green),
        ]),
    ],
    rprompt: &[],
};

// Source: duellj.zsh-theme — no git in PROMPT (RPROMPT has time)
// user@host:~/dir [HH:MM:SS]
const DUELLJ: TemplateDef = TemplateDef {
    name: "duellj",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Blue),
        lit(":", White),
        field_bold(Cwd, Yellow),
    ],
    rprompt: RPROMPT_TIME_BRACKET,
};

// Source: eastwood.zsh-theme
// user@host:~/dir [branch *]
const EASTWOOD: TemplateDef = TemplateDef {
    name: "eastwood",
    segments: &[
        field(User, Cyan),
        lit("@", White),
        field(ShortHostname, Cyan),
        lit(":", White),
        field_bold(Cwd, Yellow),
        if_git(GIT_BRACKET_GREEN),
    ],
    rprompt: &[],
};

// Source: edvardm.zsh-theme
// user@host ~/dir git:(branch) ✗
const EDVARDM: TemplateDef = TemplateDef {
    name: "edvardm",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field(Cwd, Cyan),
        if_git(GIT_COLON_PAREN_RED),
    ],
    rprompt: &[],
};

// Source: emotty.zsh-theme — uses vcs_info
// 😄 ~/dir (branch●)
const EMOTTY: TemplateDef = TemplateDef {
    name: "emotty",
    segments: &[
        text("😄 "),
        field_bold(Cwd, Blue),
        if_git(GIT_VCS_INFO),
    ],
    rprompt: &[],
};

// Source: essembeh.zsh-theme
// user@host ➜ ~/dir (branch)
const ESSEMBEH: TemplateDef = TemplateDef {
    name: "essembeh",
    segments: &[
        field(User, Blue),
        lit("@", White),
        field(ShortHostname, Blue),
        text(" "),
        lit("➜", Green),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit("(", Cyan),
            field(GitBranch, Cyan),
            lit(")", Cyan),
        ]),
    ],
    rprompt: &[],
};

// Source: evan.zsh-theme — no git
// host :: ~/dir
const EVAN: TemplateDef = TemplateDef {
    name: "evan",
    segments: &[
        field(ShortHostname, White),
        lit(" :: ", Red),
        field(Cwd, Green),
    ],
    rprompt: &[],
};

// Source: fino.zsh-theme — 256-color
// ╭─ user at host in ~/dir on branch✔/✘✘✘
const FINO: TemplateDef = TemplateDef {
    name: "fino",
    segments: &[
        BOX_TOP,
        field(User, Color256(40)),
        text(" "),
        lit("at", Color256(239)),
        text(" "),
        field(ShortHostname, Color256(33)),
        text(" "),
        lit("in", Color256(239)),
        text(" "),
        field_bold(Cwd, Color256(226)),
        if_git(&[
            text(" "),
            lit("on", Color256(239)),
            text(" "),
            field(GitBranch, Color256(255)),
            dirty("✔", "✘✘✘", Color256(40), Color256(202)),
        ]),
    ],
    rprompt: &[],
};

// Source: fino-time.zsh-theme — 256-color (like fino + time)
// ╭─ user at host in ~/dir on branch✔ %D - %*
const FINO_TIME: TemplateDef = TemplateDef {
    name: "fino-time",
    segments: &[
        BOX_TOP,
        field(User, Color256(40)),
        text(" "),
        lit("at", Color256(239)),
        text(" "),
        field(ShortHostname, Color256(33)),
        text(" "),
        lit("in", Color256(239)),
        text(" "),
        field_bold(Cwd, Color256(226)),
        if_git(&[
            text(" "),
            lit("on", Color256(239)),
            text(" "),
            field(GitBranch, Color256(255)),
            dirty("✔", "✘✘✘", Color256(40), Color256(202)),
        ]),
        text(" "),
        field(Time, White),
    ],
    rprompt: &[],
};

// Source: fishy.zsh-theme — git in RPROMPT
// user@host ~/dir >
const FISHY: TemplateDef = TemplateDef {
    name: "fishy",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            field(GitBranch, Green),
        ]),
        text(" "),
        lit(">", White),
    ],
    rprompt: &[],
};

// Source: flazz.zsh-theme
// host :: ~/dir ‹branch›
const FLAZZ: TemplateDef = TemplateDef {
    name: "flazz",
    segments: &[
        field(ShortHostname, White),
        lit(" :: ", Magenta),
        field(Cwd, Green),
        if_git(&[
            text(" "),
            lit_bold("‹", Cyan),
            field_bold(GitBranch, Cyan),
            lit_bold("›", Cyan),
        ]),
    ],
    rprompt: &[],
};

// Source: fletcherm.zsh-theme
// user@host:~/dir (branch⚡/) [HH:MM:SS]
const FLETCHERM: TemplateDef = TemplateDef {
    name: "fletcherm",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        lit(":", White),
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit_bold("(", Blue),
            field(GitBranch, Blue),
            dirty(")", "⚡)", Blue, Yellow),
        ]),
    ],
    rprompt: RPROMPT_TIME_BRACKET,
};

// Source: fox.zsh-theme
// 🦊 ~/dir -[git://branch]-  ✔/✗
const FOX: TemplateDef = TemplateDef {
    name: "fox",
    segments: &[
        text("🦊 "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit_bold("-[git://", White),
            field_bold(GitBranch, White),
            dirty("]-", "]-", White, White),
            dirty(" ✔", " ✗", Green, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: frisk.zsh-theme
// ~/dir [git::branch*] [user@host] [HH:MM:SS]
const FRISK: TemplateDef = TemplateDef {
    name: "frisk",
    segments: &[
        field(Cwd, Blue),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Green, "[git::", ""),
            dirty("]", "*]", Green, Red),
        ]),
        text(" "),
        lit("[", White),
        field(User, White),
        lit("@", White),
        field(ShortHostname, White),
        lit("]", White),
        text(" "),
        field_fmt(Time, White, "[", "]"),
    ],
    rprompt: &[],
};

// Source: frontcube.zsh-theme — git in RPROMPT
// ~/dir [git:branch] ✔/✖
const FRONTCUBE: TemplateDef = TemplateDef {
    name: "frontcube",
    segments: &[
        field(Cwd, Gray),
        if_git(&[
            text(" "),
            lit_bold("[", Blue),
            field_fmt_bold(GitBranch, Blue, "git:", ""),
            dirty("] ✔", "] ✖", Green, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: funky.zsh-theme — no git
// user@host ~/dir
const FUNKY: TemplateDef = TemplateDef {
    name: "funky",
    segments: &[
        field(User, Magenta),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field_bold(Cwd, Cyan),
    ],
    rprompt: &[],
};

// Source: fwalch.zsh-theme
// ~/dir (branch) ✗
const FWALCH: TemplateDef = TemplateDef {
    name: "fwalch",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", Red),
            field(GitBranch, Red),
            dirty(")", ") ✗", Blue, Yellow),
        ]),
    ],
    rprompt: &[],
};

// Source: gallifrey.zsh-theme
// host ~/dir ‹branch› »
const GALLIFREY: TemplateDef = TemplateDef {
    name: "gallifrey",
    segments: &[
        field(ShortHostname, Green),
        text(" "),
        field(Cwd, White),
        if_git(GIT_ANGLE_YELLOW),
    ],
    rprompt: &[],
};

// Source: garyblessington.zsh-theme
// user@host ~/dir (branch) ✗
const GARYBLESSINGTON: TemplateDef = TemplateDef {
    name: "garyblessington",
    segments: &[
        field(User, Cyan),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field_bold(Cwd, Green),
        if_git(&[
            text(" "),
            lit("(", Blue),
            field(GitBranch, Blue),
            dirty(")", ") ✗", Blue, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: gentoo.zsh-theme — uses vcs_info
// user@host ~/dir (branch*+?) $
const GENTOO: TemplateDef = TemplateDef {
    name: "gentoo",
    segments: &[
        field_bold(User, Green),
        lit_bold("@", Green),
        field_bold(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", Magenta),
            field(GitBranch, Green),
            dirty("", "*", Green, Red),
            lit(")", Magenta),
        ]),
    ],
    rprompt: &[],
};

// Source: geoffgarside.zsh-theme
// user@host ~/dir git:(branch)
const GEOFFGARSIDE: TemplateDef = TemplateDef {
    name: "geoffgarside",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Yellow, "git:(", ")"),
        ]),
    ],
    rprompt: &[],
};

// Source: gianu.zsh-theme
// [user@host dir (branch)✗]
const GIANU: TemplateDef = TemplateDef {
    name: "gianu",
    segments: &[
        lit("[", White),
        field_bold(User, White),
        lit("@", White),
        field_bold(ShortHostname, Red),
        text(" "),
        field(CwdBasename, Cyan),
        if_git(&[
            text(" "),
            lit_bold("(", Green),
            field(GitBranch, Green),
            lit(")", Green),
            dirty("", "✗", Green, Yellow),
        ]),
        lit("]", White),
    ],
    rprompt: &[],
};

// Source: gnzh.zsh-theme
// ╭─ user@host ~/dir ‹branch›
const GNZH: TemplateDef = TemplateDef {
    name: "gnzh",
    segments: &[
        BOX_TOP,
        field(User, Green),
        lit("@", Cyan),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(GIT_ANGLE_YELLOW),
    ],
    rprompt: &[],
};

// Source: gozilla.zsh-theme — git status in RPROMPT
// user@host ~/dir (branch)
const GOZILLA: TemplateDef = TemplateDef {
    name: "gozilla",
    segments: &[
        field(User, Yellow),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, White),
            lit(")", White),
        ]),
    ],
    rprompt: &[],
};

// Source: half-life.zsh-theme — uses vcs_info with 256-color
// user on branch●
const HALF_LIFE: TemplateDef = TemplateDef {
    name: "half-life",
    segments: &[
        field(User, Magenta),
        text(" in "),
        field(Cwd, Green),
        if_git(&[
            text(" on "),
            field(GitBranch, Cyan),
            dirty("", "●", Cyan, Yellow),
        ]),
    ],
    rprompt: &[],
};

// Source: humza.zsh-theme
// user@host ~/dir ±(branch);
const HUMZA: TemplateDef = TemplateDef {
    name: "humza",
    segments: &[
        field(User, Cyan),
        lit("@", White),
        field(ShortHostname, Cyan),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Red, "±(", ");"),
        ]),
    ],
    rprompt: &[],
};

// Source: imajes.zsh-theme — no git in PROMPT
// user@host ~/dir
const IMAJES: TemplateDef = TemplateDef {
    name: "imajes",
    segments: &[
        field(User, Yellow),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field(Cwd, Cyan),
    ],
    rprompt: &[],
};

// Source: intheloop.zsh-theme
// user@host ~/dir (branch⚡/)
const INTHELOOP: TemplateDef = TemplateDef {
    name: "intheloop",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", Gray),
            field(GitBranch, Red),
            dirty(")", " ⚡)", Gray, Yellow),
        ]),
    ],
    rprompt: &[],
};

// Source: itchy.zsh-theme — git in RPROMPT
// user@host ~/dir ✔/✗
const ITCHY: TemplateDef = TemplateDef {
    name: "itchy",
    segments: &[
        field(User, Cyan),
        lit("@", White),
        field(ShortHostname, Cyan),
        text(" "),
        field(Cwd, Yellow),
        if_git(&[
            text(" "),
            field(GitBranch, White),
            dirty(" ✔", " ✗", Green, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: jaischeema.zsh-theme
// user@host ~/dir ±(branch) ✗
const JAISCHEEMA: TemplateDef = TemplateDef {
    name: "jaischeema",
    segments: &[
        field(User, Blue),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("±(", Red),
            field(GitBranch, Red),
            dirty(")", ") ✗", Blue, Yellow),
        ]),
    ],
    rprompt: &[],
};

// Source: jbergantine.zsh-theme
// user@host:~/dir git:(branch) ✗
const JBERGANTINE: TemplateDef = TemplateDef {
    name: "jbergantine",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        lit(":", White),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("git:(", Red),
            field(GitBranch, Red),
            dirty(")", ") ✗", White, Yellow),
        ]),
    ],
    rprompt: &[],
};

// Source: jispwoso.zsh-theme
// ~/dir git:(branch) ✗
const JISPWOSO: TemplateDef = TemplateDef {
    name: "jispwoso",
    segments: &[
        field(Cwd, Green),
        if_git(GIT_COLON_PAREN_RED),
    ],
    rprompt: &[],
};

// Source: jnrowe.zsh-theme — uses vcs_info
// user:~/dir (branch●)
const JNROWE: TemplateDef = TemplateDef {
    name: "jnrowe",
    segments: &[
        field(User, Cyan),
        lit(":", White),
        field_bold(Cwd, Yellow),
        if_git(GIT_VCS_INFO),
    ],
    rprompt: &[],
};

// Source: jonathan.zsh-theme
// user@host ~/dir on branch
const JONATHAN: TemplateDef = TemplateDef {
    name: "jonathan",
    segments: &[
        field(User, Blue),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit("on", White),
            text(" "),
            field(GitBranch, Green),
        ]),
    ],
    rprompt: &[],
};

// Source: josh.zsh-theme
// user@host ~/dir (branch) ✗
const JOSH: TemplateDef = TemplateDef {
    name: "josh",
    segments: &[
        field(User, Blue),
        lit("@", White),
        field(ShortHostname, Blue),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, White),
            dirty(")", ") ✗", White, Yellow),
        ]),
    ],
    rprompt: &[],
};

// Source: jreese.zsh-theme
// user@host ~/dir ±branch⚡
const JREESE: TemplateDef = TemplateDef {
    name: "jreese",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field(Cwd, White),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Yellow, "±", ""),
            dirty("", "⚡", Green, Yellow),
        ]),
    ],
    rprompt: &[],
};

// Source: jtriley.zsh-theme — no git
// HH:MM:SS user@host ~/dir
const JTRILEY: TemplateDef = TemplateDef {
    name: "jtriley",
    segments: &[
        field_bold(Time, Cyan),
        text(" "),
        field_bold(User, White),
        lit("@", Magenta),
        field_bold(ShortHostname, White),
        text(" "),
        field_bold(Cwd, Green),
    ],
    rprompt: &[],
};

// Source: juanghurtado.zsh-theme — git in RPROMPT
// [user@host] [~/dir] (branch)
const JUANGHURTADO: TemplateDef = TemplateDef {
    name: "juanghurtado",
    segments: &[
        lit("[", White),
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        lit("]", White),
        text(" "),
        lit("[", White),
        field_bold(Cwd, Blue),
        lit("]", White),
        if_git(&[
            text(" "),
            field_bold(GitBranch, Green),
            dirty("", "(*)", Green, Green),
        ]),
    ],
    rprompt: &[],
};

// Source: junkfood.zsh-theme
// host @branch✗✗✗/✔
const JUNKFOOD: TemplateDef = TemplateDef {
    name: "junkfood",
    segments: &[
        field(ShortHostname, White),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit_bold("@", White),
            field_bold(GitBranch, Blue),
            dirty(" ✔", " ✗✗✗", Green, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: kafeitu.zsh-theme
// user@host ~/dir git:(branch) ✗
const KAFEITU: TemplateDef = TemplateDef {
    name: "kafeitu",
    segments: &[
        field(User, Cyan),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(GIT_COLON_PAREN_RED),
    ],
    rprompt: &[],
};

// Source: kardan.zsh-theme — git in RPROMPT
// ~/dir git:(branch✗) @host
const KARDAN: TemplateDef = TemplateDef {
    name: "kardan",
    segments: &[
        field(Cwd, White),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, White),
            dirty(")", "✗)", White, Yellow),
        ]),
        lit("@", White),
        field(ShortHostname, White),
    ],
    rprompt: &[],
};

// Source: kennethreitz.zsh-theme
// dir (branch *)  »
const KENNETHREITZ: TemplateDef = TemplateDef {
    name: "kennethreitz",
    segments: &[
        field(CwdBasename, Green),
        if_git(&[
            text(" "),
            lit("(", Yellow),
            field(GitBranch, Yellow),
            dirty(")", " *)", Yellow, Red),
        ]),
        text(" "),
        lit("»", Red),
    ],
    rprompt: &[],
};

// Source: kiwi.zsh-theme
// 🥝 ~/dir [git:branch]-
const KIWI: TemplateDef = TemplateDef {
    name: "kiwi",
    segments: &[
        text("🥝 "),
        field_bold(Cwd, Green),
        if_git(&[
            text(" "),
            lit_bold("[git:", White),
            field_bold(GitBranch, White),
            lit_bold("]-", White),
        ]),
    ],
    rprompt: &[],
};

// Source: kolo.zsh-theme — uses vcs_info
// dir (branch●)
const KOLO: TemplateDef = TemplateDef {
    name: "kolo",
    segments: &[
        field_bold(CwdBasename, Magenta),
        if_git(GIT_VCS_INFO),
    ],
    rprompt: &[],
};

// Source: kphoen.zsh-theme
// [user@host:~/dir on branch]
const KPHOEN: TemplateDef = TemplateDef {
    name: "kphoen",
    segments: &[
        lit("[", White),
        field(User, Red),
        lit("@", White),
        field(ShortHostname, Magenta),
        lit(":", White),
        field(Cwd, Blue),
        if_git(&[
            lit(" on ", Green),
            field(GitBranch, Green),
        ]),
        lit("]", White),
    ],
    rprompt: &[],
};

// Source: lambda.zsh-theme
// λ ~/dir/ branch
const LAMBDA: TemplateDef = TemplateDef {
    name: "lambda",
    segments: &[
        lit("λ", White),
        text(" "),
        field_fmt(Cwd, White, "", "/"),
        if_git(&[
            text(" "),
            field(GitBranch, Green),
        ]),
    ],
    rprompt: &[],
};

// Source: linuxonly.zsh-theme — uses vcs_info
// user@host ~/dir (branch●)
const LINUXONLY: TemplateDef = TemplateDef {
    name: "linuxonly",
    segments: &[
        field_bold(User, Green),
        lit("@", Green),
        field_bold(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(GIT_VCS_INFO),
    ],
    rprompt: &[],
};

// Source: lukerandall.zsh-theme
// user@host ~/dir (branch)
const LUKERANDALL: TemplateDef = TemplateDef {
    name: "lukerandall",
    segments: &[
        field_bold(User, Green),
        lit_bold("@", Green),
        field_bold(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(GIT_PAREN_YELLOW),
    ],
    rprompt: &[],
};

// Source: macovsky.zsh-theme
// ~/dir ‹branch›
const MACOVSKY: TemplateDef = TemplateDef {
    name: "macovsky",
    segments: &[
        field(Cwd, Green),
        if_git(GIT_ANGLE_YELLOW),
    ],
    rprompt: &[],
};

// Source: macovsky-ruby.zsh-theme — no git
// ~/dir ❯
const MACOVSKY_RUBY: TemplateDef = TemplateDef {
    name: "macovsky-ruby",
    segments: &[
        field_bold(Cwd, Cyan),
        text(" "),
        lit("❯", Magenta),
    ],
    rprompt: &[],
};

// Source: mgutz.zsh-theme
// dir [branch*]
const MGUTZ: TemplateDef = TemplateDef {
    name: "mgutz",
    segments: &[
        field_bold(CwdBasename, Magenta),
        if_git(&[
            text(" "),
            lit_bold("[", Yellow),
            field_bold(GitBranch, Yellow),
            dirty("]", "*]", Yellow, Yellow),
        ]),
    ],
    rprompt: &[],
};

// Source: mh.zsh-theme — git in RPROMPT
// ~/dir (branch ✱)
const MH: TemplateDef = TemplateDef {
    name: "mh",
    segments: &[
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, White),
            dirty(")", " ✱)", White, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: michelebologna.zsh-theme
// user@host:~/dir (git:branch*)
const MICHELEBOLOGNA: TemplateDef = TemplateDef {
    name: "michelebologna",
    segments: &[
        field_bold(User, Blue),
        lit("@", White),
        field(ShortHostname, Cyan),
        lit(":", White),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", Blue),
            field_fmt(GitBranch, Green, "git:", ""),
            dirty("", "*", Green, Green),
            lit(")", Blue),
        ]),
    ],
    rprompt: &[],
};

// Source: mikeh.zsh-theme — uses vcs_info
// [user@host] - [~/dir] - [HH:MM:SS] <git:(branch)>
const MIKEH: TemplateDef = TemplateDef {
    name: "mikeh",
    segments: &[
        lit_bold("[", Blue),
        field_bold(User, Green),
        lit("@", White),
        field(ShortHostname, Cyan),
        lit_bold("]", Blue),
        text(" - "),
        lit_bold("[", Blue),
        field_bold(Cwd, White),
        lit_bold("]", Blue),
        text(" - "),
        field_fmt(Time, Yellow, "[", "]"),
        if_git(&[
            text(" "),
            lit("<", White),
            field_fmt(GitBranch, White, "git:(", ")"),
            lit(">", White),
        ]),
    ],
    rprompt: &[],
};

// Source: miloshadzic.zsh-theme
// dir|git:branch⚡⇒
const MILOSHADZIC: TemplateDef = TemplateDef {
    name: "miloshadzic",
    segments: &[
        field(CwdBasename, Cyan),
        if_git(&[
            lit("|", Red),
            field_fmt(GitBranch, Green, "git:", ""),
            dirty("", "⚡", Green, Yellow),
        ]),
        lit(" ⇒", Cyan),
    ],
    rprompt: &[],
};

// Source: minimal.zsh-theme
// ~/dir [branch●]
const MINIMAL: TemplateDef = TemplateDef {
    name: "minimal",
    segments: &[
        field(Cwd, White),
        if_git(&[
            text(" "),
            lit("[", White),
            field(GitBranch, White),
            dirty("]", "●]", White, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: mira.zsh-theme
// user@host ~/dir (branch)
const MIRA: TemplateDef = TemplateDef {
    name: "mira",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(GIT_PAREN_YELLOW),
    ],
    rprompt: &[],
};

// Source: mlh.zsh-theme — complex RPROMPT
// user@host ~/dir (branch) [HH:MM:SS]
const MLH: TemplateDef = TemplateDef {
    name: "mlh",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            field(GitBranch, White),
        ]),
    ],
    rprompt: RPROMPT_TIME_BRACKET,
};

// Source: mortalscumbag.zsh-theme — custom git function
// ~/dir ‹dirty_color branch›
const MORTALSCUMBAG: TemplateDef = TemplateDef {
    name: "mortalscumbag",
    segments: &[
        field_bold(Cwd, Green),
        if_git(&[
            text(" "),
            lit_bold("‹", Yellow),
            field_bold(GitBranch, Yellow),
            lit_bold("›", White),
        ]),
    ],
    rprompt: &[],
};

// Source: mrtazz.zsh-theme — git in RPROMPT
// user@host:~/dir <branch✗/✓>
const MRTAZZ: TemplateDef = TemplateDef {
    name: "mrtazz",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        lit(":", White),
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            field_fmt_bold(GitBranch, Green, "<", ""),
            dirty(">", "✗>", Green, Yellow),
        ]),
    ],
    rprompt: &[],
};

// Source: murilasso.zsh-theme
// user@host ~/dir branch ✗/✔
const MURILASSO: TemplateDef = TemplateDef {
    name: "murilasso",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Blue),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            field(GitBranch, White),
            dirty(" ✔", " ✗", Green, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: muse.zsh-theme — 256-color
// ~/dir (branch ✔/✘) ᐅ
const MUSE: TemplateDef = TemplateDef {
    name: "muse",
    segments: &[
        field(Cwd, Color256(117)),
        if_git(&[
            text(" "),
            lit("(", Color256(12)),
            field(GitBranch, Color256(12)),
            dirty(" ✔", " ✘", Color256(118), Color256(133)),
            lit(")", Color256(12)),
        ]),
        text(" "),
        lit("ᐅ", Color256(77)),
    ],
    rprompt: &[],
};

// Source: nanotech.zsh-theme — git in RPROMPT
// ▸ ~/dir branch* HH:MM:SS
const NANOTECH: TemplateDef = TemplateDef {
    name: "nanotech",
    segments: &[
        lit("▸", Green),
        text(" "),
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            field(GitBranch, Yellow),
            dirty("", "*", Yellow, Red),
        ]),
    ],
    rprompt: &[field(Time, White)],
};

// Source: nebirhos.zsh-theme
// user@host ~/dir git:(branch) ✗
const NEBIRHOS: TemplateDef = TemplateDef {
    name: "nebirhos",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Blue),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(GIT_COLON_PAREN_RED),
    ],
    rprompt: &[],
};

// Source: nicoulaj.zsh-theme — uses vcs_info, RPROMPT
// ~/dir (branch!+)
const NICOULAJ: TemplateDef = TemplateDef {
    name: "nicoulaj",
    segments: &[
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", Magenta),
            field(GitBranch, Green),
            dirty("", "!", Green, Red),
            lit(")", Magenta),
        ]),
    ],
    rprompt: &[],
};

// Source: norm.zsh-theme
// λ host dir → branch → — blue "git" red branch
const NORM: TemplateDef = TemplateDef {
    name: "norm",
    segments: &[
        lit("λ", Yellow),
        text(" "),
        field(ShortHostname, Yellow),
        text(" "),
        field(CwdBasename, Green),
        if_git(&[
            text(" "),
            lit("→", Yellow),
            text(" "),
            lit("λ", Blue),
            text(" "),
            lit("git", Blue),
            text(" "),
            field(GitBranch, Red),
            text(" "),
            lit("→", Yellow),
        ]),
    ],
    rprompt: &[],
};

// Source: obraun.zsh-theme
// [HH:MM:SS] user :: host ➜ ~/dir ‹branch› »
const OBRAUN: TemplateDef = TemplateDef {
    name: "obraun",
    segments: &[
        field_fmt(Time, Green, "[", "]"),
        text(" "),
        field(User, Cyan),
        text(" "),
        lit("::", Blue),
        text(" "),
        field(ShortHostname, Yellow),
        text(" "),
        lit("➜", Magenta),
        text("  "),
        field(Cwd, Green),
        if_git(GIT_ANGLE_RED),
    ],
    rprompt: &[],
};

// Source: oldgallois.zsh-theme
// ~/dir [branch *]
const OLDGALLOIS: TemplateDef = TemplateDef {
    name: "oldgallois",
    segments: &[
        field(Cwd, Cyan),
        if_git(GIT_BRACKET_GREEN),
    ],
    rprompt: &[],
};

// Source: peepcode.zsh-theme — git in RPROMPT
// user in ~/dir
const PEEPCODE: TemplateDef = TemplateDef {
    name: "peepcode",
    segments: &[
        field_bold(User, Red),
        text(" "),
        lit("in", White),
        text(" "),
        field_bold(Cwd, Green),
        if_git(&[
            text(" "),
            field(GitBranch, White),
        ]),
    ],
    rprompt: &[],
};

// Source: philips.zsh-theme
// [~/dir] (branch *) [HH:MM:SS]
const PHILIPS: TemplateDef = TemplateDef {
    name: "philips",
    segments: &[
        lit("[", White),
        field(Cwd, Cyan),
        lit("]", White),
        if_git(&[
            text(" "),
            lit_bold("(", Blue),
            field_bold(GitBranch, Blue),
            dirty(")", " *)", Blue, Blue),
        ]),
    ],
    rprompt: RPROMPT_TIME_BRACKET,
};

// Source: pmcgee.zsh-theme
// user@host ~/dir branch* [HH:MM:SS]
const PMCGEE: TemplateDef = TemplateDef {
    name: "pmcgee",
    segments: &[
        field_bold(User, Green),
        lit("@", White),
        field_bold(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            field(GitBranch, White),
            dirty("", "*", White, Red),
        ]),
    ],
    rprompt: RPROMPT_TIME_BRACKET,
};

// Source: pygmalion.zsh-theme
// user@host ⮞ ~/dir ▶ branch⚡
const PYGMALION: TemplateDef = TemplateDef {
    name: "pygmalion",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Blue),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            field(GitBranch, Green),
            dirty("", "⚡", Green, Yellow),
        ]),
    ],
    rprompt: &[],
};

// Source: pygmalion-virtualenv.zsh-theme
// user@host ⮞ ~/dir ▶ branch⚡
const PYGMALION_VIRTUALENV: TemplateDef = TemplateDef {
    name: "pygmalion-virtualenv",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Blue),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            field(GitBranch, Green),
            dirty("", "⚡", Green, Yellow),
        ]),
    ],
    rprompt: &[],
};

// Source: re5et.zsh-theme
// → ~/dir (branch±/♥)
const RE5ET: TemplateDef = TemplateDef {
    name: "re5et",
    segments: &[
        lit("→", Green),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit_bold("^", Magenta),
            field_bold(GitBranch, Yellow),
            dirty(" ♥", " ±", Green, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: refined.zsh-theme — uses vcs_info, RPROMPT
// ~/dir (branch !+)
const REFINED: TemplateDef = TemplateDef {
    name: "refined",
    segments: &[
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            field(GitBranch, Magenta),
            dirty("", "!", Magenta, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: rgm.zsh-theme
// user@host:~/dir branch
const RGM: TemplateDef = TemplateDef {
    name: "rgm",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        lit(":", White),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            field(GitBranch, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: risto.zsh-theme
// user@host:~/dir ‹branch›
const RISTO: TemplateDef = TemplateDef {
    name: "risto",
    segments: &[
        field(User, Green),
        lit("@", White),
        field_bold(ShortHostname, Blue),
        lit(":", White),
        field(Cwd, Green),
        if_git(GIT_ANGLE_RED),
    ],
    rprompt: &[],
};

// Source: rixius.zsh-theme
// [~/dir] on branch!/√ HH:MM:SS
const RIXIUS: TemplateDef = TemplateDef {
    name: "rixius",
    segments: &[
        lit("[", Blue),
        field(Cwd, Cyan),
        lit("]", Blue),
        if_git(&[
            text(" "),
            lit("on ", Magenta),
            field(GitBranch, Magenta),
            dirty(" √", " !", Green, Magenta),
        ]),
    ],
    rprompt: &[field(Time, White)],
};

// Source: rkj.zsh-theme — no git
// user@host ~/dir
const RKJ: TemplateDef = TemplateDef {
    name: "rkj",
    segments: &[
        field(User, Cyan),
        lit("@", White),
        field(ShortHostname, Cyan),
        text(" "),
        field_bold(Cwd, Blue),
    ],
    rprompt: &[],
};

// Source: rkj-repos.zsh-theme — no git
// user@host ~/dir
const RKJ_REPOS: TemplateDef = TemplateDef {
    name: "rkj-repos",
    segments: &[
        field(User, Cyan),
        lit("@", White),
        field(ShortHostname, Cyan),
        text(" "),
        field_bold(Cwd, Blue),
    ],
    rprompt: &[],
};

// Source: sammy.zsh-theme
// user@host:~/dir (branch*)
const SAMMY: TemplateDef = TemplateDef {
    name: "sammy",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Yellow),
        lit(":", White),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, White),
            dirty(")", "*)", White, White),
        ]),
    ],
    rprompt: &[],
};

// Source: simonoff.zsh-theme
// user@host ~/dir [branch]
const SIMONOFF: TemplateDef = TemplateDef {
    name: "simonoff",
    segments: &[
        field(User, Yellow),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("[", White),
            field(GitBranch, White),
            lit("]", White),
        ]),
    ],
    rprompt: &[],
};

// Source: simple.zsh-theme
// user@host:~/dir (branch ✗/✔)
const SIMPLE: TemplateDef = TemplateDef {
    name: "simple",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        lit(":", White),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit_bold("(", Blue),
            field_bold(GitBranch, Blue),
            dirty(" ✔", " ✗", Green, Red),
            lit_bold(")", Blue),
        ]),
    ],
    rprompt: &[],
};

// Source: skaro.zsh-theme
// ┌ [~/dir] git:(branch) ✗
const SKARO: TemplateDef = TemplateDef {
    name: "skaro",
    segments: &[
        lit("┌", Blue),
        text(" "),
        field_fmt_bold(Cwd, Cyan, "[", "]"),
        if_git(GIT_COLON_PAREN_RED),
    ],
    rprompt: &[],
};

// Source: smt.zsh-theme
// host 福 ~/dir |branch ✓/⚡
const SMT: TemplateDef = TemplateDef {
    name: "smt",
    segments: &[
        field(ShortHostname, Blue),
        text(" 福 "),
        field(Cwd, Cyan),
        if_git(&[
            lit("|", White),
            field(GitBranch, White),
            dirty(" ✓", " ⚡", Green, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: sonicradish.zsh-theme
// [~/dir] :branch ✔/✘:
const SONICRADISH: TemplateDef = TemplateDef {
    name: "sonicradish",
    segments: &[
        lit("[", White),
        field_bold(Cwd, Green),
        lit("]", White),
        if_git(&[
            text(" "),
            lit(":", White),
            field(GitBranch, White),
            dirty(" ✔:", " ✘:", White, White),
        ]),
    ],
    rprompt: &[],
};

// Source: sorin.zsh-theme
// dir git:branch
const SORIN: TemplateDef = TemplateDef {
    name: "sorin",
    segments: &[
        field(CwdBasename, Cyan),
        if_git(GIT_COLON_SORIN),
    ],
    rprompt: &[],
};

// Source: sporty_256.zsh-theme — 256-color
// ±|branch ✘/✔| dir
const SPORTY_256: TemplateDef = TemplateDef {
    name: "sporty_256",
    segments: &[
        if_git(&[
            lit("±|", Color256(154)),
            field(GitBranch, Color256(124)),
            dirty(" ✔", " ✘", Green, Red),
            lit("|", Color256(154)),
            text(" "),
        ]),
        field(CwdBasename, Color256(208)),
    ],
    rprompt: &[],
};

// Source: steeef.zsh-theme — uses vcs_info with 256-color
// user at host in ~/dir (branch●)
const STEEEF: TemplateDef = TemplateDef {
    name: "steeef",
    segments: &[
        field(User, Color256(135)),
        text(" in "),
        field(Cwd, Color256(118)),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Color256(81)),
            dirty("", "●", Color256(81), Color256(166)),
            lit(")", White),
        ]),
        text(" "),
        lit("λ", Color256(166)),
    ],
    rprompt: &[],
};

// Source: strug.zsh-theme
// ~/dir on branch ✔/✘
const STRUG: TemplateDef = TemplateDef {
    name: "strug",
    segments: &[
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit_bold("on ", Yellow),
            field_bold(GitBranch, Yellow),
            dirty(" ✔", " ✘", Green, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: sunaku.zsh-theme
// ~/dir branch
const SUNAKU: TemplateDef = TemplateDef {
    name: "sunaku",
    segments: &[
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            field(GitBranch, Magenta),
        ]),
    ],
    rprompt: &[],
};

// Source: sunrise.zsh-theme — custom git function
// [~/dir] ‹branch*›
const SUNRISE: TemplateDef = TemplateDef {
    name: "sunrise",
    segments: &[
        field_fmt(Cwd, White, "[", "]"),
        if_git(&[
            text(" "),
            lit("‹", White),
            field(GitBranch, White),
            dirty("", "*", White, White),
            lit("›", White),
        ]),
    ],
    rprompt: &[],
};

// Source: superjarin.zsh-theme
// ~/dir <branch> ✗
const SUPERJARIN: TemplateDef = TemplateDef {
    name: "superjarin",
    segments: &[
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("<", White),
            field(GitBranch, Magenta),
            dirty(">", "> ✗", White, Yellow),
        ]),
    ],
    rprompt: &[],
};

// Source: suvash.zsh-theme
// ~/dir on branch!
const SUVASH: TemplateDef = TemplateDef {
    name: "suvash",
    segments: &[
        field(Cwd, Green),
        if_git(&[
            text(" "),
            lit("on ", Magenta),
            field(GitBranch, Magenta),
            dirty("", "!", Green, Green),
        ]),
    ],
    rprompt: &[],
};

// Source: takashiyoshida.zsh-theme
// ~/dir on branch!
const TAKASHIYOSHIDA: TemplateDef = TemplateDef {
    name: "takashiyoshida",
    segments: &[
        field(Cwd, Green),
        if_git(&[
            text(" "),
            lit("on ", Magenta),
            field(GitBranch, Magenta),
            dirty("", "!", Green, Green),
        ]),
    ],
    rprompt: &[],
};

// Source: terminalparty.zsh-theme — git in RPROMPT
// user@host ~/dir (branch⚡)
const TERMINALPARTY: TemplateDef = TemplateDef {
    name: "terminalparty",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field(Cwd, White),
        if_git(&[
            text(" "),
            lit("(", Yellow),
            field(GitBranch, Yellow),
            dirty(")", "⚡)", Yellow, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: theunraveler.zsh-theme — git in RPROMPT
// ~/dir branch ➜
const THEUNRAVELER: TemplateDef = TemplateDef {
    name: "theunraveler",
    segments: &[
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            field(GitBranch, Magenta),
        ]),
        text(" "),
        lit("➜", Green),
    ],
    rprompt: &[],
};

// Source: tjkirch.zsh-theme
// [~/dir] branch⚡ [HH:MM:SS]
const TJKIRCH: TemplateDef = TemplateDef {
    name: "tjkirch",
    segments: &[
        lit("[", White),
        field(Cwd, Cyan),
        lit("]", White),
        if_git(&[
            text(" "),
            field(GitBranch, Green),
            dirty("", "⚡", Green, Red),
        ]),
    ],
    rprompt: RPROMPT_TIME_BRACKET,
};

// Source: tjkirch_mod.zsh-theme
// [~/dir] branch⚡ [HH:MM:SS]
const TJKIRCH_MOD: TemplateDef = TemplateDef {
    name: "tjkirch_mod",
    segments: &[
        lit("[", White),
        field(Cwd, Cyan),
        lit("]", White),
        if_git(&[
            text(" "),
            field(GitBranch, Green),
            dirty("", "⚡", Green, Red),
        ]),
    ],
    rprompt: RPROMPT_TIME_BRACKET,
};

// Source: tonotdo.zsh-theme
// ~/dir (branch✗/) [HH:MM:SS]
const TONOTDO: TemplateDef = TemplateDef {
    name: "tonotdo",
    segments: &[
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit_bold("(", Blue),
            field(GitBranch, Blue),
            dirty(")", "✗)", Blue, Yellow),
        ]),
    ],
    rprompt: RPROMPT_TIME_BRACKET,
};

// Source: trapd00r.zsh-theme — uses vcs_info
// user@host ~/dir (branch●)
const TRAPD00R: TemplateDef = TemplateDef {
    name: "trapd00r",
    segments: &[
        field(User, Magenta),
        lit("@", White),
        field(ShortHostname, Cyan),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(GIT_VCS_INFO),
    ],
    rprompt: &[],
};

// Source: wedisagree.zsh-theme — complex: git in RPROMPT
// ~/dir ☁ branch
const WEDISAGREE: TemplateDef = TemplateDef {
    name: "wedisagree",
    segments: &[
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("☁", Red),
            text(" "),
            field(GitBranch, Magenta),
        ]),
    ],
    rprompt: &[],
};

// Source: wezm.zsh-theme
// ~/dir (branch)⚡
const WEZM: TemplateDef = TemplateDef {
    name: "wezm",
    segments: &[
        field(Cwd, Green),
        if_git(&[
            text(" "),
            lit("(", Blue),
            field(GitBranch, Blue),
            dirty(")", ")⚡", Blue, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: wezm+.zsh-theme
// ~/dir (branch)✗
const WEZM_PLUS: TemplateDef = TemplateDef {
    name: "wezm+",
    segments: &[
        field(Cwd, Green),
        if_git(&[
            text(" "),
            lit("(", Blue),
            field(GitBranch, Blue),
            dirty(")", ")✗", Blue, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: wuffers.zsh-theme
// user@host ~/dir [branch x]
const WUFFERS: TemplateDef = TemplateDef {
    name: "wuffers",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit_bold("[", Blue),
            field_bold(GitBranch, Blue),
            dirty("]", " x]", Blue, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: xiong-chiamiov.zsh-theme — no git
// [~/dir] user@host
const XIONG_CHIAMIOV: TemplateDef = TemplateDef {
    name: "xiong-chiamiov",
    segments: &[
        field_fmt_bold(Cwd, Blue, "[", "]"),
        text(" "),
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
    ],
    rprompt: &[],
};

// Source: xiong-chiamiov-plus.zsh-theme
// [~/dir] user@host branch
const XIONG_CHIAMIOV_PLUS: TemplateDef = TemplateDef {
    name: "xiong-chiamiov-plus",
    segments: &[
        field_fmt_bold(Cwd, Blue, "[", "]"),
        text(" "),
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        if_git(&[
            text(" "),
            field(GitBranch, White),
        ]),
    ],
    rprompt: &[],
};

// Source: zhann.zsh-theme — uses vcs_info
// ~/dir (branch●)
const ZHANN: TemplateDef = TemplateDef {
    name: "zhann",
    segments: &[
        field_bold(Cwd, Blue),
        if_git(GIT_VCS_INFO),
    ],
    rprompt: &[],
};

// ---------------------------------------------------------------------------
// Template registry
// ---------------------------------------------------------------------------

pub static TEMPLATES: &[&TemplateDef] = &[
    // Original themes
    &YS,
    &ROBBYRUSSELL,
    // agnoster uses manual impl (agnoster.rs)
    &AF_MAGIC,
    &BIRA,
    &BUREAU,
    &CANDY,
    &DALLAS,
    &GALLOIS,
    &MARAN,
    // All additional themes (alphabetical)
    &THREEDEN,
    &SOLIAH,
    &ADBEN,
    &AFOWLER,
    &ALANPEABODY,
    &AMUSE,
    &APPLE,
    &ARROW,
    &AUSSIEGEEK,
    &AVIT,
    &AWESOMEPANDA,
    &BLINKS,
    &CANDY_KINGDOM,
    &CLEAN,
    &CLOUD,
    &CRCANDY,
    &CRUNCH,
    &CYPHER,
    &DARKBLOOD,
    &DAVEVERWER,
    &DIETER,
    &DOGENPUNK,
    &DPOGGI,
    &DST,
    &DSTUFFT,
    &DUELLJ,
    &EASTWOOD,
    &EDVARDM,
    &EMOTTY,
    &ESSEMBEH,
    &EVAN,
    &FINO,
    &FINO_TIME,
    &FISHY,
    &FLAZZ,
    &FLETCHERM,
    &FOX,
    &FRISK,
    &FRONTCUBE,
    &FUNKY,
    &FWALCH,
    &GALLIFREY,
    &GARYBLESSINGTON,
    &GENTOO,
    &GEOFFGARSIDE,
    &GIANU,
    &GNZH,
    &GOZILLA,
    &HALF_LIFE,
    &HUMZA,
    &IMAJES,
    &INTHELOOP,
    &ITCHY,
    &JAISCHEEMA,
    &JBERGANTINE,
    &JISPWOSO,
    &JNROWE,
    &JONATHAN,
    &JOSH,
    &JREESE,
    &JTRILEY,
    &JUANGHURTADO,
    &JUNKFOOD,
    &KAFEITU,
    &KARDAN,
    &KENNETHREITZ,
    &KIWI,
    &KOLO,
    &KPHOEN,
    &LAMBDA,
    &LINUXONLY,
    &LUKERANDALL,
    &MACOVSKY,
    &MACOVSKY_RUBY,
    &MGUTZ,
    &MH,
    &MICHELEBOLOGNA,
    &MIKEH,
    &MILOSHADZIC,
    &MINIMAL,
    &MIRA,
    &MLH,
    &MORTALSCUMBAG,
    &MRTAZZ,
    &MURILASSO,
    &MUSE,
    &NANOTECH,
    &NEBIRHOS,
    &NICOULAJ,
    &NORM,
    &OBRAUN,
    &OLDGALLOIS,
    &PEEPCODE,
    &PHILIPS,
    &PMCGEE,
    &PYGMALION,
    &PYGMALION_VIRTUALENV,
    &RE5ET,
    &REFINED,
    &RGM,
    &RISTO,
    &RIXIUS,
    &RKJ,
    &RKJ_REPOS,
    &SAMMY,
    &SIMONOFF,
    &SIMPLE,
    &SKARO,
    &SMT,
    &SONICRADISH,
    &SORIN,
    &SPORTY_256,
    &STEEEF,
    &STRUG,
    &SUNAKU,
    &SUNRISE,
    &SUPERJARIN,
    &SUVASH,
    &TAKASHIYOSHIDA,
    &TERMINALPARTY,
    &THEUNRAVELER,
    &TJKIRCH,
    &TJKIRCH_MOD,
    &TONOTDO,
    &TRAPD00R,
    &WEDISAGREE,
    &WEZM,
    &WEZM_PLUS,
    &WUFFERS,
    &XIONG_CHIAMIOV,
    &XIONG_CHIAMIOV_PLUS,
    &ZHANN,
];

pub fn get_template(name: &str) -> Option<&'static TemplateDef> {
    TEMPLATES.iter().find(|t| t.name == name).copied()
}
