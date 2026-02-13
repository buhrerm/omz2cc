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

// ➜ omz2cc git:(main) ✓
const ROBBYRUSSELL: TemplateDef = TemplateDef {
    name: "robbyrussell",
    segments: &[
        lit("➜", Green),
        text(" "),
        field(CwdBasename, Cyan),
        if_git(&[
            text(" "),
            lit("git:(", Blue),
            field(GitBranch, Red),
            lit(")", Blue),
            text(" "),
            dirty("✓", "✗", Green, Yellow),
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
const BUREAU: TemplateDef = TemplateDef {
    name: "bureau",
    segments: &[
        field(User, Cyan),
        text(" "),
        field_bold(Cwd, Blue),
        text(" "),
        field_fmt(Time, Gray, "[", "]"),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Magenta, "git:", ""),
            dirty(" ✓", " ✗", Green, Yellow),
        ]),
    ],
};

// mike@zulu [15:35:04] [~/Workspace/omz2cc] [main]
const CANDY: TemplateDef = TemplateDef {
    name: "candy",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(Hostname, Blue),
        text(" "),
        field_fmt(Time, Gray, "[", "]"),
        text(" "),
        field_fmt_bold(Cwd, Yellow, "[", "]"),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Cyan, "[", "]"),
        ]),
    ],
};

// [15:35:04] zulu:~/Workspace/omz2cc @main mike
const DALLAS: TemplateDef = TemplateDef {
    name: "dallas",
    segments: &[
        field_fmt(Time, Gray, "[", "]"),
        text(" "),
        field(HostColonCwd, Cyan),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Green, "@", ""),
        ]),
        text(" "),
        field(User, Yellow),
    ],
};

// ~/Workspace/omz2cc git:(main) ✓
const GALLOIS: TemplateDef = TemplateDef {
    name: "gallois",
    segments: &[
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("git:(", Blue),
            field(GitBranch, Red),
            lit(")", Blue),
            text(" "),
            dirty("✓", "✗", Green, Yellow),
        ]),
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
};

// dir ➤
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
};

// user@host ~/dir branch ✔/✗
const AVIT: TemplateDef = TemplateDef {
    name: "avit",
    segments: &[
        field(User, Blue),
        lit("@", Blue),
        field(ShortHostname, Blue),
        text(" "),
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            field(GitBranch, Green),
            dirty(" ✔", " ✗", Green, Red),
        ]),
    ],
};

// user@host:~/dir [branch] ⚡
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
            lit("[", White),
            field(GitBranch, Magenta),
            lit("]", White),
            dirty(" ✓", " ✗", Green, Yellow),
        ]),
    ],
};

// user:dir/ (branch✗)
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
};

// ☁ dir [branch]⚡
const CLOUD: TemplateDef = TemplateDef {
    name: "cloud",
    segments: &[
        lit_bold("☁", Cyan),
        text(" "),
        field(CwdBasename, Green),
        if_git(&[
            text(" "),
            lit("[", Green),
            field(GitBranch, Cyan),
            dirty("]", "] ⚡", Green, Yellow),
        ]),
    ],
};

// [HH:MM] ~/dir [user@host] :branch ✓/✗
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
};

// host :: ~/dir »
const CYPHER: TemplateDef = TemplateDef {
    name: "cypher",
    segments: &[
        field(ShortHostname, White),
        lit(" :: ", Red),
        field(Cwd, Green),
    ],
};

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
};

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
};

// user@host: ~/dir branch!
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
};

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
};

// user@host:~/dir (branch)
const EASTWOOD: TemplateDef = TemplateDef {
    name: "eastwood",
    segments: &[
        field(User, Cyan),
        lit("@", White),
        field(ShortHostname, Cyan),
        lit(":", White),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit("(", Cyan),
            field(GitBranch, Cyan),
            lit(")", Cyan),
        ]),
    ],
};

// user@host ➜ ~/dir ±branch
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
            field_fmt(GitBranch, Magenta, "±", ""),
            dirty(" ✓", " ✗", Green, Yellow),
        ]),
    ],
};

// host :: ~/dir »
const EVAN: TemplateDef = TemplateDef {
    name: "evan",
    segments: &[
        field(ShortHostname, White),
        lit(" :: ", Red),
        field(Cwd, Green),
    ],
};

