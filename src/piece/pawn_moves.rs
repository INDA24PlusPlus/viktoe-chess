use std::ops::Range;

use crate::position::BoardPosition;
use crate::piece::{Color, Piece, PawnState, vector_multiplication};
use crate::board::{Board, MoveType};

// does not handle pawn causing own king to be in check
pub fn get_pawn_moves(
    board: &Board<Color<Piece>>,
    position: &BoardPosition,
    piece: &Color<Piece>,
) -> Board<MoveType> {
    let mut move_map = Board::default();

    for base_vector in piece.get_movement_base_vector() {
        for current_amount in Range::<i8>::from(piece.get_number_of_moves()) {
            let new_position = position
                .clone()
                .add(vector_multiplication(base_vector, current_amount))
                .unwrap();

            move_map.set(
                &new_position,
                match board.get(&new_position) {
                    None => Some(MoveType::Move),
                    Some(_) => break,
                },
            )
        }
    }

    // Handle pawn captures
    match piece {
        Color::White(_) => {
            if let Ok(position) = position.add((-1, 1)) {
                if matches!(board.get(&position), Some(Color::Black(_))) {
                    move_map.set(&position, Some(MoveType::Capture));
                }
            }

            if let Ok(position) = position.add((1, 1)) {
                if matches!(board.get(&position), Some(Color::Black(_))) {
                    move_map.set(&position, Some(MoveType::Capture));
                }
            }
        }
        Color::Black(_) => {
            if let Ok(position) = position.add((-1, -1)) {
                if matches!(board.get(&position), Some(Color::White(_))) {
                    move_map.set(&position, Some(MoveType::Capture));
                }
            }

            if let Ok(position) = position.add((1, 1)) {
                if matches!(board.get(&position), Some(Color::White(_))) {
                    move_map.set(&position, Some(MoveType::Capture));
                }
            }
        }
    }

    // Handle en passant
    match piece {
        Color::White(_) => {
            if let Ok(position) = position.add((-1, 0)) {
                if matches!(
                    board.get(&position),
                    Some(Color::Black(Piece::Pawn {
                        state: PawnState::PosibleEnPassant
                    }))
                ) {
                    move_map.set(&position, Some(MoveType::Capture));
                }
            }

            if let Ok(position) = position.add((1, 0)) {
                if matches!(
                    board.get(&position),
                    Some(Color::Black(Piece::Pawn {
                        state: PawnState::PosibleEnPassant
                    }))
                ) {
                    move_map.set(&position, Some(MoveType::Capture));
                }
            }
        }
        Color::Black(_) => {
            if let Ok(position) = position.add((-1, 0)) {
                if matches!(
                    board.get(&position),
                    Some(Color::White(Piece::Pawn {
                        state: PawnState::PosibleEnPassant
                    }))
                ) {
                    move_map.set(&position, Some(MoveType::Capture));
                }
            }

            if let Ok(position) = position.add((1, 0)) {
                if matches!(
                    board.get(&position),
                    Some(Color::White(Piece::Pawn {
                        state: PawnState::PosibleEnPassant
                    }))
                ) {
                    move_map.set(&position, Some(MoveType::Capture));
                }
            }
        }
    }

    move_map
}
