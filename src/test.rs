#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::board::ChessGame;
    use crate::board::MoveType;
    use crate::piece::CastlingState;
    use crate::piece::CheckState;
    use crate::piece::PawnState;
    use crate::piece::{ColouredPiece, Piece};
    use crate::position::{BoardPosition, Horizontal::*, Vertical::*};

    #[test]
    fn valid_moves_correct() {
        let mut board = Board::default();

        board.set_index(&BoardPosition::from((A, One)), Some(ColouredPiece::White(Piece::Rook)));

        let mut game = ChessGame::default();
        game.board = board.clone();
        

        let valid_moves = game
            .get_valid_moves(&BoardPosition::from((A, One)))
            .unwrap();

        assert!(matches!(
            valid_moves.get_index(&BoardPosition::from((A, Three))),
            Some(MoveType::Move)
        ));

        board.set_index(&BoardPosition::from((A, Eight)), Some(ColouredPiece::Black(Piece::Rook)));

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
            Some(ColouredPiece::White(Piece::Rook))
        ));
        assert!(matches!(
            game.get_index(&BoardPosition::from((B, One))),
            Some(ColouredPiece::White(Piece::Knight))
        ));
        assert!(matches!(
            game.get_index(&BoardPosition::from((C, One))),
            Some(ColouredPiece::White(Piece::Bishop))
        ));
        assert!(matches!(
            game.get_index(&BoardPosition::from((D, One))),
            Some(ColouredPiece::White(Piece::Queen))
        ));
        assert!(matches!(
            game.get_index(&BoardPosition::from((E, One))),
            Some(ColouredPiece::White(Piece::King { check_state: CheckState::None, castling_state: CastlingState::Castling }))
        ));
        assert!(matches!(
            game.get_index(&BoardPosition::from((E, Two))),
            Some(ColouredPiece::White(Piece::Pawn { state: PawnState::FirstMove }))
        ));
        assert!(game.get_index(&BoardPosition::from((E, Three))).is_none());
        assert!(game.get_index(&BoardPosition::from((E, Six))).is_none());
        assert!(matches!(
            game.get_index(&BoardPosition::from((E, Seven))),
            Some(ColouredPiece::Black(Piece::Pawn { state: PawnState::FirstMove }))
        ));
    }
}
