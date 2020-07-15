pub mod actor;
//wil get other attributes
pub trait Actor: ToString {
    fn get_name(&self) -> String;
}
