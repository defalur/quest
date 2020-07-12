use crate::item;
use crate::actor;
use crate::container;

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
 * actors: list of actors, with type of actor
 */

pub trait Goal: ToString {
    fn get_type(&self) -> GoalType;
    fn actors(&self) -> Vec<Box<dyn actor::Actor>>;
}

pub struct FetchGoal {
    pub item: item::Item,
    pub owner: Box<dyn container::Container>//container can be a mob, npc or chest
}

impl ToString for FetchGoal {
    fn to_string(&self) -> String {
        "Fetch".to_string() + &self.item.to_string() + " from " + &self.owner.to_string()
    }
}

impl FetchGoal {
    fn get_type(&self) -> GoalType {
        GoalType::FETCH
    }

    //fn actors(&self) -> Vec<Box<actor::Actor>> {
    //    vec![self.owner]
    //}
}
