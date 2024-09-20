mod serilize;
mod trait_implementation;

use std::collections::HashMap;

use crate::piece::{self, shorthands::*};
use crate::piece::{
    evaluate_vector, get_king_moves, get_pawn_moves, Color, PawnState, Piece, StepCount,
};
use crate::position::{self, BoardPosition, File::*, Rank::*};
use crate::ChessError;

#[derive(Clone)]
pub enum Turn {
    White,
    Black,
}

#[derive(Clone)]
pub enum GameState {
    Ongoing,
    Check,
    CheckMate,
    Draw,
    Promotion(BoardPosition, Color<Piece>, MoveType),
}

#[derive(Debug, Clone)]
pub enum MoveType {
    Move,
    Capture,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Board<T> {
    board: [Option<T>; 64],
}

impl<T> Board<T> {
    pub fn get(&self, position: &BoardPosition) -> &Option<T> {
        let (file, rank) = position.into();

        &self.board[(7 - usize::from(file)) + usize::from(rank) * 8]
    }

    pub(crate) fn get_mut(&mut self, position: &BoardPosition) -> &mut Option<T> {
        let (file, rank) = position.into();

        &mut self.board[(7 - usize::from(file)) + usize::from(rank) * 8]
    }

    pub(crate) fn set(&mut self, position: &BoardPosition, value: Option<T>) {
        let (file, rank) = position.into();

        self.board[(7 - usize::from(file)) + usize::from(rank) * 8] = value;
    }
}

pub struct ChessGame {
    board: Board<Color<Piece>>,
    turn: Turn,
    state: GameState,
    white_king_position: BoardPosition,
    black_king_position: BoardPosition,
    en_passant: Vec<BoardPosition>,
    half_move: u8,
    full_move: u8,
    white_possition_history: HashMap<Board<Color<Piece>>, u8>,
    black_possition_history: HashMap<Board<Color<Piece>>, u8>,
}

impl ChessGame {
    pub fn get_square(&self, position: &BoardPosition) -> &Option<Color<Piece>> {
        self.board.get(position)
    }

    pub(crate) fn get_king_position(&self, player_color: &Turn) -> &BoardPosition {
        match player_color {
            Turn::White => &self.white_king_position,
            Turn::Black => &self.black_king_position,
        }
    }

    pub fn get_valid_moves(&self, position: &BoardPosition) -> Option<Board<MoveType>> {
        let piece = self.board.get(position).as_ref()?;

        let king_position = self.get_king_position(&self.turn);

        Some(match piece.get_internal() {
            Piece::King { castling_state, .. } => {
                get_king_moves(&self.board, position, piece, castling_state, &self.turn)
            }
            Piece::Pawn { .. } => get_pawn_moves(&self.board, position, piece),
            _ => piece.get_standard_valid_move(&self.board, position, king_position, &self.turn),
        })
    }

