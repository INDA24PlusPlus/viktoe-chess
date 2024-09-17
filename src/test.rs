#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::board::ChessGame;
    use crate::board::MoveType;
    use crate::piece::CastlingState;
    use crate::piece::PawnState;
    use crate::piece::BLACK_ROOK;
    use crate::piece::NEW_BLACK_KING;
    use crate::piece::NEW_WHITE_KING;
    use crate::piece::NEW_WHITE_PAWN;
    use crate::piece::WHITE_ROOK;
    use crate::piece::{Color, Piece};
    use crate::position::{BoardPosition, File::*, Rank::*};

    #[test]
    fn valid_moves_correct() {
        let mut board = Board::default();

        board.set_index(&BoardPosition::from((A, One)), Some(WHITE_ROOK));
        board.set_index(&BoardPosition::from((E, One)), Some(NEW_WHITE_KING));
        board.set_index(&BoardPosition::from((E, Eight)), Some(NEW_BLACK_KING));

        let mut game = ChessGame::default();
        game.board = board.clone();
        

        let valid_moves = game
            .get_valid_moves(&BoardPosition::from((A, One)))
            .unwrap();

        assert!(matches!(
            valid_moves.get_index(&BoardPosition::from((A, Three))),
            Some(MoveType::Move)
        ));

        board.set_index(&BoardPosition::from((A, Eight)), Some(BLACK_ROOK));

        game.board = board;

        let valid_moves = game
            .get_valid_moves(&BoardPosition::from((A, One)))
            .unwrap();

        assert!(matches!(valid_moves.get_index(&BoardPosition::from((A, Eight))), Some(MoveType::Capture)));
    }

    #[test]
    fn board_ititialised_correct() {
        let game = ChessGame::default();

        assert!(matches!(
            game.get_index(&BoardPosition::from((A, One))),
            Some(Color::White(Piece::Rook))
        ));
        assert!(matches!(
            game.get_index(&BoardPosition::from((B, One))),
            Some(Color::White(Piece::Knight))
        ));
        assert!(matches!(
            game.get_index(&BoardPosition::from((C, One))),
            Some(Color::White(Piece::Bishop))
        ));
        assert!(matches!(
            game.get_index(&BoardPosition::from((D, One))),
            Some(Color::White(Piece::Queen))
        ));
        assert!(matches!(
            game.get_index(&BoardPosition::from((E, One))),
            Some(Color::White(Piece::King { check_state: None, castling_state: (true, true) }))
        ));
        assert!(matches!(
            game.get_index(&BoardPosition::from((E, Two))),
            Some(Color::White(Piece::Pawn { state: PawnState::FirstMove }))
        ));
        assert!(game.get_index(&BoardPosition::from((E, Three))).is_none());
        assert!(game.get_index(&BoardPosition::from((E, Six))).is_none());
        assert!(matches!(
            game.get_index(&BoardPosition::from((E, Seven))),
            Some(Color::Black(Piece::Pawn { state: PawnState::FirstMove }))
        ));
    }
}
