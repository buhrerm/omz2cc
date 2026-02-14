// ---------------------------------------------------------------------------
// Reusable theme components — common git blocks and layout elements
//
// Git block components include a leading text(" ") separator for direct use
// inside if_git(): `if_git(GIT_ANGLE_YELLOW)` renders ` ‹branch›` when in git.
//
// Each component is used by 1+ themes in defs.rs to reduce duplication.
// ---------------------------------------------------------------------------

use crate::color::Color::*;
use crate::themes::template::*;
use FieldName::*;

// ===========================================================================
// Git block components (for use inside if_git())
// ===========================================================================

/// ‹branch› — yellow angle brackets, no dirty marker
/// Used by: afowler, gallifrey, gnzh, macovsky
pub const GIT_ANGLE_YELLOW: &[Segment] = &[
    text(" "),
    lit("‹", Yellow),
    field(GitBranch, Yellow),
    lit("›", Yellow),
];

/// ‹branch●› — yellow angles, red dirty dot (bira style)
/// Used by: bira
pub const GIT_ANGLE_YELLOW_DIRTY_DOT: &[Segment] = &[
    text(" "),
    lit("‹", Yellow),
    field(GitBranch, Yellow),
    dirty("", "●", Yellow, Red),
    lit("›", Yellow),
];

/// ‹branch› — red angle brackets, no dirty marker
/// Used by: obraun, risto
pub const GIT_ANGLE_RED: &[Segment] = &[
    text(" "),
    lit("‹", Red),
    field(GitBranch, Red),
    lit("›", Red),
];

/// git:(branch) ✗ — robbyrussell style (bold blue parens, red branch, yellow dirty)
/// Used by: robbyrussell
pub const GIT_COLON_PAREN_ROBBYRUSSELL: &[Segment] = &[
    text(" "),
    lit_bold("git:(", Blue),
    field(GitBranch, Red),
    dirty(")", ") ✗", Blue, Yellow),
];

/// git:(branch) ✗ — red variant (red git:(), red branch, yellow dirty)
/// Used by: awesomepanda, edvardm, jispwoso, kafeitu, nebirhos, skaro
pub const GIT_COLON_PAREN_RED: &[Segment] = &[
    text(" "),
    lit("git:(", Red),
    field(GitBranch, Red),
    dirty(")", ") ✗", Blue, Yellow),
];

/// (branch●) — vcs_info style (magenta parens, green branch, red ● dirty)
/// Used by: apple, emotty, jnrowe, kolo, linuxonly, trapd00r, zhann
pub const GIT_VCS_INFO: &[Segment] = &[
    text(" "),
    lit("(", Magenta),
    field(GitBranch, Green),
    dirty("", "●", Green, Red),
    lit(")", Magenta),
];

/// (branch) — yellow parens, no dirty marker
/// Used by: lukerandall, mira
pub const GIT_PAREN_YELLOW: &[Segment] = &[
    text(" "),
    lit("(", Yellow),
    field(GitBranch, Yellow),
    lit(")", Yellow),
];

/// [branch *] — green brackets, red dirty star
/// Used by: candy, crcandy, eastwood, oldgallois
pub const GIT_BRACKET_GREEN: &[Segment] = &[
    text(" "),
    lit("[", Green),
    field(GitBranch, Green),
    dirty("]", " *]", Green, Red),
];

/// git:branch — sorin style (blue "git:", white ":", red branch)
/// Used by: sorin
pub const GIT_COLON_SORIN: &[Segment] = &[
    text(" "),
    lit("git", Blue),
    lit(":", White),
    field(GitBranch, Red),
];

// ===========================================================================
// Layout components
// ===========================================================================

/// Box-drawing top line: ╭─ (bira/gnzh/fino style)
/// Used by: bira, gnzh, fino, fino-time
pub const BOX_TOP: Segment = lit("╭─", Blue);

/// user@host (bold green) — gentoo/lukerandall/linuxonly style
/// Used by: gentoo, lukerandall, linuxonly
pub const USER_AT_HOST_GREEN_BOLD: &[Segment] = &[
    field_bold(User, Green),
    lit_bold("@", Green),
    field_bold(ShortHostname, Green),
];

// ===========================================================================
// RPROMPT components (for use in the rprompt field)
// ===========================================================================

/// [HH:MM:SS] — used by clean, dst, duellj, fletcherm, philips, pmcgee,
/// tjkirch, tjkirch_mod, tonotdo
pub const RPROMPT_TIME_BRACKET: &[Segment] = &[
    lit("[", White),
    field(Time, White),
    lit("]", White),
];

