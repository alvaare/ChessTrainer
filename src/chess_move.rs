use crate::{*, piece::PieceType};
use serde::{Serialize, Deserialize};
use std::str;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChessMove {
    pub piece: Piece,
    pub start: Coord,
    pub end: Coord,
    pub is_king_castling: bool,
    pub is_queen_castling: bool,
    pub is_capture: bool,
    pub is_check: bool,
    pub is_mate: bool,
    pub promotion: Option<Piece>
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
        let mut chess_move = ChessMove::default();
        if &notation[..3] == b"O-O" {
            chess_move.piece = Piece{piece_type: KING, color: turn};
            chess_move.is_king_castling = true;
        }
        if &notation[..5] == b"O-O-O" {
            chess_move.piece = Piece{piece_type: KING, color: turn};
            chess_move.is_queen_castling = true;
        }
        let piece_type = match PieceType::from_char(&notation[0]) {
            Some(p) => p,
            _ => PAWN
        };
        let piece = Piece {piece_type, color: turn};
        todo!()
    }
}

impl Default for ChessMove {
    fn default() -> ChessMove {
        ChessMove {
            piece : Piece { piece_type: PAWN, color: WHITE },
            start : Coord(0,0),
            end : Coord(0,0),
            is_king_castling : false,
            is_queen_castling : false,
            is_capture: false,
            is_check: false,
            is_mate: false,
            promotion: None
        }
    }
}