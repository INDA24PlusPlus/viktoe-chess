use crate::piece::{
    evaluate_vector, get_king_moves, get_pawn_moves, get_standard_valid_move,
    Color, PawnState, Piece, StepCount, BLACK_BISHOP, BLACK_KNIGHT, BLACK_PAWN,
    BLACK_QUEEN, BLACK_ROOK, NEW_BLACK_KING, NEW_BLACK_PAWN, NEW_WHITE_KING, NEW_WHITE_PAWN,
    WHITE_BISHOP, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
};
use crate::position::{
    BoardPosition,
    File::*,
    Rank::*,
};
use crate::ChessError;

#[derive(Clone, Copy)]
pub enum Turn {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub enum GameState {
    Ongoing,
    Check,
    CheckMate,
}

#[derive(Debug, Clone, Copy)]
pub enum MoveType {
    Move,
    Capture,
}

#[derive(Clone)]
pub struct Board<T> {
    board: [[Option<T>; 8]; 8],
    position: BoardPosition,
}

impl<T> Board<T> {
    pub fn get_index(&self, position: &BoardPosition) -> &Option<T> {
        let (file, rank) = position.into();

        &self.board[usize::from(file)][usize::from(rank)]
    }

    pub fn set_index(&mut self, position: &BoardPosition, value: Option<T>) {
        let (file, rank) = position.into();

        self.board[usize::from(file)][usize::from(rank)] = value;
    }
}

impl<T> Default for Board<T>
where
    T: Copy,
{
    fn default() -> Board<T> {
        Board {
            board: [[None; 8]; 8],
            position: BoardPosition::from((A, Eight)),
        }
    }
}

impl<T> Iterator for Board<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = (*self.get_index(&self.position)).clone();

        self.position = match self.position.add((1, 0)) {
            Ok(pos) => pos,
            Err(_) => match self.position.add((-8, -1)) {
                Ok(pos) => pos,
                Err(_) => return None,
            },
        };

        item
    }
}

// impl Board<Color<Piece>> {
//     fn from_fen(&str) -> Result<Self, > {
//
//     }
// }

pub struct ChessGame {
    pub board: Board<Color<Piece>>,
    turn: Turn,
    state: GameState,
    white_king_position: BoardPosition,
    black_king_position: BoardPosition,
    en_passant: Vec<BoardPosition>,
}

impl Default for ChessGame {
    fn default() -> Self {
        let mut board = Board::default();

        board.set_index(&BoardPosition::from((A, One)), Some(WHITE_ROOK));
        board.set_index(&BoardPosition::from((H, One)), Some(WHITE_ROOK));

        board.set_index(&BoardPosition::from((B, One)), Some(WHITE_KNIGHT));
        board.set_index(&BoardPosition::from((G, One)), Some(WHITE_KNIGHT));

        board.set_index(&BoardPosition::from((C, One)), Some(WHITE_BISHOP));
        board.set_index(&BoardPosition::from((F, One)), Some(WHITE_BISHOP));

        board.set_index(&BoardPosition::from((D, One)), Some(WHITE_QUEEN));
        board.set_index(&BoardPosition::from((E, One)), Some(NEW_WHITE_KING));

        let white_king_position = BoardPosition::from((E, One));

        for file in 0..8 {
            board.set_index(
                &BoardPosition::try_from((file, 1)).unwrap(),
                Some(NEW_WHITE_PAWN),
            );
        }

        board.set_index(&BoardPosition::from((A, Eight)), Some(BLACK_ROOK));
        board.set_index(&BoardPosition::from((H, Eight)), Some(BLACK_ROOK));

        board.set_index(&BoardPosition::from((B, Eight)), Some(BLACK_KNIGHT));
        board.set_index(&BoardPosition::from((G, Eight)), Some(BLACK_KNIGHT));

        board.set_index(&BoardPosition::from((C, Eight)), Some(BLACK_BISHOP));
        board.set_index(&BoardPosition::from((F, Eight)), Some(BLACK_BISHOP));

        board.set_index(&BoardPosition::from((D, Eight)), Some(BLACK_QUEEN));
        board.set_index(&BoardPosition::from((E, Eight)), Some(NEW_BLACK_KING));

        let black_king_position = BoardPosition::from((E, Eight));

        for file in 0..8 {
            board.set_index(
                &BoardPosition::try_from((file, 6)).unwrap(),
                Some(NEW_BLACK_PAWN),
            );
        }

        Self {
            board,
            turn: Turn::White,
            state: GameState::Ongoing,
            white_king_position,
            black_king_position,
            en_passant: Vec::new(),
        }
    }
}

impl ChessGame {
    pub fn get_index(&self, position: &BoardPosition) -> &Option<Color<Piece>> {
        self.board.get_index(position)
    }

    pub fn get_valid_moves(&self, position: &BoardPosition) -> Option<Board<MoveType>> {
        let piece = match self.board.get_index(position) {
            Some(piece) => piece,
            None => return None,
        };

        let piece_type = match piece {
            Color::White(piece) => piece,
            Color::Black(piece) => piece,
        };

        let king_position = match self.turn {
            Turn::White => &self.white_king_position,
            Turn::Black => &self.black_king_position,
        };

        Some(match piece_type {
            Piece::King { castling_state, .. } => {
                get_king_moves(&self.board, position, piece, *castling_state, self.turn)
            }
            Piece::Pawn { .. } => get_pawn_moves(&self.board, position, piece),
            _ => get_standard_valid_move(&self.board, piece, position, king_position, self.turn),
        })
    }

