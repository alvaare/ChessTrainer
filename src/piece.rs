use crate::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PieceType {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN
}

pub type PieceWrapper = (PieceType,Color);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]

pub struct Piece {
    pub piece_type : PieceType,
    pub color: Color
}

struct Pawn {
    color: Color
}
struct Knight {
    color: Color
}
struct Bishop {
    color: Color
}
struct Rook {
    color: Color
}
struct Queen {
    color: Color
}
struct King {
    color: Color
}

impl PieceType {
    fn from_char(c: &u8) -> Option<Self> {
        let up_c = (*c as char).to_ascii_uppercase();
        match up_c {
            'P' => Some(PieceType::PAWN),
            'N' => Some(PieceType::KNIGHT),
            'B' => Some(PieceType::BISHOP),
            'R' => Some(PieceType::ROOK),
            'Q' => Some(PieceType::QUEEN),
            'K' => Some(PieceType::KING),
            _ => None
        }
    }
}

impl Piece {
    pub fn from_char(c : &u8) -> Option<Self> {
        let color = if (*c as char).is_uppercase() {WHITE} else {BLACK};
        let piece_type = PieceType::from_char(c);
        match piece_type {
            Some(p) => Some(Piece {piece_type: p, color}),
            None => None
        }
    } 
}

pub trait CanMove {
    fn available_moves(board: &Board, coord: &Coord, color: &Color) -> Vec<ChessMove>; 
}

impl CanMove for Pawn {
    fn available_moves(board: &Board, coord: &Coord, color: &Color) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::new();
        let dir: Coord = match color {
            Color::WHITE => Coord(1, 0),
            Color::BLACK => Coord(-1, 0)
        };
        let new_coord = *coord+dir;
        if board.is_square_free(&new_coord) {
            moves.push(ChessMove {piece: Piece{piece_type: PAWN, color: *color}, start: *coord, end: new_coord});
            if *color==WHITE && coord.0==1 || *color==BLACK && coord.0==7 {
                let new_coord = *coord+dir*2;
                if board.is_square_free(&new_coord) {
                    moves.push(ChessMove {piece: Piece{piece_type: PAWN, color: *color}, start: *coord, end: new_coord});
                }
            }
        }
        //capture
        //enpassant
        moves
    }
}

const KNIGHT_DIRS: [Coord; 8] = [Coord(1,2), Coord(1,-2), Coord(2,1), Coord(2,-1), Coord(-1,2), Coord(-1,-2), Coord(-2,1), Coord(-2,-1)];

impl CanMove for Knight {
    
    fn available_moves(board: &Board, coord: &Coord, color: &Color) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::new();
        for dir in KNIGHT_DIRS {
            let new_coord = *coord+dir;
            if board.is_square_free(&new_coord) || board.can_capture(coord, color) {
                moves.push(ChessMove {piece: Piece{piece_type: KNIGHT, color: *color}, start: *coord, end: new_coord});
            }
        }
        moves
    }
}

// impl CanMove for Bishop {

// }

// impl CanMove for Rook {

// }

// impl CanMove for Queen {

// }

// impl CanMove for King {

// }

impl Piece {

    pub fn available_moves(&self, board: &Board, coord: &Coord) -> Vec<ChessMove> {
        match self.piece_type {
            PAWN => Pawn::available_moves(board, coord, &self.color),
            KNIGHT => Knight::available_moves(board, coord, &self.color),
            _ => Vec::new()
        }
    }
}
