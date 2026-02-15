mod agnoster;
pub mod components;
pub mod defs;
pub mod template;

use crate::info::StatusInfo;

pub trait Theme {
    fn format(&self, info: &StatusInfo) -> String;
    fn name(&self) -> &'static str;
}

/// Return a theme by name. Template themes are checked first, then manual
/// implementations (for themes too complex for the template DSL).
/// "random" picks a random template theme.
pub fn get_theme(name: &str) -> Option<Box<dyn Theme>> {
    if name == "random" {
        let idx = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as usize % defs::TEMPLATES.len())
            .unwrap_or(0);
        return Some(Box::new(*defs::TEMPLATES[idx]));
    }
    // Template themes (the vast majority)
    if let Some(tpl) = defs::get_template(name) {
        return Some(Box::new(*tpl));
    }
    // Manual escape-hatch themes
    manual_themes().into_iter().find(|t| t.name() == name)
}

pub fn all_theme_names() -> Vec<&'static str> {
    let mut names: Vec<&'static str> = defs::TEMPLATES.iter().map(|t| t.name).collect();
    for t in manual_themes() {
        if !names.contains(&t.name()) {
            names.push(t.name());
        }
    }
    names.push("random");
    names.sort();
    names
}

pub fn all_themes() -> Vec<Box<dyn Theme>> {
    let mut themes: Vec<Box<dyn Theme>> = defs::TEMPLATES
        .iter()
        .map(|t| Box::new(**t) as Box<dyn Theme>)
        .collect();

    // Append any manual themes whose names aren't already covered by templates
    for t in manual_themes() {
        if !themes.iter().any(|existing| existing.name() == t.name()) {
            themes.push(t);
        }
    }

    themes
}

/// Manual theme implementations for themes that need logic beyond the DSL.
fn manual_themes() -> Vec<Box<dyn Theme>> {
    vec![
        // agnoster is kept as a manual implementation example.
        // Currently shadowed by the template version in defs.rs.
        Box::new(agnoster::Agnoster),
    ]
}
