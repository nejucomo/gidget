use std::ops::{Index, IndexMut};

use crate::direction::Direction::{self, Down, Left, Right, Up};

#[derive(Copy, Clone, Debug, Default)]
pub struct DirPack<T> {
    pub up: T,
    pub down: T,
    pub left: T,
    pub right: T,
}

impl<T> Index<Direction> for DirPack<T> {
    type Output = T;

    fn index(&self, d: Direction) -> &Self::Output {
        use Direction::*;

        match d {
            Up => &self.up,
            Down => &self.down,
            Left => &self.left,
            Right => &self.right,
        }
    }
}

impl<T> IndexMut<Direction> for DirPack<T> {
    fn index_mut(&mut self, d: Direction) -> &mut Self::Output {
        match d {
            Up => &mut self.up,
            Down => &mut self.down,
            Left => &mut self.left,
            Right => &mut self.right,
        }
    }
}
