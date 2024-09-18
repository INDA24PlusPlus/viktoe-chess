use std::collections::HashMap;

use crate::piece::{
    self, evaluate_vector, get_king_moves, get_pawn_moves, get_standard_valid_move, CheckState,
    Color, PawnState, Piece, StepCount, BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN,
    BLACK_QUEEN, BLACK_ROOK, NEW_BLACK_KING, NEW_BLACK_PAWN, NEW_WHITE_KING, NEW_WHITE_PAWN,
    WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
};
use crate::position::{BoardPosition, File::*, Rank::*};
use crate::ChessError;

#[derive(Clone, Copy)]
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
    Promotion(BoardPosition, Board<MoveType>),
}

#[derive(Debug, Clone, Copy)]
pub enum MoveType {
    Move,
    Capture,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Board<T> {
    board: [Option<T>; 64],
}

impl<T> Board<T> {
    pub fn get_index(&self, position: &BoardPosition) -> &Option<T> {
        let (file, rank) = position.into();

        &self.board[(7 - usize::from(file)) + usize::from(rank) * 8]
    }

    pub fn set_index(&mut self, position: &BoardPosition, value: Option<T>) {
        let (file, rank) = position.into();

        self.board[(7 - usize::from(file)) + usize::from(rank) * 8] = value;
    }
}

impl<T> Default for Board<T>
where
    T: Copy,
{
    fn default() -> Board<T> {
        Board { board: [None; 64] }
    }
}

impl ChessGame {
    // does not handle check or check mate, castling and en pasant
    pub fn from_fen(fen_string: String) -> Result<Self, ChessError> {
        let mut fen = fen_string.split_whitespace();

        let board_string = fen.next().unwrap();
        let turn_string = fen.next().unwrap();
        let castle_string = fen.next().unwrap();
        let en_passant_string = fen.next().unwrap();
        let half_clock_string = fen.next().unwrap();
        let full_move_string = fen.next().unwrap();

        let mut board: Board<Color<Piece>> = Board::default();

        let temp_board = &mut board.board.iter_mut();

        let mut white_king_position = BoardPosition::from((E, One));
        let mut black_king_position = BoardPosition::from((E, Eight));

        for (i, char) in board_string.chars().enumerate() {
            match char {
                '1'..='8' => {
                    temp_board.skip(char.to_digit(10).unwrap() as usize);
                }
                'k' => {
                    *temp_board.next().unwrap() = {
                        black_king_position =
                            BoardPosition::try_from(((i / 8) as u8, (i % 8) as u8)).unwrap();
                        Some(BLACK_KING)
                    }
                }
                'q' => *temp_board.next().unwrap() = Some(BLACK_QUEEN),
                'b' => *temp_board.next().unwrap() = Some(BLACK_BISHOP),
                'n' => *temp_board.next().unwrap() = Some(BLACK_KNIGHT),
                'r' => *temp_board.next().unwrap() = Some(BLACK_ROOK),
                'p' => *temp_board.next().unwrap() = Some(BLACK_PAWN),
                'K' => {
                    *temp_board.next().unwrap() = {
                        white_king_position =
                            BoardPosition::try_from(((i / 8) as u8, (i % 8) as u8)).unwrap();
                        Some(WHITE_KING)
                    }
                }
                'Q' => *temp_board.next().unwrap() = Some(WHITE_QUEEN),
                'B' => *temp_board.next().unwrap() = Some(WHITE_BISHOP),
                'N' => *temp_board.next().unwrap() = Some(WHITE_KNIGHT),
                'R' => *temp_board.next().unwrap() = Some(WHITE_ROOK),
                'P' => *temp_board.next().unwrap() = Some(WHITE_PAWN),
                _ => return Err(ChessError::IncorrectFenString),
            }
        }

        let turn = match turn_string {
            "w" => Turn::White,
            "b" => Turn::Black,
            _ => return Err(ChessError::IncorrectFenString),
        };

        let half_move = half_clock_string.parse().unwrap();
        let full_move = full_move_string.parse().unwrap();

        let state = GameState::Ongoing;

        let en_passant = Vec::new();

        // Cannot handle earlier positions
        let white_possition_history = HashMap::new();
        let black_possition_history = HashMap::new();

        Ok(Self {
            board,
            turn,
            en_passant,
            white_king_position,
            black_king_position,
            half_move,
            full_move,
            state,
            white_possition_history,
            black_possition_history,
        })
    }
}

pub struct ChessGame {
    pub board: Board<Color<Piece>>,
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
            half_move: 0,
            full_move: 0,
            white_possition_history: HashMap::new(),
            black_possition_history: HashMap::new(),
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

