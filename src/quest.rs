pub mod goal;
pub mod goal_factory;

use crate::quest_data;

pub struct Quest {
    goals: Vec<Box<dyn goal::Goal>>,
    owner: quest_data::QData
}

impl ToString for Quest {
    fn to_string(&self) -> String {
        let mut result = "===== Quest =====\n".to_string();
        for g in &self.goals {
            result += &g.to_string()
        }
        result += &("\n for ".to_string() + &self.owner.to_string() + "\n");
        result
    }
}

impl Quest {
    pub fn new(owner_name: &str) -> Quest {
        Quest{goals: Vec::new(), owner: quest_data::QData::new_actor(owner_name)}
    }

    pub fn add_goal(&mut self, goal: Box<dyn goal::Goal>) {
        self.goals.push(goal);
    }
}

