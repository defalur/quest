#[macro_use]
extern crate lazy_static;

mod quest;
mod quest_data;
mod lang;
mod character;
mod location;
mod item;

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

    println!("{}", quest.to_string());

    let l = lang::Language::new(30, 3.0);
    //for _ in 0..100 {
    //    let name = l.gen_person_name();

    //    println!("{}", name);
    //}

    for _ in 0..100 {
        let c = character::Character::new(&l);
        println!("{}", c.to_string());
    }

    println!("====================");

    for _ in 0..100 {
        let c = location::Location::new(&l);
        println!("{}", c.to_string());
    }

    println!("====================");

    for _ in 0..100 {
        let c = item::Item::new_rand();
        println!("{}", c.to_string());
    }
}
