mod trait_implementation;

use crate::ChessError;

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

#[derive(Debug, Clone, PartialEq)]
pub struct BoardPosition {
    pub(crate) file: File,
    pub(crate) rank: Rank,
}

impl BoardPosition {
    pub fn get_file(&self) -> &File {
        &self.file
    }

    pub fn get_rank(&self) -> &Rank {
        &self.rank
    }

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
