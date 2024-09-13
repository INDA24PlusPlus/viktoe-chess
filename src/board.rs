
use crate::piece::{evaluate_vector, get_king_moves, get_pawn_moves, get_standard_valid_move, ColouredPiece, Piece};
use crate::position::{self, BoardPosition, Horizontal::*, Vertical::*};
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
        }
    }
}

pub struct ChessGame {
    pub board: Board<ColouredPiece<Piece>>,
    turn: Turn,
    state: GameState,
    white_king_position: BoardPosition,
    black_king_position: BoardPosition,
}

impl Default for ChessGame {
    fn default() -> Self {
        let mut board = Board::default();

        board.set_index(
            &BoardPosition::from((A, One)),
            Some(ColouredPiece::White(Piece::Rook)),
        );
        board.set_index(
            &BoardPosition::from((H, One)),
            Some(ColouredPiece::White(Piece::Rook)),
        );

        board.set_index(
            &BoardPosition::from((B, One)),
            Some(ColouredPiece::White(Piece::Knight)),
        );
        board.set_index(
            &BoardPosition::from((G, One)),
            Some(ColouredPiece::White(Piece::Knight)),
        );

        board.set_index(
            &BoardPosition::from((C, One)),
            Some(ColouredPiece::White(Piece::Bishop)),
        );
        board.set_index(
            &BoardPosition::from((F, One)),
            Some(ColouredPiece::White(Piece::Bishop)),
        );

        board.set_index(
            &BoardPosition::from((D, One)),
            Some(ColouredPiece::White(Piece::Queen)),
        );
        board.set_index(
            &BoardPosition::from((E, One)),
            Some(ColouredPiece::White(Piece::new_king())),
        );

        let white_king_position = BoardPosition::from((E, One));

        for file in 0..8 {
            board.set_index(
                &BoardPosition::try_from((file, 1)).unwrap(),
                Some(ColouredPiece::White(Piece::new_white_pawn())),
            );
        }

        board.set_index(
            &BoardPosition::from((A, Eight)),
            Some(ColouredPiece::Black(Piece::Rook)),
        );
        board.set_index(
            &BoardPosition::from((H, Eight)),
            Some(ColouredPiece::Black(Piece::Rook)),
        );

        board.set_index(
            &BoardPosition::from((B, Eight)),
            Some(ColouredPiece::Black(Piece::Knight)),
        );
        board.set_index(
            &BoardPosition::from((G, Eight)),
            Some(ColouredPiece::Black(Piece::Knight)),
        );

        board.set_index(
            &BoardPosition::from((C, Eight)),
            Some(ColouredPiece::Black(Piece::Bishop)),
        );
        board.set_index(
            &BoardPosition::from((F, Eight)),
            Some(ColouredPiece::Black(Piece::Bishop)),
        );

        board.set_index(
            &BoardPosition::from((D, Eight)),
            Some(ColouredPiece::Black(Piece::Queen)),
        );
        board.set_index(
            &BoardPosition::from((E, Eight)),
            Some(ColouredPiece::Black(Piece::new_king())),
        );

        let black_king_position = BoardPosition::from((E, Eight));

        for file in 0..8 {
            board.set_index(
                &BoardPosition::try_from((file, 6)).unwrap(),
                Some(ColouredPiece::Black(Piece::new_black_pawn())),
            );
        }

        Self {
            board,
            turn: Turn::White,
            state: GameState::Ongoing,
            white_king_position,
            black_king_position,
        }
    }
}

impl ChessGame {
    pub fn get_index(&self, position: &BoardPosition) -> &Option<ColouredPiece<Piece>> {
        self.board.get_index(position)
    }

    pub fn get_valid_moves(&self, position: &BoardPosition) -> Option<Board<MoveType>> {
        let piece = match self.board.get_index(position) {
            Some(piece) => piece,
            None => return None,
        };

        let piece_type = match piece {
            ColouredPiece::White(piece) => piece,
            ColouredPiece::Black(piece) => piece,
        };

        let king_position = match self.turn {
            Turn::White => &self.white_king_position,
            Turn::Black => &self.black_king_position,
        };

        let colour = match self.turn {
            Turn::White => ColouredPiece::White(()),
            Turn::Black => ColouredPiece::Black(()),
        };

        // if is_in_check(&self.board, king_position, colour) {
        //     return None;
        // }

        Some(match piece_type {
            Piece::King { castling_state, .. } => {
                get_king_moves(&self.board, position, piece, castling_state)
            }
            Piece::Pawn { .. } => get_pawn_moves(&self.board, position, piece),
            _ => get_standard_valid_move(&self.board, piece, position),
        })
    }

