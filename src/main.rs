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

fn main() {
    let lang = lang::Language::new(30, 3.0);
    let world = world::World::new(10, 10, 20, lang);

    println!("{}", world.to_string());

    for _ in 0..10 {
        let q = quest::create_quest(&world, 3);

        println!("{}", q.to_string());
    }
}