    pub fn move_piece(
        &mut self,
        initial_position: &BoardPosition,
        desired_position: &BoardPosition,
    ) -> Result<GameState, ChessError> {
        let mut piece = self
            .board
            .get(initial_position)
            .clone()
            .ok_or(ChessError::NoPiece)?;

        if !piece.same_color(&self.turn) {
            return Err(ChessError::NoPiece);
        }

        // Check if move is valid
        let moves = self
            .get_valid_moves(initial_position)
            .ok_or(ChessError::NoMoves)?;

        let move_type = moves
            .get(desired_position)
            .as_ref()
            .ok_or(ChessError::InvalidMove)?;

        // Remove castling options if appliceble
        self.remove_castling_options(&piece, initial_position)
            .unwrap();

        // Update list of pawn that can be taken using en passant
        for position in &self.en_passant {
            if let Some(pawn) = self.board.get_mut(position).as_mut() {
                pawn.change_internal(Piece::Pawn {
                    state: PawnState::Default,
                })
            }
        }

        // Handle taking by en passant

        self.en_passant = Vec::new();

        // if the piece is a pawn that on its first move moved to the fourth or fith rank allow it
        // to be taken by en passant
        if matches!(
            piece.get_internal(),
            Piece::Pawn {
                state: PawnState::FirstMove
            }
        ) && (matches!(desired_position.get_rank(), Four)
            || matches!(desired_position.get_rank(), Five))
        {
            self.en_passant.push(desired_position.clone());

            piece.change_internal(Piece::Pawn {
                state: PawnState::PosibleEnPassant,
            })
        }

        // Performe move
        self.board.set(initial_position, None);
        self.board.set(desired_position, Some(piece.clone()));

        if matches!(piece.get_internal(), Piece::King {..}) {
            match self.turn {
                Turn::White => self.white_king_position = desired_position.clone(),
                Turn::Black => self.black_king_position = desired_position.clone(),
            }
        }

        self.state = if matches!(piece.get_internal(), Piece::Pawn { .. })
            && ((matches!(self.turn, Turn::White) && matches!(desired_position.get_rank(), Eight))
                | (matches!(self.turn, Turn::Black) && matches!(desired_position.get_rank(), One)))
        {
            GameState::Promotion(
                desired_position.clone(),
                piece.clone(),
                move_type.clone()
            )
        } else {
            self.progress_turn(piece.get_internal(), move_type)
        };

        Ok(self.state.clone())
    }

    fn progress_turn(&mut self, piece: &Piece, move_type: &MoveType) -> GameState {
        // Comply with repeated position
        match self.turn {
            Turn::White => {
                match self.white_possition_history.get(&self.board) {
                    Some(amount) => self
                        .white_possition_history
                        .insert(self.board.clone(), amount + 1),
                    None => self.white_possition_history.insert(self.board.clone(), 1),
                };
            }
            Turn::Black => {
                match self.black_possition_history.get(&self.board) {
                    Some(amount) => self
                        .black_possition_history
                        .insert(self.board.clone(), amount + 1),
                    None => self.black_possition_history.insert(self.board.clone(), 1),
                };
            }
        };

        // Move to next move
        self.turn = match self.turn {
            Turn::White => Turn::Black,
            Turn::Black => Turn::White,
        };

        self.full_move += 1;

        // Performe check to comply with 50-move draw rule
        if matches!(move_type, MoveType::Capture)
            || matches!(piece, Piece::Pawn { .. })
        {
            self.half_move = 0;
        } else {
            self.half_move += 1;
        }

        self.state = if is_in_check(&self.board, self.get_king_position(&self.turn), &self.turn) {
            // If the player whos turn it is next can move any piece they are not in mate
            if self
                .board
                .board
                .iter()
                .zip(position::iter())
                .filter(|(piece, _)| {
                    if let Some(piece) = piece.as_ref() {
                        piece.same_color(&self.turn)
                    } else {
                        false
                    }
                })
                .any(|(_, position)| self.get_valid_moves(&position).is_some())
            {
                GameState::Check
            } else {
                GameState::CheckMate
            }
        } else if self.half_move >= 100 {
            GameState::Draw
        } else {
            GameState::Ongoing
        };

        self.state.clone()
    }

    pub fn request_draw_due_to_repeated_position(&self) -> bool {
        self.white_possition_history
            .values()
            .any(|value| *value >= 3)
            || self
                .black_possition_history
                .values()
                .any(|value| *value >= 3)
    }

    pub fn promote_pawn(
        &mut self,
        promotion_target: Piece,
    ) -> Result<GameState, ChessError> {
        let (pawn_position, piece, move_type)  = match &mut self.state {
            GameState::Promotion(position, piece, move_type) => (position, piece, move_type),
            _ => return Err(ChessError::InvalidMove)
        };

        // Checking pawn of same color has players turn on correct row (pawn cannot get to the
        // first or last rank exept on the other side)
        if matches!(piece.get_internal(), Piece::Pawn { .. })
            && piece.same_color(&self.turn)
            && (matches!(pawn_position.rank, Eight) || matches!(pawn_position.rank, One))
        {
            piece.change_internal(promotion_target)
        }

        let piece = piece.clone();
        let move_type = move_type.clone();

        Ok(self.progress_turn(piece.get_internal(), &move_type))
    }

