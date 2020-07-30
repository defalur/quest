use rand::Rng;
use crate::quest::goal;
use crate::quest;

use crate::quest_data;
use crate::world;
use crate::item;
use crate::mob;
use crate::character;

pub struct GoalFactory {
    goal_type: goal::GoalType,
    target_actor: quest_data::QData,
    target_object: quest_data::QData,//mob or item
    container: quest_data::QData,
    location: quest_data::QData
}

impl GoalFactory {
    pub fn new(goal_type: goal::GoalType) -> GoalFactory {
        GoalFactory{goal_type, target_actor: quest_data::QData::new_actor("NULL_ACTOR"),
        target_object: quest_data::QData::new_item("NULL_ITEM"),
        container: quest_data::QData::new_container("NULL_CONTAINER"),
        location: quest_data::QData::new_location("NULL_LOCATION")}
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

    pub fn with_location(mut self, name: &str) -> GoalFactory {
        self.location = quest_data::QData::new_location(name);

        self
    }

    pub fn build(&self) -> Box<dyn goal::Goal> {
        match self.goal_type {
            goal::GoalType::FETCH => goal::FetchGoal::new(self.target_object.clone(),
                                                            self.container.clone(),
                                                            self.location.clone()),
            goal::GoalType::DELIVER => goal::DeliverGoal::new(self.target_object.clone(),
                                                                self.target_actor.clone(),
                                                                self.location.clone()),
            goal::GoalType::KILL => goal::KillGoal::new(self.target_object.clone(),
                                                            self.location.clone())
        }
    }
}

fn rand_fetch_goal(world: &world::World, rng: &mut rand::rngs::ThreadRng) -> Box<dyn goal::Goal> {
    let mut builder = GoalFactory::new(goal::GoalType::FETCH)
        .with_item(&item::Item::new_rand().to_string(), rng.gen_range(1, 11))
        .with_location(&world.rand_location(rng.gen()).unwrap().to_string());

    let p: f32 = rng.gen();
    if p > 0.5 {//container is a chest
        builder.with_container("Chest")
    }
    else {
        builder.with_container(&world.rand_mob(rng.gen()).unwrap().to_string())
    }.build()
}

fn rand_deliver_goal(world: &world::World, rng: &mut rand::rngs::ThreadRng) -> Box<dyn goal::Goal> {
    GoalFactory::new(goal::GoalType::DELIVER)
        .with_item(&item::Item::new_rand().to_string(), rng.gen_range(1, 11))
        .with_target(&world.rand_character(rng.gen()).unwrap().to_string())
        .with_location(&world.rand_location(rng.gen()).unwrap().to_string())
        .build()
}

fn rand_kill_goal(world: &world::World, rng: &mut rand::rngs::ThreadRng) -> Box<dyn goal::Goal> {
    GoalFactory::new(goal::GoalType::KILL)
        .with_mob(&world.rand_mob(rng.gen()).unwrap().to_string(), rng.gen_range(1, 11))
        .with_location(&world.rand_location(rng.gen()).unwrap().to_string())
        .build()
}

pub fn rand_goal(world: &world::World) -> Box<dyn goal::Goal> {
//select the type of goal, then generate the corresponding type of goal and return the result
    let mut rng = rand::thread_rng();
    let goal_type: goal::GoalType = rng.gen();

    match goal_type {
        goal::GoalType::FETCH => rand_fetch_goal(&world, &mut rng),
        goal::GoalType::DELIVER => rand_deliver_goal(&world, &mut rng),
        goal::GoalType::KILL => rand_kill_goal(&world, &mut rng)
    }
}
