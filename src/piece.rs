use std::ops::Range;

use crate::board::{Board, MoveType};
use crate::position::BoardPosition;
use crate::position::{Horizontal::*, Vertical::*};

#[derive(Clone, Copy, PartialEq)]
pub enum ColouredPiece<T> {
    White(T),
    Black(T),
}

impl<T> Default for ColouredPiece<T>
where
    T: Default,
{
    fn default() -> Self {
        ColouredPiece::White(T::default())
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    King {
        check_state: CheckState,
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

impl Default for Piece {
    fn default() -> Self {
        Piece::new_white_pawn()
    }
}

impl Piece {
    pub fn new_king() -> Self {
        Piece::King {
            check_state: CheckState::None,
            castling_state: CastlingState::Castling,
        }
    }

    pub fn new_white_pawn() -> Self {
        Piece::Pawn {
            state: PawnState::FirstMove,
        }
    }

    pub fn new_black_pawn() -> Self {
        Piece::Pawn {
            state: PawnState::FirstMove,
        }
    }
}

impl ColouredPiece<Piece> {
    pub fn get_movement_base_vector(self) -> Vec<(i8, i8)> {
        match self {
            ColouredPiece::Black(piece) | ColouredPiece::White(piece) => match piece {
                Piece::King { .. } => vec![
                    (0, 1),
                    (1, 1),
                    (1, 0),
                    (1, -1),
                    (0, -1),
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                ],
                Piece::Queen => vec![
                    (0, 1),
                    (1, 1),
                    (1, 0),
                    (1, -1),
                    (0, -1),
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                ],
                Piece::Rook => vec![(0, 1), (1, 0), (0, -1), (-1, 0)],
                Piece::Bishop => vec![(1, 1), (1, -1), (-1, -1), (-1, 1)],
                Piece::Knight => vec![
                    (-1, 2),
                    (1, 2),
                    (2, 1),
                    (2, -1),
                    (1, -2),
                    (-1, -2),
                    (-2, -1),
                    (-2, 1),
                ],
                Piece::Pawn { .. } => {
                    if matches!(self, ColouredPiece::White(_)) {
                        vec![(0, 1)]
                    } else {
                        vec![(0, -1)]
                    }
                }
            },
        }
    }

    pub fn get_number_of_moves(self) -> StepCount {
        match self {
            ColouredPiece::White(piece) | ColouredPiece::Black(piece) => match piece {
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

#[derive(Clone, Copy, PartialEq)]
pub enum PawnState {
    FirstMove,
    PosibleEnPassant,
    Default,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CheckState {
    CheckMate,
    Check,
    None,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CastlingState {
    Castling,
    CastlingKingSide,
    CastlingQueenSide,
    None,
}

#[derive(Clone, Copy)]
pub enum StepCount {
    One,
    Two,
    Infinty,
}

impl From<StepCount> for Range<i8> {
    fn from(value: StepCount) -> Self {
        match value {
            StepCount::One => 1..2,
            StepCount::Two => 1..3,
            StepCount::Infinty => 1..8,
        }
    }
}

pub fn get_standard_valid_move(
    board: &Board<ColouredPiece<Piece>>,
    piece: &ColouredPiece<Piece>,
    position: &BoardPosition,
) -> Board<MoveType> {
    let mut move_map = Board::default();

    for base_vector in piece.get_movement_base_vector() {
        for current_amount in Range::<i8>::from(piece.get_number_of_moves()) {
            if let Some((position, move_type)) =
                check_square(board, position, base_vector, current_amount, piece)
            {
                move_map.set_index(&position, move_type);
            }
        }
    }
    move_map
}

fn check_square(
    board: &Board<ColouredPiece<Piece>>,
    position: &BoardPosition,
    base_vector: (i8, i8),
    current_amount: i8,
    piece: &ColouredPiece<Piece>,
) -> Option<(BoardPosition, MoveType)> {
    match position
        .clone()
        .add(vector_multiplication(base_vector, current_amount))
    {
        Ok(new_position) => {
            let square_on_new_position = board.get_index(&new_position);
            let move_type = match piece {
                // If the piece to be moved is white and the piece on the square to be moved to
                // contians a black piece
                ColouredPiece::White(_)
                    if matches!(square_on_new_position, Some(ColouredPiece::Black(_))) =>
                {
                    MoveType::Capture
                }
                // If the piece to be moved is black and the piece on the square to be moved to
                // contians a white piece
                ColouredPiece::Black(_)
                    if matches!(square_on_new_position, Some(ColouredPiece::White(_))) =>
                {
                    MoveType::Capture
                }
                _ if square_on_new_position.is_none() => MoveType::Move,
                _ => return None,
            };

            Some((new_position, move_type))
        }
        Err(_) => None,
    }
}

pub fn get_pawn_moves(
    board: &Board<ColouredPiece<Piece>>,
    position: &BoardPosition,
    piece: &ColouredPiece<Piece>,
) -> Board<MoveType> {
    let mut move_map = Board::default();

    for base_vector in piece.get_movement_base_vector() {
        for current_amount in Range::<i8>::from(piece.get_number_of_moves()) {
            let new_position = position
                .clone()
                .add(vector_multiplication(base_vector, current_amount))
                .unwrap();

            move_map.set_index(
                &new_position,
                match board.get_index(&new_position) {
                    None => MoveType::Move,
                    Some(_) => break,
                },
            )
        }
    }

    // Handle pawn captures
    match piece {
        ColouredPiece::White(_) => {
            if let Ok(position) = position.add((-1, 1)) {
                if matches!(board.get_index(&position), Some(ColouredPiece::Black(_))) {
                    move_map.set_index(&position, MoveType::Capture);
                }
            }

            if let Ok(position) = position.add((1, 1)) {
                if matches!(board.get_index(&position), Some(ColouredPiece::Black(_))) {
                    move_map.set_index(&position, MoveType::Capture);
                }
            }
        }
        ColouredPiece::Black(_) => {
            if let Ok(position) = position.add((-1, -1)) {
                if matches!(board.get_index(&position), Some(ColouredPiece::White(_))) {
                    move_map.set_index(&position, MoveType::Capture);
                }
            }

            if let Ok(position) = position.add((1, 1)) {
                if matches!(board.get_index(&position), Some(ColouredPiece::White(_))) {
                    move_map.set_index(&position, MoveType::Capture);
                }
            }
        }
    }

    // Handle en passant
    match piece {
        ColouredPiece::White(_) => {
            if let Ok(position) = position.add((-1, 0)) {
                if matches!(
                    board.get_index(&position),
                    Some(ColouredPiece::Black(Piece::Pawn {
                        state: PawnState::PosibleEnPassant
                    }))
                ) {
                    move_map.set_index(&position, MoveType::Capture);
                }
            }

            if let Ok(position) = position.add((1, 0)) {
                if matches!(
                    board.get_index(&position),
                    Some(ColouredPiece::Black(Piece::Pawn {
                        state: PawnState::PosibleEnPassant
                    }))
                ) {
                    move_map.set_index(&position, MoveType::Capture);
                }
            }
        }
        ColouredPiece::Black(_) => {
            if let Ok(position) = position.add((-1, 0)) {
                if matches!(
                    board.get_index(&position),
                    Some(ColouredPiece::White(Piece::Pawn {
                        state: PawnState::PosibleEnPassant
                    }))
                ) {
                    move_map.set_index(&position, MoveType::Capture);
                }
            }

            if let Ok(position) = position.add((1, 0)) {
                if matches!(
                    board.get_index(&position),
                    Some(ColouredPiece::White(Piece::Pawn {
                        state: PawnState::PosibleEnPassant
                    }))
                ) {
                    move_map.set_index(&position, MoveType::Capture);
                }
            }
        }
    }

    move_map
}

pub fn get_king_moves(
    board: &Board<ColouredPiece<Piece>>,
    position: &BoardPosition,
    piece: &ColouredPiece<Piece>,
    castling_state: &CastlingState,
) -> Board<MoveType> {
    let mut move_map = get_standard_valid_move(board, piece, position);

    match castling_state {
        CastlingState::Castling => {
            get_king_side_castle(board, &mut move_map, piece);
            get_queen_side_castle(board, &mut move_map, piece);
        }
        CastlingState::CastlingKingSide => get_king_side_castle(board, &mut move_map, piece),
        CastlingState::CastlingQueenSide => get_queen_side_castle(board, &mut move_map, piece),
        CastlingState::None => (),
    }

    move_map
}

fn get_king_side_castle(
    board: &Board<ColouredPiece<Piece>>,
    move_map: &mut Board<MoveType>,
    piece: &ColouredPiece<Piece>,
) {
    match piece {
        ColouredPiece::White(_) => {
            if board.get_index(&BoardPosition::from((F, One))).is_some() {
                return;
            }
            if board.get_index(&BoardPosition::from((G, One))).is_some() {
                return;
            }

            move_map.set_index(&BoardPosition::from((G, One)), MoveType::Move)
        }
        ColouredPiece::Black(_) => {
            if board.get_index(&BoardPosition::from((F, Eight))).is_some() {
                return;
            }
            if board.get_index(&BoardPosition::from((G, Eight))).is_some() {
                return;
            }

            move_map.set_index(&BoardPosition::from((G, One)), MoveType::Move)
        }
    }
}

fn get_queen_side_castle(
    board: &Board<ColouredPiece<Piece>>,
    move_map: &mut Board<MoveType>,
    piece: &ColouredPiece<Piece>,
) {
    match piece {
        ColouredPiece::White(_) => {
            if board.get_index(&BoardPosition::from((B, One))).is_some() {
                return;
            }
            if board.get_index(&BoardPosition::from((C, One))).is_some() {
                return;
            }
            if board.get_index(&BoardPosition::from((D, One))).is_some() {
                return;
            }

            move_map.set_index(&BoardPosition::from((B, One)), MoveType::Move)
        }
        ColouredPiece::Black(_) => {
            if board.get_index(&BoardPosition::from((B, Eight))).is_some() {
                return;
            }
            if board.get_index(&BoardPosition::from((C, Eight))).is_some() {
                return;
            }
            if board.get_index(&BoardPosition::from((B, Eight))).is_some() {
                return;
            }

            move_map.set_index(&BoardPosition::from((B, One)), MoveType::Move)
        }
    }
}

fn vector_multiplication(vector: (i8, i8), scalar: i8) -> (i8, i8) {
    (vector.0 * scalar, vector.1 * scalar)
}
