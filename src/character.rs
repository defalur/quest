use rand::{Rng, distributions::{Distribution, Standard}};
use crate::lang;

enum Profession {
    Carpenter,
    Blacksmith,
    Hunter,
    Herbalist,
    Farmer,
    Tailor,
    Merchant,
    Guard,
    Innkeeper
}

impl Distribution<Profession> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Profession {
        match rng.gen_range(0, 9) {
            0 => Profession::Carpenter,
            1 => Profession::Blacksmith,
            2 => Profession::Hunter,
            3 => Profession::Herbalist,
            4 => Profession::Farmer,
            5 => Profession::Tailor,
            6 => Profession::Merchant,
            7 => Profession::Guard,
            _ => Profession::Innkeeper
        }
    }
}

impl ToString for Profession {
    fn to_string(&self) -> String {
        match self {
            Profession::Carpenter => "Carpenter",
            Profession::Blacksmith => "Blacksmith",
            Profession::Hunter => "Hunter",
            Profession::Herbalist => "Herbalist",
            Profession::Farmer => "Farmer",
            Profession::Tailor => "Tailor",
            Profession::Merchant => "Merchant",
            Profession::Guard => "Guard",
            Profession::Innkeeper => "Innkeeper"
        }.to_string()
    }
}

pub struct Character {
    name: String,
    profession: String
}

impl Character {
    pub fn new(lang: &lang::Language) -> Character {
        let name = lang.gen_person_name();

        let mut rng = rand::thread_rng();
        let p: f64 = rng.gen();
        if p > 0.5 {
            let profession: Profession = rng.gen();
            let profession = profession.to_string();
            Character{name, profession}
        }
        else {
            Character{name, profession: "".to_string()}
        }
    }
}

impl ToString for Character {
    fn to_string(&self) -> String {
        if self.profession.len() > 0 {
            self.name.clone() + " the " + &self.profession
        }
        else {
            self.name.clone()
        }
    }
}
