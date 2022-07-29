use crate::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Piece {
    KING(Color),
    QUEEN(Color),
    ROOK(Color),
    BISHOP(Color),
    KNIGHT(Color),
    PAWN(Color)
}

impl Piece {
    pub fn from_char(c : &u8) -> Option<Self> {
        let color = if (*c as char).is_uppercase() {WHITE} else {BLACK};
        let up_c = (*c as char).to_ascii_uppercase();
        match up_c {
            'P' => Some(Piece::PAWN(color)),
            'N' => Some(Piece::KNIGHT(color)),
            'B' => Some(Piece::BISHOP(color)),
            'R' => Some(Piece::ROOK(color)),
            'Q' => Some(Piece::QUEEN(color)),
            'K' => Some(Piece::KING(color)),
            _ => None
        }
    } 
}