        // Check if move is valid
        let moves = self
            .get_valid_moves(initial_position)
            .ok_or(ChessError::NoMoves)?;

        moves
            .get_index(desired_position)
            .ok_or(ChessError::InvalidMove)?;

        // Update list of pawn that can be taken using en passant
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

        // Remove castling options if appliceble
        let king_position = match self.turn {
            Turn::White => &self.white_king_position,
            Turn::Black => &self.black_king_position,
        };

        if matches!(piece, WHITE_ROOK | BLACK_ROOK) {
            match self.get_index(king_position) {
                Some(Color::White(Piece::King {
                    castling_state: (true, queen_side),
                    ..
                })) if *initial_position == BoardPosition::from((H, One)) => self.board.set_index(
                    king_position,
                    Some(Color::White(Piece::King {
                        check_state: None,
                        castling_state: (false, *queen_side),
                    })),
                ),
                Some(Color::White(Piece::King {
                    castling_state: (king_side, true),
                    ..
                })) if *initial_position == BoardPosition::from((A, One)) => self.board.set_index(
                    king_position,
                    Some(Color::White(Piece::King {
                        check_state: None,
                        castling_state: (*king_side, false),
                    })),
                ),
                Some(Color::Black(Piece::King {
                    castling_state: (true, queen_side),
                    ..
                })) if *initial_position == BoardPosition::from((A, Eight)) => {
                    self.board.set_index(
                        king_position,
                        Some(Color::Black(Piece::King {
                            check_state: None,
                            castling_state: (false, *queen_side),
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
                            castling_state: (*king_side, false),
                        })),
                    )
                }
                _ => {}
            }
        }

        // Performe move
        self.board.set_index(initial_position, None);
        self.board.set_index(desired_position, Some(piece));

        if matches!(
            piece,
            Color::White(Piece::Pawn { .. }) | Color::Black(Piece::Pawn { .. })
        ) && ((matches!(self.turn, Turn::White) && matches!(desired_position.rank, Eight))
            | (matches!(self.turn, Turn::Black) && matches!(desired_position.rank, One)))
        {
            Ok(GameState::Promotion(desired_position.clone(), moves.clone()))
        } else {
            Ok(self.progress_turn(&piece, &moves, desired_position))
        }
    }

    fn progress_turn(
        &mut self,
        piece: &Color<Piece>,
        moves: &Board<MoveType>,
        desired_position: &BoardPosition,
    ) -> GameState {
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
        if matches!(moves.get_index(desired_position), Some(MoveType::Capture))
            || matches!(
                piece,
                Color::White(Piece::Pawn { .. }) | Color::Black(Piece::Pawn { .. })
            )
        {
            self.half_move = 0;
        } else {
            self.half_move += 1;
        }

        let king_position = match self.turn {
            Turn::White => &self.white_king_position,
            Turn::Black => &self.black_king_position,
        };

        if is_in_check(&self.board, king_position, self.turn) {
            GameState::Check
        } else if self.half_move >= 100 {
            GameState::Draw
        } else {
            GameState::Ongoing
        }
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
        pawn_position: &BoardPosition,
        promotion_target: Piece,
        moves: &Board<MoveType>,
    ) -> Result<GameState, ChessError> {
        let piece = self.get_index(pawn_position).unwrap();

        if matches!(piece, Color::White(Piece::Pawn { .. }))
            && (matches!(self.turn, Turn::White) && matches!(pawn_position.rank, Eight))
        {
            self.board
                .set_index(pawn_position, Some(Color::White(promotion_target)))
        } else if matches!(piece, Color::Black(Piece::Pawn { .. }))
            && (matches!(self.turn, Turn::Black) && matches!(pawn_position.rank, One))
        {
            self.board
                .set_index(pawn_position, Some(Color::Black(promotion_target)))
        }

        Ok(self.progress_turn(&piece, moves, pawn_position))
    }

    pub fn get_player_turn(&self) -> &Turn {
        &self.turn
    }

    pub fn get_game_state(&self) -> &GameState {
        &self.state
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
