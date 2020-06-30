mod quest;

fn main() {
   let g_type = quest::goal::GoalType::FETCH;
   let goalData = quest::goal::GoalData{target: "Rusty Sword".to_string(), quantity: 1};
   let goal = quest::goal::Goal{g_type, data: goalData};
   let quest = quest::Quest::new(goal);

   println!("{}", quest.to_string());
}
