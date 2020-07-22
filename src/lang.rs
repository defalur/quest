use rand::Rng;

const CONSONANTS: &'static str = "bcdfghjklmnpqrstvwxz";
const VOWELS: &'static str = "aeiouy";

enum PhonemeUse {
    General,
    City,
    People
}

struct Phoneme {
    data: String,
    use: PhonemeUse
}

impl Phoneme {
    fn new(data: &str, use: PhonemeUse) -> Phoneme {
        Phoneme{data: data.to_string(), use}
    }

    fn new_rand() -> Phoneme {
        let mut rng = rand::thread_rng();

        let len: usize = (rng.gen() % (4 - 1)) + 1;
        let data = String::new();
        for _ in 0..len {
            let c =
                if rng.gen::<bool>() {//generate vowel
                    let idx = rng.gen_range(0, VOWELS.len());
                    VOWELS[idx] as char
                }
                else { //generate consonant
                    let idx = rng.gen_range(0, CONSONANTS.len());
                    CONSONANTS[idx] as char
                };
            data.push(c);
        }


        Phoneme{data, }
    }
}

struct Language {
    
}
