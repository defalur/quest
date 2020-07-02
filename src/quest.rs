pub mod goal;

pub struct Quest {
    goal: Box<dyn goal::Goal>
}

impl ToString for Quest {
    fn to_string(&self) -> String {
        self.goal.to_string()
    }
}

impl Quest {
    pub fn new(goal: Box<dyn goal::Goal>) -> Quest {
        Quest{goal}
    }
}

