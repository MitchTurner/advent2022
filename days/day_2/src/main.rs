use file_games::correct::CorrectFileGames;
use file_games::wrong::WrongFileGames;
use crate::score_keeper::ScoreKeeper;

pub mod score_keeper;
pub mod error;
pub mod file_games;

fn main() {
    let wrong_file_games = WrongFileGames::new("data/day_2.txt".to_string());
    let score_keeper = ScoreKeeper::new(wrong_file_games);

    let (_, player) = score_keeper.total_first().unwrap();
    println!("wrong total: {:?}", player);

    let correct_file_games = CorrectFileGames::new("data/day_2.txt".to_string());
    let score_keeper = ScoreKeeper::new(correct_file_games);

    let (_, player) = score_keeper.total_first().unwrap();
    println!("correct total: {:?}", player);
}


