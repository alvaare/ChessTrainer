use serde::{Serialize, Deserialize};
#

[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Color {
    WHITE,
    BLACK
}

type Coord = (usize, usize);

pub fn from_str_to_coord(s: &str) -> Option<Coord> {
    if s.len()!=2 {return None;}
    let mut s = s.as_bytes();
    Some(((s[0]- 'a' as u8).into(),
    ((s[1] as char).to_digit(10).unwrap() -1) as usize))
}

pub const BOARD_SIZE : usize = 8;
pub const WHITE: Color = Color::WHITE;
pub const BLACK: Color = Color::BLACK;

pub enum Result {
    White, 
    Black,
    Draw
}

mod board;
pub use board::Board;

mod piece;
pub use piece::{Piece, Piece::*};

mod game;
pub use game::Game;

mod chess_move;
pub use chess_move::ChessMove;

mod opening_tree;
pub use opening_tree::OpeningTree;