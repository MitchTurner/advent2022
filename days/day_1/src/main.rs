use crate::file_reader::FileReader;
use crate::inventory_counter::InventoryCounter;

pub mod inventory_counter;
pub mod error;
pub mod file_reader;

fn main() {
    let file_reader = FileReader::new("data/day_1.txt".to_string());
    let inventory_counter = InventoryCounter::new(file_reader);

    let top = inventory_counter.highest_elf_calories_held().unwrap();

    println!("top: {}", top);

    let top_three = inventory_counter.top_three_elf_calories_held().unwrap();
    println!("top three: {}", top_three);
}