    pub fn move_piece(
        &mut self,
        initial_position: &BoardPosition,
        desired_position: &BoardPosition,
    ) -> Result<GameState, ChessError> {
        let piece = *match self.board.get_index(initial_position) {
            Some(piece) => {
                match piece {
                    ColouredPiece::White(_) if matches!(self.turn, Turn::White) => piece,
                    ColouredPiece::Black(_) if matches!(self.turn, Turn::Black) => piece,
                    _ => return Err(ChessError::NoPiece),
                    
                }
            },
            None => return Err(ChessError::NoPiece),
        };

        let moves = self.get_valid_moves(initial_position).ok_or(ChessError::NoMoves)?;

        moves.get_index(desired_position).ok_or(ChessError::InvalidMove)?;


        let king_position = match self.turn {
            Turn::Black => &self.white_king_position,
            Turn::White => &self.black_king_position,
        };

        let colour = match self.turn {
            Turn::Black => ColouredPiece::White(()),
            Turn::White => ColouredPiece::Black(()),
        };

        let mut board = self.board.clone();

        board.set_index(initial_position, None);
        board.set_index(desired_position, Some(piece));

        // if is_in_check(&board, king_position, colour) {
        //     return Err(ChessError::SelfCheck);
        // }
        
        self.board.set_index(initial_position, None);
        self.board.set_index(desired_position, Some(piece));

        let king_position = match self.turn {
            Turn::White => &self.white_king_position,
            Turn::Black => &self.black_king_position,
        };

        let colour = match self.turn {
            Turn::White => ColouredPiece::White(()),
            Turn::Black => ColouredPiece::Black(()),
        };

        self.turn = match self.turn {
            Turn::White => Turn::Black,
            Turn::Black => Turn::White,
        };

        // if is_in_check(&board, king_position, colour) {
        //     Ok(GameState::Check)
        // }
        // else {
        //     Ok(GameState::Ongoing)
        // }
        Ok(GameState::Ongoing)
    }

    pub fn get_player_turn(&self) -> Turn {
        self.turn
    }

    pub fn get_game_state(&self) -> GameState {
        self.state
    }

}

// Does not yet handle king moving to close
fn is_in_check(board: &Board<ColouredPiece<Piece>>, king_position: &BoardPosition, colour: ColouredPiece<()>) -> bool {
    // Check if pawn causes king to be in check
    match colour {
        ColouredPiece::White(_) => {
            if check_by_pawn(board, king_position, (-1, 1), ColouredPiece::Black(Piece::new_black_pawn()))||
                check_by_pawn(board, king_position, (1, 1), ColouredPiece::Black(Piece::new_black_pawn())) {
                return true;
            }
        }
        ColouredPiece::Black(_) => {
            if check_by_pawn(board, king_position, (-1, -1), ColouredPiece::White(Piece::new_white_pawn()))||
                check_by_pawn(board, king_position, (1, -1), ColouredPiece::White(Piece::new_white_pawn())) {
                return true;
            }
        }
    }

    let colour = match colour {
        ColouredPiece::White(_) => ColouredPiece::White(Piece::Queen),
        ColouredPiece::Black(_) => ColouredPiece::Black(Piece::Queen),
        
    };

    let vectors = vec![(ColouredPiece::White(Piece::Rook).get_movement_base_vector(), vec![Piece::Rook, Piece::Queen]), (ColouredPiece::White(Piece::Bishop).get_movement_base_vector(), vec![Piece::Bishop, Piece::Queen]), (ColouredPiece::White(Piece::Knight).get_movement_base_vector(), vec![Piece::Knight])];

    // Check if queen, rook, or bishop causes king to be in check
    if vectors.into_iter().any(|(vectors, pieces)| {
        check_vector(board, king_position, vectors, pieces, colour)
    }) {
        return true;
    }

    false
}

fn check_by_pawn(board: &Board<ColouredPiece<Piece>>, king_position: &BoardPosition, vector: (i8, i8), piece: ColouredPiece<Piece>) -> bool {
    king_position.add(vector).and_then(|position| {
        if board.get_index(&position).unwrap() == piece {
            Ok(position)
        }
        else {
            Err(ChessError::InvalidMove)
        }
    }).is_ok()
}

fn check_vector(board: &Board<ColouredPiece<Piece>>, position: &BoardPosition, move_set: Vec<(i8, i8)>, pieces: Vec<Piece>, colour: ColouredPiece<Piece>) -> bool {

    move_set.into_iter().any(|base_vector| {
        match evaluate_vector(board, &colour, base_vector, position).last() {
            Some((position, move_type)) => {
                return matches!(move_type, MoveType::Capture) &&
                pieces.iter().any(|piece| {check_capture(board, position, *piece, colour)})
            },
            None => false
        }
    })
}

fn check_capture(board: &Board<ColouredPiece<Piece>>, position: &BoardPosition, piece: Piece, colour: ColouredPiece<Piece>) -> bool {
    match colour {
        ColouredPiece::White(_) => {
            board.get_index(position).unwrap() == ColouredPiece::Black(piece)
        }
        ColouredPiece::Black(_) => {
            board.get_index(position).unwrap() == ColouredPiece::White(piece)
        }
    }
}

