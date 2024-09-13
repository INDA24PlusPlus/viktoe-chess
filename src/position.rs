use std::u8;

use crate::ChessError;

#[derive(Debug, Clone)]
pub struct BoardPosition {
    file: Horizontal,
    rank: Vertical,
}

#[derive(Debug, Clone)]
pub enum Horizontal {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Debug, Clone)]
pub enum Vertical {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl From<Horizontal> for u8 {
    fn from(value: Horizontal) -> Self {
        u8::from(&value)
    }
}

impl From<&Horizontal> for u8 {
    fn from(value: &Horizontal) -> Self {
        match value {
            Horizontal::A => 0,
            Horizontal::B => 1,
            Horizontal::C => 2,
            Horizontal::D => 3,
            Horizontal::E => 4,
            Horizontal::F => 5,
            Horizontal::G => 6,
            Horizontal::H => 7,
        }
    }
}

impl TryFrom<u8> for Horizontal {
    type Error = ChessError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Horizontal::A),
            1 => Ok(Horizontal::B),
            2 => Ok(Horizontal::C),
            3 => Ok(Horizontal::D),
            4 => Ok(Horizontal::E),
            5 => Ok(Horizontal::F),
            6 => Ok(Horizontal::G),
            7 => Ok(Horizontal::H),
            _ => Err(ChessError::OutOfBounds),
        }
    }
}

impl From<Vertical> for u8 {
    fn from(value: Vertical) -> Self {
        u8::from(&value)
    }
}

impl From<&Vertical> for u8 {
    fn from(value: &Vertical) -> Self {
        match value {
            Vertical::One => 0,
            Vertical::Two => 1,
            Vertical::Three => 2,
            Vertical::Four => 3,
            Vertical::Five => 4,
            Vertical::Six => 5,
            Vertical::Seven => 6,
            Vertical::Eight => 7,
        }
    }
}

impl TryFrom<u8> for Vertical {
    type Error = ChessError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Vertical::One),
            1 => Ok(Vertical::Two),
            2 => Ok(Vertical::Three),
            3 => Ok(Vertical::Four),
            4 => Ok(Vertical::Five),
            5 => Ok(Vertical::Six),
            6 => Ok(Vertical::Seven),
            7 => Ok(Vertical::Eight),
            _ => Err(ChessError::OutOfBounds),
        }
    }
}

impl From<BoardPosition> for (u8, u8) {
    fn from(value: BoardPosition) -> Self {
        let x = u8::from(value.file);
        let y = u8::from(value.rank);

        (x, y)
    }
}

impl From<&BoardPosition> for (u8, u8) {
    fn from(value: &BoardPosition) -> Self {
        let x = u8::from(&value.file);
        let y = u8::from(&value.rank);

        (x, y)
    }
}

impl TryFrom<(u8, u8)> for BoardPosition {
    type Error = ChessError;

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        let file = Horizontal::try_from(value.0)?;
        let rank = Vertical::try_from(value.1)?;

        Ok(BoardPosition { file, rank })
    }
}

impl From<(Horizontal, Vertical)> for BoardPosition {
    fn from(value: (Horizontal, Vertical)) -> Self {
        BoardPosition {
            file: value.0,
            rank: value.1,
        }
    }
}

impl BoardPosition {
    pub fn add(&self, vector: (i8, i8)) -> Result<Self, ChessError> {
        let (file, rank): (u8, u8) = self.into();

        let (file_delta, rank_delta) = vector;

        let new_file = if file_delta >= 0 {
            file.checked_add(file_delta.unsigned_abs())
        } else {
            file.checked_sub(file_delta.unsigned_abs())
        }
        .ok_or(ChessError::OutOfBounds)?;

        let new_rank = if rank_delta >= 0 {
            rank.checked_add(rank_delta.unsigned_abs())
        } else {
            rank.checked_sub(rank_delta.unsigned_abs())
        }
        .ok_or(ChessError::OutOfBounds)?;

        BoardPosition::try_from((new_file, new_rank))
    }
}

