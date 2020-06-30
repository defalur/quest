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
    fn actors(&self) -> std::Vec<std::Box<Actor>>;
}

pub struct FetchGoal {
    pub item: Item,
    pub owner: std::Box<Container>//container can be a mob, npc or chest
}

impl ToString for FetchGoal {
    fn to_string(&self) -> String {
        "Fetch".to_string() + item.to_string() + " from " + owner.to_string()
    }
}

impl FetchGoal {
    fn get_type(&self) -> GoalType {
        GoalType::FETCH
    }

    fn actors(&self) -> std::Vec<std::Box<Actor>> {
        vec![owner]
    }
}