// ╭─ user at host in ~/dir on branch✔
const FINO: TemplateDef = TemplateDef {
    name: "fino",
    segments: &[
        lit("╭─", Blue),
        text(" "),
        field(User, Green),
        text(" "),
        lit("at", Gray),
        text(" "),
        field(ShortHostname, Blue),
        text(" "),
        lit("in", Gray),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit("on", Gray),
            text(" "),
            field(GitBranch, White),
            dirty("✔", "✘✘✘", Green, Red),
        ]),
    ],
};

// ╭─ user at host in ~/dir on branch✔ MM/DD - HH:MM:SS
const FINO_TIME: TemplateDef = TemplateDef {
    name: "fino-time",
    segments: &[
        lit("╭─", Blue),
        text(" "),
        field(User, Green),
        text(" "),
        lit("at", Gray),
        text(" "),
        field(ShortHostname, Blue),
        text(" "),
        lit("in", Gray),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit("on", Gray),
            text(" "),
            field(GitBranch, White),
            dirty("✔", "✘✘✘", Green, Red),
        ]),
        text(" "),
        field(Time, White),
    ],
};

// host :: ~/dir ‹branch›
const FLAZZ: TemplateDef = TemplateDef {
    name: "flazz",
    segments: &[
        field(ShortHostname, White),
        lit(" :: ", Magenta),
        field(Cwd, Green),
        if_git(&[
            text(" "),
            lit("‹", Cyan),
            field(GitBranch, Cyan),
            lit("›", Cyan),
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

// ~/dir [git:branch] ✔/✖
const FRONTCUBE: TemplateDef = TemplateDef {
    name: "frontcube",
    segments: &[
        field(Cwd, Gray),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Blue, "[git:", "]"),
            dirty(" ✔", " ✖", Green, Red),
        ]),
    ],
};

// host ~/dir ‹branch› »
const GALLIFREY: TemplateDef = TemplateDef {
    name: "gallifrey",
    segments: &[
        field(ShortHostname, Cyan),
        text(" "),
        field(Cwd, White),
        if_git(&[
            text(" "),
            lit("‹", Yellow),
            field(GitBranch, Yellow),
            lit("›", Yellow),
        ]),
    ],
};

// user@host ~/dir git:(branch)✗
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
            field_fmt(GitBranch, Yellow, "git:(", ")"),
            dirty("", "✗", Green, Red),
        ]),
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

// ╭─ user@host ~/dir git:(branch)
const GNZH: TemplateDef = TemplateDef {
    name: "gnzh",
    segments: &[
        lit("╭─", Blue),
        field(User, Green),
        lit("@", Cyan),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Yellow, "git:(", ")"),
        ]),
    ],
};

// user in ~/dir git:(branch)●
const HALF_LIFE: TemplateDef = TemplateDef {
    name: "half-life",
    segments: &[
        field(User, Magenta),
        text(" in "),
        field(Cwd, Green),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Cyan, "git:(", ")"),
            dirty("", "●", Cyan, Yellow),
        ]),
    ],
};

// user@host ~/dir
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
            dirty("✔", "✗", Green, Red),
        ]),
    ],
};

// ~/dir - branch -
const JONATHAN: TemplateDef = TemplateDef {
    name: "jonathan",
    segments: &[
        field(User, Blue),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" - "),
            field(GitBranch, Red),
            dirty(" ✓", " ✗", Green, Yellow),
            text(" -"),
        ]),
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

// dir git:(branch) »
const KENNETHREITZ: TemplateDef = TemplateDef {
    name: "kennethreitz",
    segments: &[
        field(CwdBasename, Green),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Yellow, "git:(", ")"),
        ]),
        text(" "),
        lit("»", Red),
    ],
};

