enum GoalType {
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

struct FetchGoal {
    target: String,
    quantity: usize
}

impl ToString for FetchGoal {
    fn to_string(&self) -> String {
        (self.target.clone() + ": " + &self.quantity.to_string() + "\n")
    }
}

struct DeliverGoal {
    target: String,
    quantity: usize
}

impl ToString for DeliverGoal {
    fn to_string(&self) -> String {
        (self.target.clone() + ": " + &self.quantity.to_string() + "\n")
    }
}

struct KillGoal {
    target: String,
    quantity: usize
}

impl ToString for KillGoal {
    fn to_string(&self) -> String {
        (self.target.clone() + ": " + &self.quantity.to_string() + "\n")
    }
}

enum GoalData {
    Fetch(FetchGoal),
    Deliver(DeliverGoal),
    Kill(KillGoal)
}

impl ToString for GoalData {
    fn to_string(&self) -> String {
       match self {
            GoalData::Fetch(data) => data.to_string(),
            GoalData::Deliver(data) => data.to_string(),
            GoalData::Kill(data) => data.to_string()
       }
    }
}

struct Goal {
    g_type: GoalType,
    data: GoalData
}

impl ToString for Goal {
    fn to_string(&self) -> String {
        self.g_type.to_string() + ":\n" + &self.data.to_string()
    }
}

struct Quest {
    goal: Goal
}

impl ToString for Quest {
    fn to_string(&self) -> String {
        self.goal.to_string()
    }
}

fn main() {
   let g_type = GoalType::FETCH;
   let goalData = GoalData::Fetch(FetchGoal{target: "Rusty Sword".to_string(), quantity: 1});
   let goal = Goal{g_type, data: goalData};
   let quest = Quest{goal};

   println!("{}", quest.to_string());
}
