use crate::board::{Board, MoveType, Turn};
use crate::position::BoardPosition;
use crate::piece::shorthands::*;
use crate::piece::{Piece, Color, StepCount, evaluate_vector};

pub(crate) fn is_in_check(
    board: &Board<Color<Piece>>,
    king_position: &BoardPosition,
    player_color: &Turn,
) -> bool {
    // Check if pawn causes king to be in check
    match player_color {
        Turn::White => {
            if check_by_pawn(board, king_position, vec![(-1, 1), (1, 1)], &BLACK_PAWN) {
                return true;
            }
        }
        Turn::Black => {
            if check_by_pawn(board, king_position, vec![(-1, -1), (1, -1)], &WHITE_PAWN) {
                return true;
            }
        }
    }

    for base_vector in WHITE_KING.get_movement_base_vector() {
        match evaluate_vector(board, base_vector, 0..2, player_color, king_position).last() {
            Some((position, MoveType::Capture))
                if matches!(
                    board.get(position).as_ref().unwrap().get_internal(),
                    Piece::King { .. }
                ) =>
            {
                return true
            }
            _ => {}
        }
    }

    let move_sets = vec![
        (
            WHITE_ROOK.get_movement_base_vector(),
            vec![Piece::Rook, Piece::Queen],
        ),
        (
            WHITE_BISHOP.get_movement_base_vector(),
            vec![Piece::Bishop, Piece::Queen],
        ),
        (WHITE_KNIGHT.get_movement_base_vector(), vec![Piece::Knight]),
    ];

    // Check if queen, rook, or bishop causes king to be in check
    move_sets.into_iter().any(|(move_set, pieces)| {
        check_vector(board, king_position, move_set, pieces, player_color)
    })
}


fn check_vector(
    board: &Board<Color<Piece>>,
    position: &BoardPosition,
    move_set: Vec<(i8, i8)>,
    pieces: Vec<Piece>,
    player_color: &Turn,
) -> bool {
    move_set.into_iter().any(|base_vector| {
        let number_of_steps = StepCount::Infinty.into();

        match evaluate_vector(board, base_vector, number_of_steps, player_color, position).last() {
            Some((position, MoveType::Capture)) => {
                pieces.iter().any(|piece| match board.get(position) {
                    Some(Color::White(new_piece)) | Some(Color::Black(new_piece)) => {
                        *new_piece == *piece
                    }
                    _ => false,
                })
            }
            _ => false,
        }
    })
}

fn check_by_pawn(
    board: &Board<Color<Piece>>,
    king_position: &BoardPosition,
    vectors: Vec<(i8, i8)>,
    piece: &Color<Piece>,
) -> bool {
    vectors.into_iter().any(|vector| {
        let position = match king_position.add(vector) {
            Ok(position) => position,
            Err(_) => return false,
        };

        board.get(&position).as_ref() == Some(piece)
    })
}
