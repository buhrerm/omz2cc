// ---------------------------------------------------------------------------
// Theme definitions — each theme is a static TemplateDef
//
// To add a new theme: define a const TemplateDef and add it to TEMPLATES.
// ---------------------------------------------------------------------------

use crate::color::Color::*;
use crate::themes::template::*;
use FieldName::*;

// ===== Original 10 themes (migrated from individual .rs files) =============

// # mike @ zulu in ~/Workspace/omz2cc on git:main o [15:35:04]
const YS: TemplateDef = TemplateDef {
    name: "ys",
    segments: &[
        lit("#", Blue),
        text(" "),
        field_bold(User, Cyan),
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
        field_fmt(Time, Gray, "[", "]"),
    ],
};

// Source PROMPT: %(?:green_➜:red_➜) %c $(git_prompt_info)
// GIT_PROMPT_PREFIX="%{$fg_bold[blue]%}git:(%{$fg[red]%}"
// GIT_PROMPT_SUFFIX="%{$reset_color%} "
// GIT_PROMPT_DIRTY="%{$fg[blue]%}) %{$fg[yellow]%}✗"
// GIT_PROMPT_CLEAN="%{$fg[blue]%})"
const ROBBYRUSSELL: TemplateDef = TemplateDef {
    name: "robbyrussell",
    segments: &[
        lit_bold("➜", Green),
        text(" "),
        field(CwdBasename, Cyan),
        if_git(&[
            text(" "),
            lit_bold("git:(", Blue),
            field(GitBranch, Red),
            dirty(")", ") ✗", Blue, Yellow),
        ]),
    ],
};

// mike@zulu ~/Workspace/omz2cc  main ✓
const AGNOSTER: TemplateDef = TemplateDef {
    name: "agnoster",
    segments: &[
        field(UserAtHost, White),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text("  "),
            field(GitBranch, White),
            dirty(" ✓", " ✗", Green, Yellow),
        ]),
    ],
};

// ~/Workspace/omz2cc on git:main ✓
const AF_MAGIC: TemplateDef = TemplateDef {
    name: "af-magic",
    segments: &[
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("on", White),
            text(" "),
            field_fmt(GitBranch, Magenta, "git:", ""),
            dirty(" ✓", " ✗", Green, Yellow),
        ]),
    ],
};

// mike@zulu:~/Workspace/omz2cc on git:main ✓
const BIRA: TemplateDef = TemplateDef {
    name: "bira",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(HostColonCwd, Green),
        if_git(&[
            text(" "),
            lit("on", White),
            text(" "),
            field_fmt(GitBranch, Magenta, "git:", ""),
            dirty(" ✓", " ✗", Green, Yellow),
        ]),
    ],
};

// mike ~/Workspace/omz2cc [15:35:04] git:main ✓
// Theme: bureau
// Source: reference-themes/bureau.zsh-theme
// PROMPT (precmd): user@host path [time], then "> $ "
// RPROMPT: git [±branch status]
// GIT_PREFIX: "[±" (bold green ±, bold white branch)
// GIT_SUFFIX: "]"
// GIT_CLEAN: bold green "✓"
// GIT_DIRTY: (staged/unstaged dots, simplified to ✗)
const BUREAU: TemplateDef = TemplateDef {
    name: "bureau",
    segments: &[
        field_bold(User, White),
        lit("@", White),
        field(ShortHostname, White),
        text(" "),
        field_bold(Cwd, White),
        text(" "),
        field_fmt(Time, White, "[", "]"),
        if_git(&[
            text(" "),
            lit("[", White),
            lit_bold("±", Green),
            field_bold(GitBranch, White),
            text(" "),
            dirty_bold("✓", "✗", Green, Red),
            lit("]", White),
        ]),
        text(" "),
        lit(">", White),
        text(" "),
        lit("$", Green),
    ],
};

// Theme: candy
// Source: reference-themes/candy.zsh-theme
// PROMPT: %n@%m %D{[%X]} [%~] $(git_prompt_info) -> %#
// GIT_PREFIX: "[" (green)
// GIT_SUFFIX: "]"
// GIT_DIRTY: " *" (red *, green bracket)
// GIT_CLEAN: ""
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
        if_git(&[
            text(" "),
            lit("[", Green),
            field(GitBranch, Green),
            dirty("", " *", Green, Red),
            lit("]", Green),
        ]),
        text(" "),
        lit("->", Blue),
    ],
};

// Theme: dallas
// Source: reference-themes/dallas.zsh-theme
// PROMPT: {%D %T} %m:%~@branch(dirty) %n%%
// GIT_PREFIX: "@" (white) + branch (blue)
// GIT_SUFFIX: ""
// GIT_CLEAN: ""
// GIT_DIRTY: "✗✗✗" (cyan)
const DALLAS: TemplateDef = TemplateDef {
    name: "dallas",
    segments: &[
        lit("{", White),
        field(Time, Yellow),
        lit("}", White),
        text(" "),
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
        lit("%", White),
    ],
};

// Theme: gallois
// Source: reference-themes/gallois.zsh-theme
// Complex precmd: git_custom_status prepended, [%~] $
// Simplified: [branch][~/dir] $
const GALLOIS: TemplateDef = TemplateDef {
    name: "gallois",
    segments: &[
        if_git(&[
            lit("[", Cyan),
            field(GitBranch, Green),
            lit("]", Cyan),
        ]),
        lit("[", Cyan),
        field(Cwd, Cyan),
        lit("]", Cyan),
        lit_bold("$", Green),
    ],
};

// mike@zulu ~/Workspace/omz2cc git:(main)
const MARAN: TemplateDef = TemplateDef {
    name: "maran",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(Hostname, Blue),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit("git:(", Blue),
            field(GitBranch, Red),
            lit(")", Blue),
        ]),
    ],
};

// ===== Additional themes ===================================================

const AMUSE: TemplateDef = TemplateDef {
    name: "amuse",
    segments: &[
        field_bold(Cwd, Green),
        if_git(&[
            text(" on "),
            lit("\u{e0a0} ", Magenta),
            field(GitBranch, Magenta),
            dirty("", "!", Red, Red),
        ]),
        text(" \u{231a} "),
        field_bold(Time, Red),
    ],
};

const ARROW: TemplateDef = TemplateDef {
    name: "arrow",
    segments: &[
        field(CwdBasename, Yellow),
        text(" "),
        lit("\u{27a4}", Yellow),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Yellow, "git:", ""),
            dirty("", "*", Yellow, Yellow),
        ]),
    ],
};

// Theme: avit
// Source: reference-themes/avit.zsh-theme
// PROMPT (multiline): %3~ branch ✗/✔ (user@host only on SSH, omitted)
// GIT_PREFIX="" GIT_SUFFIX="" GIT_DIRTY=" ✗" (red) GIT_CLEAN=" ✔" (green)
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
};

// Theme: blinks
// Source: reference-themes/blinks.zsh-theme
// PROMPT: %n@%m %~ [branch *] %#
// GIT_PREFIX=" [" (bold blue branch) GIT_SUFFIX="]" (bold green) GIT_DIRTY=" *" (red) GIT_CLEAN=""
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
            lit("[", Green),
            field_bold(GitBranch, Blue),
            dirty("", " *", Blue, Red),
            lit_bold("]", Green),
        ]),
    ],
};

// Theme: clean
// Source: reference-themes/clean.zsh-theme
// PROMPT: %n:%c/ $(git_prompt_info)$
// RPROMPT: [%*]
// GIT_PREFIX: bold blue "(" + bold yellow branch
// GIT_SUFFIX: bold blue ")" + space
// GIT_CLEAN: ""
// GIT_DIRTY: bold red "✗"
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
        text(" "),
        lit("$", White),
        text(" "),
        field_fmt(Time, White, "[", "]"),
    ],
};

const CLOUD: TemplateDef = TemplateDef {
    name: "cloud",
    segments: &[
        lit_bold("\u{2601}", Cyan),
        text(" "),
        field(CwdBasename, Green),
        text(" "),
        if_git(&[
            lit("[", Green),
            field(GitBranch, Cyan),
            dirty("]", "] \u{26a1} ", Green, Yellow),
        ]),
    ],
};

// Theme: crunch
// Source: reference-themes/crunch.zsh-theme
// PROMPT: {%T} %~:branch ✓/✗ ➭ (no user@host)
// GIT_PREFIX=":" (white) GIT_SUFFIX="" GIT_DIRTY=" ✗" (red) GIT_CLEAN=" ✓" (green)
const CRUNCH: TemplateDef = TemplateDef {
    name: "crunch",
    segments: &[
        lit("{", White),
        field(Time, Yellow),
        lit("}", White),
        field(Cwd, Cyan),
        if_git(&[
            lit(":", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Red),
        ]),
        text(" "),
        lit("➭", White),
    ],
};

