use crate::quest_data;

pub enum GoalType {
    FETCH,//fetch an item
    DELIVER,//deliver an item to an npc
    KILL//kill n mobs of a certain type
}

impl ToString for GoalType {
    fn to_string(&self) -> String {
        match self {
            GoalType::FETCH => "Fetch".to_string(),
            GoalType::DELIVER => "Deliver".to_string(),
            GoalType::KILL => "Kill".to_string()
        }
    }
}

/*
 * Goal: interface
 * g_type()
 * to_string()
 */

pub trait Goal: ToString {
    fn get_type(&self) -> GoalType;
}

pub struct FetchGoal {
    item: quest_data::QData,
    owner: quest_data::QData//container can be a mob, npc or chest
}

impl ToString for FetchGoal {
    fn to_string(&self) -> String {
        "Fetch ".to_string() + &self.item.to_string() + " from " + &self.owner.to_string()
    }
}

impl FetchGoal {
    pub fn new(item: quest_data::QData, owner: quest_data::QData) -> Box<dyn Goal> {
        Box::new(FetchGoal{item, owner})
    }

}

impl Goal for FetchGoal {
    fn get_type(&self) -> GoalType {
        GoalType::FETCH
    }
}

pub struct KillGoal {
    mob: quest_data::QData,
}

impl ToString for KillGoal {
    fn to_string(&self) -> String {
        "Kill ".to_string() + &self.mob.to_string()
    }
}

impl KillGoal {
    pub fn new(mob: quest_data::QData) -> Box<dyn Goal> {
        Box::new(KillGoal{mob})
    }
}

impl Goal for KillGoal {
    fn get_type(&self) -> GoalType {
        GoalType::KILL
    }
}

pub struct DeliverGoal {
    item: quest_data::QData,
    target: quest_data::QData
}

impl ToString for DeliverGoal {
    fn to_string(&self) -> String {
        "Deliver ".to_string() + &self.item.to_string() + " to " + &self.target.to_string()
    }
}

impl DeliverGoal {
    pub fn new(item: quest_data::QData, target: quest_data::QData) -> Box<dyn Goal> {
        Box::new(DeliverGoal{item, target})
    }
}

impl Goal for DeliverGoal {
    fn get_type(&self) -> GoalType {
        GoalType::DELIVER
    }
}
