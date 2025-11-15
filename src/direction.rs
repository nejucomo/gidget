use std::ops::Add;

use Direction::*;

use crate::pt::Pt;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn each() -> impl Iterator<Item = Direction> {
        [Up, Down, Left, Right].into_iter()
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

impl Add<Direction> for Pt {
    type Output = Option<Pt>;

    fn add(self, dir: Direction) -> Self::Output {
        let Pt(x, y) = self;
        let (optx, opty) = match dir {
            Up => (Some(x), y.checked_sub(1)),
            Down => (Some(x), y.checked_add(1)),
            Left => (x.checked_sub(1), Some(y)),
            Right => (x.checked_add(1), Some(y)),
        };
        optx.zip(opty).map(Pt::from)
    }
}
