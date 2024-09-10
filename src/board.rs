use crate::piece::{get_king_moves, get_pawn_moves, get_standard_valid_move, ColouredPiece, Piece};
use crate::position::{BoardPosition, Horizontal::*, Vertical::*};

#[derive(Clone, Copy)]
pub enum Turn {
    White,
    Black,
}

pub enum GameState {
    Ongoing,
    Check,
    CheckMate,
}

#[derive(Default, Clone, Copy)]
pub enum MoveType {
    #[default]
    Invalid,
    Move,
    Capture,
}

#[derive(Clone)]
pub struct Board<T> {
    board: [[Option<T>; 8]; 8],
}

impl<T> Board<T> {
    pub fn get_index(&self, position: &BoardPosition) -> &Option<T> {
        let (file, rank) = position.into();

        &self.board[usize::from(file)][usize::from(rank)]
    }

    pub fn set_index(&mut self, position: &BoardPosition, value: T) {
        let (file, rank) = position.into();

        self.board[usize::from(file)][usize::from(rank)] = Some(value);
    }
}

impl<T> Default for Board<T> 
where T: Copy {
    fn default() -> Board<T> {
        Board {
            board: [[None; 8]; 8],
        }
    }
}

pub struct ChessGame {
    board: Board<ColouredPiece<Piece>>,
    turn: Turn,
    state: GameState,
}

impl Default for ChessGame {
    fn default() -> Self {
        let mut board = Board::default();

        board.set_index(
            &BoardPosition::from((A, One)),
            ColouredPiece::White(Piece::Rook),
        );
        board.set_index(
            &BoardPosition::from((H, One)),
            ColouredPiece::White(Piece::Rook),
        );

        board.set_index(
            &BoardPosition::from((B, One)),
            ColouredPiece::White(Piece::Knight),
        );
        board.set_index(
            &BoardPosition::from((G, One)),
            ColouredPiece::White(Piece::Knight),
        );

        board.set_index(
            &BoardPosition::from((C, One)),
            ColouredPiece::White(Piece::Bishop),
        );
        board.set_index(
            &BoardPosition::from((F, One)),
            ColouredPiece::White(Piece::Bishop),
        );

        board.set_index(
            &BoardPosition::from((D, One)),
            ColouredPiece::White(Piece::Queen),
        );
        board.set_index(
            &BoardPosition::from((E, One)),
            ColouredPiece::White(Piece::new_king()),
        );

        for file in 0..8 {
            board.set_index(
                &BoardPosition::try_from((file, 1)).unwrap(),
                ColouredPiece::White(Piece::new_white_pawn()),
            );
        }

        board.set_index(
            &BoardPosition::from((A, Eight)),
            ColouredPiece::Black(Piece::Rook),
        );
        board.set_index(
            &BoardPosition::from((H, Eight)),
            ColouredPiece::Black(Piece::Rook),
        );

        board.set_index(
            &BoardPosition::from((B, Eight)),
            ColouredPiece::Black(Piece::Knight),
        );
        board.set_index(
            &BoardPosition::from((G, Eight)),
            ColouredPiece::Black(Piece::Knight),
        );

        board.set_index(
            &BoardPosition::from((C, Eight)),
            ColouredPiece::Black(Piece::Bishop),
        );
        board.set_index(
            &BoardPosition::from((F, Eight)),
            ColouredPiece::Black(Piece::Bishop),
        );

        board.set_index( &BoardPosition::from((D, Eight)), ColouredPiece::Black(Piece::Queen),); board.set_index( &BoardPosition::from((E, Eight)), ColouredPiece::Black(Piece::new_king()),);
        for file in 0..8 {
            board.set_index(
                &BoardPosition::try_from((file, 6)).unwrap(),
                ColouredPiece::Black(Piece::new_black_pawn()),
            );
        }

        Self {
            board,
            turn: Turn::White,
            state: GameState::Ongoing,
        }
    }
}

impl ChessGame {
    // Does not yet take check into consideration
    pub fn get_valid_moves(self, position: BoardPosition) -> Option<Board<MoveType>> {
        let piece = match self.board.get_index(&position) {
            Some(piece) => piece,
            None => return None
        };

        let piece_type = match piece {
            ColouredPiece::White(piece) => piece,
            ColouredPiece::Black(piece) => piece,
        };

        Some(match piece_type {
            Piece::King { castling_state, .. } => {
                get_king_moves(&self.board, &position, piece, castling_state)
            }
            Piece::Pawn { .. } => get_pawn_moves(&self.board, &position, piece),
            _ => get_standard_valid_move(&self.board, piece, &position)
        })
    }

    pub fn move_piece(
        initial_position: BoardPosition,
        desired_position: BoardPosition,
    ) -> Result<GameState, String> {
        todo!()
    }

    pub fn get_player_turn(self) -> Turn {
        self.turn
    }
}
