use crate::ChessError;
use crate::position::{BoardPosition, File, Rank};

// Implementations for File

impl From<File> for u8 {
    fn from(value: File) -> Self {
        u8::from(&value)
    }
}

impl From<&File> for u8 {
    fn from(value: &File) -> Self {
        match value {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        }
    }
}

impl TryFrom<u8> for File {
    type Error = ChessError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(File::A),
            1 => Ok(File::B),
            2 => Ok(File::C),
            3 => Ok(File::D),
            4 => Ok(File::E),
            5 => Ok(File::F),
            6 => Ok(File::G),
            7 => Ok(File::H),
            _ => Err(ChessError::OutOfBounds),
        }
    }
}

// Implementations for Rank

impl From<Rank> for u8 {
    fn from(value: Rank) -> Self {
        u8::from(&value)
    }
}

impl From<&Rank> for u8 {
    fn from(value: &Rank) -> Self {
        match value {
            Rank::One => 0,
            Rank::Two => 1,
            Rank::Three => 2,
            Rank::Four => 3,
            Rank::Five => 4,
            Rank::Six => 5,
            Rank::Seven => 6,
            Rank::Eight => 7,
        }
    }
}

impl TryFrom<u8> for Rank {
    type Error = ChessError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rank::One),
            1 => Ok(Rank::Two),
            2 => Ok(Rank::Three),
            3 => Ok(Rank::Four),
            4 => Ok(Rank::Five),
            5 => Ok(Rank::Six),
            6 => Ok(Rank::Seven),
            7 => Ok(Rank::Eight),
            _ => Err(ChessError::OutOfBounds),
        }
    }
}

// Implementations for BoardPosition

impl From<BoardPosition> for (u8, u8) {
    fn from(value: BoardPosition) -> Self {
        (u8::from(value.file), u8::from(value.rank))

    }
}

impl From<&BoardPosition> for (u8, u8) {
    fn from(value: &BoardPosition) -> Self {
        (u8::from(&value.file), u8::from(&value.rank))
    }
}

impl TryFrom<(u8, u8)> for BoardPosition {
    type Error = ChessError;

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        Ok(BoardPosition {
            file: File::try_from(value.0)?,
            rank: Rank::try_from(value.1)?,
        })
    }
}

impl From<(File, Rank)> for BoardPosition {
    fn from(value: (File, Rank)) -> Self {
        BoardPosition {
            file: value.0,
            rank: value.1,
        }
    }
}