const CYPHER: TemplateDef = TemplateDef {
    name: "cypher",
    segments: &[
        field(ShortHostname, White),
        text(" "),
        lit_bold("::", Red),
        text(" "),
        field(CwdTruncated(3), Green),
        text(" "),
        lit("\u{00bb}", Blue),
    ],
};

// Theme: dieter
// Source: reference-themes/dieter.zsh-theme
// PROMPT: %* user@host %c branch?
// GIT_PREFIX="" (yellow branch) GIT_DIRTY="?" (yellow) GIT_CLEAN="" (green)
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
        text(" "),
        if_git(&[
            field(GitBranch, Yellow),
            dirty("", "?", Green, Yellow),
        ]),
    ],
};

// Theme: dpoggi
// Source: reference-themes/dpoggi.zsh-theme
// PROMPT: %n@%m:%~ (branch○/⚡) »
// GIT_PREFIX="(" (yellow) GIT_SUFFIX=")" (yellow) GIT_DIRTY="⚡" (red) GIT_CLEAN="○" (green)
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
};

// Theme: dst
// Source: reference-themes/dst.zsh-theme
// PROMPT: %n@%m: %~ branch! [%*]
// GIT_PREFIX=" " (green) GIT_DIRTY="!" (red) GIT_CLEAN=""
// RPROMPT: [%*] (green)
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
            dirty("", "!", Green, Red),
        ]),
        text(" "),
        field_fmt(Time, Green, "[", "]"),
    ],
};

// Theme: dstufft
// Source: reference-themes/dstufft.zsh-theme
// PROMPT: %n at %m in %~ on branch! ±/○
// GIT_PREFIX=" on " GIT_DIRTY="!" (green) GIT_CLEAN=""
// prompt_char: git=± else=○
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
        text(" "),
        if_git(&[
            lit("±", White),
        ]),
        if_not_git(&[
            lit("○", White),
        ]),
    ],
};

// $(git_custom_status)[~/dir]$
const EASTWOOD: TemplateDef = TemplateDef {
    name: "eastwood",
    segments: &[
        if_git(&[
            dirty("", "*", Green, Red),
            field_fmt(GitBranch, Green, "[", "]"),
        ]),
        field_fmt(Cwd, Cyan, "[", "]"),
        lit_bold("$", White),
    ],
};

// PROMPT: %n@%M:%~$(git_prompt_info)$
// GIT_PREFIX="%{$fg[cyan]%}("
// GIT_SUFFIX=") %{$reset_color%}"
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
            field_fmt(GitBranch, Cyan, "(", ")"),
            text(" "),
        ]),
        lit("$", White),
    ],
};

// Theme: evan
// Source: reference-themes/evan.zsh-theme
// PROMPT: %m :: %2~ » (no git, no user)
const EVAN: TemplateDef = TemplateDef {
    name: "evan",
    segments: &[
        field(ShortHostname, White),
        text(" :: "),
        field(CwdTruncated(2), White),
        text(" "),
        lit_bold("»", White),
    ],
};

// Theme: fino
// Source: reference-themes/fino.zsh-theme
// PROMPT: ╭─ user at host in ~/dir on branch✔/✘✘✘ ±/○
// Colors: user=FG[040], at=FG[239], host=FG[033], in=FG[239], cwd=bold FG[226]
// GIT: on=FG[239], branch=FG[255], dirty=FG[202]"✘✘✘", clean=FG[040]"✔"
const FINO: TemplateDef = TemplateDef {
    name: "fino",
    segments: &[
        lit("╭─", White),
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
        if_git(&[
            lit("±", White),
        ]),
        if_not_git(&[
            lit("○", White),
        ]),
    ],
};

// Theme: fino-time
// Source: reference-themes/fino-time.zsh-theme
// PROMPT: ╭─ user at host in ~/dir on branch✔/✘✘✘ %D - %* ⠠⠵/○
// Same colors as fino + time at end
const FINO_TIME: TemplateDef = TemplateDef {
    name: "fino-time",
    segments: &[
        lit("╭─", White),
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
};

// Theme: flazz
// Source: reference-themes/flazz.zsh-theme
// PROMPT: %m :: %3~ ‹branch› %#
// GIT_PREFIX="‹" (bold cyan) GIT_SUFFIX="› " GIT_DIRTY="" GIT_CLEAN=""
const FLAZZ: TemplateDef = TemplateDef {
    name: "flazz",
    segments: &[
        field(ShortHostname, White),
        lit_bold(" :: ", Magenta),
        field(CwdTruncated(3), Green),
        if_git(&[
            text(" "),
            lit_bold("‹", Cyan),
            field(GitBranch, Cyan),
            lit_bold("›", Cyan),
        ]),
    ],
};

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
};

// Theme: frontcube
// Source: reference-themes/frontcube.zsh-theme
// PROMPT (multiline): %~ [git:branch] ✔/✖ ➞
// GIT_PREFIX=bold blue "[git:" GIT_DIRTY=blue "] " + red "✖ " GIT_CLEAN=blue "] " + green "✔"
const FRONTCUBE: TemplateDef = TemplateDef {
    name: "frontcube",
    segments: &[
        field_bold(Cwd, Gray),
        if_git(&[
            text(" "),
            lit_bold("[git:", Blue),
            field(GitBranch, Blue),
            dirty("] ✔", "] ✖", Green, Red),
        ]),
        text(" "),
        lit("➞", Green),
    ],
};

// Theme: gallifrey
// Source: reference-themes/gallifrey.zsh-theme
// PROMPT: %m %2~ ‹branch› »
// GIT_PREFIX=yellow "‹" GIT_SUFFIX="› "
const GALLIFREY: TemplateDef = TemplateDef {
    name: "gallifrey",
    segments: &[
        field(ShortHostname, Green),
        text(" "),
        field(CwdTruncated(2), White),
        if_git(&[
            text(" "),
            lit("‹", Yellow),
            field(GitBranch, Yellow),
            lit("›", Yellow),
        ]),
        text(" "),
        lit_bold("»", White),
    ],
};

// Theme: gentoo
// Source: reference-themes/gentoo.zsh-theme
// PROMPT: bold_green user@host bold_blue %~ (branch) $
// Uses vcs_info: (green_branch) in magenta parens
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
            lit(")", Magenta),
        ]),
        text(" "),
        lit_bold("$", Blue),
    ],
};

// [user@host dir git:(branch)✗]
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
            lit("git:(", Green),
            field(GitBranch, Green),
            lit(")", Green),
            dirty("", "✗", Green, Yellow),
        ]),
        lit("]", White),
    ],
};

// Theme: gnzh
// Source: reference-themes/gnzh.zsh-theme
// PROMPT: ╭─ user@host bold_blue %~ ‹branch›
// GIT_PREFIX=yellow "‹" GIT_SUFFIX="› "
const GNZH: TemplateDef = TemplateDef {
    name: "gnzh",
    segments: &[
        lit("╭─", White),
        field(User, Green),
        lit("@", Cyan),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("‹", Yellow),
            field(GitBranch, Yellow),
            lit("›", Yellow),
        ]),
    ],
};

// Theme: half-life (actually steeef-derived)
// Source: reference-themes/half-life.zsh-theme
// PROMPT: purple_%n in limegreen_%~ on turquoise_branch orange_● λ
// Colors: purple=F{135}, limegreen=F{118}, turquoise=F{81}, orange=F{166}, hotpink=F{161}
const HALF_LIFE: TemplateDef = TemplateDef {
    name: "half-life",
    segments: &[
        field(User, Color256(135)),
        text(" in "),
        field(Cwd, Color256(118)),
        if_git(&[
            text(" on "),
            field(GitBranch, Color256(81)),
            dirty("", " ●", Color256(81), Color256(166)),
        ]),
        text(" "),
        lit("λ", Color256(166)),
    ],
};

// user@host ~/dir
// PROMPT='${user}${host} ${pwd}\n${smiley}  '
// user="%{$fg[cyan]%}%n", host="%{$fg[cyan]%}@%m", pwd="%{$fg[yellow]%}%~"
// RPROMPT='$(ruby_prompt_info) %{$fg[white]%}$(git_prompt_info)'
// ZSH_THEME_GIT_PROMPT_PREFIX="" ZSH_THEME_GIT_PROMPT_SUFFIX=""
// ZSH_THEME_GIT_PROMPT_DIRTY="%{$fg[red]%} ✗" ZSH_THEME_GIT_PROMPT_CLEAN="%{$fg[green]%} ✔"
const ITCHY: TemplateDef = TemplateDef {
    name: "itchy",
    segments: &[
        field(User, Cyan),
        lit("@", Cyan),
        field(ShortHostname, Cyan),
        text(" "),
        field(Cwd, Yellow),
        if_git(&[
            text(" "),
            field(GitBranch, White),
            dirty(" \u{2714}", " \u{2717}", Green, Red),
        ]),
    ],
};

