#[derive(Clone, Copy, Debug)]
pub enum Color {
    WHITE,
    BLACK
}

#[derive(Clone, Copy, Debug)]
pub enum Piece {
    KING(Color),
    QUEEN(Color),
    ROOK(Color),
    BISHOP(Color),
    KNIGHT(Color),
    PAWN(Color)
}