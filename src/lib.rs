mod test;
pub mod board;
pub mod piece;
pub mod position;

#[derive(Debug)]
pub enum ChessError {
    OutOfBounds,
    NoPiece,
    NoMoves,
    InvalidMove,
    SelfCheck,
}
