mod trait_implementation;

use crate::ChessError;

pub const FILE: [File; 8] = [File::A, File::B, File::C, File::D, File::E, File::F, File::G, File::H];
pub const RANK: [Rank; 8] = [Rank::One, Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight];

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

/// A struct representing a valid position on a chess board
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

    /// Adds a vector to a position returning an error if the result is outside the board.
    ///
    /// # Example
    /// ```rust
    /// use viktoe-chess::prelude::*;
    /// BoardPosition::from((A, One)).add((3, 2)) == BoardPosition::from((D, Three));
    /// ```
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

/// Returns an iterator over each square on the board contianing the position starting in the top
/// left.
pub fn iter() -> Vec<BoardPosition> {
    RANK.into_iter().rev().flat_map(|rank| {
        let rank = rank.clone();
        FILE.into_iter().map( move |file| BoardPosition::from((file, (rank).clone())))
    }).collect()
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

    #[test]
    fn board_possition_iter() {
        assert_eq!(iter().into_iter().next().unwrap(), BoardPosition::from((A, Eight)));
        assert_eq!(iter().into_iter().last().unwrap(), BoardPosition::from((H, One)));
    }
    
}
