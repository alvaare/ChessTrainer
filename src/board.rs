use crate::piece::{*};
use crate::piece::Piece::*;
use crate::piece::Color::*;

const BOARD_SIZE : usize = 8;
type Coord = (usize, usize);

pub fn from_str_to_coord(s: &str) -> Option<Coord> {
    if s.len()!=2 {return None;}
    let mut s = s.bytes();
    Some(((s.nth(0).unwrap() - 'a' as u8).into(),
    ((s.nth(1).unwrap() as char).to_digit(10).unwrap() -1) as usize))
}


#[derive(Debug)]
pub struct Board {
    position: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
    turn: Color,
    castlings: [bool; 4],
    en_passant: Option<Coord>,
    halfmove_clock: usize,
    move_count: usize
}

impl Board {
    pub fn new() -> Self {
        Board {
            position: [[None; BOARD_SIZE]; BOARD_SIZE],
            turn: WHITE,
            castlings: [true; 4],
            en_passant: None,
            halfmove_clock: 0,
            move_count: 0
        }
    }

    fn set(&mut self, coord: Coord, piece: Option<Piece>) {
        self.position[coord.0][coord.1] = piece;
    }

    fn set_line(&mut self, id_line: usize, piece: Option<Piece>) {
        for id_column in 0..BOARD_SIZE {
            self.position[id_line][id_column] = piece;
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

    fn read_rank(&mut self, rank: &str, id_rank: usize) {
        let mut id_col : usize = 0;
        for c in rank.bytes() {
            match c {
                b'/' | b' ' => return,
                b'1'..=b'8' => id_col += ((c as char).to_digit(10).unwrap() -1) as usize,
                c => self.set((id_rank, id_col), Piece::from_char(&c))
            }
            id_col += 1;
        }
    }

    fn read_ranks(&mut self, fen: &str) {
        let ranks = fen.split('/');
        for (id_rank, rank) in ranks.enumerate() {
            self.read_rank(rank, 7-id_rank);
        }
    }

    fn read_turn(&mut self, fen: &str) {
        self.turn = match fen {
            "b" => Color::BLACK,
            "w" => Color::WHITE,
            _ => panic!("The turn must be b or w")
        }
    }

    fn read_castlings(&mut self, fen: &str) {
        self.castlings = [false; 4];
        for c in fen.bytes() {
            match c {
                b'K' => self.castlings[0] = true,
                b'Q' => self.castlings[1] = true,
                b'k' => self.castlings[2] = true,
                b'q' => self.castlings[3] = true,
                _ => ()
            }
        }
    }

    fn read_en_passant(&mut self, fen: &str) {
        self.en_passant = match fen {
            "-" => None,
            s => from_str_to_coord(s)
        }
    }

    fn read_clock(&mut self, fen: &str) {
        self.halfmove_clock = fen.parse().unwrap();
    }

    fn read_move(&mut self, fen: &str) {
        self.move_count = fen.parse().unwrap();
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut board = Board::new();
        let vec: Vec<&str> = fen.split_whitespace().collect();
        board.read_ranks(vec[0]);
        board.read_turn(vec[1]);
        board.read_castlings(vec[2]);
        board.read_en_passant(vec[3]);
        board.read_clock(vec[4]);
        board.read_move(vec[5]);
        board
    }
}