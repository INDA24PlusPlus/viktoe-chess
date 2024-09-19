use std::ops::Range;

use crate::piece::{Color, StepCount};

impl<T> Default for Color<T>
where
    T: Default,
{
    fn default() -> Self {
        Color::White(T::default())
    }
}

impl From<StepCount> for Range<i8> {
    fn from(value: StepCount) -> Self {
        match value {
            StepCount::One => 1..2,
            StepCount::Two => 1..3,
            StepCount::Infinty => 1..8,
        }
    }
}
