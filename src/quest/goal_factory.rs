use crate::quest::goal;
use crate::quest;

use crate::quest_data;

pub struct GoalFactory {
    goal_type: goal::GoalType,
    target_actor: quest_data::QData,
    target_object: quest_data::QData,//mob or item
    container: quest_data::QData
}

impl GoalFactory {
    pub fn new(goal_type: goal::GoalType) -> GoalFactory {
        GoalFactory{goal_type, target_actor: quest_data::QData::new_actor("NULL_ACTOR"),
        target_object: quest_data::QData::new_item("NULL_ITEM"),
        container: quest_data::QData::new_container("NULL_CONTAINER")}
    }

    pub fn with_mob(mut self, name: &str, count: usize) -> GoalFactory {
        self.target_object = quest_data::QData::new_mob(name).with_quantity(count);

        self
    }

    pub fn with_item(mut self, name: &str, count: usize) -> GoalFactory {
        self.target_object = quest_data::QData::new_item(name).with_quantity(count);

        self
    }

    pub fn with_container(mut self, name: &str) -> GoalFactory {
        self.container = quest_data::QData::new_container(name);

        self
    }

    pub fn with_target(mut self, name: &str) -> GoalFactory {
        self.target_actor = quest_data::QData::new_actor(name);

        self
    }

    pub fn build(&self) -> Box<dyn goal::Goal> {
        match self.goal_type {
            goal::GoalType::FETCH => goal::FetchGoal::new(self.target_object.clone(),
                                                            self.container.clone()),
            goal::GoalType::DELIVER => goal::DeliverGoal::new(self.target_object.clone(),
                                                                self.target_actor.clone()),
            goal::GoalType::KILL => goal::KillGoal::new(self.target_object.clone())
        }
    }
}
