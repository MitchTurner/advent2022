use crate::rucksack_searcher::RucksackSearcher;
use crate::string_data::StringData;

pub mod error {
    pub type MyError = String;

    pub type Result<T, E = MyError> = std::result::Result<T, E>;
}

pub mod rucksack_searcher;
pub mod string_data;

const DATA: &str = include_str!("../../../data/day_3.txt");

fn main() {
    let string_data = StringData::new(DATA);
    let rucksack_searcher = RucksackSearcher::new(string_data);
    let sum_of_priorities = rucksack_searcher.get_sum_of_priorities().unwrap();

    println!("sum of priorities: {:?}", sum_of_priorities);

    let sum_of_groups = rucksack_searcher.get_sum_of_group_priorities().unwrap();
    println!("sum of group priorities: {:?}", sum_of_groups);
}
