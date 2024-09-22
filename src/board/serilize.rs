use std::collections::HashMap;

use crate::position::{BoardPosition, File::*, Rank::*};
use crate::piece::shorthands::*;
use crate::piece::{Color, Piece};
use crate::board::{Turn, GameState, Board};
use crate::{ChessError, ChessGame};

impl ChessGame {
    // does not handle check or check mate, castling and en pasant
    pub fn from_fen(fen_string: String) -> Result<Self, ChessError> {
        let mut fen = fen_string.split_whitespace();

        let board_string = fen.next().unwrap();
        let turn_string = fen.next().unwrap();
        let _castle_string = fen.next().unwrap();
        let _en_passant_string = fen.next().unwrap();
        let half_clock_string = fen.next().unwrap();
        let full_move_string = fen.next().unwrap();

        let mut board: Board<Color<Piece>> = Board::default();

        let temp_board = &mut board.board.iter_mut();

        let mut white_king_position = BoardPosition::from((E, One));
        let mut black_king_position = BoardPosition::from((E, Eight));

        for (i, char) in board_string.chars().enumerate() {
            match char {
                '1'..='8' => {
                    temp_board
                        .skip(char.to_digit(10).unwrap() as usize - 1)
                        .next();
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
