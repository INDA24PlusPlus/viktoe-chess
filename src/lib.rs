mod board;
mod piece;
mod position;

use board::ChessGame;

pub fn init_state() -> ChessGame {
    ChessGame::default()
}

