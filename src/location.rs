use rand::{Rng, distributions::{Distribution, Standard}};
use crate::lang;

enum LocationType {
    City,
    Swamp,
    Forest,
    Plains,
    Mountains
}

impl Distribution<LocationType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LocationType {
        match rng.gen_range(0, 6) {
            0 => LocationType::Swamp,
            1 => LocationType::Forest,
            2 => LocationType::Plains,
            3 => LocationType::Mountains,
            _ => LocationType::City
        }
    }
}

impl ToString for LocationType {
    fn to_string(&self) -> String {
        match self {
            LocationType::Swamp => "Swamp",
            LocationType::Forest => "Forest",
            LocationType::Plains => "Plains",
            LocationType::Mountains => "Mountains",
            LocationType::City => "City"
        }.to_string()
    }
}

pub struct Location {
    name: String,
    location_type: LocationType
}

impl Location {
    pub fn new(lang: &lang::Language) -> Location {
        let name = lang.gen_location_name();

        let mut rng = rand::thread_rng();
        let location_type: LocationType = rng.gen();

        Location{name, location_type}
    }
}

impl ToString for Location {
    fn to_string(&self) -> String {
        match self.location_type {
            LocationType::City => self.name.clone(),
            _ => self.name.clone() + " " + &self.location_type.to_string()
        }
    }
}
