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
// user@host [HH:MM:SS] [~/dir] [branch *] ->
const CANDY: TemplateDef = TemplateDef {
    name: "candy",
    segments: &[
        field_bold(User, Green),
        lit_bold("@", Green),
        field_bold(ShortHostname, Green),
        text(" "),
        field_fmt(Time, Blue, "[", "]"),
        text(" "),
        field_fmt(Cwd, White, "[", "]"),
        if_git(GIT_BRACKET_GREEN),
        text(" "),
        lit("->", Blue),
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

// Source: maran.zsh-theme — uses %M (full hostname) and %/
// user@HOST:~/dir git:(branch) $
const MARAN: TemplateDef = TemplateDef {
    name: "maran",
    segments: &[
        field(User, Cyan),
        lit("@", White),
        field(Hostname, Yellow),
        lit(":", White),
        field(Cwd, Green),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Cyan, "git:(", ")"),
        ]),
        text(" $ "),
    ],
    rprompt: &[],
};

// ===== Additional themes (alphabetical) ====================================

// Source: 3den.zsh-theme
// ~/dir (branch *) [HH:MM:SS] user$
const THREEDEN: TemplateDef = TemplateDef {
    name: "3den",
    segments: &[
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, White),
            dirty(")", "*)", White, White),
        ]),
        text(" "),
        field_fmt(Time, Cyan, "[", "]"),
        text(" "),
        field_bold(User, Green),
        text("$"),
    ],
    rprompt: &[],
};

