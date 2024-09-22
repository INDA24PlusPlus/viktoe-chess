pub mod prelude;
mod test;
pub mod board;
pub mod piece;
pub mod position;

use std::collections::HashMap;

use crate::position::{BoardPosition, File::*, Rank::*};
use crate::board::{Board, Turn, MoveType, GameState};
use crate::board::check::is_in_check;
use crate::piece::{Color, Piece, get_king_moves, get_pawn_moves, PawnState};
use crate::piece::shorthands::*;

#[derive(Debug)]
pub enum ChessError {
    OutOfBounds,
    NoPiece,
    NotYourPiece,
    NoMoves,
    InvalidMove,
    SelfCheck,
    IncorrectFenString,
    InternalError,
}

/// A sturuct for holding the state of a chess game as well as functions to interface with it
///
/// The squares can be accessed by either indexing or iteration
///
/// ```rust
/// let game = ChessGame::default();
/// let valid_moves = game.get_valid_moves(&(E, Two).into());
///
/// for file in File.iter() {
///     for rank in Rank.iter() {
///         match game.get_square((file, rank)) {
///             ...
///         }
///
///         match valid_moves.get((file, rank)) {
///             ...
///         }
///     }
/// }
/// ```
///
/// ```rust
/// let game = ChessGame::default();
/// let valid_moves = game.get_valid_moves(&(E, Two).into());
///
/// let squares = game.iter().zip(valid_moves.iter());
///
/// for square in squares {
///     let (piece, move_type) = square;
///
///     match piece {
///         ...
///     }
///
///     match move_type {
///         ...
///     }
/// }
/// ```
pub struct ChessGame {
    pub(crate) board: Board<Color<Piece>>,
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
    /// Returns a reference to a specific square on the board
    pub fn get_square(&self, position: &BoardPosition) -> &Option<Color<Piece>> {
        self.board.get(position)
    }

    pub(crate) fn get_king_position(&self, player_color: &Turn) -> &BoardPosition {
        match player_color {
            Turn::White => &self.white_king_position,
            Turn::Black => &self.black_king_position,
        }
    }

    /// Returns a Board containing moves that can be made from a square if moves can be made from
    /// that square.
    pub fn get_valid_moves(&self, position: &BoardPosition) -> Board<MoveType> {
        let piece = match self.board.get(position).as_ref() {
            Some(piece) => piece,
            None => return Board::default(),
        };

        if !piece.same_color(&self.turn) {
            return Board::default();
        }

        let king_position = self.get_king_position(&self.turn);

        match piece.get_internal() {
            Piece::King { castling_state, .. } => {
                get_king_moves(&self.board, position, piece, castling_state, &self.turn)
            }
            Piece::Pawn { .. } => get_pawn_moves(&self.board, position, piece),
            _ => piece.get_standard_valid_move(&self.board, position, king_position, &self.turn),
        }
    }

