mod serilize;
mod trait_implementation;
pub(crate) mod check;

use crate::piece::{
    Color, Piece,
};
use crate::position::{BoardPosition, File::*, Rank::*};

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

/// A sturuct containing each square in a game of chess
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Board<T> {
    pub(crate) board: [Option<T>; 64],
}

impl<T> Board<T> {
    pub fn get(&self, position: &BoardPosition) -> &Option<T> {
        let (file, rank) = position.into();

        &self.board[usize::from(file) + (7 - usize::from(rank)) * 8]
    }

    pub(crate) fn get_mut(&mut self, position: &BoardPosition) -> &mut Option<T> {
        let (file, rank) = position.into();

        &mut self.board[usize::from(file) + (7 - usize::from(rank)) * 8]
    }

    pub(crate) fn set(&mut self, position: &BoardPosition, value: Option<T>) {
        let (file, rank) = position.into();

        self.board[usize::from(file) + (7 - usize::from(rank)) * 8] = value;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ChessGame;
    use crate::is_in_check;
    use crate::piece::shorthands::*;

    #[test]
    fn check_test_1() {
        let mut board = Board::default();

        board.set(&BoardPosition::from((E, One)), Some(NEW_WHITE_KING));
        board.set(&BoardPosition::from((E, Eight)), Some(NEW_BLACK_KING));

        assert!(!is_in_check(&board, &(E, One).into(), &Turn::White,));
        assert!(!is_in_check(&board, &(E, Eight).into(), &Turn::Black));
    }

    #[test]
    fn check_test_2() {
        let mut board = Board::default();

        board.set(&BoardPosition::from((E, One)), Some(NEW_WHITE_KING));
        board.set(&BoardPosition::from((E, Eight)), Some(NEW_BLACK_KING));
        board.set(&BoardPosition::from((A, Eight)), Some(WHITE_ROOK));

        assert!(!is_in_check(&board, &(E, One).into(), &Turn::White,));
        assert!(is_in_check(&board, &(E, Eight).into(), &Turn::Black));
    }

    #[test]
    fn check_test_3() {
        let mut board = Board::default();

        board.set(&(E, One).into(), Some(NEW_WHITE_KING));
        board.set(&(D, Two).into(), Some(BLACK_PAWN));

        assert!(is_in_check(&board, &(E, One).into(), &Turn::White));
    }

    #[test]
    fn remove_castling_options_test_1() {
        let mut game = ChessGame::default();

        game.move_piece(&(E, Two).into(), &(E, Four).into())
            .unwrap();
        game.move_piece(&(E, Seven).into(), &(E, Five).into())
            .unwrap();
        game.move_piece(&(E, One).into(), &(E, Two).into()).unwrap();

        let king = game
            .get_square(&(E, Two).into())
            .as_ref()
            .unwrap()
            .get_internal();

        assert!(matches!(
            king,
            Piece::King {
                check_state: _,
                castling_state: (false, false)
            }
        ));
    }

    #[test]
    fn remove_castling_options_test_2() {
        let mut game = ChessGame::default();

        game.move_piece(&(A, Two).into(), &(A, Four).into())
            .unwrap();

        game.move_piece(&(A, Seven).into(), &(A, Five).into())
            .unwrap();

        game.move_piece(&(A, One).into(), &(A, Three).into())
            .unwrap();

        let king = game
            .get_square(&(E, One).into())
            .as_ref()
            .unwrap()
            .get_internal();

        assert!(matches!(
            king,
            Piece::King {
                check_state: _,
                castling_state: (true, false)
            }
        ));
    }

    #[test]
    fn remove_castling_options_test_3() {
        let mut game = ChessGame::default();

        game.move_piece(&(H, Two).into(), &(H, Four).into())
            .unwrap();

        game.move_piece(&(H, Seven).into(), &(H, Five).into())
            .unwrap();

        game.move_piece(&(H, One).into(), &(H, Three).into())
            .unwrap();

        let king = game
            .get_square(&(E, One).into())
            .as_ref()
            .unwrap()
            .get_internal();

        assert!(matches!(
            king,
            Piece::King {
                check_state: _,
                castling_state: (false, true)
            }
        ));
    }

    #[test]
    fn castling_king_side() {
        let mut game = ChessGame::default();

        game.move_piece(&(E, Two).into(), &(E, Four).into())
            .unwrap();
        game.move_piece(&(E, Seven).into(), &(E, Five).into())
            .unwrap();
        game.move_piece(&(F, One).into(), &(E, Two).into()).unwrap();
        game.move_piece(&(F, Seven).into(), &(F, Five).into())
            .unwrap();
        game.move_piece(&(G, One).into(), &(H, Three).into())
            .unwrap();
        game.move_piece(&(H, Seven).into(), &(H, Five).into())
            .unwrap();
        game.move_piece(&(E, One).into(), &(G, One).into()).unwrap();

        assert!(matches!(
            game.get_square(&(G, One).into()),
            Some(WHITE_KING)
        ));
        assert!(matches!(
            game.get_square(&(F, One).into()),
            Some(WHITE_ROOK)
        ));
    }

    #[test]
    fn test_en_passant() {
        let mut game = ChessGame::default();

        game.move_piece(&(E, Two).into(), &(E, Four).into())
            .unwrap();
        game.move_piece(&(D, Seven).into(), &(D, Five).into())
            .unwrap();
        game.move_piece(&(E, Four).into(), &(E, Five).into())
            .unwrap();
        game.move_piece(&(F, Seven).into(), &(F, Five).into())
            .unwrap();
        game.move_piece(&(E, Five).into(), &(F, Six).into())
            .unwrap();
        assert!(game.get_square(&(F, Five).into()).is_none());
    }
}
