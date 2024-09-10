mod board;
mod piece;
mod position;

#[derive(Debug)]
pub enum ChessError {
    OutOfBounds
}
