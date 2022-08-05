use crate::*;

#[derive(Debug)]
pub struct Board {
    position: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
    turn: Color,
    castlings: [bool; 4],
    pub en_passant: Option<Coord>,
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
        if coord.is_correct() {
            self.position[coord.0 as usize][coord.1 as usize] = piece;
        }
    }

    fn set_piece(&mut self, coord: Coord, piece: PieceWrapper) {
        if coord.is_correct() {
            self.position[coord.0 as usize][coord.1 as usize] = Some(Piece{piece_type: piece.0, color: piece.1});
        }
    }

    fn set_line(&mut self, id_line: usize, piece: PieceWrapper) {
        for id_column in 0..BOARD_SIZE {
            self.position[id_line][id_column] = Some(Piece{piece_type: piece.0, color: piece.1});
        }
    }

    pub fn default() -> Self {
        let mut board = Board::new();
        board.set_piece(Coord(0,0), (ROOK,WHITE));
        board.set_piece(Coord(0,1), (KNIGHT,WHITE));
        board.set_piece(Coord(0,2), (BISHOP,WHITE));
        board.set_piece(Coord(0,3), (QUEEN,WHITE));
        board.set_piece(Coord(0,4), (KING,WHITE));
        board.set_piece(Coord(0,5), (BISHOP,WHITE));
        board.set_piece(Coord(0,6), (KNIGHT,WHITE));
        board.set_piece(Coord(0,7), (ROOK,WHITE));
        board.set_piece(Coord(7,0), (ROOK,BLACK));
        board.set_piece(Coord(7,1), (KNIGHT,BLACK));
        board.set_piece(Coord(7,2), (BISHOP,BLACK));
        board.set_piece(Coord(7,3), (QUEEN,BLACK));
        board.set_piece(Coord(7,4), (KING,BLACK));
        board.set_piece(Coord(7,5), (BISHOP,BLACK));
        board.set_piece(Coord(7,6), (KNIGHT,BLACK));
        board.set_piece(Coord(7,7), (ROOK,BLACK));
        board.set_line(1, (PAWN,WHITE));
        board.set_line(6, (PAWN,BLACK));
        board
    }

    fn read_rank(&mut self, rank: &str, id_rank: isize) {
        let mut id_col: isize = 0;
        for c in rank.bytes() {
            match c {
                b'/' | b' ' => return,
                b'1'..=b'8' => id_col += ((c as char).to_digit(10).unwrap() -1) as isize,
                c => self.set(Coord(id_rank, id_col), Piece::from_char(&c))
            }
            id_col += 1;
        }
    }

    fn read_ranks(&mut self, fen: &str) {
        let ranks = fen.split('/');
        for (id_rank, rank) in ranks.enumerate() {
            self.read_rank(rank, (7-id_rank) as isize);
        }
    }

    fn read_turn(&mut self, fen: &str) {
        self.turn = match fen {
            "b" => BLACK,
            "w" => WHITE,
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

    pub fn available_moves(&self) -> Vec<ChessMove> {
        let mut av_moves = Vec::<ChessMove>::new();
        for (i, line) in self.position.iter().enumerate() {
            for (j, piece) in line.iter().enumerate() {
                if let Some(p) = piece {
                    if p.color == self.turn {
                        av_moves.append(&mut p.available_moves(&self, &Coord(i as isize,j as isize)));
                    }
                }
            }
        }
        av_moves
    }

    pub fn is_square_free(&self, coord: &Coord) -> bool {
        if !coord.is_correct() {return false}
        self.position[coord.0 as usize][coord.1 as usize].is_none()
    }

    pub fn color_is(&self, coord: &Coord, color: &Color) -> bool {
        if !coord.is_correct() {return false}
        match self.position[coord.0 as usize][coord.1 as usize] {
            Some(p) => p.color == *color,
            _ => false
        }
    }

    pub fn can_capture(&self, coord: &Coord, color: &Color) -> bool {
        if !coord.is_correct() {return false}
        match self.position[coord.0 as usize][coord.1 as usize] {
            Some(p) => p.color != *color,
            _ => false
        }
    }
}