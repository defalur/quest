use crate::actor;

pub struct NPC {
    name: String
}

impl NPC {
    fn new(name: &str) -> NPC {
        NPC{name: name.to_string()}
    }
}

impl Actor for NPC {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
