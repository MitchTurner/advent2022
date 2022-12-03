use crate::error;

#[derive(Clone, Copy, Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Debug)]
pub struct Game {
    opp: Move,
    player: Move,
}

impl From<(Move, Move)> for Game {
    fn from((opp, player): (Move, Move)) -> Self {
        Game { opp, player }
    }
}

pub trait Games {
    fn get_games(&self) -> error::Result<Vec<Game>>;
}

pub struct ScoreKeeper<G: Games> {
    games: G,
}

impl<G: Games> ScoreKeeper<G> {
    pub fn new(choices: G) -> Self {
        ScoreKeeper { games: choices }
    }

    pub fn total_first(&self) -> error::Result<(u32, u32)> {
        let totals = self
            .games
            .get_games()?
            .iter()
            .map(score_game)
            .fold((0, 0), |(opp_total, player_total), (opp, player)| {
                (opp_total + opp, player_total + player)
            });
        Ok(totals)
    }
}

fn score_game(game: &Game) -> (u32, u32) {
    let Game { opp, player } = game;
    match (opp, player) {
        (Move::Rock, Move::Rock) => (4, 4),
        (Move::Rock, Move::Paper) => (1, 8),
        (Move::Rock, Move::Scissors) => (7, 3),
        (Move::Paper, Move::Rock) => (8, 1),
        (Move::Paper, Move::Paper) => (5, 5),
        (Move::Paper, Move::Scissors) => (2, 9),
        (Move::Scissors, Move::Rock) => (3, 7),
        (Move::Scissors, Move::Paper) => (9, 2),
        (Move::Scissors, Move::Scissors) => (6, 6),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        error,
        score_keeper::{Game, Games, ScoreKeeper},
        score_keeper::Move::{Paper, Rock, Scissors}
    };

    impl Games for Vec<Game> {
        fn get_games(&self) -> error::Result<Vec<Game>> {
            let games = self.to_owned();
            Ok(games)
        }
    }

    #[test]
    fn can_total_scores() {
        let games: Vec<Game> = vec![(Rock, Paper), (Paper, Rock), (Scissors, Scissors)]
            .into_iter()
            .map(Into::into)
            .collect();

        let score_keeper = ScoreKeeper::new(games);

        let expected = 15;

        let (_, actual) = score_keeper.total_first().unwrap();

        assert_eq!(expected, actual);
    }
}
