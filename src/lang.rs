use rand::{Rng, distributions::{Distribution, Standard}};
use rand_distr::Normal;

const CONSONANTS: &'static str = "bcdfghjklmnpqrstvwxz";
const VOWELS: &'static str = "aeiouy";

enum PhonemeUse {
    General,
    City,
    People
}

impl Distribution<PhonemeUse> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PhonemeUse {
        match rng.gen_range(0, 8) {
            0 => PhonemeUse::City,
            1 => PhonemeUse::People,
            _ => PhonemeUse::General
        }
    }
}

struct Phoneme {
    data: String,
    usage: PhonemeUse
}

impl Phoneme {
    fn new_rand() -> Phoneme {
        let mut rng = rand::thread_rng();

        let mut data = String::new();
        //structures of phonemes: vc, cv, cvc, v
        let structure = rng.gen_range(0, 3);
        match structure {
            0 => {
                let idx = rng.gen_range(0, VOWELS.len());
                let v = VOWELS.chars().nth(idx).unwrap() as char;
                let idx = rng.gen_range(0, CONSONANTS.len());
                let c = CONSONANTS.chars().nth(idx).unwrap() as char;
                data.push(v);
                data.push(c);
            },//vc
            1 => {//cv
                let idx = rng.gen_range(0, VOWELS.len());
                let v = VOWELS.chars().nth(idx).unwrap() as char;
                let idx = rng.gen_range(0, CONSONANTS.len());
                let c = CONSONANTS.chars().nth(idx).unwrap() as char;
                data.push(c);
                data.push(v);
            },
            2 => {
                let idx = rng.gen_range(0, VOWELS.len());
                let v = VOWELS.chars().nth(idx).unwrap() as char;
                let idx = rng.gen_range(0, CONSONANTS.len());
                let c = CONSONANTS.chars().nth(idx).unwrap() as char;
                let idx = rng.gen_range(0, CONSONANTS.len());
                let c2 = CONSONANTS.chars().nth(idx).unwrap() as char;
                data.push(c);
                data.push(v);
                data.push(c2);
            },//cvc
            _ => {
                let idx = rng.gen_range(0, VOWELS.len());
                let v = VOWELS.chars().nth(idx).unwrap() as char;
                data.push(v);
            }//v
        }

        let phoneme_use: PhonemeUse = rng.gen();
        Phoneme{data, usage: phoneme_use}
    }
}

pub struct Language {
    general_phonemes: Vec<Phoneme>,
    city_phonemes: Vec<Phoneme>,
    people_phonemes: Vec<Phoneme>,
    len_gen: f64
}

impl Language {
    pub fn new(n_phonemes: usize, avg_length: f64) -> Language {
        let mut result = Language{general_phonemes: Vec::new(),
            city_phonemes: Vec::new(),
            people_phonemes: Vec::new(),
            len_gen: avg_length};

        for _ in 0..n_phonemes {
            let p = Phoneme::new_rand();
            match p.usage {
                PhonemeUse::General => result.general_phonemes.push(p),
                PhonemeUse::City => result.city_phonemes.push(p),
                PhonemeUse::People => result.people_phonemes.push(p)
            };
        }

        result
    }

    pub fn gen_name(&self) -> String { //only use general phonemes
        let normal = Normal::new(self.len_gen, 1.0).unwrap();
        let mut rng = rand::thread_rng();
        let mut name_len: f64 = normal.sample(&mut rng);
        while name_len < 2.0 {
            name_len = normal.sample(&mut rng);
        }
        //println!("len: {}, {}", name_len, name_len < 2.0);

        let iname_len: usize = name_len as usize;

        let mut result = String::new();
        for _ in 0..iname_len {
            let idx: usize = rng.gen_range(0, self.general_phonemes.len());
            let phoneme = self.general_phonemes[idx].data.clone();
            result.push_str(phoneme.as_str());
        }

        return result;
    }

    pub fn gen_location_name(&self) -> String { //only use general phonemes
        let normal = Normal::new(self.len_gen, 1.0).unwrap();
        let mut rng = rand::thread_rng();
        let mut name_len: f64 = normal.sample(&mut rng);
        while name_len < 2.0 {
            name_len = normal.sample(&mut rng);
        }
        //println!("len: {}, {}", name_len, name_len < 2.0);

        let iname_len: usize = name_len as usize;

        let mut result = String::new();
        for _ in 0..iname_len {
            let p: f64 = rng.gen();

            let phoneme = if p > 0.7 && self.city_phonemes.len() > 0 {
                let idx: usize = rng.gen_range(0, self.city_phonemes.len());
                self.city_phonemes[idx].data.clone()
            }
            else
            {

                let idx: usize = rng.gen_range(0, self.general_phonemes.len());
                self.general_phonemes[idx].data.clone()
            };
            result.push_str(phoneme.as_str());
        }

        return result;
    }

    pub fn gen_person_name(&self) -> String { //only use general phonemes
        let normal = Normal::new(self.len_gen, 1.0).unwrap();
        let mut rng = rand::thread_rng();
        let mut name_len: f64 = normal.sample(&mut rng);
        while name_len < 2.0 {
            name_len = normal.sample(&mut rng);
        }
        //println!("len: {}, {}", name_len, name_len < 2.0);

        let iname_len: usize = name_len as usize;

        let mut result = String::new();
        for _ in 0..iname_len {
            let p: f64 = rng.gen();

            let phoneme = if p > 0.7 && self.people_phonemes.len() > 0 {
                let idx: usize = rng.gen_range(0, self.people_phonemes.len());
                self.people_phonemes[idx].data.clone()
            }
            else
            {

                let idx: usize = rng.gen_range(0, self.general_phonemes.len());
                self.general_phonemes[idx].data.clone()
            };
            result.push_str(phoneme.as_str());
        }

        return result;
    }
}

impl ToString for Language {
    fn to_string(&self) -> String {
        let mut result = "Language:\n".to_string();
        result += "General: ";
        for p in &self.general_phonemes {
            result.push_str(p.data.clone().as_str());
            result.push_str(" ");
        }
        result += "\n";

        result += "City: ";
        for p in &self.city_phonemes {
            result.push_str(p.data.clone().as_str());
            result.push_str(" ");
        }
        result += "\n";

        result += "People: ";
        for p in &self.people_phonemes {
            result.push_str(p.data.clone().as_str());
            result.push_str(" ");
        }
        result += "\n";

        result
    }
}

#[cfg(test)]
mod phoneme_tests {
    use crate::lang;

    #[test]
    fn phoneme_rand_length() {
        for _ in 0..100 {
            let p = lang::Phoneme::new_rand();
            assert!(p.data.len() < 4);
            assert!(p.data.len() >= 1);
        }
    }

    #[test]
    fn phoneme_rand_usage() {
        let n = 10000;
        let tol = n / 10;
        let mut count_general = 0;
        let mut count_names = 0;
        for _ in 0..10000 {
            let p = lang::Phoneme::new_rand();
            match p.usage {
                lang::PhonemeUse::General => count_general += 1,
                _ => count_names += 1
            }
        }

        //println!("{}/{}", count_general, count_names);
        //compare values +- 10%
        assert!(count_general >= (count_names * 3) - tol && count_general <= (count_names * 3) + tol);
    }
}
