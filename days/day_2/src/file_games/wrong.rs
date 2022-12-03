use crate::{error, file_games};
use crate::error::error_to_string;
use crate::score_keeper::{Game, Games, Move};
use crate::score_keeper::Move::{Paper, Rock, Scissors};

pub struct WrongFileGames {
    file_path: String
}

impl WrongFileGames {
    pub fn new(file_path: String) -> Self {
        WrongFileGames { file_path }
    }
}

impl Games for WrongFileGames {
    fn get_games(&self) -> error::Result<Vec<Game>> {
         file_games::read_lines(&self.file_path)
            .map_err(error_to_string)?
            .map(|maybe_e| maybe_e.map_err(error_to_string))
            .collect::<error::Result<Vec<String>>>()?
             .iter()
             .map(|game_raw| wrong_game_from_string(&game_raw))
             .collect()
    }
}

fn wrong_game_from_string(game_raw: &str) -> error::Result<Game> {
    let mut moves_raw = game_raw.split_whitespace();
        let opp_raw = moves_raw.next().ok_or(format!("Game string malformed: {}", game_raw))?;
        let player_raw = moves_raw.next().ok_or(format!("Game string malformed: {}", game_raw))?;
        let opp = wrong_move_from_str(opp_raw)?;
        let player = wrong_move_from_str(player_raw)?;
        let game = (opp, player).into();
        Ok(game)
}

fn wrong_move_from_str(s: &str) -> error::Result<Move> {
    match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err(format!("Not a valid move: {}", s))
        }
}