// Source: Soliah.zsh-theme — complex: check_git_prompt_info + git_time_since_commit
// user on host in ~/dir (branch*)
const SOLIAH: TemplateDef = TemplateDef {
    name: "Soliah",
    segments: &[
        field(User, Blue),
        text(" on "),
        field(Hostname, Red),
        text(" in "),
        field(Cwd, Blue),
        if_git(&[
            lit("(", White),
            field(GitBranch, White),
            dirty(")", "*)", White, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: adben.zsh-theme
// PROMPT: user@host ~/dir    RPROMPT: ‹git:branch ✘/✔› time
const ADBEN: TemplateDef = TemplateDef {
    name: "adben",
    segments: &[
        field(User, Red),
        lit("@", Red),
        field(ShortHostname, Red),
        text(" "),
        field_bold(Cwd, Yellow),
    ],
    rprompt: RPROMPT_ADBEN,
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

// Source: alanpeabody.zsh-theme — git in RPROMPT
// PROMPT: user@host ~/dir$    RPROMPT: branch (green)
const ALANPEABODY: TemplateDef = TemplateDef {
    name: "alanpeabody",
    segments: &[
        field(User, Magenta),
        lit("@", Magenta),
        field(ShortHostname, Magenta),
        text(" "),
        field(Cwd, Blue),
        text("$ "),
    ],
    rprompt: &[
        if_git(&[
            field(GitBranch, Green),
        ]),
    ],
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
// PROMPT: dir ➤    RPROMPT: git:branch*
const ARROW: TemplateDef = TemplateDef {
    name: "arrow",
    segments: &[
        field(CwdBasename, Yellow),
        text(" "),
        lit("➤", Yellow),
    ],
    rprompt: RPROMPT_GIT_ARROW,
};

// Source: aussiegeek.zsh-theme
// [time] [user@host:~/dir(branch✗/✔)]
const AUSSIEGEEK: TemplateDef = TemplateDef {
    name: "aussiegeek",
    segments: &[
        field_fmt(Time, Red, "[", "]"),
        text(" "),
        lit("[", Blue),
        field(User, Red),
        lit("@", Red),
        field(ShortHostname, Red),
        lit(":", Red),
        field(Cwd, Red),
        if_git(&[
            lit_bold("(", Green),
            field_bold(GitBranch, Green),
            dirty("✔", "✗", Green, Green),
            lit_bold(")", Green),
        ]),
        lit("]", Blue),
    ],
    rprompt: &[],
};

// Source: avit.zsh-theme
// %3~ branch ✔/✗
const AVIT: TemplateDef = TemplateDef {
    name: "avit",
    segments: &[
        field_bold(CwdTruncated(3), Blue),
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
// user@host ~/dir [branch *]
const BLINKS: TemplateDef = TemplateDef {
    name: "blinks",
    segments: &[
        field_bold(User, Green),
        lit_bold("@", Blue),
        field_bold(ShortHostname, Cyan),
        text(" "),
        field(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit_bold("[", Blue),
            field_bold(GitBranch, Blue),
            dirty("]", " *]", Green, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: candy-kingdom.zsh-theme
// user@host:~/dir (branch: branch!) $
const CANDY_KINGDOM: TemplateDef = TemplateDef {
    name: "candy-kingdom",
    segments: &[
        field(User, Magenta),
        lit("@", White),
        field(ShortHostname, Yellow),
        lit(":", White),
        field_bold(Cwd, Green),
        if_git(&[
            text(" "),
            lit("(", Magenta),
            field_fmt(GitBranch, Magenta, "branch: ", ""),
            dirty(")", "!)", Magenta, Red),
        ]),
        text(" $ "),
    ],
    rprompt: &[],
};

// Source: clean.zsh-theme
// user:dir/ (branch✗) $ [HH:MM:SS]
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
        text(" $"),
    ],
    rprompt: RPROMPT_TIME_BRACKET,
};

// Source: cloud.zsh-theme
// ☁ dir [branch]⚡ %
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
        lit_bold(" %", Blue),
    ],
    rprompt: &[],
};

// Source: crcandy.zsh-theme (same git as candy)
// user@host [HH:MM:SS] [~/dir] [branch *] ->
const CRCANDY: TemplateDef = TemplateDef {
    name: "crcandy",
    segments: &[
        field_bold(User, Green),
        lit_bold("@", Green),
        field_bold(ShortHostname, Green),
        text(" "),
        field_fmt(Time, Blue, "[", "]"),
        text(" "),
        field_fmt(Cwd, White, "[", "]"),
        if_git(GIT_BRACKET_GREEN),
        text(" "),
        lit("->", Blue),
    ],
    rprompt: &[],
};

// Source: crunch.zsh-theme — uses {%T} time and ➭ prompt
// {HH:MM:SS} ~/dir :branch ✓/✗ ➭
const CRUNCH: TemplateDef = TemplateDef {
    name: "crunch",
    segments: &[
        field_fmt(Time, Cyan, "{", "}"),
        text(" "),
        field(Cwd, Yellow),
        if_git(&[
            lit(":", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Red),
        ]),
        text(" "),
        lit("➭", White),
    ],
    rprompt: &[],
};

// Source: cypher.zsh-theme — no git
// host :: %3~ »
const CYPHER: TemplateDef = TemplateDef {
    name: "cypher",
    segments: &[
        field(ShortHostname, White),
        lit_bold(" :: ", Red),
        field(CwdTruncated(3), Green),
        text(" "),
        lit_bold("»", Blue),
    ],
    rprompt: &[],
};

// Source: dallas.zsh-theme — already defined above as DALLAS

// Source: darkblood.zsh-theme — multiline box: ┌[user@host] [tty] [branch ⚡] / └[~/dir]>
// ┌[user@host] [branch ⚡] └[~/dir]>
const DARKBLOOD: TemplateDef = TemplateDef {
    name: "darkblood",
    segments: &[
        lit("┌[", Red),
        field(User, Red),
        lit("@", White),
        field(ShortHostname, Red),
        lit("]", Red),
        if_git(&[
            text(" "),
            lit("[", Red),
            field_bold(GitBranch, White),
            dirty("]", " ⚡]", Red, Red),
        ]),
        text(" "),
        lit("└[", Red),
        field(Cwd, Yellow),
        lit("]>", Red),
    ],
    rprompt: &[],
};

// Source: daveverwer.zsh-theme
// host:dir (branch) $
const DAVEVERWER: TemplateDef = TemplateDef {
    name: "daveverwer",
    segments: &[
        field(ShortHostname, Red),
        lit(":", White),
        field(CwdBasename, Green),
        if_git(&[
            text(" "),
            lit("(", Blue),
            field(GitBranch, Blue),
            lit(")", Blue),
        ]),
        text(" $ "),
    ],
    rprompt: &[],
};

// Source: dieter.zsh-theme
// HH:MM:SS user@host dir branch?
const DIETER: TemplateDef = TemplateDef {
    name: "dieter",
    segments: &[
        field(Time, Green),
        text(" "),
        field(User, Blue),
        lit("@", White),
        field(ShortHostname, White),
        text(" "),
        field(CwdBasename, Blue),
        if_git(&[
            text(" "),
            field(GitBranch, Yellow),
            dirty("", "?", Green, Yellow),
        ]),
    ],
    rprompt: &[],
};

// Source: dogenpunk.zsh-theme — complex: custom git_time_since_commit
// host ॐ ~/dir:git@branch!) (simplified)
const DOGENPUNK: TemplateDef = TemplateDef {
    name: "dogenpunk",
    segments: &[
        field(ShortHostname, Blue),
        text(" "),
        lit_bold("ॐ", White),
        text(" "),
        field(Cwd, Cyan),
        lit(":", White),
        if_git(&[
            lit_bold("git", Green),
            lit("@", White),
            field_bold(GitBranch, Black),
            dirty(")", "!)", Green, Red),
        ]),
    ],
    rprompt: &[],
};

// Source: dpoggi.zsh-theme
// user@host:~/dir (branch○/⚡) »
const DPOGGI: TemplateDef = TemplateDef {
    name: "dpoggi",
    segments: &[
        field(User, Green),
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
        text(" "),
        lit("»", Red),
    ],
    rprompt: &[],
};

// Source: dst.zsh-theme
// user@host: ~/dir branch! [HH:MM:SS] (green)
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
    rprompt: &[
        lit("[", Green),
        field(Time, Green),
        lit("]", Green),
    ],
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

// Source: duellj.zsh-theme — box-drawing layout
// PROMPT: ╭─[user@host] - [~/dir] - [!]    RPROMPT: [HH:MM:SS]
const DUELLJ: TemplateDef = TemplateDef {
    name: "duellj",
    segments: &[
        lit("┌─[", Blue),
        field_bold(User, Green),
        lit_bold("@", Blue),
        field(ShortHostname, Cyan),
        lit("]", Blue),
        text(" - "),
        lit("[", Blue),
        field_bold(Cwd, White),
        lit("]", Blue),
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
// ➜ dir git:(branch) ✗ %
const EDVARDM: TemplateDef = TemplateDef {
    name: "edvardm",
    segments: &[
        lit_bold("➜", Red),
        text(" "),
        field(CwdBasename, White),
        if_git(&[
            text(" "),
            lit("git:(", Red),
            field(GitBranch, Red),
            dirty(")", ") ✗", Blue, Yellow),
        ]),
        text(" "),
        lit_bold("%", Blue),
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
// user@HOST ~/dir (branch) $
const ESSEMBEH: TemplateDef = TemplateDef {
    name: "essembeh",
    segments: &[
        field(User, Green),
        lit("@", Green),
        field(Hostname, Green),
        lit(":", White),
        field_bold(Cwd, Yellow),
        text(" "),
        if_git(&[
            lit("(", Cyan),
            field(GitBranch, Cyan),
            lit(")", Cyan),
            text(" "),
        ]),
        text("$ "),
    ],
    rprompt: &[],
};

// Source: evan.zsh-theme — no git
// host :: %2~ »
const EVAN: TemplateDef = TemplateDef {
    name: "evan",
    segments: &[
        field(ShortHostname, White),
        lit(" :: ", Red),
        field(CwdTruncated(2), Green),
        text(" "),
        lit_bold("»", White),
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

// Source: fishy.zsh-theme
// PROMPT: user@host dir>    RPROMPT: branch
const FISHY: TemplateDef = TemplateDef {
    name: "fishy",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, White),
        text(" "),
        field(Cwd, Green),
        lit(">", White),
    ],
    rprompt: RPROMPT_GIT_FISHY,
};

// Source: flazz.zsh-theme
// host :: %3~ ‹branch›
const FLAZZ: TemplateDef = TemplateDef {
    name: "flazz",
    segments: &[
        field(ShortHostname, White),
        lit_bold(" :: ", Magenta),
        field(CwdTruncated(3), Green),
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
// user•%3~(branch⚡)» [HH:MM:SS]
const FLETCHERM: TemplateDef = TemplateDef {
    name: "fletcherm",
    segments: &[
        field(User, Cyan),
        lit("•", Magenta),
        field(CwdTruncated(3), Green),
        if_git(&[
            lit_bold("(", Blue),
            field(GitBranch, Red),
            dirty(")", "⚡)", Blue, Yellow),
        ]),
        lit("»", White),
    ],
    rprompt: RPROMPT_TIME_BRACKET,
};

// Source: fox.zsh-theme — box-drawing
// ┌[user☮HOST](~/dir)-[git://branch ✗/✔]-
const FOX: TemplateDef = TemplateDef {
    name: "fox",
    segments: &[
        lit("┌[", Cyan),
        field_bold(User, White),
        lit("☮", Cyan),
        field_bold(Hostname, White),
        lit("](", Cyan),
        field_bold(Cwd, White),
        lit(")", Cyan),
        if_git(&[
            lit("-[", Cyan),
            lit("git://", White),
            field_bold(GitBranch, White),
            dirty(" ✔", " ✗", Green, Red),
            lit("]-", Cyan),
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

// Source: frontcube.zsh-theme
// PROMPT: ~/dir ➞    RPROMPT: [git:branch] ✔/✖
const FRONTCUBE: TemplateDef = TemplateDef {
    name: "frontcube",
    segments: &[
        field(Cwd, Gray),
        text(" "),
        lit("➞", Green),
    ],
    rprompt: RPROMPT_GIT_FRONTCUBE,
};

// Source: funky.zsh-theme — multiline box-drawing, no git
// ╭─[~/dir]-[user@host] %
const FUNKY: TemplateDef = TemplateDef {
    name: "funky",
    segments: &[
        lit("╭─[", Blue),
        field(Cwd, White),
        lit("]-[", Blue),
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Cyan),
        lit("]", Blue),
        text(" % "),
    ],
    rprompt: &[],
};

// Source: fwalch.zsh-theme
// dir (branch) ✗ %
const FWALCH: TemplateDef = TemplateDef {
    name: "fwalch",
    segments: &[
        field(CwdBasename, Cyan),
        if_git(&[
            text(" "),
            lit("(", Red),
            field(GitBranch, Red),
            dirty(")", ") ✗", Blue, Yellow),
        ]),
        text(" "),
        lit_bold("%", Blue),
    ],
    rprompt: &[],
};

// Source: gallifrey.zsh-theme
// host %2~ ‹branch› »
const GALLIFREY: TemplateDef = TemplateDef {
    name: "gallifrey",
    segments: &[
        field(ShortHostname, Green),
        text(" "),
        field(CwdTruncated(2), White),
        if_git(GIT_ANGLE_YELLOW),
        text(" "),
        lit_bold("»", White),
    ],
    rprompt: &[],
};

// Source: garyblessington.zsh-theme
// dir(branch) ✗ :
const GARYBLESSINGTON: TemplateDef = TemplateDef {
    name: "garyblessington",
    segments: &[
        field(CwdBasename, Cyan),
        if_git(&[
            lit("(", Blue),
            field(GitBranch, Blue),
            dirty(")", ") ✗", Blue, Red),
        ]),
        text(" "),
        lit(":", Blue),
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
// [time] user:dir git:(branch) $
const GEOFFGARSIDE: TemplateDef = TemplateDef {
    name: "geoffgarside",
    segments: &[
        field_fmt(Time, White, "[", "]"),
        text(" "),
        field(User, Cyan),
        lit(":", White),
        field(CwdBasename, Green),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Yellow, "git:(", ")"),
        ]),
        text(" $ "),
    ],
    rprompt: &[],
};

// Source: gianu.zsh-theme
// [user@host dir (branch ✗)]$
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
            dirty("", " ✗", Green, Yellow),
            lit(")", White),
        ]),
        lit("]", White),
        text("$ "),
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

// Source: gozilla.zsh-theme — git_prompt_status in RPROMPT (simplified)
// ➜ dir (branch) %
const GOZILLA: TemplateDef = TemplateDef {
    name: "gozilla",
    segments: &[
        lit_bold("➜", Red),
        text(" "),
        field(CwdBasename, Cyan),
        if_git(&[
            text(" "),
            lit_bold("(", Blue),
            field_bold(GitBranch, Blue),
            lit_bold(")", Blue),
        ]),
        lit_bold(" %", Blue),
    ],
    rprompt: &[],
};

// Source: half-life.zsh-theme — uses vcs_info with 256-color
// user in ~/dir on branch● λ
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
        text(" "),
        lit("λ", White),
    ],
    rprompt: &[],
};

// Source: humza.zsh-theme — uses {%~}
// user {~/dir} ±(branch); $
const HUMZA: TemplateDef = TemplateDef {
    name: "humza",
    segments: &[
        field(User, Cyan),
        text(" "),
        field_fmt(Cwd, White, "{", "}"),
        if_git(&[
            field_fmt(GitBranch, Red, "±(", ");"),
        ]),
        text(" $ "),
    ],
    rprompt: &[],
};

// Source: imajes.zsh-theme — just a % prompt
// %
const IMAJES: TemplateDef = TemplateDef {
    name: "imajes",
    segments: &[
        lit("%", Red),
    ],
    rprompt: &[],
};

// Source: intheloop.zsh-theme
// [user@host] %10c (branch⚡/) ❯
const INTHELOOP: TemplateDef = TemplateDef {
    name: "intheloop",
    segments: &[
        lit_bold("[", Gray),
        field_bold(User, Green),
        lit_bold("@", Green),
        field_bold(ShortHostname, Green),
        lit_bold("]", Gray),
        text(" "),
        field_bold(CwdTruncated(10), Blue),
        if_git(&[
            text(" "),
            lit("(", Gray),
            field(GitBranch, Red),
            dirty(")", " ⚡)", Gray, Yellow),
        ]),
        text(" "),
        lit_bold("❯", Cyan),
    ],
    rprompt: &[],
};

// Source: itchy.zsh-theme
// PROMPT: user@host ~/dir    RPROMPT: branch ✗/✔
const ITCHY: TemplateDef = TemplateDef {
    name: "itchy",
    segments: &[
        field(User, Cyan),
        lit("@", Cyan),
        field(ShortHostname, Cyan),
        text(" "),
        field(Cwd, Yellow),
    ],
    rprompt: RPROMPT_GIT_ITCHY,
};

// Source: jaischeema.zsh-theme
// host at ~/dir ±(branch) ✗ ❯
const JAISCHEEMA: TemplateDef = TemplateDef {
    name: "jaischeema",
    segments: &[
        field_bold(ShortHostname, Magenta),
        text(" at "),
        field_bold(Cwd, Green),
        if_git(&[
            text(" "),
            lit_bold("±(", Blue),
            field(GitBranch, Red),
            dirty(")", ") ✗", Blue, Yellow),
        ]),
        text(" "),
        lit("❯", Red),
    ],
    rprompt: &[],
};

// Source: jbergantine.zsh-theme
// ➜ dir git:(branch) ✗ %
const JBERGANTINE: TemplateDef = TemplateDef {
    name: "jbergantine",
    segments: &[
        lit_bold("➜", Red),
        text(" "),
        field(CwdBasename, Cyan),
        if_git(&[
            text(" "),
            lit_bold("git:(", White),
            field(GitBranch, Red),
            dirty(")", ") ✗", White, Yellow),
        ]),
        text(" "),
        lit_bold("%", White),
    ],
    rprompt: &[],
};

// Source: jispwoso.zsh-theme — multiline, uses %/
// user@host: ~/dir git:(branch) ✗ ➜
const JISPWOSO: TemplateDef = TemplateDef {
    name: "jispwoso",
    segments: &[
        field(User, Green),
        lit("@", Green),
        field(ShortHostname, Green),
        lit(":", White),
        text(" "),
        field(Cwd, Blue),
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
// user@host ~/dir ±branch⚡ »
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
        text(" "),
        lit("»", Red),
    ],
    rprompt: &[],
};

// Source: jtriley.zsh-theme — no git, uses %d (full path)
// HH:MM:SS user@host ~/dir %%
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
        text(" "),
        lit_bold("%%", Yellow),
    ],
    rprompt: &[],
};

// Source: juanghurtado.zsh-theme
// PROMPT: user@host:~/dir(*)    RPROMPT: branch(*)
const JUANGHURTADO: TemplateDef = TemplateDef {
    name: "juanghurtado",
    segments: &[
        field_bold(User, Green),
        lit("@", White),
        field_bold(ShortHostname, Green),
        lit(":", White),
        field(Cwd, Yellow),
        if_git(&[
            dirty("", "(*)", Green, Red),
        ]),
    ],
    rprompt: RPROMPT_GIT_JUANGHURTADO,
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
// ➜ user@host ~/dir git:(branch) ✗ %
const KAFEITU: TemplateDef = TemplateDef {
    name: "kafeitu",
    segments: &[
        lit_bold("➜", Red),
        text(" "),
        field_bold(User, Green),
        lit("@", Cyan),
        field_bold(ShortHostname, Green),
        text(" "),
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit_bold("git:(", Blue),
            field(GitBranch, Red),
            dirty(")", ") ✗", Blue, Yellow),
        ]),
        text(" "),
        lit_bold("%", Blue),
    ],
    rprompt: &[],
};

// Source: kardan.zsh-theme
// PROMPT: >    RPROMPT: ~/dir(branch✗)@host
const KARDAN: TemplateDef = TemplateDef {
    name: "kardan",
    segments: &[
        lit(">", White),
    ],
    rprompt: RPROMPT_KARDAN,
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

// Source: kiwi.zsh-theme — uses %2~
// ┌[kiwish]-(%2~)-[git:branch]-
const KIWI: TemplateDef = TemplateDef {
    name: "kiwi",
    segments: &[
        lit_bold("┌[", Green),
        lit_bold("kiwish", Cyan),
        lit_bold("]-(", Green),
        field_bold(CwdTruncated(2), White),
        lit_bold(")", Green),
        if_git(&[
            lit_bold("-[", Green),
            lit("git:", White),
            field_bold(GitBranch, White),
            lit_bold("]-", Green),
        ]),
    ],
    rprompt: &[],
};

// Source: kolo.zsh-theme — uses vcs_info
// dir (branch●) %%
const KOLO: TemplateDef = TemplateDef {
    name: "kolo",
    segments: &[
        field_bold(CwdBasename, Magenta),
        if_git(GIT_VCS_INFO),
        text(" "),
        lit_bold("%%", Magenta),
    ],
    rprompt: &[],
};

// Source: kphoen.zsh-theme
// [user@host:~/dir on branch] %
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
        text(" % "),
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

// Source: lukerandall.zsh-theme — uses %2~
// user@host %2~ (branch) »
const LUKERANDALL: TemplateDef = TemplateDef {
    name: "lukerandall",
    segments: &[
        field_bold(User, Green),
        lit_bold("@", Green),
        field_bold(ShortHostname, Green),
        text(" "),
        field_bold(CwdTruncated(2), Blue),
        if_git(GIT_PAREN_YELLOW),
        text(" "),
        lit_bold("»", White),
    ],
    rprompt: &[],
};

// Source: macovsky.zsh-theme
// ~/dir ‹branch› $
const MACOVSKY: TemplateDef = TemplateDef {
    name: "macovsky",
    segments: &[
        field(Cwd, Green),
        if_git(GIT_ANGLE_YELLOW),
        text(" "),
        lit_bold("$", White),
    ],
    rprompt: &[],
};

// Source: macovsky-ruby.zsh-theme — same as macovsky
// ~/dir ‹branch› $
const MACOVSKY_RUBY: TemplateDef = TemplateDef {
    name: "macovsky-ruby",
    segments: &[
        field(Cwd, Green),
        if_git(GIT_ANGLE_YELLOW),
        text(" "),
        lit_bold("$", White),
    ],
    rprompt: &[],
};

// Source: mgutz.zsh-theme — uses %1~ and %#
// dir [branch*] %
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
        text(" "),
        lit_bold("%", Magenta),
    ],
    rprompt: &[],
};

// Source: mh.zsh-theme
// PROMPT: [user:~/dir]$    RPROMPT: (branch✱)
const MH: TemplateDef = TemplateDef {
    name: "mh",
    segments: &[
        lit("[", White),
        field_bold(User, White),
        lit(":", White),
        field(Cwd, Red),
        lit("]", White),
        text("$ "),
    ],
    rprompt: RPROMPT_GIT_MH,
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

// Source: miloshadzic.zsh-theme — no "git:" prefix in reference
// dir|branch⚡ ⇒
const MILOSHADZIC: TemplateDef = TemplateDef {
    name: "miloshadzic",
    segments: &[
        field(CwdBasename, Cyan),
        if_git(&[
            lit("|", Red),
            field(GitBranch, Green),
            dirty("", "⚡", Green, Yellow),
        ]),
        lit(" ⇒", Cyan),
    ],
    rprompt: &[],
};

// Source: minimal.zsh-theme — uses %2~
// %2~ [branch●] »
const MINIMAL: TemplateDef = TemplateDef {
    name: "minimal",
    segments: &[
        field(CwdTruncated(2), White),
        if_git(&[
            text(" "),
            lit("[", White),
            field(GitBranch, White),
            dirty("]", "●]", White, Red),
        ]),
        text(" "),
        lit_bold("»", White),
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

// Source: mlh.zsh-theme — uses %1~ (basename), "in" prefix
// user@host in dir on branch $
const MLH: TemplateDef = TemplateDef {
    name: "mlh",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        lit("in", White),
        text(" "),
        field_bold(CwdBasename, Blue),
        if_git(&[
            text(" "),
            lit("on", White),
            text(" "),
            field(GitBranch, White),
        ]),
        text(" $ "),
    ],
    rprompt: &[],
};

// Source: mortalscumbag.zsh-theme — multiline: user@host ‹branch› : ~/dir / [exit] %
// user@host ‹branch› : ~/dir %
const MORTALSCUMBAG: TemplateDef = TemplateDef {
    name: "mortalscumbag",
    segments: &[
        field_bold(User, Green),
        lit("@", White),
        field_bold(ShortHostname, Green),
        if_git(&[
            text(" "),
            lit("‹", White),
            field_bold(GitBranch, Yellow),
            lit("›", White),
        ]),
        text(" : "),
        field(Cwd, White),
        text(" % "),
    ],
    rprompt: &[],
};

// Source: mrtazz.zsh-theme
// PROMPT: host:dir:$    RPROMPT: <branch ✗> (bold green)
const MRTAZZ: TemplateDef = TemplateDef {
    name: "mrtazz",
    segments: &[
        field_bold(ShortHostname, Red),
        lit(":", White),
        field(CwdBasename, Cyan),
        lit(":", White),
    ],
    rprompt: RPROMPT_GIT_MRTAZZ,
};

// Source: murilasso.zsh-theme
// user@host:~/dir branch ✗/✔ $
const MURILASSO: TemplateDef = TemplateDef {
    name: "murilasso",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Blue),
        lit(":", White),
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            field(GitBranch, White),
            dirty(" ✔", " ✗", Green, Red),
        ]),
        text(" $ "),
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

// Source: nanotech.zsh-theme
// PROMPT: dir [    RPROMPT: branch* ] 12:00 PM
const NANOTECH: TemplateDef = TemplateDef {
    name: "nanotech",
    segments: &[
        field(CwdTruncated(2), Green),
        text(" "),
        lit("[", Blue),
    ],
    rprompt: RPROMPT_NANOTECH,
};

// Source: nebirhos.zsh-theme — uses @%m and %c
// @host ➜ dir git:(branch) ✗
const NEBIRHOS: TemplateDef = TemplateDef {
    name: "nebirhos",
    segments: &[
        lit_bold("@", Red),
        field_bold(ShortHostname, Red),
        text(" "),
        lit_bold("➜", Red),
        text(" "),
        field_bold(CwdBasename, Cyan),
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

// Source: obraun.zsh-theme — uses %3~
// [HH:MM:SS] user :: host ➜ %3~ ‹branch› »
const OBRAUN: TemplateDef = TemplateDef {
    name: "obraun",
    segments: &[
        field_fmt(Time, Green, "[", "]"),
        text(" "),
        field(User, Cyan),
        text(" "),
        lit_bold("::", Blue),
        text(" "),
        field(ShortHostname, Yellow),
        text(" "),
        lit("➜", Magenta),
        text("  "),
        field(CwdTruncated(3), Green),
        if_git(GIT_ANGLE_RED),
        text(" "),
        lit_bold("»", White),
    ],
    rprompt: &[],
};

// Source: oldgallois.zsh-theme — git in RPROMPT
// PROMPT: [~/dir] $    RPROMPT: [branch *]
const OLDGALLOIS: TemplateDef = TemplateDef {
    name: "oldgallois",
    segments: &[
        field_fmt(Cwd, Cyan, "[", "]"),
        text(" "),
        lit_bold("$", Green),
    ],
    rprompt: &[
        if_git(&[
            lit("[", Green),
            field(GitBranch, Green),
            dirty("]", " *]", Green, Red),
        ]),
    ],
};

// Source: peepcode.zsh-theme — custom git_prompt() function
// PROMPT: ~/dir ☺    RPROMPT: branch ✗ (custom, simplified)
const PEEPCODE: TemplateDef = TemplateDef {
    name: "peepcode",
    segments: &[
        field(Cwd, White),
    ],
    rprompt: &[
        if_git(&[
            field_bold(GitBranch, Gray),
            dirty("", " ✗", Gray, Gray),
        ]),
    ],
};

// Source: philips.zsh-theme — uses %n:%c/ (user:basename/)
// user:basename/ (branch*) $    RPROMPT: [HH:MM:SS]
const PHILIPS: TemplateDef = TemplateDef {
    name: "philips",
    segments: &[
        field_bold(User, Green),
        lit(":", White),
        field_fmt_bold(CwdBasename, Blue, "", "/"),
        if_git(&[
            text(" "),
            lit_bold("(", Blue),
            field_bold(GitBranch, Red),
            dirty(")", "*)", Blue, Blue),
        ]),
        text(" $ "),
    ],
    rprompt: &[
        field_fmt(Time, White, "[", "]"),
    ],
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

// Source: pygmalion.zsh-theme — uses %0~ and ⇒ prompt
// user@host:~/dir branch⚡ ⇒
const PYGMALION: TemplateDef = TemplateDef {
    name: "pygmalion",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Blue),
        lit(":", White),
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            field(GitBranch, Green),
            dirty("", "⚡", Green, Yellow),
        ]),
        text(" "),
        lit("⇒", White),
    ],
    rprompt: &[],
};

// Source: pygmalion-virtualenv.zsh-theme — same as pygmalion
// user@host:~/dir branch⚡ ⇒
const PYGMALION_VIRTUALENV: TemplateDef = TemplateDef {
    name: "pygmalion-virtualenv",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Blue),
        lit(":", White),
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            field(GitBranch, Green),
            dirty("", "⚡", Green, Yellow),
        ]),
        text(" "),
        lit("⇒", White),
    ],
    rprompt: &[],
};

// Source: re5et.zsh-theme — multiline: user@host:~/dir branch⚡ / $
// user@host:~/dir branch⚡ $
const RE5ET: TemplateDef = TemplateDef {
    name: "re5et",
    segments: &[
        field(User, Magenta),
        lit("@", White),
        field(ShortHostname, Yellow),
        lit(":", White),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            field(GitBranch, Green),
            dirty("", " ⚡", Green, Red),
        ]),
        text(" $ "),
    ],
    rprompt: &[
        field_fmt(Time, Green, "[", "]"),
    ],
};

// Source: refined.zsh-theme — PROMPT is just ❯, dir/branch shown via precmd
// ~/dir branch ❯
const REFINED: TemplateDef = TemplateDef {
    name: "refined",
    segments: &[
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            field(GitBranch, Magenta),
        ]),
        text(" "),
        lit("❯", Magenta),
    ],
    rprompt: &[],
};

