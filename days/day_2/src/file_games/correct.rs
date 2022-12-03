use std::str::FromStr;
use crate::{
    score_keeper::{
        Game,
        Games,
        Move::{
            self,
            Paper,
            Rock,
            Scissors
        }
    },
    error::{error_to_string, MyError},
    error,
    file_games,
};

pub enum GameResult {
    Lose,
    Draw,
    Win,
}

impl FromStr for GameResult {
    type Err = MyError;

    fn from_str(s: &str) -> error::Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(format!("Not a valid move: {}", s))
        }
    }
}

pub struct CorrectFileGames {
    file_path: String
}

impl CorrectFileGames {
    pub fn new(file_path: String) -> Self {
        CorrectFileGames { file_path }
    }
}

impl Games for CorrectFileGames {
    fn get_games(&self) -> error::Result<Vec<Game>> {
        file_games::read_lines(&self.file_path)
            .map_err(error_to_string)?
            .map(|maybe_e| maybe_e.map_err(error_to_string))
            .collect::<error::Result<Vec<String>>>()?
            .iter()
            .map(|game_raw| correct_game_from_string(&game_raw))
            .collect()
    }
}

fn correct_game_from_string(game_raw: &str) -> error::Result<Game> {
    let mut moves_raw = game_raw.split_whitespace();
    let opp_raw = moves_raw.next().ok_or(format!("Game string malformed: {}", game_raw))?;
    let result_raw = moves_raw.next().ok_or(format!("Game string malformed: {}", game_raw))?;
    let opp = correct_move_from_str(opp_raw)?;
    let result = GameResult::from_str(result_raw)?;
    let player = match (opp, result) {
        (Rock, GameResult::Lose) => Scissors,
        (Rock, GameResult::Draw) => Rock,
        (Rock, GameResult::Win) => Paper,
        (Paper, GameResult::Lose) => Rock,
        (Paper, GameResult::Draw) => Paper,
        (Paper, GameResult::Win) => Scissors,
        (Scissors, GameResult::Lose) => Paper,
        (Scissors, GameResult::Draw) => Scissors,
        (Scissors, GameResult::Win) => Rock,
    };
    let game = (opp, player).into();
    Ok(game)
}

fn correct_move_from_str(s: &str) -> error::Result<Move> {
    match s {
        "A" => Ok(Rock),
        "B" => Ok(Paper),
        "C" => Ok(Scissors),
        _ => Err(format!("Not a valid move: {}", s))
    }
}


