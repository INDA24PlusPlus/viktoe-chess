use std::ops::Range;

use crate::board::{is_in_check, Board, MoveType, Turn};
use crate::position::BoardPosition;
use crate::position::{File::*, Rank::*};

type KingSide = bool;
type QueenSide = bool;

pub const WHITE_KING: Color<Piece> = Color::White(Piece::King {
    check_state: None,
    castling_state: (false, false),
});
pub const NEW_WHITE_KING: Color<Piece> = Color::White(Piece::King {
    check_state: None,
    castling_state: (true, true)
});
pub const WHITE_QUEEN: Color<Piece> = Color::White(Piece::Queen);
pub const WHITE_BISHOP: Color<Piece> = Color::White(Piece::Bishop);
pub const WHITE_KNIGHT: Color<Piece> = Color::White(Piece::Knight);
pub const WHITE_ROOK: Color<Piece> = Color::White(Piece::Rook);
pub const WHITE_PAWN: Color<Piece> = Color::White(Piece::Pawn {
    state: PawnState::Default,
});
pub const NEW_WHITE_PAWN: Color<Piece> = Color::White(Piece::Pawn {
    state: PawnState::FirstMove,
});

pub const BLACK_KING: Color<Piece> = Color::Black(Piece::King {
    check_state: None,
    castling_state: (false, false),
});
pub const NEW_BLACK_KING: Color<Piece> = Color::Black(Piece::King {
    check_state: None,
    castling_state: (true, true),
});
pub const BLACK_QUEEN: Color<Piece> = Color::Black(Piece::Queen);
pub const BLACK_BISHOP: Color<Piece> = Color::Black(Piece::Bishop);
pub const BLACK_KNIGHT: Color<Piece> = Color::Black(Piece::Knight);
pub const BLACK_ROOK: Color<Piece> = Color::Black(Piece::Rook);
pub const BLACK_PAWN: Color<Piece> = Color::Black(Piece::Pawn {
    state: PawnState::Default,
});
pub const NEW_BLACK_PAWN: Color<Piece> = Color::Black(Piece::Pawn {
    state: PawnState::FirstMove,
});

