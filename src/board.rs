use std::usize;

use crate::piece::{self, ColouredPiece, KingState, PawnState, Piece};
use crate::position::{board_position_to_coordinate, BoardPosition};

pub struct ChessGame {
    board: [[ColouredPiece; 8]; 8],
    turn: Turn,
    state: GameState,
}

impl Default for ChessGame {
    fn default() -> Self {
        let mut board = [[ColouredPiece::None; 8]; 8];

        board[0][0] = ColouredPiece::Black(Piece::new_rook());
        board[0][7] = ColouredPiece::Black(Piece::new_rook());

        board[0][1] = ColouredPiece::Black(Piece::new_knight());
        board[0][6] = ColouredPiece::Black(Piece::new_knight());

        board[0][2] = ColouredPiece::Black(Piece::new_bishop());
        board[0][5] = ColouredPiece::Black(Piece::new_bishop());

        board[0][3] = ColouredPiece::Black(Piece::new_queen());
        board[0][4] = ColouredPiece::Black(Piece::new_king());

        for square in &mut board[1][0..8] {
            *square = ColouredPiece::Black(Piece::new_black_pawn());
        }

        board[7][0] = ColouredPiece::Black(Piece::new_rook());
        board[7][7] = ColouredPiece::Black(Piece::new_rook());

        board[7][1] = ColouredPiece::Black(Piece::new_knight());
        board[7][6] = ColouredPiece::Black(Piece::new_knight());

        board[7][2] = ColouredPiece::Black(Piece::new_bishop());
        board[7][5] = ColouredPiece::Black(Piece::new_bishop());

        board[7][3] = ColouredPiece::Black(Piece::new_queen());
        board[7][4] = ColouredPiece::Black(Piece::new_king());

        for square in &mut board[6][0..8] {
            *square = ColouredPiece::White(Piece::new_white_pawn());
        }

        Self {
            board,
            turn: Turn::White,
            state: GameState::Ongoing,
        }
    }
}

pub enum Turn {
    White,
    Black,
}

pub enum GameState {
    Ongoing,
    Check,
    CheckMate,
}

impl ChessGame {
    pub fn get_valid_moves(self, position: BoardPosition) -> Option<Vec<BoardPosition>> {
        todo!()
    }
}

pub fn move_piece(initial_position: BoardPosition, desired_position: BoardPosition) -> Result<GameState, String> {
    todo!()
}

pub fn get_player_turn() -> Turn {
    todo!()
}
