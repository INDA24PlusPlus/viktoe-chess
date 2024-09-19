mod king_moves;
mod pawn_moves;
pub mod shorthands;
mod trait_implementations;

use std::ops::Range;

use crate::board::{is_in_check, Board, MoveType, Turn};
use crate::position::BoardPosition;
use shorthands::*;

pub use king_moves::get_king_moves;
pub use pawn_moves::get_pawn_moves;

type CastlingState = (bool, bool);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Color<T> {
    White(T),
    Black(T),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Piece {
    King {
        check_state: Option<CheckState>,
        castling_state: CastlingState,
    },
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn {
        state: PawnState,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PawnState {
    FirstMove,
    PosibleEnPassant,
    Default,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckState {
    CheckMate,
    Check,
}

#[derive(Clone)]
pub enum StepCount {
    One,
    Two,
    Infinty,
}

impl Color<Piece> {
    pub fn get_movement_base_vector(&self) -> Vec<(i8, i8)> {
        match self {
            Color::White(Piece::Pawn { .. }) => WHITE_PAWN_MOVES.to_vec(),
            Color::Black(Piece::Pawn { .. }) => BLACK_PAWN_MOVES.to_vec(),
            Color::Black(piece) | Color::White(piece) => match piece {
                Piece::King { .. } => KING_MOVES.to_vec(),
                Piece::Queen => QUEEN_MOVES.to_vec(),
                Piece::Bishop => BISHOP_MOVES.to_vec(),
                Piece::Knight => BISHOP_MOVES.to_vec(),
                Piece::Rook => ROOK_MOVES.to_vec(),
                _ => Vec::new(),
            },
        }
    }

    pub fn get_number_of_moves(&self) -> StepCount {
        match self {
            Color::White(piece) | Color::Black(piece) => match piece {
                Piece::King { .. } => StepCount::One,
                Piece::Queen => StepCount::Infinty,
                Piece::Rook => StepCount::Infinty,
                Piece::Bishop => StepCount::Infinty,
                Piece::Knight => StepCount::One,
                Piece::Pawn {
                    state: PawnState::FirstMove,
                } => StepCount::Two,
                Piece::Pawn { .. } => StepCount::One,
            },
        }
    }
}

impl<T> Color<T> {
    pub fn get_internal(&self) -> &T {
        match self {
            Color::White(value) => value,
            Color::Black(value) => value,
        }
    }

    pub(crate) fn change_internal(&mut self, value: T) {
        *self = match self {
            Color::White(_) => Color::White(value),
            Color::Black(_) => Color::Black(value),
        }
    }

    pub(crate) fn same_color(&self, player_color: &Turn) -> bool {
        match self {
            Color::White(_) if matches!(player_color, Turn::White) => true,
            Color::Black(_) if matches!(player_color, Turn::Black) => true,
            _ => false,
        }
    }
}

impl Color<Piece> {
    pub(crate) fn get_standard_valid_move(
        &self,
        board: &Board<Color<Piece>>,
        position: &BoardPosition,
        king_position: &BoardPosition,
        player_color: &Turn,
    ) -> Board<MoveType> {
        let mut move_map = Board::default();

        for base_vector in self.get_movement_base_vector() {
            let number_of_steps = self.get_number_of_moves().into();

            // Get all squares the piece can move to and remove the ones that cause the king to be
            // in check
            for (position, move_type) in
                evaluate_vector(board, base_vector, number_of_steps, player_color, position)
                    .into_iter()
                    .map(|(position, move_type)| {
                        let mut test_board = board.clone();
                        test_board.set(&position, Some(self.clone()));
                        test_board.set(&position, None);

                        if is_in_check(&test_board, king_position, player_color) {
                            (position, None)
                        } else {
                            (position, Some(move_type))
                        }
                    })
            {
                move_map.set(&position, move_type);
            }
        }
        move_map
    }
}

pub(crate) fn evaluate_vector(
    board: &Board<Color<Piece>>,
    base_vector: (i8, i8),
    number_of_steps: Range<i8>,
    player_color: &Turn,
    position: &BoardPosition,
) -> Vec<(BoardPosition, MoveType)> {
    // Return a vector containing each square along a vector up to and including the first capture
    // or up to the edge of the board
    number_of_steps
        .map_while(|current_amount| {
            check_square(board, position, base_vector, current_amount, player_color)
        })
        .scan(0, |state, (position, move_type)| {
            if *state >= 1 {
                return None;
            }

            if matches!(move_type, MoveType::Capture) {
                *state += 1;
            }

            Some((position, move_type))
        })
        .collect()
}

fn check_square(
    board: &Board<Color<Piece>>,
    position: &BoardPosition,
    base_vector: (i8, i8),
    current_amount: i8,
    player_color: &Turn,
) -> Option<(BoardPosition, MoveType)> {
    let new_position = position
        .clone()
        .add(vector_multiplication(base_vector, current_amount))
        .ok()?;

    let square_on_new_position = board.get(&new_position);

    let move_type = if let Some(piece) = square_on_new_position.as_ref() {
        if piece.same_color(player_color) {
            return None;
        } else {
            MoveType::Capture
        }
    } else {
        MoveType::Move
    };

    Some((new_position, move_type))
}

fn vector_multiplication(vector: (i8, i8), scalar: i8) -> (i8, i8) {
    (vector.0 * scalar, vector.1 * scalar)
}

#[cfg(test)]
mod tests {
    fn test() {}
}
