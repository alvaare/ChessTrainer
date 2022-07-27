mod board;
mod piece;

use crate::board::Board;

fn main() {
    let board = Board::default();
    println!("{:?}", board);
}
