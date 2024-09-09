use std::ops::Range;

use crate::piece::{ColouredPiece, Piece, StepCount};
use crate::position::{board_position_to_coordinate, BoardPosition};

pub struct ChessGame {
    board: [[ColouredPiece; 8]; 8],
    turn: Turn,
    state: GameState,
}

impl Default for ChessGame {
    fn default() -> Self {
        let mut board = [[ColouredPiece::None; 8]; 8];

        board[0][0] = ColouredPiece::Black(Piece::Rook);
        board[0][7] = ColouredPiece::Black(Piece::Rook);

        board[0][1] = ColouredPiece::Black(Piece::Knight);
        board[0][6] = ColouredPiece::Black(Piece::Knight);

        board[0][2] = ColouredPiece::Black(Piece::Bishop);
        board[0][5] = ColouredPiece::Black(Piece::Bishop);

        board[0][3] = ColouredPiece::Black(Piece::Queen);
        board[0][4] = ColouredPiece::Black(Piece::new_king());

        for square in &mut board[1][0..8] {
            *square = ColouredPiece::Black(Piece::new_black_pawn());
        }

        board[7][0] = ColouredPiece::Black(Piece::Rook);
        board[7][7] = ColouredPiece::Black(Piece::Rook);

        board[7][1] = ColouredPiece::Black(Piece::Knight);
        board[7][6] = ColouredPiece::Black(Piece::Knight);

        board[7][2] = ColouredPiece::Black(Piece::Bishop);
        board[7][5] = ColouredPiece::Black(Piece::Bishop);

        board[7][3] = ColouredPiece::Black(Piece::Queen);
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

#[derive(Clone, Copy)]
pub enum MoveType {
    Invalid,
    Move,
    Capture,
}

impl ChessGame {
    pub fn get_valid_moves(self, position: BoardPosition) -> Option<[[MoveType; 8]; 8]> {
        let (file, rank) = board_position_to_coordinate(position);

        let current_board_position = self.board[usize::from(file)][usize::from(rank)];

        let (piece, kind) = match current_board_position {
            ColouredPiece::White(piece) => Some((piece, Turn::White)),
            ColouredPiece::Black(piece) => Some((piece, Turn::Black)),
            ColouredPiece::None => None,
        }?;
        let base_vectors = current_board_position.get_movement_base_vector()?;
        let move_amount = current_board_position.get_number_of_moves()?;

        match piece {
            _ => Some(self.get_standard_valid_move(
                (file, rank),
                base_vectors,
                move_amount,
                kind
            )),
        }
    }

    fn get_standard_valid_move(
        &self,
        position: (u8, u8),
        base_vector: Vec<(i8, i8)>,
        move_count: StepCount,
        kind: Turn,
    ) -> [[MoveType; 8]; 8] {
        let mut move_map = [[MoveType::Invalid; 8]; 8];

        let move_amount = match move_count {
            StepCount::One => 1..2,
            StepCount::Two => 1..3,
            StepCount::Infinty => 1..8,
        };

        let moves = base_vector
            .into_iter()
            .map(|vector| {
                self.compute_move_in_one_direction(
                    position,
                    kind,
                    vector,
                    move_amount.clone(),
                )
            })
            .flatten();

        for (new_file, new_rank) in moves {
            move_map[new_file][new_rank] = match self.board[new_file][new_rank] {
                ColouredPiece::None => MoveType::Move,
                ColouredPiece::White(_) => {
                    if matches!(kind, Turn::White) {
                        MoveType::Invalid
                    } else {
                        MoveType::Capture
                    }
                }
                ColouredPiece::Black(_) => {
                    if matches!(kind, Turn::Black) {
                        MoveType::Invalid
                    } else {
                        MoveType::Capture
                    }
                }
            }
        }
        move_map
    }

    fn compute_move_in_one_direction(
        &self,
        position: (u8, u8),
        kind: Turn,
        vector: (i8, i8),
        move_amount: Range<i8>,
    ) -> Vec<(usize, usize)> {
        let (file, rank) = vector;
        let mut moves = Vec::new();

        for current_amount in move_amount {
            let new_file = position.0 as i8 + file * current_amount;

            let new_file = if new_file >= 8 || new_file < 0 {
                break;
            } else {
                new_file as usize
            };

            let new_rank = position.1 as i8 + rank * current_amount;

            let new_rank = if new_rank >= 8 || new_rank < 0 {
                break;
            } else {
                new_rank as usize
            };

            moves.push(match self.board[new_file][new_rank] {
                ColouredPiece::None => (new_file, new_rank),
                ColouredPiece::White(_) => {
                    if matches!(kind, Turn::White) {
                        break;
                    } else {
                        (new_file, new_rank)
                    }
                }
                ColouredPiece::Black(_) => {
                    if matches!(kind, Turn::Black) {
                        break;
                    } else {
                        (new_file, new_rank)
                    }
                }
            })
        }
        moves
    }
}

pub fn move_piece(
    initial_position: BoardPosition,
    desired_position: BoardPosition,
) -> Result<GameState, String> {
    todo!()
}

impl ChessGame {
    pub fn get_player_turn(self) -> Turn {
        self.turn
    }
}
