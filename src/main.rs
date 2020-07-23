mod quest;
mod quest_data;
mod lang;

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

    let l = lang::Language::new(15, 3.0);
    for _ in 0..100 {
        let name = l.gen_person_name();

        println!("{}", name);
    }
}
