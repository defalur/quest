use crate::item;

struct ItemSlot {
    item: item::Item,
    quantity: u32
}

//use a map
pub struct Container {
    items: Vec<ItemSlot>
}

impl Container {
    pub fn new() -> Container {
        Container{items: Vec::new()}
    }
}

impl ToString for Container {
    fn to_string(&self) -> String {
        "Container".to_string()
    }
}
