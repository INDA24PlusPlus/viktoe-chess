#[derive(Clone, Copy)]
pub enum ColouredPiece {
    White(Piece),
    Black(Piece),
    None,
}

#[derive(Clone, Copy)]
pub enum Piece {
    King{ check_state: CheckState, castling_state: CastlingState },
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn{ state: PawnState},
}

impl Piece {
    pub fn new_king() -> Self {
        Piece::King {
            check_state: CheckState::None,
            castling_state: CastlingState::Castling,
        }
    }

    pub fn new_white_pawn() -> Self {
        Piece::Pawn {
            state: PawnState::FirstMove,
        }
    }

    pub fn new_black_pawn() -> Self {
        Piece::Pawn {
            state: PawnState::FirstMove,
        }
    }
}

impl ColouredPiece {
    pub fn get_movement_base_vector(self) -> Option<Vec<(i8, i8)>> {
        match self {
            ColouredPiece::Black(piece) | ColouredPiece::White(piece) => {
                Some(match piece {
                    Piece::King{..} => vec![(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)],
                    Piece::Queen => vec![(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)],
                    Piece::Rook => vec![(0, 1), (1, 0), (0, -1), (-1, 0)],
                    Piece::Bishop => vec![(1, 1), (1, -1), (-1, -1), (-1, 1)],
                    Piece::Knight => vec![(-1, 2), (1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1)],
                    Piece::Pawn{..} => {
                        if matches!(self, ColouredPiece::White(_)) {
                            vec![(0, 1)]
                        }
                        else {
                            vec![(0, -1)]
                        }
                    }
                    
                })
            },
            ColouredPiece::None => None,
        }
    }

    pub fn get_number_of_moves(self) -> Option<StepCount> {
        match self {
            ColouredPiece::White(piece)| ColouredPiece::Black(piece) => {
                Some(match piece {
                    Piece::King {..} => StepCount::One,
                    Piece::Queen => StepCount::Infinty,
                    Piece::Rook => StepCount::Infinty,
                    Piece::Bishop => StepCount::Infinty,
                    Piece::Knight => StepCount::One,
                    Piece::Pawn{ state: PawnState::FirstMove } => StepCount::Two,
                    Piece::Pawn{..} => StepCount::One,
                })
            },
            ColouredPiece::None => None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum PawnState {
    FirstMove,
    PosibleEnPassant,
    Default,
}

#[derive(Clone, Copy)]
pub enum CheckState {
    CheckMate,
    Check,
    None,
}

#[derive(Clone, Copy)]
pub enum CastlingState {
    Castling,
    CastlingKingSide,
    CastlingQueenSide,
}

#[derive(Clone, Copy)]
pub enum StepCount {
    One,
    Two,
    Infinty,
}
