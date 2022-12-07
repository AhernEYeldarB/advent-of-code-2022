pub fn jobs() -> &'static [(fn(), &'static str)] {
  &[
      (day1_calorie_counting::main, "day1_calorie_counting"),
      (day2_rock_paper_scissors::main, "day2_rock_paper_scissors"),
      (day3_rucksack_reorganization::main, "day3_rucksack_reorganization"),
      (day4_camp_cleanup::main, "day4_camp_cleanup"),
      (day5_supply_stacks::main, "day5_supply_stacks"),
  ]
}