    pub fn get_player_turn(&self) -> &Turn {
        &self.turn
    }

    pub fn get_game_state(&self) -> &GameState {
        &self.state
    }
}

impl ChessGame {
    // If king_position is out of sync will not remove
    fn remove_castling_options(
        &mut self,
        piece: &Color<Piece>,
        initial_position: &BoardPosition,
    ) -> Result<(), ChessError> {
        let king_position = self.get_king_position(&self.turn).clone();
        let king = self
            .board
            .get_mut(&king_position)
            .as_mut()
            .ok_or(ChessError::InternalError)?;

        let castling_state = if matches!(piece.get_internal(), Piece::King { .. }) {
            king.change_internal(Piece::King {
                check_state: None,
                castling_state: (false, false),
            });

            return Ok(());
        } else {
            match king.get_internal() {
                Piece::King { castling_state, .. } => castling_state,
                _ => return Ok(()),
            }
        };

        if matches!(*piece, WHITE_ROOK | BLACK_ROOK) {
            match initial_position {
                BoardPosition { file: A, rank: One } => king.change_internal(Piece::King {
                    check_state: None,
                    castling_state: (false, castling_state.1),
                }),
                BoardPosition { file: H, rank: One } => king.change_internal(Piece::King {
                    check_state: None,
                    castling_state: (castling_state.0, false),
                }),
                BoardPosition {
                    file: A,
                    rank: Eight,
                } => king.change_internal(Piece::King {
                    check_state: None,
                    castling_state: (false, castling_state.1),
                }),
                BoardPosition {
                    file: H,
                    rank: Eight,
                } => king.change_internal(Piece::King {
                    check_state: None,
                    castling_state: (castling_state.0, false),
                }),
                _ => {}
            }
        }

        Ok(())
    }
}

// Does not yet handle king moving to close
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_test_1() {
        let mut board = Board::default();

        board.set(&BoardPosition::from((E, One)), Some(NEW_WHITE_KING));
        board.set(&BoardPosition::from((E, Eight)), Some(NEW_BLACK_KING));

        assert!(!is_in_check(
            &board,
            &(E, One).into(),
            &Turn::White,
        ));
        assert!(!is_in_check(
            &board,
            &(E, Eight).into(),
            &Turn::Black
        ));
    }

    #[test]
    fn check_test_2() {
        let mut board = Board::default();

        board.set(&BoardPosition::from((E, One)), Some(NEW_WHITE_KING));
        board.set(&BoardPosition::from((E, Eight)), Some(NEW_BLACK_KING));
        board.set(&BoardPosition::from((A, Eight)), Some(WHITE_ROOK));

        assert!(!is_in_check(
            &board,
            &(E, One).into(),
            &Turn::White,
        ));
        assert!(is_in_check(
            &board,
            &(E, Eight).into(),
            &Turn::Black
        ));
    }

    #[test]
    fn check_test_3() {
        let mut board = Board::default();

        board.set(&(E, One).into(), Some(NEW_WHITE_KING));
        board.set(&(D, Two).into(), Some(BLACK_PAWN));

        assert!(is_in_check(&board, &(E, One).into(), &Turn::White));
    }

    #[test]
    fn remove_castling_correct() {
        let mut game = ChessGame::default();

        game.move_piece(&(E, Two).into(), &(E, Four).into()).unwrap();
        game.move_piece(&(E, Seven).into(), &(E, Five).into()).unwrap();
        game.move_piece(&(E, One).into(), &(E, Two).into()).unwrap();

        let king = game.get_square(&(E, Two).into()).as_ref().unwrap().get_internal();

        println!("{:?}", king);

        assert!(matches!(king, Piece::King { check_state: _, castling_state: (false, false) }));
        
    }
}