// dir git:(branch)
const KOLO: TemplateDef = TemplateDef {
    name: "kolo",
    segments: &[
        field_bold(CwdBasename, Magenta),
        if_git(&[
            text(" "),
            lit("git:(", Green),
            field(GitBranch, Green),
            lit(")", Green),
        ]),
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

// λ ~/dir git:(branch)
const LAMBDA: TemplateDef = TemplateDef {
    name: "lambda",
    segments: &[
        lit("λ", White),
        text(" "),
        field(Cwd, White),
        if_git(&[
            text(" "),
            lit("git:(", Green),
            field(GitBranch, Green),
            lit(")", Green),
        ]),
    ],
};

// user@host ~/dir (branch)
const LUKERANDALL: TemplateDef = TemplateDef {
    name: "lukerandall",
    segments: &[
        field_bold(User, Green),
        lit_bold("@", Green),
        field_bold(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", Yellow),
            field(GitBranch, Yellow),
            lit(")", Yellow),
        ]),
    ],
};

// ~/dir git:(branch)
const MACOVSKY: TemplateDef = TemplateDef {
    name: "macovsky",
    segments: &[
        field(Cwd, Green),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Yellow, "git:(", ")"),
        ]),
    ],
};

// dir git:(branch)
const MGUTZ: TemplateDef = TemplateDef {
    name: "mgutz",
    segments: &[
        field_bold(CwdBasename, Magenta),
        if_git(&[
            text(" "),
            field_fmt_bold(GitBranch, Yellow, "git:(", ")"),
        ]),
    ],
};

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

// ~/dir git:[branch]●
const MINIMAL: TemplateDef = TemplateDef {
    name: "minimal",
    segments: &[
        field(Cwd, White),
        if_git(&[
            text(" "),
            lit("git:[", White),
            field(GitBranch, White),
            lit("]", White),
            dirty("", "●", White, Red),
        ]),
    ],
};

// user@host ~/dir
const MIRA: TemplateDef = TemplateDef {
    name: "mira",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
        ]),
    ],
};

// ~/dir >> branch
const MORTALSCUMBAG: TemplateDef = TemplateDef {
    name: "mortalscumbag",
    segments: &[
        field_bold(Cwd, Green),
        if_git(&[
            text(" >> "),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
        ]),
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

// ▸ ~/dir ±branch
const NANOTECH: TemplateDef = TemplateDef {
    name: "nanotech",
    segments: &[
        lit("▸", Green),
        text(" "),
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Magenta, "±", ""),
            dirty(" ✓", " ✗", Green, Yellow),
        ]),
    ],
};

// ~/dir (branch)
const NICOULAJ: TemplateDef = TemplateDef {
    name: "nicoulaj",
    segments: &[
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch)
const NORM: TemplateDef = TemplateDef {
    name: "norm",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user in ~/dir on branch
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
            lit("on", White),
            text(" "),
            field_bold(GitBranch, Blue),
            dirty(" ✓", " ✗", Green, Yellow),
        ]),
    ],
};

// [~/dir] (branch)
const PHILIPS: TemplateDef = TemplateDef {
    name: "philips",
    segments: &[
        lit("[", White),
        field(Cwd, Cyan),
        lit("]", White),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// user@host ⮞ ~/dir ▶ (branch ✓)
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
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// → ~/dir (branch)
const RE5ET: TemplateDef = TemplateDef {
    name: "re5et",
    segments: &[
        lit("→", Green),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// ~/dir branch
const REFINED: TemplateDef = TemplateDef {
    name: "refined",
    segments: &[
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            field(GitBranch, Magenta),
            dirty(" ✓", " ✗", Green, Yellow),
        ]),
    ],
};

// user@host:~/dir [branch]
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
            lit("[", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit("]", White),
        ]),
    ],
};

// ~/dir (branch ✓)
const RISTO: TemplateDef = TemplateDef {
    name: "risto",
    segments: &[
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Magenta),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// user@host:~/dir
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
            field(GitBranch, Magenta),
            dirty(" ✓", " ✗", Green, Red),
        ]),
    ],
};

// user@host:~/dir
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
            lit("(", White),
            field(GitBranch, Yellow),
            lit(")", White),
        ]),
    ],
};

// ┌ [~/dir] [branch]
const SKARO: TemplateDef = TemplateDef {
    name: "skaro",
    segments: &[
        lit("┌", Blue),
        text(" "),
        field_fmt_bold(Cwd, Cyan, "[", "]"),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Yellow, "[", "]"),
            dirty(" ✓", " ✗", Green, Red),
        ]),
    ],
};