// Source: rgm.zsh-theme — multiline: user@host ~/dir / exit_code branch %%
// user@host ~/dir branch %%
const RGM: TemplateDef = TemplateDef {
    name: "rgm",
    segments: &[
        field(User, White),
        lit("@", White),
        field(ShortHostname, White),
        text(" "),
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            field(GitBranch, Red),
        ]),
        text(" "),
        lit_bold("%%", Blue),
    ],
    rprompt: &[],
};

// Source: risto.zsh-theme — uses %2~
// user@host:%2~ ‹branch› $
const RISTO: TemplateDef = TemplateDef {
    name: "risto",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        lit(":", White),
        field_bold(CwdTruncated(2), Blue),
        if_git(GIT_ANGLE_RED),
        text(" $ "),
    ],
    rprompt: &[],
};

// Source: rixius.zsh-theme — multiline: user in ~/dir on branch! / ± (prompt_char)
// user in ~/dir on branch! ±    RPROMPT: HH:MM
const RIXIUS: TemplateDef = TemplateDef {
    name: "rixius",
    segments: &[
        field(User, White),
        text(" in "),
        field_bold(Cwd, Green),
        if_git(&[
            text(" "),
            lit("on ", Magenta),
            field(GitBranch, Magenta),
            dirty(" √", " !", Green, Magenta),
        ]),
        text(" "),
        lit("±", White),
    ],
    rprompt: &[field(Time, White)],
};

