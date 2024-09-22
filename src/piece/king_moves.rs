use crate::position::{BoardPosition, File::*, Rank::*};
use crate::piece::{Color, Piece};
use crate::board::{Board, MoveType, Turn};
use crate::board::check::is_in_check;

use super::CastlingState;

pub fn get_king_moves(
    board: &Board<Color<Piece>>,
    position: &BoardPosition,
    piece: &Color<Piece>,
    castling_state: &CastlingState,
    player_color: &Turn,
) -> Board<MoveType> {
    let mut move_map = piece.get_standard_valid_move(board, position, position, player_color);

    if castling_state.0 {
        get_king_side_castle(board, &mut move_map, piece);
    }
    if castling_state.1 {
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
            if board.get(&BoardPosition::from((F, One))).is_some() {
                return;
            }
            if board.get(&BoardPosition::from((G, One))).is_some() {
                return;
            }

            if is_in_check(board, &BoardPosition::from((F, One)), &Turn::White) {
                return
            }
            if is_in_check(board, &BoardPosition::from((G, One)), &Turn::White) {
                return
            }

            move_map.set(&BoardPosition::from((G, One)), Some(MoveType::Move))
        }
        Color::Black(_) => {
            if board.get(&BoardPosition::from((F, Eight))).is_some() {
                return;
            }
            if board.get(&BoardPosition::from((G, Eight))).is_some() {
                return;
            }

            if is_in_check(board, &BoardPosition::from((F, Eight)), &Turn::Black) {
                return
            }
            if is_in_check(board, &BoardPosition::from((G, Eight)), &Turn::Black) {
                return
            }
            
            move_map.set(&BoardPosition::from((G, Eight)), Some(MoveType::Move))
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
            if board.get(&BoardPosition::from((B, One))).is_some() {
                return;
            }
            if board.get(&BoardPosition::from((C, One))).is_some() {
                return;
            }
            if board.get(&BoardPosition::from((D, One))).is_some() {
                return;
            }

            if is_in_check(board, &BoardPosition::from((B, One)), &Turn::White) {
                return
            }
            if is_in_check(board, &BoardPosition::from((C, One)), &Turn::White) {
                return
            }
            if is_in_check(board, &BoardPosition::from((D, One)), &Turn::White) {
                return
            }

            move_map.set(&BoardPosition::from((B, One)), Some(MoveType::Move))
        }
        Color::Black(_) => {
            if board.get(&BoardPosition::from((B, Eight))).is_some() {
                return;
            }
            if board.get(&BoardPosition::from((C, Eight))).is_some() {
                return;
            }
            if board.get(&BoardPosition::from((B, Eight))).is_some() {
                return;
            }

            if is_in_check(board, &BoardPosition::from((B, Eight)), &Turn::Black) {
                return
            }
            if is_in_check(board, &BoardPosition::from((C, Eight)), &Turn::Black) {
                return
            }
            if is_in_check(board, &BoardPosition::from((D, Eight)), &Turn::Black) {
                return
            }

            move_map.set(&BoardPosition::from((B, Eight)), Some(MoveType::Move))
        }
    }
}