// PROMPT (multiline, box-drawing, condensed):
// ┌─(~/dir)──(user@host:tty)─┐
// └─(HH:MM:SS git_prompt_info)──> '
// ZSH_THEME_GIT_PROMPT_PREFIX=" on %{$fg[green]%}"
// ZSH_THEME_GIT_PROMPT_SUFFIX="" ZSH_THEME_GIT_PROMPT_DIRTY="" ZSH_THEME_GIT_PROMPT_CLEAN=""
const JONATHAN: TemplateDef = TemplateDef {
    name: "jonathan",
    segments: &[
        lit("\u{2500}(", Cyan),
        field(Cwd, Green),
        lit(")\u{2500}\u{2500}(", Cyan),
        field(User, Cyan),
        lit("@", Gray),
        field(ShortHostname, Green),
        lit(")", Cyan),
        text(" "),
        lit("(", Blue),
        field(Time, Yellow),
        if_git(&[
            text(" on "),
            field(GitBranch, Green),
        ]),
        lit(")\u{2500}>", Cyan),
    ],
};

// user@host ~/dir git:(branch⚡)
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
            field_fmt(GitBranch, Yellow, "git:(", ""),
            dirty(")", "⚡)", Yellow, Yellow),
        ]),
    ],
};

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
};

// ~/dir git:(branch✗) @host
const KARDAN: TemplateDef = TemplateDef {
    name: "kardan",
    segments: &[
        field(Cwd, White),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, White, "git:(", ""),
            dirty(")", "✗)", White, Yellow),
        ]),
        lit("@", White),
        field(ShortHostname, White),
    ],
};

const KENNETHREITZ: TemplateDef = TemplateDef {
    name: "kennethreitz",
    segments: &[
        field(CwdBasename, Green),
        text(" "),
        if_git(&[
            field_fmt(GitBranch, Yellow, "(", ""),
            dirty(")", "*)", Yellow, Red),
            text(" "),
        ]),
        lit("»", Red),
    ],
};

const KOLO: TemplateDef = TemplateDef {
    name: "kolo",
    segments: &[
        field_bold(CwdBasename, Magenta),
        if_git(&[
            text(" "),
            lit_bold("[", Green),
            field_bold(GitBranch, Green),
            lit_bold("]", Green),
        ]),
        text(" "),
        lit_bold("%", Magenta),
    ],
};

// [user@host:~/dir git:(branch)]
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
            text(" "),
            lit("git:(", Green),
            field(GitBranch, Green),
            lit(")", Green),
        ]),
        lit("]", White),
    ],
};

const LAMBDA: TemplateDef = TemplateDef {
    name: "lambda",
    segments: &[
        text("λ "),
        field(Cwd, White),
        text("/ "),
        if_git(&[
            field(GitBranch, Green),
        ]),
    ],
};

const LUKERANDALL: TemplateDef = TemplateDef {
    name: "lukerandall",
    segments: &[
        field_bold(User, Green),
        lit_bold("@", Green),
        field_bold(ShortHostname, Green),
        text(" "),
        field_bold(CwdTruncated(2), Blue),
        text(" "),
        if_git(&[
            field_fmt(GitBranch, Yellow, "(", ") "),
        ]),
        text("»"),
    ],
};

const MACOVSKY: TemplateDef = TemplateDef {
    name: "macovsky",
    segments: &[
        field(Cwd, Green),
        text(" "),
        if_git(&[
            field_fmt(GitBranch, Yellow, "\u{2039}", "\u{203a}"),
            text(" "),
        ]),
        text("$ "),
    ],
};

const MGUTZ: TemplateDef = TemplateDef {
    name: "mgutz",
    segments: &[
        field_bold(CwdBasename, Magenta),
        if_git(&[
            field_fmt_bold(GitBranch, Yellow, "[", ""),
            dirty("]", "*]", Yellow, Yellow),
        ]),
        text(" "),
        lit_bold("% ", Magenta),
    ],
};

// PROMPT: blue_bold(user)@hostname_bold(host):blue_bold(~dir) bold_blue(() bold_green(branch) bold_blue()) green_bold(%#)
// GIT_PREFIX="" GIT_SUFFIX="" GIT_DIRTY="" GIT_CLEAN=""
// Note: hostname color is dynamic based on hostname chars; Cyan used as representative
const MICHELEBOLOGNA: TemplateDef = TemplateDef {
    name: "michelebologna",
    segments: &[
        field_bold(User, Blue),
        lit_bold("@", Blue),
        field_bold(ShortHostname, Cyan),
        lit(":", White),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit_bold("(", Blue),
            field_bold(GitBranch, Green),
            lit_bold(")", Blue),
        ]),
        text(" "),
        lit_bold("%", Green),
    ],
};

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
};

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
};

const MINIMAL: TemplateDef = TemplateDef {
    name: "minimal",
    segments: &[
        field(CwdTruncated(2), White),
        text(" "),
        if_git(&[
            lit("[", White),
            field(GitBranch, White),
            dirty("]", "\u{25cf}]", White, Red),
            text(" "),
        ]),
        text("\u{00bb} "),
    ],
};

const MIRA: TemplateDef = TemplateDef {
    name: "mira",
    segments: &[
        lit("\u{256d}\u{2500}", White),
        field_bold(UserAtHost, Green),
        text(" "),
        field_bold(Cwd, Blue),
        text(" "),
        if_git(&[
            field_fmt(GitBranch, Yellow, "(", ")"),
            text(" "),
        ]),
        text("\n\u{2570}\u{2500}$ "),
    ],
};

const MORTALSCUMBAG: TemplateDef = TemplateDef {
    name: "mortalscumbag",
    segments: &[
        field_bold(UserAtHost, Green),
        if_git(&[
            lit(" \u{2039} ", White),
            field_bold(GitBranch, Yellow),
            lit_bold(" \u{203a}", White),
        ]),
        text(" : "),
        field(Cwd, White),
        text("\n$ "),
    ],
};

// user@host:~/dir (branch)
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
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

const NANOTECH: TemplateDef = TemplateDef {
    name: "nanotech",
    segments: &[
        field(Cwd, Color256(117)),
        if_git(&[
            lit(" (", Color256(12)),
            field(GitBranch, Color256(12)),
            dirty(" \u{2714}", " \u{2718}", Color256(118), Color256(133)),
            lit(")", Color256(12)),
        ]),
        text(" "),
        lit("\u{1D05}", Color256(77)),
        text(" "),
    ],
};

const NICOULAJ: TemplateDef = TemplateDef {
    name: "nicoulaj",
    segments: &[
        field(Cwd, Color256(71)),
        if_git(&[
            text(" "),
            field(GitBranch, Color256(242)),
        ]),
        text(" "),
        lit("\u{276f}", Color256(71)),
        text(" "),
    ],
};

const NORM: TemplateDef = TemplateDef {
    name: "norm",
    segments: &[
        lit("\u{03bb} ", Yellow),
        field(ShortHostname, Yellow),
        text(" "),
        field(CwdBasename, Green),
        text(" "),
        lit("\u{2192} ", Yellow),
        if_git(&[
            lit("\u{03bb} ", Yellow),
            lit("git ", Blue),
            field(GitBranch, Red),
            lit(" \u{2192} ", Yellow),
        ]),
    ],
};

const PEEPCODE: TemplateDef = TemplateDef {
    name: "peepcode",
    segments: &[
        field(Cwd, White),
        if_git(&[
            text(" "),
            field_bold(GitBranch, Gray),
            dirty("", " \u{2717}", Gray, Gray),
        ]),
        text("\n$ "),
    ],
};

const PHILIPS: TemplateDef = TemplateDef {
    name: "philips",
    segments: &[
        lit_bold("@", Red),
        field_bold(ShortHostname, Red),
        text(" "),
        lit_bold("\u{279c} ", Red),
        field_bold(CwdBasename, Cyan),
        text(" "),
        if_git(&[
            lit_bold("git:(", Blue),
            field(GitBranch, Red),
            dirty("", " \u{2717}", Blue, Yellow),
            lit_bold(")", Blue),
            text(" "),
        ]),
    ],
};