// Source: rkj.zsh-theme — multiline box-drawing, no git
// ┌─[user@host] - [~/dir] - [time]
const RKJ: TemplateDef = TemplateDef {
    name: "rkj",
    segments: &[
        lit("┌─[", Blue),
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Cyan),
        lit("]", Blue),
        text(" - "),
        lit("[", Blue),
        field(Cwd, White),
        lit("]", Blue),
        text(" - "),
        field_fmt(Time, Yellow, "[", "]"),
    ],
    rprompt: &[],
};

// Source: rkj-repos.zsh-theme — multiline box-drawing with git
// ┌─[user@host] - [~/dir] - [time] git
const RKJ_REPOS: TemplateDef = TemplateDef {
    name: "rkj-repos",
    segments: &[
        lit("┌─[", Blue),
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Cyan),
        lit("]", Blue),
        text(" - "),
        lit("[", Blue),
        field(Cwd, White),
        lit("]", Blue),
        text(" - "),
        field_fmt(Time, Yellow, "[", "]"),
        if_git(&[
            text(" "),
            field(GitBranch, White),
        ]),
    ],
    rprompt: &[],
};

// Source: sammy.zsh-theme — uses %c (basename)
// basename(branch*)$ %
const SAMMY: TemplateDef = TemplateDef {
    name: "sammy",
    segments: &[
        field(CwdBasename, White),
        if_git(&[
            lit("(", White),
            field(GitBranch, White),
            dirty(")", "*)", White, White),
        ]),
        text("$ %"),
    ],
    rprompt: &[],
};

