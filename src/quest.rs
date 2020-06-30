pub mod goal;

pub struct Quest {
    goal: std::Box<goal::Goal>
}

impl ToString for Quest {
    fn to_string(&self) -> String {
        self.goal.to_string()
    }
}

impl Quest {
    pub fn new(goal: goal::Goal) -> Quest {
        Quest{goal}
    }
}