const PYGMALION: TemplateDef = TemplateDef {
    name: "pygmalion",
    segments: &[
        field(User, Magenta),
        lit("@", Cyan),
        field(ShortHostname, Yellow),
        lit(":", Red),
        field(CwdBasename, Cyan),
        lit("|", Red),
        if_git(&[
            field(GitBranch, Green),
            dirty("", "\u{26a1}", Green, Yellow),
        ]),
        lit("\u{21d2}", Cyan),
        text("  "),
    ],
};

const RE5ET: TemplateDef = TemplateDef {
    name: "re5et",
    segments: &[
        field_bold(User, Cyan),
        lit("@", Yellow),
        field_bold(ShortHostname, Blue),
        text(":"),
        field_bold(Cwd, Green),
        if_git(&[
            lit_bold("^", Magenta),
            field_bold(GitBranch, Yellow),
            dirty_bold(" \u{2665}", " \u{00b1}", Red, Red),
        ]),
    ],
};

const REFINED: TemplateDef = TemplateDef {
    name: "refined",
    segments: &[
        field(Cwd, Blue),
        if_git(&[
            text(" "),
            field(GitBranch, Gray),
            dirty("", "*", Gray, Gray),
        ]),
        text(" "),
        lit("\u{276f}", Magenta),
    ],
};

const RGM: TemplateDef = TemplateDef {
    name: "rgm",
    segments: &[
        field(User, White),
        text("@"),
        field(ShortHostname, White),
        text(" "),
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            field(GitBranch, Red),
        ]),
        text(" "),
        lit_bold("%", Blue),
    ],
};

const RISTO: TemplateDef = TemplateDef {
    name: "risto",
    segments: &[
        field(User, Green),
        text("@"),
        field(ShortHostname, Green),
        text(":"),
        field_bold(CwdTruncated(2), Blue),
        text(" "),
        if_git(&[
            field_fmt(GitBranch, Red, "\u{2039}", "\u{203a}"),
        ]),
        text(" "),
    ],
};

// Source PROMPT: '%{$fg[white]%}%c$(git_prompt_info)$ %'
// GIT_PROMPT_PREFIX="("  GIT_PROMPT_SUFFIX=""
// GIT_PROMPT_DIRTY="*)"  GIT_PROMPT_CLEAN=")"
const SAMMY: TemplateDef = TemplateDef {
    name: "sammy",
    segments: &[
        field(CwdBasename, White),
        if_git(&[
            lit("(", White),
            field(GitBranch, White),
            dirty(")", "*)", White, White),
        ]),
        lit("$", White),
    ],
};

// Source PROMPT: '%(!.%{$fg[red]%}.%{$fg[green]%})%~$(git_prompt_info)'
// GIT_PROMPT_PREFIX=" %{$fg_bold[blue]%}("
// GIT_PROMPT_SUFFIX="%{$fg_bold[blue]%})"
// GIT_PROMPT_DIRTY=" %{$fg[red]%}✗"
// GIT_PROMPT_CLEAN=" %{$fg[green]%}✔"
const SIMPLE: TemplateDef = TemplateDef {
    name: "simple",
    segments: &[
        field(Cwd, Green),
        if_git(&[
            lit_bold(" (", Blue),
            field(GitBranch, Green),
            dirty(" ✔", " ✗", Green, Red),
            lit_bold(")", Blue),
        ]),
    ],
};

// Source PROMPT: '%{$fg_bold[green]%}%h %{$fg[cyan]%}%2~ %{$fg_bold[blue]%}$(git_prompt_info) » '
// GIT_PROMPT_PREFIX="git:(%{$fg[red]%}"
// GIT_PROMPT_DIRTY="%{$fg[blue]%}) %{$fg[yellow]%}✗"
// GIT_PROMPT_CLEAN="%{$fg[blue]%})"
const SKARO: TemplateDef = TemplateDef {
    name: "skaro",
    segments: &[
        field(CwdTruncated(2), Cyan),
        text(" "),
        if_git(&[
            lit_bold("git:(", Blue),
            field(GitBranch, Red),
            dirty(")", ") ✗", Blue, Yellow),
            text(" "),
        ]),
        lit("»", White),
    ],
};

// Source PROMPT (multiline condensed):
// %{$fg[blue]%}%m 福 %{$fg[cyan]%}%~ |branch✓/⚡ $(prompt_char) :
// GIT_PROMPT_PREFIX="|"  GIT_PROMPT_SUFFIX="%{$reset_color%}"
// GIT_PROMPT_DIRTY="%{$fg_bold[red]%}⚡"
// GIT_PROMPT_CLEAN="%{$fg_bold[green]%}✓"
// prompt_char: git=±(green), else=◯(cyan)
const SMT: TemplateDef = TemplateDef {
    name: "smt",
    segments: &[
        field(ShortHostname, Blue),
        text(" 福 "),
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("|", White),
            field(GitBranch, White),
            dirty_bold("✓", "⚡", Green, Red),
        ]),
        text(" "),
        if_git(&[
            lit("±", Green),
        ]),
        if_not_git(&[
            lit("◯", Cyan),
        ]),
        text(" :"),
    ],
};

// Source PROMPT: '%m➜  %c $(git_prompt_info)$(git_prompt_status) ᐅ'
// MACHINE_NAME_COLOR=$FG[208], PROMPT_SUCCESS_COLOR=$FG[103]
// GIT_PROMPT_INFO=$FG[148], GIT_DIRTY_COLOR=$FG[124], GIT_CLEAN_COLOR=$FG[148]
// PROMPT_PROMPT=$FG[208]
// GIT_PROMPT_PREFIX=": "  GIT_PROMPT_SUFFIX="$FG[148] :"
// GIT_PROMPT_DIRTY=" $FG[124]✘"  GIT_PROMPT_CLEAN=" $FG[148]✔"
const SONICRADISH: TemplateDef = TemplateDef {
    name: "sonicradish",
    segments: &[
        field(ShortHostname, Color256(208)),
        lit("➜ ", Color256(208)),
        field(CwdBasename, Color256(103)),
        if_git(&[
            text(" "),
            lit(": ", Color256(148)),
            field(GitBranch, Color256(148)),
            dirty(" ✔", " ✘", Color256(148), Color256(124)),
            lit(" :", Color256(148)),
        ]),
        text(" "),
        lit("ᐅ", Color256(208)),
    ],
};

const SORIN: TemplateDef = TemplateDef {
    name: "sorin",
    segments: &[
        field(CwdBasename, Cyan),
        if_git(&[
            text(" "),
            lit("git", Blue),
            text(":"),
            field(GitBranch, Red),
        ]),
        text(" "),
        lit_bold("\u{276f}", Green),
    ],
};

// Theme: steeef
// Source: reference-themes/steeef.zsh-theme
// PROMPT: purple_%n in limegreen_%~ on turquoise_branch orange_● λ
// Uses vcs_info. Colors: purple=F{135}, limegreen=F{118}, turquoise=F{81}, orange=F{166}
const STEEEF: TemplateDef = TemplateDef {
    name: "steeef",
    segments: &[
        field(User, Color256(135)),
        text(" in "),
        field(Cwd, Color256(118)),
        if_git(&[
            text(" on "),
            field(GitBranch, Color256(81)),
            dirty("", " ●", Color256(81), Color256(166)),
        ]),
        text(" "),
        lit("λ", Color256(166)),
    ],
};

const SUNAKU: TemplateDef = TemplateDef {
    name: "sunaku",
    segments: &[
        field(Cwd, Green),
        if_git(&[
            text("("),
            field(GitBranch, White),
            dirty("", "*", White, Red),
            text(")"),
        ]),
        text(" $ "),
    ],
};

const SUVASH: TemplateDef = TemplateDef {
    name: "suvash",
    segments: &[
        field(User, Magenta),
        text(" at "),
        field(ShortHostname, Yellow),
        text(" in "),
        field_bold(Cwd, Green),
        if_git(&[
            text(" on "),
            field(GitBranch, Magenta),
            dirty("", "!", Magenta, Green),
        ]),
        text(" "),
        if_git(&[
            lit("\u{00b1}", White),
        ]),
        if_not_git(&[
            lit("\u{25cb}", White),
        ]),
    ],
};

const TAKASHIYOSHIDA: TemplateDef = TemplateDef {
    name: "takashiyoshida",
    segments: &[
        lit_bold("[", White),
        field_bold(ShortHostname, Cyan),
        text(":"),
        field_bold(CwdBasename, Yellow),
        lit_bold("]", White),
        if_git(&[
            text(" on "),
            field(GitBranch, Magenta),
            dirty("", "!", Magenta, Green),
        ]),
        text(" "),
        lit_bold("[", White),
        field_bold(User, White),
        lit_bold("]", White),
        text("% "),
    ],
};

