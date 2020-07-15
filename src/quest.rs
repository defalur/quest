pub mod quest;

use crate::actor;

pub struct Quest {
    goal: Box<dyn goal::Goal>
    owner: Box<dyn actor::Actor>
}

impl ToString for Quest {
    fn to_string(&self) -> String {
        self.goal.to_string() + " for " + self.owner.to_string()
    }
}

impl Quest {
    pub fn new(goal: Box<dyn goal::Goal>) -> Quest {
        Quest{goal}
    }
}

