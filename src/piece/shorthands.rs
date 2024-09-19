use crate::piece::{Color, Piece, PawnState};

pub const WHITE_KING: Color<Piece> = Color::White(Piece::King {
    check_state: None,
    castling_state: (false, false),
});
pub const NEW_WHITE_KING: Color<Piece> = Color::White(Piece::King {
    check_state: None,
    castling_state: (true, true)
});
pub const WHITE_QUEEN: Color<Piece> = Color::White(Piece::Queen);
pub const WHITE_BISHOP: Color<Piece> = Color::White(Piece::Bishop);
pub const WHITE_KNIGHT: Color<Piece> = Color::White(Piece::Knight);
pub const WHITE_ROOK: Color<Piece> = Color::White(Piece::Rook);
pub const WHITE_PAWN: Color<Piece> = Color::White(Piece::Pawn {
    state: PawnState::Default,
});
pub const NEW_WHITE_PAWN: Color<Piece> = Color::White(Piece::Pawn {
    state: PawnState::FirstMove,
});

pub const BLACK_KING: Color<Piece> = Color::Black(Piece::King {
    check_state: None,
    castling_state: (false, false),
});
pub const NEW_BLACK_KING: Color<Piece> = Color::Black(Piece::King {
    check_state: None,
    castling_state: (true, true),
});
pub const BLACK_QUEEN: Color<Piece> = Color::Black(Piece::Queen);
pub const BLACK_BISHOP: Color<Piece> = Color::Black(Piece::Bishop);
pub const BLACK_KNIGHT: Color<Piece> = Color::Black(Piece::Knight);
pub const BLACK_ROOK: Color<Piece> = Color::Black(Piece::Rook);
pub const BLACK_PAWN: Color<Piece> = Color::Black(Piece::Pawn {
    state: PawnState::Default,
});
pub const NEW_BLACK_PAWN: Color<Piece> = Color::Black(Piece::Pawn {
    state: PawnState::FirstMove,
});

pub const KING_MOVES: [(i8, i8); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];
pub const QUEEN_MOVES: [(i8, i8); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];
pub const ROOK_MOVES: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
pub const BISHOP_MOVES: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
pub const KNIGTH_MOVES: [(i8, i8); 8] = [
    (-1, 2),
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
];
pub const WHITE_PAWN_MOVES: [(i8, i8); 1] = [(0, 1)];
pub const BLACK_PAWN_MOVES: [(i8, i8); 1] = [(0, -1)];
