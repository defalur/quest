#[derive(Clone, Debug)]
pub enum QDataType {
    Actor,
    Container,
    Mob,
    Location,
    Item
}

#[derive(Clone, Debug)]
pub struct QData{
    q_type: QDataType,
    name: String,
    quantity: usize//used for some types of data
}

impl QData {
    pub fn new(q_type: QDataType, name: &str) -> QData {
        QData{q_type, name: name.to_string(), quantity: 0}
    }

    pub fn new_actor(name: &str) -> QData {
        QData::new(QDataType::Actor, name)
    }

    pub fn new_container(name: &str) -> QData {
        QData::new(QDataType::Container, name)
    }

    pub fn new_mob(name: &str) -> QData {
        QData::new(QDataType::Mob, name)
    }

    pub fn new_location(name: &str) -> QData {
        QData::new(QDataType::Location, name)
    }

    pub fn new_item(name: &str) -> QData {
        QData::new(QDataType::Item, name)
    }

    pub fn with_quantity(mut self, quantity: usize) -> QData {
        self.quantity = quantity;
        self
    }
}

impl ToString for QData {
    fn to_string(&self) -> String {
        if self.quantity > 0 {
            self.quantity.to_string() + " " + &self.name.clone()
        }
        else {
            self.name.clone()
        }
    }
}
