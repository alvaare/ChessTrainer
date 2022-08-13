use crate::*;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
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

    // pub fn from_pgn(pgn: &str) -> Self {
    //     //println!("{}", pgn);
    //     let mut game = Game::new();
    //     game
    // }

    pub fn random() -> Self {
        let mut game = Game::new();
        let mut board = Board::default();
        loop {
            let available_moves = board.available_moves();
            let chess_move = available_moves.choose(&mut thread_rng());
            if chess_move.is_none() {break;}
            let chess_move = chess_move.unwrap();
            game.moves.push(*chess_move);
            board.do_move(&chess_move);
        }
        game.result = Some(Result::Draw);
        game
    }
}