// Source: simonoff.zsh-theme — very complex, simplified to visible structure
// user@host %2~ git:(branch) ✗ »
const SIMONOFF: TemplateDef = TemplateDef {
    name: "simonoff",
    segments: &[
        field(User, Blue),
        lit("@", Green),
        field(Hostname, Blue),
        lit(":", White),
        field(CwdTruncated(2), Green),
        if_git(&[
            text(" "),
            lit("[", White),
            field(GitBranch, White),
            lit("]", White),
        ]),
        text(" "),
        lit("»", White),
    ],
    rprompt: &[],
};

// Source: simple.zsh-theme — just %~ and git, no user@host
// ~/dir (branch ✗/✔)
const SIMPLE: TemplateDef = TemplateDef {
    name: "simple",
    segments: &[
        field(Cwd, Green),
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

// Source: skaro.zsh-theme — uses %2~
// %2~ git:(branch) ✗ »
const SKARO: TemplateDef = TemplateDef {
    name: "skaro",
    segments: &[
        field(CwdTruncated(2), Cyan),
        if_git(GIT_COLON_PAREN_RED),
        text(" "),
        lit("»", White),
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

// Source: sonicradish.zsh-theme — uses %m and %c
// host➜ basename :branch ✔/✘: ᐅ
const SONICRADISH: TemplateDef = TemplateDef {
    name: "sonicradish",
    segments: &[
        field(ShortHostname, Color256(208)),
        lit("➜", Color256(208)),
        text("  "),
        field(CwdBasename, Color256(111)),
        if_git(&[
            text(" "),
            lit(":", White),
            field(GitBranch, White),
            dirty(" ✔:", " ✘:", Green, Red),
        ]),
        text(" "),
        lit("ᐅ", White),
    ],
    rprompt: &[],
};

// Source: sorin.zsh-theme
// dir git:branch ❯
const SORIN: TemplateDef = TemplateDef {
    name: "sorin",
    segments: &[
        field(CwdBasename, Cyan),
        if_git(GIT_COLON_SORIN),
        text(" "),
        lit_bold("❯", Green),
    ],
    rprompt: &[],
};

// Source: sporty_256.zsh-theme — 256-color, RPROMPT: user@host
// ±|branch ✘/✔| dir %    RPROMPT: user@host
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
        text(" % "),
    ],
    rprompt: &[
        field_bold(User, Color256(208)),
        lit_bold("@", White),
        field(ShortHostname, Color256(39)),
    ],
};

// Source: steeef.zsh-theme — uses vcs_info with 256-color
// user at host in ~/dir (branch●) $
const STEEEF: TemplateDef = TemplateDef {
    name: "steeef",
    segments: &[
        field(User, Color256(135)),
        text(" at "),
        field(ShortHostname, Color256(135)),
        text(" in "),
        field(Cwd, Color256(118)),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Color256(81)),
            dirty("", "●", Color256(81), Color256(166)),
            lit(")", White),
        ]),
        text(" $ "),
    ],
    rprompt: &[],
};

