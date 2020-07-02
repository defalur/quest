//only a struct for now, might get stats and other things
pub struct Item
{
    name: String
}

impl Item {
    pub fn new(name: &str) -> Item {
        Item{name: name.to_string()}
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}
