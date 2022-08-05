use serde::{Serialize, Deserialize};
use std::ops::{Add, Mul};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    WHITE,
    BLACK
}

pub fn change_color(color: &Color) -> Color {
    match color {
        Color::WHITE => Color::BLACK,
        Color::BLACK => Color::WHITE
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Coord(isize, isize);

pub fn from_str_to_coord(s: &str) -> Option<Coord> {
    if s.len()!=2 {return None;}
    let s = s.as_bytes();
    Some(Coord((s[0]- 'a' as u8) as isize,
    ((s[1] as char).to_digit(10).unwrap() -1) as isize))
}

impl Coord {
    pub fn is_correct(&self) -> bool {
        self.0 >= 0 && self.0 < 8 && self.1 >= 0 && self.1 < 8
    }

    pub fn from_str(s: &str) -> Option<Coord> {
        if s.len()!=2 {return None;}
        let s = s.as_bytes();
        Some(Coord((s[0]- 'a' as u8) as isize,
        ((s[1] as char).to_digit(10).unwrap() -1) as isize))
    }

    pub fn get_char_column(&self) -> u8 {
        (('a' as isize) + self.1) as u8
    }

    pub fn get_char_line(&self) -> u8 {
        (('1' as isize) + self.0) as u8
    }

    pub fn get_str(&self) -> String {
        let mut res = String::new();
        res.push(self.get_char_column() as char);
        res.push(self.get_char_line() as char);
        res
    }
}

impl From<(i32, i32)> for Coord {
    fn from(input: (i32, i32)) -> Coord {
        Coord(input.0 as isize, input.1 as isize)
    }
}

impl Add for Coord {
    type Output = Coord; 

    fn add(self, other: Coord) -> Coord {
        Coord(self.0+other.0, self.1+other.1)
    }
}

impl Mul<isize> for Coord {
    type Output = Coord;

    fn mul(self, other: isize) -> Coord {
        Coord(self.0*other, self.1*other)
    }
}

pub const BOARD_SIZE : usize = 8;
pub const WHITE: Color = Color::WHITE;
pub const BLACK: Color = Color::BLACK;

#[derive(Debug)]
pub enum Result {
    White, 
    Black,
    Draw
}

mod board;
pub use board::Board;

mod piece;
pub use piece::{Piece, PieceType::*, PieceWrapper};

mod game;
pub use game::Game;

mod chess_move;
pub use chess_move::ChessMove;

mod opening_tree;
pub use opening_tree::OpeningTree;