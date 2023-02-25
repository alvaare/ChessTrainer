use crate::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PieceType {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN
}

pub type PieceWrapper = (PieceType,Color);

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]

pub struct Piece {
    pub piece_type : PieceType,
    pub color: Color
}

struct Pawn;
struct Knight;
struct Bishop;
struct Rook;
struct Queen;
struct King;

impl PieceType {
    pub fn from_uppercase(c: &u8) -> Option<Self> {
        match c {
            b'P' => Some(PieceType::PAWN),
            b'N' => Some(PieceType::KNIGHT),
            b'B' => Some(PieceType::BISHOP),
            b'R' => Some(PieceType::ROOK),
            b'Q' => Some(PieceType::QUEEN),
            b'K' => Some(PieceType::KING),
            _ => None
        }
    }
    pub fn from_char(c: &u8) -> Option<Self> {
        let up_c = (*c as char).to_ascii_uppercase() as u8;
        PieceType::from_uppercase(&up_c)
    }

    pub fn get_char(&self) -> u8 {
        match self {
            PAWN => 'P' as u8,
            KNIGHT => 'N' as u8,
            BISHOP => 'B' as u8,
            ROOK => 'R' as u8,
            QUEEN => 'Q' as u8,
            KING => 'K' as u8
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

    pub fn get_notation(&self) -> Option<u8> {
        match self.piece_type.get_char() {
            b'P' => None,
            c => Some(c)
        }
    }

    pub fn to_char(&self) -> u8 {
        let c = self.piece_type.get_char();
        match self.color {
            BLACK => c.to_ascii_lowercase(),
            WHITE => c
        }
    }

    pub fn get_unicode(&self) -> char {
        match (self.piece_type, self.color) {
            (PAWN, WHITE) => '\u{2659}',
            (PAWN, BLACK) => '\u{265F}',
            (KNIGHT, WHITE) => '\u{2658}',
            (KNIGHT, BLACK) => '\u{265E}',
            (BISHOP, WHITE) => '\u{2657}',
            (BISHOP, BLACK) => '\u{265D}',
            (ROOK, WHITE) => '\u{2656}',
            (ROOK, BLACK) => '\u{265C}',
            (QUEEN, WHITE) => '\u{2655}',
            (QUEEN, BLACK) => '\u{265B}',
            (KING, WHITE) => '\u{2654}',
            (KING, BLACK) => '\u{265A}',

        }
    }
}

pub trait CanMove {
    fn available_moves(board: &Board, coord: &Coord, color: &Color) -> Vec<ChessMove>; 
}

const PAWN_CAPTURE_DIRS: [Coord; 2] = [Coord(0,1), Coord(0,-1)];

// impl CanMove for Pawn {
//     fn available_moves(board: &Board, coord: &Coord, color: &Color) -> Vec<ChessMove> {
//         let mut moves = Vec::<ChessMove>::new();
//         let dir: Coord = match color {
//             Color::WHITE => Coord(1, 0),
//             Color::BLACK => Coord(-1, 0)
//         };
//         let new_coord = *coord+dir;
//         if board.is_square_free(&new_coord) {
//             moves.push(ChessMove {piece: Piece{piece_type: PAWN, color: *color}, start: *coord, end: new_coord});
//             if *color==WHITE && coord.0==1 || *color==BLACK && coord.0==7 {
//                 let new_coord = *coord+dir*2;
//                 if board.is_square_free(&new_coord) {
//                     moves.push(ChessMove {piece: Piece{piece_type: PAWN, color: *color}, start: *coord, end: new_coord});
//                 }
//             }
//         }
//         for cap_dir in PAWN_CAPTURE_DIRS {
//             let new_coord = *coord+dir+cap_dir;
//             if board.can_capture(&new_coord, &color) {
//                 moves.push(ChessMove {piece: Piece{piece_type: PAWN, color: *color}, start: *coord, end: new_coord});
//             }
//             if Some(new_coord) == board.en_passant {
//                 moves.push(ChessMove {piece: Piece{piece_type: PAWN, color: *color}, start: *coord, end: new_coord});
//             }
//         }
//         moves
//     }
// }

// const KNIGHT_DIRS: [Coord; 8] = [Coord(1,2), Coord(1,-2), Coord(2,1), Coord(2,-1), Coord(-1,2), Coord(-1,-2), Coord(-2,1), Coord(-2,-1)];

// impl CanMove for Knight {
    
//     fn available_moves(board: &Board, coord: &Coord, color: &Color) -> Vec<ChessMove> {
//         let mut moves = Vec::<ChessMove>::new();
//         for dir in KNIGHT_DIRS {
//             let new_coord = *coord+dir;
//             if board.is_square_free(&new_coord) || board.can_capture(&new_coord, color) {
//                 moves.push(ChessMove {piece: Piece{piece_type: KNIGHT, color: *color}, start: *coord, end: new_coord});
//             }
//         }
//         moves
//     }
// }

// const BISHOP_DIRS: [Coord; 4] = [Coord(1,1), Coord(1,-1), Coord(-1, 1), Coord(-1, -1)];

// impl CanMove for Bishop {

//     fn available_moves(board: &Board, coord: &Coord, color: &Color) -> Vec<ChessMove> {
//         let mut moves = Vec::<ChessMove>::new();
//         for dir in BISHOP_DIRS {
//             for nb_squares in 1..8 {
//                 let new_coord = *coord+dir*nb_squares;
//                 if board.is_square_free(&new_coord) || board.can_capture(&new_coord, color) {
//                     moves.push(ChessMove {piece: Piece{piece_type: BISHOP, color: *color}, start: *coord, end: new_coord});
//                 }
//                 if !board.is_square_free(&new_coord) {
//                     break;
//                 }
//             }
//         }
//         moves
//     }
// }

// const ROOK_DIRS: [Coord; 4] = [Coord(0,1), Coord(0,-1), Coord(-1, 0), Coord(1, 0)];

// impl CanMove for Rook {
//     fn available_moves(board: &Board, coord: &Coord, color: &Color) -> Vec<ChessMove> {
//         let mut moves = Vec::<ChessMove>::new();
//         for dir in ROOK_DIRS {
//             for nb_squares in 1..8 {
//                 let new_coord = *coord+dir*nb_squares;
//                 if board.is_square_free(&new_coord) || board.can_capture(&new_coord, color) {
//                     moves.push(ChessMove {piece: Piece{piece_type: ROOK, color: *color}, start: *coord, end: new_coord});
//                 }
//                 if !board.is_square_free(&new_coord) {
//                     break;
//                 }
//             }
//         }
//         moves
//     }
// }

// const QUEEN_DIRS: [Coord; 8] = [Coord(0,1), Coord(0,-1), Coord(-1, 0), Coord(1, 0), Coord(1,1), Coord(1,-1), Coord(-1, 1), Coord(-1, -1)];

// impl CanMove for Queen {
//     fn available_moves(board: &Board, coord: &Coord, color: &Color) -> Vec<ChessMove> {
//         let mut moves = Vec::<ChessMove>::new();
//         for dir in QUEEN_DIRS {
//             for nb_squares in 1..8 {
//                 let new_coord = *coord+dir*nb_squares;
//                 if board.is_square_free(&new_coord) || board.can_capture(&new_coord, color) {
//                     moves.push(ChessMove {piece: Piece{piece_type: QUEEN, color: *color}, start: *coord, end: new_coord});
//                 }
//                 if !board.is_square_free(&new_coord) {
//                     break;
//                 }
//             }
//         }
//         moves
//     }
// }

// impl CanMove for King {
//     fn available_moves(board: &Board, coord: &Coord, color: &Color) -> Vec<ChessMove> {
//         let mut moves = Vec::<ChessMove>::new();
//         for dir in QUEEN_DIRS {
//             let new_coord = *coord+dir;
//             if board.is_square_free(&new_coord) || board.can_capture(&new_coord, color) {
//                 moves.push(ChessMove {piece: Piece{piece_type: KING, color: *color}, start: *coord, end: new_coord});
//             }
//         }
//         moves
//     }
// }

// impl Piece {

//     pub fn available_moves(&self, board: &Board, coord: &Coord) -> Vec<ChessMove> {
//         match self.piece_type {
//             PAWN => Pawn::available_moves(board, coord, &self.color),
//             KNIGHT => Knight::available_moves(board, coord, &self.color),
//             BISHOP => Bishop::available_moves(board, coord, &self.color),
//             ROOK => Rook::available_moves(board, coord, &self.color),
//             QUEEN => Queen::available_moves(board, coord, &self.color),
//             KING => King::available_moves(board, coord, &self.color)
//         }
//     }
// }
