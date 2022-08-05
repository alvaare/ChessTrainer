use crate::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ChessMove {
    pub piece: Piece,
    pub start: Coord,
    pub end: Coord
}

impl ChessMove {
    pub fn new(piece: Piece, start: &str, end: &str) -> Self {
        ChessMove {
            piece,
            start: from_str_to_coord(start).unwrap(), 
            end: from_str_to_coord(end).unwrap()
        }
    }

    pub fn notation(&self) -> String {
        let mut res = String::new();
        if let Some(c) = self.piece.get_notation() {
            res.push(c as char);
        }
        res.push_str(&self.end.get_str());
        res
    }
}