// Source: strug.zsh-theme — multiline box: ╭─user@host in ~/dir on branch ✘ / ╰$
// ╭─user@host in ~/dir on branch ✘ $
const STRUG: TemplateDef = TemplateDef {
    name: "strug",
    segments: &[
        lit("╭─", Green),
        field(User, Green),
        lit("@", Green),
        field(ShortHostname, Green),
        text(" "),
        lit("in", Yellow),
        text(" "),
        field(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit_bold("on ", Yellow),
            field_bold(GitBranch, Yellow),
            dirty(" ✔", " ✘", Green, Red),
        ]),
        text(" $ "),
    ],
    rprompt: &[],
};

// Source: sunaku.zsh-theme — multiline ╭─user@host in ~/dir on branch ✘/✔ / ╰$
// ╭─user@host in ~/dir on branch ✘/✔ $
const SUNAKU: TemplateDef = TemplateDef {
    name: "sunaku",
    segments: &[
        lit("╭─", Green),
        field(User, Green),
        lit("@", Green),
        field(ShortHostname, Green),
        text(" "),
        lit("in", Yellow),
        text(" "),
        field(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit_bold("on", Yellow),
            text(" "),
            field(GitBranch, Yellow),
            dirty(" ✔", " ✘", Green, Red),
        ]),
        text(" $ "),
    ],
    rprompt: &[],
};

