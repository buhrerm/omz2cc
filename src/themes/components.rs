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
