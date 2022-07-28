#[derive(Clone, Copy, Debug)]
pub enum Color {
    WHITE,
    BLACK
}

type Coord = (usize, usize);

pub const BOARD_SIZE : usize = 8;
pub const WHITE: Color = Color::WHITE;
pub const BLACK: Color = Color::BLACK;

mod board;
pub use board::Board;

mod piece;
pub use piece::Piece;