// Source: sunrise.zsh-theme — custom git, git status before branch
// ‹branch*› ~/dir >
const SUNRISE: TemplateDef = TemplateDef {
    name: "sunrise",
    segments: &[
        if_git(&[
            lit("‹", Yellow),
            field(GitBranch, Yellow),
            dirty("", "*", Yellow, Red),
            lit("›", Yellow),
            text(" "),
        ]),
        field(Cwd, White),
        text(" > "),
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
// user at host in ~/dir on branch!
const SUVASH: TemplateDef = TemplateDef {
    name: "suvash",
    segments: &[
        field(User, Blue),
        text(" at "),
        field(ShortHostname, Blue),
        text(" in "),
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

// Source: takashiyoshida.zsh-theme — multiline [host:basename] on branch! / [user]%
// [host:basename] on branch! [user]%
const TAKASHIYOSHIDA: TemplateDef = TemplateDef {
    name: "takashiyoshida",
    segments: &[
        lit("[", Cyan),
        field(ShortHostname, Cyan),
        lit(":", White),
        field(CwdBasename, Yellow),
        lit("]", Cyan),
        if_git(&[
            text(" "),
            lit("on ", Magenta),
            field(GitBranch, Magenta),
            dirty("", "!", Green, Green),
        ]),
        text(" "),
        lit("[", White),
        field(User, White),
        lit("]", White),
        text("% "),
    ],
    rprompt: &[],
};

// Source: terminalparty.zsh-theme
// PROMPT: %%    RPROMPT: dir(branch⚡) host
const TERMINALPARTY: TemplateDef = TemplateDef {
    name: "terminalparty",
    segments: &[
        lit("%%", Green),
    ],
    rprompt: RPROMPT_TERMINALPARTY,
};

// Source: theunraveler.zsh-theme
// PROMPT: [dir]    RPROMPT: branch (magenta)
const THEUNRAVELER: TemplateDef = TemplateDef {
    name: "theunraveler",
    segments: &[
        field_fmt(CwdBasename, Magenta, "[", "]"),
    ],
    rprompt: RPROMPT_GIT_THEUNRAVELER,
};

// Source: tjkirch.zsh-theme — multiline: user@host:~/dir branch⚡ / $
// user@host:~/dir branch⚡ $    RPROMPT: [HH:MM:SS] (green)
const TJKIRCH: TemplateDef = TemplateDef {
    name: "tjkirch",
    segments: &[
        field(User, Magenta),
        lit("@", White),
        field(ShortHostname, Yellow),
        lit(":", White),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            field(GitBranch, Green),
            dirty("", " ⚡", Green, Red),
        ]),
        text(" $ "),
    ],
    rprompt: &[
        lit("[", Green),
        field(Time, Green),
        lit("]", Green),
    ],
};

// Source: tjkirch_mod.zsh-theme — same as tjkirch but single-line
// user@host:~/dir branch⚡ $    RPROMPT: [HH:MM:SS] (green)
const TJKIRCH_MOD: TemplateDef = TemplateDef {
    name: "tjkirch_mod",
    segments: &[
        field(User, Magenta),
        lit("@", White),
        field(ShortHostname, Yellow),
        lit(":", White),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            field(GitBranch, Green),
            dirty("", " ⚡", Green, Red),
        ]),
        text(" $ "),
    ],
    rprompt: &[
        lit("[", Green),
        field(Time, Green),
        lit("]", Green),
    ],
};

