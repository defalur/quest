use crate::item;

pub struct ItemSlot {
    item: item::Item,
    quantity: u32
}

//use a map
pub trait Container: ToString {
    fn get_items(&self) -> Vec<ItemSlot>;
    fn add_item(&self, item: item::Item);
    fn get_item(&self, index: usize) -> item::Item;
}
