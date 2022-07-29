use crate::*;
pub struct Game {
    moves: Vec<ChessMove>,
    result: Option<Result>
}

impl Game {
    fn new() -> Self {
        Game {
            moves: Vec::<ChessMove>::new(),
            result: None
        }
    }

    pub fn from_pgn(pgn: &str) -> Self {
        //println!("{}", pgn);
        let mut game = Game::new();
        game
    }
}