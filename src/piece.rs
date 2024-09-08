#[derive(Clone, Copy)]
pub enum ColouredPiece {
    White(Piece),
    Black(Piece),
    None,
}

#[derive(Clone, Copy)]
pub enum Piece {
    King{ movement_base_vector: [(i8, i8); 8], movement_steps: StepCount, check_state: CheckState, castling_state: CastlingState },
    Queen{ movement_base_vector: [(i8, i8); 8], movement_steps: StepCount },
    Rook{ movement_base_vector: [(i8, i8); 4], movement_steps: StepCount },
    Bishop{ movement_base_vector: [(i8, i8); 4], movement_steps: StepCount },
    Knight{ movement_base_vector: [(i8, i8); 8], movement_steps: StepCount },
    Pawn{ movement_base_vector: [(i8, i8); 1], capture_base_vector: [(i8, i8); 2], movement_steps: StepCount, state: PawnState},
}

impl Piece {
    pub fn new_king() -> Self {
        Piece::King {
            movement_base_vector: [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)],
            movement_steps: StepCount::One,
            check_state: CheckState::None,
            castling_state: CastlingState::Castling,
        }
    }

    pub fn new_queen() -> Self {
        Piece::Queen {
            movement_base_vector: [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)],
            movement_steps: StepCount::Infinty,
        }
    }

    pub fn new_rook() -> Self {
        Piece::Rook {
            movement_base_vector: [(1, 0), (0, 1), (-1, 0), (0, -1)],
            movement_steps: StepCount::Infinty,
        }
    }

    pub fn new_bishop() -> Self {
        Piece::Bishop {
            movement_base_vector: [(1, 1), (-1, 1), (-1, -1), (1, -1)],
            movement_steps: StepCount::Infinty,
        }
    }

    pub fn new_knight() -> Self {
        Piece::Knight {
            movement_base_vector: [(-1, 2), (1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1)],
            movement_steps: StepCount::One,
        }
    }
}

impl Piece {
    pub fn new_white_pawn() -> Self {
        Piece::Pawn {
            movement_base_vector: [(0, 1)],
            capture_base_vector: [(-1, 1), (1, 1)],
            movement_steps: StepCount::One,
            state: PawnState::FirstMove,
        }
    }

    pub fn new_black_pawn() -> Self {
        Piece::Pawn {
            movement_base_vector: [(0, -1)],
            capture_base_vector: [(-1, -1), (1, -1)],
            movement_steps: StepCount::One,
            state: PawnState::FirstMove,
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
    Infinty,
}
