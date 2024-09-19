mod prelude;
mod test;
mod board;
mod piece;
pub mod position;

#[derive(Debug)]
pub enum ChessError {
    OutOfBounds,
    NoPiece,
    NoMoves,
    InvalidMove,
    SelfCheck,
    IncorrectFenString,
    InternalError,
}

pub use board::{ChessGame, Board, };

