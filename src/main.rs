#[macro_use]
extern crate lazy_static;

mod quest;
mod quest_data;
mod lang;
mod character;
mod location;
mod item;
mod mob;
mod world;

use crate::quest::goal_factory;

fn main() {
    let blacksmith = quest_data::QData::new_actor("Blacksmith");
    let bob = "Bob";
    let rusty_sword = quest_data::QData::new_item("Rusty Sword");

    let delivery = goal_factory::GoalFactory::new(quest::goal::GoalType::DELIVER)
        .with_item("Rusty Sword", 0)
        .with_target("blacksmith");

    let mut quest = quest::Quest::new(bob);
    quest.add_goal(delivery.build());
    let lang = lang::Language::new(30, 3.0);
    let world = world::World::new(10, 10, 20, lang);

    println!("{}", world.to_string());

    for _ in 0..10 {
        let q = quest::create_quest(&world, 3);

        println!("{}", q.to_string());
    }
}
