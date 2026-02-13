mod af_magic;
mod agnoster;
mod bira;
mod bureau;
mod candy;
mod dallas;
mod gallois;
mod maran;
mod robbyrussell;
mod ys;

use crate::info::StatusInfo;

pub trait Theme {
    fn format(&self, info: &StatusInfo) -> String;
    fn name(&self) -> &'static str;
}

pub fn all_themes() -> Vec<Box<dyn Theme>> {
    vec![
        Box::new(ys::Ys),
        Box::new(robbyrussell::Robbyrussell),
        Box::new(agnoster::Agnoster),
        Box::new(af_magic::AfMagic),
        Box::new(bira::Bira),
        Box::new(bureau::Bureau),
        Box::new(candy::Candy),
        Box::new(dallas::Dallas),
        Box::new(gallois::Gallois),
        Box::new(maran::Maran),
    ]
}

pub fn get_theme(name: &str) -> Option<Box<dyn Theme>> {
    all_themes().into_iter().find(|t| t.name() == name)
}
