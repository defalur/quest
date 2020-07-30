use rand::{Rng, distributions::{Distribution, Standard, Normal}};

#[derive(Clone)]
struct ItemType {
    name: String,
    modable: bool,
    lootable: bool
}

impl ItemType {
    fn new(name: &str, modable: bool, lootable: bool) -> ItemType {
        ItemType{name: name.to_string(), modable, lootable}
    }
}

lazy_static! {
    static ref ITEM_TYPES: Vec<ItemType> = vec![
        ItemType::new("Sword", true, true),
        ItemType::new("Shield", true, true),
        ItemType::new("Health Potion", false, true),
        ItemType::new("Leather Boots", false, true),
        ItemType::new("Wizard Hat", false, true)
    ];

    static ref ITEM_MODS: Vec<&'static str> = vec![
        "Powerful",
        "Sharp",
        "Large",
        "Magic",
        "Rusty"
    ];
}

pub struct Item {
    item_type: ItemType,
    modifier: Option<&'static str>
}

impl Item {
    pub fn new_rand() -> Item {
        let mut rng = rand::thread_rng();

        let type_id = rng.gen_range(0, ITEM_TYPES.len());

        let item_type = ITEM_TYPES[type_id].clone();
        let p: f64 = rng.gen();
        let modifier = match item_type.modable {
            true if p > 0.7 => Some(ITEM_MODS[rng.gen_range(0, ITEM_MODS.len())]),
            _ => None
        };

        Item{item_type, modifier}
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        match self.modifier {
            Some(s) => s.to_string() + " " + &self.item_type.name,
            None => self.item_type.name.clone()
        }
    }
}