// user@host ~/dir (branch)
const SMT: TemplateDef = TemplateDef {
    name: "smt",
    segments: &[
        field(User, Cyan),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field_bold(Cwd, Green),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Magenta),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// [~/dir] (branch)
const SONICRADISH: TemplateDef = TemplateDef {
    name: "sonicradish",
    segments: &[
        lit("[", White),
        field_bold(Cwd, Green),
        lit("]", White),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// ~/dir ❯ branch
const SORIN: TemplateDef = TemplateDef {
    name: "sorin",
    segments: &[
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("❯", Magenta),
            text(" "),
            field(GitBranch, Blue),
            dirty(" ✓", " ✗", Green, Yellow),
        ]),
    ],
};

// user at host in ~/dir (branch ✓)
const STEEEF: TemplateDef = TemplateDef {
    name: "steeef",
    segments: &[
        field_bold(User, Red),
        text(" "),
        lit("at", White),
        text(" "),
        field_bold(ShortHostname, Yellow),
        text(" "),
        lit("in", White),
        text(" "),
        field_bold(Cwd, Green),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Cyan),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// ~/dir ± branch
const SUNAKU: TemplateDef = TemplateDef {
    name: "sunaku",
    segments: &[
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("±", Yellow),
            text(" "),
            field(GitBranch, Magenta),
            dirty(" ✓", " ✗", Green, Red),
        ]),
    ],
};

// user@host ~/dir (branch)
const SUVASH: TemplateDef = TemplateDef {
    name: "suvash",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Blue),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Cyan),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// ~/dir (branch)
const TAKASHIYOSHIDA: TemplateDef = TemplateDef {
    name: "takashiyoshida",
    segments: &[
        field(Cwd, Green),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir
const TERMINALPARTY: TemplateDef = TemplateDef {
    name: "terminalparty",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Magenta),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// ~/dir (branch) ➜
const THEUNRAVELER: TemplateDef = TemplateDef {
    name: "theunraveler",
    segments: &[
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Red),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
        text(" "),
        lit("➜", Green),
    ],
};

// ~/dir (branch ✓)
const TONOTDO: TemplateDef = TemplateDef {
    name: "tonotdo",
    segments: &[
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// ~/dir git:branch ➜
const WEDISAGREE: TemplateDef = TemplateDef {
    name: "wedisagree",
    segments: &[
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            field_fmt(GitBranch, Yellow, "git:", ""),
            dirty(" ✓", " ✗", Green, Red),
        ]),
        text(" "),
        lit("➜", Green),
    ],
};

// user@host [~/dir]
const WEZM: TemplateDef = TemplateDef {
    name: "wezm",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field_fmt_bold(Cwd, Blue, "[", "]"),
        if_git(&[
            text(" "),
            field(GitBranch, Magenta),
            dirty(" ✓", " ✗", Green, Red),
        ]),
    ],
};

// user@host [~/dir] ⚡ branch
const WEZM_PLUS: TemplateDef = TemplateDef {
    name: "wezm+",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field_fmt_bold(Cwd, Blue, "[", "]"),
        if_git(&[
            text(" "),
            lit("⚡", Yellow),
            text(" "),
            field(GitBranch, Magenta),
            dirty(" ✓", " ✗", Green, Red),
        ]),
    ],
};

// user@host ~/dir ⚡ branch
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
            lit("⚡", Yellow),
            text(" "),
            field(GitBranch, Magenta),
            dirty(" ✓", " ✗", Green, Red),
        ]),
    ],
};

// [~/dir] user@host
const XIONG_CHIAMIOV: TemplateDef = TemplateDef {
    name: "xiong-chiamiov",
    segments: &[
        field_fmt_bold(Cwd, Blue, "[", "]"),
        text(" "),
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// [~/dir] user@host ⚡ branch
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
            lit("⚡", Yellow),
            text(" "),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
        ]),
    ],
};

// ~/dir (branch)
const ZHANN: TemplateDef = TemplateDef {
    name: "zhann",
    segments: &[
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir {branch}
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
            lit("{", Red),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Yellow),
            lit("}", Red),
        ]),
    ],
};

