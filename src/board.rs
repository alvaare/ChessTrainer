use crate::piece::{*};
use crate::piece::Piece::*;
use crate::piece::Color::*;

type Coord = (usize, usize);
const BOARD_SIZE : usize = 8;

#[derive(Debug)]
pub struct Board {
    board: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE]
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [[None; BOARD_SIZE]; BOARD_SIZE]
        }
    }

    fn set(&mut self, coord: Coord, piece: Option<Piece>) {
        self.board[coord.0][coord.1] = piece;
    }

    fn set_line(&mut self, id_line: usize, piece: Option<Piece>) {
        for id_column in 0..BOARD_SIZE {
            self.board[id_line][id_column] = piece;
        }
    }

    pub fn default() -> Self {
        let mut board = Board::new();
        board.set((0,0), Some(ROOK(WHITE)));
        board.set((0,1), Some(KNIGHT(WHITE)));
        board.set((0,2), Some(BISHOP(WHITE)));
        board.set((0,3), Some(QUEEN(WHITE)));
        board.set((0,4), Some(KING(WHITE)));
        board.set((0,5), Some(BISHOP(WHITE)));
        board.set((0,6), Some(KNIGHT(WHITE)));
        board.set((0,7), Some(ROOK(WHITE)));
        board.set((7,0), Some(ROOK(BLACK)));
        board.set((7,1), Some(KNIGHT(BLACK)));
        board.set((7,2), Some(BISHOP(BLACK)));
        board.set((7,3), Some(QUEEN(BLACK)));
        board.set((7,4), Some(KING(BLACK)));
        board.set((7,5), Some(BISHOP(BLACK)));
        board.set((7,6), Some(KNIGHT(BLACK)));
        board.set((7,7), Some(ROOK(BLACK)));
        board.set_line(1, Some(PAWN(WHITE)));
        board.set_line(6, Some(PAWN(BLACK)));
        board
    }
}