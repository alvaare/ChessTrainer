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

    fn read_rank(&mut self, fen: &[u8], id_rank: usize, id_char: &mut usize) {
        let mut id_col : usize = 0;
        loop {
            println!("{}", fen[*id_char] as char);
            match fen[*id_char] {
                b'/' | b' ' => return,
                b'1'..=b'8' => id_col += ((fen[*id_char] as char).to_digit(10).unwrap() -1) as usize,
                c => self.set((id_rank, id_col), Piece::from_char(&c))
            }
            id_col += 1;
            *id_char += 1;
        }
    }

    pub fn from_fen(fen: &[u8]) -> Self {
        let mut board = Board::new();
        let mut id_char : usize = 0;
        for id_rank in 1..=8 {
            board.read_rank(&fen, 8-id_rank, &mut id_char);
            id_char += 1;
        }
        board
    }
}