// Source: tonotdo.zsh-theme — uses %n➜%3~
// user➜%3~(branch✗) »    RPROMPT: [HH:MM:SS]
const TONOTDO: TemplateDef = TemplateDef {
    name: "tonotdo",
    segments: &[
        field(User, Cyan),
        lit("➜", Magenta),
        field(CwdTruncated(3), Green),
        if_git(&[
            lit_bold("(", Blue),
            field(GitBranch, Red),
            dirty(")", "✗)", Blue, Yellow),
        ]),
        text(" "),
        lit("»", White),
    ],
    rprompt: &[
        field_fmt(Time, White, "[", "]"),
    ],
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

// Source: wedisagree.zsh-theme
// PROMPT: [dir] (magenta)    RPROMPT: time ☁ branch ☂/☀
const WEDISAGREE: TemplateDef = TemplateDef {
    name: "wedisagree",
    segments: &[
        field_fmt(CwdBasename, Magenta, "[", "]"),
    ],
    rprompt: RPROMPT_WEDISAGREE,
};

// Source: wezm.zsh-theme — git first, cwd in RPROMPT
// (branch)⚡ %    RPROMPT: ~/dir
const WEZM: TemplateDef = TemplateDef {
    name: "wezm",
    segments: &[
        if_git(&[
            lit("(", Blue),
            field(GitBranch, Blue),
            dirty(")", ")⚡", Blue, Red),
            text(" "),
        ]),
        lit("%", Yellow),
    ],
    rprompt: &[
        field(Cwd, Green),
    ],
};

// Source: wezm+.zsh-theme — user@host + git, cwd in RPROMPT
// user@host (branch)✗ %    RPROMPT: ~/dir
const WEZM_PLUS: TemplateDef = TemplateDef {
    name: "wezm+",
    segments: &[
        field_bold(User, Yellow),
        lit("@", Yellow),
        field(ShortHostname, Yellow),
        text(" "),
        if_git(&[
            lit("(", Blue),
            field(GitBranch, Blue),
            dirty(")", ")✗", Blue, Red),
            text(" "),
        ]),
        lit("%", Yellow),
    ],
    rprompt: &[
        field(Cwd, Green),
    ],
};

// Source: wuffers.zsh-theme — git comes first, then basename
// [branch x] basename
const WUFFERS: TemplateDef = TemplateDef {
    name: "wuffers",
    segments: &[
        if_git(&[
            lit_bold("[", Blue),
            field_bold(GitBranch, Blue),
            dirty("]", " x]", Blue, Red),
            text(" "),
        ]),
        field(CwdBasename, Cyan),
    ],
    rprompt: &[],
};

// Source: xiong-chiamiov.zsh-theme — multiline box-drawing
// ┌─[user@host] - [~/dir] └─[$]>
const XIONG_CHIAMIOV: TemplateDef = TemplateDef {
    name: "xiong-chiamiov",
    segments: &[
        lit("┌─[", Blue),
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Cyan),
        lit("]", Blue),
        text(" - "),
        lit("[", Blue),
        field(Cwd, White),
        lit("]", Blue),
        text(" "),
        lit("└─[$]>", Blue),
    ],
    rprompt: &[],
};

// Source: xiong-chiamiov-plus.zsh-theme — multiline box with git
// ┌─[user@host] - [~/dir] └─[$] <branch>
const XIONG_CHIAMIOV_PLUS: TemplateDef = TemplateDef {
    name: "xiong-chiamiov-plus",
    segments: &[
        lit("┌─[", Blue),
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Cyan),
        lit("]", Blue),
        text(" - "),
        lit("[", Blue),
        field(Cwd, White),
        lit("]", Blue),
        text(" "),
        lit("└─[$]", Blue),
        if_git(&[
            text(" <"),
            field(GitBranch, White),
            text(">"),
        ]),
    ],
    rprompt: &[],
};

// Source: zhann.zsh-theme — uses vcs_info, %c (basename)
// dir [branch●] %
const ZHANN: TemplateDef = TemplateDef {
    name: "zhann",
    segments: &[
        field_bold(CwdBasename, Blue),
        if_git(GIT_VCS_INFO),
        text(" "),
        lit_bold("%", Magenta),
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
