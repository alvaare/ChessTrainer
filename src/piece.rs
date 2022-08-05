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

const PAWN_CAPTURE_DIRS: [Coord; 2] = [Coord(0,1), Coord(0,-1)];

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
        for cap_dir in PAWN_CAPTURE_DIRS {
            let new_coord = *coord+dir+cap_dir;
            if board.can_capture(&new_coord, &color) {
                moves.push(ChessMove {piece: Piece{piece_type: PAWN, color: *color}, start: *coord, end: new_coord});
            }
            if Some(new_coord) == board.en_passant {
                moves.push(ChessMove {piece: Piece{piece_type: PAWN, color: *color}, start: *coord, end: new_coord});
            }
        }
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

const BISHOP_DIRS: [Coord; 4] = [Coord(1,1), Coord(1,-1), Coord(-1, 1), Coord(-1, -1)];

impl CanMove for Bishop {

    fn available_moves(board: &Board, coord: &Coord, color: &Color) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::new();
        for dir in BISHOP_DIRS {
            for nb_squares in 1..8 {
                let new_coord = *coord+dir*nb_squares;
                if board.is_square_free(&new_coord) || board.can_capture(coord, color) {
                    moves.push(ChessMove {piece: Piece{piece_type: BISHOP, color: *color}, start: *coord, end: new_coord});
                }
                if !board.is_square_free(&new_coord) {
                    break;
                }
            }
        }
        moves
    }
}

const ROOK_DIRS: [Coord; 4] = [Coord(0,1), Coord(0,-1), Coord(-1, 0), Coord(1, 0)];

impl CanMove for Rook {
    fn available_moves(board: &Board, coord: &Coord, color: &Color) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::new();
        for dir in ROOK_DIRS {
            for nb_squares in 1..8 {
                let new_coord = *coord+dir*nb_squares;
                if board.is_square_free(&new_coord) || board.can_capture(coord, color) {
                    moves.push(ChessMove {piece: Piece{piece_type: ROOK, color: *color}, start: *coord, end: new_coord});
                }
                if !board.is_square_free(&new_coord) {
                    break;
                }
            }
        }
        moves
    }
}

const QUEEN_DIRS: [Coord; 8] = [Coord(0,1), Coord(0,-1), Coord(-1, 0), Coord(1, 0), Coord(1,1), Coord(1,-1), Coord(-1, 1), Coord(-1, -1)];

impl CanMove for Queen {
    fn available_moves(board: &Board, coord: &Coord, color: &Color) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::new();
        for dir in QUEEN_DIRS {
            for nb_squares in 1..8 {
                let new_coord = *coord+dir*nb_squares;
                if board.is_square_free(&new_coord) || board.can_capture(coord, color) {
                    moves.push(ChessMove {piece: Piece{piece_type: QUEEN, color: *color}, start: *coord, end: new_coord});
                }
                if !board.is_square_free(&new_coord) {
                    break;
                }
            }
        }
        moves
    }
}

impl CanMove for King {
    fn available_moves(board: &Board, coord: &Coord, color: &Color) -> Vec<ChessMove> {
        let mut moves = Vec::<ChessMove>::new();
        for dir in BISHOP_DIRS {
            let new_coord = *coord+dir;
            if board.is_square_free(&new_coord) || board.can_capture(coord, color) {
                moves.push(ChessMove {piece: Piece{piece_type: BISHOP, color: *color}, start: *coord, end: new_coord});
            }
        }
        moves
    }
}

impl Piece {

    pub fn available_moves(&self, board: &Board, coord: &Coord) -> Vec<ChessMove> {
        match self.piece_type {
            PAWN => Pawn::available_moves(board, coord, &self.color),
            KNIGHT => Knight::available_moves(board, coord, &self.color),
            _ => Vec::new()
        }
    }
}
