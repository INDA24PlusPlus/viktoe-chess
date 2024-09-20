use std::collections::HashMap;

use crate::board::{Board, ChessGame, GameState, Turn};
use crate::piece::shorthands::*;
use crate::position::{BoardPosition, File::*, Rank::*, FILE};

// The repeated None is to avoid implementing Copy on nearly every struct and enum which would risk
// a lot of unnessesary deep copying
impl<T> Default for Board<T> {
    fn default() -> Board<T> {
        Board { board: [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None] }
    }
}

impl Default for ChessGame {
    fn default() -> Self {
        let mut board = Board::default();

        board.set(&BoardPosition::from((A, One)), Some(WHITE_ROOK));
        board.set(&BoardPosition::from((H, One)), Some(WHITE_ROOK));

        board.set(&BoardPosition::from((B, One)), Some(WHITE_KNIGHT));
        board.set(&BoardPosition::from((G, One)), Some(WHITE_KNIGHT));

        board.set(&BoardPosition::from((C, One)), Some(WHITE_BISHOP));
        board.set(&BoardPosition::from((F, One)), Some(WHITE_BISHOP));

        board.set(&BoardPosition::from((D, One)), Some(WHITE_QUEEN));
        board.set(&BoardPosition::from((E, One)), Some(NEW_WHITE_KING));

        let white_king_position = BoardPosition::from((E, One));

        for file in FILE.into_iter() {
            board.set(
                &BoardPosition::from((file, Two)),
                Some(NEW_WHITE_PAWN),
            );
        }

        board.set(&BoardPosition::from((A, Eight)), Some(BLACK_ROOK));
        board.set(&BoardPosition::from((H, Eight)), Some(BLACK_ROOK));

        board.set(&BoardPosition::from((B, Eight)), Some(BLACK_KNIGHT));
        board.set(&BoardPosition::from((G, Eight)), Some(BLACK_KNIGHT));

        board.set(&BoardPosition::from((C, Eight)), Some(BLACK_BISHOP));
        board.set(&BoardPosition::from((F, Eight)), Some(BLACK_BISHOP));

        board.set(&BoardPosition::from((D, Eight)), Some(BLACK_QUEEN));
        board.set(&BoardPosition::from((E, Eight)), Some(NEW_BLACK_KING));

        let black_king_position = BoardPosition::from((E, Eight));

        for file in FILE.into_iter() {
            board.set(
                &BoardPosition::from((file, Seven)),
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