const TERMINALPARTY: TemplateDef = TemplateDef {
    name: "terminalparty",
    segments: &[
        field(User, Magenta),
        text("@"),
        field(ShortHostname, Yellow),
        text(": "),
        field_bold(Cwd, Blue),
        if_git(&[
            field(GitBranch, Green),
            dirty("", " \u{26a1}", Green, Red),
        ]),
        text(" $ "),
        lit("[", Green),
        field(Time, Green),
        lit("]", Green),
    ],
};

const THEUNRAVELER: TemplateDef = TemplateDef {
    name: "theunraveler",
    segments: &[
        lit("[", Magenta),
        field(CwdBasename, Magenta),
        lit("]", Magenta),
        text(" "),
        if_git(&[
            field(GitBranch, Magenta),
        ]),
    ],
};

const TONOTDO: TemplateDef = TemplateDef {
    name: "tonotdo",
    segments: &[
        field(User, Cyan),
        lit("\u{279c}", Magenta),
        field(CwdTruncated(3), Green),
        if_git(&[
            lit_bold("(", Blue),
            field(GitBranch, Red),
            dirty("", "\u{2717}", Red, BrightYellow),
            lit_bold(")", Blue),
        ]),
        text("\u{00bb} "),
    ],
};

const WEDISAGREE: TemplateDef = TemplateDef {
    name: "wedisagree",
    segments: &[
        if_git(&[
            lit("(", Blue),
            field(GitBranch, Blue),
            dirty(")", ")⚡", Blue, Red),
            text(" "),
        ]),
        lit("$", Yellow),
        text(" "),
        field(Cwd, Green),
    ],
};

const WEZM: TemplateDef = TemplateDef {
    name: "wezm",
    segments: &[
        if_git(&[
            lit("(", Blue),
            field(GitBranch, Blue),
            dirty(")", ")⚡", Blue, Red),
            text(" "),
        ]),
        lit("$", Yellow),
        text(" "),
        field(Cwd, Green),
    ],
};

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
        lit("$", Yellow),
        text(" "),
        field(Cwd, Green),
    ],
};

const WUFFERS: TemplateDef = TemplateDef {
    name: "wuffers",
    segments: &[
        if_git(&[
            lit_bold("[", Blue),
            field_bold(GitBranch, Blue),
            dirty("", " x", Blue, Red),
            lit_bold("]", Blue),
            text(" "),
        ]),
        field(CwdBasename, Cyan),
    ],
};

const XIONG_CHIAMIOV: TemplateDef = TemplateDef {
    name: "xiong-chiamiov",
    segments: &[
        lit_bold("[", Blue),
        field_bold(User, Green),
        lit_bold("@", Gray),
        field(ShortHostname, Cyan),
        lit_bold("]", Blue),
        text(" - "),
        lit_bold("[", Blue),
        field_bold(Cwd, BrightWhite),
        lit_bold("]", Blue),
        text(" - "),
        field_fmt(Time, Yellow, "[", "]"),
    ],
};

const XIONG_CHIAMIOV_PLUS: TemplateDef = TemplateDef {
    name: "xiong-chiamiov-plus",
    segments: &[
        lit_bold("[", Blue),
        field_bold(User, Green),
        lit_bold("@", Gray),
        field(ShortHostname, Cyan),
        lit_bold("]", Blue),
        text(" - "),
        lit_bold("[", Blue),
        field_bold(Cwd, BrightWhite),
        lit_bold("]", Blue),
        text(" - "),
        field_fmt(Time, Yellow, "[", "]"),
        if_git(&[
            text(" <"),
            field(GitBranch, Green),
            dirty("", " ✗", Green, Red),
            text(">"),
        ]),
    ],
};

const ZHANN: TemplateDef = TemplateDef {
    name: "zhann",
    segments: &[
        field_bold(CwdBasename, Blue),
        if_git(&[
            text(" "),
            lit_bold("[", Green),
            field_bold(GitBranch, Green),
            dirty("", "●", Green, Yellow),
            lit_bold("]", Green),
        ]),
    ],
};

const DARKBLOOD: TemplateDef = TemplateDef {
    name: "darkblood",
    segments: &[
        lit("┌[", Red),
        field_bold(User, White),
        lit("@", Red),
        field_bold(ShortHostname, White),
        lit("]", Red),
        text(" "),
        if_git(&[
            lit("[", Red),
            field_bold(GitBranch, White),
            dirty("", " \u{26A1}", White, Red),
            lit("]", Red),
            text(" "),
        ]),
        lit("└[", Red),
        field_bold(Cwd, White),
        lit("]>", Red),
    ],
};

const EDVARDM: TemplateDef = TemplateDef {
    name: "edvardm",
    segments: &[
        lit_bold("➜", Red),
        lit_bold(" ", Green),
        field_bold(CwdBasename, White),
        text(" "),
        if_git(&[
            lit_bold("git:(", Blue),
            field(GitBranch, Red),
            dirty(")", ") \u{2717}", Blue, Yellow),
        ]),
    ],
};

const FOX: TemplateDef = TemplateDef {
    name: "fox",
    segments: &[
        lit("┌[", Cyan),
        field_bold(User, White),
        lit("☮", Cyan),
        field_bold(Hostname, White),
        lit("]", Cyan),
        lit("-", White),
        lit("(", Cyan),
        field_bold(Cwd, White),
        lit(")", Cyan),
        if_git(&[
            lit("-[", Cyan),
            lit("git://", White),
            field_bold(GitBranch, White),
            dirty(" \u{2714}", " \u{2717}", Green, Red),
            lit("]", Cyan),
            lit("-", Cyan),
        ]),
        text(" "),
        lit("└>", Cyan),
    ],
};

const GEOFFGARSIDE: TemplateDef = TemplateDef {
    name: "geoffgarside",
    segments: &[
        lit_bold("➜", Red),
        lit_bold(" ", Green),
        field(CwdBasename, Cyan),
        text(" "),
        if_git(&[
            lit_bold("(", Blue),
            field(GitBranch, Blue),
            lit_bold(")", Blue),
            text(" "),
        ]),
        text("$ "),
    ],
};

const GOZILLA: TemplateDef = TemplateDef {
    name: "gozilla",
    segments: &[
        field(CwdBasename, Cyan),
        if_git(&[
            lit_bold("(", Blue),
            field_bold(GitBranch, Blue),
            dirty(")", ") \u{2717}", Blue, Red),
        ]),
        text(": "),
    ],
};

// user@host ~/dir ❯ branch
// PROMPT='%n %{$fg[green]%}{%~%{$fg[green]%}}%{$reset_color%}$(git_prompt_info) greetings, earthling %{$fg[red]%}$ ☞ '
// ZSH_THEME_GIT_PROMPT_PREFIX="%{$fg[red]%}±("
// ZSH_THEME_GIT_PROMPT_SUFFIX=");%{$reset_color%}"
// ZSH_THEME_GIT_PROMPT_DIRTY="" ZSH_THEME_GIT_PROMPT_CLEAN=""
const HUMZA: TemplateDef = TemplateDef {
    name: "humza",
    segments: &[
        field(User, White),
        text(" "),
        lit("{", Green),
        field(Cwd, White),
        lit("}", Green),
        if_git(&[
            text(" "),
            lit("±(", Red),
            field(GitBranch, Red),
            lit(");", Red),
        ]),
        text(" greetings, earthling "),
        lit("$ \u{261E}", Red),
    ],
};

// PROMPT="%{$fg[red]%}%%%{$reset_color%} "
// (no git integration)
const IMAJES: TemplateDef = TemplateDef {
    name: "imajes",
    segments: &[
        lit("%", Red),
    ],
};

// PROMPT='%{$fg_bold[red]%}➜ %{$fg_bold[green]%} %{$fg[cyan]%}%c %{$fg_bold[white]%}$(git_prompt_info) % '
// ZSH_THEME_GIT_PROMPT_PREFIX="git:(%{$fg[red]%}"
// ZSH_THEME_GIT_PROMPT_SUFFIX="%{$reset_color%}"
// ZSH_THEME_GIT_PROMPT_DIRTY="%{$fg[white]%}) %{$fg[yellow]%}✗"
// ZSH_THEME_GIT_PROMPT_CLEAN="%{$fg[white]%})"
const JBERGANTINE: TemplateDef = TemplateDef {
    name: "jbergantine",
    segments: &[
        lit_bold("\u{279C}", Red),
        text(" "),
        field(CwdBasename, Cyan),
        text(" "),
        if_git(&[
            lit_bold("git:(", White),
            field(GitBranch, Red),
            dirty(")", ") \u{2717}", White, Yellow),
            text(" "),
        ]),
    ],
};

