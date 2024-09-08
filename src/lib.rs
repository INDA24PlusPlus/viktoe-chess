struct ChessGame {
    board: [ColouredPiece; 64],
    turn: Turn,
}

enum Turn {
    White,
    Black,
}

enum ColouredPiece {
    White(Piece),
    Black(Piece),
}

enum Piece {
    King{ movement_base_vector: [(i8, i8); 8], movement_steps: StepCount, state: KingState },
    Queen{ movement_base_vector: [(i8, i8); 8], movement_steps: StepCount },
    Rook{ movement_base_vector: [(i8, i8); 4], movement_steps: StepCount },
    Bishop{ movement_base_vector: [(i8, i8); 4], movement_steps: StepCount },
    Knight{ movement_base_vector: [(i8, i8); 8], movement_steps: StepCount },
    Pawn{ movement_base_vector: [(i8, i8); 1], capture_base_vector: [(i8, i8); 2], movement_steps: StepCount, state: PawnState},
}

enum PawnState {
    FirstMove,
    PosibleEnPassant,
    Default,
}

enum KingState {
    CheckMate,
    Check,
    Castling,
    CastlingCheck,
    CastlingKingSide,
    CastlingQueenSide,
}

enum StepCount {
    One,
    Infinty,
}

enum Horizontal {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

enum Vertical {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