    /// Moves a piece from one square to another
    ///
    /// Fails with custom errors explaining why the move cannot be made
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
            return Err(ChessError::NotYourPiece);
        }

        // Check if move is valid
        let moves = self
            .get_valid_moves(initial_position);

        let move_type = moves
            .get(desired_position)
            .as_ref()
            .ok_or(ChessError::InvalidMove)?;

        // Move rook during castling
        if matches!(piece.get_internal(), Piece::King { .. })
            && (i32::from(u8::from(initial_position.file.clone())) - i32::from(u8::from(desired_position.file.clone()))).abs() >= 2 {
            if matches!(desired_position, BoardPosition { file: G, rank: One}) {
                self.board.set(&BoardPosition::from((H, One)), None);
                self.board.set(&BoardPosition::from((F, One)), Some(WHITE_ROOK));
            } else if matches!(desired_position, BoardPosition { file: B, rank: One}) {
                self.board.set(&BoardPosition::from((A, One)), None);
                self.board.set(&BoardPosition::from((C, One)), Some(WHITE_ROOK));
            } else if matches!(desired_position, BoardPosition { file: G, rank: Eight}) {
                self.board.set(&BoardPosition::from((H, Eight)), None);
                self.board.set(&BoardPosition::from((F, Eight)), Some(BLACK_ROOK));
            } else if matches!(desired_position, BoardPosition { file: B, rank: Eight}) {
                self.board.set(&BoardPosition::from((A, Eight)), None);
                self.board.set(&BoardPosition::from((C, Eight)), Some(BLACK_ROOK));
            }
        }

        // Remove castling options if appliceble
        self.remove_castling_options(&mut piece, initial_position)
            .unwrap();

        // Update list of pawn that can be taken using en passant
        for position in &self.en_passant {
            if let Some(pawn) = self.board.get_mut(position).as_mut() {
                pawn.change_internal(Piece::Pawn {
                    state: PawnState::Default,
                })
            }
        }

        // Capture piece by en passant
        if matches!(move_type, MoveType::Capture)
            && matches!(piece.get_internal(), Piece::Pawn { .. })
            && self.get_square(desired_position).is_none()
        {
            match self.turn {
                Turn::White => self.board.set(
                    &desired_position
                        .add((0, -1))
                        .expect("Taking by en passant, captured piece should be on board"),
                    None,
                ),
                Turn::Black => self.board.set(
                    &desired_position
                        .add((0, 1))
                        .expect("Taking by en passant, captured piece should be on board"),
                    None,
                ),
            }
        }

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

        if matches!(piece.get_internal(), Piece::King { .. }) {
            match self.turn {
                Turn::White => self.white_king_position = desired_position.clone(),
                Turn::Black => self.black_king_position = desired_position.clone(),
            }
        }

        // Performe move
        self.board.set(initial_position, None);
        self.board.set(desired_position, Some(piece.clone()));

        self.state = if matches!(piece.get_internal(), Piece::Pawn { .. })
            && ((matches!(self.turn, Turn::White) && matches!(desired_position.get_rank(), Eight))
                | (matches!(self.turn, Turn::Black) && matches!(desired_position.get_rank(), One)))
        {
            GameState::Promotion(desired_position.clone(), piece.clone(), move_type.clone())
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
        if matches!(move_type, MoveType::Capture) || matches!(piece, Piece::Pawn { .. }) {
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
                .any(|(_, position)| self.get_valid_moves(&position).iter().any(|square| square.is_some()))
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

    /// If a specific position has been repeated 3 times in a game, a player can request a draw.
    /// The function will return true if 3 repeated positions has occured.
    pub fn request_draw_due_to_repeated_position(&self) -> bool {
        self.white_possition_history
            .values()
            .any(|value| *value >= 3)
            || self
                .black_possition_history
                .values()
                .any(|value| *value >= 3)
    }

    /// Promotes a pawn on the final rank, will return invalid move if there is no pawn to promote.
    pub fn promote_pawn(&mut self, promotion_target: Piece) -> Result<GameState, ChessError> {
        let (pawn_position, piece, move_type) = match &mut self.state {
            GameState::Promotion(position, piece, move_type) => (position, piece, move_type),
            _ => return Err(ChessError::InvalidMove),
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
        piece: &mut Color<Piece>,
        initial_position: &BoardPosition,
    ) -> Result<(), ChessError> {
        let king_position = self.get_king_position(&self.turn).clone();
        let king = self
            .board
            .get_mut(&king_position)
            .as_mut()
            .ok_or(ChessError::InternalError)?;

        let castling_state = if matches!(piece.get_internal(), Piece::King { .. }) {
            piece.change_internal(Piece::King {
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
                    castling_state: (castling_state.0, false),
                }),
                BoardPosition { file: H, rank: One } => king.change_internal(Piece::King {
                    check_state: None,
                    castling_state: (false, castling_state.1),
                }),
                BoardPosition {
                    file: A,
                    rank: Eight,
                } => king.change_internal(Piece::King {
                    check_state: None,
                    castling_state: (castling_state.0, false),
                }),
                BoardPosition {
                    file: H,
                    rank: Eight,
                } => king.change_internal(Piece::King {
                    check_state: None,
                    castling_state: (false, castling_state.1),
                }),
                _ => {}
            }
        }

        Ok(())
    }
}
