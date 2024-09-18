use crate::ChessError;

use File::*;
use Rank::*;

#[derive(Debug, Clone, PartialEq)]
pub struct BoardPosition {
    pub file: File,
    pub rank: Rank,
}

#[derive(Debug, Clone, PartialEq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

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
        let file = File::try_from(value.0)?;
        let rank = Rank::try_from(value.1)?;

        Ok(BoardPosition { file, rank })
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

#[cfg(test)]
mod tests {
    
    use super::*;
    use super::Rank::*;
    use super::File::*;

    #[test]
    fn convertions_are_correct() {
        let pos: Vec<BoardPosition> = vec![(A, One).into(), (A, Two).into(), (A, Three).into(), (A, Four).into(), (A, Five).into(), (A, Six).into(), (A, Seven).into(), (A, Eight).into()];

        for (i, pos) in pos.iter().enumerate() {
            assert_eq!(*pos, BoardPosition::try_from((0, i as u8)).unwrap());
        }

        let pos: Vec<BoardPosition> = vec![(A, One).into(), (B, One).into(), (C, One).into(), (D, One).into(), (E, One).into(), (F, One).into(), (G, One).into(), (H, One).into()];

        for (i, pos) in pos.iter().enumerate() {
            assert_eq!(*pos, BoardPosition::try_from((i as u8, 0)).unwrap());
        }
    }
}
