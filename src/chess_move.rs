use crate::{*, piece::PieceType};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChessMove {
    pub piece: Piece,
    pub start: Coord,
    pub end: Coord
}

impl ChessMove {
    // pub fn new(piece: Piece, start: &str, end: &str) -> Self {
    //     ChessMove {
    //         piece,
    //         start: from_str_to_coord(start).unwrap(), 
    //         end: from_str_to_coord(end).unwrap()
    //     }
    // }

    pub fn notation(&self) -> Vec<u8> {
        let mut res = vec![];
        if let Some(c) = self.piece.get_notation() {
            res.push(c);
        }
        res.extend(self.end.get_str());
        res
    }

    pub fn from_notation(notation: &[u8], turn: Color) -> Self {
        let piece_type = match PieceType::from_char(&notation[0]) {
            Some(p) => p,
            _ => PAWN
        };
        let piece = Piece {piece_type, color: turn};
        todo!()
    }
}