/// git:branch* (yellow) — arrow RPROMPT style
/// Used by: arrow
pub const RPROMPT_GIT_ARROW: &[Segment] = &[
    if_git(&[
        field_fmt(GitBranch, Yellow, "git:", ""),
        dirty("", "*", Yellow, Yellow),
    ]),
];

/// (branch✱) — gray parens, bold yellow branch, red dirty — mh RPROMPT style
/// Used by: mh
pub const RPROMPT_GIT_MH: &[Segment] = &[
    if_git(&[
        lit("(", Gray),
        field_bold(GitBranch, Yellow),
        dirty("", "✱", Yellow, Red),
        lit(")", Gray),
        text(" "),
    ]),
];

/// <branch ✗> — bold green wrapper, red branch, yellow dirty ✗ — mrtazz RPROMPT
/// Used by: mrtazz
pub const RPROMPT_GIT_MRTAZZ: &[Segment] = &[
    if_git(&[
        lit_bold("<", Green),
        field(GitBranch, Red),
        dirty(">", " ✗>", Green, Yellow),
    ]),
];

/// [git:branch] ✔/✖ — bold blue brackets — frontcube RPROMPT
/// Used by: frontcube
pub const RPROMPT_GIT_FRONTCUBE: &[Segment] = &[
    if_git(&[
        lit_bold("[git:", Blue),
        field_bold(GitBranch, Blue),
        dirty("] ✔", "] ✖", Green, Red),
    ]),
];

/// branch — magenta, no dirty/clean markers — theunraveler RPROMPT
/// Used by: theunraveler
pub const RPROMPT_GIT_THEUNRAVELER: &[Segment] = &[
    if_git(&[
        field(GitBranch, Magenta),
    ]),
];

/// branch ✗/✔ — white branch, red/green dirty — itchy RPROMPT
/// Used by: itchy
pub const RPROMPT_GIT_ITCHY: &[Segment] = &[
    if_git(&[
        field(GitBranch, White),
        dirty(" ✔", " ✗", Green, Red),
    ]),
];

/// branch — plain, no dirty in prefix/suffix — fishy RPROMPT
/// Used by: fishy
pub const RPROMPT_GIT_FISHY: &[Segment] = &[
    if_git(&[
        text(" "),
        field(GitBranch, White),
    ]),
];

/// branch* ] 12:00 PM — nanotech RPROMPT (git + bracket + 12h time)
/// Used by: nanotech
pub const RPROMPT_NANOTECH: &[Segment] = &[
    if_git(&[
        field(GitBranch, Yellow),
        dirty("", " *", Yellow, Red),
    ]),
    text(" "),
    lit("]", Blue),
    text(" "),
    field(Time, Green),
];

/// ~/dir(branch✗)@host — kardan RPROMPT
/// Used by: kardan
pub const RPROMPT_KARDAN: &[Segment] = &[
    field(Cwd, White),
    if_git(&[
        lit("(", White),
        field(GitBranch, White),
        dirty(")", "✗)", White, Yellow),
    ]),
    lit("@", White),
    field(Hostname, White),
];

/// ‹git:branch ✘/✔› time — adben RPROMPT
/// Used by: adben
pub const RPROMPT_ADBEN: &[Segment] = &[
    if_git(&[
        lit("‹", Red),
        field_fmt(GitBranch, Red, "git:", ""),
        dirty(" ✔", " ✘", Green, Yellow),
        lit("›", Red),
        text(" "),
    ]),
    field(Time, Yellow),
];

/// time ☁ branch ☂/☀ — wedisagree RPROMPT
/// Used by: wedisagree
pub const RPROMPT_WEDISAGREE: &[Segment] = &[
    field(Time, Green),
    if_git(&[
        lit(" ☁ ", Magenta),
        field(GitBranch, Red),
        dirty(" ☀", " ☂", Green, Yellow),
    ]),
];

/// %2~(branch⚡) host — terminalparty RPROMPT
/// Used by: terminalparty
pub const RPROMPT_TERMINALPARTY: &[Segment] = &[
    field(CwdTruncated(2), White),
    if_git(&[
        lit("(", Yellow),
        field(GitBranch, Yellow),
        dirty(")", "⚡)", Yellow, Red),
    ]),
    text(" "),
    field_bold(ShortHostname, Blue),
];

/// branch(*) — bold green, juanghurtado RPROMPT
/// Used by: juanghurtado
pub const RPROMPT_GIT_JUANGHURTADO: &[Segment] = &[
    if_git(&[
        field_bold(GitBranch, Green),
        dirty("", "(*)", Green, Red),
    ]),
];
