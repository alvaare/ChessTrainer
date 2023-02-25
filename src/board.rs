use crate::*;
use std::fmt;
use std::str;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

    fn read_rank(&mut self, rank: &[u8], id_rank: isize) {
        let mut id_col: isize = 0;
        for c in rank {
            match c {
                b'/' | b' ' => return,
                b'1'..=b'8' => id_col += (c-b'0') as isize,
                c => self.set(Coord(id_rank, id_col), Piece::from_char(&c))
            }
            id_col += 1;
        }
    }

    fn read_ranks(&mut self, fen: &[u8]) {
        let ranks = fen.split(|c| *c == b'/');
        for (id_rank, rank) in ranks.enumerate() {
            self.read_rank(rank, (7-id_rank) as isize);
        }
    }

    fn read_turn(&mut self, fen: &[u8]) {
        self.turn = match fen {
            [b'b'] => BLACK,
            [b'w'] => WHITE,
            _ => panic!("The turn must be b or w")
        }
    }

    fn read_castlings(&mut self, fen: &[u8]) {
        self.castlings = [false; 4];
        for c in fen {
            match c {
                b'K' => self.castlings[0] = true,
                b'Q' => self.castlings[1] = true,
                b'k' => self.castlings[2] = true,
                b'q' => self.castlings[3] = true,
                _ => ()
            }
        }
    }

    fn read_en_passant(&mut self, fen: &[u8]) {
        self.en_passant = match fen {
            [b'-'] => None,
            s => Coord::from_str(s)
        }
    }

    fn read_clock(&mut self, fen: &[u8]) {
        self.halfmove_clock = match str::from_utf8(fen) {
            Ok(v) => v.parse().unwrap(),
            _  => unreachable!()  
        }
    }

    fn read_move(&mut self, fen: &[u8]) {
        self.move_count = match str::from_utf8(fen) {
            Ok(v) => v.parse().unwrap(),
            _  => unreachable!()  
        }
    }

    pub fn from_fen(fen: &FEN) -> Self {
        let mut board = Board::new();
        let vec: Vec<&[u8]> = fen.split(|c| c.is_ascii_whitespace()).collect();
        board.read_ranks(vec[0]);
        board.read_turn(vec[1]);
        board.read_castlings(vec[2]);
        board.read_en_passant(vec[3]);
        board.read_clock(vec[4]);
        board.read_move(vec[5]);
        board
    }

    // pub fn available_moves(&self) -> Vec<ChessMove> {
    //     let mut av_moves = Vec::<ChessMove>::new();
    //     for (i, line) in self.position.iter().enumerate() {
    //         for (j, piece) in line.iter().enumerate() {
    //             if let Some(p) = piece {
    //                 if p.color == self.turn {
    //                     av_moves.append(&mut p.available_moves(&self, &Coord(i as isize,j as isize)));
    //                 }
    //             }
    //         }
    //     }
    //     av_moves
    // }

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

    pub fn do_move(&mut self, chess_move: &ChessMove) {
        self.set(chess_move.start, None);
        let move_is_capture = !self.is_square_free(&chess_move.end);
        self.set(chess_move.end, Some(chess_move.piece));
        self.turn = change_color(&self.turn);
        self.halfmove_clock += 1;
        if self.turn == Color::WHITE {
            self.move_count += 1;
        }
        if chess_move.piece.piece_type == PAWN || move_is_capture {
            self.halfmove_clock = 0;
        }
    }

    fn write_rank(&self, id_rank: usize) -> Vec<u8> {
        let mut res = vec![];
        let mut acc = 0;
        for square in self.position[id_rank] {
            match square {
                Some(piece) =>  {if acc>0 {
                                            res.push(b'0' + acc);
                                       }
                                        res.push(piece.to_char()); 
                                        acc = 0;
                                    }
                None => acc += 1
            }
        }
        if acc>0 {
            res.push(b'0' + acc);
        }
        res
    }

    fn write_ranks(&self) -> Vec<u8> {
        let mut res = vec![];
        for i in 0..8 {
            res.append(&mut self.write_rank(7-i));
            if i != 7 {
                res.push(b'/');
            }
        }
        res
    }

    fn write_turn(&self) -> Vec<u8> {
        match self.turn {
            WHITE => vec![b'w'],
            BLACK => vec![b'b']
        }
    }

    fn write_castlings(&self) -> Vec<u8> {
        if self.castlings == [false; 4] {
            return vec![b'-'];
        }
        let mut res = vec![];
        if self.castlings[0] {res.push(b'K');}
        if self.castlings[1] {res.push(b'Q');}
        if self.castlings[2] {res.push(b'k');}
        if self.castlings[3] {res.push(b'q');}
        res
    }

    fn write_en_passant(&self) -> Vec<u8> {
        if let Some(sq) = self.en_passant {
            sq.get_str()
        }
        else {
            vec![b'-']
        }
    }

    pub fn to_fen(&self) -> FEN {
        let mut fen: FEN = FEN::new();
        const SEPARATOR: u8 = b' ';

        fen.extend(self.write_ranks());
        fen.push(SEPARATOR);

        fen.extend( self.write_turn());
        fen.push(SEPARATOR);

        fen.extend(self.write_castlings());
        fen.push(SEPARATOR);

        fen.extend(self.write_en_passant());
        fen.push(SEPARATOR);

        fen.extend(self.halfmove_clock.to_string().bytes());
        fen.push(SEPARATOR);

        fen.extend(self.move_count.to_string().bytes());
        fen

        
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.position {
            let mut disp = String::new();
            for square in line {
                if let Some(p) = square {
                    disp.push(p.get_unicode());
                }
                else {
                    disp.push('.');
                }
            }
            disp.push('\n');
            let res = write!(f, "{}", disp);
            if res.is_err() {return res}
        }   
        Ok(())
    }
}