// PROMPT='%{$fg[green]%}%n@%m: %{$fg[blue]%}%/ %{$fg_bold[blue]%}$(git_prompt_info) %
// ${ret_status} '
// ZSH_THEME_GIT_PROMPT_PREFIX="git:(%{$fg[red]%}"
// ZSH_THEME_GIT_PROMPT_SUFFIX="%{$reset_color%}"
// ZSH_THEME_GIT_PROMPT_DIRTY="%{$fg[blue]%}) %{$fg[yellow]%}✗"
// ZSH_THEME_GIT_PROMPT_CLEAN="%{$fg[blue]%})"
const JISPWOSO: TemplateDef = TemplateDef {
    name: "jispwoso",
    segments: &[
        field(User, Green),
        lit("@", Green),
        field(ShortHostname, Green),
        lit(": ", Green),
        field(Cwd, Blue),
        text(" "),
        if_git(&[
            lit_bold("git:(", Blue),
            field(GitBranch, Red),
            dirty(")", ") \u{2717}", Blue, Yellow),
            text(" "),
        ]),
        lit_bold("\u{279C}", Green),
    ],
};

// PROMPT='${ret_status}%{$fg[blue]%}${PROMPT_HOST}%{$fg_bold[green]%} %{$fg_bold[yellow]%}%2~ ${vcs_info_msg_0_}${dir_status} '
// Uses vcs_info, not git_prompt_info. Simplified: Ξ host %2~ git:(branch) ▶
// dir_status: green ▶ (clean), yellow ▶ (dirty), green → (no git)
const JNROWE: TemplateDef = TemplateDef {
    name: "jnrowe",
    segments: &[
        lit_bold("\u{039E}", Green),
        text(" "),
        field_bold(CwdTruncated(2), Yellow),
        if_git(&[
            text(" "),
            lit("git:", Green),
            lit("(", Green),
            field(GitBranch, Red),
            lit(")", Green),
            text(" "),
            dirty("\u{25B6}", "\u{25B6}", Green, Yellow),
        ]),
        if_not_git(&[
            text(" "),
            lit("\u{2192}", Green),
        ]),
    ],
};

const JOSH: TemplateDef = TemplateDef {
    name: "josh",
    segments: &[
        field(User, White),
        text("@"),
        field(ShortHostname, White),
        text(" "),
        field(Cwd, Green),
        if_git(&[
            text(" "),
            lit("(", Gray),
            field(GitBranch, Gray),
            dirty(")", ") ✗", Gray, Yellow),
        ]),
        text(" "),
        lit("⚡", Green),
    ],
};

const JUANGHURTADO: TemplateDef = TemplateDef {
    name: "juanghurtado",
    segments: &[
        field_bold(User, Green),
        lit_bold("@", Green),
        field_bold(ShortHostname, Green),
        lit(":", White),
        field(Cwd, Yellow),
        if_git(&[
            dirty("", " (*)", White, Red),
            text(" "),
            field_bold(GitBranch, Green),
        ]),
        text(" "),
        lit(">", Blue),
    ],
};

const JUNKFOOD: TemplateDef = TemplateDef {
    name: "junkfood",
    segments: &[
        lit_bold("#", Red),
        lit_bold("( ", White),
        field_bold(Time, Yellow),
        text(" "),
        lit_bold(")( ", White),
        field_bold(User, Green),
        text("@"),
        field_bold(ShortHostname, Blue),
        lit(" ):", White),
        field(Cwd, Cyan),
        if_git(&[
            lit("@", White),
            field_bold(GitBranch, White),
            dirty_bold("✔", "✗✗✗", Green, Red),
        ]),
    ],
};

const KAFEITU: TemplateDef = TemplateDef {
    name: "kafeitu",
    segments: &[
        lit_bold("➜", Red),
        text(" "),
        field_bold(User, Green),
        lit("@", Cyan),
        field_bold(ShortHostname, Green),
        text("  "),
        field(Cwd, Cyan),
        text(" "),
        if_git(&[
            lit_bold("git:(", Blue),
            field(GitBranch, Red),
            dirty(")", ") ✗", Blue, Yellow),
        ]),
    ],
};

const KIWI: TemplateDef = TemplateDef {
    name: "kiwi",
    segments: &[
        lit_bold("┌[", Green),
        lit_bold("kiwish-4.2", Cyan),
        lit_bold("]-(", Green),
        field_bold(CwdTruncated(2), White),
        lit_bold(")-", Green),
        if_git(&[
            lit_bold("[", Green),
            lit("git:", White),
            field_bold(GitBranch, White),
            lit_bold("]-", Green),
        ]),
        text(" "),
        lit_bold("└>", Green),
    ],
};

const LINUXONLY: TemplateDef = TemplateDef {
    name: "linuxonly",
    segments: &[
        field(User, Color256(215)),
        lit("@", Color256(209)),
        field(ShortHostname, Color256(209)),
        text(":"),
        field(Cwd, Color256(203)),
        if_git(&[
            text(" "),
            lit("git", Color256(126)),
            lit(":", Color256(149)),
            lit("(", Color256(149)),
            field(GitBranch, Color256(162)),
            lit(")", Color256(149)),
        ]),
        text(" "),
        text(">"),
    ],
};

const MACOVSKY_RUBY: TemplateDef = TemplateDef {
    name: "macovsky-ruby",
    segments: &[
        field(Cwd, Green),
        text(" "),
        if_git(&[
            field_fmt(GitBranch, Yellow, "\u{2039}", "\u{203a}"),
            text(" "),
        ]),
        text("$ "),
    ],
};

const MH: TemplateDef = TemplateDef {
    name: "mh",
    segments: &[
        lit("[", White),
        field_bold(User, White),
        lit(":", White),
        field(Cwd, Red),
        lit("]", White),
        text("$ "),
        if_git(&[
            lit_bold("(", Gray),
            field_bold(GitBranch, Yellow),
            dirty("", "\u{2731}", Yellow, Red),
            lit_bold(")", Gray),
        ]),
    ],
};

const MURILASSO: TemplateDef = TemplateDef {
    name: "murilasso",
    segments: &[
        field_bold(UserAtHost, Green),
        lit(":", White),
        field_bold(Cwd, Blue),
        text(" "),
        if_git(&[
            field(GitBranch, Blue),
            dirty(" \u{2714}", " \u{2717}", Green, Red),
            text(" "),
        ]),
        text("$ "),
    ],
};

const NEBIRHOS: TemplateDef = TemplateDef {
    name: "nebirhos",
    segments: &[
        field(CwdTruncated(2), Green),
        lit(" [", Blue),
        if_git(&[
            field(GitBranch, Yellow),
            dirty("", " *", Yellow, Red),
            text(" "),
        ]),
        lit("]", Blue),
        text(" "),
        field(Time, Green),
    ],
};

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
        lit("\u{279c} ", Magenta),
        field(CwdTruncated(3), Green),
        text(" "),
        if_git(&[
            field_fmt(GitBranch, Red, "\u{2039}", "\u{203a}"),
            text(" "),
        ]),
        lit_bold("\u{00bb}", Blue),
        text(" "),
    ],
};

const PMCGEE: TemplateDef = TemplateDef {
    name: "pmcgee",
    segments: &[
        field(User, Magenta),
        lit("@", Cyan),
        field(ShortHostname, Yellow),
        lit(":", Red),
        field(CwdBasename, Cyan),
        lit("|", Red),
        if_git(&[
            field(GitBranch, Green),
            dirty("", "\u{26a1}", Green, Yellow),
        ]),
        lit("\u{21d2}", Cyan),
        text("  "),
    ],
};

const RIXIUS: TemplateDef = TemplateDef {
    name: "rixius",
    segments: &[
        field(User, Red),
        text(" in "),
        field_bold(Cwd, Green),
        if_git(&[
            text(" on "),
            field(GitBranch, Magenta),
            dirty(" \u{221a}", " !", Red, Red),
        ]),
        text(" "),
        if_git(&[
            lit("\u{00b1}", Red),
        ]),
        if_not_git(&[
            lit("\u{2265}", Red),
        ]),
        text(" "),
        field(Time, Red),
    ],
};