// ☿ ~/dir (branch)
const EDVARDM: TemplateDef = TemplateDef {
    name: "edvardm",
    segments: &[
        lit("☿", Blue),
        text(" "),
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// 🦊 ~/dir [branch]
const FOX: TemplateDef = TemplateDef {
    name: "fox",
    segments: &[
        text("🦊 "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("[", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit("]", White),
        ]),
    ],
};

// user@host ~/dir — branch
const GEOFFGARSIDE: TemplateDef = TemplateDef {
    name: "geoffgarside",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field(Cwd, Cyan),
        if_git(&[
            text(" — "),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
        ]),
    ],
};

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
            field(GitBranch, Red),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir ❯ branch
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
            lit("❯", Magenta),
            text(" "),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Red),
        ]),
    ],
};

// user@host ~/dir (branch)
const IMAJES: TemplateDef = TemplateDef {
    name: "imajes",
    segments: &[
        field(User, Yellow),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Magenta),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host:~/dir (branch)
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
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// ~/dir:branch
const JISPWOSO: TemplateDef = TemplateDef {
    name: "jispwoso",
    segments: &[
        field(Cwd, Green),
        if_git(&[
            lit(":", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
        ]),
    ],
};

// user:~/dir (branch)
const JNROWE: TemplateDef = TemplateDef {
    name: "jnrowe",
    segments: &[
        field(User, Cyan),
        lit(":", White),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch)
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
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

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
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir [branch]
const JUNKFOOD: TemplateDef = TemplateDef {
    name: "junkfood",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("[", White),
            field(GitBranch, Magenta),
            dirty(" ✓", " ✗", Green, Red),
            lit("]", White),
        ]),
    ],
};

// user@host ~/dir [branch]
const KAFEITU: TemplateDef = TemplateDef {
    name: "kafeitu",
    segments: &[
        field(User, Cyan),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit("[", White),
            field(GitBranch, Blue),
            dirty(" ✓", " ✗", Green, Red),
            lit("]", White),
        ]),
    ],
};

// 🥝 ~/dir (branch)
const KIWI: TemplateDef = TemplateDef {
    name: "kiwi",
    segments: &[
        text("🥝 "),
        field_bold(Cwd, Green),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch)
const LINUXONLY: TemplateDef = TemplateDef {
    name: "linuxonly",
    segments: &[
        field_bold(User, Green),
        lit("@", Green),
        field_bold(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// ~/dir ❯ branch ❯
const MACOVSKY_RUBY: TemplateDef = TemplateDef {
    name: "macovsky-ruby",
    segments: &[
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("❯", Magenta),
            text(" "),
            field(GitBranch, Red),
            dirty(" ✓", " ✗", Green, Yellow),
        ]),
        text(" "),
        lit("❯", Magenta),
    ],
};

// ~/dir (branch)
const MH: TemplateDef = TemplateDef {
    name: "mh",
    segments: &[
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Red),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// ~/dir (branch)
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
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir [branch]
const NEBIRHOS: TemplateDef = TemplateDef {
    name: "nebirhos",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Blue),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("[", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit("]", White),
        ]),
    ],
};

// user@host ~/dir (branch)
const OBRAUN: TemplateDef = TemplateDef {
    name: "obraun",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch)
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
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// [~/dir] (branch)
const RIXIUS: TemplateDef = TemplateDef {
    name: "rixius",
    segments: &[
        lit("[", Blue),
        field(Cwd, Cyan),
        lit("]", Blue),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir [branch]
const RKJ: TemplateDef = TemplateDef {
    name: "rkj",
    segments: &[
        field(User, Cyan),
        lit("@", White),
        field(ShortHostname, Cyan),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("[", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Yellow),
            lit("]", White),
        ]),
    ],
};

// user@host ~/dir (branch)
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
            lit("(", White),
            field(GitBranch, Magenta),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// ★ ~/dir (branch)
const SPORTY_256: TemplateDef = TemplateDef {
    name: "sporty_256",
    segments: &[
        lit("★", Green),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch ✓)
const SUNRISE: TemplateDef = TemplateDef {
    name: "sunrise",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Cyan),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch)