impl Piece {
    const KING_MOVES: [(i8, i8); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    const QUEEN_MOVES: [(i8, i8); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    const ROOK_MOVES: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    const BISHOP_MOVES: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
    const KNIGTH_MOVES: [(i8, i8); 8] = [
        (-1, 2),
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
        (-2, 1),
    ];
    const WHITE_PAWN_MOVES: [(i8, i8); 1] = [(0, 1)];
    const BLACK_PAWN_MOVES: [(i8, i8); 1] = [(0, -1)];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color<T> {
    White(T),
    Black(T),
}

impl<T> Default for Color<T>
where
    T: Default,
{
    fn default() -> Self {
        Color::White(T::default())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq, Hash)]
pub enum Piece {
    King {
        check_state: Option<CheckState>,
        castling_state: (KingSide, QueenSide)
    },
    #[default]
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn {
        state: PawnState,
    },
}

impl Color<Piece> {
    pub fn get_movement_base_vector(self) -> Vec<(i8, i8)> {
        match self {
            Color::White(Piece::Pawn { .. }) => Piece::WHITE_PAWN_MOVES.to_vec(),
            Color::Black(Piece::Pawn { .. }) => Piece::BLACK_PAWN_MOVES.to_vec(),
            Color::Black(piece) | Color::White(piece) => match piece {
                Piece::King { .. } => Piece::KING_MOVES.to_vec(),
                Piece::Queen => Piece::QUEEN_MOVES.to_vec(),
                Piece::Bishop => Piece::BISHOP_MOVES.to_vec(),
                Piece::Knight => Piece::BISHOP_MOVES.to_vec(),
                Piece::Rook => Piece::ROOK_MOVES.to_vec(),
                _ => Vec::new(),
            },
        }
    }

    pub fn get_number_of_moves(self) -> StepCount {
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
    pub fn change_internal(&mut self, value: T) {
        *self = match self {
            Color::White(_) => Color::White(value),
            Color::Black(_) => Color::Black(value)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PawnState {
    FirstMove,
    PosibleEnPassant,
    Default,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckState {
    CheckMate,
    Check,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CastlingState {
    Castling,
    CastlingKingSide,
    CastlingQueenSide,
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
    board: &Board<Color<Piece>>,
    piece: &Color<Piece>,
    position: &BoardPosition,
    king_position: &BoardPosition,
    player_color: Turn,
) -> Board<MoveType> {
    let mut move_map = Board::default();

    for base_vector in piece.get_movement_base_vector() {
        let number_of_steps = piece.get_number_of_moves().into();

        if let Ok(new_position) = position.add(base_vector) {
            let mut test_board = board.clone();
            test_board.set_index(&new_position, Some(piece.clone()));
            test_board.set_index(position, None);

            if is_in_check(board, king_position, player_color) {
                continue;
            }
        }

        for (position, move_type) in
            evaluate_vector(board, base_vector, number_of_steps, player_color, position)
        {
            move_map.set_index(&position, Some(move_type));
        }
    }
    move_map
}

pub fn evaluate_vector(
    board: &Board<Color<Piece>>,
    base_vector: (i8, i8),
    number_of_steps: Range<i8>,
    player_color: Turn,
    position: &BoardPosition,
) -> Vec<(BoardPosition, MoveType)> {
    number_of_steps
        .map_while(|current_amount| {
            check_square(board, position, base_vector, current_amount, player_color)
        })
        .collect()
}

fn check_square(
    board: &Board<Color<Piece>>,
    position: &BoardPosition,
    base_vector: (i8, i8),
    current_amount: i8,
    player_color: Turn,
) -> Option<(BoardPosition, MoveType)> {
    match position
        .clone()
        .add(vector_multiplication(base_vector, current_amount))
    {
        Ok(new_position) => {
            let square_on_new_position = board.get_index(&new_position);
            let move_type = match player_color {
                // If the piece to be moved is white and the piece on the square to be moved to
                // contians a black piece
                Turn::White if matches!(square_on_new_position, Some(Color::Black(_))) => {
                    MoveType::Capture
                }
                // If the piece to be moved is black and the piece on the square to be moved to
                // contians a white piece
                Turn::Black if matches!(square_on_new_position, Some(Color::White(_))) => {
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

            move_map.set_index(
                &new_position,
                match board.get_index(&new_position) {
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
                if matches!(board.get_index(&position), Some(Color::Black(_))) {
                    move_map.set_index(&position, Some(MoveType::Capture));
                }
            }

            if let Ok(position) = position.add((1, 1)) {
                if matches!(board.get_index(&position), Some(Color::Black(_))) {
                    move_map.set_index(&position, Some(MoveType::Capture));
                }
            }
        }
        Color::Black(_) => {
            if let Ok(position) = position.add((-1, -1)) {
                if matches!(board.get_index(&position), Some(Color::White(_))) {
                    move_map.set_index(&position, Some(MoveType::Capture));
                }
            }

            if let Ok(position) = position.add((1, 1)) {
                if matches!(board.get_index(&position), Some(Color::White(_))) {
                    move_map.set_index(&position, Some(MoveType::Capture));
                }
            }
        }
    }

    // Handle en passant
    match piece {
        Color::White(_) => {
            if let Ok(position) = position.add((-1, 0)) {
                if matches!(
                    board.get_index(&position),
                    Some(Color::Black(Piece::Pawn {
                        state: PawnState::PosibleEnPassant
                    }))
                ) {
                    move_map.set_index(&position, Some(MoveType::Capture));
                }
            }

            if let Ok(position) = position.add((1, 0)) {
                if matches!(
                    board.get_index(&position),
                    Some(Color::Black(Piece::Pawn {
                        state: PawnState::PosibleEnPassant
                    }))
                ) {
                    move_map.set_index(&position, Some(MoveType::Capture));
                }
            }
        }
        Color::Black(_) => {
            if let Ok(position) = position.add((-1, 0)) {
                if matches!(
                    board.get_index(&position),
                    Some(Color::White(Piece::Pawn {
                        state: PawnState::PosibleEnPassant
                    }))
                ) {
                    move_map.set_index(&position, Some(MoveType::Capture));
                }
            }

            if let Ok(position) = position.add((1, 0)) {
                if matches!(
                    board.get_index(&position),
                    Some(Color::White(Piece::Pawn {
                        state: PawnState::PosibleEnPassant
                    }))
                ) {
                    move_map.set_index(&position, Some(MoveType::Capture));
                }
            }
        }
    }

    move_map
}

pub fn get_king_moves(
    board: &Board<Color<Piece>>,
    position: &BoardPosition,
    piece: &Color<Piece>,
    castling_state: (KingSide, QueenSide),
    player_color: Turn,
) -> Board<MoveType> {
    let mut move_map = get_standard_valid_move(board, piece, position, position, player_color);

    let (king_side, queen_side) = castling_state;

    if king_side {
        get_king_side_castle(board, &mut move_map, piece);
    }
    if queen_side {
        get_queen_side_castle(board, &mut move_map, piece);
    }

    move_map
}

fn get_king_side_castle(
    board: &Board<Color<Piece>>,
    move_map: &mut Board<MoveType>,
    piece: &Color<Piece>,
) {
    match piece {
        Color::White(_) => {
            if board.get_index(&BoardPosition::from((F, One))).is_some() {
                return;
            }
            if board.get_index(&BoardPosition::from((G, One))).is_some() {
                return;
            }

            move_map.set_index(&BoardPosition::from((G, One)), Some(MoveType::Move))
        }
        Color::Black(_) => {
            if board.get_index(&BoardPosition::from((F, Eight))).is_some() {
                return;
            }
            if board.get_index(&BoardPosition::from((G, Eight))).is_some() {
                return;
            }

            move_map.set_index(&BoardPosition::from((G, One)), Some(MoveType::Move))
        }
    }
}

fn get_queen_side_castle(
    board: &Board<Color<Piece>>,
    move_map: &mut Board<MoveType>,
    piece: &Color<Piece>,
) {
    match piece {
        Color::White(_) => {
            if board.get_index(&BoardPosition::from((B, One))).is_some() {
                return;
            }
            if board.get_index(&BoardPosition::from((C, One))).is_some() {
                return;
            }
            if board.get_index(&BoardPosition::from((D, One))).is_some() {
                return;
            }

            move_map.set_index(&BoardPosition::from((B, One)), Some(MoveType::Move))
        }
        Color::Black(_) => {
            if board.get_index(&BoardPosition::from((B, Eight))).is_some() {
                return;
            }
            if board.get_index(&BoardPosition::from((C, Eight))).is_some() {
                return;
            }
            if board.get_index(&BoardPosition::from((B, Eight))).is_some() {
                return;
            }

            move_map.set_index(&BoardPosition::from((B, One)), Some(MoveType::Move))
        }
    }
}

fn vector_multiplication(vector: (i8, i8), scalar: i8) -> (i8, i8) {
    (vector.0 * scalar, vector.1 * scalar)
}

#[cfg(test)]
mod tests {
    fn test(){}

}
