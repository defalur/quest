use crate::character;
use crate::location;
use crate::mob;
use crate::lang;

pub struct World {
    lang: lang::Language,
    mobs: Vec<mob::Mob>,
    locations: Vec<location::Location>,
    characters: Vec<character::Character>
}

impl World {
    pub fn new(n_mobs: usize, n_location: usize, n_characters: usize, lang: lang::Language) -> World {
        let mobs: Vec<mob::Mob> = (0..n_mobs)
            .map(|_| -> mob::Mob {mob::Mob::new(&lang)})
            .collect();
        let locations: Vec<location::Location> = (0..n_location)
            .map(|_| -> location::Location {location::Location::new(&lang)})
            .collect();
        let characters: Vec<character::Character> = (0..n_characters)
            .map(|_| -> character::Character {character::Character::new(&lang)})
            .collect();

        World{lang, mobs, locations, characters}
    }

    pub fn rand_character(&self, seed: usize) -> Option<&character::Character> {
        if self.characters.len() > 0 {
            Some(&self.characters[seed % self.characters.len()])
        }
        else {
            None
        }
    }

    pub fn rand_location(&self, seed: usize) -> Option<&location::Location> {
        if self.locations.len() > 0 {
            Some(&self.locations[seed % self.locations.len()])
        }
        else {
            None
        }
    }

    pub fn rand_mob(&self, seed: usize) -> Option<&mob::Mob> {
        if self.mobs.len() > 0 {
            Some(&self.mobs[seed % self.mobs.len()])
        }
        else {
            None
        }
    }
}

impl ToString for World {
    fn to_string(&self) -> String {
        let mut result = "World: \n\n".to_string();
        result += &(self.lang.to_string() + "\n");
        result += "Characters: \n";
        for c in &self.characters {
            result += &("- ".to_string() + &c.to_string() + "\n");
        }
        result += "\nLocations: \n";
        for l in &self.locations {
            result += &("- ".to_string() + &l.to_string() + "\n");
        }
        result += "\nMobs: \n";
        for m in &self.mobs {
            result += &("- ".to_string() + &m.to_string() + "\n");
        }

        result
    }
}
