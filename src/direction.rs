use std::ops::Add;

use Direction::*;
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
use rand::seq::IndexedRandom as _;

use crate::pt::Pt;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];

impl Direction {
    pub fn each() -> impl Iterator<Item = Direction> {
        DIRECTIONS.into_iter()
    }

    pub fn opposite(self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

impl Distribution<Direction> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        *DIRECTIONS.choose(rng).unwrap()
    }
}

impl Add<Direction> for Pt {
    type Output = Option<Pt>;

    fn add(self, dir: Direction) -> Self::Output {
        let (optcol, optrow) = match dir {
            Up => (Some(self.col()), self.row().checked_sub(1)),
            Down => (Some(self.col()), self.row().checked_add(1)),
            Left => (self.col().checked_sub(1), Some(self.row())),
            Right => (self.col().checked_add(1), Some(self.row())),
        };
        optcol.zip(optrow).map(Pt::from)
    }
}
