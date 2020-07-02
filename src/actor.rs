//wil get other attributes
pub struct Actor {
    name: String
}

impl Actor {
    pub fn new(name: &str) -> Actor {
        Actor{name: name.to_string()}
    }
}