const RKJ: TemplateDef = TemplateDef {
    name: "rkj",
    segments: &[
        lit("\u{250c}\u{2500}[", Blue),
        field_bold(User, Green),
        lit("@", Gray),
        field(ShortHostname, Cyan),
        lit("]", Blue),
        text(" - "),
        lit("[", Blue),
        field_bold(Cwd, White),
        lit("]", Blue),
        text(" - "),
        lit("[", Blue),
        field(Time, Yellow),
        lit("]", Blue),
    ],
};

const SIMONOFF: TemplateDef = TemplateDef {
    name: "simonoff",
    segments: &[
        lit_bold("-<", Red),
        field_bold(User, Blue),
        lit_bold("@", Green),
        field_bold(Hostname, Blue),
        lit_bold(":", Blue),
        field_bold(Cwd, Green),
        if_git(&[
            field_fmt(GitBranch, Cyan, " [", "]"),
        ]),
        lit_bold(">-", Red),
    ],
};

const SPORTY_256: TemplateDef = TemplateDef {
    name: "sporty_256",
    segments: &[
        if_git(&[
            lit("\u{00b1}|", Color256(154)),
            field(GitBranch, Color256(124)),
            dirty(" \u{2714}", " \u{2718}", Green, Red),
            lit("|", Color256(154)),
            text(" "),
        ]),
        field(CwdBasename, Color256(208)),
        text(" "),
        field_bold(User, Color256(208)),
        lit("@", BrightWhite),
        field(ShortHostname, Color256(39)),
    ],
};

const SUNRISE: TemplateDef = TemplateDef {
    name: "sunrise",
    segments: &[
        text("--- "),
        field(CwdTruncated(2), White),
        if_git(&[
            text(" "),
            lit("\u{2039}", Yellow),
            field(GitBranch, Yellow),
            dirty("", "*", Yellow, Red),
            lit("\u{203a}", Yellow),
        ]),
        text(" "),
        lit_bold("\u{00bb}", Magenta),
    ],
};

const SUPERJARIN: TemplateDef = TemplateDef {
    name: "superjarin",
    segments: &[
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("<", White),
            field(GitBranch, Magenta),
            dirty(">", "> \u{2717}", White, Yellow),
        ]),
    ],
};

const TJKIRCH: TemplateDef = TemplateDef {
    name: "tjkirch",
    segments: &[
        field(CwdTruncated(2), White),
        if_git(&[
            text(" "),
            lit("(", Yellow),
            field(GitBranch, Yellow),
            dirty("", " \u{26a1}", Yellow, Red),
            lit(")", Yellow),
        ]),
        text(" "),
        field_bold(ShortHostname, Blue),
    ],
};

const TRAPD00R: TemplateDef = TemplateDef {
    name: "trapd00r",
    segments: &[
        if_git(&[
            lit("❨", Color256(242)),
            text(" "),
            field_bold(GitBranch, Color256(208)),
            text(" "),
            lit("❩", Color256(242)),
            dirty("", " DIRTY", Color256(242), Red),
            text(" "),
        ]),
        field(User, Color256(245)),
        lit("@", Color256(197)),
        field(ShortHostname, Color256(250)),
        lit("->", Color256(240)),
        text(" "),
        field(Cwd, Color256(65)),
    ],
};

const CRCANDY: TemplateDef = TemplateDef {
    name: "crcandy",
    segments: &[
        field_fmt(CwdBasename, Magenta, "[", "]"),
        text(" "),
        field(Time, Green),
        if_git(&[
            lit(" ☁  ", Magenta),
            field(GitBranch, Red),
            dirty(" ☀", " ☂", Green, Yellow),
        ]),
    ],
};

const DAVEVERWER: TemplateDef = TemplateDef {
    name: "daveverwer",
    segments: &[
        field(ShortHostname, Red),
        text(":"),
        field(CwdBasename, Green),
        if_git(&[
            lit(" (", Blue),
            field(GitBranch, Blue),
            lit(")", Blue),
        ]),
        text(" $ "),
    ],
};

const FLETCHERM: TemplateDef = TemplateDef {
    name: "fletcherm",
    segments: &[
        field(User, Cyan),
        lit("\u{2022}", Magenta),
        field(CwdTruncated(3), Green),
        if_git(&[
            lit_bold("(", Blue),
            field(GitBranch, Red),
            dirty(")", "\u{26A1})", Yellow, Blue),
        ]),
        text("\u{00BB} "),
    ],
};

const FUNKY: TemplateDef = TemplateDef {
    name: "funky",
    segments: &[
        text("\u{256D}\u{2500}"),
        lit("[", Blue),
        field(Cwd, White),
        lit("]", Blue),
        lit("-", White),
        lit("[", Blue),
        field(User, White),
        text("@"),
        field(ShortHostname, White),
        lit("]", Blue),
    ],
};

const GARYBLESSINGTON: TemplateDef = TemplateDef {
    name: "garyblessington",
    segments: &[
        field_fmt(Time, White, "[", "]"),
        text(" "),
        field(User, Cyan),
        text(":"),
        field(CwdBasename, Green),
        if_git(&[
            lit(" git:(", Yellow),
            field(GitBranch, Yellow),
            lit(")", Yellow),
        ]),
        text(" $ "),
    ],
};

// user@host ~/dir (branch)
// PROMPT='%{$fg_bold[grey]%}[%{$fg_bold[green]%}%n@%m%{$fg_bold[grey]%}]%{$reset_color%} %{$fg_bold[blue]%}%10c%{$reset_color%} $(git_prompt_info)
// %{$fg_bold[cyan]%}❯%{$reset_color%} '
// ZSH_THEME_GIT_PROMPT_PREFIX="%{$fg[grey]%}(%{$fg[red]%}"
// ZSH_THEME_GIT_PROMPT_SUFFIX="%{$reset_color%}"
// ZSH_THEME_GIT_PROMPT_DIRTY="%{$fg[grey]%}) %{$fg[yellow]%}⚡"
// ZSH_THEME_GIT_PROMPT_CLEAN="%{$fg[grey]%})"
const INTHELOOP: TemplateDef = TemplateDef {
    name: "intheloop",
    segments: &[
        lit_bold("[", Gray),
        field_bold(User, Green),
        lit_bold("@", Green),
        field_bold(ShortHostname, Green),
        lit_bold("]", Gray),
        text(" "),
        field_bold(CwdBasename, Blue),
        if_git(&[
            text(" "),
            lit("(", Gray),
            field(GitBranch, Red),
            dirty(")", ") \u{26A1}", Gray, Yellow),
        ]),
        text(" "),
        lit_bold("\u{276F}", Cyan),
    ],
};

// PROMPT='%{$fg_bold[magenta]%}%m at %{$fg_bold[green]%}%~ %{$fg_bold[blue]%}$(git_prompt_info)%{$fg[red]%}❯ '
// ZSH_THEME_GIT_PROMPT_PREFIX="±(%{$fg[red]%}"
// ZSH_THEME_GIT_PROMPT_SUFFIX="%{$reset_color%}"
// ZSH_THEME_GIT_PROMPT_DIRTY="%{$fg[blue]%}) %{$fg[yellow]%}✗ "
// ZSH_THEME_GIT_PROMPT_CLEAN="%{$fg[blue]%}) "
const JAISCHEEMA: TemplateDef = TemplateDef {
    name: "jaischeema",
    segments: &[
        field_bold(ShortHostname, Magenta),
        text(" at "),
        field_bold(Cwd, Green),
        text(" "),
        if_git(&[
            lit_bold("\u{00B1}(", Blue),
            field(GitBranch, Red),
            dirty(") ", ") \u{2717} ", Blue, Yellow),
        ]),
        lit("\u{276F}", Red),
    ],
};

const AUSSIEGEEK: TemplateDef = TemplateDef {
    name: "aussiegeek",
    segments: &[
        lit_bold("[", Blue),
        field(Time, Red),
        lit_bold("] [", Blue),
        field(User, Red),
        lit("@", Red),
        field(ShortHostname, Red),
        lit(":", Red),
        field(Cwd, Red),
        if_git(&[
            field_fmt_bold(GitBranch, Green, "(", ""),
            dirty("\u{2714}", "\u{2718}", Green, Green),
            lit_bold(")", Green),
        ]),
        lit_bold("]", Blue),
    ],
};

const THREEDEN: TemplateDef = TemplateDef {
    name: "3den",
    segments: &[
        field_bold(Cwd, Cyan),
        if_git(&[
            lit(" (", White),
            field(GitBranch, White),
            dirty("", "*", White, White),
            lit(")", White),
        ]),
        text(" "),
        field(Time, Cyan),
        text(" "),
        field_bold(User, Green),
        lit_bold("$", Green),
    ],
};

