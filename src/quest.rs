pub mod goal;
pub mod goal_factory;

use rand::{Rng};
use crate::quest_data;
use crate::world;

pub struct Quest {
    goals: Vec<Box<dyn goal::Goal>>,
    owner: quest_data::QData
}

impl ToString for Quest {
    fn to_string(&self) -> String {
        let mut result = "===== Quest =====\n".to_string();
        for g in &self.goals {
            result += "- ";
            result += &g.to_string();
            result += "\n";
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

pub fn create_quest(world: &world::World, max_n_goals: usize) -> Quest {
    let mut rng = rand::thread_rng();

    let owner = world.rand_character(rng.gen()).unwrap();

    let n_goals = rng.gen_range(1, max_n_goals);

    let mut result = Quest::new(&owner.to_string());

    for _ in 0..n_goals {
        let goal = goal_factory::rand_goal(world);
        result.add_goal(goal);
    }

    result
}
