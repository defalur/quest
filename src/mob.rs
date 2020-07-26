use rand::{Rng, distributions::{Distribution, Standard, Normal}};
use crate::lang;

pub struct Mob {
    name: String
}

impl Mob {
    pub fn new(lang: &lang::Language) -> Mob {
        let name = lang.gen_name();

        Mob{name}
    }
}

impl ToString for Mob {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}
