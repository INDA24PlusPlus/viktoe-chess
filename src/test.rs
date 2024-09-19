// #[cfg(test)]
// mod tests {
//     use crate::position::{BoardPosition, File::*, Rank::*};
//     use crate::piece::shorthands::*;
//     use crate::piece::{Color, Piece, PawnState};
//     use crate::board::{ChessGame, Board, MoveType};
//
//     #[test]
//     fn valid_moves_correct() {
//         let mut board = Board::default();
//
//         board.set(&BoardPosition::from((A, One)), Some(WHITE_ROOK));
//         board.set(&BoardPosition::from((E, One)), Some(NEW_WHITE_KING));
//         board.set(&BoardPosition::from((E, Eight)), Some(NEW_BLACK_KING));
//
//         let mut game = ChessGame::default();
//         game.board = board.clone();
//         
//
//         let valid_moves = game
//             .get_valid_moves(&BoardPosition::from((A, One)))
//             .unwrap();
//
//         assert!(matches!(
//             valid_moves.get(&BoardPosition::from((A, Three))),
//             Some(MoveType::Move)
//         ));
//
//         board.set(&BoardPosition::from((A, Eight)), Some(BLACK_ROOK));
//
//         game.board = board;
//
//         let valid_moves = game
//             .get_valid_moves(&BoardPosition::from((A, One)))
//             .unwrap();
//
//         assert!(matches!(valid_moves.get(&BoardPosition::from((A, Eight))), Some(MoveType::Capture)));
//     }
//
//     #[test]
//     fn board_ititialised_correct() {
//         let game = ChessGame::default();
//
//         assert!(matches!(
//             game.get_square(&BoardPosition::from((A, One))),
//             Some(Color::White(Piece::Rook))
//         ));
//         assert!(matches!(
//             game.get_square(&BoardPosition::from((B, One))),
//             Some(Color::White(Piece::Knight))
//         ));
//         assert!(matches!(
//             game.get_square(&BoardPosition::from((C, One))),
//             Some(Color::White(Piece::Bishop))
//         ));
//         assert!(matches!(
//             game.get_square(&BoardPosition::from((D, One))),
//             Some(Color::White(Piece::Queen))
//         ));
//         assert!(matches!(
//             game.get_square(&BoardPosition::from((E, One))),
//             Some(Color::White(Piece::King { check_state: None, castling_state: (true, true) }))
//         ));
//         assert!(matches!(
//             game.get_square(&BoardPosition::from((E, Two))),
//             Some(Color::White(Piece::Pawn { state: PawnState::FirstMove }))
//         ));
//         assert!(game.get_square(&BoardPosition::from((E, Three))).is_none());
//         assert!(game.get_square(&BoardPosition::from((E, Six))).is_none());
//         assert!(matches!(
//             game.get_square(&BoardPosition::from((E, Seven))),
//             Some(Color::Black(Piece::Pawn { state: PawnState::FirstMove }))
//         ));
//     }
// }
