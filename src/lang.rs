use rand::{Rng, distributions::{Distribution, Standard}};

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
    fn new(data: &str, usage: PhonemeUse) -> Phoneme {
        Phoneme{data: data.to_string(), usage}
    }

    fn new_rand() -> Phoneme {
        let mut rng = rand::thread_rng();

        let len: usize = rng.gen_range(1, 4);
        let mut data = String::new();
        for _ in 0..len {
            let c =
                if rng.gen::<bool>() {//generate vowel
                    let idx = rng.gen_range(0, VOWELS.len());
                    VOWELS.chars().nth(idx).unwrap() as char
                }
                else { //generate consonant
                    let idx = rng.gen_range(0, CONSONANTS.len());
                    CONSONANTS.chars().nth(idx).unwrap() as char
                };
            data.push(c);
        }

        let phoneme_use: PhonemeUse = rng.gen();
        Phoneme{data, usage: phoneme_use}
    }
}

struct Language {
    general_phonemes: Vec<Phoneme>,
    city_phonemes: Vec<Phoneme>,
    people_phonemes: Vec<Phoneme>
}

impl Language {
    fn new(n_phonemes: usize, avg_length: usize) -> Language {
        let mut result = Language{general_phonemes: Vec::new(),
            city_phonemes: Vec::new(),
            people_phonemes: Vec::new()};

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

    fn gen_name() -> String { //only use general phonemes

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