const SOLIAH: TemplateDef = TemplateDef {
    name: "Soliah",
    segments: &[
        field(User, Blue),
        text(" on "),
        field(Hostname, Red),
        text(" in "),
        field(Cwd, Blue),
        if_git(&[
            text("("),
            field(GitBranch, White),
            dirty("", "*", White, Red),
            text(")"),
        ]),
        text(" $ "),
    ],
};

const ADBEN: TemplateDef = TemplateDef {
    name: "adben",
    segments: &[
        lit("<", Gray),
        lit("<", Red),
        lit("<", BrightRed),
        text(" "),
        field(User, Red),
        lit("@", Red),
        field(ShortHostname, Red),
        field_bold(Cwd, BrightYellow),
        text(" "),
        lit(">", BrightGreen),
        lit(">", Green),
        lit(">", Gray),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Red, "\u{2039}git:", ""),
            dirty(" \u{2714}", " \u{2718}", Green, Yellow),
            lit("\u{203a}", Red),
        ]),
        text(" "),
        field(Time, Yellow),
    ],
};

const AFOWLER: TemplateDef = TemplateDef {
    name: "afowler",
    segments: &[
        field(ShortHostname, White),
        text(" "),
        lit_bold("::", Blue),
        text(" "),
        field(CwdTruncated(3), Green),
        text(" "),
        if_git(&[
            field_fmt(GitBranch, Yellow, "\u{2039}", "\u{203a}"),
            text(" "),
        ]),
        lit_bold("\u{00bb}", Blue),
    ],
};

const ALANPEABODY: TemplateDef = TemplateDef {
    name: "alanpeabody",
    segments: &[
        field(User, Magenta),
        lit("@", Magenta),
        field(ShortHostname, Magenta),
        text(" "),
        field(Cwd, Blue),
        text("$ "),
        if_git(&[
            field(GitBranch, Green),
        ]),
    ],
};

const APPLE: TemplateDef = TemplateDef {
    name: "apple",
    segments: &[
        text("\u{f8ff} "),
        field(Cwd, White),
        if_git(&[
            text(" "),
            lit("[", Magenta),
            field(GitBranch, Green),
            dirty("", "*", Red, Red),
            lit("]", Magenta),
        ]),
    ],
};

const AWESOMEPANDA: TemplateDef = TemplateDef {
    name: "awesomepanda",
    segments: &[
        lit_bold("\u{279c}", Green),
        text(" "),
        field(CwdBasename, Cyan),
        text(" "),
        if_git(&[
            lit_bold("git:(", Blue),
            field(GitBranch, Red),
            dirty("", " \u{2718} ", Blue, Yellow),
            lit_bold(") ", Blue),
        ]),
    ],
};

const CANDY_KINGDOM: TemplateDef = TemplateDef {
    name: "candy-kingdom",
    segments: &[
        field(User, Magenta),
        lit("@", White),
        field(ShortHostname, Yellow),
        lit(":", White),
        field_bold(Cwd, Green),
        if_git(&[
            text(" ("),
            lit("branch: ", Magenta),
            field(GitBranch, Magenta),
            dirty("", "!", Yellow, Yellow),
            text(")"),
        ]),
        text(" $ "),
    ],
};

const DOGENPUNK: TemplateDef = TemplateDef {
    name: "dogenpunk",
    segments: &[
        field(ShortHostname, Blue),
        lit_bold(" \u{0950}  ", White),
        field_fmt(Cwd, Cyan, "", ":"),
        if_git(&[
            lit_bold("git", Green),
            text("@"),
            field(GitBranch, White),
            dirty("", "!", White, Red),
            text(")"),
            text(" "),
            lit("±", Green),
        ]),
        if_not_git(&[
            lit("\u{25EF} ", Cyan),
        ]),
    ],
};

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
        text(" "),
        lit("└─[", Blue),
        lit_bold("$", Magenta),
        lit("]", Blue),
        text(" "),
        field_fmt(Time, White, "[", "]"),
    ],
};

const FISHY: TemplateDef = TemplateDef {
    name: "fishy",
    segments: &[
        field(User, Green),
        text("@"),
        field(ShortHostname, White),
        text(" "),
        field(Cwd, Green),
        if_git(&[
            text(" "),
            field(GitBranch, White),
        ]),
        text("> "),
    ],
};

const FWALCH: TemplateDef = TemplateDef {
    name: "fwalch",
    segments: &[
        lit_bold(" ", Green),
        field(CwdBasename, Cyan),
        text(" "),
        if_git(&[
            lit_bold("(", Blue),
            field(GitBranch, Red),
            dirty(")", ") \u{2717}", Blue, Red),
        ]),
        text(": "),
    ],
};

const MLH: TemplateDef = TemplateDef {
    name: "mlh",
    segments: &[
        field(User, Color256(1)),
        lit("@", White),
        field(ShortHostname, Color256(33)),
        text(" in "),
        field(CwdBasename, Color256(220)),
        if_git(&[
            text(" on "),
            field(GitBranch, Color256(1)),
        ]),
        text("\n$ "),
    ],
};

const MUSE: TemplateDef = TemplateDef {
    name: "muse",
    segments: &[
        field(Cwd, Color256(117)),
        if_git(&[
            lit(" (", Color256(12)),
            field(GitBranch, Color256(12)),
            dirty(" \u{2714}", " \u{2718}", Color256(118), Color256(133)),
            lit(")", Color256(12)),
        ]),
        text(" "),
        lit("\u{1D05}", Color256(77)),
        text(" "),
    ],
};

const OLDGALLOIS: TemplateDef = TemplateDef {
    name: "oldgallois",
    segments: &[
        field_fmt(Cwd, Cyan, "[", "]"),
        if_git(&[
            text(" "),
            dirty("", "*", Green, Red),
            field_fmt(GitBranch, Green, "[", "]"),
        ]),
    ],
};

const PYGMALION_VIRTUALENV: TemplateDef = TemplateDef {
    name: "pygmalion-virtualenv",
    segments: &[
        field(User, Magenta),
        lit("@", Cyan),
        field(ShortHostname, Yellow),
        lit(":", Red),
        field(CwdBasename, Cyan),
        lit("|", Red),
        if_git(&[
            field(GitBranch, Green),
            dirty("", "\u{26a1}", Green, Yellow),
        ]),
        lit("\u{21d2}", Cyan),
        text("  "),
    ],
};

const RKJ_REPOS: TemplateDef = TemplateDef {
    name: "rkj-repos",
    segments: &[
        lit_bold("\u{250c}\u{2500}[", Blue),
        field_bold(User, Green),
        lit("@", Black),
        field(ShortHostname, Cyan),
        lit_bold("]", Blue),
        text(" - "),
        lit_bold("[", Blue),
        field_bold(Cwd, White),
        lit_bold("]", Blue),
        text(" - "),
        lit_bold("[", Blue),
        field(Time, Yellow),
        lit_bold("]", Blue),
        if_git(&[
            text(" "),
            lit("<", Blue),
            field(GitBranch, Magenta),
            lit(">", Blue),
        ]),
    ],
};

const STRUG: TemplateDef = TemplateDef {
    name: "strug",
    segments: &[
        lit("\u{256d}\u{2500}", Green),
        field(User, Green),
        text("@"),
        field(ShortHostname, Green),
        text(" "),
        lit("in ", Yellow),
        field(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit_bold("on ", Yellow),
            field(GitBranch, Yellow),
            dirty(" \u{2714}", " \u{2718}", Green, Red),
        ]),
    ],
};

const TJKIRCH_MOD: TemplateDef = TemplateDef {
    name: "tjkirch_mod",
    segments: &[
        field(User, Magenta),
        text("@"),
        field(ShortHostname, Yellow),
        text(": "),
        field_bold(Cwd, Blue),
        if_git(&[
            field(GitBranch, Green),
            dirty("", " \u{26a1}", Green, Red),
        ]),
        text(" $ "),
        lit("[", Green),
        field(Time, Green),
        lit("]", Green),
    ],
};

const EMOTTY: TemplateDef = TemplateDef {
    name: "emotty",
    segments: &[
        text("😄 "),
        if_git(&[
            field(GitBranch, Yellow),
            dirty("", " ●", Green, Red),
            text(" "),
        ]),
        field(CwdTruncated(3), Cyan),
    ],
};

// ---------------------------------------------------------------------------
// Template registry
// ---------------------------------------------------------------------------

pub static TEMPLATES: &[&TemplateDef] = &[
    // Original 10
    &YS,
    &ROBBYRUSSELL,
    &AGNOSTER,
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
    &EMOTTY,
    &EDVARDM,
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
