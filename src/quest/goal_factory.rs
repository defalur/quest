use crate::quest::goal;
use crate::quest;

use crate::item;
use crate::actor;
use crate::container;
use crate::mob;

pub struct GoalFactory {
    goal_type: goal::GoalType,
    target_actor: Box<dyn actor::Actor>,
    item: item::Item,
    quantity: usize,
    mob: mob::Mob,
    container: Box<dyn container::Container>
}

impl GoalFactory {
    fn new(goal_type: goal::GoalType) -> GoalFactory {
        GoalFactory{goal_type, actors: Vec::new()}
    }

    fn with_mob(&self, mob: mob::Mob, count: usize) -> &GoalFactory {
        self.mob = mob;
        self.quantity = count;

        self
    }

    fn with_item(&self. item: item::Item, count: usize) -> &GoalFactory {
        self.item = item;
        self.quantity = count;

        self
    }

    fn with_container(&self, container: Box<dyn container::Container>) -> &GoalFactory {
        self.container = container;

        self
    }

    fn with_target(&self, actor: Box<dyn actor::Actor>) -> &GoalFactory {
        self.target_actor = actor;

        self
    }

    fn build(&self) -> Box<dyn goal::Goal> {
        match self.goal_type {
            goal::GoalType::FETCH => Box(goal::FetchGoal{item: self.item, owner: self.container}),
            goal::GoalType::DELIVER => Box(goal::FetchGoal{item: self.item, owner: self.container}),
            goal::GoalType::KILL => Box(goal::FetchGoal{item: self.item, owner: self.container})
        }
    }
}