const SUPERJARIN: TemplateDef = TemplateDef {
    name: "superjarin",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// [~/dir] (branch)
const TJKIRCH: TemplateDef = TemplateDef {
    name: "tjkirch",
    segments: &[
        lit("[", White),
        field(Cwd, Cyan),
        lit("]", White),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch)
const TRAPD00R: TemplateDef = TemplateDef {
    name: "trapd00r",
    segments: &[
        field(User, Magenta),
        lit("@", White),
        field(ShortHostname, Cyan),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir
const CRCANDY: TemplateDef = TemplateDef {
    name: "crcandy",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Magenta),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

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
            lit("(", White),
            field(GitBranch, Cyan),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch ✓)
const FLETCHERM: TemplateDef = TemplateDef {
    name: "fletcherm",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
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

// user@host ~/dir (branch)
const FUNKY: TemplateDef = TemplateDef {
    name: "funky",
    segments: &[
        field(User, Magenta),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch)
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
            lit("(", White),
            field(GitBranch, Blue),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch)
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
            lit("(", White),
            field(GitBranch, Red),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch)
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
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch)
const AUSSIEGEEK: TemplateDef = TemplateDef {
    name: "aussiegeek",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Blue),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Red),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// [~/dir] user@host (branch)
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
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch)
const SOLIAH: TemplateDef = TemplateDef {
    name: "Soliah",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Red),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch)
const ADBEN: TemplateDef = TemplateDef {
    name: "adben",
    segments: &[
        field(User, Cyan),
        lit("@", White),
        field(ShortHostname, Green),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Red),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch)
const AFOWLER: TemplateDef = TemplateDef {
    name: "afowler",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field(Cwd, Magenta),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Cyan),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// [user@host ~/dir] (branch)
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
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// ⌘ user@host ~/dir (branch)
const APPLE: TemplateDef = TemplateDef {
    name: "apple",
    segments: &[
        lit("⌘", White),
        text(" "),
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Yellow),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Red),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// 🐼 ~/dir (branch)
const AWESOMEPANDA: TemplateDef = TemplateDef {
    name: "awesomepanda",
    segments: &[
        text("🐼 "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch)
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
            lit("(", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

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
            lit("(", White),
            field(GitBranch, Magenta),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch)
const DUELLJ: TemplateDef = TemplateDef {
    name: "duellj",
    segments: &[
        field(User, Green),
        lit("@", White),
        field(ShortHostname, Blue),
        text(" "),
        field_bold(Cwd, Yellow),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Red),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// > ~/dir (branch)
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
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
        text(" "),
        lit(">", White),
    ],
};

// user@host ~/dir (branch)
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
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir (branch)
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
            lit("(", White),
            field(GitBranch, Magenta),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// ➜ ~/dir (branch)
const MUSE: TemplateDef = TemplateDef {
    name: "muse",
    segments: &[
        lit("➜", Green),
        text(" "),
        field_bold(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// ~/dir git:(branch) ✓
const OLDGALLOIS: TemplateDef = TemplateDef {
    name: "oldgallois",
    segments: &[
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("git:(", Blue),
            field(GitBranch, Red),
            lit(")", Blue),
            text(" "),
            dirty("✓", "✗", Green, Yellow),
        ]),
    ],
};

// user@host ⮞ ~/dir ▶ (branch)
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
            lit("(", White),
            field(GitBranch, Yellow),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// user@host ~/dir [branch]
const RKJ_REPOS: TemplateDef = TemplateDef {
    name: "rkj-repos",
    segments: &[
        field(User, Cyan),
        lit("@", White),
        field(ShortHostname, Cyan),
        text(" "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("[", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Yellow),
            lit("]", White),
        ]),
    ],
};

// ~/dir (branch)
const STRUG: TemplateDef = TemplateDef {
    name: "strug",
    segments: &[
        field(Cwd, Cyan),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Red),
            lit(")", White),
        ]),
    ],
};

// [~/dir] (branch) ~
const TJKIRCH_MOD: TemplateDef = TemplateDef {
    name: "tjkirch_mod",
    segments: &[
        lit("[", White),
        field(Cwd, Cyan),
        lit("]", White),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
    ],
};

// 😄 ~/dir (branch)
const EMOTTY: TemplateDef = TemplateDef {
    name: "emotty",
    segments: &[
        text("😄 "),
        field_bold(Cwd, Blue),
        if_git(&[
            text(" "),
            lit("(", White),
            field(GitBranch, Green),
            dirty(" ✓", " ✗", Green, Yellow),
            lit(")", White),
        ]),
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
