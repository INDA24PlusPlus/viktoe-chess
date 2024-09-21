pub mod prelude;
mod test;
pub mod board;
pub mod piece;
pub mod position;

#[derive(Debug)]
pub enum ChessError {
    OutOfBounds,
    NoPiece,
    NotYourPiece,
    NoMoves,
    InvalidMove,
    SelfCheck,
    IncorrectFenString,
    InternalError,
}