    pub fn move_piece(
        &mut self,
        initial_position: &BoardPosition,
        desired_position: &BoardPosition,
    ) -> Result<GameState, ChessError> {
        let mut piece = *match self.board.get_index(initial_position) {
            Some(piece) => match piece {
                Color::White(_) if matches!(self.turn, Turn::White) => piece,
                Color::Black(_) if matches!(self.turn, Turn::Black) => piece,
                _ => return Err(ChessError::NoPiece),
            },
            None => return Err(ChessError::NoPiece),
        };

        let moves = self
            .get_valid_moves(initial_position)
            .ok_or(ChessError::NoMoves)?;

        moves
            .get_index(desired_position)
            .ok_or(ChessError::InvalidMove)?;

        self.en_passant = Vec::new();

        match piece {
            NEW_WHITE_PAWN | NEW_BLACK_PAWN => piece.change_internal(Piece::Pawn {
                state: PawnState::PosibleEnPassant,
            }),
            Color::White(Piece::King { .. }) | Color::Black(Piece::King { .. }) => piece
                .change_internal(Piece::King {
                    check_state: None,
                    castling_state: (false, false),
                }),
            _ => {}
        }

        let king_position = match self.turn {
            Turn::White => &self.white_king_position,
            Turn::Black => &self.black_king_position,
        };

        if matches!(piece, WHITE_ROOK | BLACK_ROOK) {
            match self.get_index(king_position) {
                Some(Color::White(Piece::King {
                    castling_state: (true, queen_side),
                    ..
                })) if *initial_position == BoardPosition::from((H, One)) => {
                    self.board.set_index(
                        king_position,
                        Some(Color::White(Piece::King {
                            check_state: None,
                            castling_state: (false, *queen_side)
                        })),
                    )
                }
                Some(Color::White(Piece::King {
                    castling_state: (king_side, true),
                    ..
                })) if *initial_position == BoardPosition::from((A, One)) => {
                    self.board.set_index(
                        king_position,
                        Some(Color::White(Piece::King {
                            check_state: None,
                            castling_state: (*king_side, false)
                        })),
                    )
                }
                Some(Color::Black(Piece::King {
                    castling_state: (true, queen_side),
                    ..
                })) if *initial_position == BoardPosition::from((A, Eight)) => {
                    self.board.set_index(
                        king_position,
                        Some(Color::Black(Piece::King {
                            check_state: None,
                            castling_state: (false, *queen_side)
                        })),
                    )
                }
                Some(Color::Black(Piece::King {
                    castling_state: (king_side, true),
                    ..
                })) if *initial_position == BoardPosition::from((H, Eight)) => {
                    self.board.set_index(
                        king_position,
                        Some(Color::Black(Piece::King {
                            check_state: None,
                            castling_state: (*king_side, false)
                        })),
                    )
                }
                _ => {}
            }
        }

        self.board.set_index(initial_position, None);
        self.board.set_index(desired_position, Some(piece));

        self.turn = match self.turn {
            Turn::White => Turn::Black,
            Turn::Black => Turn::White,
        };

        let king_position = match self.turn {
            Turn::White => &self.white_king_position,
            Turn::Black => &self.black_king_position,
        };

        if is_in_check(&self.board, king_position, self.turn) {
            Ok(GameState::Check)
        } else {
            Ok(GameState::Ongoing)
        }
    }

    pub fn get_player_turn(&self) -> Turn {
        self.turn
    }

    pub fn get_game_state(&self) -> GameState {
        self.state
    }
}

// Does not yet handle king moving to close
pub fn is_in_check(
    board: &Board<Color<Piece>>,
    king_position: &BoardPosition,
    player_color: Turn,
) -> bool {
    // Check if pawn causes king to be in check
    match player_color {
        Turn::White => {
            if check_by_pawn(board, king_position, vec![(-1, 1), (1, 1)], BLACK_PAWN) {
                return true;
            }
        }
        Turn::Black => {
            if check_by_pawn(board, king_position, vec![(-1, -1), (1, -1)], WHITE_PAWN) {
                return true;
            }
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
    player_color: Turn,
) -> bool {
    move_set.into_iter().any(|base_vector| {
        let number_of_steps = StepCount::Infinty.into();

        match evaluate_vector(board, base_vector, number_of_steps, player_color, position).last() {
            Some((position, MoveType::Capture)) => {
                pieces.iter().any(|piece| match board.get_index(position) {
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
    piece: Color<Piece>,
) -> bool {
    vectors.into_iter().any(|vector| {
        king_position
            .add(vector)
            .and_then(|position| {
                if *board.get_index(&position) == Some(piece) {
                    Ok(position)
                } else {
                    Err(ChessError::InvalidMove)
                }
            })
            .is_ok()
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_test_1() {
        let mut board = Board::default();

        board.set_index(&BoardPosition::from((E, One)), Some(NEW_WHITE_KING));
        board.set_index(&BoardPosition::from((E, Eight)), Some(NEW_BLACK_KING));

        let game = ChessGame {
            board,
            ..Default::default()
        };

        assert!(!is_in_check(
            &game.board,
            &game.white_king_position,
            game.turn
        ));
        assert!(!is_in_check(
            &game.board,
            &game.black_king_position,
            Turn::Black
        ));
    }

    #[test]
    fn check_test_2() {
        let mut board = Board::default();

        board.set_index(&BoardPosition::from((E, One)), Some(NEW_WHITE_KING));
        board.set_index(&BoardPosition::from((E, Eight)), Some(NEW_BLACK_KING));
        board.set_index(&BoardPosition::from((A, Eight)), Some(WHITE_ROOK));

        let game = ChessGame {
            board,
            ..Default::default()
        };

        assert!(!is_in_check(
            &game.board,
            &game.white_king_position,
            game.turn
        ));
        assert!(is_in_check(
            &game.board,
            &game.black_king_position,
            Turn::Black
        ));
    }
}
